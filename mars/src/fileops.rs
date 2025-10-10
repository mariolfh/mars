use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::io::{Write, Read};

fn writer() {
    let mut treasure: PathBuf = std::env::temp_dir();
    treasure.push("mars.txt");
    let mut file = File::create(treasure).unwrap();
    file.write_all(b"test");
    
    /*
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&treasure)?;
    writeln!(file, "This is how we progress")?;
    */
}

fn reader() {
    let mut treasure: PathBuf = env::temp_dir();
    treasure.push("mars.txt");
}