//! Kernel build module
//!
//! `TEAM_466`: Extracted from commands.rs during refactor.

use anyhow::{bail, Context, Result};
use std::process::Command;

/// Build kernel with specified features.
pub fn build_kernel_with_features(features: &[&str], arch: &str) -> Result<()> {
    println!("Building kernel for {arch}...");
    let target = match arch {
        "aarch64" => "aarch64-unknown-none",
        "x86_64" => "x86_64-unknown-none",
        _ => bail!("Unsupported architecture: {arch}"),
    };

    let mut args = vec![
        "build".to_string(),
        "-Z".to_string(),
        "build-std=core,alloc".to_string(),
        "--release".to_string(),
        "--target".to_string(),
        target.to_string(),
        "-p".to_string(),
        "levitate-kernel".to_string(), // TEAM_426: Only build kernel, not all workspace members
    ];

    if !features.is_empty() {
        args.push("--features".to_string());
        args.push(features.join(","));
    }

    // Kernel is its own workspace - build from kernel directory
    let status = Command::new("cargo")
        .current_dir("crates/kernel")
        .args(&args)
        .status()
        .context("Failed to run cargo build")?;

    if !status.success() {
        bail!("Kernel build failed");
    }

    // Convert to binary for boot protocol support (Rule 38)
    if arch == "aarch64" {
        println!("Converting to raw binary...");
        let objcopy_status = Command::new("aarch64-linux-gnu-objcopy")
            .args([
                "-O",
                "binary",
                "crates/kernel/target/aarch64-unknown-none/release/levitate-kernel",
                "kernel64_rust.bin",
            ])
            .status()
            .context("Failed to run objcopy - is aarch64-linux-gnu-objcopy installed?")?;

        if !objcopy_status.success() {
            bail!("objcopy failed");
        }
    } else {
        // x86_64 uses multiboot2 (ELF) directly or needs different conversion
        println!("x86_64 kernel build complete (ELF format for multiboot2)");
    }

    Ok(())
}
