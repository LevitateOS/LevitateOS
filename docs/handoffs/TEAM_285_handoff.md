# TEAM_285: Boot Abstraction & Limine Migration Handoff (Part 2)

Building on TEAM_284's work, I investigated and resolved the "LH" hang and subsequent page faults.

## 1. Accomplishments & Knowledge Discovered

### **1.1. Page Table Fixes & Transitions**
- **Safe CR3 Switch**: I moved the `CR3` reload from `boot.S` assembly to `los_hal::arch::init` in Rust. This ensures we only switch once our new page tables are fully initialized and verified.
- **APIC Mapping Correction**: Discovered a critical bug where `boot.S` was mapping Local APIC (`0xFEE00000`) to the physical address of the IOAPIC (`0xFEC00000`). I corrected this to map both regions as 2MB huge pages.
- **Dynamic PMO**: `PHYS_OFFSET` is now a dynamic `static mut` in `los_hal::arch::mmu`. It is initialized in `kernel_main_unified` from Limine's HHDM offset. This is essential because Limine does not guarantee a fixed offset.

### **1.2. Linker & Protocol Compliance**
- **Writable Requests**: Moved `.requests` section to `.boot_data` (RW) in `linker.ld`. Limine writes responses into these structures, so they cannot be in a Read-Only segment.
- **Multiboot1 Alignment**: Fixed the `multiboot1_header` load address to `0x200000` to match the kernel's physical offset.

### **1.3. Diagnostic Progress**
- Added a sequence of serial diagnostic characters to track the boot flow:
  - `L`, `H`, `1..9`: `boot.S` progress.
  - `C`: CR3 load skipped in assembly (intentional).
  - `X`: Entered arch-specific `kernel_main`.
  - `L`/`M`: Detected Limine or Multiboot.
  - `P`: Parse done.
  - `R`: Entered `kernel_main_unified`.
  - `K`: HAL initialization complete (IDT, APIC, etc.).

---

## 2. Current State & Remaining TODOs

### **2.1. The "Serial Silence" Bug**
The kernel now successfully initializes the HAL and reaches the transition to the unified main. However, `cargo xtask test behavior` currently reports **0 bytes of output**.
- **Symptoms**: Serial output seems to "die" after the transition to kernel page tables or during early HAL init.
- **Investigation Needed**: Verify COM1 mapping in `mmu.rs`. Although we map `0..1MB` as identity, the port `0x3f8` is I/O space, not memory-mapped. However, if the kernel is running in a different context, the `out` instructions might be behaving unexpectedly or the diagnostic characters are overwriting each other.

### **2.2. Critical Path**
- [ ] **Restore Serial**: Fix why output stops after `K`.
- [ ] **Memory Mapping**: Complete the integration of Limine's memory map into the `BootInfo` struct passed to `crate::memory::init`.
- [ ] **HAL Refactor**: Use the new libraries (see below) to replace manual bit-shifting in `apic.rs` and `ioapic.rs`.

---

## 3. Recommended Library Migration

We should stop "rolling our own" for standard PC hardware. I recommend migrating to these crates:

- **`uart_16550`**: Replaces our manual `SerialPort` implementation. It's safer and handles edge cases better.
- **`pic8259`**: For handling the legacy PIC if needed (though we use APIC).
- **`pc-keyboard`**: To handle PS/2 keyboard scancodes properly.
- **`x86_64` (Full Usage)**: We have the crate, but we're still manually writing assembly for some things. We should use its `structures::paging` and `instructions` modules exclusively.
- **`raw-cpuid`**: For feature detection instead of manual `asm!`.

---

**Status:** Handoff Complete. 
**Team ID:** TEAM_285 
**Files Modified:** `kernel/src/arch/x86_64/boot.S`, `linker.ld`, `kernel/src/main.rs`, `crates/hal/src/x86_64/mmu.rs`, `crates/hal/src/x86_64/mod.rs`.
