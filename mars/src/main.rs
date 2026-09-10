use clap::{Parser, Subcommand};
mod clipboard;
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
    Copy,
    /// Pastes content of the system's clipboard into Mars
    Paste
}

fn main() { 
    fileops::file_exists().unwrap();
    let cli = Args::parse();
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot and astronaut, 2009"),
        Commands::String{operation, value} => stringtype(operation, value),
        Commands::Copy => copying(),
        Commands::Paste => pasting()
    }
}

fn stringtype(operation: String, value: String) {
    if operation == "uppercase"{
        uppercase(value);
    }
    else if operation == "lowercase"{
        lowercase(value);
    }
    else if operation == "size"{
        size(value);
    }
}

fn copying(){
    let copier = fileops::get_copier();
    clipboard::copy(copier.unwrap());
    println!("Last used element copied from Mars to the clipboard.");
}

fn pasting(){
    let paster = clipboard::paste();
    fileops::set_copier(&paster).unwrap();
    println!("Clipboard contents pasted into Mars from the clipboard.");
}

/// String Functions
fn uppercase(value: String) {
    let result = value.to_uppercase();
    println!("{}", result);
    fileops::set_copier(&result).unwrap();
}

fn lowercase(value: String) {
    let result = value.to_lowercase();
    println!("{}", result);
    fileops::set_copier(&result).unwrap();
}

fn size (value: String) {
    let result = value.len().to_string();
    println!("{}", result);
    fileops::set_copier(&result).unwrap();
}