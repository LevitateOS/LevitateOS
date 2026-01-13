//! Alpine Linux rootfs builder
//!
//! TEAM_475: Downloads Alpine minirootfs and converts to initramfs CPIO.
//! Provides Alpine compatibility with OpenRC init system.

use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Command;

use super::initramfs::cpio::CpioArchive;

/// Alpine Linux version to use
const ALPINE_VERSION: &str = "3.21";
const ALPINE_RELEASE: &str = "3.21.2";

/// Get Alpine download URL for architecture
fn alpine_url(arch: &str) -> Result<String> {
    let alpine_arch = match arch {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => bail!("Unsupported architecture for Alpine: {arch}"),
    };

    Ok(format!(
        "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/alpine-minirootfs-{}-{}.tar.gz",
        ALPINE_VERSION, alpine_arch, ALPINE_RELEASE, alpine_arch
    ))
}

/// Get cache path for Alpine tarball
fn cache_path(arch: &str) -> PathBuf {
    PathBuf::from(format!("toolchain/alpine/alpine-minirootfs-{arch}.tar.gz"))
}

/// Ensure Alpine minirootfs is downloaded and cached
pub fn ensure_alpine_rootfs(arch: &str) -> Result<PathBuf> {
    let cache = cache_path(arch);

    if cache.exists() {
        let size = fs::metadata(&cache)?.len();
        println!("  Using cached Alpine rootfs ({:.1} MB)", size as f64 / 1_000_000.0);
        return Ok(cache);
    }

    // Create cache directory
    if let Some(parent) = cache.parent() {
        fs::create_dir_all(parent)?;
    }

    let url = alpine_url(arch)?;
    println!("  Downloading Alpine minirootfs...");
    println!("    URL: {url}");

    // Use curl to download (respects user's data concerns - shows progress)
    let status = Command::new("curl")
        .args(["-L", "-o"])
        .arg(&cache)
        .arg("--progress-bar")
        .arg(&url)
        .status()
        .context("Failed to run curl (is it installed?)")?;

    if !status.success() {
        // Clean up partial download
        let _ = fs::remove_file(&cache);
        bail!("Download failed");
    }

    let size = fs::metadata(&cache)?.len();
    println!("  Downloaded: {:.1} MB", size as f64 / 1_000_000.0);

    Ok(cache)
}

/// Extract Alpine tarball and convert to CPIO initramfs
pub fn build_alpine_initramfs(arch: &str) -> Result<PathBuf> {
    println!("Building Alpine initramfs for {arch}...");

    // 1. Ensure rootfs is downloaded
    let tarball = ensure_alpine_rootfs(arch)?;

    // 2. Create temp directory for extraction
    let extract_dir = PathBuf::from(format!("target/alpine-rootfs-{arch}"));
    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir)?;
    }
    fs::create_dir_all(&extract_dir)?;

    // 3. Extract tarball
    println!("  Extracting Alpine rootfs...");
    let status = Command::new("tar")
        .args(["-xzf"])
        .arg(&tarball)
        .args(["-C"])
        .arg(&extract_dir)
        .status()
        .context("Failed to extract tarball")?;

    if !status.success() {
        bail!("Extraction failed");
    }

    // 4. Apply LevitateOS overlay
    apply_overlay(&extract_dir)?;

    // 5. Ensure /init symlink exists (kernel expects rdinit=/init)
    let init_path = extract_dir.join("init");
    if !init_path.exists() {
        println!("  Creating /init -> /sbin/init symlink...");
        std::os::unix::fs::symlink("/sbin/init", &init_path)?;
    }

    // 6. Build CPIO archive
    println!("  Building CPIO archive...");
    let mut archive = CpioArchive::new();
    add_directory_recursive(&mut archive, &extract_dir, "")?;

    // 7. Write output
    let output_dir = PathBuf::from("target/initramfs");
    fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join(format!("{arch}.cpio"));

    let file = File::create(&output_path)?;
    let total_size = archive.write(file)?;

    println!("  Created: {} ({:.1} MB)", output_path.display(), total_size as f64 / 1_000_000.0);

    // 8. Copy to legacy location for backward compatibility
    let legacy_path = format!("initramfs_{arch}.cpio");
    fs::copy(&output_path, &legacy_path)?;
    println!("  Copied to: {legacy_path}");

    // 9. Clean up extract directory
    fs::remove_dir_all(&extract_dir)?;

    Ok(output_path)
}

/// Apply LevitateOS overlay files
fn apply_overlay(root: &Path) -> Result<()> {
    let overlay_dir = PathBuf::from("initramfs/alpine-overlay");

    if !overlay_dir.exists() {
        // Create default overlay files
        println!("  Creating default overlay...");
        create_default_overlay(&overlay_dir)?;
    }

    // Copy overlay files to extracted root
    if overlay_dir.exists() {
        println!("  Applying LevitateOS overlay...");
        copy_overlay(&overlay_dir, root)?;
    }

    Ok(())
}

/// Create default overlay files if none exist
fn create_default_overlay(overlay_dir: &Path) -> Result<()> {
    fs::create_dir_all(overlay_dir.join("etc"))?;
    fs::create_dir_all(overlay_dir.join("root"))?;

    // /etc/hostname
    fs::write(overlay_dir.join("etc/hostname"), "levitate\n")?;

    // /etc/motd
    fs::write(
        overlay_dir.join("etc/motd"),
        r#"
    __           _ __        __       ____  _____
   / /   ___ _  __(_) /_____ _/ /____  / __ \/ ___/
  / /   / _ \ | / / / __/ __ `/ __/ _ \/ / / /\__ \
 / /___/  __/ |/ / / /_/ /_/ / /_/  __/ /_/ /___/ /
/_____/\___/|___/_/\__/\__,_/\__/\___/\____//____/

Welcome to LevitateOS - Alpine Linux Compatible

"#,
    )?;

    // /root/hello.txt
    fs::write(
        overlay_dir.join("root/hello.txt"),
        "Hello from LevitateOS!\n",
    )?;

    Ok(())
}

/// Copy overlay directory to root
fn copy_overlay(overlay: &Path, root: &Path) -> Result<()> {
    for entry in walkdir(overlay)? {
        let entry = entry?;
        let rel_path = entry.strip_prefix(overlay)?;
        let dest = root.join(rel_path);

        if entry.is_dir() {
            fs::create_dir_all(&dest)?;
        } else if entry.is_file() {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&entry, &dest)?;
        }
    }
    Ok(())
}

/// Simple directory walker
fn walkdir(path: &Path) -> Result<Vec<Result<PathBuf, std::io::Error>>> {
    let mut results = Vec::new();
    walkdir_recursive(path, &mut results)?;
    Ok(results)
}

fn walkdir_recursive(path: &Path, results: &mut Vec<Result<PathBuf, std::io::Error>>) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            results.push(Ok(path.clone()));
            if path.is_dir() {
                walkdir_recursive(&path, results)?;
            }
        }
    }
    Ok(())
}

/// Recursively add directory contents to CPIO archive
fn add_directory_recursive(archive: &mut CpioArchive, root: &Path, prefix: &str) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(root)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        let archive_path = if prefix.is_empty() {
            name_str.to_string()
        } else {
            format!("{}/{}", prefix, name_str)
        };

        let metadata = path.symlink_metadata()?;
        let file_type = metadata.file_type();

        if file_type.is_symlink() {
            let target = fs::read_link(&path)?;
            archive.add_symlink(&archive_path, &target.to_string_lossy());
        } else if file_type.is_dir() {
            let mode = metadata.permissions().mode() & 0o7777;
            archive.add_directory(&archive_path, mode);
            add_directory_recursive(archive, &path, &archive_path)?;
        } else if file_type.is_file() {
            let mode = metadata.permissions().mode() & 0o7777;
            let data = fs::read(&path)?;
            archive.add_file(&archive_path, &data, mode);
        } else if file_type.is_block_device() {
            let mode = metadata.permissions().mode() & 0o7777;
            let rdev = metadata.rdev();
            let major = ((rdev >> 8) & 0xfff) as u32;
            let minor = (rdev & 0xff) as u32;
            archive.add_block_device(&archive_path, mode, major, minor);
        } else if file_type.is_char_device() {
            let mode = metadata.permissions().mode() & 0o7777;
            let rdev = metadata.rdev();
            let major = ((rdev >> 8) & 0xfff) as u32;
            let minor = (rdev & 0xff) as u32;
            archive.add_char_device(&archive_path, mode, major, minor);
        }
        // Skip sockets, fifos, etc.
    }

    Ok(())
}

/// Clean Alpine build artifacts
pub fn clean_alpine() -> Result<()> {
    println!("Cleaning Alpine build artifacts...");

    // Remove extracted rootfs
    for arch in ["x86_64", "aarch64"] {
        let extract_dir = PathBuf::from(format!("target/alpine-rootfs-{arch}"));
        if extract_dir.exists() {
            fs::remove_dir_all(&extract_dir)?;
        }
    }

    // Note: Don't remove cached tarballs to save bandwidth
    println!("  Note: Cached tarballs in toolchain/alpine/ preserved");

    Ok(())
}

/// Remove cached Alpine tarballs (use with caution - will re-download)
pub fn purge_alpine_cache() -> Result<()> {
    let cache_dir = PathBuf::from("toolchain/alpine");
    if cache_dir.exists() {
        println!("Removing Alpine cache...");
        fs::remove_dir_all(&cache_dir)?;
    }
    Ok(())
}
