//! x86_64 Boot Compartment
//!
//! TEAM_316: Simplified to Limine-only boot.
//!
//! Limine provides everything we need:
//! - Memory map
//! - HHDM (Higher Half Direct Map)
//! - Framebuffer
//! - Kernel/module addresses
//!
//! The actual Limine protocol handling is in kernel/src/boot/limine.rs
