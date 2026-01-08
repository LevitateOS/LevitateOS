# Higher-Half Kernel Investigation Notes

**Teams:** TEAM_024 (Review), TEAM_025 (Implementation Attempt)  
**Status:** INCOMPLETE — Requires Future Investigation  
**Last Updated:** 2026-01-03

---

## Problem Statement

LevitateOS currently runs as an identity-mapped kernel (VA == PA). The goal is to implement a **higher-half kernel** where:
- Boot code runs at physical address (identity mapped)
- Kernel code runs at high virtual address (0x0000FFFF80000000+)
- User space (future) uses low virtual addresses

This separation is standard practice for production kernels and enables proper user/kernel memory isolation.

---

## Current Kernel State

The working kernel uses:
- **Single memory region:** `0x40080000` (physical)
- **Boot assembly:** Inline in `kernel/src/main.rs` via `global_asm!()`
- **MMU setup:** Done in Rust (`levitate-hal::mmu`) AFTER entering `kmain`
- **All code identity-mapped:** VA == PA for all addresses

---

## The Bug: Code Execution Fails at High Virtual Address

### Symptoms
1. MMU enables successfully (boot prints `ABCD`)
2. **Data reads from high VA work** — can `ldr` from high address
3. **Code execution from high VA fails** — `br x0` to high address causes exception
4. Exception: "Undefined Instruction" with ESR EC=0 ("Unknown reason")

### Why This Is Strange
- If page table was wrong, data read would also fail
- Data read succeeds → translation is working
- But instruction fetch fails → something specific to execute permission

---

## Technical Details

### Target Address Space (TTBR0 approach)
```
Virtual Address: 0x0000_FFFF_8000_0000+
                 ^^^^
                 Top 16 bits = 0x0000 → uses TTBR0
                 
Page Table Indices for 0x0000_FFFF_8000_0000:
- L0 index: bits[47:39] = 511
- L1 index: bits[38:30] = 510  
- L2 index: bits[29:21] = 0
- Page offset: bits[20:0] = offset within 2MB block
```

### Page Table Structure Attempted
```
L0[0]   → L1_low  (identity map for 0x00000000-0x7FFFFFFFFF)
L0[511] → L1_high (higher-half map for 0x0000FFFF80000000+)

L1_low[0]  → L2_devices (0x00000000-0x3FFFFFFF)
L1_low[1]  → L2_ram     (0x40000000-0x7FFFFFFF)

L1_high[510] → L2_high_ram (maps to same physical 0x40000000+)
```

### Page Table Entry Flags Used
```
RAM entries:     0x701 = Valid | AF | SH_INNER (no PXN/UXN)
Device entries:  0x6000_0000_0000_0405 = Valid | AF | Attr1 | PXN | UXN
```

### TCR_EL1 Configuration
```
T0SZ = 16      (48-bit VA for TTBR0)
EPD1 = 1       (Disable TTBR1 walks)
TG0  = 0       (4KB granule)
IPS  = 0b101   (48-bit PA)
```

### SCTLR_EL1 Configuration
```
M = 1   (MMU enable)
C = 1   (D-cache enable)  
I = 1   (I-cache enable)
```

---

## Debugging Steps Performed

### 1. Verified Page Table Setup Compiles ✓
Assembly page table code compiled without errors.

### 2. Verified MMU Enables ✓
Boot code prints `ABCD` — MMU enable completes without fault.

### 3. Verified Data Read from High VA ✓
```asm
ldr     x0, =kmain_high    /* High VA address */
ldr     w1, [x0]           /* Read data - WORKS */
```

### 4. Verified Physical Memory Content ✓
```bash
xxd -s 0x9870 -l 16 kernel64_rust.bin
# Shows correct instruction bytes (d2a12009 = mov x9, #0x9000000)
```

### 5. Code Execution Fails ✗
```asm
br      x0                 /* Jump to high VA - FAILS */
```
QEMU exception log:
```
Taking exception 1 [Undefined Instruction] on CPU 0
...with ESR 0x0/0x2000000
...with ELR 0xffff80089870
```

### 6. Tried Cache Invalidation ✗
```asm
tlbi    vmalle1
ic      iallu
dsb     sy
isb
```
Did not resolve the issue.

---

## Hypotheses (Not Yet Tested)

### Hypothesis 1: QEMU CPU Model Issue
The cortex-a53 model may have specific requirements. Try:
```bash
qemu-system-aarch64 -cpu max ...
```

### Hypothesis 2: Missing SCTLR Bits
Other bits in SCTLR_EL1 may be required:
- SA (Stack Alignment check)
- nTWI/nTWE (WFI/WFE trapping)
- Check ARMv8 reference manual for I-fetch requirements

### Hypothesis 3: Memory Type Mismatch
MAIR attributes may need adjustment for instruction fetch:
```
Current: Attr0 = 0xFF (Normal, Write-Back)
Try:     Attr0 = 0x44 (Normal, Non-Cacheable)
```

### Hypothesis 4: Page Table Address Alignment
Page tables may need specific alignment beyond 4KB.

---

## Files to Examine

1. **Working kernel:** `kernel/src/main.rs` (inline assembly)
2. **Linker script:** `linker.ld`
3. **MMU module:** `levitate-hal/src/mmu.rs`
4. **Reference implementations:**
   - `.external-kernels/theseus/kernel/nano_core/src/asm/bios/boot.asm`
   - `.external-kernels/theseus/kernel/nano_core/linker_higher_half-aarch64.ld`
   - `.external-kernels/redox-kernel/linkers/aarch64.ld`

---

## Recommended Debugging Approach

### Step 1: Enable QEMU Debug Logging
```bash
qemu-system-aarch64 \
    -d int,mmu,guest_errors \
    -D qemu_debug.log \
    ...
```
This will show:
- Translation table walks
- Page table entry values
- Exact fault details

### Step 2: Create Minimal Test Case
Create a minimal assembly-only kernel that ONLY:
1. Sets up page tables
2. Enables MMU
3. Jumps to high address
4. Prints success character

Remove all Rust code to isolate the issue.

### Step 3: Compare with Working Higher-Half Kernel
Run Theseus in QEMU and examine its:
- TCR_EL1 value
- SCTLR_EL1 value
- Page table entry format
- Boot sequence

### Step 4: Single-Step in GDB
```bash
qemu-system-aarch64 -s -S ...
# In another terminal:
gdb-multiarch target/aarch64-unknown-none/release/levitate-kernel
(gdb) target remote :1234
(gdb) break *0x400800dc  # Before br x0
(gdb) c
(gdb) si  # Single step into the jump
(gdb) info registers
```

---

## Key Insight from TEAM_025

**Data reads work but instruction fetch fails.** This is the critical clue.

Possible explanations:
1. **PXN/UXN bits set incorrectly** — but we verified they're 0
2. **I-cache serving stale data** — but we invalidated
3. **SCTLR.I not set** — but we set it
4. **Something QEMU-specific** — most likely candidate

---

## External Reference: Theseus Approach

Theseus uses `KERNEL_OFFSET = 0x0000FFFF80000000` (note: NOT 0xFFFF...).

From `kernel_config/src/memory.rs:25-28`:
```rust
#[cfg(target_arch = "aarch64")]
const fn canonicalize(addr: usize) -> usize {
    addr & !0xFFFF_0000_0000_0000  // Clear top 16 bits for TTBR0
}
```

Theseus relies on **bootloader** (UEFI/Multiboot) to set up initial mappings. The kernel enters already running at high address.

---

## Next Team Checklist

- [ ] Read this document completely
- [ ] Read TEAM_024 and TEAM_025 team files
- [ ] Enable QEMU debug logging (`-d int,mmu`)
- [ ] Create minimal assembly-only test case
- [ ] Compare page table entries with Theseus
- [ ] Try different QEMU CPU models
- [ ] Check ARMv8 ARM for I-fetch requirements
- [ ] Document findings in team file

---

## Quick Reference: Address Calculations

```
High VA:    0x0000_FFFF_8008_9870
            ├─ L0 index: (VA >> 39) & 0x1FF = 511
            ├─ L1 index: (VA >> 30) & 0x1FF = 510
            ├─ L2 index: (VA >> 21) & 0x1FF = 0
            └─ Offset:   VA & 0x1FFFFF = 0x89870

Physical:   0x4008_9870 (when mapped to 0x40000000 physical base)
```

---

## Contact

For questions about previous attempts, refer to:
- `.teams/TEAM_024_review_impl_high_memory.md`
- `.teams/TEAM_025_implement_higher_half_ttbr0.md`
