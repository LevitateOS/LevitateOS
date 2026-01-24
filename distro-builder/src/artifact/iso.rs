//! ISO image builder.
//!
//! # Status: SKELETON
//!
//! The actual ISO building logic remains in leviso.
//! This module defines the interface for future extraction.

use anyhow::Result;
use std::path::Path;

/// Options for building an ISO image.
pub struct IsoOptions<'a> {
    /// Volume label (used for boot device detection)
    pub label: &'a str,
    /// OS name for GRUB menu
    pub os_name: &'a str,
    /// Kernel command line options
    pub cmdline: &'a str,
}

/// Build a UEFI-bootable ISO image.
///
/// # Arguments
/// * `iso_root` - Directory containing ISO contents
/// * `output` - Path for the output ISO file
/// * `options` - ISO build options
///
/// # Status: UNIMPLEMENTED
///
/// This is a placeholder. The actual implementation is in leviso.
pub fn build_iso(_iso_root: &Path, _output: &Path, _options: &IsoOptions) -> Result<()> {
    unimplemented!("ISO building not yet extracted from leviso")
}
