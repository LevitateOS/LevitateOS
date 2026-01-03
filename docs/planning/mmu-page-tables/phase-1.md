# Phase 1 — Discovery: AArch64 Page Tables

**TEAM_018** | **Feature:** Implement AArch64 page table walking and modification

---

## Feature Summary

**Problem:** LevitateOS currently runs with no explicit MMU configuration. The kernel relies on QEMU's default identity mapping. This blocks:
- Memory isolation (userspace separation)
- Memory protection (NX, RO sections)
- Virtual memory features (demand paging, memory-mapped I/O)

**Goal:** Implement foundational page table support to enable virtual memory.

**Who Benefits:** Future phases (userspace, process isolation, advanced memory management).

---

## Success Criteria

1. Page table data structures defined in `levitate-hal`
2. Ability to allocate and configure L0-L3 page tables
3. Identity mapping of kernel works (no crash on MMU enable)
4. TTBR0_EL1 loaded with valid page table base
5. MMU enable sequence implemented

---

## Current State Analysis

### Memory Layout (from linker.ld)
| Region | Address | Size |
|--------|---------|------|
| Kernel base | 0x40080000 | — |
| Stack top | 0x48000000 | 64KB |
| Heap | linker end → 0x41F00000 | ~30MB |
| RAM (QEMU virt) | 0x40000000 | 512MB |

### Current MMU Status
- **Not explicitly enabled** — QEMU virt machine boots at EL1 with MMU likely disabled
- No page tables created
- No MAIR configuration
- No TTBR setup

---

## Codebase Reconnaissance

### Files to Create
- `levitate-hal/src/mmu.rs` — Page table structures, walker, modifier
- `levitate-hal/src/mmu/` — Possibly split into submodules (entry.rs, table.rs)

### Files to Modify
- `kernel/src/main.rs` — Call MMU init before enabling interrupts
- `levitate-hal/src/lib.rs` — Export mmu module

### Dependencies
- None external (self-contained implementation initially)
- Future: frame allocator integration

---

## AArch64 Paging Architecture Reference

### 4KB Granule, 4-Level Translation (48-bit VA)
| Level | VA Bits | Entry Maps | Entry Count |
|-------|---------|------------|-------------|
| L0 | [47:39] | 512GB | 512 |
| L1 | [38:30] | 1GB | 512 |
| L2 | [29:21] | 2MB | 512 |
| L3 | [20:12] | 4KB (page) | 512 |

### Key Registers
- **TTBR0_EL1** — Translation Table Base Register 0 (lower VA range)
- **TTBR1_EL1** — Translation Table Base Register 1 (higher VA range, kernel)
- **TCR_EL1** — Translation Control Register (granule size, VA size)
- **MAIR_EL1** — Memory Attribute Indirection Register
- **SCTLR_EL1** — System Control Register (M bit enables MMU)

---

## Constraints

1. **Identity mapping first** — Kernel must continue running after MMU enable
2. **No external crates** — Initially self-contained (no RMM dependency)
3. **Minimal complexity** — Start with 4KB pages only, no huge pages
4. **QEMU virt machine** — Target only; Pi 4 later
