//! Arch Linux ISO extraction and path helpers.
//!
//! Uses a pinned Arch Linux ISO as a reference for studying distro internals.
//! The ISO contains a squashfs image (airootfs.sfs) with the live root filesystem.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Arch Linux ISO configuration.
const ARCH_ISO: &str = "archlinux-x86_64.iso";
const ARCH_SHA256: &str = "16502a7c18eed827ecead95c297d26f9f4bd57c4b3e4a8f4e2b88cf60e412d6f";

/// Paths relative to workspace root.
const ISO_PATH: &str = "vendor/images/archlinux-x86_64.iso";
const ARCH_ROOT: &str = "vendor/arch-root";

/// Get the path to the extracted Arch root filesystem.
pub fn root() -> PathBuf {
    PathBuf::from(ARCH_ROOT)
}

/// Ensure the Arch root filesystem is extracted.
/// Returns the path to the extracted root.
pub fn ensure_extracted() -> Result<PathBuf> {
    let root_path = root();

    // Check if already extracted
    if root_path.join("usr/bin/bash").exists() {
        println!("Arch root already extracted at {}", root_path.display());
        return Ok(root_path);
    }

    println!("=== Extracting Arch Linux root filesystem ===");

    // Verify ISO exists
    let iso_path = Path::new(ISO_PATH);
    if !iso_path.exists() {
        bail!(
            "Arch Linux ISO not found at {}\n\
             Download from: https://archlinux.org/download/\n\
             Expected: {}\n\
             SHA256: {}",
            ISO_PATH, ARCH_ISO, ARCH_SHA256
        );
    }

    // Verify ISO checksum
    verify_iso_checksum(iso_path)?;

    // Extract the ISO and squashfs
    extract_arch_root(iso_path, &root_path)?;

    Ok(root_path)
}

/// Verify the ISO checksum matches expected.
fn verify_iso_checksum(iso_path: &Path) -> Result<()> {
    println!("  Verifying ISO checksum...");

    let output = Command::new("sha256sum")
        .arg(iso_path)
        .output()
        .context("Failed to run sha256sum")?;

    if !output.status.success() {
        bail!("sha256sum failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let checksum_output = String::from_utf8_lossy(&output.stdout);
    let computed_hash = checksum_output
        .split_whitespace()
        .next()
        .context("Failed to parse sha256sum output")?;

    if computed_hash != ARCH_SHA256 {
        bail!(
            "ISO checksum mismatch!\n\
             Expected: {}\n\
             Got: {}\n\
             The ISO may be corrupted or a different version.",
            ARCH_SHA256, computed_hash
        );
    }

    println!("  Checksum verified: {}", &ARCH_SHA256[..16]);
    Ok(())
}

/// Extract the Arch root filesystem from the ISO.
fn extract_arch_root(iso_path: &Path, root_path: &Path) -> Result<()> {
    // Create temp directory for ISO extraction
    let temp_dir = PathBuf::from("build/arch-extract-temp");

    // Clean up any previous extraction attempt
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }
    std::fs::create_dir_all(&temp_dir)?;

    // Step 1: Extract airootfs.sfs from ISO using 7z (no sudo needed)
    // Arch structure: arch/x86_64/airootfs.sfs
    println!("  Extracting airootfs.sfs from ISO...");
    let output = Command::new("7z")
        .args(["x", "-y", &format!("-o{}", temp_dir.display())])
        .arg(iso_path)
        .arg("arch/x86_64/airootfs.sfs")
        .output()
        .context("Failed to run 7z - is p7zip installed?")?;

    if !output.status.success() {
        // Try bsdtar as fallback
        println!("  7z failed, trying bsdtar...");
        let output = Command::new("bsdtar")
            .args(["-xf", &iso_path.to_string_lossy()])
            .args(["-C", &temp_dir.to_string_lossy()])
            .arg("arch/x86_64/airootfs.sfs")
            .output()
            .context("Failed to run bsdtar")?;

        if !output.status.success() {
            bail!(
                "Failed to extract airootfs.sfs from ISO.\n\
                 Install one of: p7zip, bsdtar\n\
                 Error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    let airootfs_sfs = temp_dir.join("arch/x86_64/airootfs.sfs");
    if !airootfs_sfs.exists() {
        bail!("airootfs.sfs not found in ISO at arch/x86_64/airootfs.sfs");
    }

    // Step 2: Extract squashfs using unsquashfs
    println!("  Extracting root filesystem from squashfs...");

    if root_path.exists() {
        std::fs::remove_dir_all(root_path)?;
    }

    let output = Command::new("unsquashfs")
        .arg("-no-xattrs") // Skip xattrs (may require root otherwise)
        .args(["-d", &root_path.to_string_lossy()])
        .arg(&airootfs_sfs)
        .output()
        .context("Failed to run unsquashfs - is squashfs-tools installed?")?;

    if !output.status.success() {
        bail!(
            "unsquashfs failed: {}\n\
             Install squashfs-tools package.",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Fix permissions (squashfs preserves original perms which may be restrictive)
    println!("  Fixing permissions...");
    let chmod_output = Command::new("chmod")
        .args(["-R", "u+rwX"])
        .arg(root_path)
        .output()
        .context("Failed to fix permissions")?;

    if !chmod_output.status.success() {
        println!("  Warning: chmod failed, some files may be inaccessible");
    }

    // Verify extraction succeeded
    if !root_path.join("usr/bin/bash").exists() {
        bail!("Extraction failed - /usr/bin/bash not found in extracted root");
    }

    // Clean up temp directory
    std::fs::remove_dir_all(&temp_dir)?;

    // Show extraction stats
    let bin_count = std::fs::read_dir(root_path.join("usr/bin"))
        .map(|d| d.count())
        .unwrap_or(0);
    let lib_count = std::fs::read_dir(root_path.join("usr/lib"))
        .map(|d| d.count())
        .unwrap_or(0);

    println!(
        "  Extracted Arch root: ~{} binaries, ~{} libraries",
        bin_count, lib_count
    );

    Ok(())
}
