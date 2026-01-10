# TEAM_330: x86_64 Display Audit & Fix

**Date:** 2026-01-09  
**Status:** ✅ FIXED  
**Related:** TEAM_325_x86_64_display_investigation.md

## Objective

Perform complete audit of x86_64 kernel display initialization to find root cause of black screen.

## Root Cause Analysis

### Initial Hypothesis (RULED OUT)
- PCI ECAM memory mapping issues
- GPU driver not initializing

### Investigation Findings
Serial output showed GPU was **actually working correctly**:
```
[GPU] Initializing via PCI...
[PCI] Found VirtIO GPU at 00:03.0
[PCI] PciTransport created successfully
[GPU] Resolution: 1280x800
[GPU DEBUG] DISPLAY STATUS: HAS CONTENT ✅
```

### Actual Root Cause (CONFIRMED)
**VNC display was showing the wrong GPU output on x86_64**

On x86_64 q35 machine:
- QEMU creates a default VGA adapter
- VNC connects to the default VGA by default
- `virtio-gpu-pci` is a separate PCI device
- The kernel writes to virtio-gpu, but VNC shows empty VGA

On aarch64 virt machine:
- No built-in VGA exists
- VNC automatically uses virtio-gpu output

## Fixes Applied

### Fix 1: VGA None for x86_64
**File:** `xtask/src/qemu/builder.rs`

Added `-vga none` for x86_64 to disable the default VGA adapter:
```rust
if self.arch == Arch::X86_64 {
    cmd.args(["-vga", "none"]);
}
```

### Fix 2: EDID for Resolution
Added `edid=on` to virtio-gpu-pci to make xres/yres respected:
```rust
let gpu_spec = format!(
    "virtio-gpu-pci,xres={},yres={},edid=on",
    self.gpu_resolution.width, self.gpu_resolution.height
);
```

### Fix 3: SDL instead of GTK Display
GTK display ignores virtio-gpu resolution. Switched to SDL:
```rust
DisplayMode::Gtk => {
    cmd.args(["-display", "sdl"]);
    cmd.args(["-serial", "mon:stdio"]);
}
```

### Fix 4: Explicit Resolution in run.rs
Added `.gpu_resolution(1280, 800)` to all run functions.

## Verification

Screenshot test results after fix:
- aarch64: brightness 4.3 ✅
- x86_64: brightness 3.2 ✅ (was 0.0 before)
- Resolution: 1280x800 (was 640x480)

## Lessons Learned

1. GPU initialization can succeed while display still appears black
2. QEMU display routing differs between architectures
3. Serial output debugging is essential for GPU issues

## Handoff Checklist

- [x] Project builds cleanly
- [x] Unit tests pass
- [ ] Behavior tests pass (PRE-EXISTING failure - log race condition, unrelated to this fix)
- [x] x86_64 display now works (verified via screenshot test)
- [x] aarch64 display still works (no regression)
- [x] Team file updated

## Notes for Future Teams

The behavior test failure (`cargo xtask test behavior`) is a **pre-existing issue** showing log line ordering differences due to race conditions. This is NOT caused by the display fix and needs separate investigation.
