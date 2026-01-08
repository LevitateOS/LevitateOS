# Phase 3: Fix Design and Validation Plan

**Bug:** VirtIO GPU "Display output is not active"  
**Team:** TEAM_089  
**Status:** 🟡 IN PROGRESS

---

## Root Cause Summary

QEMU `virtio-gpu-device` display surface goes inactive shortly after boot. The VirtIO GPU driver is correct, but QEMU may require continuous flush commands to keep the display active.

**Location:** QEMU behavior (external), but can be worked around in:
- `kernel/src/gpu.rs` — flush cadence
- `kernel/src/main.rs` — timer-based refresh
- `run.sh` — QEMU device type

---

## Fix Strategy Options

### Option A: Timer-Based GPU Flush (RECOMMENDED)

**Approach:** Add a periodic GPU flush triggered by the timer interrupt (~10Hz).

**Pros:**
- Ensures continuous display activity
- Minimal code change
- Keeps display alive even during idle

**Cons:**
- Slight CPU overhead (10 flushes/second)
- Timer handler becomes slightly more complex

**Reversal:** Remove the flush call from timer handler

---

### Option B: Change QEMU Device Type to `virtio-vga`

**Approach:** Modify `run.sh` to use `-device virtio-vga,xres=1280,yres=800`

**Pros:**
- Simple configuration change
- VGA emulation may have different display surface handling

**Cons:**
- May not work on aarch64 virt machine
- Different feature set than virtio-gpu-device
- May require driver changes

**Reversal:** Revert run.sh change

---

### Option C: Increase Main Loop Flush Frequency

**Approach:** Change flush from every 10000 iterations to every ~100 iterations in main loop.

**Pros:**
- Simple code change
- Already have flush logic in main loop

**Cons:**
- Only works when main loop is running
- Doesn't help during blocking operations
- Less reliable than timer-based

**Reversal:** Change back to original iteration count

---

## Selected Fix: Option A (Timer-Based GPU Flush)

**Why:**
1. Most reliable — timer fires regardless of main loop state
2. Guaranteed cadence — consistent 10Hz refresh
3. Minimal change — one function modification
4. Easy reversal — just remove the call

---

## Reversal Strategy

### How to Revert
1. Remove GPU flush call from `TimerHandler::handle()` in `main.rs`
2. Run `./run.sh` to verify kernel boots
3. Serial console should still work (GPU display will still be broken)

### Signals to Revert
- Kernel hangs or panics after adding timer flush
- Timer interrupt stops working
- Serial output stops

### Side Effects of Reversal
- None — GPU display will return to "not active" state (original bug)
- Serial console remains functional

---

## Test Strategy

### 1. Reproduction Test (Manual)
```bash
# Before fix: Verify bug exists
./run.sh
# Expected: Red flash, then "Display output is not active"
```

### 2. Fix Verification (Manual)
```bash
# After fix: Verify display stays active
./run.sh
# Expected: Display shows boot messages and remains active
# Expected: Can see text output on GPU screen
```

### 3. Regression Tests (Automated)
```bash
# Run behavior tests to ensure kernel still boots correctly
cargo xtask test behavior
# Expected: All tests pass
```

### 4. Timer Interrupt Test (Manual)
- Verify timer interrupt still fires at 100Hz
- Check serial output for any timing issues

---

## Impact Analysis

### API Changes
- None — internal implementation only

### Behavior Changes
- GPU display stays active instead of going inactive
- Timer handler does slightly more work per tick

### Performance
- ~10 GPU flush calls per second (minimal overhead)
- Flush is a fast VirtIO operation

### Resource Usage
- No additional memory
- Minimal CPU overhead

---

## Files to Modify

| File | Change |
|------|--------|
| `kernel/src/main.rs` | Add GPU flush to `TimerHandler::handle()` |
| (optional) `run.sh` | Test with `virtio-vga` if Option A fails |

---

## Phase 3 Outcome

Selected approach: Timer-based GPU flush at ~10Hz.

**Next:** Phase 4 (Implementation and Tests)
