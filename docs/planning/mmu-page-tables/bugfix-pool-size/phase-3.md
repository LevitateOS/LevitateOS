# Phase 3 — Fix Design and Validation Plan

**TEAM_019** | **Bugfix:** MMU Page Table Pool Exhaustion

---

## Root Cause Summary

`PT_POOL` in `mmu.rs` is sized at 8 tables, but 128MB identity mapping with 4KB pages requires ~67 tables.

---

## Fix Options

### Option A: Increase Pool Size (Recommended)

**Description:** Increase `PT_POOL` from 8 to 80 tables

**Pros:**
- Minimal code change (single constant)
- No logic changes needed
- Easy to verify

**Cons:**
- 80 × 4KB = 320KB in `.bss` (acceptable for kernel)
- Wastes memory if fewer tables needed

**Implementation:**
```rust
// Change from:
static mut PT_POOL: [PageTable; 8] = [const { PageTable::new() }; 8];

// To:
static mut PT_POOL: [PageTable; 80] = [const { PageTable::new() }; 80];
```

**Reversal:** Change constant back to 8

---

### Option B: Implement 2MB Block Mappings

**Description:** Use L2 block descriptors (2MB granularity) instead of L3 pages

**Pros:**
- Only ~4 tables needed total
- More efficient TLB usage
- Standard practice for kernel identity maps

**Cons:**
- Requires new `map_block()` function
- Different descriptor format (no TABLE bit at L2)
- Less granular permissions

**Implementation:** New function + modify `identity_map_range()` to detect when 2MB alignment allows block mapping

**Reversal:** More complex; would need to revert logic changes

---

### Option C: Hybrid Approach

**Description:** Use 2MB blocks for large aligned regions, 4KB pages for small/unaligned regions

**Pros:**
- Optimal memory usage
- Flexible mapping granularity

**Cons:**
- Most complex implementation
- Overkill for current needs

---

## Selected Fix: Option A

**Rationale:**
1. **Simplicity** — Single constant change, no new logic
2. **Correctness** — Guaranteed to work for current memory layout
3. **Reversible** — Trivial to undo
4. **Memory cost acceptable** — 320KB is <1% of 512MB RAM

Future teams can implement Option B (2MB blocks) as an optimization.

---

## Proposed Changes

### [MODIFY] [mmu.rs](file:///home/vince/Projects/LevitateOS/levitate-hal/src/mmu.rs)

#### Change 1: Increase pool size (line 344)

```diff
-/// We need at least 4 tables for L0, L1, L2, L3 to identity map kernel.
-static mut PT_POOL: [PageTable; 8] = [const { PageTable::new() }; 8];
+/// For 128MB kernel identity map with 4KB pages:
+/// - 1 L0 + 1 L1 + 1 L2 + ~64 L3 = ~67 tables
+/// - Plus ~10 for MMIO regions = ~77 tables
+/// - Rounded up to 80 for safety margin
+static mut PT_POOL: [PageTable; 80] = [const { PageTable::new() }; 80];
```

#### Change 2: Update integration-guide.md GOTCHA 4

Remove or update the warning about pool size being too small.

---

## Reversal Strategy

**If the fix causes issues:**

1. Revert `PT_POOL` size back to 8
2. Implement 2MB block mapping as alternative
3. Or wait for dynamic frame allocator

**Git command:**
```bash
git revert HEAD
```

**Verification of reversal:**
- Build succeeds
- Kernel boots with MMU disabled (as before)

---

## Test Strategy

### Automated Tests

No unit tests for MMU currently. Build verification only:
```bash
cargo build --release
```

### Manual Verification

1. Apply fix
2. Attempt to enable MMU in `kmain()` with full identity mapping
3. Run in QEMU with `-d mmu` to observe page table walks
4. Verify kernel continues to run after `enable_mmu()`

**Expected output:**
```
Initializing MMU...
Root PT at: 0x40XXXXXX
About to enable MMU...
MMU enabled with identity mapping.
```

**Failure indicators:**
- Hang after "About to enable MMU"
- Translation fault in QEMU logs
- "Page table pool exhausted" error (if fix didn't apply correctly)

---

## Implementation Checklist

- [ ] Modify `PT_POOL` size in `mmu.rs`
- [ ] Update comment explaining size calculation
- [ ] Update `integration-guide.md` GOTCHA 4
- [ ] Build and verify no compile errors
- [ ] Test in QEMU with MMU enable code
- [ ] Document in team file

---

## References

- [Phase 1](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-1.md) — Scoping
- [Phase 2](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/bugfix-pool-size/phase-2.md) — Root Cause
- [integration-guide.md](file:///home/vince/Projects/LevitateOS/docs/planning/mmu-page-tables/integration-guide.md) — Original GOTCHA documentation
