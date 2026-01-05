# levitate-pci

PCI subsystem for LevitateOS.

## Overview

This crate provides PCI enumeration and BAR allocation using the `virtio-drivers` PCI module.

## Features

- ECAM (Enhanced Configuration Access Mechanism) support
- PCI bus enumeration
- BAR allocation for VirtIO devices
- PciTransport creation

## Usage

```rust
use levitate_pci::{find_virtio_gpu, PciTransport};

if let Some(transport) = find_virtio_gpu() {
    // Use transport with virtio-drivers GPU
}
```

## TEAM_114

Created as part of VirtIO PCI migration.
