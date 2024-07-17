use clap::{Parser, Subcommand};

/// A CLI application containing multiple most used functions and scripts

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    com: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// shows a welcome message
    Welcome,
    /// Shows the quote that inspires the project
    Mars,
    /// Converts a string into uppercase
    Uppercase {
        value: String
    },
    /// Converts a string into lowercase
    Lowercase {
        value: String
    }
}

fn main() {
    let cli = Args::parse();
    
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot, and astronaut"),
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