# Phase 1 — Understanding and Scoping

**TEAM_019** | **Bugfix:** MMU Page Table Pool Exhaustion

---

## Bug Summary

The static page table pool in `levitate-hal/src/mmu.rs` is sized at 8 entries, which is **insufficient** to identity-map the kernel memory range using 4KB pages.

**Severity:** HIGH — Blocks MMU enablement entirely  
**Impact:** Kernel cannot enable virtual memory; `identity_map_range()` fails

---

## Reproduction Status

**Reproducible:** Yes (deterministic at compile/boot time)

### Reproduction Steps

1. Build kernel with current `mmu.rs`
2. In `kmain()`, call `identity_map_range()` with:
   - Start: `0x4008_0000`
   - End: `0x4800_0000`
   - Flags: `PageFlags::KERNEL_DATA`
3. Observe: "Page table pool exhausted" error after consuming 8 tables

### Expected Behavior

- `identity_map_range()` completes successfully
- All 128MB of kernel memory is identity-mapped
- MMU can be enabled

### Actual Behavior

- `alloc_page_table()` returns `None` after allocating 8 tables
- `identity_map_range()` returns `Err("Page table pool exhausted")`
- MMU cannot be enabled

---

## Context

### Code Location

[mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs#L344-L346):
```rust
static mut PT_POOL: [PageTable; 8] = [const { PageTable::new() }; 8];
static mut PT_POOL_NEXT: usize = 0;
```

### Why 8 Is Too Small

For 4KB page mapping of 128MB (0x4008_0000 to 0x4800_0000):

| Level | Purpose | Tables Needed |
|-------|---------|---------------|
| L0 | Index VA[47:39] | 1 |
| L1 | Index VA[38:30] | 1 |
| L2 | Index VA[29:21] | 1 (covering ~64 entries) |
| L3 | Index VA[20:12] | **~64** (each covers 2MB) |
| **Total** | | **~67 tables** |

The pool of 8 tables is exhausted after allocating L0, L1, L2, and 5× L3 tables.

### MMIO Regions Also Need Tables

Additional mappings required beyond kernel memory:
- UART PL011: 0x0900_0000 — 0x0900_1000 (1 page, but new L0-L3 path)
- GIC: 0x0800_0000 — 0x0802_0000 (32 pages, new L0-L3 path)

These regions are at different L0/L1/L2 indices, potentially requiring additional intermediate tables.

---

## Constraints

1. **No heap allocator for page tables** — Cannot use heap; must work at early boot
2. **Static allocation required** — Page tables must be in `.bss` section
3. **Memory budget** — Each `PageTable` is 4KB; 100 tables = 400KB of `.bss`
4. **Minimal complexity** — Prefer simple fix over major refactor

---

## Open Questions

> [!IMPORTANT]
> **Q1:** Should we use 2MB block mappings instead of 4KB pages?
> **Context:** 2MB block mappings would reduce table count to ~4 (1 L0 + 1 L1 + 2 L2), but require changes to `map_page()` logic.

> [!IMPORTANT]
> **Q2:** Is 128MB the correct mapping size?
> **Context:** Current constants map from `KERNEL_PHYS_START` (0x40080000) to `KERNEL_PHYS_END` (0x48000000). Should the heap end be the actual boundary, or should we map all 512MB RAM?

---

## Step Plan

1. **Step 1:** Consolidate bug information ✓ (this document)
2. **Step 2:** Calculate exact table requirements for all regions
3. **Step 3:** Propose fix options in Phase 2

---

## References

- [integration-guide.md](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/integration-guide.md) — Original documentation of this GOTCHA
- [mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs) — Implementation
- [phase-2.md](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/phase-2.md) — Original MMU design
