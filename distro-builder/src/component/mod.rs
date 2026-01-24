//! Component system for declarative system building.
//!
//! Components are defined as data structures that describe WHAT needs
//! to happen, not HOW. An executor interprets these definitions.
//!
//! # Distro-Agnostic vs Distro-Specific
//!
//! This module contains ONLY the distro-agnostic parts:
//! - `Installable` trait - anything that can be installed
//! - `Phase` enum - build ordering
//! - Generic `Op` variants - directory, file, symlink, user/group
//!
//! Distro-specific operations (systemd units, OpenRC services) are
//! defined in the distro crates (leviso, AcornOS).

use std::fmt;

/// Trait for anything that can be installed by an executor.
pub trait Installable {
    /// Name for logging.
    fn name(&self) -> &str;
    /// Build phase for ordering.
    fn phase(&self) -> Phase;
    /// Generate the operations to perform.
    fn ops(&self) -> Vec<Op>;
}

/// Build phases determine component ordering.
///
/// Components are sorted by phase before execution to ensure
/// dependencies are satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Phase {
    /// Create FHS directories and merged-usr symlinks.
    Filesystem = 1,
    /// Copy shells, coreutils, essential binaries.
    Binaries = 2,
    /// Init system setup (systemd for LevitateOS, OpenRC for AcornOS).
    Init = 3,
    /// Message bus (dbus).
    MessageBus = 4,
    /// System services (network, time, ssh).
    Services = 5,
    /// /etc configuration files.
    Config = 6,
    /// Package manager, bootloader tools.
    Packages = 7,
    /// Firmware and hardware support.
    Firmware = 8,
    /// Final cleanup and setup.
    Final = 9,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Phase::Filesystem => write!(f, "Filesystem"),
            Phase::Binaries => write!(f, "Binaries"),
            Phase::Init => write!(f, "Init"),
            Phase::MessageBus => write!(f, "MessageBus"),
            Phase::Services => write!(f, "Services"),
            Phase::Config => write!(f, "Config"),
            Phase::Packages => write!(f, "Packages"),
            Phase::Firmware => write!(f, "Firmware"),
            Phase::Final => write!(f, "Final"),
        }
    }
}

/// Generic operations that work across distributions.
///
/// These operations are distro-agnostic. Distro-specific operations
/// (like systemd unit enabling or OpenRC service setup) are defined
/// in the respective distro crates.
#[derive(Debug, Clone)]
pub enum Op {
    // ─────────────────────────────────────────────────────────────────────
    // Directory operations
    // ─────────────────────────────────────────────────────────────────────
    /// Create a directory (uses create_dir_all).
    Dir(String),

    /// Create a directory with specific permissions.
    DirMode(String, u32),

    /// Create multiple directories at once.
    Dirs(Vec<String>),

    // ─────────────────────────────────────────────────────────────────────
    // File operations
    // ─────────────────────────────────────────────────────────────────────
    /// Write a file with given content.
    WriteFile(String, String),

    /// Write a file with specific permissions.
    WriteFileMode(String, String, u32),

    /// Create a symlink (link_path, target).
    Symlink(String, String),

    /// Copy a single file from source to staging.
    CopyFile(String),

    /// Copy a directory tree from source to staging.
    CopyTree(String),

    // ─────────────────────────────────────────────────────────────────────
    // User/group operations
    // ─────────────────────────────────────────────────────────────────────
    /// Ensure a user exists.
    User {
        name: String,
        uid: u32,
        gid: u32,
        home: String,
        shell: String,
    },

    /// Ensure a group exists.
    Group { name: String, gid: u32 },

    // ─────────────────────────────────────────────────────────────────────
    // Binary operations
    // ─────────────────────────────────────────────────────────────────────
    /// Copy a binary with library dependencies to /usr/bin.
    Bin(String),

    /// Copy a binary with library dependencies to /usr/sbin.
    Sbin(String),

    /// Copy multiple binaries to /usr/bin.
    Bins(Vec<String>),

    /// Copy multiple binaries to /usr/sbin.
    Sbins(Vec<String>),

    // ─────────────────────────────────────────────────────────────────────
    // Extension point for distro-specific operations
    // ─────────────────────────────────────────────────────────────────────
    /// Distro-specific custom operation.
    ///
    /// The executor must handle this based on the distro being built.
    /// For LevitateOS: systemd units, udev helpers, etc.
    /// For AcornOS: OpenRC services, busybox applets, etc.
    Custom(String),
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions for readable component definitions
// ─────────────────────────────────────────────────────────────────────────────

/// Create a directory.
pub fn dir(path: impl Into<String>) -> Op {
    Op::Dir(path.into())
}

/// Create a directory with specific mode.
pub fn dir_mode(path: impl Into<String>, mode: u32) -> Op {
    Op::DirMode(path.into(), mode)
}

/// Create multiple directories.
pub fn dirs(paths: impl IntoIterator<Item = impl Into<String>>) -> Op {
    Op::Dirs(paths.into_iter().map(|p| p.into()).collect())
}

/// Write a file.
pub fn write_file(path: impl Into<String>, content: impl Into<String>) -> Op {
    Op::WriteFile(path.into(), content.into())
}

/// Write a file with permissions.
pub fn write_file_mode(path: impl Into<String>, content: impl Into<String>, mode: u32) -> Op {
    Op::WriteFileMode(path.into(), content.into(), mode)
}

/// Create a symlink.
pub fn symlink(link: impl Into<String>, target: impl Into<String>) -> Op {
    Op::Symlink(link.into(), target.into())
}

/// Copy a binary to /usr/bin.
pub fn bin(name: impl Into<String>) -> Op {
    Op::Bin(name.into())
}

/// Copy a binary to /usr/sbin.
pub fn sbin(name: impl Into<String>) -> Op {
    Op::Sbin(name.into())
}

/// Copy multiple binaries to /usr/bin.
pub fn bins(names: impl IntoIterator<Item = impl Into<String>>) -> Op {
    Op::Bins(names.into_iter().map(|n| n.into()).collect())
}

/// Copy multiple binaries to /usr/sbin.
pub fn sbins(names: impl IntoIterator<Item = impl Into<String>>) -> Op {
    Op::Sbins(names.into_iter().map(|n| n.into()).collect())
}

/// Custom distro-specific operation.
pub fn custom(name: impl Into<String>) -> Op {
    Op::Custom(name.into())
}
