# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is LevitateOS?

LevitateOS is a **Rust-native Linux distribution builder**. It creates minimal, bootable Linux systems from source:

- **Linux kernel** (6.19-rc5 from submodule)
- **BusyBox** (shell + 300 utilities, static musl)
- **OpenRC** (init system, static musl)
- **initramfs** (CPIO archive)

Think of it as a Rust alternative to Alpine Linux's shell-script build toolchain.

## Build & Development Commands

### Building

```bash
# Build everything
cargo run -- build all

# Build specific components
cargo run -- build linux          # Linux kernel
cargo run -- build busybox        # BusyBox (shell + utilities)
cargo run -- build openrc         # OpenRC (init system)
cargo run -- build initramfs      # BusyBox-only initramfs
cargo run -- build openrc-initramfs  # OpenRC + BusyBox initramfs
```

### Running

```bash
# Boot in QEMU
cargo run -- run --term          # Serial console (most common)
cargo run -- run                 # GUI mode
cargo run -- run --gdb           # With GDB server on port 1234
cargo run -- run --vnc           # VNC display

# Minimal mode (BusyBox init instead of OpenRC)
cargo run -- run --term --minimal

# Architecture selection
cargo run -- --arch x86_64 run   # Default
cargo run -- --arch aarch64 run
```

### Testing

```bash
# Run all tests
cargo run -- test

# Specific test suites
cargo run -- test unit           # Host-side unit tests
cargo run -- test behavior       # Boot output vs golden file
cargo run -- test serial         # Serial input tests
cargo run -- test screenshot     # Screenshot tests

# Update golden files when behavior changes
cargo run -- test behavior --update
```

### VM Interaction

```bash
cargo run -- vm start            # Start persistent VM session
cargo run -- vm send "ls"        # Send keystrokes to VM
cargo run -- vm screenshot       # Take screenshot
cargo run -- vm stop             # Stop session
```

### Utilities

```bash
cargo run -- check               # Preflight checks
cargo run -- clean               # Clean build artifacts
cargo run -- kill                # Kill running QEMU
```

## Project Structure

```
levitate/
├── src/                    # Main binary source
│   ├── main.rs             # CLI entry point
│   ├── builder/            # Core build system
│   │   ├── linux.rs        # Linux kernel builder
│   │   ├── busybox.rs      # BusyBox builder
│   │   ├── openrc.rs       # OpenRC builder
│   │   └── initramfs/      # Initramfs CPIO builder
│   ├── qemu/               # QEMU command builder
│   ├── run.rs              # Run commands
│   ├── vm/                 # VM interaction
│   ├── support/            # Utilities (preflight, clean)
│   ├── disk/               # Disk image management
│   └── tests/              # Test modules
├── linux/                  # Linux kernel (git submodule)
├── toolchain/              # Build outputs (gitignored)
│   ├── busybox-out/        # Built BusyBox binaries
│   └── openrc-out/         # Built OpenRC binaries
├── tests/                  # Golden files
├── docs/                   # Documentation
├── .teams/                 # Team logs (development history)
├── Cargo.toml              # Package manifest
└── xtask.toml              # Test configuration
```

## Development Guidelines

### Code Quality Rules

1. **Quality Over Speed**: Take the correct architectural path, never shortcuts
2. **No Dead Code**: Remove unused functions, modules, commented-out code
3. **Breaking Changes > Compatibility Hacks**: Fix call sites, don't add shims
4. **Modular Structure**: Files < 1000 lines preferred, < 500 ideal

### Testing Philosophy

- **Golden File Testing**: Compare boot output against known-good reference
- **Update when intentional**: `cargo run -- test behavior --update`
- **All tests must pass**: Never dismiss failures without investigation

### Team Workflow

Every AI conversation = one team. Track work in `.teams/TEAM_XXX_*.md`:

1. Check `.teams/` for highest existing number
2. Create `.teams/TEAM_XXX_summary.md`
3. Log progress, decisions, gotchas
4. Update handoff notes before finishing

## Requirements

- **Rust** (stable toolchain)
- **QEMU** (`qemu-system-x86_64`, `qemu-system-aarch64`)
- **musl-gcc** (for static linking)
- **meson + ninja** (for OpenRC)

```bash
# Fedora
sudo dnf install qemu musl-gcc meson ninja-build

# Ubuntu/Debian
sudo apt install qemu-system-x86 musl-tools meson ninja-build
```

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entry point |
| `src/builder/initramfs/mod.rs` | OpenRC initramfs builder (core product) |
| `src/qemu/builder.rs` | QEMU command construction |
| `tests/golden_boot_linux_openrc.txt` | Expected boot output |
| `xtask.toml` | Test configuration |
