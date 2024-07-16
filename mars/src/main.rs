use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    com: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Welcome,
    Mars,
    Uppercase {
        value: String
    }
}

fn main() {
    let cli = Args::parse();
    
    match cli.com {
        Commands::Welcome => println!("Welcome to Mars!"),
        Commands::Mars => println!("“Mars is there, waiting to be reached.” -Buzz Aldrin, American pilot, and astronaut"),
        Commands::Uppercase{value} => uppercase(value)
    }
}

fn uppercase(value: String) {
    let result = value.to_uppercase();
    println!("{}", result);
}