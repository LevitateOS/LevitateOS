# MMU Integration Guide

**TEAM_018** | **Document for Future Teams**

This guide explains how to wire up the MMU module (`levitate-hal/src/mmu.rs`) into the kernel.

---

## Prerequisites

The `mmu.rs` module is complete and provides:
- `init()` — Configure MAIR_EL1, TCR_EL1
- `identity_map_range()` — Map kernel memory (VA == PA)
- `enable_mmu()` — Load TTBR0, enable SCTLR.M
- `tlb_flush_all()` — Flush TLB

---

## Integration Steps

### Step 1: Add to `kmain()` (BEFORE interrupts enabled)

In `kernel/src/main.rs`, add after heap init but BEFORE GIC/timer init:

```rust
use levitate_hal::mmu;

// In kmain():

// --- MMU Setup (TEAM_018) ---
println!("Initializing MMU...");

// 1. Configure MAIR and TCR registers
mmu::init();

// 2. Get root page table from static pool
let root = unsafe {
    static mut ROOT_PT: mmu::PageTable = mmu::PageTable::new();
    &mut ROOT_PT
};

// 3. Identity map kernel region (VA == PA)
// Map from KERNEL_PHYS_START to KERNEL_PHYS_END
mmu::identity_map_range(
    root,
    mmu::KERNEL_PHYS_START,
    mmu::KERNEL_PHYS_END,
    mmu::PageFlags::KERNEL_DATA,
).expect("Failed to identity map kernel");

// 4. Also map device memory regions (GIC, UART)
// UART PL011 at 0x0900_0000
mmu::identity_map_range(
    root,
    0x0900_0000,
    0x0900_1000,
    mmu::PageFlags::DEVICE,
).expect("Failed to map UART");

// GIC at 0x0800_0000 - 0x0802_0000
mmu::identity_map_range(
    root,
    0x0800_0000,
    0x0802_0000,
    mmu::PageFlags::DEVICE,
).expect("Failed to map GIC");

// 5. Flush TLB before enabling
mmu::tlb_flush_all();

// 6. Enable MMU
let root_phys = root as *const _ as usize;
unsafe { mmu::enable_mmu(root_phys); }

println!("MMU enabled with identity mapping.");
// --- End MMU Setup ---
```

---

## Critical Gotchas

> [!CAUTION]
> **GOTCHA 1: Device Memory Must Be Mapped**
> You MUST identity-map all MMIO regions (GIC, UART, VirtIO) BEFORE enabling MMU.
> If any device access happens to an unmapped address, the CPU will fault.

> [!CAUTION]
> **GOTCHA 2: Stack Must Be Mapped**
> The stack is at 0x48000000 (from boot.S). Ensure this is within your mapped range.
> `KERNEL_PHYS_END` is 0x48000000, so this should be covered.

> [!CAUTION]
> **GOTCHA 3: Code Must Be Mapped**
> The instruction being executed when MMU enables must be in the identity map.
> Since kernel starts at 0x40080000 and we map from there, this is covered.

> [!WARNING]
> **GOTCHA 4: Page Table Pool Size**
> The static pool has 8 tables. For a 128MB identity map with 4KB pages, you need:
> - 1 L0 table (covers 512GB each entry)
> - 1 L1 table (covers 1GB each entry)
> - ~1 L2 table (covers 2MB each entry)
> - ~64 L3 tables (covers 4KB each entry)
>
> **The current pool is too small for full identity mapping!**
> Either: increase pool size, or use 2MB block mappings (not yet implemented).

---

## Verification

### Debug Output
Add these prints before `enable_mmu()`:
```rust
println!("Root PT at: {:x}", root_phys);
println!("About to enable MMU...");
```

### QEMU Debug
Run with `-d int,mmu` to see MMU faults:
```bash
qemu-system-aarch64 -d int,mmu ...
```

### If Kernel Hangs
1. MMU enable failed — check identity mapping covers all accessed memory
2. Check stack address is mapped
3. Check device memory is mapped

---

## Future Improvements

- [ ] Implement 2MB block mapping to reduce page table count
- [ ] Dynamic frame allocator instead of static pool
- [ ] Higher-half kernel mapping
- [ ] Separate code/data/rodata with proper permissions

---

## References

- [phase-1.md](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/phase-1.md) — Discovery
- [phase-2.md](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/phase-2.md) — Design
- [mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs) — Implementation
