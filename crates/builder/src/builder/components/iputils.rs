//! iputils networking utilities builder.

use super::Buildable;
use crate::builder::vendor;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// iputils networking utilities component.
/// Provides: ping, tracepath, arping, clockdiff
pub struct Iputils;

impl Buildable for Iputils {
    fn name(&self) -> &'static str {
        "iputils"
    }

    fn build(&self) -> Result<()> {
        println!("=== Building iputils ===");

        let src = vendor::require("iputils")?;
        let build_dir = src.join("build");

        std::fs::create_dir_all(&build_dir)?;

        // iputils uses meson
        // Disable features that require extra dependencies
        let meson_args = if build_dir.join("build.ninja").exists() {
            vec![
                "setup", "build", ".", "--reconfigure",
                "-DUSE_CAP=false",
                "-DUSE_IDN=false",
                "-DBUILD_ARPING=false",
                "-DBUILD_CLOCKDIFF=false",
                "-DBUILD_MANS=false",
            ]
        } else {
            vec![
                "setup", "build", ".",
                "-DUSE_CAP=false",
                "-DUSE_IDN=false",
                "-DBUILD_ARPING=false",
                "-DBUILD_CLOCKDIFF=false",
                "-DBUILD_MANS=false",
            ]
        };

        run_cmd("meson", &meson_args, &src)?;
        run_cmd("ninja", &["-C", "build"], &src)?;

        println!("  Built: vendor/iputils/build/");
        Ok(())
    }

    fn binaries(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("vendor/iputils/build/ping/ping", "bin/ping"),
            ("vendor/iputils/build/tracepath", "bin/tracepath"),
        ]
    }

    fn setuid_binaries(&self) -> &'static [(&'static str, &'static str)] {
        // ping needs setuid for raw sockets (or CAP_NET_RAW)
        &[]
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
