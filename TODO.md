# LevitateOS TODO List

Global tracking of incomplete tasks per Rule 11.

---

## Critical

### VirtIO GPU Display Not Active — TEAM_087 / TEAM_089
**File:** `kernel/src/gpu.rs`, VirtIO GPU driver  
**Symptom:** QEMU window shows "Display output is not active"  

**TEAM_089 Findings:**
- **Driver is CORRECT** — red flash proves init, scanout, and flush all work
- `setup_framebuffer()` properly calls `set_scanout()` in virtio-drivers v0.12.0
- Nothing in kernel code resets scanout after init

**Root Cause:** QEMU display surface timing — display goes inactive shortly after boot

**What Works:**
- Serial console (type in terminal where QEMU runs)
- Kernel boots fully, shows "[SUCCESS] LevitateOS System Ready"
- VirtIO keyboard input (echoes to serial)

**Fixes to Try (in order):**
1. **Add timer-based GPU flush** — flush at ~10Hz from timer interrupt
2. **Try `virtio-vga`** — change run.sh: `-device virtio-vga,xres=1280,yres=800`
3. **Comment out `console_gpu::clear()`** — line 544 in main.rs may trigger issue

**Workaround:** Use serial console - all kernel functionality works there.

---


## High Priority

*None*

---

## Medium Priority

### Boot Hijack Code Still Present — TEAM_081
**File:** `kernel/src/main.rs:612`  
**Description:** TEAM_073's demo code `run_from_initramfs("hello")` is commented out but not removed. Should be cleaned up once userspace shell is working properly.

---

## Low Priority

### Stale Golden Tests — TEAM_081
**File:** `tests/golden_boot.txt`  
**Description:** Behavior tests reflect old boot sequence. Need update after Phase 8b is complete.

---

## Completed

- [x] TEAM_082: Linker script conflict for userspace builds
- [x] TEAM_083: UART debug spam from stale binary
- [x] TEAM_083: Timer "T" debug output flooding console
- [x] TEAM_083: GPU reference compilation error
- [x] TEAM_086: GPU Display Deadlock API — Refactored Display to accept `&mut GpuState`
- [x] TEAM_087: Enabled dual console callback (but GPU display still not active)
- [x] TEAM_087: Fixed per-println flush causing kernel hang
