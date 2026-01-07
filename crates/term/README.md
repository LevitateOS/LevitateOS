# los_term

Platform-agnostic ANSI terminal emulator for LevitateOS.

## Overview

This crate provides a standalone terminal emulator that handles text rendering, cursor management, and ANSI escape sequences. It is designed to be hardware-agnostic, rendering to any `embedded-graphics` `DrawTarget`.

## Features

- **ANSI/VT100 Support**: Full support for common escape sequences (colors, cursor movement, clearing).
- **Profont Support**: High-readability monospace font for system consoles.
- **Hardware Agnostic**: Works on any `DrawTarget<Color = Rgb888>`.
- **Scrolling & Wrapping**: Built-in logic for line management.

## Integration

The terminal is used in the kernel to provide:
1. **GPU Console**: Direct rendering to the VirtIO GPU framebuffer.
2. **Dual Console**: Mirroring of UART output to the screen.
3. **Userspace TTY**: Providing a display target for shell processes.

## Usage

```rust
use los_term::Terminal;

let mut term = Terminal::new(width, height);
term.write_str(&mut display, "Hello World\n");
```

## Traceability

- **TEAM_058**: Initial GPU terminal.
- **TEAM_081**: Global terminal for dual-output.
- **TEAM_115**: Fixed userspace rendering.
