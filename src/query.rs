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
use std::env;



pub fn query(path: &String) {
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