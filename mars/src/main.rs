use clap::{Parser, Subcommand};
use arboard::Clipboard;
mod fileops;

/// A CLI application containing multiple useful functions and scripts

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    com: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// returns a simple welcoming message to the user.
    Welcome,
    /// returns the inspirational quote that inspires the project along with the author of the quote.
    Mars,
    /// Allows operations on Strings
    String {
        operation: String,
        value: String
    },
    /// Copies last used String to the system's clipboard
    Copy
}

fn main() { 
    let cli = Args::parse();
    let mut copied = String::new();
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot and astronaut, 2009"),
        Commands::String{operation, value} => stringtype(operation, value, &mut copied),
        Commands::Copy => copy(&copied)
    }
}

fn stringtype(operation: String, value: String, copied: &mut String) {
    if operation == "uppercase"{
        uppercase(value, copied);
    }
    else if operation == "lowercase"{
        lowercase(value, copied);
    }
    else if operation == "size"{
        size(value, copied);
    }
}

fn copy (copied: &String){
    let mut clippie = Clipboard::new().unwrap();
    clippie.set_text(copied.clone()).unwrap();
    println!("Copied to clipboard: {}", copied)
}

fn copify(copied: &mut String, value: String){
    *copied = value;
}

/// String Functions
fn uppercase(value: String, copied: &mut String) {
    let result = value.to_uppercase();
    println!("{}", result);
    copify(copied, result);
}

fn lowercase(value: String, copied: &mut String) {
    let result = value.to_lowercase();
    println!("{}", result);
    copify(copied, result);
}

fn size (value: String, copied: &mut String) {
    let result = value.len().to_string();
    println!("{}", result);
    copify(copied, result);
}