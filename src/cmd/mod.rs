use clap::Parser;

pub mod apply;
pub mod init;

#[derive(Parser)]
pub struct ApplyArgs {}

#[derive(Parser)]
pub struct InitArgs {}

pub use apply::apply_command;
pub use init::init_command;
