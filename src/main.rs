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
mod files;
use crate::files::files;
use crate::install::install;
use crate::info::info;
use crate::query::query;
use crate::remove::remove;
use crate::update::update;
use crate::package::package;
use anyhow::{Result};



fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "package" {
        package()?;
        return Ok(());
    } 

    if args[1] == "install" {
        let argument = format!("{}", args[2]);
        install(&argument)?;
        return Ok(());
    }

    if args[1] == "info" {
        let argument = format!("{}", args[2]);
        info(&argument)?;
        return Ok(())
    }
    if args[1] == "remove" {
        let argument = format!("{}", args[2]);
        remove(&argument)?;
        return Ok(())
    }
    if args[1] == "query" {
        let argument = format!("{}", args[2]);
        query(&argument);
        return Ok(())
    }
    if args[1] == "update" {
        let argument = format!("{}", args[2]);
        update(&argument);
        return Ok(())
    }
    if args[1] == "files" {
        let argument = format!("{}", args[2]);
        files(&argument)?;
        return Ok(())
    } 
    return Ok(());
}

