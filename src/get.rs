use crate::getconf::getconf;
use anyhow::{Result, Context};
use crate::download::download;
use crate::install::install;
use std::fs;
use std::env;
use crate::depends::depends;



pub fn get(pkg: &str) -> Result<()> {
    let (mode, url) = getconf().unwrap();
    if mode != "binary" {
        println!("Raw is used in binary mode, cannot connect to the repo");
        std::process::exit(1);
    }
    env::set_current_dir("/var/cache/").unwrap();
    let metadata = download(&format!("{}/metadata", url))?;
    if metadata.contains(&format!("{}", pkg)) {
        let path = metadata.lines().find(|l| l.starts_with(&format!("{}-", pkg))).unwrap().to_string();
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
