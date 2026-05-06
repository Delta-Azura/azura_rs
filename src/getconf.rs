use std::path::Path;
use std::fs;
use std::fs::File;
use std::env;
use anyhow::Context;


pub fn getconf() -> Option<String> {
    match Path::new("/etc/raw.conf").exists() {
        true => {
            let config = fs::read_to_string("/etc/raw.conf").unwrap();
            if config.clone().contains("mode binary") {
                let repo = config.clone().split_once("url").map(|(_, repo)| repo).unwrap().to_string();
                return Some(repo)
            }
            if config.clone().contains("mode source") {
                let root = config.clone().split_once("root").map(|(_, root)| root)?.to_string();
                println!("{}", root);
                env::set_current_dir(&root.trim()).unwrap();//.context("Repertory doesn't exists")?;

                File::create("Onyx.tmp").unwrap();//.context("This repertory isn't usable as not root, aborting")?;
                return Some(root)
            }
        }
        false => {
            println!("No mode specified, aborting");
            std::process::exit(1)
        }
        
    }
    None

}