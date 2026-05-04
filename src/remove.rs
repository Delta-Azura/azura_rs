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

use std::env;
use std::fs;
use std::path::Path;
use std::env::current_dir;
use anyhow::{Result, Context};
use std::fs::File;


pub fn remove(rawpkg: &String) -> Result<()> {
    File::create("/var/cache/raw.tmp")?;
    let _ = fs::remove_file("/var/cache/raw.tmp");
    let current = current_dir()?;
    let check = format!("/var/lib/pkg/DB/{}", rawpkg);
    if Path::new(&check).exists() {
        env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg))?;
        let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", rawpkg))?;
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
    Ok(())
}
