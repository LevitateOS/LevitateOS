//! x86_64 Hardware Abstraction Layer
//!
//! # Compartments
//!
//! The x86_64 HAL is organized into logical compartments:
//!
//! - **cpu/** - CPU structures (GDT, TSS, IDT, exceptions)
//! - **mem/** - Memory management (paging, MMU, frame allocator)
//! - **interrupts/** - Interrupt handling (APIC, IOAPIC, PIT)
//! - **io/** - I/O devices (serial, VGA, console)
//! - **boot/** - Boot protocols (Multiboot2)
//!
//! See the README.md in this directory for architecture diagrams.

use crate::traits::InterruptController;

// === Compartments ===
pub mod cpu;
pub mod mem;
pub mod interrupts;
pub mod io;
pub mod boot;

// === Re-exports for backward compatibility ===
// CPU
pub use cpu::gdt;
pub use cpu::idt;
pub use cpu::exceptions;
pub mod tss {
    pub use crate::x86_64::cpu::gdt::*;
}

// Memory
pub use mem::paging;
pub use mem::mmu;
pub use mem::frame_alloc;

// Interrupts
pub use interrupts::apic;
pub use interrupts::ioapic;
pub use interrupts::pit;

// I/O
pub use io::serial;
pub use io::vga;
pub use io::console;

// Boot - TEAM_316: Limine-only, no multiboot re-exports needed

/// TEAM_316: Initialize HAL for Limine boot (simplified, Unix philosophy).
///
/// Limine provides:
/// - Correct page tables with HHDM mapping
/// - Memory map
/// - Framebuffer (optional)
///
/// We just need to initialize our CPU structures and drivers.
pub fn init() {
    // 1. Initialize serial for early logging
    unsafe {
        core::arch::asm!("mov dx, 0x3f8", "mov al, 'd'", "out dx, al", out("ax") _, out("dx") _);
    }
    unsafe { console::WRITER.lock().init() };

    // 2. Initialize GDT, IDT and exceptions
    unsafe {
        core::arch::asm!("mov al, 'e'", "out dx, al", out("ax") _, out("dx") _);
    }
    unsafe { gdt::init() };
    idt::init();
    exceptions::init();

    // 3. APIC/IOAPIC - SKIP for now
    // TEAM_317: Limine HHDM only maps RAM, not MMIO regions like APIC (0xFEE00000).
    // phys_to_virt(0xFEE00000) returns unmapped address, causing page fault.
    // TODO: Map APIC region explicitly before enabling APIC mode.
    // For now, use legacy PIC mode (PIT timer on IRQ0).
    unsafe {
        core::arch::asm!("mov al, 'f'", "out dx, al", out("ax") _, out("dx") _);
    }

    // 4. Initialize PIT timer (legacy mode, works without APIC)
    unsafe {
        core::arch::asm!("mov al, 'g'", "out dx, al", out("ax") _, out("dx") _);
    }
    pit::Pit::init(100); // 100Hz

    // Done
    unsafe {
        core::arch::asm!("mov al, 'h'", "out dx, al", out("ax") _, out("dx") _);
    }
}
