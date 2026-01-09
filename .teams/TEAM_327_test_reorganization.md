# TEAM_327: Test Suite Reorganization + Backspace Fix

**Date:** 2026-01-09  
**Status:** COMPLETE

---

## Summary

Reorganizing the `xtask/src/tests/` directory to eliminate duplicated code and add userspace test screenshots.

---

## Completed

### 1. Fixed aarch64 Initramfs Bug
- Root cause: Single `initramfs.cpio` shared across architectures
- Fix: Use arch-specific files (`initramfs_aarch64.cpio`, `initramfs_x86_64.cpio`)
- Files modified: 8 files in xtask

### 2. New Test Infrastructure
- Created `common.rs` - Shared `QemuSession` utilities
- Created `screenshot.rs` - Unified screenshot tests with userspace support

### 3. New Test Commands
```bash
cargo xtask test screenshot           # All screenshot tests
cargo xtask test screenshot:userspace # Run userspace tests + capture results
cargo xtask test screenshot:levitate  # Basic display test
cargo xtask test screenshot:alpine    # Alpine Linux reference tests
cargo xtask test userspace           # Alias for screenshot:userspace
```

---

### 4. Backspace Fix
- **Root Cause:** Keyboard sends `0x08` (BS), but TTY only recognized `0x7F` (DEL) as VERASE
- **Fix 1:** Handle BS/DEL before echo (prevents `^H` from being echoed)
- **Fix 2:** Accept both `0x08` and `0x7F` as erase characters
- **File:** `kernel/src/fs/tty/mod.rs:84-98`

### 5. Backspace Regression Test
- **Purpose:** Prevents false positives - catches if backspace breaks again
- **Test:** Types "abc", backspace, "x", verifies shell received "abx"
- **File:** `xtask/src/tests/backspace.rs`
- **Command:** `cargo xtask test backspace`

---

## Files Created/Modified

| File | Purpose |
|------|---------|
| `xtask/src/tests/backspace.rs` | **NEW** Backspace regression test |
| `xtask/src/tests/common.rs` | **NEW** Shared QemuSession, send_keys, etc. |
| `xtask/src/tests/screenshot.rs` | **NEW** Unified screenshot tests |
| `kernel/src/fs/tty/mod.rs` | **FIXED** Backspace handling |
| `docs/planning/test-reorganization/plan.md` | Reorganization plan |

---

## Files To Consolidate (Future)

These legacy files have duplicated QEMU setup code:
- `serial_input.rs` → merge into `input.rs`
- `keyboard_input.rs` → merge into `input.rs`  
- `shutdown.rs` → merge into `golden.rs`
- `screenshot_alpine.rs` → delete (replaced by screenshot.rs)
- `screenshot_levitate.rs` → delete (replaced by screenshot.rs)

---

## Verification

```bash
# Userspace test with screenshot
cargo xtask test userspace
# Screenshot saved to: tests/screenshots/userspace_aarch64.png
```

---

## Known Issues

1. **Backspace not working** - Needs investigation
2. **Echo duplication in test output** - May be related to backspace issue
3. **Legacy test files still present** - Will consolidate after verification
