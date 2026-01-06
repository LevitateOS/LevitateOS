# levitate-gpu

GPU driver wrapper for LevitateOS.

## Overview

This crate provides a wrapper around `virtio-drivers::VirtIOGpu` with embedded-graphics support.

## Features

- VirtIO GPU via PCI transport
- Framebuffer management
- embedded-graphics DrawTarget implementation

## Usage

```rust
use levitate_gpu::Gpu;

let gpu = Gpu::new()?;
let (width, height) = gpu.resolution();
gpu.flush()?;
```

## TEAM_114

Created as part of VirtIO PCI migration. Replaces the archived `levitate-drivers-gpu`.
