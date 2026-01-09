//! # input-device
//!
//! Input device trait for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 1).
//!
//! ## Design Pattern
//!
//! Inspired by Theseus OS device trait patterns. Separating traits from implementations
//! enables non-VirtIO input drivers (PS2 keyboard, USB HID, etc.) to implement the same interfaces.
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one task: defining input device interface.
//! - **Rule 2 (Type-Driven Composition):** Uses traits to define interfaces consumable by other subsystems.
//! - **Rule 6 (Robust Error Handling):** Operations that can fail return `Option` or `Result`.

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

use alloc::sync::Arc;
use spin::Mutex;

/// TEAM_334: Input event types.
///
/// Following kernel SOP Rule 13 (Representation): Use enums to encode state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEventType {
    /// Key press event
    KeyDown,
    /// Key release event
    KeyUp,
    /// Mouse movement
    MouseMove,
    /// Mouse button press
    MouseDown,
    /// Mouse button release
    MouseUp,
}

/// TEAM_334: Raw input event from device.
///
/// Contains the event type and associated data (keycode, coordinates, etc.)
#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    /// Type of input event
    pub event_type: InputEventType,
    /// Event code (keycode for keyboard, button for mouse)
    pub code: u16,
    /// Event value (1 for press, 0 for release, delta for movement)
    pub value: i32,
}

/// Trait for input devices (keyboards, mice, touchpads, etc.)
///
/// TEAM_334: Following Theseus device trait patterns.
///
/// ## Kernel SOP Alignment
///
/// - **Rule 2 (Composition):** Trait-based interface for orthogonal subsystems
/// - **Rule 9 (Asynchrony):** `poll()` is non-blocking, returns immediately
/// - **Rule 11 (Separation):** Trait defines mechanism (raw input), not policy (keymaps)
pub trait InputDevice: Send {
    /// Poll for pending input events.
    ///
    /// Returns `true` if there were events to process, `false` if the queue was empty.
    /// This method is non-blocking per kernel SOP Rule 9.
    fn poll(&mut self) -> bool;

    /// Read next character from keyboard buffer.
    ///
    /// Returns `None` if no character is available.
    /// This is a convenience method for text input; use `poll_event()` for raw events.
    fn read_char(&mut self) -> Option<char>;

    /// Check if Ctrl+C was pressed (for interrupt handling).
    ///
    /// This is polled by the kernel for process interruption.
    fn ctrl_c_pressed(&self) -> bool;

    /// Read next raw input event.
    ///
    /// Returns `None` if no event is available.
    fn poll_event(&mut self) -> Option<InputEvent> {
        // Default implementation - devices that don't support raw events return None
        None
    }
}

/// Thread-safe reference to an input device.
///
/// TEAM_334: Following Theseus pattern for device references.
pub type InputDeviceRef = Arc<Mutex<dyn InputDevice + Send>>;

#[cfg(test)]
mod tests {
    use super::*;

    // TEAM_334: Mock device for testing
    struct MockInputDevice {
        chars: alloc::collections::VecDeque<char>,
        ctrl_c: bool,
    }

    impl MockInputDevice {
        fn new() -> Self {
            Self {
                chars: alloc::collections::VecDeque::new(),
                ctrl_c: false,
            }
        }
    }

    impl InputDevice for MockInputDevice {
        fn poll(&mut self) -> bool {
            !self.chars.is_empty()
        }

        fn read_char(&mut self) -> Option<char> {
            self.chars.pop_front()
        }

        fn ctrl_c_pressed(&self) -> bool {
            self.ctrl_c
        }
    }

    #[test]
    fn test_mock_input_device() {
        let mut device = MockInputDevice::new();
        assert!(!device.poll());
        assert_eq!(device.read_char(), None);
    }
}
