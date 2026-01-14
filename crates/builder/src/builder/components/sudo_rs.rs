//! sudo-rs builder.

use super::Buildable;
use crate::builder::vendor;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// sudo-rs component.
pub struct SudoRs;

impl Buildable for SudoRs {
    fn name(&self) -> &'static str {
        "sudo-rs"
    }

    fn build(&self) -> Result<()> {
        println!("=== Building sudo-rs ===");
        let src = vendor::require("sudo-rs")?;
        run_cargo(&src, &["build", "--release"])?;
        println!("  Built: vendor/sudo-rs/target/release/{{sudo,su}}");
        Ok(())
    }

    fn setuid_binaries(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("vendor/sudo-rs/target/release/sudo", "bin/sudo"),
            ("vendor/sudo-rs/target/release/su", "bin/su"),
        ]
    }
}

fn run_cargo(dir: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .current_dir(dir)
        .env("CARGO_UNSTABLE_WORKSPACES", "disable-inheritance")
        .status()
        .context("Failed to run cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }
    Ok(())
}
