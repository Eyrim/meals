use std::{fs, io, path::PathBuf};

pub fn read_to_str(p: PathBuf) -> io::Result<String> {
    fs::read_to_string(p)
}
