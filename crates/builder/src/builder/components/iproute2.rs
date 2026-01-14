//! iproute2 networking utilities builder.

use super::Buildable;
use crate::builder::vendor;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// iproute2 networking utilities component.
/// Provides: ip, ss, bridge, tc, etc.
pub struct Iproute2;

impl Buildable for Iproute2 {
    fn name(&self) -> &'static str {
        "iproute2"
    }

    fn build(&self) -> Result<()> {
        println!("=== Building iproute2 ===");

        let src = vendor::require("iproute2")?;

        // iproute2 uses ./configure && make
        if !src.join("config.mk").exists() {
            run_cmd("./configure", &[], &src)?;
        }

        run_cmd("make", &["-j4"], &src)?;

        println!("  Built: vendor/iproute2/");
        Ok(())
    }

    fn binaries(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("vendor/iproute2/ip/ip", "sbin/ip"),
            ("vendor/iproute2/misc/ss", "bin/ss"),
            ("vendor/iproute2/bridge/bridge", "sbin/bridge"),
        ]
    }
}

fn run_cmd(cmd: &str, args: &[&str], dir: &Path) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .context(format!("Failed to run {cmd}"))?;

    if !status.success() {
        bail!("{cmd} failed");
    }
    Ok(())
}
