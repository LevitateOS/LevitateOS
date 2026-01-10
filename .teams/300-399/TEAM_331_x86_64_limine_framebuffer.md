# TEAM_331: x86_64 Limine Framebuffer Support

**Date:** 2026-01-09  
**Status:** ✅ FIXED  
**Bug:** x86_64 display is tiny (640x480) because QEMU GTK/SDL don't respect virtio-gpu resolution

## Bug Report

### Symptom
- x86_64 `./run.sh` shows a tiny ~640x480 window
- Text is unreadable due to small size
- VNC/headless mode works correctly at 1280x800

### Expected
- x86_64 should display at 1280x800 (same as screenshot tests)
- Text should be readable

### Root Cause (from TEAM_330 investigation)
QEMU's GTK/SDL display backends initialize virtio-gpu at their own default resolution (640x480/720x400) regardless of xres/yres settings. This is a QEMU limitation, NOT a kernel bug.

## Solution Approach

Instead of working around QEMU, implement **Limine framebuffer support** in the kernel:

1. Limine bootloader provides a framebuffer via `FRAMEBUFFER_REQUEST`
2. This framebuffer comes from QEMU's VGA device (`-vga std`)
3. VGA device respects Limine's RESOLUTION config
4. Kernel can render directly to this framebuffer

### Why This Works
- Limine's `RESOLUTION=1280x800` in limine.cfg tells Limine to request that resolution
- QEMU's VGA device provides a framebuffer at that resolution
- GTK/SDL display shows the VGA output at the correct size
- No virtio-gpu needed for x86_64 GUI mode

## Fix Implemented

### Files Changed

1. **`kernel/src/gpu.rs`** - Added Limine framebuffer fallback
   - Created `FramebufferGpu` struct for direct framebuffer access
   - Created `FramebufferDisplay` with `DrawTarget` implementation
   - Created `UnifiedDisplay` enum to support both VirtIO and Limine backends
   - Modified `init()` to fall back to Limine framebuffer when VirtIO GPU unavailable

2. **`xtask/src/qemu/builder.rs`** - Updated QEMU display config
   - x86_64 GTK mode: Uses `-vga std` (Limine gets framebuffer)
   - x86_64 VNC/headless: Uses virtio-gpu-pci with edid=on
   - aarch64: Always uses virtio-gpu-pci

3. **`limine.cfg`** - Added resolution config
   - `RESOLUTION=1280x800` tells Limine to request this framebuffer size

### How It Works

1. When x86_64 runs with `-vga std`, QEMU creates a standard VGA device
2. Limine bootloader requests a 1280x800 framebuffer from VGA
3. Kernel GPU init tries VirtIO first, fails (no virtio-gpu device)
4. Kernel falls back to Limine framebuffer from `boot_info().framebuffer`
5. Terminal renders to Limine framebuffer via `UnifiedDisplay`
6. GTK display shows VGA output at correct 1280x800 size

## Verification

- Screenshot test: **1280x800** resolution ✅
- aarch64: brightness 4.3 ✅
- x86_64: brightness 3.2 ✅

## Additional Fix: x86_64 Keyboard Input

### Issue
After switching to `-vga std` for x86_64, keyboard input stopped working.

### Root Cause
The input driver only supported MMIO transport (aarch64), but x86_64 uses PCI transport for VirtIO devices.

### Fix
1. **`kernel/src/input.rs`** - Added PCI transport support
   - Created `InputDevice` enum for MMIO/PCI
   - Added `init_pci()` function for x86_64
   - Updated `poll()` to handle both transports

2. **`kernel/src/virtio.rs`** - Call `input::init_pci()` on x86_64

3. **`crates/pci/src/lib.rs`** - Added `find_virtio_input()` function

### Verification
```
[INPUT] Initializing Input via PCI...
[PCI] Found VirtIO Input at 00:02.0
[INPUT] VirtIO Input initialized via PCI.
```

## Handoff Checklist

- [x] Kernel builds cleanly
- [x] Screenshot tests pass
- [x] x86_64 display at 1280x800
- [x] x86_64 keyboard input working via PCI
- [x] aarch64 unchanged (still uses virtio-gpu + MMIO input)
- [x] Team file updated
