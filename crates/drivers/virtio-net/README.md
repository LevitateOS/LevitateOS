# virtio-net

VirtIO Network device driver for LevitateOS.

## Overview

This crate provides a driver for VirtIO network interfaces, enabling Ethernet communication and packet processing in LevitateOS.

## Features

- **Packet Transmission/Reception**: Low-level handling of network frames.
- **MAC Address Management**: Retrieval and configuration of device hardware addresses.
- **Interrupt Processing**: Efficient handling of RX/TX completion interrupts.

## Integration

Initialized by the kernel when a network card is detected:

```rust
let transport = detect_transport(device);
virtio_net::init(transport);
```

## Dependencies

- `virtio-drivers`: Core VirtIO implementation.
- `network-device`: LevitateOS network traits.
