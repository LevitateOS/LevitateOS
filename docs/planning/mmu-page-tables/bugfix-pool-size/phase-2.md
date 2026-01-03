# Phase 2 — Root Cause Analysis

**TEAM_019** | **Bugfix:** MMU Page Table Pool Exhaustion

---

## Root Cause Summary

The bug is a **simple miscalculation of pool size** during initial implementation.

**Where:** [mmu.rs:344](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs#L344)  
**What:** `PT_POOL: [PageTable; 8]` — pool sized for "at least 4 tables" comment, not for actual 4KB mapping requirements

```rust
/// Static pool of page tables for early boot.
/// We need at least 4 tables for L0, L1, L2, L3 to identity map kernel.
static mut PT_POOL: [PageTable; 8] = [const { PageTable::new() }; 8];
```

The comment correctly identifies the **minimum** for a single page, but 128MB requires **many L3 tables**.

---

## Detailed Analysis

### Table Allocation Trace

For identity mapping kernel memory (0x40080000 to 0x48000000):

```
Address Range: 0x4008_0000 → 0x4800_0000
Size:          127.5 MB (~32,640 pages at 4KB each)
```

#### L0 Index Calculation
```
L0[index] = VA >> 39 = 0x4008_0000 >> 39 = 0
L0[index] = VA >> 39 = 0x4800_0000 >> 39 = 0
→ All addresses use L0[0]
→ 1 L1 table needed
```

#### L1 Index Calculation
```
L1[index] = (VA >> 30) & 0x1FF
L1[0x4008_0000] = 1
L1[0x4800_0000] = 1
→ All addresses use L1[1]
→ 1 L2 table needed
```

#### L2 Index Calculation
```
L2[index] = (VA >> 21) & 0x1FF
L2[0x4008_0000] = 0 (within 0x4000_0000 - 0x401F_FFFF range)
L2[0x4800_0000] = 64
→ 128MB spans L2 entries [0..64]
→ 64 L3 tables needed (one per 2MB)
```

#### Total Table Requirement

| Level | Tables | Size (each 4KB) |
|-------|--------|-----------------|
| L0 | 1 | 4 KB |
| L1 | 1 | 4 KB |
| L2 | 1 | 4 KB |
| L3 | 64 | 256 KB |
| **Total** | **67** | **268 KB** |

### MMIO Tables (Additional)

UART (0x0900_0000) and GIC (0x0800_0000) are at different L0/L1 indices:
```
UART: L0[0], L1[0], L2[4], L3[x] → 3-4 additional tables
GIC:  L0[0], L1[0], L2[4], L3[x] → shares L0/L1/L2 with UART
```

**Revised total:** ~70 tables for full mapping

---

## Why 8 Was Chosen

From the original design comment:
> "We need at least 4 tables for L0, L1, L2, L3 to identity map kernel."

This is correct for mapping **a single page**, but the implementation then calls `identity_map_range()` for **many pages**. The designer chose 8 tables as a "safe buffer" without calculating actual requirements.

---

## Conclusion

**Root Cause Type:** Design oversight / miscalculation

**Verification:** The bug is deterministic and can be validated by:
1. Adding debug prints in `alloc_page_table()` showing allocation count
2. Observing it fail after 8 allocations

No additional investigation needed. Proceed to Phase 3 for fix design.

---

## Next Step

[Phase 3: Fix Design](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-3.md)
