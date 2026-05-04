use std::fs;
use std::env;
use anyhow::{Result};
use anyhow::Context;


pub fn info(rawpkg: &String) -> Result<()> {
    //let path = format!("/var/lib/pkg/DB/");
    env::set_current_dir(format!("/var/lib/pkg/DB/{}", rawpkg)).context("Package isn't installed")?;
    // Add directory listing
    //let entry = fs::read_dir(".")
    //    .unwrap()
    //    .filter_map(|e| e.ok())
    //    .find(|e| e.file_name().to_str().unwrap_or("").starts_with(rawpkg));
    //if let Some(e) = entry {
    //let directory_tmp = e.file_name(); 
    //    let directory = directory_tmp.to_str().unwrap();
    let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg)).unwrap();
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
    let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    let name = content.iter().find(|l| l.starts_with('N')).unwrap().split_once('N').map(|(_, name)| name).unwrap().to_string();
    println!("Name : {}", name);
    let version = content.iter().find(|l| l.starts_with('V')).unwrap().to_string().split_once('V').map(|(_, version)| version).unwrap().to_string();
    println!("Version = {}", version);
    let description = content.iter().find(|l| l.starts_with('D')).unwrap().to_string().split_once('D').map(|(_, description)| description).unwrap().to_string();
    println!("Description = {}", description);
    let packager = content.iter().find(|l| l.starts_with('P')).unwrap().to_string().split_once('P').map(|(_, packager)| packager).unwrap().to_string();
    println!("Packager = {}", packager);
    Ok(()) 
}