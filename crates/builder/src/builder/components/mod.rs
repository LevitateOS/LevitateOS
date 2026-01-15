//! Buildable components for `LevitateOS`.
//!
//! Each component implements the [`Buildable`] trait, providing a consistent
//! interface for building, and declaring artifacts (binaries, symlinks, runtime dirs).

use anyhow::Result;

pub mod glibc;
pub mod linux;
pub mod registry;

/// A buildable component for `LevitateOS`.
///
/// Components implement this trait to declare their build logic and artifacts.
/// The registry collects all components for orchestration.
pub trait Buildable: Send + Sync {
    /// Component name (used for CLI commands and display).
    fn name(&self) -> &'static str;

    /// Build the component from source.
    fn build(&self) -> Result<()>;

    /// Binaries to copy to initramfs: `(source_path, dest_path)`.
    fn binaries(&self) -> &'static [(&'static str, &'static str)] {
        &[]
    }

    /// Setuid binaries to copy (mode 4755): `(source_path, dest_path)`.
    fn setuid_binaries(&self) -> &'static [(&'static str, &'static str)] {
        &[]
    }

    /// Symlinks to create in `/bin`: `(link_name, target)`.
    fn symlinks(&self) -> &'static [(&'static str, &'static str)] {
        &[]
    }

    /// Runtime directories to copy: `(source, dest)`.
    fn runtime_dirs(&self) -> &'static [(&'static str, &'static str)] {
        &[]
    }

    /// Shared library paths produced by this component (for glibc collection).
    fn lib_paths(&self) -> Vec<&'static str> {
        vec![]
    }
}
