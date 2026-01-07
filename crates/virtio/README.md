# los_virtio

Core VirtIO transport and abstraction layer for LevitateOS.

## Overview

This crate provides the foundational VirtIO abstractions used by all device drivers in the system. It handles transport-agnostic logic and DMA memory management.

## Features

- **Transport Abstraction**: Unified interface for both Legacy MMIO and modern PCI transports.
- **DMA Management**: Integration with `los_hal` for safe physical memory allocation.
- **VirtQueue Support**: Efficient descriptor ring management.
- **Driver Helpers**: Common initialization patterns for GPU, Block, and Net devices.

## Architecture

```
los_virtio (Transport Layer)
├── los_gpu (PCI/MMIO)
├── los_hal (DMA/Address Translation)
└── los_pci (Discovery)
```

## Usage

```rust
use los_virtio::VirtioHal;

// Drivers use VirtioHal to satisfy virtio-drivers requirements
let gpu = VirtIOGpu::<VirtioHal, _>::new(transport)?;
```

## Traceability

- **TEAM_098**: Initial transport refactor.
- **TEAM_114**: Support for PCI transport.
