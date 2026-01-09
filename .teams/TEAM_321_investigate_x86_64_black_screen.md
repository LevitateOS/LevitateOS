# TEAM_321: Investigate x86_64 Black Screen

## Task

Investigate and fix the x86_64 VM black screen on startup.

## Previous Work (from team logs)

- TEAM_318: Added timer handler registration for x86_64 using `apic::register_handler(32, &TIMER_HANDLER)`
- TEAM_319: Added 8259 PIC initialization (remap IRQs, unmask timer/serial)
- TEAM_320: Confirmed black screen persists. Framebuffer has content but VNC shows black.

## Current Hypothesis

The timer interrupt is either:
1. Not firing at all (PIC configuration issue)
2. Firing but not calling the GPU flush properly
3. GPU scanout not configured correctly

## Investigation Status

- [ ] Verify current display state via VNC
- [ ] Check PIC initialization code
- [ ] Check timer handler registration
- [ ] Add debug logging to confirm timer interrupts fire
- [ ] Trace GPU flush path

## Files to Investigate

- `crates/hal/src/x86_64/interrupts/pic.rs` - PIC driver
- `kernel/src/init.rs` - Timer handler registration
- `kernel/src/gpu.rs` - GPU flush logic
