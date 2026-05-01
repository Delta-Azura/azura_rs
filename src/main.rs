use ferris_says::say; // from the previous step
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
    } else {
        if args[1] == "install" {
            let argument = format!("{}", args[2]);
            install(&argument)
        } else {
            println!("Please specify an option")
        }
    }

}

fn trash () {
    let stdout = stdout(); 
    let message = String::from("Hello world!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed : {guess}");
    say(&guess, width, &mut writer).unwrap();
    match env::set_current_dir("/var/cache/azura/pkg") {
        Ok(_) => println!("Changing folder"),
        Err(e) => {
            println!("Directory doesn't exist, first run : {e}");
            match fs::create_dir("/var/cache/azura/pkg") {
                Ok(_) => println!("Directory created"),
                Err(e) => {
                    println!("You don't have the root privelegies : {e}");
                    std::process::exit(1);
                }
            }
        }
    }

    println!("everything is good");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No arguments, please run : azura build packagename");
        std::process::exit(1);
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
    let mut footprint = File::create("footprint").unwrap();
    for entry in WalkDir::new(&prepare).follow_links(true) {
        let foot = entry.unwrap().path().display().to_string();
        //let mut footprint = format!("{}", foot);
        writeln!(footprint, "{}", foot).unwrap();
    }
    fs::copy("META", "pkg/META").unwrap();
    fs::copy("footprint", "pkg/footprint").unwrap();
    //let packagename = format!("{}", name);
    let tar = File::create(format!("{}-{}.raw.tar.gz", name, version)).unwrap();
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.append_dir_all("", "pkg/").unwrap();
    a.finish().unwrap();
}


fn install(package: &str) {
    let pkg_name = package.split_once('.').map(|(name, _)| name).unwrap_or(package);
    fs::create_dir(format!("/var/lib/pkg/DB/{}", pkg_name)).unwrap();
    env::set_current_dir(format!("/var/lib/pkg/DB/{}", pkg_name)).unwrap();
    if package.ends_with(".tar.gz") || package.ends_with(".tgz") {
        let file = fs::File::open(package).unwrap();
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
    copy_recursive(Path::new(""), Path::new("/"), &opts).unwrap();
    fs::copy("/META", format!("/var/lib/pkg/DB/{}", pkg_name)).unwrap();
    fs::copy("/footprint", format!("/var/lib/pkg/DB{}", pkg_name)).unwrap();
    fs::remove_file("/META").unwrap();
    fs::remove_file("/footprint").unwrap();

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