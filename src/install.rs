use std::path::Path;
use std::fs;
use std::process::Command;
use std::env;
use recursive_copy::{copy_recursive, CopyOptions};
use crate::conflict::conflict;
use tar::Archive;
use flate2::read::GzDecoder;


pub fn install(rawpkg: &String) {
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