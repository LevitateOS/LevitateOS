# UoW: Refactor Display Struct in gpu.rs

**Phase:** 4  
**Step:** 1  
**UoW:** 1  
**Team:** TEAM_084

---

## Goal

Refactor `Display` in `kernel/src/gpu.rs` to accept `&mut GpuState` instead of locking internally.

---

## Input Context

Read first:
- `phase-3.md` — Fix design (Option A)
- `kernel/src/gpu.rs` — Current implementation

---

## Tasks

### Task 1: Add lifetime parameter to Display

Change from:
```rust
pub struct Display;
```

To:
```rust
pub struct Display<'a> {
    state: &'a mut GpuState,
}

impl<'a> Display<'a> {
    pub fn new(state: &'a mut GpuState) -> Self {
        Self { state }
    }
}
```

### Task 2: Update DrawTarget implementation

Change from:
```rust
impl DrawTarget for Display {
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> {
        let mut guard = GPU.lock();
        let state = guard.as_mut().ok_or(GpuError::NotInitialized)?;
        // ...
    }
}
```

To:
```rust
impl<'a> DrawTarget for Display<'a> {
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> {
        let width = self.state.width;
        let height = self.state.height;
        let fb = self.state.framebuffer();
        // ... (same pixel writing logic, using self.state)
    }
}
```

**Note:** Error type changes from `GpuError::NotInitialized` to `Infallible` or remove the error case since we now have guaranteed access to GpuState.

### Task 3: Update OriginDimensions implementation

Change from:
```rust
impl OriginDimensions for Display {
    fn size(&self) -> Size {
        let guard = GPU.lock();
        // ...
    }
}
```

To:
```rust
impl<'a> OriginDimensions for Display<'a> {
    fn size(&self) -> Size {
        Size::new(self.state.width, self.state.height)
    }
}
```

### Task 4: Remove BREADCRUMB comment

The deadlock warning breadcrumb is no longer needed after fix.

### Task 5: Build check

Run `cargo check` to identify all call sites that need updating (will fail with type errors).

---

## Expected Outputs

1. `Display` struct now has lifetime parameter
2. `DrawTarget` impl uses `self.state` instead of locking
3. `OriginDimensions` impl uses `self.state`
4. Build fails with errors in `terminal.rs`, `console_gpu.rs`, `cursor.rs` (expected — these are updated in subsequent UoWs)

---

## Verification

```bash
cargo check 2>&1 | grep "error\[E"
```

Expected: Errors about `Display` type mismatches in other files. No errors within `gpu.rs` itself.
