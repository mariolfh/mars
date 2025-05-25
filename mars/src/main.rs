use clap::{Parser, Subcommand};

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
    }
}

fn main() {
    let cli = Args::parse();
    
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot and astronaut, 2009"),
        Commands::String{operation, value} => stringtype(operation, value)
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


/// String Functions
fn uppercase(value: String) {
    let result = value.to_uppercase();
    println!("{}", result);
}

fn lowercase(value: String) {
    let result = value.to_lowercase();
    println!("{}", result);
}

fn size (value: String) {
    let result = value.size(value);
    println!("{}", result);
}