---
description: how to view and interact with the LevitateOS GUI via VNC
---

# VNC Graphical Interface Interaction

LevitateOS runs a VNC server by default when started via `xtask`. You can use the internal browser to view the screen.

## 1. Launching the OS
Ensure the OS is running in a background terminal or another session:
```bash
cargo run --package xtask -- run term --arch x86_64 --iso
```

## 2. Connecting via Browser
Use the `browser_subagent` tool with this instruction:
```text
Navigate to http://localhost:6080/vnc.html, click the "Connect" button, and verify the display output.
```

## 3. Manual Troubleshooting
- **URL**: `http://localhost:6080/vnc.html`
- **Port**: 6080 (WebSocket proxy for VNC)
- **Wait Time**: Ensure the serial output says `[SUCCESS] LevitateOS System Ready.` before connecting.

## 4. Key Interactions
- The terminal in the VNC window supports keyboard input.
- You can test GUI components or the terminal emulator rendering here.

---
**TEAM_296 Note**: This is the preferred way to verify graphical output and terminal rendering on the VirtIO GPU.
