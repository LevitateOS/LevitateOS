# Phase 1: Discovery & Safeguards - Initramfs Builder Refactor

## Refactor Summary

### What
Complete reimagining of the initramfs builder system to be declarative, maintainable, and self-contained.

### Why (Pain Points)

| Pain Point | Current State | Impact |
|------------|---------------|--------|
| **External tool deps** | Shells out to `find \| cpio` | Fragile, platform-dependent, CI issues |
| **Hardcoded everything** | Directory structure, files, symlinks all in Rust | Adding anything requires code changes |
| **Build artifacts at repo root** | `initrd_root/`, `initramfs_*.cpio` pollute workspace | Confusing, gitignore bloat |
| **Scattered logic** | `initramfs.rs`, `busybox.rs`, `apps.rs`, shell scripts | Hard to understand full picture |
| **BusyBox-specific** | Builder tightly coupled to BusyBox model | Can't easily swap init systems |
| **Test scripts embedded** | `include_str!` for test.sh, test-core.sh | Changes require recompile |
| **Legacy cruft** | `scripts/make_initramfs.sh` (broken), `initramfs/lib/` (mystery) | Confusing, maintenance burden |
| **No declarative config** | TEAM_439 planned Rhai but never implemented | Still editing Rust to change contents |

### Prior Art
- **TEAM_439**: Planned Rhai-based scripting (not implemented)
- **TEAM_451**: Migrated to BusyBox (current state)
- **TEAM_466**: Extracted `initramfs.rs` from commands.rs

## Success Criteria

### Before → After

| Metric | Before | After |
|--------|--------|-------|
| External tools needed | `find`, `cpio` | None (pure Rust) |
| Lines of Rust for initramfs | ~800 (initramfs.rs + busybox.rs) | ~300 |
| Config format | Hardcoded Rust | Declarative TOML |
| Add new file | Edit Rust, recompile xtask | Edit TOML |
| Build artifacts location | Repo root | `target/initramfs/` |
| Test script changes | Edit Rust, recompile | Edit file in `initramfs/` |

### Acceptance Criteria
- [ ] `cargo xtask build initramfs` works with new system
- [ ] Same output CPIO contents as current (verified by diffing)
- [ ] No external `cpio` or `find` binaries required
- [ ] All build artifacts in `target/` not repo root
- [ ] Manifest file (`initramfs.toml`) controls contents
- [ ] Test scripts in separate files, not embedded

## Behavioral Contracts (APIs That Must Not Change)

### Public API Surface

| Function | Signature | Must Preserve |
|----------|-----------|---------------|
| `create_busybox_initramfs` | `fn(arch: &str) -> Result<()>` | Output file location pattern |
| CPIO output path | `initramfs_{arch}.cpio` | Exact path (callers depend on it) |

### Call Sites (Must Continue Working)

1. `xtask/src/main.rs:304-306` - `build initramfs` command
2. `xtask/src/run.rs:263` - creates initramfs before run
3. `xtask/src/build/orchestration.rs:37` - `build_all` flow
4. `xtask/src/build/iso.rs:41` - ISO build creates initramfs
5. `xtask/src/tests/behavior.rs:122` - references `initramfs_{arch}.cpio`

### CPIO Format Contract
- Must produce `newc` format (ASCII, 110-byte header)
- Kernel CPIO parser expects this exact format

## Golden/Regression Tests to Lock In

### Existing Tests
| Test | File | What It Verifies |
|------|------|------------------|
| Behavior tests | `xtask/src/tests/behavior.rs` | Boot with initramfs works |
| Regression test | `xtask/src/tests/regression.rs:319` | `test_initramfs_parser` |

### New Tests Needed
| Test | Purpose |
|------|---------|
| CPIO writer unit tests | Verify newc format correctness |
| Manifest parser tests | Verify TOML parsing |
| Contents diff test | Compare old vs new CPIO contents |

### Golden Files
- `tests/golden_boot_x86_64.txt` - Boot log must still pass

## Current Architecture

### File Map

```
xtask/src/build/
├── initramfs.rs          # 184 lines - create_busybox_initramfs()
├── busybox.rs            # 636 lines - BusyBox build + applet list
├── apps.rs               # 288 lines - ExternalApp registry (empty now)
├── mod.rs                # Re-exports
└── ...

xtask/initrd_resources/
├── test.sh               # 68 lines - Basic ash test
└── test-core.sh          # 705 lines - Comprehensive coreutils test

scripts/
└── make_initramfs.sh     # 37 lines - DEAD CODE (references non-existent paths)

initramfs/
└── lib/                  # Unknown purpose - orphaned?

# Build artifacts (at repo root - BAD)
initrd_root/              # Staging directory
initramfs_x86_64.cpio     # Output
initramfs_aarch64.cpio    # Output
```

### Dependency Graph

```
create_busybox_initramfs()
    ├── busybox::require(arch)     # Gets BusyBox binary path
    ├── busybox::applets()         # Hardcoded symlink list
    ├── include_str!(test.sh)      # Embedded test scripts
    ├── include_str!(test-core.sh)
    ├── hardcoded inittab content
    ├── hardcoded passwd/group/profile
    ├── create_cpio_archive()
    │   ├── Command::new("find")   # EXTERNAL DEP
    │   └── Command::new("cpio")   # EXTERNAL DEP
    └── writes to repo root
```

### Use Cases

| Use Case | Current Flow | Pain |
|----------|-------------|------|
| **Build for run** | `cargo xtask run` → `create_busybox_initramfs` | Works |
| **Build for ISO** | `cargo xtask build iso` → same | Works |
| **Add test file** | Edit Rust, recompile xtask | Slow iteration |
| **Add utility** | Edit `busybox::applets()`, recompile | Requires code knowledge |
| **Different init** | Would need new function | Not flexible |
| **Arch-specific** | Hardcoded `if arch ==` checks | Scattered |

## Constraints

1. **Pure Rust CPIO** - No shelling out to `cpio` binary
2. **TOML over Rhai** - Simpler, no scripting runtime (contradicts TEAM_439 but is pragmatic)
3. **Backward compatible output** - Same CPIO contents (for now)
4. **No breaking API** - Existing call sites must work unchanged
5. **Build artifacts in target/** - Clean workspace

## Pain Points Discovered (Adding nano exercise - TEAM_471)

### Session 1: Building nano from source with musl

Attempting to add `nano` to the initramfs with the **current system** revealed:

| # | Pain Point | Time Wasted | Root Cause |
|---|------------|-------------|------------|
| 1 | **Where to start?** | 10 min | Had to read `c_apps.rs` to understand the pattern. No clear entry point. |
| 2 | **Dependency chain** | 15 min | nano requires ncurses. Need to build ncurses first with musl. |
| 3 | **ncurses C++ fails** | 5 min | musl doesn't play well with ncurses C++ bindings. Need `--without-cxx`. |
| 4 | **ncurses terminfo path** | 5 min | Default installs to `/usr/share/terminfo` (needs root). Reconfigure needed. |
| 5 | **nano needs makeinfo** | 5 min | Documentation build fails. Need to skip docs or install texinfo. |
| 6 | **nano needs kernel headers** | BLOCKER | `linux/vt.h` not in musl. Would need to patch nano or provide headers. |

**Total time before hitting blocker: ~40 minutes**

**What we discovered:** BusyBox vi was already in the initramfs! Adding a new editor wasn't even necessary.

### Session 2: Using Alpine pre-built packages

Tried a different approach - download pre-built `nano` from Alpine Linux packages:

| # | Pain Point | Time Wasted | Root Cause |
|---|------------|-------------|------------|
| 7 | **Package discovery** | 5 min | Had to search Alpine repos to find exact package versions |
| 8 | **Dynamic linking** | 10 min | Alpine nano requires `libncursesw.so.6` + dynamic linker |
| 9 | **Dependency chain** | 10 min | Had to download 4 packages: nano, musl, libncursesw, terminfo |
| 10 | **CPIO list_directory bug** | 15 min | Kernel bug: `/lib` showed empty even though files existed |
| 11 | **Inode size = 0** | 10 min | Kernel bug: All files showed 0 bytes, breaking library loading |
| 12 | **musl library path** | 5 min | Needed `/etc/ld-musl-x86_64.path` to tell linker where to find libs |
| 13 | **readlink syscall missing** | - | Syscall 89 not implemented, symlinks can't be read |
| 14 | **File-backed mmap missing** | BLOCKER | Dynamic linker needs `mmap(fd)` but kernel only has `MAP_ANONYMOUS` |

**Total time before hitting blocker: ~55 minutes**

### Kernel bugs found and fixed during this exercise

| Bug | File | Fix |
|-----|------|-----|
| `list_directory()` prefix matching | `crates/kernel/lib/utils/src/cpio.rs` | Check for "prefix/" not just "prefix" |
| Inode size always 0 | `crates/kernel/fs/initramfs/src/lib.rs` | Set `inode.size` from `entry.data.len()` |

### Infrastructure gaps discovered

| Gap | Impact | Resolution |
|-----|--------|------------|
| **No `readlink` syscall** | Symlinks unusable at runtime | Kernel needs syscall 89 |
| **No file-backed mmap** | Dynamic linking impossible | Kernel needs `mmap(MAP_PRIVATE, fd)` |
| **No library search path** | Linker can't find libs | Need `/etc/ld-musl-*.path` in initramfs |
| **No terminfo** | Curses apps broken | Need `/etc/terminfo/` in initramfs |

### Why this proves the refactor is needed

| Current System | Proposed System |
|----------------|-----------------|
| Hunt through Rust code to find where to add app | Edit `initramfs.toml` |
| Build dependencies manually (ncurses) | Declare dependency, let build handle it |
| Debug musl compatibility issues | Use pre-tested musl packages or fail fast |
| Discover BusyBox already has vi after 40 min | See all symlinks in manifest upfront |
| No visibility into initramfs contents | `initramfs.toml` shows everything declaratively |
| Manual `readelf` to find library deps | Manifest could auto-detect or document deps |
| Hunt for kernel bugs in VFS code | Errors visible immediately at build time |
| Scattered config (terminfo, ld.path) | All config in `initramfs/files/etc/` |

## Open Questions

### Resolved by This Plan
1. **Rhai vs TOML?** → TOML (simpler, 95% of use cases, no runtime)
2. **Where do test scripts live?** → `initramfs/scripts/` (version controlled, not embedded)
3. **BusyBox symlinks source?** → Generated from BusyBox `--list` or static list in TOML

### Deferred (Kernel Prerequisites)
1. **Dynamic linker support?** → Blocked on file-backed mmap (kernel)
2. **Shared library support?** → Blocked on file-backed mmap (kernel)
3. **readlink syscall?** → Needed for symlinks to work at runtime
4. **Multiple init systems?** → Future (not needed yet)

### New Questions from nano exercise
1. **Should manifest validate at build time?** → YES: check binaries exist, libs are present
2. **How to handle dynamic vs static binaries?** → Auto-detect with `file` or `readelf`
3. **Should we auto-include library dependencies?** → Consider for future (complex)
4. **How to handle terminfo?** → Include basic set in `initramfs/files/etc/terminfo/`

## Files to Delete (Dead Code - Rule 6)

| File | Reason |
|------|--------|
| `scripts/make_initramfs.sh` | References non-existent paths, unused |
| `initramfs/` directory | Unknown purpose, orphaned |
| `xtask/src/build/apps.rs` | `APPS` array is empty, unused |

## Next Phase
Phase 2 will define the target architecture with:
- Pure Rust CPIO writer
- TOML manifest format
- New file layout under `initramfs/`
