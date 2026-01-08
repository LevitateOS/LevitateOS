# Phase 2 — Design: AArch64 Page Tables

**TEAM_018** | **Feature:** Page table walking and modification

---

## Reference Implementations Studied

| Kernel | Key Learnings |
|--------|---------------|
| **Redox** | Uses RMM (Redox Memory Manager) crate, TLB flushing abstraction, InactiveFlusher pattern |
| **Theseus** | TableLevel trait hierarchy (L4→L3→L2→L1), recursive page table mapping, `next_table_create()` for lazy alloc |

---

## Proposed Solution

### High-Level Architecture

```
┌───────────────────────────────────────────────────────────┐
│                    levitate-hal/src/mmu.rs                │
├───────────────────────────────────────────────────────────┤
│  PageTableEntry      - 64-bit entry with flags           │
│  PageTable           - Array of 512 entries              │
│  PageTableLevel      - L0, L1, L2, L3 enum               │
│  PageFlags           - Present, RW, NX, User, etc.       │
├───────────────────────────────────────────────────────────┤
│  init()              - Configure MAIR, TCR               │
│  map_page()          - Map VA → PA at a given level      │
│  identity_map()      - Map entire kernel range           │
│  enable_mmu()        - Set TTBR, enable SCTLR.M          │
│  tlb_flush()         - Invalidate TLB entries            │
└───────────────────────────────────────────────────────────┘
```

---

## Data Structures (Informed by Theseus)

### PageTableEntry (64-bit)
```rust
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self { Self(0) }
    pub fn is_valid(&self) -> bool { (self.0 & 0x1) != 0 }
    pub fn is_table(&self) -> bool { (self.0 & 0x2) != 0 }
    pub fn address(&self) -> u64 { self.0 & 0x0000_FFFF_FFFF_F000 }
    pub fn set(&mut self, addr: u64, flags: PageFlags) {
        self.0 = (addr & 0x0000_FFFF_FFFF_F000) | flags.bits();
    }
}
```

### PageTable (4KB, 512 entries)
```rust
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self { entries: [PageTableEntry::empty(); 512] }
    }
    
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageTableEntry::empty();
        }
    }
}
```

### PageFlags (AArch64 Stage 1 Descriptor)
```rust
bitflags! {
    pub struct PageFlags: u64 {
        const VALID      = 1 << 0;    // Entry is valid
        const TABLE      = 1 << 1;    // Table (vs Block) descriptor
        // MAIR index (AttrIndx[2:0] at bits [4:2])
        const ATTR_NORMAL= 0b000 << 2; // MAIR index 0: Normal memory
        const ATTR_DEVICE= 0b001 << 2; // MAIR index 1: Device memory
        const NS         = 1 << 5;    // Non-secure
        const AP_RW_EL1  = 0b00 << 6; // R/W at EL1, none at EL0
        const AP_RW_ALL  = 0b01 << 6; // R/W at all ELs
        const AP_RO_EL1  = 0b10 << 6; // RO at EL1, none at EL0
        const AP_RO_ALL  = 0b11 << 6; // RO at all ELs
        const SH_INNER   = 0b11 << 8; // Inner Shareable
        const AF         = 1 << 10;   // Access Flag (set by HW or SW)
        const NG         = 1 << 11;   // Not Global
        const PXN        = 1 << 53;   // Privileged Execute Never
        const UXN        = 1 << 54;   // User Execute Never
    }
}
```

---

## API Design (Informed by Theseus patterns)

### MAIR Configuration (from Theseus)
```rust
// MAIR_EL1 configuration:
// Attr0: Normal memory (WriteBack, Non-Transient, ReadWriteAlloc)
// Attr1: Device memory (nGnRE)
const MAIR_VALUE: u64 = 
    (0xFF << 0) |  // Attr0: Normal
    (0x04 << 8);   // Attr1: Device nGnRE

pub fn init() {
    unsafe {
        // Set MAIR_EL1
        asm!("msr mair_el1, {}", in(reg) MAIR_VALUE);
        
        // Set TCR_EL1: T0SZ=16 (48-bit VA), 4KB granule
        let tcr: u64 = (16 << 0)      // T0SZ = 16
                     | (0b00 << 14)   // TG0 = 4KB
                     | (0b10 << 32);  // IPS = 48-bit PA
        asm!("msr tcr_el1, {}", in(reg) tcr);
        
        asm!("isb");
    }
}
```

### TLB Flush (from Theseus)
```rust
pub fn tlb_flush_all() {
    unsafe {
        asm!("tlbi vmalle1");
        asm!("dsb sy");
        asm!("isb");
    }
}

pub fn tlb_flush_page(va: usize) {
    unsafe {
        asm!("tlbi vae1, {}", in(reg) va >> 12);
        asm!("dsb sy");
        asm!("isb");
    }
}
```

### Enable MMU (from Theseus)
```rust
pub unsafe fn enable_mmu(root_phys: usize) {
    // Load TTBR0_EL1 with page table root
    asm!("msr ttbr0_el1, {}", in(reg) root_phys);
    asm!("isb");
    
    // Enable MMU via SCTLR_EL1.M bit
    let mut sctlr: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    sctlr |= 1;  // M = 1
    asm!("msr sctlr_el1, {}", in(reg) sctlr);
    asm!("isb");
}
```

---

## Open Questions

> [!IMPORTANT]
> **Q1:** Frame allocator strategy?
> **Answer (from analysis):** Use a dedicated static region for page table frames initially, NOT the heap. Theseus uses `frame_allocator::allocate_frames()`.

> [!IMPORTANT]  
> **Q2:** Recursive mapping?
> **Answer:** Defer. Theseus uses recursive mapping (entry 510), but we can start simpler with direct physical access (identity mapped during setup).

---

## Implementation Order

1. Define `PageTableEntry`, `PageTable`, `PageFlags` types
2. Implement `init()` — configure MAIR, TCR (from Theseus pattern)
3. Implement frame allocation (static pool for PT frames)
4. Implement `map_page()` — create/walk page tables
5. Implement `identity_map_range()` — map kernel memory
6. Implement `tlb_flush_all()` and `tlb_flush_page()`
7. Implement `enable_mmu()` — set TTBR0, enable SCTLR.M
8. Wire into `kmain()` before enabling interrupts
