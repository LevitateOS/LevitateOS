# virtio-blk

VirtIO Block device driver for LevitateOS.

## Overview

This crate provides a high-level driver for VirtIO block devices, enabling the kernel to perform disk I/O operations. It leverages the `virtio-drivers` crate and integrates with the LevitateOS `virtio-transport` layer.

## Features

- **Read/Write Operations**: Asynchronous and synchronous block access.
- **Capacity Detection**: Automatically detects Disk size and geometry.
- **Queue Management**: Efficient handling of VirtIO request and response queues.

## Integration

The driver is initialized during the kernel boot sequence when a block device is detected on the VirtIO bus:

```rust
let transport = detect_transport(device);
virtio_blk::init(transport);
```

## Dependencies

- `virtio-drivers`: Core VirtIO implementation.
- `virtio-transport`: LevitateOS transport abstraction (MMIO/PCI).
- `storage-device`: LevitateOS storage traits.
