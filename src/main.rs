use clap::{Parser, Subcommand};

mod cmd;
pub mod traits;

use cmd::apply::ApplyArgs;
use cmd::init::InitArgs;
use traits::cmd::Cmd;

#[derive(Parser)]
#[command(about, version, arg_required_else_help = true)]
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

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Apply(args) => args.run(),
        Commands::Init(args) => args.run(),
    }
}
