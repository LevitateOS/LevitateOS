# Phase 3 — Fix Design and Validation Plan

**Bug:** GPU Display Deadlock  
**Team:** TEAM_084  
**Status:** IN_PROGRESS

---

## 3.1 Root Cause Summary

**What:** `Display::draw_iter()` acquires `GPU.lock()` internally.  
**Where:** `kernel/src/gpu.rs:113`  
**Why It Breaks:** Callers need to lock GPU again for `flush()`, but `IrqSafeLock` is not re-entrant.

---

## 3.2 Fix Strategy Options

### Option A: Display borrows `&mut GpuState` (RECOMMENDED)

Refactor `Display` to hold a reference to `GpuState` instead of locking internally:

```rust
pub struct Display<'a> {
    state: &'a mut GpuState,
}

impl<'a> Display<'a> {
    pub fn new(state: &'a mut GpuState) -> Self {
        Self { state }
    }
}

impl<'a> DrawTarget for Display<'a> {
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> {
        // Use self.state directly — no lock needed
        let fb = self.state.framebuffer();
        // ... draw pixels ...
    }
}
```

**Usage Pattern:**
```rust
let mut gpu_guard = GPU.lock();
if let Some(state) = gpu_guard.as_mut() {
    let mut display = Display::new(state);
    Text::new("Hello", point, style).draw(&mut display)?;
    state.flush();  // Safe — still holding gpu_guard
}
```

| Aspect | Assessment |
|--------|------------|
| Complexity | Medium — requires updating all call sites |
| Risk | Low — follows Rust borrow semantics |
| Reversibility | Easy — revert Display struct changes |
| Performance | Improved — single lock scope |

### Option B: Add `Display::flush()` method

Keep Display zero-sized, add integrated flush:

```rust
impl Display {
    pub fn flush(&self) {
        GPU.lock().as_mut().map(|s| s.flush());
    }
}
```

| Aspect | Assessment |
|--------|------------|
| Complexity | Low — minimal code changes |
| Risk | Medium — doesn't fix nested lock scenarios |
| Reversibility | Easy |
| Performance | Same |

**Rejected:** Doesn't solve the fundamental problem. Nested draw operations would still deadlock.

### Option C: Make IrqSafeLock re-entrant

Modify `IrqSafeLock` to track owner and allow re-entry.

| Aspect | Assessment |
|--------|------------|
| Complexity | High — requires thread/context tracking |
| Risk | High — changes core synchronization primitive |
| Reversibility | Difficult |
| Performance | Degraded — ownership checks on every lock |

**Rejected:** Over-engineering. The root issue is API design, not the lock.

---

## 3.3 Chosen Approach: Option A

**Rationale:**
1. Explicit lock management follows Rust idioms
2. Compile-time enforcement of correct lock scope
3. No runtime overhead for re-entrancy tracking
4. Consistent with how other drivers handle mutable state

---

## 3.4 Reversal Strategy

**Trigger Conditions:**
- New deadlocks appear in unrelated code
- Performance regression observed
- embedded_graphics compatibility issues

**Rollback Steps:**
1. Revert `Display` struct to zero-sized
2. Revert `DrawTarget` impl to use internal locking
3. Re-apply TEAM_083 workarounds (disable dual console)
4. Document regression in team file

---

## 3.5 Test Strategy

### New Tests to Add
| Test | Purpose |
|------|---------|
| Integration: GPU draw + flush in single scope | Verify no deadlock |
| Integration: Multiple draw calls before flush | Verify batching works |
| Integration: Terminal write + scroll + cursor | Verify complex sequences work |

### Existing Tests
No GPU-specific unit tests exist currently. The kernel is tested via boot behavior.

### Verification Approach
1. Boot kernel in QEMU
2. Enable dual console (re-enable callback)
3. Verify boot messages appear on GPU
4. Verify cursor blinks
5. Type characters and verify they appear

---

## 3.6 Impact Analysis

### API Changes
| Change | Impact |
|--------|--------|
| `Display` gains lifetime parameter | All usages must be within lock scope |
| `Display::new(state)` constructor required | No more zero-cost instantiation |

### Affected Modules
| Module | Changes Needed |
|--------|----------------|
| `gpu.rs` | Display struct + DrawTarget impl |
| `terminal.rs` | All functions taking `&mut Display` |
| `console_gpu.rs` | `clear()`, `check_blink()`, `write_str()` |
| `cursor.rs` | `draw()` function |

### Performance
- **Improved:** Single lock acquisition per draw sequence
- **Unchanged:** Pixel write performance

---

## 3.7 Steps

### Step 1 — Define Fix Requirements ✅
- Display must not lock GPU internally
- Callers must manage GPU lock scope
- embedded_graphics `DrawTarget` trait must remain implemented
- Flush must be callable within same lock scope as draw

### Step 2 — Propose Fix Options ✅
Three options evaluated above. Option A selected.

### Step 3 — Define Test Changes ✅
Manual boot verification. No automated GPU tests currently exist.

---

**Phase 3 Complete.** Proceed to Phase 4.
