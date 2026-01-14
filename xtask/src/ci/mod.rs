use anyhow::Result;
use clap::Subcommand;

use crate::GlobalArgs;

#[derive(Subcommand)]
pub enum CiCommand {
    /// Run all CI checks
    All,
}

pub fn run(cmd: &CiCommand, _global: &GlobalArgs) -> Result<()> {
    match cmd {
        CiCommand::All => {
            println!("CI workflow not yet implemented");
            Ok(())
        }
    }
}
