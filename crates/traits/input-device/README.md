# input-device

Input device trait for LevitateOS.

## Overview

This crate defines the `InputDevice` trait, providing a common interface for all input devices in the kernel:

- VirtIO Input devices (`virtio-input`)
- PS/2 keyboard/mouse (future)
- USB HID devices (future)

## Design Pattern

Inspired by [Theseus OS](https://github.com/theseus-os/Theseus) device trait patterns. Separating traits from implementations enables non-VirtIO input drivers to implement the same interfaces.

## Kernel SOP Alignment

- **Rule 1 (Modular Scope):** This crate handles exactly one task: defining input device interface.
- **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
- **Rule 9 (Asynchrony):** Non-blocking `poll()` method.
- **Rule 11 (Separation):** Trait defines mechanism (raw input), not policy (keymaps).

## Usage

```rust
use input_device::{InputDevice, InputDeviceRef};

// Poll for input
if device.poll() {
    if let Some(ch) = device.read_char() {
        // Handle character
    }
}
```

## Created By

TEAM_334 as part of VirtIO driver refactor (Phase 2 Step 1).
