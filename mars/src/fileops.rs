use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::io::{self, Write, Read};

/// Copy related file operations
pub fn get_cpath() -> PathBuf{
    let mut cpath = env::temp_dir();
    cpath.push("mars-copy.txt");
    cpath
}

pub fn cpath_exists() -> io::Result<()> {
    let path = get_cpath();
    if !path.exists() {
        File::create(&path)?;
    }
    Ok(())
}

pub fn save_copier(value: &str) -> io::Result<()> {
    let path = get_cpath();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn get_copier() -> io::Result<String> {
    let path = get_cpath();
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    Ok(content)
}