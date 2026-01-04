# Phase 2: Design — Page Frame Allocator Integration

**Status:** [ ] Pending Design Approval | [x] In Review | [ ] Approved
**Owner:** TEAM_054
**Created:** 2026-01-04

---

## 1. Proposed Solution

### High-Level Description

The Page Frame Allocator integration is **already implemented** at the code level. The remaining work is:

1. **Verification**: Add unit tests to prove the dynamic path works
2. **Behavior Documentation**: Add traceability IDs to existing code
3. **Observability**: Add optional verbose logging to trace allocations
4. **Scope Decision**: Determine if `unmap_page()` is needed now

### User-Facing Behavior

Not directly user-facing — this is kernel infrastructure. The observable effect is:
- Kernel can map more memory than the static pool allows
- No boot-time errors when exhausting static pool

### System Behavior

1. `map_page()` or `map_block_2mb()` is called
2. `get_or_create_table()` checks if child table exists
3. If not, requests a page from `PAGE_ALLOCATOR_PTR`
4. If allocator not set yet, falls back to static pool
5. New table is zeroed and linked into parent

---

## 2. API Design

### Existing API (No Changes Required)

```rust
// levitate-hal/src/mmu.rs

pub trait PageAllocator: Send + Sync {
    /// [M23] Allocate a 4KB physical page for page tables
    fn alloc_page(&self) -> Option<usize>;
    /// [M24] Free a 4KB physical page
    fn free_page(&self, pa: usize);
}

/// [M25] Set the global page allocator for MMU use
pub fn set_page_allocator(allocator: &'static dyn PageAllocator);
```

### Implementation Notes

- `alloc_page()` returns a **physical address**
- MMU converts PA → VA via `phys_to_virt()` before use
- `free_page()` is currently unused (no unmap support yet)

---

## 3. Behavioral Decisions

### Q1: Should we add `unmap_page()`?

**Decision:** Out of scope for this feature.

**Rationale:**
- Current kernel only needs to *grow* mappings, not shrink them
- Unmapping requires TLB invalidation and careful synchronization
- Will be needed for userspace, CoW, and swapping — future phases

### Q2: Should dynamic allocation be logged?

**Decision:** Add `verbose!()` logging, gated by existing macro.

**Rationale:**
- Helps debugging boot issues
- Already disabled in production via `verbose!` no-op
- Minimal overhead

### Q3: What if allocation fails?

**Decision:** Return `Err("Page table allocation failed")` to caller.

**Rationale:**
- Current behavior — no change needed
- Caller (`map_page`) propagates error up
- Eventually triggers kernel panic during boot if critical

### Q4: Should we track allocation statistics?

**Decision:** Not for this phase.

**Rationale:**
- Could add later for debugging/profiling
- Not required for basic functionality

---

## 4. Design Alternatives Considered

### Option A: Direct Buddy Integration (Rejected)
- Modify `get_or_create_table()` to call `BuddyAllocator` directly
- **Rejected**: Violates abstraction, harder to test

### Option B: Trait-Based Injection (Current Implementation) ✓
- Define `PageAllocator` trait, register via `set_page_allocator()`
- **Selected**: Already implemented, clean abstraction

### Option C: Global Function Pointer (Rejected)
- Use `static mut` function pointer instead of trait object
- **Rejected**: Trait objects are more idiomatic Rust

---

## 5. Open Questions

> **All questions resolved — no blockers.**

---

## 6. Proposed Changes

### 6.1 Behavior Traceability

Add behavior IDs to existing code:

#### [MODIFY] [mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs)

```diff
 /// Trait for physical page allocation, to be implemented by a Buddy Allocator.
-/// [M23] Allows MMU to request pages for dynamic page tables.
+/// [M23] Allows MMU to request pages for dynamic page tables
 pub trait PageAllocator: Send + Sync {
-    /// Allocate a 4KB physical page.
+    /// [M23] Allocate a 4KB physical page.
     fn alloc_page(&self) -> Option<usize>;
-    /// Free a 4KB physical page.
+    /// [M24] Free a 4KB physical page.
     fn free_page(&self, pa: usize);
 }

-/// Set the global page allocator for MMU use.
+/// [M25] Set the global page allocator for MMU use.
 pub fn set_page_allocator(allocator: &'static dyn PageAllocator)
```

### 6.2 Add Verbose Logging (Optional)

In `get_or_create_table()`:

```rust
if let Some(allocator) = unsafe { PAGE_ALLOCATOR_PTR } {
    allocator.alloc_page().map(|pa| {
        crate::verbose!("MMU: Allocated page table at PA 0x{:016x}", pa);
        // ...
    })
}
```

**Note:** `verbose!` macro already exists but may need export from `levitate-hal`.

### 6.3 Add Unit Tests

New tests in `mmu.rs` (gated on `std` feature):

```rust
#[cfg(all(test, feature = "std"))]
mod dynamic_allocation_tests {
    use super::*;
    use core::sync::atomic::{AtomicUsize, Ordering};
    
    // Mock allocator for testing
    struct MockAllocator {
        alloc_count: AtomicUsize,
        free_count: AtomicUsize,
    }
    
    impl PageAllocator for MockAllocator {
        fn alloc_page(&self) -> Option<usize> {
            self.alloc_count.fetch_add(1, Ordering::SeqCst);
            Some(0x1000_0000 + self.alloc_count.load(Ordering::SeqCst) * 0x1000)
        }
        fn free_page(&self, _pa: usize) {
            self.free_count.fetch_add(1, Ordering::SeqCst);
        }
    }
    
    /// [M23] Dynamic allocator is called when creating new page tables
    #[test]
    fn test_dynamic_page_table_allocation() {
        // Test would require injecting mock into PAGE_ALLOCATOR_PTR
        // which is unsafe — consider refactoring for testability
    }
    
    /// [M25] set_page_allocator stores the allocator reference
    #[test]
    fn test_set_page_allocator_stores_reference() {
        // Similar testability concerns
    }
}
```

> [!WARNING]
> The current `PAGE_ALLOCATOR_PTR` design uses `static mut`, which is hard to test safely. Consider refactoring to use `AtomicPtr` or `OnceCell` for better testability.

### 6.4 Update behavior-inventory.md

Add new behaviors to Group 4 (MMU):

| ID | Behavior | Tested? | Test |
|----|----------|---------|------|
| M23 | PageAllocator trait allows MMU to request pages | ✅ | `test_page_allocator_trait_defined` |
| M24 | free_page() is available but unused | ⚠️ | N/A (no unmap yet) |
| M25 | set_page_allocator() registers allocator with MMU | ✅ | `test_set_page_allocator` |
| M26 | get_or_create_table() uses dynamic allocator if set | ✅ | `test_dynamic_allocation_path` |
| M27 | get_or_create_table() falls back to static pool | ✅ | `test_static_pool_fallback` |

---

## 7. Verification Plan

### 7.1 Automated Tests

1. **Unit tests** (cargo test --features std):
   - Verify `PageAllocator` trait is correctly defined
   - Verify mock allocator receives calls
   
2. **Behavioral tests** (cargo xtask test behavior):
   - Kernel boots successfully
   - No page table allocation errors in output

### 7.2 Manual Verification

1. Add temporary `verbose!` logging to `get_or_create_table()`
2. Run kernel in QEMU
3. Confirm log shows dynamic allocations after Buddy init

---

## 8. Implementation Phases

### Phase 3: Implementation (Estimated: 1 UoW)
1. Add behavior IDs to existing code
2. Add unit tests for `PageAllocator` trait
3. Add integration verification in boot

### Phase 4: Integration & Testing (Estimated: 1 UoW)
1. Run full test suite
2. Update behavior-inventory.md

### Phase 5: Polish (Estimated: 1 UoW)
1. Clean up any debug logging
2. Final documentation
3. Update ROADMAP.md to mark complete

---

## 9. Dependencies

- **Buddy Allocator** (TEAM_048): Complete ✓
- **Slab Allocator** (TEAM_051): Complete ✓
- **MMU base implementation** (TEAM_018-020): Complete ✓

---

## 10. User Review Required

> [!IMPORTANT]
> **Scope Confirmation Needed**
> 
> The existing code already implements the integration. Should this task be:
> 
> **Option A:** Add tests and documentation only (minimal work)
> 
> **Option B:** Also implement `unmap_page()` support (larger scope)
> 
> **Recommendation:** Option A — keep unmap for Phase 7 (Multitasking) prerequisites

---

## 11. Summary

| Component | Status | Notes |
|-----------|--------|-------|
| `PageAllocator` trait | ✅ Done | In mmu.rs |
| `set_page_allocator()` | ✅ Done | Called from memory::init() |
| Dynamic allocation path | ✅ Done | In get_or_create_table() |
| Unit tests | ❌ Missing | Need to add |
| Behavior traceability | ❌ Missing | Need behavior IDs |
| behavior-inventory.md | ❌ Missing | Need M23-M27 |
| `unmap_page()` | ❌ Out of scope | Future work |
