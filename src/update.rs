use std::fs::File;
use std::fs;
use crate::remove;
use std::path::Path;
use crate::install::install;
use crate::conflict::conflict;


pub fn update(rawpkg: &String) {
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