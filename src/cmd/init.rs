use crate::traits::cmd::Cmd;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {}

impl Cmd for InitArgs {
    fn run(self) -> Result<(), String> {
        println!("init subcommand called");
        Ok(())
    }
}
