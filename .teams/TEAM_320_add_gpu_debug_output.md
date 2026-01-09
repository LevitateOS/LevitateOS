# TEAM_320: GPU Display Verification via VNC + Puppeteer

## Task
Add debugging to detect if QEMU GPU shows a black screen, visible in the host shell.

## Problem Found
**The kernel's framebuffer check is a FALSE POSITIVE!**

The framebuffer reports content (400 non-black pixels), but Puppeteer screenshot of the VNC display shows **COMPLETELY BLACK SCREEN**.

This means:
- Framebuffer is being written to ✅
- GPU flush is being called ✅
- **But actual display output is NOT working** ❌

## Solution: Automated VNC Verification

Created a Puppeteer-based verification system that actually looks at the VNC display.

### 1. Verification Script (`scripts/verify-display.js`)
Node.js script that:
- Connects to noVNC
- Takes screenshot of actual display
- Analyzes pixels to detect black screen
- Returns exit code: 0=content, 1=black, 2=error

### 2. New xtask Command (`xtask/src/run.rs`)
Added `verify-gpu` command:
- Starts QEMU with VNC
- Runs Puppeteer verification
- Reports actual display status

### 3. Kernel Debug (kept for serial logging)
- `debug_display_status()` function
- Shows framebuffer status (note: can be misleading)

## Usage
```bash
# Run automated GPU verification
cargo xtask run verify-gpu

# Manual VNC inspection
./run.sh vnc
# Then use Puppeteer MCP to check
```

## Files Changed
- `scripts/verify-display.js` - NEW: Puppeteer verification script
- `xtask/src/run.rs` - Added `VerifyGpu` command and `verify_gpu()` function
- `xtask/src/main.rs` - Added handler for VerifyGpu
- `kernel/src/gpu.rs` - Added `debug_display_status()` (but note: can be false positive)
- `kernel/src/init.rs` - Call debug function after terminal init

## Current Status
**x86_64 GPU Display: BLACK SCREEN ❌**

The VNC screenshot confirms the display is completely black despite:
- GPU initializing successfully
- Framebuffer being written to
- Flush being called

Root cause needs further investigation (timer IRQ not firing? virtio-gpu scanout issue?)

## Handoff
- [x] Project builds cleanly
- [x] Automated VNC verification added
- [ ] x86_64 GPU display still broken - needs fix
