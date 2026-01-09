//! # virtio-blk
//!
//! VirtIO Block driver for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 4).
//!
//! ## Design
//!
//! This crate provides a transport-agnostic VirtIO block driver that:
//! - Implements the `StorageDevice` trait for use with the kernel's storage subsystem
//! - Supports block read/write operations
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one device: VirtIO Block.
//! - **Rule 2 (Type-Driven Composition):** Implements `StorageDevice` trait.
//! - **Rule 6 (Robust Error Handling):** All operations return `Result`.
//! - **Rule 11 (Separation):** Driver provides mechanism; kernel handles policy.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use storage_device::{StorageDevice, StorageError};

/// Standard block size for VirtIO Block devices
pub const BLOCK_SIZE: usize = 512;

/// TEAM_334: VirtIO Block device state.
///
/// This struct wraps the block device configuration and provides
/// an implementation of the `StorageDevice` trait.
///
/// Note: The actual `VirtIOBlk` device is held by the kernel's driver
/// wrapper since it requires architecture-specific HAL and transport.
pub struct VirtioBlkState {
    /// Block size in bytes
    block_size: usize,
    /// Total number of blocks
    block_count: usize,
    /// Whether the device is initialized
    initialized: bool,
}

impl VirtioBlkState {
    /// Create a new block device state.
    ///
    /// # Arguments
    ///
    /// * `block_count` - Total number of blocks on the device
    pub fn new(block_count: usize) -> Self {
        Self {
            block_size: BLOCK_SIZE,
            block_count,
            initialized: true,
        }
    }

    /// Create an uninitialized state (for error cases).
    pub fn uninitialized() -> Self {
        Self::const_uninitialized()
    }

    /// TEAM_334: Const constructor for static initialization
    pub const fn const_uninitialized() -> Self {
        Self {
            block_size: BLOCK_SIZE,
            block_count: 0,
            initialized: false,
        }
    }

    /// Check if the device is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Validate block range for read/write operations.
    pub fn validate_block_range(
        &self,
        block_id: usize,
        buf_len: usize,
    ) -> Result<usize, StorageError> {
        if !self.initialized {
            return Err(StorageError::NotReady);
        }

        if buf_len % self.block_size != 0 {
            return Err(StorageError::InvalidBufferSize);
        }

        let num_blocks = buf_len / self.block_size;

        if block_id >= self.block_count {
            return Err(StorageError::InvalidBlock);
        }

        if block_id + num_blocks > self.block_count {
            return Err(StorageError::InvalidBlock);
        }

        Ok(num_blocks)
    }
}

impl StorageDevice for VirtioBlkState {
    fn block_size(&self) -> usize {
        self.block_size
    }

    fn size_in_blocks(&self) -> usize {
        self.block_count
    }

    fn read_blocks(&mut self, block_id: usize, buf: &mut [u8]) -> Result<(), StorageError> {
        // Validate the operation
        let _num_blocks = self.validate_block_range(block_id, buf.len())?;

        // Note: Actual read is performed by the kernel wrapper which has
        // access to the VirtIOBlk device. This trait implementation is
        // used for metadata and validation.
        //
        // In Phase 3, we'll update this to actually perform I/O when
        // we integrate with the kernel.

        Ok(())
    }

    fn write_blocks(&mut self, block_id: usize, buf: &[u8]) -> Result<(), StorageError> {
        // Validate the operation
        let _num_blocks = self.validate_block_range(block_id, buf.len())?;

        // Note: Actual write is performed by the kernel wrapper.
        // See read_blocks comment.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_size() {
        let state = VirtioBlkState::new(1000);
        assert_eq!(state.block_size(), BLOCK_SIZE);
    }

    #[test]
    fn test_size_in_blocks() {
        let state = VirtioBlkState::new(1000);
        assert_eq!(state.size_in_blocks(), 1000);
    }

    #[test]
    fn test_size_in_bytes() {
        let state = VirtioBlkState::new(1000);
        assert_eq!(state.size_in_bytes(), 1000 * BLOCK_SIZE);
    }

    #[test]
    fn test_validate_block_range_valid() {
        let state = VirtioBlkState::new(100);
        // Read 1 block at position 0
        assert!(state.validate_block_range(0, 512).is_ok());
        // Read 10 blocks at position 50
        assert!(state.validate_block_range(50, 5120).is_ok());
    }

    #[test]
    fn test_validate_block_range_invalid_block() {
        let state = VirtioBlkState::new(100);
        // Block 100 is out of range (0-99 valid)
        assert_eq!(
            state.validate_block_range(100, 512),
            Err(StorageError::InvalidBlock)
        );
    }

    #[test]
    fn test_validate_block_range_overflow() {
        let state = VirtioBlkState::new(100);
        // Read 10 blocks starting at 95 would overflow
        assert_eq!(
            state.validate_block_range(95, 5120),
            Err(StorageError::InvalidBlock)
        );
    }

    #[test]
    fn test_validate_block_range_invalid_size() {
        let state = VirtioBlkState::new(100);
        // Buffer size not multiple of block size
        assert_eq!(
            state.validate_block_range(0, 100),
            Err(StorageError::InvalidBufferSize)
        );
    }

    #[test]
    fn test_uninitialized_state() {
        let state = VirtioBlkState::uninitialized();
        assert!(!state.is_initialized());
        assert_eq!(
            state.validate_block_range(0, 512),
            Err(StorageError::NotReady)
        );
    }
}
