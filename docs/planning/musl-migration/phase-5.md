# Phase 5: Polish - Migrate from c-gull to musl

## Cleanup Tasks

### Code Removal

- [ ] Delete `toolchain/libc-levitateos/` directory
- [ ] Remove c-gull references from any Cargo.toml
- [ ] Remove `+nightly-2025-04-28` from all build commands
- [ ] Remove `-Z build-std` flags
- [ ] Remove custom RUSTFLAGS
- [ ] Clean up unused helper functions

### Code Quality

- [ ] Run `cargo clippy` on xtask
- [ ] Ensure no `unwrap()` in new code
- [ ] Add doc comments to new public functions
- [ ] Run `cargo fmt` on all changes

### Error Messages

Verify all error paths have actionable messages:
- [ ] musl target not installed → shows rustup command
- [ ] musl-gcc not found → shows package install commands
- [ ] autoreconf not found → shows package install commands
- [ ] Build failure → shows full output

## Documentation Updates

### CLAUDE.md

Update Build Commands section:
```markdown
### Building

# Build everything (kernel + userspace + initramfs + apps)
cargo xtask build all

# Build external apps
cargo xtask build coreutils    # uutils coreutils
cargo xtask build brush        # Rust bash shell
cargo xtask build dash         # POSIX dash shell (requires musl-gcc)

# Note: 'cargo xtask build sysroot' is no longer needed
# Rust uses x86_64-unknown-linux-musl target directly
```

Update prerequisites:
```markdown
## Prerequisites

### Rust Userspace
Rust musl target is auto-installed when needed:
rustup target add x86_64-unknown-linux-musl

### C Userspace (optional, for dash)
# Fedora
sudo dnf install musl-gcc musl-devel autoconf automake

# Ubuntu/Debian
sudo apt install musl-tools musl-dev autoconf automake

# Arch
sudo pacman -S musl autoconf automake
```

### README.md

No changes needed (build instructions live in CLAUDE.md).

### Architecture Documentation

Add note about libc choice:
```markdown
### Userspace libc

LevitateOS userspace uses musl libc for all programs:
- Rust programs: Built with --target x86_64-unknown-linux-musl
- C programs: Built with musl-gcc

This provides:
- Static linking by default
- Small binary sizes
- Complete POSIX compatibility
- Single libc for all languages
```

## Handoff Notes

### For Future Teams

**What Changed:**
- Replaced c-gull (Rust libc) with musl (C libc)
- Simplified build system significantly
- Added C program support (dash)
- Removed nightly Rust requirement for userspace

**What's NOT Changed:**
- Kernel (still uses no_std, no libc)
- Linux ABI compatibility
- Behavior test expectations

**Benefits:**
- Standard Rust musl target (no -Z build-std)
- One libc for Rust and C programs
- Can now easily add any C program
- Possibly stable Rust for userspace

**New Dependencies:**
- System: musl-tools/musl-dev, autoconf, automake
- Rust: x86_64-unknown-linux-musl target (auto-installed)

### Debugging Tips

**Binary not statically linked?**
```bash
file /path/to/binary
# Should say "statically linked"
# If not, check CFLAGS includes -static
```

**musl-gcc not found?**
```bash
which musl-gcc
# If missing, install musl-tools package
```

**Rust build fails with musl?**
```bash
rustup target list --installed | grep musl
# Should show x86_64-unknown-linux-musl
```

### Future Improvements

1. **aarch64 support**: Add cross-compilation for ARM64
2. **More C programs**: sash, busybox, etc.
3. **Dynamic linking**: Eventually add ld.so for .so support
4. **Stable Rust**: Verify stable works, remove any nightly deps

## Migration Summary

### Before (c-gull)
```
Rust apps → cargo +nightly -Z build-std → custom RUSTFLAGS → c-gull sysroot
C apps → NOT POSSIBLE
```

### After (musl)
```
Rust apps → cargo --target x86_64-unknown-linux-musl → system musl
C apps → musl-gcc → system musl
```

### Lines of Code Changed

| File | Change |
|------|--------|
| apps.rs | ~50 lines simplified |
| sysroot.rs | ~80 lines removed/simplified |
| c_apps.rs | ~100 lines added |
| commands.rs | ~10 lines added |
| **Net** | **~20 lines added, much simpler** |

## Team/Plan Locations

- Team file: `.teams/TEAM_444_feature_dash_shell_support.md` (update to reference musl)
- Plan files: `docs/planning/musl-migration/`
- Old plan: `docs/planning/dash-shell-support/` (superseded by this)
