# TEAM_127: Visual Install Testing via Puppeteer + noVNC

## Objective
Visually test LevitateOS installation: see what users see, type what they type, verify via screenshots.

## Status: COMPLETE

## Architecture
```
Puppeteer MCP ──► noVNC (browser) ──► websockify ──► QEMU VNC
     │                                                  │
     └─ puppeteer_fill: send keystrokes ────────────────┘
     └─ puppeteer_screenshot: capture screen
```

## Setup

### Prerequisites
```bash
# Clone noVNC (one-time)
git clone --depth 1 https://github.com/novnc/noVNC.git /tmp/novnc

# websockify (pip install websockify)
```

### Running
```bash
# Terminal 1: QEMU with VNC
qemu-system-x86_64 -enable-kvm -m 4G -cpu host \
  -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/ovmf/OVMF_CODE.fd \
  -cdrom output/levitateos.iso -vnc :0 -device virtio-vga -boot d -display none

# Terminal 2: websockify bridge
websockify 6080 localhost:5900 --web /tmp/novnc
```

## Puppeteer MCP Usage

### Connect to VM
```
puppeteer_navigate → http://localhost:6080/vnc.html?autoconnect=true
```

### Send Keystrokes
```
puppeteer_fill selector="#noVNC_keyboardinput" value="echo hello\n"
```
- Include `\n` at end to press Enter
- Text is typed character by character

### Capture Screenshot
```
puppeteer_screenshot name="test" width=1024 height=768
```

## Verified Working (2026-01-27)
- Boot LevitateOS ISO in QEMU with VNC
- Connect Puppeteer to noVNC
- Type `echo hello world` → Output: `hello world`
- Type `uname -a` → Shows kernel version
- Screenshots capture VM display correctly

## Notes
- The `puppeteer_evaluate` approach for sending keystrokes via JS events is unreliable
- `puppeteer_fill` on `#noVNC_keyboardinput` works reliably (noVNC's hidden keyboard textarea)
- Serial backend still works for CI/CD where visual testing isn't needed
