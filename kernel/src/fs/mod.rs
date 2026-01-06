//! Virtual Filesystem Layer
//!
//! TEAM_032: Provides unified filesystem access with multiple backend support.
//! - FAT32 via embedded-sdmmc (boot partition)
//! - ext4 via ext4-view (root partition, read-only)
//!
//! Note: Some functions are kept for future VFS integration.
#![allow(dead_code)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use levitate_utils::Spinlock;

pub mod ext4;
pub mod fat;

/// TEAM_152: Filesystem error type with error codes (0x05xx) per unified error system plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    /// Failed to open volume (0x0501)
    VolumeOpen,
    /// Failed to open directory (0x0502)
    DirOpen,
    /// Failed to open file (0x0503)
    FileOpen,
    /// Read error (0x0504)
    ReadError,
    /// Write error (0x0505)
    WriteError,
    /// Filesystem not mounted (0x0506)
    NotMounted,
    /// Block device error (0x0507)
    BlockError(crate::block::BlockError),
}

impl FsError {
    /// TEAM_152: Get numeric error code for debugging
    pub const fn code(&self) -> u16 {
        match self {
            Self::VolumeOpen => 0x0501,
            Self::DirOpen => 0x0502,
            Self::FileOpen => 0x0503,
            Self::ReadError => 0x0504,
            Self::WriteError => 0x0505,
            Self::NotMounted => 0x0506,
            Self::BlockError(_) => 0x0507,
        }
    }

    /// TEAM_152: Get error name for logging
    pub const fn name(&self) -> &'static str {
        match self {
            Self::VolumeOpen => "Failed to open volume",
            Self::DirOpen => "Failed to open directory",
            Self::FileOpen => "Failed to open file",
            Self::ReadError => "Read error",
            Self::WriteError => "Write error",
            Self::NotMounted => "Filesystem not mounted",
            Self::BlockError(_) => "Block device error",
        }
    }
}

impl core::fmt::Display for FsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BlockError(inner) => write!(f, "E{:04X}: {} ({})", self.code(), self.name(), inner),
            _ => write!(f, "E{:04X}: {}", self.code(), self.name()),
        }
    }
}

impl core::error::Error for FsError {}

impl From<crate::block::BlockError> for FsError {
    fn from(e: crate::block::BlockError) -> Self {
        FsError::BlockError(e)
    }
}

/// Filesystem type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsType {
    Fat32,
    Ext4,
}

/// Mount status
static FAT32_MOUNTED: Spinlock<bool> = Spinlock::new(false);
static EXT4_MOUNTED: Spinlock<bool> = Spinlock::new(false);

/// Initialize filesystems
///
/// Attempts to mount FAT32 boot partition.
/// ext4 root partition is optional and can be mounted later.
pub mod initramfs;

pub static INITRAMFS: Spinlock<Option<initramfs::CpioArchive<'static>>> = Spinlock::new(None);

pub fn init() -> Result<(), FsError> {
    // TEAM_152: Updated to use FsError
    // Mount FAT32 boot partition
    match fat::mount_and_list() {
        Ok(entries) => {
            *FAT32_MOUNTED.lock() = true;
            crate::verbose!("FAT32 mounted. Root contains {} entries.", entries.len());
            for _entry in entries.iter().take(5) {
                // Entries logged via verbose macro if enabled
            }
            Ok(())
        }
        Err(e) => {
            crate::println!("ERROR: Failed to mount FAT32: {}", e);
            Err(e)
        }
    }
}

/// Initialize ext4 filesystem (optional second disk)
pub fn init_ext4() -> Result<(), FsError> {
    // TEAM_152: Updated to use FsError
    match ext4::mount_and_list() {
        Ok(entries) => {
            *EXT4_MOUNTED.lock() = true;
            crate::verbose!("ext4 mounted. Root contains {} entries.", entries.len());
            for _entry in entries.iter().take(5) {
                // Entries logged via verbose macro if enabled
            }
            Ok(())
        }
        Err(e) => {
            crate::verbose!("ext4 not available: {}", e);
            Err(e)
        }
    }
}

/// Read file from mounted filesystem
///
/// Tries FAT32 first, then ext4 if available.
pub fn read_file(path: &str) -> Option<Vec<u8>> {
    // Try FAT32 first
    if *FAT32_MOUNTED.lock() {
        if let Some(data) = fat::read_file(path) {
            return Some(data);
        }
    }

    // Try ext4
    if *EXT4_MOUNTED.lock() {
        if let Some(data) = ext4::read_file(path) {
            return Some(data);
        }
    }

    None
}

/// TEAM_065: List directory contents with proper error propagation (Rule 6)
/// TEAM_152: Updated to use FsError
pub fn list_dir(_path: &str, fs_type: FsType) -> Result<Vec<String>, FsError> {
    match fs_type {
        FsType::Fat32 => fat::mount_and_list(),
        FsType::Ext4 => ext4::mount_and_list(),
    }
}
