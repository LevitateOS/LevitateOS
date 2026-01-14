use anyhow::Result;
use clap::Subcommand;

use crate::GlobalArgs;

mod alpine;
mod helpers;

#[derive(Subcommand)]
pub enum TestCommand {
    /// Run golden file tests
    Golden,

    /// Run integration tests
    Integration,
}

pub fn run(cmd: &TestCommand, global: &GlobalArgs) -> Result<()> {
    match cmd {
        TestCommand::Golden => run_golden_tests(global),
        TestCommand::Integration => run_integration_tests(global),
    }
}

fn run_golden_tests(global: &GlobalArgs) -> Result<()> {
    crate::common::info_println(global.quiet, "Running golden file tests...");
    println!("Golden tests not yet implemented");
    Ok(())
}

fn run_integration_tests(global: &GlobalArgs) -> Result<()> {
    crate::common::info_println(
        global.quiet,
        "Running integration tests with Alpine ISO...",
    );
    alpine::run_all()
}
