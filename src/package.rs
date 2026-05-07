// Raw is a simple package manager written in rust, it aims to be compatible with the Pkgfiles written that works with pkgmk from pkgutils/cards
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use std::fs::File;
use std::io::Write;
//use crate::Path;
use std::path::Path;
//use crate::fs;
use std::fs;
//use crate::env;
use std::env;
use std::process::Command;
use crate::download::download;
use crate::extract::extract;
use walkdir::WalkDir;
use tar::Builder;
use liblzma::write::XzEncoder;
use flate2::Compression;
use flate2::write::GzEncoder;
use anyhow::{Result, Context};



pub fn package() -> Result<()> {
        match File::create("/var/cache/raw.tmp") {
        Ok(_) => {
            println!("You are building as root !");
            fs::remove_file("/var/cache/raw.tmp");
            std::process::exit(1)
        }
        Err(e) => {}
    }
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
        .args(["-c", "source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo $depends && echo ${source[@]}"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut variables  = stdout.lines();
    let version = variables.next().unwrap();
    let name = variables.next().unwrap(); 
    let packager = variables.next().unwrap();
    let release = variables.next().unwrap();
    let description = variables.next().unwrap();
    let depends = variables.next().unwrap();
    let source = variables.next().unwrap();
    //let makedepends = variables.next().unwrap();
    //if makedepends == "none" {
    //    println!("No makedepends");
    //} else {
    //    for i makedepends.lines() {

    //    }

    //}
    let actual = std::env::current_dir().unwrap();
    let col = actual.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
    let collection = std::env::current_dir().unwrap();
    let current = collection.file_name().unwrap().to_str().unwrap().to_string();
    let collection = collection.display().to_string();
    println!("Setting collection as : {}", col);
    let mut meta = File::create("META").unwrap();
    let metadata = format!("N{}\nV{}\nr{}\nc{}\nD{}\nP{}\nR{}\n", name, version, release, col, description, packager, depends);
    write!(meta, "{}", metadata).unwrap();
    if Path::new("work").exists() {
        println!("Removing work/");
        fs::remove_dir_all("work/")?;
    }
    if Path::new("pkg").exists() {
        println!("Removing pkg/");
        fs::remove_dir_all("pkg/")?;
    }
    fs::create_dir("work")?;
    fs::create_dir("pkg")?;
    //if Path::new("config").exists() {
    //    fs::copy("config", "work/config").unwrap();
    //}
    let building = format!("{}/work", collection);
    println!("Switching to the work directory {}", building);
    for src in source.split_whitespace() {
        if src.contains("http") {
            if src.contains("rpm") {
                let tarball = download(src)?;
                fs::remove_dir("work/").unwrap();
                //env::set_current_dir("/home/alexis/vivaldi")?;
                extract(&tarball)
            } else {
                env::set_current_dir(&building)?;
                let tarball = download(src)?;
                env::set_current_dir(&collection)?;
                if tarball.contains(".patch.gz") {
                    continue;
                } else {
                    env::set_current_dir(&building)?;
                    extract(&tarball);
                    env::set_current_dir(&collection)?;
                }
            }
        } else {
            fs::copy(src, format!("work/{}", src))?;
            env::set_current_dir(&building)?;
        }
    }
    env::set_current_dir(&building)?;
    //let extracted = Path::new("{}/{}", collection, tarball)
    env::set_current_dir(&collection)?;
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
        fs::remove_file(format!("{}.footprint", name))?;
    }
    if Path::new(&format!("{}.{}.raw.tar.gz", name, version)).exists() {
        println!("Removing the previous generated package");
        fs::remove_file(format!("{}.{}.raw.tar.gz", name, version))?;
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
        fs::copy(format!("{}.post-install", name), format!("pkg/{}.post-install", name))?;
    } else {
        println!("No need to prepare post-installation");
    }
    //let packagename = format!("{}", name);
    println!("Generating package");
    let tar = File::create(format!("{}.{}#1.raw.tar.gz", name, version))?;
    let enc = GzEncoder::new(tar, Compression::default());
    let mut a = tar::Builder::new(enc);
    a.follow_symlinks(false);
    a.append_dir_all("", "pkg/")?;
    a.finish().unwrap();
    fs::remove_dir_all("pkg")?;
    Ok(())
}