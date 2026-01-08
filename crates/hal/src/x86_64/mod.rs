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

// Boot
pub use boot::multiboot2;

/// TEAM_286: Initialize HAL with optional CR3 switch.
/// `switch_cr3`: Set to false for Limine boot (Limine's page tables are already correct).
/// When false, APIC/IOAPIC init is also skipped (Limine may not identity-map APIC region).
pub fn init_with_options(switch_cr3: bool) {
    let is_limine = !switch_cr3; // If not switching CR3, we're on Limine
    // 0. Initialize MMU with higher-half mappings using early_pml4
    unsafe extern "C" {
        static mut early_pml4: paging::PageTable;
    }

    if switch_cr3 {
        // TEAM_308: Diagnostic 'a' - init_kernel_mappings Start
        unsafe {
            core::arch::asm!("mov dx, 0x3f8", "mov al, 'a'", "out dx, al", out("ax") _, out("dx") _);
        }
        unsafe {
            let root = &mut *core::ptr::addr_of_mut!(early_pml4);
            mmu::init_kernel_mappings(root);

            // TEAM_308: Diagnostic 'b' - init_kernel_mappings Done
            core::arch::asm!("mov al, 'b'", "out dx, al", out("ax") _, out("dx") _);

            // TEAM_285: Switch to our own page tables now that they are initialized.
            // This is safer than doing it in assembly because we have verified mappings.
            let phys = mmu::virt_to_phys(root as *const _ as usize);
            core::arch::asm!("mov cr3, {}", in(reg) phys);

            // TEAM_308: Diagnostic 'c' - CR3 Switched
            core::arch::asm!("mov al, 'c'", "out dx, al", out("ax") _, out("dx") _);
        }
    }
    // else: Limine boot - stay on Limine's page tables which have correct HHDM

    // 1. Initialize serial for early logging
    // TEAM_308: Diagnostic 'd' - Serial Init
    unsafe {
        core::arch::asm!("mov dx, 0x3f8", "mov al, 'd'", "out dx, al", out("ax") _, out("dx") _);
    }
    unsafe { console::WRITER.lock().init() };

    // 2. Initialize GDT, IDT and exceptions
    // TEAM_308: Diagnostic 'e' - GDT/IDT Init
    unsafe {
        core::arch::asm!("mov al, 'e'", "out dx, al", out("ax") _, out("dx") _);
    }
    unsafe { gdt::init() };
    idt::init();
    exceptions::init();

    // 3. Initialize APIC and IOAPIC
    // TEAM_308: Diagnostic 'f' - APIC Init
    unsafe {
        core::arch::asm!("mov dx, 0x3f8", "mov al, 'f'", "out dx, al", out("ax") _, out("dx") _);
    }
    // TEAM_316: Skip APIC init for now - APIC code uses phys_to_virt() which fails
    // because 0xFEE00000 is outside the 1GB PMO range. Assembly already identity-maps
    // APIC region, so basic APIC access works through identity mapping.
    // TODO: Fix APIC code to use identity-mapped access for Multiboot boot.
    let _ = is_limine; // Suppress unused warning

    // 4. Initialize PIT
    // TEAM_308: Diagnostic 'g' - PIT Init
    unsafe {
        core::arch::asm!("mov dx, 0x3f8", "mov al, 'g'", "out dx, al", out("ax") _, out("dx") _);
    }
    // TEAM_316: Skip PIT init temporarily - timer interrupts may be causing crash
    // pit::Pit::init(100); // 100Hz

    // TEAM_308: Diagnostic 'h' - HAL Init Done
    unsafe {
        core::arch::asm!("mov al, 'h'", "out dx, al", out("ax") _, out("dx") _);
    }
}

/// TEAM_286: Default init for multiboot boot (switches CR3).
pub fn init() {
    init_with_options(true)
}
