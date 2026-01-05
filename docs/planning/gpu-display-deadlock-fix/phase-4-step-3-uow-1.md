# UoW: Update console_gpu.rs and cursor.rs

**Phase:** 4  
**Step:** 3  
**UoW:** 1  
**Team:** TEAM_084

---

## Goal

Update remaining files to work with new Display API and verify the complete fix.

---

## Input Context

Read first:
- `phase-4-step-1-uow-1.md` — Display struct changes
- `phase-4-step-2-uow-1.md` — Terminal changes
- `kernel/src/console_gpu.rs` — Current implementation
- `kernel/src/cursor.rs` — Current implementation

---

## Tasks

### Task 1: Update console_gpu.rs::clear()

Current (broken):
```rust
pub fn clear() {
    let mut guard = GPU_TERMINAL.lock();
    if let Some(term) = guard.as_mut() {
        let mut display = Display;  // OLD: zero-sized
        term.clear(&mut display);
        let mut gpu = GPU.lock();   // DEADLOCK RISK
        if let Some(state) = gpu.as_mut() {
            state.flush();
        }
    }
}
```

Fixed:
```rust
pub fn clear() {
    let mut gpu_guard = GPU.lock();
    let mut term_guard = GPU_TERMINAL.lock();
    
    if let (Some(gpu_state), Some(term)) = (gpu_guard.as_mut(), term_guard.as_mut()) {
        let mut display = Display::new(gpu_state);
        term.clear(&mut display);
        gpu_state.flush();  // Safe — same lock scope
    }
}
```

### Task 2: Update console_gpu.rs::check_blink()

Current:
```rust
pub fn check_blink() {
    let mut guard = GPU_TERMINAL.lock();
    if let Some(term) = guard.as_mut() {
        let mut display = Display;
        term.check_blink(&mut display);
    }
}
```

Fixed:
```rust
pub fn check_blink() {
    let mut gpu_guard = GPU.lock();
    let mut term_guard = GPU_TERMINAL.lock();
    
    if let (Some(gpu_state), Some(term)) = (gpu_guard.as_mut(), term_guard.as_mut()) {
        let mut display = Display::new(gpu_state);
        term.check_blink(&mut display);
        gpu_state.flush();
    }
}
```

### Task 3: Verify write_str() is already safe

TEAM_083 rewrote this to bypass Display. Verify it still works and consider whether to convert it to use new Display API for consistency.

### Task 4: Update cursor.rs::draw()

**TEAM_085 REVIEW NOTE:** Actual code is worse than described. There are **4 separate lock acquisitions**:

1. Line 54: `GPU.lock()` — restore previous pixels + flush
2. Line 80: `GPU.lock()` — save new pixels under cursor
3. Line 103: `Display.draw()` — internally locks GPU (via draw_iter)
4. Line 108: `GPU.lock()` — explicit flush after draw

Current (broken):
```rust
pub fn draw(display: &mut Display) {
    // Block 1: Restore previous pixels
    if state.has_saved {
        let mut guard = crate::gpu::GPU.lock();  // LOCK #1
        // ... restore pixels ...
        gpu.flush();
    }  // unlocks
    
    // Block 2: Save new pixels
    {
        let mut guard = crate::gpu::GPU.lock();  // LOCK #2
        // ... save pixels ...
    }  // unlocks
    
    // Block 3: Draw cursor
    let _ = Rectangle::new(...).draw(display);  // LOCK #3 (inside draw_iter)
    
    // Block 4: Flush
    GPU.lock().as_mut().map(|s| s.flush());     // LOCK #4 - DEADLOCK with new API
}
```

Fixed — change signature to accept GpuState:
```rust
pub fn draw(gpu_state: &mut GpuState) {
    let mut state = CURSOR.lock();
    
    // Restore previous pixels (direct framebuffer access)
    if state.has_saved {
        let fb = gpu_state.framebuffer();
        // ... restore pixels ...
    }
    
    // Save new pixels
    let fb = gpu_state.framebuffer();
    // ... save pixels ...
    
    // Draw cursor
    let mut display = Display::new(gpu_state);
    let _ = Rectangle::new(...).draw(&mut display);
    
    // Flush
    gpu_state.flush();
}
```

### Task 5: Update cursor.rs imports and any callers

Update imports and fix any code that calls `cursor::draw()`.

### Task 6: Full build

```bash
cargo build
```

### Task 7: Boot test

1. Run kernel in QEMU
2. Verify boot completes without hang
3. Re-enable dual console callback (if disabled)
4. Verify GPU displays text

---

## Expected Outputs

1. `console_gpu.rs` uses new Display API correctly
2. `cursor.rs` uses new Display API correctly  
3. No GPU.lock() calls after Display usage
4. Full build succeeds
5. Kernel boots without deadlock

---

## Verification

```bash
# Build
cargo build

# Boot test
cargo xtask run
# Or: ./run.sh
```

Watch for:
- Boot messages on GPU screen
- No system hangs
- Cursor visible and blinking
