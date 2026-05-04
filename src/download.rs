use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::fs::File;
use std::io::Write;


pub fn download(url: &str) -> String {
    // Personnal notes :
    // Setting up the first variable to get the answer
    // Checking lenght of the answer
    // Setting the progress bar style (random settings)
    let mut answer = reqwest::blocking::get(url).unwrap();
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
    let mut source = File::create(tarball).unwrap();
    // As the downloading goes on we read the answer from the buffer
    // We write into the file we are downloading and we move the progress bar forward.
    loop {
        let n = answer.read(&mut buf).unwrap();
        if n == 0 { break; }
        source.write_all(&buf[..n]).unwrap();
        pb.inc(n as u64);
    }
    tarball.to_string() //giving back file name
}