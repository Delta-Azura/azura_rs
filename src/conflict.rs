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

use std::fs;
use crate::query;
use std::env;
use std::path::Path;
use crate::file_type::file_type;
use flate2::read::GzDecoder;
use tar::Archive;


pub fn conflict(rawpkg: &String) {
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
                        if test != "/usr/share/info/dir" {
                            let owner = query(&test);
                        //println!("{}"owner);
                            std::process::exit(1)

                        }
                        
                    }
                }
            }
        }
    }
    for i in compare.lines() {
        let r = format!("/{}", i);
        if file_type(&r) == true {
            if Path::new(&r).exists() {
                println!("File {} already present on the system", i);
                std::process::exit(1)
            }
        }
    }
}