//! Unit tests - runs cargo test on crates with std feature
//!
//! TEAM_030: Tests individual functions in isolation

use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn run() -> Result<()> {
    println!("=== Unit Tests ===\n");

    // Run unit tests for los_hal (has most tests)
    println!("Running los_hal unit tests...");
    let hal_status = Command::new("cargo")
        .args([
            "test",
            "-p", "los_hal",
            "--features", "std",
            "--target", "x86_64-unknown-linux-gnu",
        ])
        .status()
        .context("Failed to run los_hal tests")?;

    if !hal_status.success() {
        bail!("los_hal unit tests failed");
    }

    // Run unit tests for los_utils
    println!("\nRunning los_utils unit tests...");
    let utils_status = Command::new("cargo")
        .args([
            "test",
            "-p", "los_utils",
            "--features", "std",
            "--target", "x86_64-unknown-linux-gnu",
        ])
        .status()
        .context("Failed to run los_utils tests")?;

    if !utils_status.success() {
        bail!("los_utils unit tests failed");
    }

    println!("\n✅ All unit tests passed\n");
    Ok(())
}
