//! The SSH key scenario guests trust: the repo key in `.artifacts/ssh`
//! unless a key is given, so runs never depend on `~/.ssh`.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn repo_key_path(root: &Path) -> PathBuf {
    root.join(".artifacts/ssh/id_ed25519")
}

/// The repo key, created on first use.
pub fn ensure_repo_key(root: &Path) -> Result<PathBuf> {
    let key = repo_key_path(root);
    if key.is_file() && key.with_extension("pub").is_file() {
        return Ok(key);
    }
    let dir = key.parent().expect("key path has a parent");
    std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let _ = std::fs::remove_file(&key);
    let _ = std::fs::remove_file(key.with_extension("pub"));
    let status = Command::new("ssh-keygen")
        .args(["-q", "-t", "ed25519", "-N", "", "-C", "levitate-dev", "-f"])
        .arg(&key)
        .status()
        .map_err(|_| {
            crate::util::exit::missing(
                "ssh-keygen is not installed (needed to create the repo SSH key)",
                "install OpenSSH's client tools",
            )
        })?;
    if !status.success() {
        bail!("ssh-keygen failed creating {}", key.display());
    }
    eprintln!("[info] created the repo SSH key {}", key.display());
    Ok(key)
}

/// The private key to use (given, or the repo key) and its public half.
pub fn resolve(root: &Path, given: Option<&Path>) -> Result<(PathBuf, String)> {
    let key = match given {
        Some(path) => path.to_path_buf(),
        None => ensure_repo_key(root)?,
    };
    let public_path = PathBuf::from(format!("{}.pub", key.display()));
    if !key.is_file() {
        bail!("SSH private key {} does not exist", key.display());
    }
    let public = std::fs::read_to_string(&public_path)
        .with_context(|| format!("reading the public key {}", public_path.display()))?;
    Ok((key, public.trim().to_string()))
}
