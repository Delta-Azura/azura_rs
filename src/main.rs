use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::io;
use std::env;
use std::fs;

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



