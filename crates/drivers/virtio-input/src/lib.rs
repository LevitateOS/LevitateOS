//! # virtio-input
//!
//! VirtIO Input driver for LevitateOS.
//!
//! TEAM_334: Created as part of VirtIO driver refactor (Phase 2 Step 3).
//!
//! ## Design
//!
//! This crate provides a transport-agnostic VirtIO input driver that:
//! - Implements the `InputDevice` trait for use with the kernel's input subsystem
//! - Handles Linux keycode to ASCII mapping
//! - Manages keyboard buffer and modifier key state
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles exactly one device: VirtIO Input.
//! - **Rule 2 (Type-Driven Composition):** Implements `InputDevice` trait.
//! - **Rule 11 (Separation):** Driver provides mechanism; kernel provides policy (interrupt handling).

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

pub mod keymap;

use alloc::collections::VecDeque;
use input_device::{InputDevice, InputEvent};
use spin::Mutex;

/// Linux event type for key events
pub const EV_KEY: u16 = 1;
/// Linux event type for absolute position events (touch/mouse)
pub const EV_ABS: u16 = 3;

/// Linux keycode for left shift
pub const KEY_LEFTSHIFT: u16 = 42;
/// Linux keycode for right shift
pub const KEY_RIGHTSHIFT: u16 = 54;
/// Linux keycode for enter
pub const KEY_ENTER: u16 = 28;
/// Linux keycode for backspace
pub const KEY_BACKSPACE: u16 = 14;
/// Linux keycode for space
pub const KEY_SPACE: u16 = 57;
/// Linux keycode for tab
pub const KEY_TAB: u16 = 15;
/// Linux keycode for left control
pub const KEY_LEFTCTRL: u16 = 29;
/// Linux keycode for right control
pub const KEY_RIGHTCTRL: u16 = 97;
/// Linux keycode for C key
pub const KEY_C: u16 = 46;

/// TEAM_334: VirtIO Input device state.
///
/// This struct manages keyboard buffer and modifier key state.
/// The actual `VirtIOInput` device is held by the kernel's driver wrapper
/// since it requires architecture-specific HAL.
pub struct VirtioInputState {
    /// Character buffer for keyboard input
    keyboard_buffer: VecDeque<char>,
    /// Maximum buffer size
    buffer_capacity: usize,
    /// Shift key state
    shift_pressed: bool,
    /// Control key state
    ctrl_pressed: bool,
    /// Ctrl+C was detected
    ctrl_c_pending: bool,
}

impl VirtioInputState {
    /// Create a new input state with specified buffer capacity.
    pub fn new(buffer_capacity: usize) -> Self {
        Self {
            keyboard_buffer: VecDeque::with_capacity(buffer_capacity),
            buffer_capacity,
            shift_pressed: false,
            ctrl_pressed: false,
            ctrl_c_pending: false,
        }
    }

    /// TEAM_334: Create a new input state with const-compatible initialization.
    /// Used for static initialization in kernel.
    pub const fn const_new(buffer_capacity: usize) -> Self {
        Self {
            keyboard_buffer: VecDeque::new(),
            buffer_capacity,
            shift_pressed: false,
            ctrl_pressed: false,
            ctrl_c_pending: false,
        }
    }

    /// Check if keyboard buffer is empty.
    pub fn keyboard_buffer_is_empty(&self) -> bool {
        self.keyboard_buffer.is_empty()
    }

    /// Process a VirtIO input event.
    ///
    /// Call this from the kernel's poll loop for each event from the VirtIO device.
    /// Returns `true` if Ctrl+C was detected (for kernel to handle signal delivery).
    pub fn process_event(&mut self, event_type: u16, code: u16, value: i32) -> bool {
        let mut ctrl_c_detected = false;

        match event_type {
            EV_KEY => {
                let pressed = value != 0;
                match code {
                    KEY_LEFTSHIFT | KEY_RIGHTSHIFT => {
                        self.shift_pressed = pressed;
                    }
                    KEY_LEFTCTRL | KEY_RIGHTCTRL => {
                        self.ctrl_pressed = pressed;
                    }
                    _ if pressed => {
                        if self.ctrl_pressed && code == KEY_C {
                            // Ctrl+C detected
                            self.push_char('\x03');
                            ctrl_c_detected = true;
                            self.ctrl_c_pending = true;
                        } else if let Some(c) =
                            keymap::linux_code_to_ascii(code, self.shift_pressed)
                        {
                            self.push_char(c);
                        }
                    }
                    _ => {}
                }
            }
            EV_ABS => {
                // Mouse/touch events - currently not handled
                // Future: implement mouse cursor support
            }
            _ => {}
        }

        ctrl_c_detected
    }

    /// Push a character to the keyboard buffer.
    ///
    /// Returns `true` if successful, `false` if buffer is full.
    fn push_char(&mut self, c: char) -> bool {
        if self.keyboard_buffer.len() < self.buffer_capacity {
            self.keyboard_buffer.push_back(c);
            true
        } else {
            // Buffer overflow
            log::trace!("virtio-input: keyboard buffer overflow");
            false
        }
    }

    /// Check and clear the Ctrl+C pending flag.
    pub fn take_ctrl_c(&mut self) -> bool {
        let was_pending = self.ctrl_c_pending;
        self.ctrl_c_pending = false;
        was_pending
    }
}

impl InputDevice for VirtioInputState {
    fn poll(&mut self) -> bool {
        // Polling is handled by the kernel wrapper which calls process_event()
        // This returns whether there's data in the buffer
        !self.keyboard_buffer.is_empty()
    }

    fn read_char(&mut self) -> Option<char> {
        self.keyboard_buffer.pop_front()
    }

    fn ctrl_c_pressed(&self) -> bool {
        self.ctrl_c_pending
    }

    fn poll_event(&mut self) -> Option<InputEvent> {
        // Raw events are processed immediately in process_event()
        // This could be extended to queue raw events if needed
        None
    }
}

/// TEAM_334: Thread-safe wrapper for VirtioInputState.
pub type SharedInputState = Mutex<VirtioInputState>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_input() {
        let mut state = VirtioInputState::new(256);

        // Simulate pressing 'a' (keycode 30)
        state.process_event(EV_KEY, 30, 1); // press
        state.process_event(EV_KEY, 30, 0); // release

        assert_eq!(state.read_char(), Some('a'));
        assert_eq!(state.read_char(), None);
    }

    #[test]
    fn test_shift_modifier() {
        let mut state = VirtioInputState::new(256);

        // Simulate Shift+A
        state.process_event(EV_KEY, KEY_LEFTSHIFT, 1); // shift press
        state.process_event(EV_KEY, 30, 1); // 'a' press
        state.process_event(EV_KEY, 30, 0); // 'a' release
        state.process_event(EV_KEY, KEY_LEFTSHIFT, 0); // shift release

        assert_eq!(state.read_char(), Some('A'));
    }

    #[test]
    fn test_ctrl_c_detection() {
        let mut state = VirtioInputState::new(256);

        // Simulate Ctrl+C
        state.process_event(EV_KEY, KEY_LEFTCTRL, 1); // ctrl press
        let ctrl_c = state.process_event(EV_KEY, KEY_C, 1); // 'c' press

        assert!(ctrl_c);
        assert!(state.ctrl_c_pressed());
        assert_eq!(state.read_char(), Some('\x03'));

        // Take clears the flag
        assert!(state.take_ctrl_c());
        assert!(!state.ctrl_c_pressed());
    }

    #[test]
    fn test_buffer_overflow() {
        let mut state = VirtioInputState::new(3);

        // Fill buffer
        state.process_event(EV_KEY, 30, 1); // a
        state.process_event(EV_KEY, 31, 1); // s
        state.process_event(EV_KEY, 32, 1); // d

        // This should overflow
        state.process_event(EV_KEY, 33, 1); // f

        // Only first 3 characters should be in buffer
        assert_eq!(state.read_char(), Some('a'));
        assert_eq!(state.read_char(), Some('s'));
        assert_eq!(state.read_char(), Some('d'));
        assert_eq!(state.read_char(), None);
    }
}
