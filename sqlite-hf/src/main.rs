use clap::{Parser, Subcommand};
use sqlite_hf::{classify_lyrics, get_all_zeroshotcandidates, read_lyrics};

#[derive(Parser)]
#[command(name = "sqlite-hf", about = "Classify song lyrics by genre using Hugging Face zero-shot classification")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Classify lyrics from a file against genre candidates stored in SQLite
    Classify {
        /// Path to the lyrics text file
        #[arg(short, long, default_value = "lyrics.txt")]
        file: String,
    },
    /// List all genre candidates from the SQLite database
    Candidates,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Classify { file } => {
            println!("Reading lyrics from: {}", file);
            let lyrics = read_lyrics(&file);
            println!("Classifying {} lines of lyrics...", lyrics.len());
            let results = classify_lyrics(lyrics);
            for (i, labels) in results.iter().enumerate() {
                println!("\n--- Input {} ---", i + 1);
                for label in labels {
                    println!("  {:>10}: {:.4}", label.text, label.score);
                }
            }
        }
        Commands::Candidates => {
            let candidates = get_all_zeroshotcandidates();
            println!("Genre candidates in database:");
            for candidate in &candidates {
                println!("  - {}", candidate);
            }
        }
    }
}
