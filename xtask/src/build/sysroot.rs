//! Sysroot management (simplified for musl)
//!
//! TEAM_444: Migrated from c-gull to musl.
//!
//! With musl, we use system-installed musl via standard Rust targets.
//! No need to build our own libc anymore!
//!
//! Old approach (c-gull):
//! - Clone c-ward repo
//! - Build libc-levitateos wrapper
//! - Copy libc.a to sysroot
//! - Use complex RUSTFLAGS
//!
//! New approach (musl):
//! - rustup target add x86_64-unknown-linux-musl
//! - cargo build --target x86_64-unknown-linux-musl
//! - That's it!

use anyhow::{Context, Result};
use std::process::Command;

/// Ensure musl target is installed for Rust builds
pub fn ensure_rust_musl_target(arch: &str) -> Result<()> {
    let target = match arch {
        "x86_64" => "x86_64-unknown-linux-musl",
        "aarch64" => "aarch64-unknown-linux-musl",
        _ => "x86_64-unknown-linux-musl",
    };

    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .context("Failed to run rustup")?;

    let installed = String::from_utf8_lossy(&output.stdout);
    if installed.contains(target) {
        return Ok(());
    }

    println!("📥 Installing Rust musl target: {}", target);
    let status = Command::new("rustup")
        .args(["target", "add", target])
        .status()
        .context("Failed to run rustup target add")?;

    if !status.success() {
        anyhow::bail!("Failed to install {} target", target);
    }

    Ok(())
}

/// Legacy function for backward compatibility
///
/// With musl, the "sysroot" is just the system musl, so this always returns true.
/// Code that checks sysroot_exists() before building will still work.
pub fn sysroot_exists() -> bool {
    true
}

/// Legacy function for backward compatibility
///
/// Previously built c-gull sysroot. Now just ensures musl target is installed.
pub fn build_sysroot(arch: &str) -> Result<()> {
    println!("ℹ️  Using system musl (no custom sysroot build needed)");
    ensure_rust_musl_target(arch)?;
    println!("✅ musl target ready for {}", arch);
    Ok(())
}
