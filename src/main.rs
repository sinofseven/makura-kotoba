use clap::{Parser, Subcommand};

mod cmd;

use cmd::{apply_command, init_command, ApplyArgs, InitArgs};

#[derive(Parser)]
#[command(name = "makura-kotoba")]
#[command(about = "A CLI tool for managing environment variables")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate wrapper scripts for applying environment variables
    Apply(ApplyArgs),
    /// Generate shell initialization scripts
    Init(InitArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply(args) => apply_command(args),
        Commands::Init(args) => init_command(args),
    }
}
