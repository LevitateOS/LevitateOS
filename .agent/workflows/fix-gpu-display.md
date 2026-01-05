---
description: fix GPU display until terminal is visible on QEMU screen
---

# Fix GPU Display (VirtIO GPU Scanout)

// turbo-all

## Goal

Make the QEMU graphical window show a terminal instead of "Display output is not active".

---

## 0. Verify Current State

```bash
# Start QEMU with VNC for browser viewing
cargo xtask run-vnc
```

Then:
1. Open browser to `http://localhost:6080/vnc.html`
2. Click "Connect"
3. If you see **"Display output is not active"** → GPU is BROKEN, continue to fix
4. If you see **terminal text** → GPU is WORKING, you're done!

---

## 1. Understand the Problem

The VirtIO GPU requires these commands to display:

| Command | Purpose | Current Status |
|---------|---------|----------------|
| GET_DISPLAY_INFO | Get resolution | ✅ Working |
| RESOURCE_CREATE_2D | Create framebuffer | ⚠️ Check |
| RESOURCE_ATTACH_BACKING | Attach memory | ⚠️ Check |
| SET_SCANOUT | Connect resource to display | ❌ Likely broken |
| TRANSFER_TO_HOST_2D | Copy pixels to host | ⚠️ Check |
| RESOURCE_FLUSH | Refresh display | ⚠️ Check |

**"Display output is not active"** means SET_SCANOUT was never called successfully.

---

## 2. Key Files to Investigate

```bash
# GPU driver state machine
view_file levitate-drivers-gpu/src/driver.rs

# Command building
view_file levitate-drivers-gpu/src/device.rs

# VirtQueue (may have layout bugs)
view_file levitate-virtio/src/queue.rs

# Kernel GPU wrapper
view_file kernel/src/gpu.rs
```

---

## 3. Debug Approach

### 3.1 Add Debug Output

Add serial debug prints to trace which commands are being sent:

```rust
// In levitate-drivers-gpu/src/device.rs
levitate_hal::serial_println!("[GPU-DBG] Sending SET_SCANOUT...");
```

### 3.2 Check VirtQueue Response

After each command, verify the device responded:

```rust
levitate_hal::serial_println!("[GPU-DBG] Response status: {:?}", response);
```

### 3.3 Common Issues

1. **VirtQueue layout mismatch** - Check padding and alignment
2. **Command timeout** - Device not seeing descriptors
3. **Missing flush()** - Kernel not calling flush after draw
4. **Resource ID mismatch** - Using wrong resource_id in commands

---

## 4. Test Loop

Repeat until success:

```
make change → cargo xtask run-vnc → check browser → still broken? → repeat
```

---

## 5. Success Criteria

- [ ] `cargo xtask run-vnc` starts successfully
- [ ] Browser at `localhost:6080/vnc.html` shows connection
- [ ] **QEMU display shows terminal text** (not "Display output is not active")
- [ ] Serial output still shows boot messages and shell prompt

---

## 6. References

- `docs/handoffs/TEAM_111_gpu_display_fix_handoff.md` - Full context
- `docs/GOTCHAS.md` #16 - GPU crate history  
- `.teams/TEAM_109_fix_gpu_driver_no_fallback.md` - Previous attempts
- VirtIO GPU spec: https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html#x1-3400008
