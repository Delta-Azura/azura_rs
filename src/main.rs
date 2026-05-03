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
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::env::current_dir;
use std::fs::metadata;

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
    if args[1] == "update" {
        let argument = format!("{}", args[2]);
        update(&argument)
    }

}

// Match can have different implementation depending on the type of error
fn package() {
    match fs::exists("Pkgfile") {
        Ok(true) => println!("Starting to build"),
        Ok(false) => {
            println!("Pkgfile doesn't exist.");
            std::process::exit(1);
        }
        Err(e) => {
            println!("Error : {e}");
            std::process::exit(1);
        }
    }
    let output = Command::new("bash")
        .args(["-c", "source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo ${source[@]}"])
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
    //if Path::new("config").exists() {
    //    fs::copy("config", "work/config").unwrap();
    //}
    let building = format!("{}/work", collection);
    println!("Switching to the work directory {}", building);
    for src in source.split_whitespace() {
        if src.contains("http") {
            env::set_current_dir(&building).unwrap();
            let tarball = download(src);
            env::set_current_dir(&collection).unwrap();
            if tarball.contains(".patch.gz") {
                continue;
            } else {
                env::set_current_dir(&building).unwrap();
                extract(&tarball);
                env::set_current_dir(&collection).unwrap();

            }
        } else {
            fs::copy(src, format!("work/{}", src)).unwrap();
            env::set_current_dir(&building).unwrap();
        }
    }
    env::set_current_dir(&building).unwrap();
    //let extracted = Path::new("{}/{}", collection, tarball)
    env::set_current_dir(&collection).unwrap();
    match Command::new("bash")
    .args(["-c", "fakeroot bash -c 'source Pkgfile && PKG=$(pwd)/pkg && cd work && build'"])
    .env("MAKEFLAGS", format!("-j{}", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)))
    .env("CFLAGS", "-O2 -pipe")
    .env("CXXFLAGS", "-O2 -pipe")
    //.status()
    .status() {
        // need if s.success because of the type of answer from status
        Ok(s) if s.success() => {
            println!("Build succeded");
            env::set_current_dir(&collection).unwrap();
            fs::remove_dir_all("work").unwrap();
        }
        Ok(s) => {
            // Don't ask
            println!("The build failed (code {:?})", s.code());
            std::process::exit(1);
        }
        Err(e) => {
            println!("The build failed {e}");
            std::process::exit(1);
        }

    }

    let prepare = format!("{}/pkg", collection);
    
    //env::set_current_dir(&prepare).unwrap();
    if Path::new(&format!("{}.footprint", name)).exists() {
        println!("Removing actual footprint");
        fs::remove_file(format!("{}.footprint", name)).unwrap();
    }
    if Path::new(&format!("{}.{}.raw.tar.gz", name, version)).exists() {
        println!("Removing the previous generated package");
        fs::remove_file(format!("{}.{}.raw.tar.gz", name, version)).unwrap();
    }
    println!("Generating footprint");
    let mut footprint = File::create(format!("{}.footprint", name)).unwrap();
    for entry in WalkDir::new(&prepare).follow_links(false) {
        let foot = entry.unwrap().path().display().to_string();
        let pathpkg = foot.split_once(&prepare).map(|(_,pathpkg)| pathpkg).unwrap().to_string();
        if pathpkg.is_empty() { continue; }
        let list = pathpkg.split_once('/').map(|(_,list)| list).unwrap().to_string();
        //if list.is_empty() { continue; }
        //let mut footprint = format!("{}", foot);
        writeln!(footprint, "{}", list).unwrap();
    }
    fs::copy("META", "pkg/META").unwrap();
    fs::remove_file("META").unwrap();
    fs::copy(format!("{}.footprint", name), format!("pkg/{}.footprint", name)).unwrap();
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
    println!("Generating package");
    let tar = File::create(format!("{}.{}.raw.tar.gz", name, version)).unwrap();
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.follow_symlinks(false);
    a.append_dir_all("", "pkg/").unwrap();
    a.finish().unwrap();
    fs::remove_dir_all("pkg").unwrap();
}


fn install(rawpkg: &String) {
    //let pkg_name = rawpkg.split_once(".raw").map(|(name, _)| name).unwrap_or(rawpkg);
    if Path::new("/tmp/conflict").exists() {
        fs::remove_file("/tmp/conflict").unwrap();
    } else {
        conflict(&rawpkg);
    }
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap();
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg)).unwrap();
    println!("Copying {} to /var/lib/pkg/DB/{}/{}", rawpkg, pkg, rawpkg);
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
        follow_symlinks: false,
        restrict_symlinks: false,
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
    fs::copy(format!("/{}.footprint", pkg), format!("/var/lib/pkg/DB/{}/files", pkg)).unwrap();
    fs::remove_file("/META").unwrap();
    fs::remove_file(format!("/{}.footprint", pkg)).unwrap();
    fs::remove_file(format!("/{}", rawpkg)).unwrap();
    if Path::new(&format!("/{}.pre-install", pkg)).exists() {
        fs::remove_file(format!("/{}.pre-install", pkg)).unwrap();
    }
    if Path::new(&format!("/{}.post-install", pkg)).exists() {
        fs::remove_file(format!("/{}.post-install", pkg)).unwrap();
    }

}

fn download(url: &str) -> String {
    // Personnal notes :
    // Setting up the first variable to get the answer
    // Checking lenght of the answer
    // Setting the progress bar style (random settings)
    let mut answer = reqwest::blocking::get(url).unwrap();
    let progress = answer.content_length().unwrap_or(0);
    let pb = ProgressBar::new(progress);
     pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("##"),
    );
    // init buf size, 8192 is the most used one 
    let mut buf = [0u8; 8192];
    //let bytes = answer.bytes().unwrap();
    let tarball = url.split('/').last().unwrap();
    let mut source = File::create(tarball).unwrap();
    // As the downloading goes on we read the answer from the buffer
    // We write into the file we are downloading and we move the progress bar forward.
    loop {
        let n = answer.read(&mut buf).unwrap();
        if n == 0 { break; }
        source.write_all(&buf[..n]).unwrap();
        pb.inc(n as u64);
    }
    tarball.to_string() //giving back file name
}

fn extract(tarball: &String) {
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
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
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
    let current = current_dir().unwrap();
    let check = format!("/var/lib/pkg/DB/{}", rawpkg);
    if Path::new(&check).exists() {
        env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg)).unwrap();
        let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", rawpkg)).unwrap();
        let content = file.lines();
        fs::remove_dir_all(format!("/var/lib/pkg/DB/{}", rawpkg));
        let protected = ["bin", "lib", "lib64", "sbin"];
        for i in content {
            if !protected.contains(&i) {
                let _ = fs::remove_file(format!("/{}", i));
                let _ = fs::remove_dir(format!("/{}", i));
            }
            //println!("Package has been correctly uninstalled !");
        } 
    } else {
            println!("This package isn't installed, can't remove it");
    }
    // Necessary for the update function.
    env::set_current_dir(current).unwrap();
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


fn update(rawpkg: &String) {
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap().to_string();
    //println!("{}", pkg);
    if Path::new(&format!("/var/lib/pkg/DB/{}", pkg)).exists() {
        println!("removing previous package");
        remove(&pkg);
        File::create("/tmp/conflict").unwrap();
        conflict(&rawpkg);
        println!("Installing the new one");
        install(&rawpkg);
        fs::remove_file("/tmp/conflict");

    } else {
        println!("Package isn't installed");
        std::process::exit(1);
    }

}

fn conflict(rawpkg: &String) {
    //File::create("/tmp/conflict").unwrap();
    let pkg = rawpkg.split_once('.').map(|(pkg, _)| pkg).unwrap().to_string();
    if Path::new(&format!("/tmp/{}", pkg)).exists() {
        fs::remove_dir_all(format!("/tmp/{}", pkg)).unwrap();
    }
    fs::create_dir(format!("/tmp/{}", pkg)).unwrap();
    fs::copy(rawpkg, format!("/tmp/{}/{}", pkg, rawpkg)).unwrap();
    env::set_current_dir(format!("/tmp/{}", pkg)).unwrap();
    if rawpkg.ends_with(".tar.gz") || rawpkg.ends_with(".tgz") {
        let file = fs::File::open(rawpkg).unwrap();
        let mut archive = Archive::new(GzDecoder::new(file));
        archive.unpack(".").unwrap();
    }
    let compare = fs::read_to_string(format!("/tmp/{}/{}.footprint", pkg, pkg)).unwrap();
    for e in fs::read_dir("/var/lib/pkg/DB/.").unwrap().filter_map(|e| e.ok()) {
        let directory_tmp = e.file_name();
        let directory = directory_tmp.to_str().unwrap();
        let target = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", directory)).unwrap();
        println!("{}", compare);
        for lines in target.lines() {
            //let release = variables.next().unwrap();
            for line in compare.lines() {
                if  line == lines {
                    let list = format!("{}", lines);
                    if list.is_empty() { continue; }
                    //file_type(&list);
                    if file_type(&list) == true {
                        let test = format!("/{}", &list);
                        let owner = query(&test);
                        //println!("{}"owner);
                        std::process::exit(1)
                        
                    }
                }
            }
        }
    }
}

fn file_type(list: &String) -> bool {
    //let metadata = fs::metadata(list)?;
    match fs::metadata(list) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
    //assert!(!metadata.is_dir());
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}
