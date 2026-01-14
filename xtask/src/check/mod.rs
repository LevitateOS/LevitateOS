use anyhow::Result;
use clap::Subcommand;

use crate::GlobalArgs;

#[derive(Subcommand)]
pub enum CheckCommand {
    /// Check code formatting
    Fmt,

    /// Run clippy lints
    Lint,
}

pub fn run(cmd: &CheckCommand, _global: &GlobalArgs) -> Result<()> {
    match cmd {
        CheckCommand::Fmt => {
            println!("Format checking not yet implemented");
            Ok(())
        }
        CheckCommand::Lint => {
            println!("Linting not yet implemented");
            Ok(())
        }
    }
}
