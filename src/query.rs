use std::fs;
use std::env;



pub fn query(path: &String) {
    env::set_current_dir("/var/lib/pkg/DB/").unwrap();
    let target = format!("{}", path).split_once('/').map(|(_, target)| target).unwrap().to_string(); 
    //.map(|(_, name)| name).unwrap().to_string();
    for e in fs::read_dir(".").unwrap().filter_map(|e| e.ok()) {
        let directory_tmp = e.file_name(); 
        let directory = directory_tmp.to_str().unwrap();
        let compare = fs::read_to_string(format!("/var/lib/pkg/DB/{}/files", directory)).unwrap();
        for line in compare.lines() {
            if line == target {
            println!("This file/repertory belongs to : {}", directory);
            }
        }
    }
}