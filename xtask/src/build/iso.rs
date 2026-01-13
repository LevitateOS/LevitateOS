//! ISO build module
//!
//! `TEAM_466`: Extracted from commands.rs during refactor.
//! Handles Limine ISO creation for `x86_64`.

use anyhow::{bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

/// `TEAM_283`: Build a bootable Limine ISO
// TEAM_435: Replaced Eyra with c-gull sysroot
// TEAM_444: Migrated to musl
pub fn build_iso(arch: &str) -> Result<()> {
    build_iso_internal(&[], arch, false)
}

/// `TEAM_286`: Build ISO with verbose feature for behavior testing
pub fn build_iso_verbose(arch: &str) -> Result<()> {
    build_iso_internal(&["verbose"], arch, false)
}

/// `TEAM_374`: Build ISO for testing with test initramfs
pub fn build_iso_test(arch: &str) -> Result<()> {
    build_iso_internal(&["verbose"], arch, true)
}

fn build_iso_internal(features: &[&str], arch: &str, use_test_initramfs: bool) -> Result<()> {
    if arch != "x86_64" {
        bail!("ISO build currently only supported for x86_64");
    }

    println!("💿 Building Limine ISO for {arch}...");

    // TEAM_438: Build sysroot and all external apps if not present
    // TEAM_444: Now just ensures musl target is installed
    super::sysroot::ensure_rust_musl_target(arch)?;
    super::apps::ensure_all_built(arch)?;

    super::userspace::build_userspace(arch)?;
    // TEAM_451: Always use BusyBox initramfs now
    super::initramfs::create_busybox_initramfs(arch)?;
    crate::disk::install_userspace_to_disk(arch)?;
    super::kernel::build_kernel_with_features(features, arch)?;

    let iso_root = PathBuf::from("iso_root");
    let boot_dir = iso_root.join("boot");

    // Clean and create staging directory
    if iso_root.exists() {
        std::fs::remove_dir_all(&iso_root)?;
    }
    std::fs::create_dir_all(&boot_dir)?;

    // 2. Copy components to ISO root
    let kernel_path = "crates/kernel/target/x86_64-unknown-none/release/levitate-kernel";
    // TEAM_374: Use test initramfs when in test mode
    let initramfs_path = if use_test_initramfs {
        "initramfs_test.cpio".to_string()
    } else {
        format!("initramfs_{arch}.cpio")
    };
    let limine_cfg_path = "limine.cfg";

    std::fs::copy(kernel_path, boot_dir.join("levitate-kernel"))
        .context("Failed to copy levitate-kernel to ISO boot dir")?;
    if std::path::Path::new(&initramfs_path).exists() {
        std::fs::copy(&initramfs_path, boot_dir.join("initramfs.cpio"))
            .context("Failed to copy initramfs to ISO boot dir")?;
    }
    std::fs::copy(limine_cfg_path, iso_root.join("limine.cfg"))
        .context("Failed to copy limine.cfg - ensure it exists in repo root")?;

    // 3. Download/Prepare Limine binaries if needed
    prepare_limine_binaries(&iso_root)?;

    // 4. Create ISO using xorriso
    let iso_file = "levitate.iso";
    let status = Command::new("xorriso")
        .args([
            "-as",
            "mkisofs",
            "-b",
            "limine-bios-cd.bin",
            "-no-emul-boot",
            "-boot-load-size",
            "4",
            "-boot-info-table",
            "--efi-boot",
            "limine-uefi-cd.bin",
            "-efi-boot-part",
            "--efi-boot-image",
            "--protective-msdos-label",
            &iso_root.to_string_lossy(),
            "-o",
            iso_file,
        ])
        .status()
        .context("Failed to run xorriso")?;

    if !status.success() {
        bail!("xorriso failed to create ISO");
    }

    println!("✅ ISO created: {iso_file}");
    Ok(())
}

fn prepare_limine_binaries(iso_root: &PathBuf) -> Result<()> {
    let limine_dir = PathBuf::from("limine-bin");
    let files = [
        "limine-bios-cd.bin",
        "limine-uefi-cd.bin",
        "limine-bios.sys",
    ];

    // TEAM_304: Check if all required files exist, not just directory
    let all_files_exist = files.iter().all(|f| limine_dir.join(f).exists());

    if !all_files_exist {
        println!("📥 Downloading Limine binaries (v7.x)...");
        std::fs::create_dir_all(&limine_dir)?;

        let base_url = "https://github.com/limine-bootloader/limine/raw/v7.x-binary/";

        for file in &files {
            let url = format!("{base_url}{file}");
            let output = limine_dir.join(file);
            println!("  Fetching {file}...");

            let status = Command::new("curl")
                .args(["-L", "-f", "-o", output.to_str().unwrap(), &url])
                .status()
                .context(format!("Failed to run curl for {file}"))?;

            if !status.success() {
                bail!("Failed to download {file} from {url}");
            }
        }
    }

    // Copy to ISO root for xorriso
    for file in &files {
        let src = limine_dir.join(file);
        let dst = iso_root.join(file);
        std::fs::copy(&src, &dst)
            .with_context(|| format!("Failed to copy {} to {}", src.display(), dst.display()))?;
    }

    Ok(())
}
