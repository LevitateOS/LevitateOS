//! Build orchestration module
//!
//! `TEAM_466`: Extracted from commands.rs during refactor.
//! High-level build coordination functions.

use anyhow::Result;

use crate::disk;

// TEAM_435: Replaced Eyra with c-gull sysroot approach
// TEAM_438: Uses apps registry for external app builds
// TEAM_444: Migrated to musl - much simpler now!
pub fn build_all(arch: &str) -> Result<()> {
    // Ensure musl target is installed (replaces sysroot build)
    super::sysroot::ensure_rust_musl_target(arch)?;

    // Build all external Rust apps (coreutils, brush, etc.) if not present
    super::apps::ensure_all_built(arch)?;

    // Build C apps if musl-gcc is available (optional)
    if super::c_apps::musl_gcc_available() {
        for app in super::c_apps::C_APPS {
            if !app.exists(arch) {
                // Don't fail build_all if C app build fails - it's optional
                if let Err(e) = app.build(arch) {
                    println!("⚠️  Optional C app {} failed to build: {}", app.name, e);
                }
            }
        }
    } else {
        println!("ℹ️  musl-gcc not found, skipping C apps (dash). Install musl-tools to enable.");
    }

    // TEAM_073: Build userspace first
    super::userspace::build_userspace(arch)?;
    // TEAM_451: Use BusyBox initramfs (replaces old init + dash + coreutils)
    super::initramfs::create_busybox_initramfs(arch)?;
    // TEAM_121: Ensure disk image is populated
    disk::install_userspace_to_disk(arch)?;

    super::kernel::build_kernel_with_features(&[], arch)
}

pub fn build_kernel_only(arch: &str) -> Result<()> {
    super::kernel::build_kernel_with_features(&[], arch)
}

/// Build kernel with verbose feature for behavior testing (Rule 4: Silence is Golden)
pub fn build_kernel_verbose(arch: &str) -> Result<()> {
    super::kernel::build_kernel_with_features(&["verbose"], arch)
}
