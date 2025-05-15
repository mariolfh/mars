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
    /// returns a simple welcoming message to the program.
    Welcome,
    /// returns the inspirational quote that inspires the project.
    Mars,
    /// Converts all letters to uppercase in a given string.
    Uppercase {
        value: String
    },
    /// Converts all letters to lowercase in a given string.
    Lowercase {
        value: String
    }
}

fn main() {
    let cli = Args::parse();
    
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot and astronaut"),
        Commands::Uppercase{value} => uppercase(value),
        Commands::Lowercase{value} => lowercase(value)  
    }
}

fn uppercase(value: String) {
    let result = value.to_uppercase();
    println!("{}", result);
}

fn lowercase(value: String) {
    let result = value.to_lowercase();
    println!("{}", result);
}