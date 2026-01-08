# Phase 2 — Root Cause Analysis

**Bug:** GPU Display Deadlock  
**Team:** TEAM_084  
**Status:** COMPLETE (from prior investigation)

---

## 2.1 Hypotheses List

| ID | Hypothesis | Evidence | Confidence |
|----|------------|----------|------------|
| H1 | `Display::draw_iter()` locks GPU internally, preventing subsequent locks | Code inspection confirms lock at line 113 | HIGH ✅ CONFIRMED |
| H2 | `IrqSafeLock` is not re-entrant | Documented in `levitate-hal`, confirmed by behavior | HIGH ✅ CONFIRMED |
| H3 | Lock ordering inconsistency between modules | Some code locks GPU→TERMINAL, others TERMINAL→GPU | MEDIUM (secondary issue) |

---

## 2.2 Key Code Areas

### Primary: `kernel/src/gpu.rs` lines 108-136

```rust
impl DrawTarget for Display {
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> {
        let mut guard = GPU.lock();  // <-- LOCKS HERE
        let state = guard.as_mut().ok_or(GpuError::NotInitialized)?;
        // ... draw pixels ...
        Ok(())
    }  // <-- UNLOCKS HERE
}
```

### Secondary: All callers that use Display then lock GPU

1. **`cursor.rs::draw()`** — Lines 103-108
   ```rust
   .draw(display);           // Locks GPU via Display
   GPU.lock()...flush();     // Tries to lock again
   ```

2. **`console_gpu.rs::clear()`** — Lines 94-100
   ```rust
   term.clear(&mut display); // Uses Display internally
   GPU.lock()...flush();     // Tries to lock again
   ```

---

## 2.3 Investigation Strategy

Investigation complete. Root cause confirmed:

**The `Display` struct locks `GPU` internally on every `DrawTarget` method call. Any code that uses `Display` and then attempts to lock `GPU` for additional operations (like flush) will deadlock.**

---

## 2.4 Steps

### Step 1 — Map Execution Path ✅

Traced during investigation:
1. Caller creates `Display` instance
2. Caller calls `.draw(&mut display)` on an embedded_graphics primitive
3. `Display::draw_iter()` is invoked, which calls `GPU.lock()`
4. Pixels are written to framebuffer
5. `draw_iter()` returns, lock is released
6. Caller attempts `GPU.lock()` for flush
7. If step 3-5 is nested inside another GPU lock → DEADLOCK

### Step 2 — Narrow Down Faulty Region ✅

The fault is specifically in `Display::draw_iter()` acquiring its own lock rather than borrowing from caller.

### Step 3 — Validate Hypotheses ✅

- H1 CONFIRMED: Lock acquisition visible at line 113
- H2 CONFIRMED: `IrqSafeLock` wraps `Spinlock`, not re-entrant
- H3 ACKNOWLEDGED: Lock ordering varies but is secondary to H1/H2

---

## 2.5 Root Cause Statement

**Root Cause:** `Display::draw_iter()` in `kernel/src/gpu.rs` acquires `GPU.lock()` internally (line 113). This design assumes callers never need to access GPU after using Display within the same scope. However, callers need to call `flush()` after drawing, which requires locking GPU again. Since `IrqSafeLock` is not re-entrant, this causes permanent deadlock.

**Location:** `kernel/src/gpu.rs:113`

**Why It Exists:** The original design made `Display` a zero-sized type that could be used anywhere without passing GPU state. This was convenient but fundamentally unsafe with non-re-entrant locks.

---

**Phase 2 Complete.** Proceed to Phase 3.
