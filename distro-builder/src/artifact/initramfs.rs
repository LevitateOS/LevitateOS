//! Initramfs builder.
//!
//! # Status: SKELETON
//!
//! The actual initramfs building logic remains in leviso.
//! This module defines the interface for future extraction.

use anyhow::Result;
use std::path::Path;

/// Options for building an initramfs.
pub struct InitramfsOptions<'a> {
    /// Busybox commands to symlink
    pub busybox_commands: &'a [&'a str],
    /// Boot modules to include
    pub boot_modules: &'a [&'a str],
    /// Gzip compression level (1-9)
    pub gzip_level: u8,
}

impl Default for InitramfsOptions<'_> {
    fn default() -> Self {
        Self {
            busybox_commands: &[],
            boot_modules: &[],
            gzip_level: 6,
        }
    }
}

/// Build an initramfs from components.
///
/// # Arguments
/// * `output_dir` - Directory to create initramfs in
/// * `options` - Initramfs build options
///
/// # Status: UNIMPLEMENTED
///
/// This is a placeholder. The actual implementation is in leviso.
pub fn build_initramfs(_output_dir: &Path, _options: &InitramfsOptions) -> Result<()> {
    unimplemented!("Initramfs building not yet extracted from leviso")
}
