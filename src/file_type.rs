use std::fs::metadata;
use std::fs;

pub fn file_type(list: &str) -> bool {
    //let metadata = fs::metadata(list)?;
    match fs::metadata(list) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
    //assert!(!metadata.is_dir());
}
