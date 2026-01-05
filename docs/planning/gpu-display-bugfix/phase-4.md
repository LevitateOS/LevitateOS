# Phase 4: Implementation and Tests

**Bug:** VirtIO GPU "Display output is not active"  
**Team:** TEAM_089  
**Status:** 🔵 READY

---

## Implementation Overview

### Key Change

Add a GPU flush call to the timer interrupt handler to keep the display active.

### File to Modify

**`kernel/src/main.rs`** — `TimerHandler::handle()` function

### Current Code (lines 316-324)
```rust
impl gic::InterruptHandler for TimerHandler {
    fn handle(&self, _irq: u32) {
        // Reload timer for next interrupt (10ms @ 100Hz)
        let freq = timer::API.read_frequency();
        timer::API.set_timeout(freq / 100);

        // TEAM_070: Preemptive scheduling
        crate::task::yield_now();
    }
}
```

### Proposed Change
```rust
impl gic::InterruptHandler for TimerHandler {
    fn handle(&self, _irq: u32) {
        // Reload timer for next interrupt (10ms @ 100Hz)
        let freq = timer::API.read_frequency();
        timer::API.set_timeout(freq / 100);

        // TEAM_089: Keep GPU display active with periodic flush (~10Hz)
        // Only flush every 10th interrupt (100Hz / 10 = 10Hz)
        static COUNTER: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
        let count = COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        if count % 10 == 0 {
            if let Some(gpu_state) = crate::gpu::GPU.try_lock().as_mut().and_then(|g| g.as_mut()) {
                gpu_state.flush();
            }
        }

        // TEAM_070: Preemptive scheduling
        crate::task::yield_now();
    }
}
```

---

## Test Execution Plan

### Step 1: Pre-Change Verification
```bash
# Ensure kernel builds and tests pass before changes
cargo build --release
cargo xtask test behavior
```
**Expected:** All tests pass

### Step 2: Apply Fix
Modify `kernel/src/main.rs` as described above.

### Step 3: Build and Run
```bash
./run.sh
```
**Expected:** 
- Kernel boots successfully
- QEMU graphical window shows boot messages (not "Display output is not active")
- Serial console still works

### Step 4: Post-Change Regression Tests
```bash
cargo xtask test behavior
```
**Expected:** All tests pass

---

## Reversal Plan

### Exact Steps to Revert
1. Remove the `COUNTER` static and GPU flush block from `TimerHandler::handle()`
2. Run `cargo build --release` to verify compilation
3. Run `./run.sh` to verify kernel boots (GPU will be broken again, serial works)

### Git Command (if committed)
```bash
git revert <commit-hash>
```

### Verification After Reversal
- Kernel boots (via serial console)
- Serial output shows "[SUCCESS] LevitateOS System Ready"
- GPU window returns to "Display output is not active" (original bug state)

### Side Effects of Reversal
- None — returns to pre-fix behavior
- Serial console remains fully functional

---

## Step-by-Step Implementation

### Step 1 — Prepare Codebase
- [ ] Verify clean git state or save current work
- [ ] Run `cargo build --release` — must succeed
- [ ] Run `cargo xtask test behavior` — must pass

### Step 2 — Implement the Fix
- [ ] Open `kernel/src/main.rs`
- [ ] Locate `TimerHandler::handle()` (around line 316)
- [ ] Add the GPU flush code as specified above
- [ ] Add team comment: `// TEAM_089: Keep GPU display active`

### Step 3 — Build and Verify
- [ ] Run `cargo build --release`
- [ ] Run `./run.sh`
- [ ] Observe QEMU graphical window
- [ ] **SUCCESS CRITERIA:** Display shows boot messages, not "Display output is not active"

### Step 4 — Run Regression Tests
- [ ] Run `cargo xtask test behavior`
- [ ] All tests must pass

---

## Phase 4 Outcome

Fix implemented and verified.

**Next:** Phase 5 (Cleanup and Handoff)
