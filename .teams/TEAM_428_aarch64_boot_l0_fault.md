# TEAM_428: AArch64 Boot L0 Translation Fault Investigation

## Status: COMPLETE ✓

## Continuing From: TEAM_427

Team 427 identified the symptom:
- Data Abort (EC=0x25) with ESR 0x96000044
- FAR: 0xffff800040082000 (L0 translation fault, level 0)
- Occurs during `memset` in `memory::init()`

Team 427's hypothesis: Cache coherency issue where the L0[256] entry isn't visible to the MMU after the store.

## Root Cause Found

**Team 427's hypothesis was incorrect.** The cache coherency was not the issue.

### Actual Root Cause: Memory Region Overlap

The DTB parser (`crates/kernel/levitate/src/boot/dtb.rs`) marked ALL memory from the device tree as "Usable" without reserving the kernel physical region.

**Memory Layout:**
- RAM from DTB: 0x40000000 - 0xC0000000 (2GB)
- Kernel physical region: 0x40080000 - 0x41F00000
- Boot page tables: 0x40081000 - 0x40085000

**What happened:**
1. `memory::init()` allocates page metadata array at first usable RAM (0x40000000)
2. Array size: 524288 pages × 8 bytes = 4MB (spans 0x40000000 - 0x40400000)
3. Array overlaps with boot page tables at 0x40081000!
4. `core::ptr::write_bytes()` (memset) zeros the page array
5. This **destroys the active boot page tables**
6. Next memory access causes L0 translation fault

### Why instruction fetch worked initially
The kernel code had already been fetched into the instruction cache before the page tables were overwritten. Data accesses require fresh page table walks.

## Fix Applied

### 1. `crates/kernel/levitate/src/boot/dtb.rs`
Added `add_memory_region_with_kernel_split()` helper function that splits DTB memory regions around the kernel physical region:
- Memory before kernel (0x40000000 - 0x40080000) → Usable
- Kernel region (0x40080000 - 0x41F00000) → Reserved (MemoryKind::Kernel)
- Memory after kernel (0x41F00000 - 0xC0000000) → Usable

### 2. `crates/kernel/arch/aarch64/src/asm/boot.S` (defensive improvement)
Enhanced cache invalidation to flush entire 16KB page table region (256 cache lines) instead of just 4 individual entries. Not strictly necessary for this bug, but good practice.

## Verification

After fix:
```
[BOOT] Protocol: DeviceTree
[BOOT] Memory: 3 regions, 2017 MB usable  ← Was "1 regions, 2048 MB"
[BOOT] Stage 2: Memory & MMU (PEI)
[MEM] Physical: 0x40000000 - 0xc0000000 (524288 pages, 2048 MB)
[MEM] Frame allocator initialized  ← SUCCESS!
[CPU] AArch64 PCR initialized
... continues to maintenance shell ...
```

## Files Modified

1. `crates/kernel/levitate/src/boot/dtb.rs` - Reserve kernel region in memory map
2. `crates/kernel/arch/aarch64/src/asm/boot.S` - Improved cache flush (defensive)

## Lessons Learned

1. **Memory overlap bugs can masquerade as MMU/cache issues** - The L0 translation fault looked like a cache coherency problem, but was actually a memory allocation overlap.

2. **Follow the data flow** - Understanding that memset was writing to the page array, and the page array overlapped with page tables, was key.

3. **Check memory reservations** - When porting to a new boot protocol (DTB vs Limine), ensure kernel regions are properly reserved.

## Handoff Checklist

- [x] Project builds cleanly (both aarch64 and x86_64)
- [x] AArch64 kernel boots to maintenance shell
- [x] Root cause identified and fixed
- [x] Team file updated
- [ ] No behavioral regression tests exist for aarch64 boot yet
