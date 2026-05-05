use std::fs;



pub fn depends(pkg: &str) {
    let path = format!("/var/lib/pkg/DB/{}/META", pkg);
    let file = fs::read_to_string(format!("/var/lib/pkg/DB/{}/META", pkg)).unwrap();
    // Adding vect to be able to read properly, without this it would be unable to read if the order isn't respected
   // let mut content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    //let deps = content.lines().filter(|l| l.starts_with('R'));
    for i in file.lines().filter(|l| l.starts_with('R')) {
        if i.is_empty() {
            continue
        }
        if let Some((_, i)) = i.split_once('R') {
        //let depends = i.split_once('R');
            let name = i.trim_end_matches(|c: char| c.is_numeric());
            println!("{}", name);
            for s in name.lines()
        }
    }
    
}