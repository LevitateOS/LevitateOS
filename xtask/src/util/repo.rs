use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub fn repo_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .context("xtask is expected at <repo>/xtask")
}

/// Cargo's target directory for this checkout (`CARGO_TARGET_DIR`, else `<repo>/target`).
pub fn cargo_target_dir(root: &Path) -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"))
}

/// Build the workspace binary `bin` of `package` with xtask's own profile (debug or
/// release, as xtask was built) and return its path. Recipe, built by the builder,
/// follows the same rule.
pub fn cargo_bin(root: &Path, package: &str, bin: &str) -> Result<PathBuf> {
    let release = !cfg!(debug_assertions);
    let mut build = std::process::Command::new("cargo");
    build.current_dir(root).args(["build", "-q", "-p", package, "--bin", bin]);
    if release {
        build.arg("--release");
    }
    if !build.status().context("running cargo build")?.success() {
        anyhow::bail!("building {bin} failed");
    }
    Ok(cargo_target_dir(root).join(if release { "release" } else { "debug" }).join(bin))
}
