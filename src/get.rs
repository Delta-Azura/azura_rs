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

use crate::getconf::getconf;
use anyhow::{Result, Context};
use crate::download::download;
use crate::install::install;
use std::fs;
use std::env;
use crate::depends::depends;



pub fn get(pkg: &str) -> Result<()> {
    let (mode, trash, url) = getconf().unwrap();
    if mode != "binary" {
        println!("Raw is used in binary mode, cannot connect to the repo");
        std::process::exit(1);
    }
    env::set_current_dir("/var/cache/").unwrap();
    let metadata = download(&format!("{}/metadata", url))?;
    if metadata.contains(&format!("{}", pkg)) {
        let version = metadata.split_once('.').map(|(_, version)| version).unwrap();
        let release = metadata.split_once('#').map(|(_, release)| release).unwrap();
        let path = metadata.lines().find(|l| l.ends_with(&format!("{}.{}#{}.raw.tar.gz", pkg, version, release))).unwrap().to_string();

        let tarball = download(&path)?;
        install(&tarball)?;
        let dependencies = depends(pkg);
        for i in dependencies {
            get(&i);
        }
    } else {
        println!("Not found");
        std::process::exit(1)
    }

    Ok(())

}
