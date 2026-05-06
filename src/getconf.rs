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


use std::path::Path;
use std::fs;
use std::fs::File;
use std::env;
use anyhow::Context;


pub fn getconf() ->   Result<(String, String), String> {
    match Path::new("/etc/raw.conf").exists() {
        true => {
            let config = fs::read_to_string("/etc/raw.conf").unwrap();
            if config.clone().contains("mode binary") {
                let repo = config.clone().split_once("url").map(|(_, repo)| repo).unwrap().to_string();
                return Ok(("binary".to_string(), repo));
            }
            if config.clone().contains("mode source") {
                let root = config.clone().split_once("root").map(|(_, root)| root).unwrap().to_string();
                println!("{}", root);
                env::set_current_dir(&root.trim()).unwrap();//.context("Repertory doesn't exists")?;
                return Ok(("source".to_string(), root.to_string()));
            } else {
                return Err("not specified".to_string());
            }
        }
        false => {
            println!("No mode specified, aborting");
            std::process::exit(1)
        }
        
    }
}