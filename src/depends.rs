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
use std::collections::HashSet;
use anyhow::Context;
use std::path::Path;

pub fn depends(pkg: &str) -> Vec<String> {  //-> Result<(), String> {
    let mut stack = vec![pkg.to_string()];
    let mut visited = std::collections::HashSet::new();

    //let path = format!("/var/lib/pkg/DB/{}/META", pkg);
    //let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
   // let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    //let deps = content.lines().filter(|l| l.starts_with('R'));
    while let Some(rawpkg) = stack.pop() {
        if !visited.insert(rawpkg.clone()) {
            continue;
        }
        println!("{}", rawpkg);
        //let rawwpkg = format!("{}", rawpkg);
        if !Path::new(&format!("/var/lib/pkg/DB/{}/META", rawpkg)).exists() {
            println!("{} isn't installed", rawpkg);
            std::process::exit(1)
        }
        let META = std::fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg));
        for i in META.unwrap().lines().filter(|l| l.starts_with('R')) {
            if i.is_empty() {
                continue
            }
            if let Some((_, deps)) = i.split_once('R') {
                for dep in deps.lines() {
                    let name = dep.trim_end_matches(|c: char| c.is_numeric());
                    if !visited.contains(name) {
                        stack.push(name.to_string());
                        //println!("pushing {}", name);
                    }
                }
               // for e in name.lines() {
                    //if Path::new(format!("/var/lib/pkg/DB/{}", e)).exists() {
                    //    println!("{} is already installed", e);
                    //} else {
                    //    println!("You need to install {}", e);
                    //    std::process::exit(1)
                    //}    //env::set_current_dir(format!("/var/lib/pkg/DB/{}", e))
                //}
                //println!("{}", name);
            }
        }
        //println!("{}", rawpkg);
    }
    //return stack.to_string()
    //println!("{}", rawpkg);
    //Ok(())
    return stack
}
