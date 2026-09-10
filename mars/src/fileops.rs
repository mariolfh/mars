use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::env;
use std::io::{self, Write, Read};

pub fn get_file() -> PathBuf{
    let mut file_path = env::temp_dir();
    file_path.push("mars.txt");
    file_path
}

pub fn file_exists() -> io::Result<()> {
    let mut file_path = env::temp_dir();
    file_path.push("mars.txt");
    if !file_path.exists() {
        File::create(&file_path)?;
    }
    Ok(())
}

pub fn set_copier(value: &str) -> io::Result<()> {
    let path = get_file();
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn get_copier() -> io::Result<String> {
    let path = get_file();
    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}