# TEAM_471: Refactor Initramfs Builder

## Objective

Complete reimagining of the initramfs builder system:
- Replace external `find | cpio` with pure Rust CPIO writer
- Replace hardcoded Rust with declarative TOML manifest
- Move build artifacts from repo root to `target/`
- Externalize embedded test scripts

## Progress Log

### Session 1 (2026-01-13)

**Discovery:**
- Analyzed current pain points (external deps, hardcoded content, scattered logic)
- Reviewed TEAM_439 Rhai planning (decided TOML is simpler and sufficient)
- Inventoried all call sites and dependencies
- Documented current architecture

**Planning Complete:**
- Created 5-phase refactor plan in `docs/planning/refactor-initramfs-builder/`
- Phase 1: Discovery & Safeguards
- Phase 2: Structural Extraction (target design, TOML format, Rust types)
- Phase 3: Migration (step-by-step, call site inventory, rollback plan)
- Phase 4: Cleanup (dead code removal, encapsulation)
- Phase 5: Hardening (verification, documentation)

## Key Decisions

1. **TOML over Rhai**: TOML with `${arch}` substitution covers 95% of use cases without a scripting runtime. Simpler is better.

2. **Pure Rust CPIO**: Write our own ~150 line CPIO writer. The newc format is simple (110-byte ASCII headers). Eliminates `find` and `cpio` dependencies.

3. **Manifest at `initramfs/initramfs.toml`**: Declarative specification of directories, binaries, symlinks, and files.

4. **Artifacts in `target/initramfs/`**: Clean workspace, consistent with Cargo conventions.

5. **Externalize scripts**: Move `test.sh` and `test-core.sh` from `include_str!` to `initramfs/scripts/`.

6. **Live TUI dashboard**: Using `ratatui` to show real-time build progress with activity feed, progress bar, and statistics. Falls back to simple output in non-TTY environments.

## Gotchas Discovered

1. **Kernel can't follow symlinks for /init**: BusyBox binary must be copied to `/init`, not symlinked. Already handled in current code.

2. **BusyBox already has vi**: When trying to add nano, discovered vi is already in the initramfs as a BusyBox applet. The current system made this hard to discover.

3. **nano requires ncurses + kernel headers**: Adding nano to musl initramfs is non-trivial:
   - Requires building ncurses first (with `--without-cxx` for musl)
   - nano needs `linux/vt.h` which musl doesn't provide
   - Would need patches or kernel headers to complete

4. **Pain point validation**: Attempting to add a simple program like nano with the current system took 40+ minutes and hit a blocker. This validates the need for the refactor.

5. **CPIO newc format**: Uses ASCII octal numbers in header fields, 4-byte alignment for data. Must get this exactly right.

6. **Many call sites reference legacy paths**: `initramfs_{arch}.cpio` at repo root is used by ~10 files. Need backward-compat wrapper during migration.

7. **`apps.rs` is dead code**: The `APPS` array is empty since TEAM_459 migrated to BusyBox. Can delete.

8. **`scripts/make_initramfs.sh` is dead code**: References paths that haven't existed since early development.

### Session 2 (2026-01-13) - Alpine nano experiment

**Objective:** Try adding nano from Alpine packages to validate the new initramfs builder.

**Findings:**

9. **Alpine packages are dynamically linked**: Downloaded `nano-8.2-r0.apk` - the binary requires:
   - `/lib/ld-musl-x86_64.so.1` (dynamic linker)
   - `libncursesw.so.6`
   - `libc.musl-x86_64.so.1`

10. **CPIO list_directory bug found and fixed**: The CPIO parser's `list_directory()` function had a bug where it checked for prefix "lib" but entries like "lib/foo.so" have a "/" after the prefix. Fixed in `crates/kernel/lib/utils/src/cpio.rs`.

11. **Initramfs inode sizes were zero**: The `make_inode()` function wasn't setting file sizes. Fixed in `crates/kernel/fs/initramfs/src/lib.rs` to use `entry.data.len()`.

12. **Kernel mmap doesn't support file-backed mappings**: The critical blocker - dynamic linking requires `mmap(MAP_PRIVATE, fd, ...)` to map library files into memory. Our kernel only supports `MAP_ANONYMOUS`. Error message:
    ```
    [MMAP] Only MAP_ANONYMOUS supported, got flags=0x2
    Error loading shared library libncursesw.so.6: Invalid argument
    ```

13. **musl library path config**: Created `/etc/ld-musl-x86_64.path` with `/lib` to tell musl where to find libraries, but this doesn't help without proper mmap support.

**Code Changes:**
- `crates/kernel/lib/utils/src/cpio.rs`: Fixed `list_directory()` prefix matching (TEAM_471)
- `crates/kernel/fs/initramfs/src/lib.rs`: Set inode size from CPIO entry data (TEAM_471)
- `xtask/src/build/initramfs.rs`: Added `copy_dir_recursive()` helper for lib/etc dirs (TEAM_471)
- `xtask/initrd_resources/lib/`: Added ncurses library files
- `xtask/initrd_resources/etc/`: Added musl library path config and terminfo

**Conclusion:** Dynamic linking is blocked by kernel limitations (no file-backed mmap). Until TEAM_470 or similar completes file-backed mmap support, only statically-linked binaries work in the initramfs.

### Session 3 (2026-01-13) - Kernel Prerequisites Research

**Objective:** Research and document all kernel prerequisites needed to run nano from Alpine packages.

**Actions:**
- Researched musl dynamic linker requirements (mmap, mprotect, pread, fstat, readlink)
- Researched ncurses requirements (TIOCGWINSZ, SIGWINCH, poll)
- Audited current kernel implementation status
- **Discovered TEAM_470 already completed PT_INTERP support** - interpreter loading works!
- Created separate planning document: `docs/planning/nano-support/`

**Current Status:**
| Feature | Status |
|---------|--------|
| PT_INTERP handling | **COMPLETE (TEAM_470)** |
| Interpreter loads | **COMPLETE (TEAM_470)** |
| mprotect | COMPLETE |
| poll/ppoll | COMPLETE |
| sigaction | COMPLETE |
| TIOCGWINSZ | COMPLETE (stub 80x24) |
| pread64 | COMPLETE |
| readlinkat | COMPLETE |
| **File-backed mmap** | **COMPLETE (Session 4)** |

**Key Finding:** All blockers for dynamic linking are now resolved!

**New Planning Document:** `docs/planning/nano-support/` - Separate plan for kernel nano prerequisites
- Phase 1: Discovery & Requirements (complete)
- Phase 2: File-backed mmap implementation (detailed design)

### Session 4 (2026-01-13) - File-Backed mmap Implementation

**Objective:** Implement file-backed mmap to enable dynamic linking.

**Implementation:**
- Added `sys_mmap_file()` function in `syscall/src/mm.rs`
- Modified `sys_mmap()` to dispatch file-backed mappings (non-MAP_ANONYMOUS)
- Uses "eager copy" approach: reads file data, allocates pages, copies to user space
- Supports MAP_PRIVATE semantics (private copy, not shared)
- ~150 lines of new code

**Key Code Changes:**
```rust
// syscall/src/mm.rs - Added file-backed mmap support
fn sys_mmap_file(addr, len, prot, flags, fd, offset) -> SyscallResult {
    // 1. Get file from fd table
    // 2. Read file data at offset
    // 3. Allocate pages, copy data
    // 4. Map into user address space
    // 5. Track VMA
}
```

**Test Result:**
```
$ cargo xtask vm exec "/bin/hello_dynamic"
[EXEC] Dynamic binary, interpreter: /lib/ld-musl-x86_64.so.1
[EXEC] Dynamic: interp_entry=0x7f000006cbfa main_entry=0x400340 AT_BASE=0x7f0000000000
Hello from dynamic binary!
```

**DYNAMIC LINKING NOW WORKS!** The musl dynamic linker successfully loads shared libraries and executes the program.

### Session 5 (2026-01-13) - TUI Design & Directory Structure

**Objective:** Design non-interactive TUI dashboard and create initramfs directory structure.

**Key Decision:** User requested the TUI be part of xtask (not a separate binary) and explicitly NON-INTERACTIVE - just a dashboard showing progress and status.

**Planning Documents Created:**
- `docs/planning/refactor-initramfs-builder/phase-0-tui-design.md` - Complete TUI specification:
  - Non-interactive dashboard using ratatui
  - Event-driven architecture (builder emits BuildEvent, TUI renders)
  - Auto-detects TTY, falls back to simple line output in CI
  - Three panels: Activity Feed, Overall Progress, Statistics
  - State machine: Idle → Building → Complete/Error

**Initramfs Directory Structure Created:**
```
initramfs/
├── initramfs.toml          # Declarative manifest (60+ symlinks, devices, files)
├── files/
│   └── etc/
│       ├── inittab         # BusyBox init config
│       ├── passwd          # root:x:0:0:root:/root:/bin/ash
│       ├── group           # root:x:0:
│       └── profile         # PATH, PS1, aliases
└── scripts/
    ├── test.sh             # ASH shell test script
    └── test-core.sh        # Core functionality tests
```

**initramfs.toml highlights:**
- `[layout]` - 11 directories to create
- `[binaries.busybox]` - Uses `${arch}` substitution
- `[symlinks]` - 60+ BusyBox applet symlinks
- `[files]` - Config files with mode specifications
- `[scripts]` - Test scripts with executable mode
- `[devices]` - Character devices (null, zero, tty, console, etc.)

## Remaining Work

Planning complete. Initramfs directory structure created. Implementation next.

**Phase 3 Implementation Order:**
1. Create `xtask/src/build/initramfs/cpio.rs` (pure Rust CPIO writer)
2. Create `xtask/src/build/initramfs/manifest.rs` (TOML parser)
3. ~~Create `initramfs/` directory with static files and manifest~~ **DONE**
4. Create `xtask/src/build/initramfs/builder.rs`
5. Create `xtask/src/build/initramfs/mod.rs`
6. Create `xtask/src/build/initramfs/tui.rs` (dashboard)
7. Verify output matches current CPIO
8. Update call sites to new paths (Phase 4)
9. Delete dead code (Phase 4)

## Handoff Notes

**For next team:**

The plan is complete and ready for implementation. Start with Phase 3.

Key files:
- `docs/planning/refactor-initramfs-builder/phase-3.md` - Step-by-step migration
- `docs/planning/refactor-initramfs-builder/phase-2.md` - Target design and Rust types

The TOML manifest format in Phase 2 is the source of truth for what the new system should support.

Rollback plan is documented in Phase 3 if issues arise.
