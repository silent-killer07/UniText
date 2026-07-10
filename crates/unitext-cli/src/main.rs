use clap::{Parser, Subcommand};
use unitext_string::UniString;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze text and show everything about it
    Analyze { text: String },
    /// Byte-level inspection of text
    Inspect { text: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { text } => {
            let us = UniString::new(text);
            println!("╔══════════════════════════════════════════════════════╗");
            println!("║  UniText Analysis Report                             ║");
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Input:           \"{}\"", text);
            println!("║  Graphemes:       {}", us.length());
            println!("╚══════════════════════════════════════════════════════╝");
        }
        Commands::Inspect { text } => {
            let us = UniString::new(text);
            println!("Graphemes: {}", us.length());
            println!("UTF-8 Bytes: {:?}", text.as_bytes());
        }
    }
}
