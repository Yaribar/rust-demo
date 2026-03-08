use clap::{Parser, Subcommand};

// A command-line tool to play Marco Polo
#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Yarib",
    about = "A command-line tool to play Marco Polo"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]  // ← was Parser, should be Subcommand
enum Commands {
    #[clap(version = "1.0", author = "Yarib")]
    /// Play the Marco Polo game
    Play {
        #[clap(short, long)]
        /// The name to play with
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Play { name }) => {
            let response = marco_polo(name);
            println!("{}", response);
        }
        None => {
            println!("Please provide a command. Use --help for more information.");
        }
    }
}

pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "Whats your name?".to_string()
    }
}