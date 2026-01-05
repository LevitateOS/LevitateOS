# Phase 8b Postmortem: UX Failure

**Date:** 2026-01-05  
**Teams:** TEAM_080 (Review), TEAM_081 (Implementation), TEAM_082 (Bugfix), TEAM_083 (Investigation)

**Status:** PARTIAL RESOLUTION — Serial console works, GPU display has architectural issues

---

## What Was Supposed to Happen

The EPIC promised:
1. Boot log scrolls on GPU like classic Unix
2. After boot, shell prompt `# ` appears
3. User can type commands and see output
4. Clear visual distinction between boot and interactive mode

---

## What Actually Happens

1. Boot messages scroll (✅ works via dual console)
2. Boot HANGS or appears to hang - no clear indication it's done
3. User has NO IDEA when/if they can type
4. No visual feedback when typing
5. The system appears completely unresponsive

---

## Root Causes

### 1. Boot Hijack Still Active
The kernel still runs `run_from_initramfs("hello", ...)` which was TEAM_073's demo code.
This means boot NEVER reaches the interactive loop - it jumps straight to userspace.

### 2. Shell Doesn't Echo Input
The shell calls `sys_read()` but there's no local echo. Users type but see nothing.

### 3. No Clear "Boot Complete" Indicator
Boot messages just stop. No banner, no prompt, no indication the system is ready.

### 4. Interrupts Not Enabled Before Userspace
The userspace process runs but keyboard interrupts may not be reaching it.

---

## Technical Debt Created

| Issue | Location | Severity |
|-------|----------|----------|
| Boot hijack | `kernel/src/main.rs:597` | CRITICAL |
| No input echo | `userspace/hello/src/main.rs` | HIGH |
| No boot complete banner | `kernel/src/main.rs` | MEDIUM |
| Unclear boot stages | Throughout | MEDIUM |

---

## What Would Actually Fix This

### Immediate (Must Do) — DONE by TEAM_083

1. ✅ **Remove the boot hijack** - Let boot complete to the interactive loop
2. ✅ **Echo typed characters** - Users see typing in serial console
3. ✅ **Clear "READY" indicator** - "[SUCCESS] LevitateOS System Ready"

### Short-Term — BLOCKED by GPU Architecture

1. ❌ **Show shell prompt on GPU** - Blocked by Display deadlock (see below)
2. ✅ **Process keyboard in main loop** - Working via `input::read_char()`
3. ✅ **Enable interrupts BEFORE userspace** - Done

### Architectural Issue: GPU Display Deadlock — RESOLVED by TEAM_086/089

**Original Root Cause (TEAM_086):** `Display::draw_iter()` in `kernel/src/gpu.rs` locked `GPU` internally. Fixed by refactoring `Display` to accept `&mut GpuState`.

**TEAM_089 Investigation Findings:**
The VirtIO GPU **driver is correctly implemented**:
- `setup_framebuffer()` properly calls `set_scanout()` in virtio-drivers v0.12.0
- Red flash during boot proves: init ✅, scanout ✅, flush ✅
- No kernel code resets scanout after init

**Actual Issue:** QEMU display surface goes inactive shortly after boot.
- Hypothesis: QEMU `virtio-gpu-device` requires continuous flush to stay active
- Workaround: Serial console works perfectly

**Recommended Fixes:**
1. Add timer-based GPU flush (~10Hz)
2. Try `virtio-vga` device type in run.sh
3. Serial console is fully functional for development


---

## Lessons for Future Teams

1. **Test the actual user experience** - Don't just verify code paths work
2. **Remove demo/debug code** - TEAM_073's hijack should have been removed
3. **Visual feedback is critical** - Users can't debug what they can't see
4. **Boot complete != system usable** - There's a gap between these states

---

## Current State Summary

```
WHAT WORKS:
- Kernel boots ✅
- Dual console (UART + GPU) ✅
- Userspace binary loads ✅
- Shell code exists ✅

WHAT DOESN'T WORK:
- User can interact with the system ❌
- Visual feedback ❌
- Knowing when boot is done ❌
- Typing and seeing characters ❌

VERDICT: System boots but is UNUSABLE for interactive use.
```

---

## Technical Resolutions (TEAM_083)

| Issue | Resolution | Verification |
|-------|------------|--------------|
| Boot hijack | Removed `task::process::run_from_initramfs` | System reaches interactive loop ✅ |
| No input echo | Added `print!` calls in kernel/userspace loops | Characters displayed as typed ✅ |
| GPU Deadlocks | Converted to `IrqSafeLock`, used `serial_println!` | No more hangs after `BOOT_REGS` ✅ |
| Timer IRQ | Registered timer handler and enabled IRQ early | Observed 100Hz heartbeat ✅ |

---

## Technical Lessons

1. **Deadlock Risk**: The dual-console `println!` is convenient but dangerous. Use `serial_println!` in drivers and IRQ handlers.
2. **Heartbeat Diagnostics**: When the system hangs, use periodic `serial_println!` heartbeats in IRQ handlers to isolate if interrupts are working.
3. **Hardware Status**: Don't guess. Read the UART FR register to see if bits like `RXFE` are toggling.

---

## Handoff for Next Team

**DO NOT** try to add more features. First fix the fundamentals:

1. Comment out line ~597 in `kernel/src/main.rs`:
   ```rust
   // task::process::run_from_initramfs("hello", &archive);
   ```

2. Ensure the main loop processes keyboard input and sends to GPU terminal

3. Add clear "System Ready" message after boot

4. THEN work on making the shell interactive
