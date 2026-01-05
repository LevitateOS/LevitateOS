//! PCI Subsystem for VirtIO PCI Transport
//!
//! TEAM_114: Implements PCI enumeration and BAR allocation using virtio-drivers.
//!
//! This module provides:
//! - ECAM (Enhanced Configuration Access Mechanism) access
//! - PCI bus enumeration
//! - BAR allocation for VirtIO devices
//! - PciTransport creation for VirtIO GPU

#![allow(clippy::unwrap_used)]

use levitate_hal::mmu::{ECAM_VA, PCI_MEM32_PA, PCI_MEM32_SIZE};
use levitate_hal::serial_println;
use virtio_drivers::transport::pci::bus::{
    BarInfo, Cam, Command, DeviceFunction, MemoryBarType, MmioCam, PciRoot,
};
use virtio_drivers::transport::pci::{virtio_device_type, PciTransport};
use virtio_drivers::transport::DeviceType;

use crate::virtio::VirtioHal;

/// Simple bump allocator for PCI 32-bit memory region
struct PciMemoryAllocator {
    next: u32,
    end: u32,
}

impl PciMemoryAllocator {
    /// Create a new allocator for the PCI 32-bit memory region
    fn new() -> Self {
        Self {
            next: PCI_MEM32_PA as u32,
            end: (PCI_MEM32_PA + PCI_MEM32_SIZE) as u32,
        }
    }

    /// Allocate a memory region with the given size (must be power of 2)
    /// Returns the allocated address, or None if out of space
    fn allocate(&mut self, size: u32) -> Option<u32> {
        if size == 0 || !size.is_power_of_two() {
            return None;
        }

        // Align to size (PCI BARs require alignment = size)
        let aligned = (self.next + size - 1) & !(size - 1);

        if aligned.checked_add(size)? > self.end {
            return None;
        }

        self.next = aligned + size;
        Some(aligned)
    }
}

/// Allocate BARs for a PCI device
fn allocate_bars<C: virtio_drivers::transport::pci::bus::ConfigurationAccess>(
    root: &mut PciRoot<C>,
    device_function: DeviceFunction,
    allocator: &mut PciMemoryAllocator,
) {
    // TEAM_114: Iterate through all BARs and allocate memory for them
    if let Ok(bars) = root.bars(device_function) {
        for (bar_index, bar_info) in bars.into_iter().enumerate() {
            let Some(info) = bar_info else { continue };

            if let BarInfo::Memory {
                address_type, size, ..
            } = info
            {
                if size == 0 || size > u32::MAX as u64 {
                    continue;
                }

                let size = size as u32;

                match address_type {
                    MemoryBarType::Width32 => {
                        if let Some(addr) = allocator.allocate(size) {
                            root.set_bar_32(device_function, bar_index as u8, addr);
                        }
                    }
                    MemoryBarType::Width64 => {
                        if let Some(addr) = allocator.allocate(size) {
                            root.set_bar_64(device_function, bar_index as u8, addr as u64);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Enable memory space and bus mastering
    root.set_command(device_function, Command::MEMORY_SPACE | Command::BUS_MASTER);
}

/// Initialize PCI subsystem and find VirtIO GPU
///
/// Returns a PciTransport for the GPU if found, None otherwise.
pub fn find_virtio_gpu() -> Option<PciTransport> {
    serial_println!("[PCI] Scanning Bus 0...");

    // TEAM_114: Create MmioCam for ECAM access
    // SAFETY: ECAM_VA must be mapped as Device memory before calling this
    let cam = unsafe { MmioCam::new(ECAM_VA as *mut u8, Cam::Ecam) };

    let mut pci_root = PciRoot::new(cam);
    let mut allocator = PciMemoryAllocator::new();

    // Enumerate bus 0 (QEMU virt puts devices on bus 0)
    for (device_function, info) in pci_root.enumerate_bus(0) {
        // Check if this is a VirtIO device
        if let Some(virtio_type) = virtio_device_type(&info) {
            if virtio_type == DeviceType::GPU {
                serial_println!("[PCI] Found VirtIO GPU at {}", device_function);

                // Allocate BARs
                allocate_bars(&mut pci_root, device_function, &mut allocator);

                // Create PciTransport
                match PciTransport::new::<VirtioHal, _>(&mut pci_root, device_function) {
                    Ok(transport) => {
                        serial_println!("[PCI] PciTransport created successfully");
                        return Some(transport);
                    }
                    Err(e) => {
                        serial_println!("[PCI] Failed to create PciTransport: {:?}", e);
                    }
                }
            }
        }
    }

    serial_println!("[PCI] No VirtIO GPU found");
    None
}
