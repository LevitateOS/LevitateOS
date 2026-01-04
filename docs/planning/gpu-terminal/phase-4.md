# Phase 4: Integration & Testing — GPU Terminal

**Feature**: GPU Refinement — Extended Scope (Phase 6, Task 6.2)
**Team**: TEAM_060
**Depends on**: `phase-2.md`, `phase-3.md`

---

## 1. Integration Points

The GPU Terminal interacts with the following kernel subsystems:

- **GPU Driver (`gpu.rs`)**: 
  - Directly accesses the mutable framebuffer via `GPU.lock()`.
  - Links to `Display` trait for rendering primitives.
  - Relies on `GpuState::flush()` for hardware-level buffer swapping.
- **Console Interface (`main.rs`)**:
  - Echoes UART input to the terminal emulator.
  - Maps `\r` and `\n` from serial input to terminal controls.
- **Input System (`input.rs` / `cursor.rs`)**:
  - Operates concurrently. The tablet mouse cursor is drawn over the terminal text.
  - **Constraint**: Terminal rendering must not disrupt the mouse cursor (fixed in TEAM_059).

---

## 2. Test Strategy

### 2.1 Automated Tests (Regression Protection)
- **Crate Tests**: Run `cargo xtask test` to ensure core primitives (`levitate-utils`) and HAL logic (`levitate-hal`) are unaffected.
- **Mock Tests**: Ensure `terminal.rs` logic remains unit-testable if separated from hardware (currently hardware-bound).

### 2.2 Runtime Verification (The "Human-in-the-Loop" Suite)
Since the GPU terminal is inherently visual, we use a structured manual suite:

| ID | Behavior | Method | Expected |
|----|----------|--------|----------|
| **VIS-1** | Boot Banner | Visual | "LevitateOS Terminal v0.1" visible at top. |
| **VIS-2** | Text Entry | Visual | Typed A-Z mapping correctly to screen coordinates. |
| **VIS-3** | Newline | Visual | Cursor moves to col 0 of the *next* line. No overlap. |
| **VIS-4** | Scrolling | Visual | Filling the last line moves the whole screen up. |
| **VIS-5** | Resolution | Visual | Works on both 1280x800 and 2400x1080 (Pixel 6). |

### 2.3 Behavior Inventory
Update `docs/testing/behavior-inventory.md` with full `✅` status for Group 10 after this phase.

---

## 3. Impact Analysis

| Area | Impact | Mitigation |
|------|--------|------------|
| **Memory** | Minimal (~40 bytes for state) | No large back-buffers are used; we render directly to FB. |
| **CPU** | Low (during text render) | `embedded-graphics` is efficient. Heavy UART traffic may cause brief spikes. |
| **Latency** | Medium (scroll operation) | `copy_within` is used for fastest possible FB manipulation. |
| **Coexistence** | Critical (Mouse Cursor) | `state.flush()` is called after every char render to ensure visibility. |

---

## 4. Success Metrics

- [ ] **Visually Stable**: No flickering or flickering minimal during scroll.
- [ ] **Correct Geometry**: `240x49` chars on Pixel 6 resolution.
- [ ] **Persistent State**: Backspace/Newline don't leave artifacts.
- [ ] **Zero Regressions**: 149/149 baseline behaviors pass.

---

## 5. Rollback Plan

1. Revert `main.rs` changes to bypass `terminal::Terminal`.
2. Restore original rectangle-based graphics demo in `main.rs`.
3. Revert `run.sh` to `-display none` if visual environment is unstable.
