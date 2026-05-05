use std::fs;
use std::collections::HashSet;
use anyhow::Context;
use std::path::Path;

pub fn depends(pkg: &str) {//-> Result<(), String> {
    let mut stack = vec![pkg.to_string()];
    let mut visited = std::collections::HashSet::new;

    //let path = format!("/var/lib/pkg/DB/{}/META", pkg);
    //let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
   // let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    //let deps = content.lines().filter(|l| l.starts_with('R'));
    while let Some(rawpkg) = stack.pop() {
        if !visited().insert(rawpkg.clone()) {
            continue;
        }
        //let rawwpkg = format!("{}", rawpkg);
        if Path::new(&format!("/var/lib/pkg/DB/{}/META", rawpkg)).exists() {
            continue
        } else {
            println!("{} isn't installed", rawpkg);
            std::process::exit(1)
        }
        let META = std::fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", rawpkg));
        for i in META.unwrap().lines().filter(|l| l.starts_with('R')) {
            if i.is_empty() {
                continue
            }
            if let Some((_, deps)) = i.split_once('R') {
                for dep in deps.lines() {
                    let name = dep.trim_end_matches(|c: char| c.is_numeric());
                    stack.push(name.to_string());
                }
               // for e in name.lines() {
                    //if Path::new(format!("/var/lib/pkg/DB/{}", e)).exists() {
                    //    println!("{} is already installed", e);
                    //} else {
                    //    println!("You need to install {}", e);
                    //    std::process::exit(1)
                    //}    //env::set_current_dir(format!("/var/lib/pkg/DB/{}", e))
                //}
                //println!("{}", name);
            }
        }
        println!("{}", rawpkg);
    }
    //println!("{}", rawpkg);
    //Ok(())
    
}
