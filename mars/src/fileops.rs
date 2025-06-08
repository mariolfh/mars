use std::fs::File;
use std::path::PathBuf;
use std::env;
use std::io::{self, Write, Read};

fn writer() -> io::Result<()> {
    let mut treasure: PathBuf = env::temp_dir();
    treasure.push("mars.txt");
    let mut file = File::create(treasure).unwrap();
    file.write(b"test");
    
    //let mut file = OpenOptions::new()
    //    .write(true)
    //    .read(true)
    //    .create(true)
    //    .open(&treasure)?;
    //writeln!(file, "This is how we progress")?;
    Ok(())
}

fn reader() {
    let mut treasure: PathBuf = env::temp_dir();
    treasure.push("mars.txt");
}