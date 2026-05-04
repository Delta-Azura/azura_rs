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

use std::io::{stdout, BufWriter};
use std::io;
use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use flate2::read::GzDecoder;
use tar::Archive;
use bzip2::read::BzDecoder;
use xz2::read::XzDecoder;
use walkdir::WalkDir;
use std::fs::write;
use tar::Builder;
use xz2::write::XzEncoder;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::path::Path;
use recursive_copy::{copy_recursive, CopyOptions};
use std::io::Read;
use std::env::current_dir;
use std::fs::metadata;
mod install;
mod conflict;
mod info;
mod query;
mod remove;
mod update;
mod package;
mod download;
mod extract;
mod file_type;
use crate::install::install;
use crate::info::info;
use crate::query::query;
use crate::remove::remove;
use crate::update::update;
use crate::package::package;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "package" {
        package()
    } 

    if args[1] == "install" {
        let argument = format!("{}", args[2]);
        install(&argument)
    }

    if args[1] == "info" {
        let argument = format!("{}", args[2]);
        info(&argument)
    }
    if args[1] == "remove" {
        let argument = format!("{}", args[2]);
        remove(&argument)
    }
    if args[1] == "query" {
        let argument = format!("{}", args[2]);
        query(&argument)
    }
    if args[1] == "update" {
        let argument = format!("{}", args[2]);
        update(&argument)
    }

}

