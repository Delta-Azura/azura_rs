use anyhow::{Result, Context};
use std::fs;

pub fn libs(pkg : &str,option: Option<&str>) -> Result<()> {
    if option == Some("all") {
        let libs = format!("/var/lib/pkg/DB/{}/files", pkg);
        let output = fs::read_to_string(libs).context("Package isn't installed")?;
        for i in output.lines() {
            if i.contains(".so") {
                println!("{}", i);
            }
        }
    } else {
        let libs = format!("/var/lib/pkg/DB/{}/files", pkg);
        let output = fs::read_to_string(libs).context("Package isn't installed")?;
        for i in output.lines() {
            if !i.contains("security") {
                if i.contains(".so") {
                    println!("{}", i);
                }
            }
        }
    }
    Ok (())
}