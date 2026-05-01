//use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::io;
use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use flate2::read::GzDecoder;
use tar::Archive;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;
use walkdir::WalkDir;
use std::fs::write;
use tar::Builder;
use xz2::write::XzEncoder;
use flate2::Compression;
use flate2::write::GzEncoder;
//use walkdir_minimal_copy::{copy_recursive, CopyOptions};
use std::path::Path;
use recursive_copy::{copy_recursive, CopyOptions};
//use walkdir_minimal_copy::{copy_recursive, CopyOptions};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "package" {
        package()
    } 

    if args[1] == "install" {
        let argument = format!("{}", args[2]);
        install(&argument)
    }

    if args[1] == "info" {
        let argument = format!("{}", args[2]);
        info(&argument)
    }
    if args[1] == "remove" {
        let argument = format!("{}", args[2]);
        remove(&argument)
    }
    if args[1] == "query" {
        let argument = format!("{}", args[2]);
        query(&argument)
    }

}

fn package() {
    match fs::exists("Pkgfile") {
        Ok(_) => println!("Starting to build"),
        Err(e) => {
            println!("Pkgfile doesn't exist. {e}");
            std::process::exit(1);
        }
    }
    let output = Command::new("bash")
        .args(["-c", "source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo $source"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut variables  = stdout.lines();
    let version = variables.next().unwrap();
    let name = variables.next().unwrap(); 
    let packager = variables.next().unwrap();
    let release = variables.next().unwrap();
    let description = variables.next().unwrap();
    let source = variables.next().unwrap();
    let collection = std::env::current_dir().unwrap();
    let current = collection.file_name().unwrap().to_str().unwrap().to_string();
    let collection = collection.display().to_string();
    println!("Setting collection as : {}", current);
    let mut meta = File::create("META").unwrap();
    let metadata = format!("N{}\nV{}\nr{}\nc{}\nD{}\nP{}", name, version, release, current, description, packager);
    write!(meta, "{}", metadata).unwrap();
    if Path::new("work").exists() {
        println!("Removing work/");
        fs::remove_dir_all("work").unwrap();
    }
    if Path::new("pkg").exists() {
        println!("Removing pkg/");
        fs::remove_dir_all("pkg").unwrap();
    }
    fs::create_dir("work").unwrap();
    fs::create_dir("pkg").unwrap();
    let building = format!("{}/work", collection);
    env::set_current_dir(&building).unwrap();
    println!("Switching to the work directory {}", building);
    let tarball = download(&source);
    extract(&tarball);
    //let extracted = Path::new("{}/{}", collection, tarball)
    env::set_current_dir(&collection).unwrap();
    Command::new("bash")
    .args(["-c", "fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && cd work && build'"])
    .status()
    .unwrap();
    let prepare = format!("{}/pkg", collection);
    
    //env::set_current_dir(&prepare).unwrap();
    let mut footprint = File::create(format!("footprint.{}", name)).unwrap();
    for entry in WalkDir::new(&prepare).follow_links(true) {
        let foot = entry.unwrap().path().display().to_string();
        let pathpkg = foot.split_once(&prepare).map(|(_,pathpkg)| pathpkg).unwrap().to_string();
        if pathpkg.is_empty() { continue; }
        let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
        //if list.is_empty() { continue; }
        //let mut footprint = format!("{}", foot);
        writeln!(footprint, "{}", list).unwrap();
    }
    fs::copy("META", "pkg/META").unwrap();
    fs::copy(format!("footprint.{}", name), format!("pkg/footprint.{}", name)).unwrap();
    if Path::new(&format!("{}/{}.pre-install", collection, name)).exists() {
        fs::copy(format!("{}.pre-install", name), format!("pkg/{}.pre-install", name)).unwrap();
    } else {
        println!("No need to prepare pre-installation");
    }
    if Path::new(&format!("{}/{}.post-install", collection, name)).exists() {
        fs::copy(format!("{}.post-install", name), format!("pkg/{}.post-install", name)).unwrap();
    } else {
        println!("No need to prepare post-installation");
    }
    //let packagename = format!("{}", name);
    let tar = File::create(format!("{}-{}.raw.tar.gz", name, version)).unwrap();
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.append_dir_all("", "pkg/").unwrap();
    a.finish().unwrap();
}


fn install(rawpkg: &str) {
    //let pkg_name = rawpkg.split_once(".raw").map(|(name, _)| name).unwrap_or(rawpkg);
    let pkg = rawpkg.split_once('-').map(|(pkg, _)| pkg).unwrap();
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    fs::copy(rawpkg, format!("/var/lib/pkg/DB/{}/{}", pkg, rawpkg)).unwrap();
    env::set_current_dir(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    if rawpkg.ends_with(".tar.gz") || rawpkg.ends_with(".tgz") {
        let file = fs::File::open(rawpkg).unwrap();
        let mut archive = Archive::new(GzDecoder::new(file));
        archive.unpack(".").unwrap();
    } else {
        println!("No package in the format required : ABORTING");
        std::process::exit(1);
    }
    let opts = CopyOptions {
        overwrite: true,
        follow_symlinks: true,
        restrict_symlinks: true,
        content_only: false,
        ..Default::default()
    };
    if Path::new(&format!("{}.pre-install", pkg)).exists() {
        let pre_install = format!("chmod u+x {}.pre-install && ./{}.pre-install", pkg, pkg);
        println!("Starting pre-installation.");
        Command::new("bash")
        .args(["-c", &pre_install])
        .status()
        .unwrap();
        fs::remove_file(format!("{}.pre-install", pkg));
    } else {
        println!("No pre-installation required");
    }
    copy_recursive(Path::new("."), Path::new("/"), &opts).unwrap();
    println!("running ldconfig.....");
    Command::new("bash")
    .args(["-c", "ldconfig"])
    .status()
    .unwrap();
    if Path::new(&format!("{}.post-install", pkg)).exists() {
        let post_install = format!("chmod u+x {}.post-install && ./{}.post-install", pkg, pkg);
        println!("Starting post-installation.");
        Command::new("bash")
        .args(["-c", &post_install])
        .status()
        .unwrap();
        fs::remove_file(format!("{}.post-install", pkg));
    } else {
        println!("No post-installation required");
    }
    fs::remove_dir_all(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    fs::copy("/META", format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
    fs::copy(format!("/footprint.{}", pkg), format!("/var/lib/pkg/DB/{}/files", pkg)).unwrap();
    fs::remove_file("/META").unwrap();
    fs::remove_file(format!("/footprint.{}", pkg)).unwrap();

}

fn download(url: &str) -> String {
    let answer = reqwest::blocking::get(url).unwrap();
    let bytes = answer.bytes().unwrap();
    let tarball = url.split('/').last().unwrap();
    let mut source = File::create(tarball).unwrap();
    source.write_all(&bytes).unwrap();
    tarball.to_string() //giving back file name
}

fn extract(tarball: &str) {
    let source = File::open(tarball).unwrap();
    if tarball.ends_with(".tar.gz") || tarball.ends_with(".tgz") {
        let mut archive = Archive::new(GzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.xz") {
        let mut archive = Archive::new(XzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.bz2") {
        let mut archive = Archive::new(BzDecoder::new(source));
        archive.unpack(".").unwrap();
    } else if tarball.ends_with(".tar.zst") {
        let decoder = zstd::stream::read::Decoder::new(source).unwrap();
        let mut archive = Archive::new(decoder);
        archive.unpack(".").unwrap();
    }
}


// not ready yet
fn info(rawpkg: &String) {
    //let path = format!("/var/lib/pkg/DB/");
    env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg)).unwrap();
    // Add directory listing
    //let entry = fs::read_dir(".")
    //    .unwrap()
    //    .filter_map(|e| e.ok())
    //    .find(|e| e.file_name().to_str().unwrap_or("").starts_with(rawpkg));
    //if let Some(e) = entry {
    //let directory_tmp = e.file_name(); 
    //    let directory = directory_tmp.to_str().unwrap();
    let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg)).unwrap();
    let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    let name = content.iter().find(|l| l.starts_with('N')).unwrap().split_once('N').map(|(_, name)| name).unwrap().to_string();
    println!("Name : {}", name);
    let version = content.iter().find(|l| l.starts_with('V')).unwrap().to_string().split_once('V').map(|(_, version)| version).unwrap().to_string();
    println!("Version = {}", version);
    let description = content.iter().find(|l| l.starts_with('D')).unwrap().to_string().split_once('D').map(|(_, description)| description).unwrap().to_string();
    println!("Description = {}", description);
    let packager = content.iter().find(|l| l.starts_with('P')).unwrap().to_string().split_once('P').map(|(_, packager)| packager).unwrap().to_string();
    println!("Packager = {}", packager);      
}

fn remove(rawpkg: &String) {
    let check = format!("/var/lib/pkg/DB/{}", rawpkg);
    if Path::new(&check).exists() {
        env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg)).unwrap();
        let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", rawpkg)).unwrap();
        let content = file.lines();
        fs::remove_dir_all(format!("/var/lib/pkg/DB/{}", rawpkg));
        for i in content {
            let _ = fs::remove_file(format!("/{}", i));
            let _ = fs::remove_dir(format!("/{}", i));
            println!("Package has been correctly uninstalled !");
        } 
    } else {
            println!("This package isn't installed, can't remove it");
    }
    //scanning the entire directory to find the right path
    //let entry = fs::read_dir(".")
    //    .unwrap()
    //    .filter_map(|e| e.ok())
    //    .find(|e| e.file_name().to_str().unwrap_or("").starts_with(rawpkg));
    //if let Some(e) = entry {
    //    let directory_tmp = e.file_name(); 
    //    let directory = directory_tmp.to_str().unwrap();
}

fn query(path: &String) {
    env::set_current_dir("/var/lib/pkg/DB/").unwrap();
    let target = format!("{}", path).split_once('/').map(|(_, target)| target).unwrap().to_string(); 
    //.map(|(_, name)| name).unwrap().to_string();
    for e in fs::read_dir(".").unwrap().filter_map(|e| e.ok()) {
        let directory_tmp = e.file_name(); 
        let directory = directory_tmp.to_str().unwrap();
        let compare = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", directory)).unwrap();
        for line in compare.lines() {
            if line == target {
            println!("This file/repertory belongs to : {}", directory);
            }
        }
    }
}