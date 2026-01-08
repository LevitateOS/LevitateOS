---
description: how to verify GPU display state when QEMU window is blank
---

# Verifying GPU State via Browser VNC

When the QEMU graphical window reports "Display output is not active", use the browser-based VNC viewer to verify.

## Recommended: Using `cargo xtask run-vnc`

// turbo-all

```bash
# 1. Start QEMU with VNC
cargo xtask run-vnc
```

```
# 2. Open browser to http://localhost:6080/vnc.html
# 3. Click "Connect" button
# 4. Observe the display:
#    - "Display output is not active" = GPU is BROKEN ❌
#    - Terminal text visible = GPU is WORKING ✅
```

## Alternative: GPU Dump via QMP

If QEMU is already running with QMP enabled (default in `run.sh`):

```bash
# Capture framebuffer to PNG
cargo xtask gpu-dump screenshot.png
```

### Analyze Results
- **If `screenshot.png` has contents (text, terminal):** Kernel-side driver is WORKING.
- **If `screenshot.png` is all black/red:** GPU scanout is not configured correctly.

