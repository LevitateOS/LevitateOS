# virtio-gpu

VirtIO GPU device driver for LevitateOS.

## Overview

This crate provides a driver for VirtIO-compatible graphics devices, enabling hardware-accelerated (or paravirtualized) framebuffers and 2D/3D command processing.

## Features

- **Framebuffer Initialization**: Set up display resolutions and pixel formats.
- **Resource Management**: Create and manage 2D graphics resources.
- **Transfer Commands**: Efficiently copy memory to the display device.
- **Cursor Support**: Managed hardware cursor positions.

## Integration

Used by the kernel's display subsystem to provide a graphical console:

```rust
let transport = detect_transport(device);
let gpu = VirtioGpu::new(transport);
gpu.init_display(1024, 768);
```

## Dependencies

- `virtio-drivers`: Core VirtIO implementation.
- `gpu`: LevitateOS GPU/Display traits.
