//! FAT32 Filesystem Support
//!
//! TEAM_032: Provides filesystem access over VirtIO block device.
//!
//! Uses the `embedded-sdmmc` crate with our block driver implementing
//! the BlockDevice trait.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use embedded_sdmmc::{
    Block, BlockCount, BlockDevice, BlockIdx, Mode, TimeSource, Timestamp, VolumeIdx, VolumeManager,
};
use levitate_utils::Spinlock;

use crate::block;

/// Error type for our block device
#[derive(Debug, Clone, Copy)]
pub struct BlockError;

/// Adapter that implements embedded-sdmmc BlockDevice over our block driver
pub struct VirtioBlockDevice {
    /// Total number of blocks on the device
    num_blocks: BlockCount,
}

impl VirtioBlockDevice {
    /// Create a new VirtioBlockDevice with the given size in bytes
    pub fn new(size_bytes: u64) -> Self {
        let num_blocks = (size_bytes / 512) as u32;
        Self {
            num_blocks: BlockCount(num_blocks),
        }
    }
}

impl BlockDevice for VirtioBlockDevice {
    type Error = BlockError;

    fn read(&self, blocks: &mut [Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter_mut().enumerate() {
            let block_id = start_block_idx.0 as usize + i;
            block::read_block(block_id, &mut block.contents);
        }
        Ok(())
    }

    fn write(&self, blocks: &[Block], start_block_idx: BlockIdx) -> Result<(), Self::Error> {
        for (i, block) in blocks.iter().enumerate() {
            let block_id = start_block_idx.0 as usize + i;
            block::write_block(block_id, &block.contents);
        }
        Ok(())
    }

    fn num_blocks(&self) -> Result<BlockCount, Self::Error> {
        Ok(self.num_blocks)
    }
}

/// Dummy time source (we don't have RTC yet)
pub struct DummyTimeSource;

impl TimeSource for DummyTimeSource {
    fn get_timestamp(&self) -> Timestamp {
        // Return a fixed timestamp (2026-01-04 00:00:00)
        Timestamp {
            year_since_1970: 56, // 2026 - 1970
            zero_indexed_month: 0,
            zero_indexed_day: 3,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

// ============================================================================
// Public API
// ============================================================================

/// Global filesystem ready flag
static FS_READY: Spinlock<bool> = Spinlock::new(false);

/// Disk size in bytes (16MB = 16 * 1024 * 1024 = 16777216)
const DISK_SIZE_BYTES: u64 = 16 * 1024 * 1024;

/// Initialize the filesystem from the block device
///
/// Returns Ok(()) if filesystem was successfully mounted, Err otherwise.
pub fn init() -> Result<(), &'static str> {
    let block_device = VirtioBlockDevice::new(DISK_SIZE_BYTES);
    let time_source = DummyTimeSource;

    // Create volume manager with small limits for kernel use
    // DIRS=4, FILES=4, VOLUMES=1
    let volume_mgr = VolumeManager::<_, _, 4, 4, 1>::new(block_device, time_source);

    // Try to open volume 0
    match volume_mgr.open_volume(VolumeIdx(0)) {
        Ok(volume) => {
            crate::verbose!("Filesystem mounted (FAT).");

            // Demo: List root directory
            match volume.open_root_dir() {
                Ok(root_dir) => {
                    crate::verbose!("Root directory contents:");
                    root_dir
                        .iterate_dir(|entry| {
                            crate::verbose!("  - {}", entry.name);
                        })
                        .ok();
                }
                Err(_) => {
                    crate::println!("ERROR: Failed to open root directory");
                }
            }

            *FS_READY.lock() = true;
            Ok(())
        }
        Err(_e) => {
            crate::println!("ERROR: Failed to mount filesystem (not FAT?)");
            Err("Failed to mount filesystem")
        }
    }
}

/// Read entire file contents by path
///
/// Returns None if file not found or filesystem not ready.
pub fn read_file(path: &str) -> Option<Vec<u8>> {
    if !*FS_READY.lock() {
        return None;
    }

    let block_device = VirtioBlockDevice::new(DISK_SIZE_BYTES);
    let time_source = DummyTimeSource;
    let volume_mgr = VolumeManager::<_, _, 4, 4, 1>::new(block_device, time_source);

    let volume = volume_mgr.open_volume(VolumeIdx(0)).ok()?;
    let root_dir = volume.open_root_dir().ok()?;

    // Open file (path must be 8.3 format for now)
    let mut file = root_dir.open_file_in_dir(path, Mode::ReadOnly).ok()?;

    let mut contents = Vec::new();
    let mut buf = [0u8; 512];
    loop {
        match file.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => contents.extend_from_slice(&buf[..n]),
            Err(_) => return None,
        }
    }

    Some(contents)
}

/// List directory entries at path
///
/// Returns empty Vec if directory not found or filesystem not ready.
pub fn list_dir(path: &str) -> Vec<String> {
    if !*FS_READY.lock() {
        return Vec::new();
    }

    let block_device = VirtioBlockDevice::new(DISK_SIZE_BYTES);
    let time_source = DummyTimeSource;
    let volume_mgr = VolumeManager::<_, _, 4, 4, 1>::new(block_device, time_source);

    let volume = match volume_mgr.open_volume(VolumeIdx(0)) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let root_dir = match volume.open_root_dir() {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };

    let mut entries = Vec::new();

    // If path is root, list root_dir directly
    if path == "/" || path.is_empty() {
        let _ = root_dir.iterate_dir(|entry| {
            entries.push(entry.name.to_string());
        });
    } else {
        // Open subdirectory
        if let Ok(sub_dir) = root_dir.open_dir(path) {
            let _ = sub_dir.iterate_dir(|entry| {
                entries.push(entry.name.to_string());
            });
        }
    }

    entries
}
