use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::fs::File;
use std::io::Write;
use anyhow::{Result, Context};

pub fn download(url: &str) -> Result<String> {
    // Personnal notes :
    // Setting up the first variable to get the answer
    // Checking lenght of the answer
    // Setting the progress bar style (random settings)
    let mut answer = reqwest::blocking::get(url)?;
    let progress = answer.content_length().unwrap_or(0);
    let pb = ProgressBar::new(progress);
     pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("##"),
    );
    // init buf size, 8192 is the most used one 
    let mut buf = [0u8; 8192];
    //let bytes = answer.bytes().unwrap();
    let tarball = url.split('/').last().unwrap();
    let mut source = File::create(tarball)?;
    // As the downloading goes on we read the answer from the buffer
    // We write into the file we are downloading and we move the progress bar forward.
    loop {
        let n = answer.read(&mut buf).unwrap();
        if n == 0 { break; }
        source.write_all(&buf[..n]).unwrap();
        pb.inc(n as u64);
    }
    //giving back file name or the error 
    return Ok(tarball.to_string())
}