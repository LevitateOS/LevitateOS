# Phase 3 — Fix Design and Validation Plan

**TEAM_131 → TEAM_132** | Reduce Unsafe Code via Safe Abstractions

## Root Cause Summary

The codebase has ~148 unsafe blocks that follow 4 repeatable patterns. These can be reduced by ~80% through safe abstraction libraries.

## Fix Strategy

Create 4 new abstraction modules/crates:

### 1. Volatile I/O — Use `safe-mmio` (Already Available)

**Reduces:** ~12 unsafe blocks → 0 in consumer code

**Crate:** `safe-mmio` (already in Cargo.lock)

**Why this crate:**
- Avoids creating references to MMIO space (prevents UB)
- Works around aarch64 virtualization bug (uses inline asm)
- Distinguishes pure reads vs side-effect reads
- Google-maintained, actively developed

**Migration Example:**
```rust
// Before
unsafe { core::ptr::write_volatile(&mut self.avail_idx, new_idx) }

// After (with safe-mmio)
use safe_mmio::{UniqueMmioPointer, fields::ReadWrite};
self.avail_idx.write(new_idx)  // No unsafe!
```

---

### 2. System Registers + Barriers — Use `aarch64-cpu`

**Reduces:** ~38 unsafe blocks → 0 in consumer code

**Crate:** `aarch64-cpu` v11.2 (rust-embedded official Cortex-A crate)

**Why this crate:**
- Official rust-embedded team crate
- Defines ~200+ ARM system registers with type-safe bitfield accessors
- Includes all barriers (dsb, dmb, isb) and intrinsics (wfi, wfe, eret)
- Actively maintained, production-quality

**Migration Example:**
```rust
// Before
let mut sctlr: u64;
unsafe { core::arch::asm!("mrs {}, sctlr_el1", out(reg) sctlr) };
unsafe { core::arch::asm!("dsb sy") };

// After (with aarch64-cpu)
use aarch64_cpu::{asm::barrier, registers::*};
let sctlr = SCTLR_EL1.get();  // No unsafe!
barrier::dsb(barrier::SY);    // No unsafe!
```

---

### 3. Intrusive Linked Lists — Use `intrusive-collections`

**Reduces:** ~8 unsafe blocks → 0 in consumer code

**Crate:** `intrusive-collections` v0.10 (by Amanieu, author of hashbrown)

**Why this crate:**
- Battle-tested, used in production
- `no_std` compatible
- Provides LinkedList, SinglyLinkedList, RBTree
- Safe cursor-based API

**Migration Example:**
```rust
// Before
unsafe { page_ptr.as_mut() }.next = Some(other);

// After (with intrusive-collections)
use intrusive_collections::{LinkedList, LinkedListLink, intrusive_adapter};

struct Page {
    link: LinkedListLink,
    order: u8,
}

intrusive_adapter!(PageAdapter = Box<Page>: Page { link: LinkedListLink });
let mut free_list: LinkedList<PageAdapter> = LinkedList::new(PageAdapter::new());
free_list.push_back(page);  // No unsafe!
```

---

## Reversal Strategy

Each abstraction is additive:
1. New module is added alongside existing code
2. Call sites migrate incrementally
3. Old patterns can coexist with new wrappers
4. If issues arise, simply don't migrate more call sites

**Revert signals:**
- Performance regression in benchmarks
- Compilation issues on new architectures
- Unsound abstraction discovered

---

## Test Strategy

1. **Unit tests** for each wrapper type
2. **Existing behavior tests** must continue passing
3. **Compile-time verification** that unsafe count decreases
4. **No new unsafe blocks** added except in wrapper implementations

---

## Impact Analysis

| Change | Impact |
|--------|--------|
| API changes | None — internal refactoring only |
| Performance | Zero-cost abstractions (inline, no runtime overhead) |
| Dependencies | May add `volatile` or `intrusive-collections` crates |
| Build time | Minimal increase from new modules |

## Implementation Order

| Priority | Abstraction | Est. Reduction | Complexity |
|----------|-------------|----------------|------------|
| 1 | `barrier` module | 8 unsafe → 1 | Low |
| 2 | `volatile` wrapper | 12 unsafe → 2 | Low |
| 3 | `sysreg` macros | 30 unsafe → 2 | Medium |
| 4 | `intrusive_list` | 8 unsafe → 2 | High |

**Total estimated reduction:** ~58 unsafe blocks → ~7 internal blocks (**88% reduction**)
