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
    /// Perform a deep security/threat analysis
    Security { text: String },
    /// Compare two strings at multiple levels
    Compare { text1: String, text2: String },
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
        Commands::Security { text } => {
            let us = UniString::new(text);
            let safe = us.is_safe();
            println!("╔══════════════════════════════════════════════════════╗");
            println!("║  SECURITY ALERT                                      ║");
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Risk Level:    {}                              ║", if safe { "NONE ✅" } else { "HIGH 🔴" });
            println!("╚══════════════════════════════════════════════════════╝");
        }
        Commands::Compare { text1, text2 } => {
            let visually_equal = UniString::visually_equal(text1, text2);
            let bytes_equal = text1.as_bytes() == text2.as_bytes();
            println!("╔══════════════════════════════════════════════════════╗");
            println!("║  String Comparison Report                            ║");
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Byte-equal:      {}                              ║", if bytes_equal { "Yes ✅" } else { "No  ❌" });
            println!("║  Visual-equal:    {}                              ║", if visually_equal { "Yes ✅" } else { "No  ❌" });
            println!("╚══════════════════════════════════════════════════════╝");
        }
    }
}
