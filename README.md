# LevitateOS

**A Rust-native Linux distribution builder**

Build minimal, bootable Linux systems from source with type-safe, fast Rust tooling.

---

## What is LevitateOS?

LevitateOS is a build system that creates minimal Linux distributions from:

- **Linux kernel** (6.19-rc5)
- **musl libc** (static linking)
- **BusyBox** (shell + 300 utilities)
- **OpenRC** (init system)

Think of it as a Rust alternative to Alpine Linux's shell-script toolchain.

---

## Quick Start

```bash
# Build everything
cargo run -- build all

# Boot in QEMU with serial console
cargo run -- run --term
```

You'll see Linux boot with OpenRC services starting, then get a shell prompt:

```
Linux version 6.19.0-rc5-levitate ...
OpenRC 0.54 is starting up Linux ...
 * Mounting /proc ... [ ok ]
 * Mounting /sys ... [ ok ]
levitate#
```

---

## Commands

```bash
# Building
cargo run -- build all           # Build everything
cargo run -- build linux         # Linux kernel only
cargo run -- build busybox       # BusyBox only
cargo run -- build openrc        # OpenRC only

# Running
cargo run -- run --term          # Serial console
cargo run -- run                 # GUI mode
cargo run -- run --gdb           # With GDB server
cargo run -- run --minimal       # BusyBox init (no OpenRC)

# Testing
cargo run -- test                # Run all tests
cargo run -- test behavior       # Boot output test

# Utilities
cargo run -- clean               # Clean build artifacts
cargo run -- check               # Preflight checks
```

---

## Project Structure

```
levitate/
├── src/                    # Build system source (Rust)
│   ├── builder/            # Linux/BusyBox/OpenRC builders
│   ├── qemu/               # QEMU runner
│   └── tests/              # Test modules
├── linux/                  # Kernel submodule
├── toolchain/              # Build outputs (gitignored)
├── tests/                  # Golden files
└── docs/                   # Documentation
```

---

## Requirements

- **Rust** (stable)
- **QEMU** (`qemu-system-x86_64`)
- **musl-gcc** (for static linking)
- **meson + ninja** (for OpenRC)

```bash
# Fedora
sudo dnf install qemu musl-gcc meson ninja-build

# Ubuntu/Debian
sudo apt install qemu-system-x86 musl-tools meson ninja-build
```

---

## Architecture Support

| Arch | Status |
|------|--------|
| x86_64 | Primary |
| aarch64 | Experimental |

---

## Development

This project was developed with AI assistance. Each development session is logged in `.teams/TEAM_XXX_*.md` files.

**476+ team sessions** have contributed to this codebase.

---

## License

MIT
