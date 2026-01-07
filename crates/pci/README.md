# los_pci

PCI (Peripheral Component Interconnect) subsystem for LevitateOS.

## Overview

This crate provides PCI bus enumeration, capability discovery, and BAR (Base Address Register) allocation. It is primarily used to discover and initialize VirtIO devices on the PCI bus.

## Features

- **ECAM Support**: Enhanced Configuration Access Mechanism for modern AArch64 systems.
- **Bus Enumeration**: Recursive scanning of PCI bridges and devices.
- **Capability Discovery**: Support for VirtIO-specific capabilities (common cfg, notify, device-specific).
- **BAR Allocation**: Automatic mapping of PCI BARs into virtual memory.
- **VirtIO Transport**: Creation of `PciTransport` for use with `virtio-drivers`.

## Usage

```rust
use los_pci::{find_virtio_gpu, PciTransport};

// Find a specific device type
if let Some(transport) = find_virtio_gpu::<VirtioHal>() {
    // Initialize driver with transport
}
```

## Traceability

- **TEAM_114**: Initial implementation for PCI migration.
- **TEAM_116**: Added support for multiple VirtIO capabilities.
