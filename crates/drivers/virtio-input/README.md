# virtio-input

VirtIO Input driver for LevitateOS.

## Overview

This crate provides a transport-agnostic VirtIO input driver that:

- Implements the `InputDevice` trait for use with the kernel's input subsystem
- Handles Linux keycode to ASCII mapping via the `keymap` module
- Manages keyboard buffer and modifier key state (shift, ctrl)
- Detects Ctrl+C for process interruption

## Design

The driver is split into two layers:

1. **`VirtioInputState`** (this crate) - Handles input processing, buffering, and keymap
2. **Kernel wrapper** (`kernel/src/input.rs`) - Handles VirtIO device, HAL, interrupts

This separation follows kernel SOP Rule 11 (Separation of Mechanism and Policy).

## Kernel SOP Alignment

- **Rule 1 (Modular Scope):** Handles exactly one device type: VirtIO Input.
- **Rule 2 (Type-Driven Composition):** Implements `InputDevice` trait.
- **Rule 11 (Separation):** Driver provides mechanism; kernel provides policy (interrupt handling, signal delivery).
- **Rule 13 (Representation):** Uses match for exhaustive keycode handling.

## Usage

```rust
use virtio_input::{VirtioInputState, EV_KEY};
use input_device::InputDevice;

// Create input state
let mut state = VirtioInputState::new(1024);

// Process events from VirtIO device
let ctrl_c = state.process_event(EV_KEY, 30, 1); // 'a' key press

// Read characters
while let Some(c) = state.read_char() {
    // Handle character
}
```

## Modules

- `keymap` - Linux keycode to ASCII mapping

## Created By

TEAM_334 as part of VirtIO driver refactor (Phase 2 Step 3).
