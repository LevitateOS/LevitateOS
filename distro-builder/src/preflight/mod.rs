//! Preflight checks for build validation.
//!
//! Validates that the host system has required tools before building.
//!
//! # Status: SKELETON
//!
//! Basic host tool checking is implemented.
//! More comprehensive validation remains in leviso.

use anyhow::{bail, Result};
use std::process::Command;

/// Check if a command exists on the host system.
pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Required host tools for building distribution ISOs.
pub const REQUIRED_TOOLS: &[(&str, &str)] = &[
    ("mksquashfs", "squashfs-tools"),
    ("xorriso", "xorriso"),
    ("mkfs.fat", "dosfstools"),
    ("mmd", "mtools"),
    ("mcopy", "mtools"),
    ("cpio", "cpio"),
    ("gzip", "gzip"),
];

/// Check that all required host tools are available.
pub fn check_host_tools() -> Result<()> {
    let mut missing = Vec::new();

    for (tool, package) in REQUIRED_TOOLS {
        if !command_exists(tool) {
            missing.push((*tool, *package));
        }
    }

    if !missing.is_empty() {
        let msg = missing
            .iter()
            .map(|(t, p)| format!("  {} (install: {})", t, p))
            .collect::<Vec<_>>()
            .join("\n");
        bail!("Missing required host tools:\n{}", msg);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_exists() {
        // 'ls' should exist on any Unix system
        assert!(command_exists("ls"));
        // Random garbage should not exist
        assert!(!command_exists("definitely_not_a_real_command_12345"));
    }
}
