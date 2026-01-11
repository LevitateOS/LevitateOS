# TEAM_427: AArch64 Boot Investigation

## Status: PARTIAL - Handoff to Next Team

## Original Symptom

The AArch64 kernel produced no serial output when run locally with QEMU:

```bash
cargo xtask --arch aarch64 run
```

QEMU would run at 99% CPU with no visible output. The x86_64 kernel worked correctly, producing boot messages.

## Investigation Method

Used QEMU debug flags to trace execution:
```bash
qemu-system-aarch64 -d int,cpu_reset -D /tmp/debug.log ...
```

This revealed the exception history and register states during boot.

## Bugs Found and Fixed

### Bug 1: Virtual Base Address Constant Calculation (FIXED)

**Location**: `crates/kernel/arch/aarch64/src/asm/boot.S:80-92`

**Symptom**: Data Abort with malformed FAR `0x8000400f9010` (missing `0xFFFF` prefix)

**Root Cause**: The movz/movk sequence to build `0xFFFF_8000_0000_0000` was incorrect.

**Original code** (wrong):
```asm
movz x5, #0x8000, lsl #48  // This sets 0x8000_0000_0000_0000
movk x5, #0xFFFF, lsl #48  // This tries to keep, but overwrites!
```

**Fixed code**:
```asm
movz x5, #0xFFFF, lsl #48  // x5 = 0xFFFF_0000_0000_0000
movk x5, #0x8000, lsl #32  // x5 = 0xFFFF_8000_0000_0000
```

### Bug 2: FP/SIMD Not Enabled (FIXED)

**Location**: `crates/kernel/arch/aarch64/src/asm/boot.S:62-67`

**Symptom**: Undefined Instruction exception (EC=0x7) immediately after jumping to Rust code

**Root Cause**: CPACR_EL1.FPEN (bits 21:20) was not set, causing FP/SIMD instructions to trap. Rust code uses FP registers for various operations.

**Fix added**:
```asm
// Enable FP/SIMD (required for Rust code) FIRST
mov x9, #(3 << 20)
msr cpacr_el1, x9
isb
```

### Bug 3: Exception Vectors Not Set Early Enough (FIXED)

**Location**: `crates/kernel/arch/aarch64/src/asm/boot.S:225-230`

**Symptom**: When exceptions occurred, the handler code wasn't mapped or accessible

**Fix**: Added VBAR_EL1 setup after MMU enable, before jumping to Rust:
```asm
.extern vectors
ldr x4, =vectors
msr vbar_el1, x4
isb
```

## Current State After Fixes

The kernel now produces boot output:
```
[BOOT] Protocol: DeviceTree
[BOOT] Memory: 1 regions, 2048 MB usable
[BOOT] Stage 2: Memory & MMU (PEI)
[MEM] Physical: 0x40000000 - 0xc0000000 (524288 pages, 2048 MB)
```

Then crashes with:
```
Taking exception 4 [Data Abort] on CPU 0
...with ESR 0x25/0x96000044
...with FAR 0xffff800040082000
...with ELR 0xffff8000400c7980
```

Followed by infinite Prefetch Abort loop at the exception vector address.

## Exception Analysis

### ESR 0x96000044 Breakdown
- EC (bits 31:26) = 0x25 = 37 = Data Abort from same EL
- ISS bit 6 (WnR) = 1 = Write access
- DFSC (bits 5:0) = 0x04 = Translation fault, level 0

### Faulting Addresses
- **FAR**: `0xffff800040082000` - The virtual address being accessed
  - Physical equivalent: `0x40082000`
  - This is within the boot page tables region (`__boot_l0_ttbr0`)
- **ELR**: `0xffff8000400c7980` - The instruction that faulted
  - This is in `memset` during `memory::init()`
- **Exception Vector**: `0xffff800040085200` - Also faults with L0 translation fault

### Page Table Layout (from objdump)
```
Symbol                      Physical Address
_start                      0x40080000
__bss_boot_start            0x40080160
__boot_page_tables_start    0x40081000
__boot_l0_ttbr1            0x40081000
__boot_l0_ttbr0            0x40082000
__boot_l1_high             0x40083000
__boot_l1_low              0x40084000
__boot_page_tables_end     0x40085000
vectors (virtual)          0xffff800040085000
```

### Key Observation

The L0 translation fault means the MMU cannot find a valid entry at L0 index 256 (which covers the `0xFFFF_8000_*` address range). The entry should be at physical address `0x40081800` (L0_ttbr1 + 256*8).

The boot.S code writes to L0[256]:
```asm
mov x3, x12                 // L1_high physical address = 0x40083000
orr x3, x3, #TABLE_FLAGS    // = 0x40083003
str x3, [x10, #(256 * 8)]   // Store at 0x40081000 + 0x800 = 0x40081800
```

## Cache Coherency Concern

The last fix attempted was improving cache invalidation. The DC CIVAC instruction only invalidates one 64-byte cache line, but the original code was invalidating the wrong addresses:

- L0_ttbr1[256] is at offset 0x800 (2048 bytes) from table start
- The original code did `dc civac, x10` where x10 = start of table
- This invalidated the wrong cache line (32 cache lines apart from the entry)

A partial fix was added but not tested:
```asm
add x3, x10, #(256 * 8)     // Address of L0_ttbr1[256]
dc civac, x3                // Invalidate the correct cache line
```

## Files Modified

All changes are in: `crates/kernel/arch/aarch64/src/asm/boot.S`

Search for `TEAM_427` comments to find all modifications.

## Memory Layout Reference

From `crates/kernel/levitate/src/arch/aarch64/linker.ld`:
- Kernel physical load: `0x40080000`
- Higher-half virtual base: `0xFFFF_8000_0000_0000`
- Boot page tables: 4 x 4KB tables for L0/L1 TTBR0/TTBR1

## Debugging Commands

```bash
# Build kernel
cargo xtask build kernel --arch aarch64

# Binary is at kernel64_rust.bin (not target/...)

# Run with debug tracing
qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a53 \
  -m 2048 \
  -nographic \
  -kernel kernel64_rust.bin \
  -d int,cpu_reset

# Check symbol addresses
aarch64-linux-gnu-objdump -t crates/kernel/target/aarch64-unknown-none/release/levitate-kernel | grep -E "__boot|_start|vectors"
```

## Technical References

- AArch64 Address Translation: ARM ARM D5 "The AArch64 Virtual Memory System Architecture"
- TCR_EL1 configuration: T0SZ=16, T1SZ=16 for 48-bit VA with 4KB granule
- L0 index calculation: bits [47:39] of virtual address
- DC CIVAC: Clean and Invalidate by VA to Point of Coherency (one cache line)

## Questions for Next Team

1. Why is the L0 entry not being found after MMU enable?
2. Is there an issue with how QEMU handles the early page table memory region?
3. Could there be a timing issue between the store to L0[256] and the cache invalidation?
4. Are there other ARM barriers or cache operations needed?
