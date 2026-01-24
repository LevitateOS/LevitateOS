//! Filesystem operations for building distribution images.
//!
//! These are distro-agnostic filesystem utilities.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Create essential FHS directory structure.
///
/// Creates the basic directory layout needed for a Linux filesystem:
/// - /usr/{bin,sbin,lib,lib64}
/// - /var/{log,tmp,cache,spool}
/// - /etc
/// - /tmp, /root, /home, /mnt
/// - /run, /run/lock
/// - /proc, /sys, /dev, /dev/pts
pub fn create_fhs_dirs(root: &Path) -> Result<()> {
    let dirs = [
        "usr/bin",
        "usr/sbin",
        "usr/lib",
        "usr/lib64",
        "var/log",
        "var/tmp",
        "var/cache",
        "var/spool",
        "etc",
        "tmp",
        "root",
        "home",
        "mnt",
        "run",
        "run/lock",
        "proc",
        "sys",
        "dev",
        "dev/pts",
    ];

    for dir in &dirs {
        let path = root.join(dir);
        if !path.exists() {
            fs::create_dir_all(&path)
                .with_context(|| format!("Failed to create {}", path.display()))?;
        }
    }

    Ok(())
}

/// Create merged /usr symlinks.
///
/// Creates:
/// - /bin -> usr/bin
/// - /sbin -> usr/sbin
/// - /lib -> usr/lib
/// - /lib64 -> usr/lib64
pub fn create_merged_usr_symlinks(root: &Path) -> Result<()> {
    let symlinks = [
        ("bin", "usr/bin"),
        ("sbin", "usr/sbin"),
        ("lib", "usr/lib"),
        ("lib64", "usr/lib64"),
    ];

    for (link, target) in &symlinks {
        let link_path = root.join(link);
        if link_path.exists() && !link_path.is_symlink() {
            fs::remove_dir_all(&link_path)?;
        }
        if !link_path.exists() {
            std::os::unix::fs::symlink(target, &link_path)
                .with_context(|| format!("Failed to create /{} symlink", link))?;
        }
    }

    Ok(())
}

/// Create /var symlinks for merged /usr layout.
///
/// Creates:
/// - /var/run -> /run
/// - /var/lock -> /run/lock
pub fn create_var_symlinks(root: &Path) -> Result<()> {
    let var_dir = root.join("var");
    if !var_dir.exists() {
        fs::create_dir_all(&var_dir).context("Failed to create /var")?;
    }

    let var_run = root.join("var/run");
    if !var_run.exists() && !var_run.is_symlink() {
        std::os::unix::fs::symlink("/run", &var_run)
            .context("Failed to create /var/run symlink")?;
    }

    let var_lock = root.join("var/lock");
    if !var_lock.exists() && !var_lock.is_symlink() {
        std::os::unix::fs::symlink("/run/lock", &var_lock)
            .context("Failed to create /var/lock symlink")?;
    }

    Ok(())
}

/// Create a complete FHS structure with merged /usr.
///
/// Combines create_fhs_dirs, create_merged_usr_symlinks, and create_var_symlinks.
pub fn create_fhs_structure(root: &Path) -> Result<()> {
    create_fhs_dirs(root)?;
    create_merged_usr_symlinks(root)?;
    create_var_symlinks(root)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_fhs_dirs() {
        let temp = TempDir::new().unwrap();
        create_fhs_dirs(temp.path()).unwrap();

        assert!(temp.path().join("usr/bin").exists());
        assert!(temp.path().join("usr/sbin").exists());
        assert!(temp.path().join("etc").exists());
        assert!(temp.path().join("var/log").exists());
    }

    #[test]
    fn test_create_merged_usr_symlinks() {
        let temp = TempDir::new().unwrap();
        create_fhs_dirs(temp.path()).unwrap();
        create_merged_usr_symlinks(temp.path()).unwrap();

        assert!(temp.path().join("bin").is_symlink());
        assert!(temp.path().join("sbin").is_symlink());
    }

    #[test]
    fn test_create_var_symlinks() {
        let temp = TempDir::new().unwrap();
        create_fhs_dirs(temp.path()).unwrap();
        create_var_symlinks(temp.path()).unwrap();

        assert!(temp.path().join("var/run").is_symlink());
        assert!(temp.path().join("var/lock").is_symlink());
    }
}
