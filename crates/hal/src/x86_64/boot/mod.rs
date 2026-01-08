//! x86_64 Boot Compartment
//!
//! This module handles boot protocol parsing:
//! - **Multiboot2** - GRUB/Multiboot2 boot info parsing

pub mod multiboot2;

pub use multiboot2::*;
