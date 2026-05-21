use crate::traits::cmd::Cmd;
use clap::Args;

#[derive(Args)]
pub struct ApplyArgs {}

impl Cmd for ApplyArgs {
    fn run(self) -> Result<(), String> {
        println!("apply subcommand called");
        Ok(())
    }
}
