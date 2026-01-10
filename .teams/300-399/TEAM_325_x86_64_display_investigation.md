# Investigation Request: x86_64 Display Black Screen

**Reported by:** TEAM_325  
**Date:** 2026-01-09  
**Priority:** Medium  
**Status:** Open

---

## Problem Summary

LevitateOS x86_64 boots successfully (serial console works) but the **display output is completely black**. The same QEMU configuration works perfectly with Alpine Linux, confirming this is a LevitateOS issue, not a QEMU/host problem.

---

## Evidence

### Screenshot Comparison

| OS | Architecture | Result |
|----|--------------|--------|
| Alpine Linux | x86_64 | ✅ Display works |
| Alpine Linux | aarch64 | ✅ Display works |
| LevitateOS | aarch64 | ✅ Display works |
| LevitateOS | x86_64 | ❌ **Black screen** |

### Screenshots Location

```
tests/screenshots/
├── alpine_x86_64_shell.png    # Alpine working
├── alpine_aarch64_shell.png   # Alpine working  
├── levitate_aarch64.png       # LevitateOS working
└── levitate_x86_64.png        # BLACK SCREEN
```

---

## How to Reproduce

### Quick Test (Recommended)

```bash
# Run the LevitateOS display test
cargo xtask test levitate

# Screenshots saved to tests/screenshots/
# Compare levitate_aarch64.png vs levitate_x86_64.png
```

### Manual Test

```bash
# Build x86_64
cargo xtask build all --arch x86_64

# Run with display
qemu-system-x86_64 \
  -M q35 -cpu qemu64 -m 512M -enable-kvm \
  -cdrom levitate.iso -boot d \
  -vga std \
  -serial mon:stdio

# Observe: Serial shows boot messages, but VGA display is black
```

### Alpine Comparison (Proves QEMU Works)

```bash
# Download Alpine if needed
./tests/images/download.sh

# Run Alpine screenshot test
cargo xtask test screenshot

# x86_64 Alpine shows working display
```

---

## Technical Analysis

### What Works
- x86_64 kernel boots (serial output confirms)
- x86_64 serial console functional
- aarch64 LevitateOS display works perfectly
- Alpine Linux x86_64 display works (same QEMU config)

### What's Broken
- x86_64 virtio-gpu/VGA framebuffer receives no writes
- No text appears on screen at any point during boot

### Likely Root Causes

1. **No x86_64 framebuffer driver**
   - aarch64 uses virtio-gpu driver
   - x86_64 may be missing equivalent driver

2. **VGA text mode vs linear framebuffer mismatch**
   - QEMU `-vga std` provides VGA text mode at 0xB8000
   - Kernel may expect different framebuffer address

3. **Console initialization missing on x86_64**
   - Check if `los_hal::io::vga` is initialized on x86_64
   - Compare with aarch64 display initialization path

---

## Files to Investigate

```
crates/hal/src/x86_64/io/vga.rs      # VGA driver (unused?)
crates/hal/src/x86_64/io/mod.rs      # IO initialization
crates/hal/src/aarch64/io/           # Compare with working aarch64
kernel/src/init.rs                    # Kernel init (arch differences?)
```

---

## Questions for Investigator

1. Is there a framebuffer/display driver for x86_64?
2. Is the VGA text buffer at 0xB8000 being written to?
3. What's the display initialization path for each arch?
4. Is there a `print!` macro that works on x86_64 display (not just serial)?

---

## Acceptance Criteria

- [ ] x86_64 LevitateOS shows boot messages on display
- [ ] `cargo xtask test levitate` produces non-black x86_64 screenshot
- [ ] Both architectures have parity in display functionality

---

## Related Files

- `xtask/src/tests/screenshot_levitate.rs` — Test that captures these screenshots
- `xtask/src/tests/screenshot_alpine.rs` — Proves QEMU display works
- `tests/screenshots/` — Screenshot artifacts
