//! VirtIO Block Device Driver
//!
//! TEAM_032: Updated for virtio-drivers v0.12.0
//! - Uses StaticMmioTransport for 'static lifetime compatibility
//!
//! TEAM_150: Converted panics to Result types for proper error handling

use crate::virtio::{StaticMmioTransport, VirtioHal};
use levitate_utils::Spinlock;
use virtio_drivers::device::blk::VirtIOBlk;

/// TEAM_150: Block device error type with error codes.
/// Error codes in range 0x06xx (Block subsystem).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockError {
    /// Device not initialized (0x0601)
    NotInitialized,
    /// Read operation failed (0x0602)
    ReadFailed,
    /// Write operation failed (0x0603)
    WriteFailed,
    /// Invalid buffer size (0x0604)
    InvalidBufferSize,
}

impl BlockError {
    /// TEAM_150: Get numeric error code for debugging
    pub const fn code(&self) -> u16 {
        match self {
            BlockError::NotInitialized => 0x0601,
            BlockError::ReadFailed => 0x0602,
            BlockError::WriteFailed => 0x0603,
            BlockError::InvalidBufferSize => 0x0604,
        }
    }

    /// TEAM_150: Get error name for logging
    pub const fn name(&self) -> &'static str {
        match self {
            BlockError::NotInitialized => "Block device not initialized",
            BlockError::ReadFailed => "Block read failed",
            BlockError::WriteFailed => "Block write failed",
            BlockError::InvalidBufferSize => "Invalid buffer size",
        }
    }
}

impl core::fmt::Display for BlockError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "E{:04X}: {}", self.code(), self.name())
    }
}

// TEAM_150: Implement Error trait for compatibility with Box<dyn Error>
impl core::error::Error for BlockError {}

// TEAM_032: Use StaticMmioTransport (MmioTransport<'static>) for static storage
static BLOCK_DEVICE: Spinlock<Option<VirtIOBlk<VirtioHal, StaticMmioTransport>>> =
    Spinlock::new(None);

pub const BLOCK_SIZE: usize = 512;

pub fn init(transport: StaticMmioTransport) {
    crate::verbose!("Initializing Block device...");
    match VirtIOBlk::<VirtioHal, StaticMmioTransport>::new(transport) {
        Ok(blk) => {
            crate::verbose!("VirtIO Block initialized successfully.");
            *BLOCK_DEVICE.lock() = Some(blk);
        }
        Err(e) => crate::println!("Failed to init VirtIO Block: {:?}", e),
    }
}

/// TEAM_150: Read a single block from the device.
///
/// # Arguments
/// * `block_id` - Block number to read
/// * `buf` - Buffer to read into (must be exactly 512 bytes)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(BlockError)` on failure
pub fn read_block(block_id: usize, buf: &mut [u8]) -> Result<(), BlockError> {
    if buf.len() != BLOCK_SIZE {
        return Err(BlockError::InvalidBufferSize);
    }
    let mut dev = BLOCK_DEVICE.lock();
    if let Some(ref mut blk) = *dev {
        let blk: &mut VirtIOBlk<VirtioHal, StaticMmioTransport> = blk;
        blk.read_blocks(block_id, buf)
            .map_err(|_| BlockError::ReadFailed)
    } else {
        Err(BlockError::NotInitialized)
    }
}

/// TEAM_150: Write a single block to the device.
///
/// # Arguments
/// * `block_id` - Block number to write
/// * `buf` - Buffer to write from (must be exactly 512 bytes)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(BlockError)` on failure
pub fn write_block(block_id: usize, buf: &[u8]) -> Result<(), BlockError> {
    if buf.len() != BLOCK_SIZE {
        return Err(BlockError::InvalidBufferSize);
    }
    let mut dev = BLOCK_DEVICE.lock();
    if let Some(ref mut blk) = *dev {
        let blk: &mut VirtIOBlk<VirtioHal, StaticMmioTransport> = blk;
        blk.write_blocks(block_id, buf)
            .map_err(|_| BlockError::WriteFailed)
    } else {
        Err(BlockError::NotInitialized)
    }
}
