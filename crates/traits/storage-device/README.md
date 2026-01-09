# storage-device

Storage device trait for LevitateOS.

## Overview

This crate defines the `StorageDevice` trait, providing a common interface for all block storage devices in the kernel:

- VirtIO Block devices (`virtio-blk`)
- AHCI/SATA drives (future)
- NVMe devices (future)
- RAM disks (future)

## Design Pattern

Inspired by [Theseus OS](https://github.com/theseus-os/Theseus) `storage_device` crate. Separating traits from implementations enables non-VirtIO drivers to implement the same interfaces.

Reference: `.external-kernels/theseus/kernel/storage_device/src/lib.rs`

## Kernel SOP Alignment

- **Rule 1 (Modular Scope):** This crate handles exactly one task: defining storage device interface.
- **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
- **Rule 6 (Robust Error Handling):** All operations return `Result<T, StorageError>`.
- **Rule 15 (Verification):** Supports `std` feature for host-side testing.

## Usage

```rust
use storage_device::{StorageDevice, StorageDeviceRef, StorageError};

// Create a storage device reference
let device: StorageDeviceRef = Arc::new(Mutex::new(my_device));

// Read blocks
let mut buf = [0u8; 512];
device.lock().read_blocks(0, &mut buf)?;
```

## Created By

TEAM_334 as part of VirtIO driver refactor (Phase 2 Step 1).
