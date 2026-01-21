# TEAM_081: Replace ldd with readelf for Cross-Compilation Support

## Problem

The previous implementation used `ldd` to find library dependencies:

```rust
let ldd_output = Command::new("ldd").arg(&bin_path).output();
```

**Why this is broken:**
- `ldd` **executes the binary** using the host's dynamic linker
- If host is musl (Alpine) and target is glibc (Rocky) → wrong libraries
- If host has different libc version → wrong libraries
- Cross-compiling for ARM/RISC-V on x86 → completely broken
- Builds silently produce broken initramfs

## Solution

Use `readelf -d` which reads ELF headers directly without executing:

```rust
let output = Command::new("readelf")
    .args(["-d", binary_path.to_str().unwrap()])
    .output()?;
```

**Why this works:**
- Reads ELF NEEDED entries directly from file
- No execution, no dynamic linker
- Works for any architecture on any host
- Enables cross-compilation

## Changes

### src/initramfs/binary.rs
- Replaced `parse_ldd_output()` with `parse_readelf_output()`
- Added `get_library_dependencies()` using readelf
- Added `get_all_dependencies()` for recursive transitive deps
- Updated `copy_binary_with_libs()` and `copy_bash()`
- Added unit tests for readelf parsing

### src/rootfs/binary.rs
- Same changes as initramfs/binary.rs
- Maintains strict rootfs isolation (no host fallback)

### src/initramfs/dbus.rs
- Updated to use new `get_all_dependencies()` function

### src/initramfs/rootfs.rs
- Changed host tool check from `ldd` to `readelf`

## Before vs After

**Before (broken for cross-compilation):**
```rust
// Executes binary with HOST dynamic linker
let ldd_output = Command::new("ldd").arg(&bin_path).output();
let libs = parse_ldd_output(&output);  // Host paths, host libc
```

**After (works everywhere):**
```rust
// Reads ELF headers directly - no execution
let libs = get_all_dependencies(&ctx.rootfs, &bin_path)?;
// Returns: ["libc.so.6", "libtinfo.so.6", ...]
// We then search rootfs for these by name
```

## Benefits

1. **Cross-compilation ready** - Can build ARM/RISC-V on x86
2. **Reproducible builds** - Same output regardless of host
3. **Container-safe** - Works in Alpine, Debian, any container
4. **Transitive dependencies** - Recursively finds all deps

## Verification

```bash
cargo build  # ✓ Compiles
cargo test   # ✓ Unit tests pass (readelf parsing)
```

## Host Requirements

Now requires `readelf` (part of binutils) instead of `ldd`:
- Fedora: `dnf install binutils`
- Debian: `apt install binutils`
- Alpine: `apk add binutils`
