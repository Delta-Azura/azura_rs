use std::fs;

pub fn list() {
    let entries = fs::read_dir("/var/lib/pkg/DB/").unwrap().filter_map(|e| e.ok());
    //let output = entries.file_name().unwrap().to_string();
    for i in entries {
        println!("{}", i.file_name().to_str().unwrap());
    }
}