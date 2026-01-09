//! # virtio-transport
//!
//! Unified VirtIO transport abstraction for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 2).
//!
//! ## Design Decision (Q1 from plan.md)
//!
//! **Answer: WRAP** `virtio-drivers` transports, not replace.
//!
//! **Rationale (Rule 20 - Simplicity > Perfection):** Wrapping is simpler — avoids rewriting
//! the complex `Transport` trait implementations that `virtio-drivers` already provides.
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one task: unified transport abstraction.
//! - **Rule 2 (Type-Driven Composition):** `Transport` enum provides unified interface via delegation.
//! - **Rule 13 (Representation):** Uses enum to encode transport type; match for dispatch.
//! - **Rule 20 (Simplicity):** Wraps existing transports rather than reimplementing.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

// TEAM_334: Import necessary types for Trait implementation
use virtio_drivers::transport::DeviceType;
// TEAM_334: Import Transport trait to access device_type() method
use virtio_drivers::transport::Transport as VirtioTransportTrait;
use virtio_drivers::{PhysAddr, Result};

// TEAM_334: Re-export useful types from virtio-drivers
pub use virtio_drivers::transport::DeviceType as VirtioDeviceType;
pub use virtio_drivers::Hal;

/// MMIO transport type alias for static lifetime.
pub type MmioTransport = virtio_drivers::transport::mmio::MmioTransport<'static>;

/// PCI transport type (when PCI feature enabled).
#[cfg(feature = "pci")]
pub type PciTransport = virtio_drivers::transport::pci::PciTransport;

/// TEAM_334: Unified transport for VirtIO devices.
///
/// Wraps either MMIO or PCI transport from `virtio-drivers` crate.
///
/// ## Kernel SOP Alignment
///
/// - **Rule 2 (Composition):** Single enum interface for multiple transport types
/// - **Rule 13 (Representation):** Match over enum variants for dispatch
/// - **Rule 19 (Diversity):** Supports both MMIO (aarch64) and PCI (x86_64) transports
#[derive(Debug)]
pub enum Transport {
    /// MMIO transport (typical for aarch64/QEMU virt machine)
    Mmio(MmioTransport),
    /// PCI transport (typical for x86_64)
    #[cfg(feature = "pci")]
    Pci(PciTransport),
}

impl Transport {
    /// Create a new MMIO transport.
    ///
    /// # Safety
    ///
    /// The caller must ensure the MMIO region is valid and properly mapped.
    #[cfg(feature = "mmio")]
    pub fn new_mmio(transport: MmioTransport) -> Self {
        Transport::Mmio(transport)
    }

    /// Create a new PCI transport.
    #[cfg(feature = "pci")]
    pub fn new_pci(transport: PciTransport) -> Self {
        Transport::Pci(transport)
    }
}

// TEAM_334: Implement the unified Transport trait by delegating to inner transports
impl VirtioTransportTrait for Transport {
    fn device_type(&self) -> DeviceType {
        match self {
            Transport::Mmio(t) => t.device_type(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.device_type(),
        }
    }

    fn read_device_features(&mut self) -> u64 {
        match self {
            Transport::Mmio(t) => t.read_device_features(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.read_device_features(),
        }
    }

    fn write_driver_features(&mut self, driver_features: u64) {
        match self {
            Transport::Mmio(t) => t.write_driver_features(driver_features),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.write_driver_features(driver_features),
        }
    }

    fn max_queue_size(&mut self, queue: u16) -> u32 {
        match self {
            Transport::Mmio(t) => t.max_queue_size(queue),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.max_queue_size(queue),
        }
    }

    fn notify(&mut self, queue: u16) {
        match self {
            Transport::Mmio(t) => t.notify(queue),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.notify(queue),
        }
    }

    fn set_status(&mut self, status: virtio_drivers::transport::DeviceStatus) {
        match self {
            Transport::Mmio(t) => t.set_status(status),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.set_status(status),
        }
    }

    fn set_guest_page_size(&mut self, guest_page_size: u32) {
        match self {
            Transport::Mmio(t) => t.set_guest_page_size(guest_page_size),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.set_guest_page_size(guest_page_size),
        }
    }

    fn requires_legacy_layout(&self) -> bool {
        match self {
            Transport::Mmio(t) => t.requires_legacy_layout(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.requires_legacy_layout(),
        }
    }

    fn queue_set(
        &mut self,
        queue: u16,
        size: u32,
        descriptors: PhysAddr,
        driver_area: PhysAddr,
        device_area: PhysAddr,
    ) {
        match self {
            Transport::Mmio(t) => t.queue_set(queue, size, descriptors, driver_area, device_area),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.queue_set(queue, size, descriptors, driver_area, device_area),
        }
    }

    fn queue_unset(&mut self, queue: u16) {
        match self {
            Transport::Mmio(t) => t.queue_unset(queue),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.queue_unset(queue),
        }
    }

    fn queue_used(&mut self, queue: u16) -> bool {
        match self {
            Transport::Mmio(t) => t.queue_used(queue),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.queue_used(queue),
        }
    }

    fn ack_interrupt(&mut self) -> virtio_drivers::transport::InterruptStatus {
        match self {
            Transport::Mmio(t) => t.ack_interrupt(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.ack_interrupt(),
        }
    }

    fn get_status(&self) -> virtio_drivers::transport::DeviceStatus {
        match self {
            Transport::Mmio(t) => t.get_status(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.get_status(),
        }
    }

    fn read_config_generation(&self) -> u32 {
        match self {
            Transport::Mmio(t) => t.read_config_generation(),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.read_config_generation(),
        }
    }

    fn read_config_space<T: zerocopy::FromBytes + zerocopy::IntoBytes>(
        &self,
        offset: usize,
    ) -> Result<T> {
        match self {
            Transport::Mmio(t) => t.read_config_space(offset),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.read_config_space(offset),
        }
    }

    fn write_config_space<T: zerocopy::IntoBytes + zerocopy::Immutable>(
        &mut self,
        offset: usize,
        value: T,
    ) -> Result<()> {
        match self {
            Transport::Mmio(t) => t.write_config_space(offset, value),
            #[cfg(feature = "pci")]
            Transport::Pci(t) => t.write_config_space(offset, value),
        }
    }
}

/// TEAM_334: Driver initialization error types.
///
/// Following kernel SOP Rule 6: custom error enums for explicit error handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    /// Transport type doesn't match expected device type
    TransportMismatch,
    /// Device initialization failed
    InitFailed,
    /// No device found at expected location
    NoDevice,
    /// Device configuration error
    ConfigError,
    /// Unsupported device version or feature
    Unsupported,
}

/// TEAM_334: Common interface for VirtIO device drivers.
///
/// ## Kernel SOP Alignment
///
/// - **Rule 2 (Composition):** Trait-based interface for orthogonal subsystems
/// - **Rule 6 (Robustness):** Initialization returns `Result`
/// - **Rule 11 (Separation):** Trait defines mechanism, not policy
pub trait VirtioDriver: Send + Sync {
    /// Device type this driver handles.
    const DEVICE_TYPE: DeviceType;

    /// Handle device interrupt.
    ///
    /// Called from interrupt handler when this device's interrupt fires.
    fn handle_interrupt(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_error_variants() {
        // TEAM_334: Verify error enum is exhaustive
        let errors = [
            DriverError::TransportMismatch,
            DriverError::InitFailed,
            DriverError::NoDevice,
            DriverError::ConfigError,
            DriverError::Unsupported,
        ];
        assert_eq!(errors.len(), 5);
    }
}
