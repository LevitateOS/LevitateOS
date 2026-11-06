//! distro-builder is the producer of every product; xtask runs its binary
//! (the release hooks call that same binary back).

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

fn profile() -> &'static str {
    if cfg!(debug_assertions) { "debug" } else { "release" }
}

/// Build distro-builder with xtask's own profile and return its path.
pub fn binary(root: &Path) -> Result<PathBuf> {
    let mut build = Command::new("cargo");
    build
        .current_dir(root)
        .args(["build", "-q", "-p", "distro-builder", "--bin", "distro-builder"]);
    if profile() == "release" {
        build.arg("--release");
    }
    if !build.status().context("running cargo build")?.success() {
        bail!("building distro-builder failed");
    }
    Ok(crate::util::repo::cargo_target_dir(root)
        .join(profile())
        .join("distro-builder"))
}

/// Run distro-builder with the repo's tool environment.
pub fn run(root: &Path, args: &[&str]) -> Result<()> {
    run_with(root, args, true)
}

/// Run distro-builder without the tool environment (to install the tools).
pub fn run_bare(root: &Path, args: &[&str]) -> Result<()> {
    run_with(root, args, false)
}

fn run_with(root: &Path, args: &[&str], tools_env: bool) -> Result<()> {
    let bin = binary(root)?;
    let mut cmd = Command::new(&bin);
    cmd.current_dir(root).args(args);
    if tools_env {
        crate::util::tools_env::apply_to_command(&mut cmd, root)?;
    }
    let status = cmd
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        bail!("distro-builder {} failed ({status})", args.join(" "));
    }
    Ok(())
}
