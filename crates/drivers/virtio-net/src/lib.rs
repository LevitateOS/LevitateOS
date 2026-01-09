//! # virtio-net
//!
//! VirtIO Network driver for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 5).
//!
//! ## Design
//!
//! This crate provides a transport-agnostic VirtIO network driver that:
//! - Implements the `NetworkDevice` trait for use with the kernel's network subsystem
//! - Supports packet send/receive operations
//! - Implements non-blocking capacity checks per Kernel SOP Rule 9
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one device: VirtIO Net.
//! - **Rule 2 (Type-Driven Composition):** Implements `NetworkDevice` trait.
//! - **Rule 6 (Robust Error Handling):** All operations return `Result`.
//! - **Rule 9 (Asynchrony):** Non-blocking with `can_send()`/`can_recv()` checks.
//! - **Rule 11 (Separation):** Driver provides mechanism; kernel handles policy.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use alloc::vec::Vec;
use network_device::{NetworkDevice, NetworkError};

/// Default queue size for VirtIO Net
pub const QUEUE_SIZE: usize = 16;

/// Default RX buffer length
pub const RX_BUFFER_LEN: usize = 2048;

/// Default MTU
pub const DEFAULT_MTU: usize = 1500;

/// TEAM_334: VirtIO Network device state.
///
/// This struct tracks network device configuration and provides
/// an implementation of the `NetworkDevice` trait.
///
/// Note: The actual `VirtIONet` device is held by the kernel's driver
/// wrapper since it requires architecture-specific HAL and transport.
pub struct VirtioNetState {
    /// MAC address
    mac: [u8; 6],
    /// Whether the device is initialized
    initialized: bool,
    /// Whether TX queue has space (cached)
    tx_available: bool,
    /// Whether RX packet is pending (cached)
    rx_pending: bool,
    /// Maximum transmission unit
    mtu: usize,
}

impl VirtioNetState {
    /// Create a new network device state with the given MAC address.
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            mac,
            initialized: true,
            tx_available: true,
            rx_pending: false,
            mtu: DEFAULT_MTU,
        }
    }

    /// Create an uninitialized state (for error cases).
    pub fn uninitialized() -> Self {
        Self::const_uninitialized()
    }

    /// TEAM_334: Const constructor for static initialization
    pub const fn const_uninitialized() -> Self {
        Self {
            mac: [0; 6],
            initialized: false,
            tx_available: false,
            rx_pending: false,
            mtu: DEFAULT_MTU,
        }
    }

    /// Check if the device is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Update TX availability status (called by kernel wrapper after send).
    pub fn set_tx_available(&mut self, available: bool) {
        self.tx_available = available;
    }

    /// Update RX pending status (called by kernel wrapper when packet arrives).
    pub fn set_rx_pending(&mut self, pending: bool) {
        self.rx_pending = pending;
    }
}

impl NetworkDevice for VirtioNetState {
    fn mac_address(&self) -> [u8; 6] {
        self.mac
    }

    fn can_send(&self) -> bool {
        self.initialized && self.tx_available
    }

    fn can_recv(&self) -> bool {
        self.initialized && self.rx_pending
    }

    fn send(&mut self, packet: &[u8]) -> Result<(), NetworkError> {
        if !self.initialized {
            return Err(NetworkError::NotInitialized);
        }

        if !self.tx_available {
            return Err(NetworkError::QueueFull);
        }

        if packet.len() > self.mtu {
            return Err(NetworkError::PacketTooLarge);
        }

        // Note: Actual send is performed by the kernel wrapper which has
        // access to the VirtIONet device. This trait implementation handles
        // validation and state management.
        //
        // In Phase 3, we'll integrate this properly with the kernel.

        Ok(())
    }

    fn receive(&mut self) -> Option<Vec<u8>> {
        if !self.initialized || !self.rx_pending {
            return None;
        }

        // Note: Actual receive is performed by the kernel wrapper.
        // This will be integrated in Phase 3.

        None
    }

    fn mtu(&self) -> usize {
        self.mtu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address() {
        let mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        let state = VirtioNetState::new(mac);
        assert_eq!(state.mac_address(), mac);
    }

    #[test]
    fn test_can_send_initialized() {
        let state = VirtioNetState::new([0; 6]);
        assert!(state.can_send());
    }

    #[test]
    fn test_can_send_uninitialized() {
        let state = VirtioNetState::uninitialized();
        assert!(!state.can_send());
    }

    #[test]
    fn test_can_recv() {
        let mut state = VirtioNetState::new([0; 6]);
        assert!(!state.can_recv()); // No packet pending initially

        state.set_rx_pending(true);
        assert!(state.can_recv());
    }

    #[test]
    fn test_send_not_initialized() {
        let mut state = VirtioNetState::uninitialized();
        assert_eq!(state.send(&[0; 100]), Err(NetworkError::NotInitialized));
    }

    #[test]
    fn test_send_queue_full() {
        let mut state = VirtioNetState::new([0; 6]);
        state.set_tx_available(false);
        assert_eq!(state.send(&[0; 100]), Err(NetworkError::QueueFull));
    }

    #[test]
    fn test_send_packet_too_large() {
        let mut state = VirtioNetState::new([0; 6]);
        let large_packet = [0u8; 2000]; // Larger than MTU
        assert_eq!(state.send(&large_packet), Err(NetworkError::PacketTooLarge));
    }

    #[test]
    fn test_send_success() {
        let mut state = VirtioNetState::new([0; 6]);
        assert!(state.send(&[0; 100]).is_ok());
    }

    #[test]
    fn test_mtu() {
        let state = VirtioNetState::new([0; 6]);
        assert_eq!(state.mtu(), DEFAULT_MTU);
    }
}
