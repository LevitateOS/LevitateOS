# Phase 1 — Understanding and Scoping

**Bug:** GPU Display Deadlock  
**Team:** TEAM_084  
**Status:** COMPLETE (from prior investigation)

---

## 1.1 Bug Summary

The `Display` struct's `DrawTarget` implementation in `kernel/src/gpu.rs` acquires `GPU.lock()` internally on every draw operation. When callers subsequently attempt to lock `GPU` again (e.g., for `flush()`), the system deadlocks because `IrqSafeLock` is not re-entrant.

**Severity:** HIGH  
**Impact:**
- GPU text rendering completely broken
- Dual console (UART + GPU) disabled as workaround
- Mouse cursor drawing causes hangs

---

## 1.2 Reproduction Status

**Reproducible:** Yes (when using affected code paths)

**Reproduction Steps:**
1. Enable dual console output (register GPU callback)
2. Boot kernel in QEMU
3. Any `println!` triggers `console_gpu::write_str()`
4. If code path uses `Display` then calls `GPU.lock()` → system hangs

**Expected Behavior:** Text renders on GPU screen, system continues  
**Actual Behavior:** System hangs permanently

**Current Mitigation:** TEAM_083 disabled dual console and rewrote `write_str()` to bypass `Display`

---

## 1.3 Context

### Suspected Code Areas
| File | Function | Role |
|------|----------|------|
| `kernel/src/gpu.rs` | `Display::draw_iter()` | Locks GPU internally (ROOT CAUSE) |
| `kernel/src/gpu.rs` | `Display::size()` | Also locks GPU internally |
| `kernel/src/terminal.rs` | Multiple functions | Uses Display for text rendering |
| `kernel/src/console_gpu.rs` | `clear()`, `check_blink()` | Uses Display then tries to flush |
| `kernel/src/cursor.rs` | `draw()` | Uses Display then locks for flush |

### Recent Related Changes
- TEAM_083: Rewrote `console_gpu::write_str()` as workaround
- TEAM_083: Disabled dual console callback registration
- TEAM_083: Added BREADCRUMB warning in `gpu.rs`

### Related Documentation
- `docs/GOTCHAS.md` — Documents the deadlock pattern
- `TODO.md` — Lists this as critical issue
- `.teams/TEAM_083_investigate_unusable_shell.md` — Initial investigation

---

## 1.4 Constraints

| Constraint | Details |
|------------|---------|
| Backwards Compatibility | `embedded_graphics` `DrawTarget` trait must still be implemented |
| Performance | Drawing should not require multiple lock/unlock cycles |
| API Stability | Terminal and cursor modules depend on Display |

---

## 1.5 Open Questions

None — root cause and scope are clear from TEAM_083/084 investigation.

---

## 1.6 Steps

### Step 1 — Consolidate Bug Information ✅
Completed during investigation phase. All information captured above.

### Step 2 — Confirm Reproduction ✅
Confirmed by TEAM_083. Deadlock occurs when:
1. `Display::draw_iter()` is called (locks GPU)
2. Lock is released when `draw_iter()` returns
3. Caller then calls `GPU.lock()` for flush
4. If any nested call re-enters `draw_iter()` while GPU is locked → deadlock

### Step 3 — Identify Suspected Code Areas ✅
All affected files identified (see 1.3 above).

---

**Phase 1 Complete.** Proceed to Phase 2.
