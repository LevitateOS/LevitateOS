# UoW: Update terminal.rs

**Phase:** 4  
**Step:** 2  
**UoW:** 1  
**Team:** TEAM_084

---

## Goal

Update all `Display` usages in `kernel/src/terminal.rs` to work with the new lifetime-parameterized `Display<'a>`.

---

## Input Context

Read first:
- `phase-4-step-1-uow-1.md` — Display struct changes
- `kernel/src/terminal.rs` — Current implementation

---

## Tasks

### Task 1: Update function signatures

Functions currently take `&mut Display`. They should now take `&mut Display<'_>`:

```rust
// Before
pub fn write_char(&mut self, display: &mut Display, c: char)

// After  
pub fn write_char(&mut self, display: &mut Display<'_>, c: char)
```

Apply to all functions:
- `write_char()`
- `write_str()`
- `newline()`
- `carriage_return()`
- `tab()`
- `backspace()`
- `clear()`
- `scroll_up()`
- `check_blink()`
- `show_cursor()`
- `hide_cursor()`

### Task 2: Fix scroll_up() direct GPU access

`scroll_up()` currently locks GPU directly:
```rust
let mut guard = GPU.lock();
```

This is now redundant since Display already holds GpuState. Refactor to use the display's state:

**Option A:** Pass `&mut GpuState` directly to scroll_up instead of Display  
**Option B:** Access framebuffer through Display (requires adding accessor)

Recommended: Option A — change signature to accept `&mut GpuState` since scroll_up does low-level framebuffer ops.

### Task 3: Fix show_cursor() / hide_cursor()

**TEAM_085 REVIEW NOTE:** These functions are more complex than originally described:

**Current `show_cursor()` pattern (lines 352-387):**
1. Locks GPU to save pixels
2. **Drops the lock** (`drop(guard)`)
3. Uses Display to draw cursor block (which locks GPU again internally)

This pattern will **deadlock with new API** if Display is passed in from caller's lock scope.

**Current `hide_cursor()` pattern (lines 389-422):**
1. Locks GPU
2. Restores pixels AND flushes within same lock scope
3. Works correctly but doesn't use Display

**Refactor both functions to accept `&mut GpuState` instead of `&mut Display`:**

```rust
fn show_cursor(&mut self, gpu_state: &mut GpuState) {
    if self.cursor_visible { return; }
    
    // Save pixels using gpu_state.framebuffer() directly
    let fb = gpu_state.framebuffer();
    // ... save pixels to self.saved_pixels ...
    self.has_saved = true;
    
    // Draw cursor block
    let mut display = Display::new(gpu_state);
    let _ = Rectangle::new(...).draw(&mut display);
    
    self.cursor_visible = true;
}

fn hide_cursor(&mut self, gpu_state: &mut GpuState) {
    if !self.cursor_visible || !self.has_saved {
        self.cursor_visible = false;
        return;
    }
    
    // Restore pixels using gpu_state.framebuffer() directly
    let fb = gpu_state.framebuffer();
    // ... restore from self.saved_pixels ...
    gpu_state.flush();
    
    self.cursor_visible = false;
}
```

**Cascade effect:** All functions calling show_cursor/hide_cursor must also change signature.

### Task 4: Update import

Change:
```rust
use crate::gpu::{Display, GPU};
```

To:
```rust
use crate::gpu::{Display, GPU, GpuState};
```

### Task 5: Build check

Run `cargo check` to verify terminal.rs compiles.

---

## Expected Outputs

1. All Terminal functions work with new Display API
2. No internal GPU.lock() calls remain in terminal.rs (except where intentional)
3. `cargo check` passes for terminal.rs

---

## Verification

```bash
cargo check 2>&1 | grep -E "(terminal|error)"
```

Expected: No errors in terminal.rs.
