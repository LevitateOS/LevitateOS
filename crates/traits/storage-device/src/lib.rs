//! # storage-device
//!
//! Storage device trait for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 1).
//!
//! ## Design Pattern
//!
//! Inspired by Theseus OS `storage_device` crate. Separating traits from implementations
//! enables non-VirtIO drivers (AHCI, NVMe, etc.) to implement the same interfaces.
//!
//! Reference: `.external-kernels/theseus/kernel/storage_device/src/lib.rs`
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one task: defining storage device interface.
//! - **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
//! - **Rule 6 (Robust Error Handling):** All operations return `Result<T, StorageError>`.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use alloc::sync::Arc;
use spin::Mutex;

/// TEAM_334: Storage device error types.
///
/// Following kernel SOP Rule 6: custom error enums for explicit error handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Device is not ready or not initialized
    NotReady,
    /// Invalid block number (out of range)
    InvalidBlock,
    /// Generic I/O error during read/write
    IoError,
    /// Buffer size does not match block size
    InvalidBufferSize,
}

/// Trait for block storage devices (HDDs, SSDs, VirtIO Block, AHCI, NVMe, etc.)
///
/// TEAM_334: Inspired by Theseus `StorageDevice` trait.
///
/// ## Kernel SOP Alignment
///
/// - **Rule 2 (Composition):** Trait-based interface for orthogonal subsystems
/// - **Rule 6 (Robustness):** All operations return `Result`
/// - **Rule 11 (Separation):** Trait defines mechanism, not policy
pub trait StorageDevice: Send {
    /// Block size in bytes (typically 512 for traditional drives, 4096 for some SSDs)
    fn block_size(&self) -> usize;

    /// Total size of device in blocks (sectors)
    fn size_in_blocks(&self) -> usize;

    /// Read blocks starting at `block_id` into `buf`.
    ///
    /// The buffer must be exactly `block_size() * num_blocks` bytes where
    /// `num_blocks = buf.len() / block_size()`.
    ///
    /// # Errors
    ///
    /// Returns `StorageError` if the operation fails.
    fn read_blocks(&mut self, block_id: usize, buf: &mut [u8]) -> Result<(), StorageError>;

    /// Write blocks starting at `block_id` from `buf`.
    ///
    /// The buffer must be exactly `block_size() * num_blocks` bytes where
    /// `num_blocks = buf.len() / block_size()`.
    ///
    /// # Errors
    ///
    /// Returns `StorageError` if the operation fails.
    fn write_blocks(&mut self, block_id: usize, buf: &[u8]) -> Result<(), StorageError>;

    /// Total size in bytes (convenience method)
    fn size_in_bytes(&self) -> usize {
        self.size_in_blocks() * self.block_size()
    }
}

/// Thread-safe reference to a storage device.
///
/// TEAM_334: Following Theseus pattern for device references.
/// Enables runtime polymorphism and shared ownership across kernel subsystems.
pub type StorageDeviceRef = Arc<Mutex<dyn StorageDevice + Send>>;

#[cfg(test)]
mod tests {
    use super::*;

    // TEAM_334: Mock device for testing
    struct MockStorageDevice {
        block_size: usize,
        block_count: usize,
    }

    impl StorageDevice for MockStorageDevice {
        fn block_size(&self) -> usize {
            self.block_size
        }

        fn size_in_blocks(&self) -> usize {
            self.block_count
        }

        fn read_blocks(&mut self, _block_id: usize, _buf: &mut [u8]) -> Result<(), StorageError> {
            Ok(())
        }

        fn write_blocks(&mut self, _block_id: usize, _buf: &[u8]) -> Result<(), StorageError> {
            Ok(())
        }
    }

    #[test]
    fn test_size_in_bytes() {
        let device = MockStorageDevice {
            block_size: 512,
            block_count: 1000,
        };
        assert_eq!(device.size_in_bytes(), 512 * 1000);
    }
}
