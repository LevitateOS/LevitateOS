# TEAM_316: Investigate x86_64 Boot Failure

## Bug Report

**Symptom:** x86_64 build completes but does not boot; aarch64 boots successfully

**Environment:**
- Platform: x86_64 (QEMU with Multiboot)
- Working: aarch64 boots to shell
- Failing: x86_64 crashes during boot

## Investigation Summary

### Issues Found and Fixed

1. **PMO Only Maps 1GB** - Assembly sets up Physical Memory Offset (PMO) mapping at 
   `0xFFFF800000000000` but only covers first 1GB of physical memory.
   
2. **APIC/IOAPIC Access Crash** - APIC at `0xFEE00000` and IOAPIC at `0xFEC00000` are 
   ~4GB addresses. `phys_to_virt()` returns addresses outside the 1GB PMO range.
   - **Fix:** Removed APIC mapping from `init_kernel_mappings()` (assembly already maps it)
   - **Fix:** Skipped APIC init in HAL `init_with_options()`
   - **Fix:** Skipped interrupt controller init in `init.rs` for x86_64
   - **Fix:** Changed `irq_dispatch` to use PIC EOI instead of APIC EOI

3. **PCI ECAM/MMIO Mapping Crash** - `ECAM_PA=0xB0000000`, `PCI_MEM32_PA=0xC0000000` 
   are also outside 1GB PMO range.
   - **Fix:** Removed from `init_kernel_mappings()`, deferred to PCI subsystem init

4. **Inline ASM Clobber Declarations** - Added `out("ax") _, out("dx") _` to all 
   diagnostic inline asm to prevent register corruption.

### Remaining Issue: Crash at 0x800200188

**Symptom:** After all above fixes, kernel crashes at address `0x800200188` during 
`TaskControlBlock::new_bootstrap()`.

**Evidence:**
- Simple loop of 100M iterations works AFTER syscall init
- Crash happens consistently at exact same address
- `instruction_pointer = accessed_address = 0x800200188` (execution at bad address)
- Stack pointer is valid (`0xFFFFFFFF8004E598`)
- Crash persists regardless of how struct is initialized

**Analysis:**
- Address `0x800200188` looks like truncated/corrupted 64-bit address
- Not stack overflow (loop works, stack pointer valid)
- Not timer interrupts (disabling PIT doesn't help)
- Systematic issue - always same address

### Files Modified

- `crates/hal/src/x86_64/mmu.rs` - Removed APIC/PCI mappings, added diagnostics
- `crates/hal/src/x86_64/mod.rs` - Skipped APIC init, added asm clobbers
- `crates/hal/src/x86_64/exceptions.rs` - Changed to PIC EOI
- `kernel/src/init.rs` - Skipped interrupt controller for x86_64
- `kernel/src/main.rs` - Added asm clobbers
- `kernel/src/arch/x86_64/mod.rs` - Added asm clobbers
- `kernel/src/task/mod.rs` - Various attempts to fix TCB creation

### Next Steps for Future Teams

1. **Investigate 0x800200188 address origin** - Check linker output, relocations
2. **Extend PMO to 4GB** - Proper fix for APIC/PCI access
3. **Check compiler codegen** - May be x86_64 specific optimization issue
4. **Verify boot stack size** - Currently 16KB, may need increase
5. **Binary analysis** - Disassemble around crash to find bad jump source

## Status: PARTIAL FIX

Multiple PMO-related issues fixed. Boot progresses much further but crashes at
`0x800200188` during TaskControlBlock creation. Requires deeper binary-level analysis.
