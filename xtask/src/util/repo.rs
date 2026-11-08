use anyhow::{Context, Result};
use std::fs;
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

/// The repo's QEMU tools (`cargo xtask setup`), or `None` when they are not
/// installed and the host's QEMU and OVMF are used instead.
pub fn repo_tools(root: &Path) -> Result<Option<PathBuf>> {
    let selection = root.join(".artifacts/tools/current");
    match fs::canonicalize(&selection) {
        Ok(published) if published.is_dir() => Ok(Some(published)),
        Ok(_) => Err(crate::util::exit::missing(
            format!("the QEMU tools selection at {} is not a directory", selection.display()),
            "remove .artifacts/tools and run cargo xtask setup",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(crate::util::exit::missing(
                format!(
                    "the QEMU tools at {} are not readable by this user ({error})",
                    selection.display()
                ),
                "run as the user that installed them, or remove .artifacts/tools and run cargo xtask setup",
            ))
        }
        Err(_) => Ok(None),
    }
}

/// UEFI firmware: the repo tools' OVMF, else the host's (`OVMF_PATH` or the
/// installed edk2/ovmf package).
pub fn ovmf_path(root: &Path) -> Result<PathBuf> {
    if let Some(tools) = repo_tools(root)? {
        return Ok(tools.join("usr/share/edk2/ovmf/OVMF_CODE.fd"));
    }
    recqemu::find_ovmf().map_err(|_| {
        crate::util::exit::missing(
            "no OVMF firmware: the repo QEMU tools are not installed and the host has none",
            "cargo xtask setup (or install the host's edk2-ovmf / ovmf package)",
        )
    })
}
