# network-device

Network device trait for LevitateOS.

## Overview

This crate defines the `NetworkDevice` trait, providing a common interface for all network interface cards (NICs) in the kernel:

- VirtIO Net devices (`virtio-net`)
- Intel e1000 (future)
- Intel ixgbe (future)

## Design Pattern

Inspired by [Theseus OS](https://github.com/theseus-os/Theseus) device trait patterns. Separating traits from implementations enables non-VirtIO network drivers to implement the same interfaces.

## Kernel SOP Alignment

- **Rule 1 (Modular Scope):** This crate handles exactly one task: defining network device interface.
- **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
- **Rule 6 (Robust Error Handling):** All operations return `Result<T, NetworkError>`.
- **Rule 9 (Asynchrony):** Non-blocking with `can_send()`/`can_recv()` checks.
- **Rule 11 (Separation):** Trait defines mechanism (raw packets), not policy (protocols).

## Usage

```rust
use network_device::{NetworkDevice, NetworkDeviceRef, NetworkError};

// Check and send
if device.can_send() {
    device.send(&packet)?;
}

// Check and receive
if device.can_recv() {
    if let Some(packet) = device.receive() {
        // Handle packet
    }
}
```

## Created By

TEAM_334 as part of VirtIO driver refactor (Phase 2 Step 1).
