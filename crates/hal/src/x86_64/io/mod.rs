//! x86_64 I/O Compartment
//!
//! This module handles input/output devices:
//! - **Serial** - COM1 serial port for early debug output
//! - **VGA** - Text-mode VGA buffer for screen output
//! - **Console** - Unified console writer abstraction

pub mod serial;
pub mod vga;
pub mod console;

pub use serial::SerialPort;
pub use vga::{Color, ColorCode};
pub use console::WRITER;
