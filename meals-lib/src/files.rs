use std::{fs, path::PathBuf};

pub fn read_to_str(p: PathBuf) -> String {
    match fs::read_to_string(p) {
        Ok(f) => f,
        Err(e) => panic!("Could not open file, {}", e)
    }
}
