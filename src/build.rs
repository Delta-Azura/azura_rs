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


use anyhow::{Result, Context};
use crate::package::package;
//use std::fs::File;
use std::env;
use std::fs;
use crate::getconf::getconf;
use std::path::Path;
use crate::install::install;
use crate::update::update;
use question::{Answer, Question};
use std::process::Command;

//use users::switch::switch_user_group;


pub fn build(to_build: &str) -> Result<()> {
    if let Some(root) = getconf() {
        let index = fs::read_to_string("index.raw").unwrap();
        let found = index.lines().find(|line| line.contains(to_build));
        if let Some(building) = found {
            env::set_current_dir(&building).unwrap();
            package()?;
            let question = Question::new("Do you want to install the new package ? [yes/no] : ")
                .yes_no()
                .until_acceptable()
                .default(Answer::YES)
                .show_defaults()
                .clarification("Please enter either 'yes' or 'no'\n")
                .ask();
            if question == Some(Answer::YES) {
                if Path::new(&format!("/var/lib/pkg/DB/{}", to_build)).exists() {
                    let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                    if Path::new("/usr/bin/sudo").exists() {
                        Command::new("sudo").args(["raw", "update", &content.unwrap()]).status().unwrap();
                    } else {
                        println!("sudo isn't installed, please go to the build directory to install {}", to_build);
                    }
                    //drop(guard);
                } else {
                    let content = fs::read_dir(".").unwrap().filter_map(|e| e.ok()).map(|e| e.file_name().to_str().unwrap().to_owned()).find(|name| name.contains("raw"));
                    if Path::new("/usr/bin/sudo").exists() {
                        Command::new("sudo").args(["raw", "install", &content.unwrap()]).status().unwrap();
                    } else {
                        println!("sudo isn't installed, please go to the build directory to install {}", to_build);
                    }
                }
            }

        } else {
            println!("Not found, try running raw index to update the repo database");
            std::process::exit(1)
        }
    }
    Ok(())
}