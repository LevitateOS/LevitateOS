//! procps-ng process utilities builder.

use super::Buildable;
use crate::builder::vendor;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// procps-ng process utilities component.
/// Provides: ps, top, free, vmstat, pgrep, pkill, watch, pmap, pwdx
pub struct Procps;

impl Buildable for Procps {
    fn name(&self) -> &'static str {
        "procps-ng"
    }

    fn build(&self) -> Result<()> {
        println!("=== Building procps-ng ===");

        let src = vendor::require("procps-ng")?;

        // procps-ng uses autotools
        // Run autogen.sh if configure doesn't exist
        if !src.join("configure").exists() {
            run_cmd("./autogen.sh", &[], &src)?;
        }

        // Configure with minimal features
        if !src.join("Makefile").exists() {
            run_cmd(
                "./configure",
                &[
                    "--disable-nls",
                    "--disable-modern-top",
                    "--without-systemd",
                    "--without-ncurses",
                ],
                &src,
            )?;
        }

        // Build
        run_cmd("make", &["-j4"], &src)?;

        println!("  Built: vendor/procps-ng/");
        Ok(())
    }

    fn binaries(&self) -> &'static [(&'static str, &'static str)] {
        // Use actual binaries from .libs/, not libtool wrapper scripts
        &[
            ("vendor/procps-ng/src/ps/.libs/pscommand", "bin/ps"),
            ("vendor/procps-ng/src/.libs/free", "bin/free"),
            ("vendor/procps-ng/src/.libs/vmstat", "bin/vmstat"),
            ("vendor/procps-ng/src/.libs/pgrep", "bin/pgrep"),
            ("vendor/procps-ng/src/.libs/pkill", "bin/pkill"),
            ("vendor/procps-ng/src/.libs/pmap", "bin/pmap"),
            ("vendor/procps-ng/src/.libs/uptime", "bin/uptime"),
            ("vendor/procps-ng/src/.libs/pidof", "bin/pidof"),
            ("vendor/procps-ng/src/.libs/w", "bin/w"),
            // These are statically linked, no .libs wrapper
            ("vendor/procps-ng/src/kill", "bin/kill"),
            ("vendor/procps-ng/src/pwdx", "bin/pwdx"),
            ("vendor/procps-ng/src/sysctl", "sbin/sysctl"),
        ]
    }

    fn lib_paths(&self) -> Vec<&'static str> {
        vec!["vendor/procps-ng/library/.libs/libproc2.so.0.0.2"]
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
