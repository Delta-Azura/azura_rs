use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::io;
use std::env;
use std::fs;
use std::process::Command;
use std::fs::File;
use std::io::Write;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() {
    let stdout = stdout(); 
    let message = String::from("Hello world!");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed : {guess}");
    say(&guess, width, &mut writer).unwrap();
    match env::set_current_dir("/var/cache/azura/pkg") {
        Ok(_) => println!("Changing folder"),
        Err(e) => {
            println!("Directory doesn't exist, first run : {e}");
            match fs::create_dir("/var/cache/azura/pkg") {
                Ok(_) => println!("Directory created"),
                Err(e) => {
                    println!("You don't have the root privelegies : {e}");
                    std::process::exit(1);
                }
            }
        }
    }

    println!("everything is good");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No arguments, please run : azura build packagename");
        std::process::exit(1);
    }

}



fn package() {
    match fs::exists("Pkgfile") {
        Ok(_) => println!("Starting to build"),
        Err(e) => {
            println!("Pkgfile doesn't exist. {e}");
            std::process::exit(1);
        }
    }
    let output = Command::new("bash")
        .args(["-c", "source Pkgfile && echo $version && echo $name && echo $packager && echo $release && echo $description && echo $source"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut variables  = stdout.lines();
    let version = variables.next().unwrap();
    let name = variables.next().unwrap(); 
    let packager = variables.next().unwrap();
    let release = variables.next().unwrap();
    let description = variables.next().unwrap();
    let source = variables.next().unwrap();
    let collection = std::env::current_dir().unwrap();
    let current = collection.file_name().unwrap().to_str().unwrap().to_string();
    let collection = collection.display().to_string();
    println!("Setting collection as : {}", current);
    let mut meta = File::create("META").unwrap();
    let metadata = format!("N{}\nV{}\nr{}\nc{}\nD{}\nP{}", name, version, release, current, description, packager);
    write!(meta, "{}", metadata).unwrap();
    fs::create_dir("work").unwrap();
    fs::create_dir("pkg").unwrap();
    let building = format!("{}/work", collection);
    env::set_current_dir(&building).unwrap();
    println!("Switching to the work directory {}", building);
    let tarball = download(&source);
    extract(&tarball);
    //let extracted = Path::new("{}/{}", collection, tarball)
    Command::new("bash")
    .args(["-c", "source Pkgfile && fakeroot build"])
    .status()
    .unwrap();
}


fn download(url: &str) -> String {
    let answer = reqwest::blocking::get(url).unwrap();
    let bytes = answer.bytes().unwrap();
    let tarball = url.split('/').last().unwrap();
    let mut source = File::create(tarball).unwrap();
    source.write_all(&bytes).unwrap();
    tarball.to_string() //giving back file name
}

fn extract(tarball: &str) {
    let source = File::open(tarball).unwrap();
    let gz = GzDecoder::new(source);
    let mut archive = Archive::new(gz);
    let directory = if let Some(pos) = tarball.find(".tar.") {
        tarball[..pos].to_string()
    } else {
        tarball.to_string()
    };
    fs::create_dir(&directory).unwrap();
    let current = std::env::current_dir().unwrap();
    let current = current.display().to_string();
    let unpacked = format!("{}/{}", current, directory);
    assert!(env::set_current_dir(&unpacked).is_ok());
    archive.unpack(".").unwrap();
}