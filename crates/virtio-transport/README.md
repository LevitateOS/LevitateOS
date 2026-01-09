# virtio-transport

Unified VirtIO transport abstraction for LevitateOS.

## Overview

This crate provides a unified `Transport` enum that wraps both MMIO and PCI transports from the `virtio-drivers` crate, allowing driver code to be transport-agnostic.

## Design Decision

**Q1 from plan.md: Should `virtio-transport` wrap or replace `virtio-drivers` transports?**

**Answer: WRAP**

**Rationale (Rule 20 - Simplicity > Perfection):** Wrapping is simpler — avoids rewriting the complex `Transport` trait implementations that `virtio-drivers` already provides.

## Kernel SOP Alignment

- **Rule 1 (Modular Scope):** This crate handles exactly one task: unified transport abstraction.
- **Rule 2 (Type-Driven Composition):** `Transport` enum provides unified interface via delegation.
- **Rule 13 (Representation):** Uses enum to encode transport type; match for dispatch.
- **Rule 20 (Simplicity):** Wraps existing transports rather than reimplementing.

## Usage

```rust
use virtio_transport::{Transport, DriverError, VirtioDriver};

// Create transport from MMIO
let transport = Transport::new_mmio(mmio_transport);

// Get device type
let device_type = transport.device_type();
```

## Features

- `mmio` - Enable MMIO transport support
- `pci` - Enable PCI transport support
- `std` - Enable host-side testing

## Created By

TEAM_334 as part of VirtIO driver refactor (Phase 2 Step 2).
