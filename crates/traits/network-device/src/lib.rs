//! # network-device
//!
//! Network device trait for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 1).
//!
//! ## Design Pattern
//!
//! Inspired by Theseus OS device trait patterns. Separating traits from implementations
//! enables non-VirtIO network drivers (e1000, ixgbe, etc.) to implement the same interfaces.
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one task: defining network device interface.
//! - **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
//! - **Rule 6 (Robust Error Handling):** All fallible operations return `Result<T, NetworkError>`.
//! - **Rule 9 (Asynchrony):** Non-blocking send/receive with `can_send()`/`can_recv()` checks.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::Mutex;

/// TEAM_334: Network device error types.
///
/// Following kernel SOP Rule 6: custom error enums for explicit error handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    /// Device is not ready or not initialized
    NotInitialized,
    /// TX queue is full, cannot send
    QueueFull,
    /// Transmission failed
    TransmitFailed,
    /// Receive failed
    ReceiveFailed,
    /// Packet too large for MTU
    PacketTooLarge,
}

/// Trait for network devices (NICs - VirtIO Net, e1000, ixgbe, etc.)
///
/// TEAM_334: Following Theseus device trait patterns.
///
/// ## Kernel SOP Alignment
///
/// - **Rule 2 (Composition):** Trait-based interface for orthogonal subsystems
/// - **Rule 6 (Robustness):** All operations return `Result`
/// - **Rule 9 (Asynchrony):** Non-blocking with capacity checks
/// - **Rule 11 (Separation):** Trait defines mechanism (raw packets), not policy (protocols)
pub trait NetworkDevice: Send {
    /// Get the device's MAC address.
    fn mac_address(&self) -> [u8; 6];

    /// Check if the TX queue has space to send a packet.
    ///
    /// Per kernel SOP Rule 9: Avoid blocking; check capacity first.
    fn can_send(&self) -> bool;

    /// Check if there are packets available to receive.
    ///
    /// Per kernel SOP Rule 9: Avoid blocking; check availability first.
    fn can_recv(&self) -> bool;

    /// Send a raw Ethernet packet.
    ///
    /// The packet should include the full Ethernet frame (header + payload).
    ///
    /// # Errors
    ///
    /// Returns `NetworkError` if the operation fails.
    fn send(&mut self, packet: &[u8]) -> Result<(), NetworkError>;

    /// Receive a raw Ethernet packet.
    ///
    /// Returns `None` if no packet is available.
    /// Use `can_recv()` to check before calling to avoid unnecessary allocations.
    fn receive(&mut self) -> Option<Vec<u8>>;

    /// Get the Maximum Transmission Unit (MTU) for this device.
    ///
    /// Default is 1500 bytes (standard Ethernet MTU).
    fn mtu(&self) -> usize {
        1500
    }
}

/// Thread-safe reference to a network device.
///
/// TEAM_334: Following Theseus pattern for device references.
pub type NetworkDeviceRef = Arc<Mutex<dyn NetworkDevice + Send>>;

#[cfg(test)]
mod tests {
    use super::*;

    // TEAM_334: Mock device for testing
    struct MockNetworkDevice {
        mac: [u8; 6],
        tx_space: bool,
        rx_packet: Option<Vec<u8>>,
    }

    impl MockNetworkDevice {
        fn new(mac: [u8; 6]) -> Self {
            Self {
                mac,
                tx_space: true,
                rx_packet: None,
            }
        }
    }

    impl NetworkDevice for MockNetworkDevice {
        fn mac_address(&self) -> [u8; 6] {
            self.mac
        }

        fn can_send(&self) -> bool {
            self.tx_space
        }

        fn can_recv(&self) -> bool {
            self.rx_packet.is_some()
        }

        fn send(&mut self, _packet: &[u8]) -> Result<(), NetworkError> {
            if self.tx_space {
                Ok(())
            } else {
                Err(NetworkError::QueueFull)
            }
        }

        fn receive(&mut self) -> Option<Vec<u8>> {
            self.rx_packet.take()
        }
    }

    #[test]
    fn test_mock_network_device() {
        let device = MockNetworkDevice::new([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(device.mac_address(), [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert!(device.can_send());
        assert!(!device.can_recv());
    }

    #[test]
    fn test_default_mtu() {
        let device = MockNetworkDevice::new([0; 6]);
        assert_eq!(device.mtu(), 1500);
    }
}
