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
use std::fs::File;
use std::io::Write;
use std::io::Read;
use walkdir::WalkDir;
use anyhow::{Context, Result};
use std::path::Path;
use question::{Answer, Question};
use std::fs;



pub fn index() -> Result <()> {
    if let Some(root) = getconf() {
        if Path::new("index.raw").exists() {
            let question = Question::new("The index already exists, do you want to update it ? [y/n]")
                .yes_no()
                .until_acceptable()
                .default(Answer::YES)
                .show_defaults()
                .clarification("Please enter either 'yes' or 'no'\n")
                .ask();
            if question == Some(Answer::YES) {
                fs::remove_file("index.raw").unwrap();
                let mut rawfile = File::create("index.raw").context("This directory isn't usable as non-root, aborting")?;
                for entry in WalkDir::new(&root.trim()).max_depth(2).min_depth(2) {
            //File::open("index.raw").unwrap();
                    let entries = entry.unwrap().path().display().to_string().split_once(&root.trim()).map(|(_, entries)| entries).unwrap().to_string().split_once("/").map(|(_, remove)| remove).unwrap().to_string();
                    writeln!(rawfile,"{}", entries).unwrap();
                }
            } else {
                println!("Aborting");
                std::process::exit(0)
            }
        }
    }
    Ok(())
}