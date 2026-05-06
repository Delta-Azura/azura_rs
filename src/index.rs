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