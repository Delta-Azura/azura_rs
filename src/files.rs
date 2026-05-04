use anyhow::{Result};
use anyhow::Context;
use std::fs;


pub fn files(pkg: &str) -> Result<()> {
    let footprint = format!("/var/lib/pkg/DB/{}/files", pkg);
    let output = fs::read_to_string(footprint).context("Package isn't installed")?;
    println!("{}", output);
    Ok (())
}