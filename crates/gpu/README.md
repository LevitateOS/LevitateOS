# los_gpu

VirtIO GPU driver for LevitateOS.

## Overview

This crate provides a robust driver for VirtIO GPU devices, supporting framebuffer management and 2D acceleration primitives. It implements the `embedded-graphics` `DrawTarget` trait for seamless integration with the Rust graphics ecosystem.

## Features

- **PCI Transport**: Fully migrated to VirtIO over PCI (TEAM_114).
- **Framebuffer Management**: Support for multiple scanouts and resource management.
- **embedded-graphics**: Native support for drawing text, shapes, and images.
- **Double Buffering**: Support for flushing changes to the host display.

## Architecture

- `Gpu`: Core device driver handling VirtIO protocol.
- `Display`: A `DrawTarget` implementation for `embedded-graphics`.
- `VirtioHal`: DMA and memory mapping integration.

## Usage

```rust
use los_gpu::{Gpu, Display};

// Initialize with a PCI transport
let mut gpu = Gpu::new(transport)?;
let (width, height) = gpu.resolution();

// Use as a DrawTarget
let mut display = Display::new(&mut gpu);
// ... draw things ...
gpu.flush()?;
```

## Traceability

- **TEAM_114**: Migrated to PCI transport.
- **TEAM_098**: Refactored for better resource management.
