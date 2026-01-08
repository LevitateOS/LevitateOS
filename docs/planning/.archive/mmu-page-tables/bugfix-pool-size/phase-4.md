# Phase 4 — Implementation Plan: 2MB Block Mappings

**TEAM_019** | **Bugfix:** MMU Page Table Pool Exhaustion

---

## Revised Fix Strategy

Per user direction: implement the "hard and correct" solution with **2MB block mappings** and **comprehensive tests**.

---

## AArch64 Block Descriptor Format

### Key Insight: Block vs Table Descriptors

From ARM documentation and [Löwenware reference](https://lowenware.com/blog/aarch64-mmu-programming/):

| Descriptor Type | bits[1:0] | Location | Coverage |
|-----------------|-----------|----------|----------|
| Invalid | 0b00 | Any | — |
| Block | 0b01 | L1, L2 only | 1GB (L1), 2MB (L2) |
| Table | 0b11 | L0, L1, L2 | Points to next level |
| Page | 0b11 | L3 only | 4KB |

**Block descriptor at L2:**
- bits[1:0] = 0b01 (VALID, but NOT TABLE)
- bits[47:21] = Physical block address (2MB aligned)
- bits[4:2] = MAIR index
- bits[7:6] = AP (access permissions)
- bits[9:8] = SH (shareability)
- bit[10] = AF (access flag)
- bit[53] = PXN
- bit[54] = UXN

---

## Implementation Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    mmu.rs Changes                                   │
├─────────────────────────────────────────────────────────────────────┤
│  NEW: PageFlags::BLOCK        = 0b01 (Valid block descriptor)       │
│  NEW: BLOCK_2MB_SIZE          = 2 * 1024 * 1024                     │
│  NEW: BLOCK_2MB_MASK          = BLOCK_2MB_SIZE - 1                  │
├─────────────────────────────────────────────────────────────────────┤
│  NEW: map_block_2mb()         = Map single 2MB block at L2          │
│  NEW: can_use_block_mapping() = Check alignment and size            │
│  MOD: identity_map_range()    = Use blocks when possible, else 4KB  │
├─────────────────────────────────────────────────────────────────────┤
│  NEW: #[cfg(test)] mod tests  = Comprehensive unit tests            │
│       - test_page_flags_block_vs_table()                            │
│       - test_va_index_extraction()                                  │
│       - test_block_address_alignment()                              │
│       - test_identity_map_block_count()                             │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Proposed Changes

### [MODIFY] [mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs)

#### Change 1: Add block mapping constants

```rust
/// 2MB block size (for L2 block mappings)
pub const BLOCK_2MB_SIZE: usize = 2 * 1024 * 1024;
/// 2MB block alignment mask
pub const BLOCK_2MB_MASK: usize = BLOCK_2MB_SIZE - 1;
/// 1GB block size (for L1 block mappings)
pub const BLOCK_1GB_SIZE: usize = 1024 * 1024 * 1024;
```

#### Change 2: Add BLOCK flag to PageFlags

```rust
bitflags! {
    pub struct PageFlags: u64 {
        /// Entry is valid
        const VALID       = 1 << 0;
        /// Table descriptor (vs Block) - must be set for table entries
        const TABLE       = 1 << 1;
        // ... rest unchanged ...
    }
}

impl PageFlags {
    /// Block descriptor for 2MB mapping (VALID but NOT TABLE)
    pub const BLOCK_2MB: PageFlags = PageFlags::VALID;  // bits[1:0] = 0b01
    
    /// Standard flags for kernel data as 2MB block
    pub const KERNEL_DATA_BLOCK: PageFlags = PageFlags::VALID
        .union(PageFlags::AF)
        .union(PageFlags::SH_INNER)
        .union(PageFlags::AP_RW_EL1)
        .union(PageFlags::PXN)
        .union(PageFlags::UXN);
        // NOTE: No TABLE bit — this is a block descriptor

    /// Standard flags for device memory as 2MB block
    pub const DEVICE_BLOCK: PageFlags = PageFlags::VALID
        .union(PageFlags::AF)
        .union(PageFlags::ATTR_DEVICE)
        .union(PageFlags::AP_RW_EL1)
        .union(PageFlags::PXN)
        .union(PageFlags::UXN);
        // NOTE: No TABLE bit — this is a block descriptor
}
```

#### Change 3: Add `map_block_2mb()` function

```rust
/// Map a single 2MB block at L2 level.
///
/// # Arguments
/// - `root`: L0 page table
/// - `va`: Virtual address (must be 2MB aligned)
/// - `pa`: Physical address (must be 2MB aligned)
/// - `flags`: Page flags (should use KERNEL_DATA_BLOCK or DEVICE_BLOCK)
///
/// # Returns
/// Ok(()) on success, Err if allocation fails or misaligned
pub fn map_block_2mb(
    root: &mut PageTable,
    va: usize,
    pa: usize,
    flags: PageFlags,
) -> Result<(), &'static str> {
    // Verify 2MB alignment
    if (va & BLOCK_2MB_MASK) != 0 {
        return Err("VA not 2MB aligned for block mapping");
    }
    if (pa & BLOCK_2MB_MASK) != 0 {
        return Err("PA not 2MB aligned for block mapping");
    }

    // Get indices
    let l0_idx = va_l0_index(va);
    let l1_idx = va_l1_index(va);
    let l2_idx = va_l2_index(va);

    // Walk L0 -> L1
    let l1_table = get_or_create_table(root, l0_idx)?;

    // Walk L1 -> L2
    let l2_table = get_or_create_table(l1_table, l1_idx)?;

    // Set L2 entry as BLOCK (not TABLE)
    let entry = l2_table.entry_mut(l2_idx);
    // Block descriptor: bits[1:0] = 0b01 (VALID, not TABLE)
    entry.set(pa, flags);

    Ok(())
}
```

#### Change 4: Add smart `identity_map_range_optimized()`

```rust
/// Identity map a range using 2MB blocks where possible, otherwise 4KB pages.
///
/// This is more efficient than pure 4KB mapping:
/// - 128MB with 4KB pages = ~67 tables needed
/// - 128MB with 2MB blocks = ~3 tables needed
pub fn identity_map_range_optimized(
    root: &mut PageTable,
    start: usize,
    end: usize,
    flags: PageFlags,
) -> Result<(), &'static str> {
    let mut addr = start & !0xFFF; // Page align start
    let end_aligned = (end + 0xFFF) & !0xFFF; // Page align end

    while addr < end_aligned {
        let remaining = end_aligned - addr;

        // Check if we can use 2MB block:
        // 1. Address is 2MB aligned
        // 2. At least 2MB remaining
        if (addr & BLOCK_2MB_MASK) == 0 && remaining >= BLOCK_2MB_SIZE {
            // Use block mapping
            let block_flags = if flags.contains(PageFlags::ATTR_DEVICE) {
                PageFlags::DEVICE_BLOCK
            } else {
                PageFlags::KERNEL_DATA_BLOCK
            };
            map_block_2mb(root, addr, addr, block_flags)?;
            addr += BLOCK_2MB_SIZE;
        } else {
            // Use 4KB page mapping
            map_page(root, addr, addr, flags)?;
            addr += PAGE_SIZE;
        }
    }

    Ok(())
}
```

#### Change 5: Reduce pool size (now only ~4 tables needed)

```rust
/// Static pool of page tables for early boot.
/// With 2MB block mappings:
/// - 1 L0 table
/// - 1-2 L1 tables (kernel region)
/// - 1-2 L2 tables (kernel + MMIO)
/// - Few L3 tables for non-aligned regions at boundaries
/// Total: ~8-10 tables is now sufficient
static mut PT_POOL: [PageTable; 16] = [const { PageTable::new() }; 16];
```

---

## Test Strategy

### Unit Tests (in `mmu.rs`)

We'll add a `#[cfg(test)]` module following the pattern from `timer.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // === Flag Construction Tests ===
    
    #[test]
    fn test_page_flags_block_vs_table() {
        // Block descriptor: bits[1:0] = 0b01
        let block = PageFlags::VALID;
        assert_eq!(block.bits() & 0b11, 0b01);
        
        // Table descriptor: bits[1:0] = 0b11
        let table = PageFlags::VALID | PageFlags::TABLE;
        assert_eq!(table.bits() & 0b11, 0b11);
    }

    #[test]
    fn test_block_flags_no_table_bit() {
        let block = PageFlags::KERNEL_DATA_BLOCK;
        assert!(!block.contains(PageFlags::TABLE));
        assert!(block.contains(PageFlags::VALID));
    }

    // === Address Index Extraction Tests ===

    #[test]
    fn test_va_l0_index() {
        assert_eq!(va_l0_index(0x0000_0000_0000_0000), 0);
        assert_eq!(va_l0_index(0x0000_0080_0000_0000), 1);  // 512GB boundary
        assert_eq!(va_l0_index(0x0000_FF80_0000_0000), 511);
    }

    #[test]
    fn test_va_l1_index() {
        assert_eq!(va_l1_index(0x0000_0000_0000_0000), 0);
        assert_eq!(va_l1_index(0x0000_0000_4000_0000), 1);  // 1GB boundary
        assert_eq!(va_l1_index(0x0000_0000_8000_0000), 2);
    }

    #[test]
    fn test_va_l2_index() {
        assert_eq!(va_l2_index(0x0000_0000_0000_0000), 0);
        assert_eq!(va_l2_index(0x0000_0000_0020_0000), 1);  // 2MB boundary
        assert_eq!(va_l2_index(0x0000_0000_0040_0000), 2);
    }

    #[test]
    fn test_va_l3_index() {
        assert_eq!(va_l3_index(0x0000_0000_0000_0000), 0);
        assert_eq!(va_l3_index(0x0000_0000_0000_1000), 1);  // 4KB boundary
        assert_eq!(va_l3_index(0x0000_0000_0000_2000), 2);
    }

    #[test]
    fn test_kernel_address_indices() {
        // Kernel start: 0x4008_0000
        let va = 0x4008_0000usize;
        assert_eq!(va_l0_index(va), 0);   // Within first 512GB
        assert_eq!(va_l1_index(va), 1);   // Second 1GB region
        assert_eq!(va_l2_index(va), 0);   // First 2MB within that 1GB
    }

    // === Alignment Tests ===

    #[test]
    fn test_block_alignment() {
        assert_eq!(0x4000_0000 & BLOCK_2MB_MASK, 0); // 1GB is 2MB aligned
        assert_eq!(0x4020_0000 & BLOCK_2MB_MASK, 0); // 2MB aligned
        assert_ne!(0x4010_0000 & BLOCK_2MB_MASK, 0); // 1MB is NOT 2MB aligned
        assert_ne!(0x4008_0000 & BLOCK_2MB_MASK, 0); // 512KB is NOT 2MB aligned
    }

    // === Page Table Entry Tests ===

    #[test]
    fn test_page_table_entry_empty() {
        let entry = PageTableEntry::empty();
        assert!(!entry.is_valid());
        assert!(!entry.is_table());
        assert_eq!(entry.address(), 0);
    }

    #[test]
    fn test_page_table_entry_set_block() {
        let mut entry = PageTableEntry::empty();
        entry.set(0x4000_0000, PageFlags::KERNEL_DATA_BLOCK);
        assert!(entry.is_valid());
        assert!(!entry.is_table()); // Block, not table
        assert_eq!(entry.address(), 0x4000_0000);
    }

    #[test]
    fn test_page_table_entry_set_table() {
        let mut entry = PageTableEntry::empty();
        entry.set(0x4000_0000, PageFlags::VALID | PageFlags::TABLE);
        assert!(entry.is_valid());
        assert!(entry.is_table());
    }

    // === Mapping Logic Tests ===

    #[test]
    fn test_block_2mb_alignment_check() {
        // This tests the alignment validation in map_block_2mb
        // without actually calling MMU hardware
        
        // Valid alignments
        assert!((0x0000_0000 & BLOCK_2MB_MASK) == 0);
        assert!((0x0020_0000 & BLOCK_2MB_MASK) == 0);
        assert!((0x4000_0000 & BLOCK_2MB_MASK) == 0);
        
        // Invalid alignments
        assert!((0x0010_0000 & BLOCK_2MB_MASK) != 0); // 1MB
        assert!((0x0008_0000 & BLOCK_2MB_MASK) != 0); // 512KB
        assert!((0x4008_0000 & BLOCK_2MB_MASK) != 0); // Kernel start
    }

    #[test]
    fn test_table_count_for_block_mapping() {
        // Calculate tables needed for 128MB kernel with block mapping
        // Start: 0x4008_0000 (not 2MB aligned)
        // End:   0x4800_0000

        // First, round up to 2MB alignment: 0x4020_0000
        // Then we can use blocks from 0x4020_0000 to 0x4800_0000
        // That's (0x4800_0000 - 0x4020_0000) / 2MB = 126MB / 2MB = 63 blocks
        
        // For the leading edge (0x4008_0000 to 0x4020_0000 = 0x18_0000 = 1.5MB)
        // We need 4KB pages: 1.5MB / 4KB = 384 pages
        // But they all fall within same L0/L1/L2 path, need 1 L3 table
        
        // Total tables: 1 L0 + 1 L1 + 1 L2 + 1 L3 = 4 tables (approximately)
        // Much better than 67!
        
        let start = 0x4008_0000usize;
        let end = 0x4800_0000usize;
        
        let first_block_aligned = (start + BLOCK_2MB_SIZE - 1) & !BLOCK_2MB_MASK;
        let last_block_aligned = end & !BLOCK_2MB_MASK;
        
        assert_eq!(first_block_aligned, 0x4020_0000);
        assert_eq!(last_block_aligned, 0x4800_0000);
        
        let num_blocks = (last_block_aligned - first_block_aligned) / BLOCK_2MB_SIZE;
        assert_eq!(num_blocks, 63); // 63 blocks of 2MB
    }
}
```

### Running Tests

```bash
# Run HAL tests (these run on host, not target)
cargo test -p levitate-hal

# Or run all workspace tests
cargo test --workspace
```

---

## Integration Test (QEMU Verification)

After implementation, verify in QEMU:

```bash
./scripts/run.sh
```

**Expected output:**
```
Initializing MMU...
Using 2MB block mappings for kernel identity map
Mapped 63 blocks (126MB) + 384 pages (1.5MB leading edge)
Root PT at: 0x40XXXXXX
About to enable MMU...
MMU enabled with identity mapping.
```

---

## Implementation Checklist

### Phase 4-Step-1: Add Constants and Flags
- [ ] Add `BLOCK_2MB_SIZE`, `BLOCK_2MB_MASK` constants
- [ ] Add `PageFlags::KERNEL_DATA_BLOCK` and `PageFlags::DEVICE_BLOCK`
- [ ] Verify block flags have bits[1:0] = 0b01

### Phase 4-Step-2: Add Block Mapping Function
- [ ] Implement `map_block_2mb()` with alignment checks
- [ ] Add inline documentation

### Phase 4-Step-3: Add Optimized Range Mapping
- [ ] Implement `identity_map_range_optimized()`
- [ ] Handle leading/trailing unaligned regions with 4KB pages
- [ ] Use 2MB blocks for aligned middle region

### Phase 4-Step-4: Add Unit Tests
- [ ] Test flag construction
- [ ] Test VA index extraction
- [ ] Test alignment validation
- [ ] Test entry set/get operations
- [ ] Test table count calculation

### Phase 4-Step-5: Update Pool Size
- [ ] Reduce pool to 16 tables (with safety margin)
- [ ] Update comment explaining calculation

### Phase 4-Step-6: Build and Test
- [ ] `cargo build --release` passes
- [ ] `cargo test -p levitate-hal` passes
- [ ] QEMU verification

### Phase 4-Step-7: Update Documentation
- [ ] Update `integration-guide.md`
- [ ] Update GOTCHA 4 to document 2MB block usage

---

## Reversal Strategy

If 2MB block mapping causes issues:

1. Revert to 4KB-only mapping
2. Fall back to Option A (increase pool to 80)
3. Git revert the changes

```bash
git revert HEAD
```

---

## References

- [ARM DDI 0487: ARMv8 Reference Manual, Section D5.2](https://developer.arm.com/documentation/ddi0487/)
- [Löwenware: AArch64 MMU Programming](https://lowenware.com/blog/aarch64-mmu-programming/)
- [Phase 1](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-1.md)
- [Phase 2](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-2.md)
- [Phase 3](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-3.md)
