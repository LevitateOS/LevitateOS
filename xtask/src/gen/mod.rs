use anyhow::Result;
use clap::Subcommand;

use crate::GlobalArgs;

#[derive(Subcommand)]
pub enum GenCommand {
    /// Generate documentation
    Docs,
}

pub fn run(cmd: &GenCommand, _global: &GlobalArgs) -> Result<()> {
    match cmd {
        GenCommand::Docs => {
            println!("Documentation generation not yet implemented");
            Ok(())
        }
    }
}
