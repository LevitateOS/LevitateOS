# TEAM_444: Dash Shell Support → musl Migration

## Objective

~~Add dash (Debian Almquist Shell) as a simpler shell alternative to brush.~~

**REVISED**: Replace c-gull with musl libc entirely. This enables both:
1. Simpler Rust builds (no -Z build-std, no custom RUSTFLAGS)
2. C program support (dash, and any other C program)

## Progress Log

### Session 1 (2026-01-12)

**Research Phase:**
- Researched dash requirements: autotools build, ~120KB binary, POSIX-only
- Discovered key difference: dash is C code, current toolchain only supports Rust
- c-gull/c-ward provides libc for Rust programs via Eyra/Mustang pattern
- c-gull does NOT provide C headers - it's a Rust libc, not a C libc

**Key Decision:**
User asked "how much easier would our lives be if we replaced c-gull with musl?"
Answer: MUCH easier. Single libc for everything.

**Implementation Phase:**
1. ✅ Proof of concept - built coreutils with `--target x86_64-unknown-linux-musl`
2. ✅ Updated `apps.rs` - removed all complex RUSTFLAGS, -Z build-std, nightly pinning
3. ✅ Created `c_apps.rs` - C program support via musl-gcc
4. ✅ Simplified `sysroot.rs` - now just ensures musl target is installed
5. ✅ Updated `commands.rs` - added Dash command
6. ✅ Cleaned up - removed `toolchain/libc-levitateos/`, old sysroot
7. ✅ Updated CLAUDE.md documentation

**Verification:**
- `cargo xtask build coreutils` works with musl target
- Binary is statically linked (`static-pie linked`)
- Build is much simpler (no special flags)

## Key Decisions

1. **Replace c-gull with musl entirely** - One libc for everything
2. **Shell progression** - Built-in → dash → brush (not all at once)
3. **Removed brush from default builds** - Too complex, verify musl with simpler shells first
4. **Keep nightly Rust** - Other parts of the project depend on nightly (kernel)
5. **System musl packages** - Use distro packages, document build-from-source
6. **musl-gcc for C programs** - Standard, widely available
7. **aarch64**: Document both musl-gcc cross and clang paths

## Gotchas Discovered

1. c-ward/c-gull is specifically for Rust programs using Eyra/Mustang pattern
2. It does NOT provide C headers or a traditional libc.a for C compilers
3. musl target is a standard Rust target - no special setup needed
4. musl-gcc needs to be installed separately for C programs

## Files Changed

### Modified
- `xtask/src/build/apps.rs` - Simplified for musl target
- `xtask/src/build/sysroot.rs` - Now just installs musl target
- `xtask/src/build/commands.rs` - Added Dash command, updated build_all
- `xtask/src/build/mod.rs` - Added c_apps module
- `xtask/src/main.rs` - Added Dash command handler
- `toolchain/.gitignore` - Added dash entries
- `CLAUDE.md` - Updated libc documentation

### Added
- `xtask/src/build/c_apps.rs` - C program build support

### Removed
- `toolchain/libc-levitateos/` - Old c-gull wrapper (entire directory)
- `toolchain/sysroot/` - Old c-gull output (gitignored, cleaned)

## Verification Complete

**musl migration verified working!** Shell now spawns correctly.

### Session 2 (2026-01-12)

**Dash working!**
1. ✅ Built dash with musl-gcc (713KB static binary)
2. ✅ Added syscall 2 (open) - legacy open() maps to openat(AT_FDCWD, ...)
3. ✅ Fixed foreground process - init now calls set_foreground(shell_pid)
4. ✅ Dash displays prompt and waits for input

**Syscalls added:**
- `SyscallNumber::Open = 2` in x86_64 arch
- Handler maps to `sys_openat(-100, pathname, flags, mode)` (AT_FDCWD)

**Foreground fix:**
- Init calls `set_foreground(shell_pid)` after spawning dash
- Without this, dash thinks it's in background and sends SIGTTIN to itself

### Session 3 (2026-01-12)

**Fixed keyboard input for dash!**

**Root Cause Analysis:**
The serial input wasn't reaching dash. Investigation revealed:
1. Serial port polling was working correctly (verified via debug output)
2. Line status register showed 0x60 (transmitter ready, no data ready)
3. Issue was with QEMU configuration for `--term` mode

**The Problem:**
- QEMU's `-nographic` option was conflicting with VirtIO devices
- When used with `-vga none` and VirtIO GPU, input handling was inconsistent
- The monitor mux (`mux=on`) in `-serial mon:stdio` routes input to monitor by default

**The Fix:**
1. Changed nographic mode to use explicit options: `-display none -serial stdio`
2. Skip VirtIO keyboard device in nographic mode (use serial for input instead)
3. Removed redundant `-serial mon:stdio` that was creating conflicts

**Key Files Changed:**
- `xtask/src/qemu/builder.rs` - Fixed QEMU configuration for nographic mode

**Verification:**
- Serial input now works: `yes "test" | qemu ...` shows bytes being received
- Line status shows 0x61 (data ready) when input is available
- Dash receives keyboard input correctly

## Remaining Work

- [x] Fix keyboard input for dash
- [ ] Test dash commands interactively (requires manual testing by user)
- [ ] Add aarch64 musl cross-compilation support

## Handoff Notes

**What Changed:**
- Replaced c-gull with musl libc
- Rust programs now use `--target x86_64-unknown-linux-musl` (standard target)
- C programs can be built with musl-gcc
- Build system is MUCH simpler

**Before (c-gull):**
```bash
cargo +nightly-2025-04-28 build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target x86_64-unknown-linux-gnu
# + complex RUSTFLAGS with link args
```

**After (musl):**
```bash
cargo build --target x86_64-unknown-linux-musl
# That's it!
```

**To test dash:**
```bash
# Install musl-gcc first
sudo dnf install musl-gcc musl-devel autoconf automake  # Fedora
# or
sudo apt install musl-tools musl-dev autoconf automake  # Ubuntu

# Then build
cargo xtask build dash
```

## Plan Documents

- `docs/planning/musl-migration/` - Full migration plan
- `docs/planning/dash-shell-support/` - Original dash plan (superseded)
