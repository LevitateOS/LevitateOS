# Bugfix Plan: GPU Display Deadlock

**Created by:** TEAM_084  
**Date:** 2026-01-05  
**Status:** ✅ IMPLEMENTED by TEAM_086  
**Estimated Effort:** 3 UoWs

---

## Phase Files

| Phase | File | Status |
|-------|------|--------|
| 1. Understanding and Scoping | `phase-1.md` | ✅ Complete |
| 2. Root Cause Analysis | `phase-2.md` | ✅ Complete |
| 3. Fix Design and Validation | `phase-3.md` | ✅ Complete |
| 4. Implementation and Tests | `phase-4.md` | 🔲 Ready |
| 5. Cleanup and Handoff | `phase-5.md` | 🔲 Pending |

## UoW Files (Phase 4)

| UoW | File | Scope |
|-----|------|-------|
| 4.1.1 | `phase-4-step-1-uow-1.md` | Refactor Display struct in gpu.rs |
| 4.2.1 | `phase-4-step-2-uow-1.md` | Update terminal.rs |
| 4.3.1 | `phase-4-step-3-uow-1.md` | Update console_gpu.rs and cursor.rs |

---

## 1. Problem Statement

`Display::draw_iter()` in `kernel/src/gpu.rs` locks `GPU` internally. Multiple call sites then attempt to lock `GPU` again (typically for `flush()`), causing a deadlock because `IrqSafeLock` is not re-entrant.

### Symptoms
- System hangs when GPU text rendering is enabled
- Dual console (UART + GPU) disabled as workaround
- Mouse cursor drawing broken

### Root Cause Location
`@kernel/src/gpu.rs:108-136` — `Display::draw_iter()` acquires `GPU.lock()` internally

---

## 2. Affected Code

### Primary (must change)
| File | Function/Struct | Issue |
|------|-----------------|-------|
| `kernel/src/gpu.rs` | `Display::draw_iter()` | Locks GPU internally |
| `kernel/src/gpu.rs` | `Display::size()` | Also locks GPU internally |

### Secondary (must update after primary fix)
| File | Function | Current Pattern |
|------|----------|-----------------|
| `kernel/src/terminal.rs` | `write_char()`, `clear()`, `backspace()`, etc. | Pass `&mut Display`, expect internal lock |
| `kernel/src/terminal.rs` | `show_cursor()`, `hide_cursor()` | Mix of direct GPU access + Display |
| `kernel/src/console_gpu.rs` | `clear()`, `check_blink()` | Create Display, use it, then try to lock GPU |
| `kernel/src/cursor.rs` | `draw()` | Uses Display then locks GPU for flush |

---

## 3. Proposed Solution

### Option A: Display accepts `&mut GpuState` (Recommended)

Refactor `Display` to be a wrapper that borrows `GpuState`:

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
        // No lock needed - caller already holds GpuState
        let fb = self.state.framebuffer();
        // ... draw pixels ...
    }
}
```

**Usage pattern:**
```rust
let mut gpu_guard = GPU.lock();
if let Some(state) = gpu_guard.as_mut() {
    let mut display = Display::new(state);
    Text::new("Hello", point, style).draw(&mut display)?;
    state.flush();  // Safe - we still hold the guard
}
```

**Pros:**
- Explicit lock management
- Single lock scope for draw + flush
- Follows Rust borrowing semantics

**Cons:**
- Requires updating all call sites
- Display is no longer zero-sized (contains reference)

### Option B: Display with integrated flush

Keep Display zero-sized but add `flush()` method:

```rust
impl Display {
    pub fn flush(&self) {
        GPU.lock().as_mut().map(|s| s.flush());
    }
}
```

And remove separate flush calls.

**Pros:** Fewer changes to call sites  
**Cons:** Doesn't fix the fundamental lock ordering issue; still can deadlock in complex scenarios

### Recommendation: Option A

Option A provides correct-by-construction lock safety.

---

## 4. Implementation Steps

### Step 1: Refactor Display struct (gpu.rs)
- [ ] Add lifetime parameter to Display
- [ ] Add `state: &'a mut GpuState` field
- [ ] Update `draw_iter()` to use `self.state` instead of locking
- [ ] Update `size()` to use `self.state`
- [ ] Add `Display::new(state: &mut GpuState)` constructor

### Step 2: Update terminal.rs
- [ ] Change all functions taking `&mut Display` to manage GPU lock themselves
- [ ] Create Display from locked GpuState within each function
- [ ] Remove redundant `GPU.lock()` calls
- [ ] Fix `scroll_up()`, `show_cursor()`, `hide_cursor()` which already do direct GPU access

### Step 3: Update console_gpu.rs  
- [ ] Fix `clear()` to use single lock scope
- [ ] Fix `check_blink()` to use single lock scope
- [ ] Verify `write_str()` still works (already bypasses Display)

### Step 4: Update cursor.rs
- [ ] Fix `draw()` to use single lock scope with new Display

### Step 5: Remove breadcrumbs and update documentation
- [ ] Remove BREADCRUMB from gpu.rs
- [ ] Update GOTCHAS.md to mark issue as resolved
- [ ] Update TODO.md to mark as complete
- [ ] Remove workaround notes

---

## 5. Testing Strategy

### Before Starting
- [ ] Verify current build passes
- [ ] Verify current tests pass
- [ ] Document current GPU behavior (broken)

### After Each Step
- [ ] Build compiles without errors
- [ ] No new warnings

### Final Verification
- [ ] Boot kernel in QEMU
- [ ] Verify text appears on GPU screen
- [ ] Verify cursor blinks
- [ ] Verify no deadlocks during normal operation
- [ ] Test dual console output (UART + GPU)

---

## 6. Rollback Plan

If issues arise:
1. Revert to workaround state (disable dual console)
2. Keep `write_str()` bypass approach
3. Document partial progress in team file

---

## 7. Open Questions

None at this time. Root cause and fix approach are clear.

---

## 8. Dependencies

- None external
- Internal: Must complete before re-enabling dual console
