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