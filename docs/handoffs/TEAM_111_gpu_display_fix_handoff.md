# TEAM_112+ Handoff: Fix GPU Display Until Terminal is Visible

**Priority:** HIGH  
**End Goal:** QEMU screen shows a terminal, not "Display output is not active"

---

## Quick Start

```bash
# 1. Start QEMU with VNC (browser-based display)
cargo xtask run-vnc

# 2. Open browser to verify display
# URL: http://localhost:6080/vnc.html
# Click "Connect"

# 3. Check what you see:
#    - "Display output is not active" = STILL BROKEN, keep fixing
#    - Terminal text visible = SUCCESS!
```

---

## Current State (TEAM_111)

| Component | Status |
|-----------|--------|
| Serial output | ✅ Working - boots to `# ` prompt |
| GPU init message | ⚠️ FALSE POSITIVE - says "success" |
| QEMU display | ❌ BROKEN - shows nothing |

**The Problem:** GPU driver initializes but never displays anything because VirtIO GPU scanout commands are not working.

---

## What You Need to Fix

The VirtIO GPU display requires this command sequence:

```
1. GET_DISPLAY_INFO      ← Working (gets 1280x800)
2. RESOURCE_CREATE_2D    ← Unknown
3. RESOURCE_ATTACH_BACKING ← Unknown  
4. SET_SCANOUT           ← Likely BROKEN (display inactive = scanout not set)
5. TRANSFER_TO_HOST_2D   ← Unknown
6. RESOURCE_FLUSH        ← Unknown
```

**Key Files:**
- `levitate-drivers-gpu/src/driver.rs` - GPU state machine
- `levitate-drivers-gpu/src/device.rs` - Command building
- `levitate-virtio/src/queue.rs` - VirtQueue (may have layout issues)
- `kernel/src/gpu.rs` - Kernel-side GPU wrapper

---

## Debug Loop

```
┌─────────────────────────────────────┐
│   cargo xtask run-vnc               │
└─────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────┐
│   Open browser: localhost:6080      │
│   Click "Connect"                   │
└─────────────────────────────────────┘
                  │
                  ▼
        ┌─────────────────┐
        │ Terminal shows? │
        └─────────────────┘
               │
      ┌────────┴────────┐
      │                 │
     YES               NO
      │                 │
      ▼                 ▼
   SUCCESS!      Check serial logs
                 for GPU errors,
                 fix code, repeat
```

---

## Known Issues to Check

### 1. VirtQueue Layout (TEAM_109)
Previous teams found VirtQueue struct may have layout issues. Check:
- `used_event` / `avail_event` fields (should only exist if negotiated)
- Padding alignment for used ring

### 2. Missing Flush After Draw
The kernel might not be calling `flush()` after drawing. Check:
- `kernel/src/terminal.rs` - does it call `gpu.flush()`?
- `kernel/src/gpu.rs` - does `flush()` actually send RESOURCE_FLUSH?

### 3. SET_SCANOUT Command
This is the most likely culprit. "Display output is not active" means QEMU never received a valid scanout configuration.

---

## Verification Commands

```bash
# Check GPU-related code
grep -r "SET_SCANOUT\|set_scanout" levitate-drivers-gpu/

# Check if flush is called
grep -r "\.flush()" kernel/src/

# View VirtQueue layout
view_file levitate-virtio/src/queue.rs
```

---

## Success Criteria

✅ `cargo xtask run-vnc` starts QEMU  
✅ Browser shows noVNC at `localhost:6080`  
✅ **QEMU display shows terminal text** (not "Display output is not active")  
✅ `cargo xtask test behavior` passes  

---

## References

| Document | Purpose |
|----------|---------|
| `docs/GOTCHAS.md` #16 | GPU crate history |
| `docs/VIRTIO_IMPLEMENTATION.md` | VirtIO queue details |
| `.teams/TEAM_109_fix_gpu_driver_no_fallback.md` | Previous fix attempts |
| `.teams/TEAM_110_investigate_gpu_fallback_and_cleanup.md` | VirtQueue fixes |
| `.teams/TEAM_111_investigate_desired_behaviors_and_qemu_vnc.md` | Current investigation |
