# TEAM_474: Implement Initramfs Builder Refactor

## Objective

Execute the initramfs builder refactor plan from `docs/planning/refactor-initramfs-builder/`.

Key deliverables:
- Pure Rust CPIO writer (no external `cpio` dependency)
- TOML manifest for declarative initramfs configuration
- Non-interactive TUI dashboard for build progress
- Clean up dead code and legacy paths

## Progress Log

### Session 1 (2026-01-13)

**Starting state:**
- Plan documents complete (phase-0 through phase-5)
- `initramfs/` directory structure created by TEAM_471
- `initramfs/initramfs.toml` manifest exists

**Phase 3 Implementation:**

1. Added dependencies to `xtask/Cargo.toml`:
   - `ratatui = "0.29"` for TUI dashboard
   - `crossterm = "0.28"` for terminal control

2. Created `xtask/src/build/initramfs/` module:
   - `cpio.rs` (~220 lines): Pure Rust CPIO newc format writer
     - No external `find | cpio` dependency
     - Supports directories, files, symlinks, device nodes
     - Proper 4-byte alignment and TRAILER
   - `manifest.rs` (~180 lines): TOML manifest parser
     - Variable substitution (`${arch}`, `${toolchain}`)
     - Validation of file existence
   - `builder.rs` (~200 lines): Event-driven archive builder
     - Emits `BuildEvent` variants for TUI
     - Handles directories, binaries, symlinks, files, devices
     - Adds musl dynamic linker automatically
   - `tui.rs` (~280 lines): Non-interactive TUI dashboard
     - Auto-detects TTY, falls back to simple output
     - Shows progress bar, activity log, statistics
   - `mod.rs` (~50 lines): Public API

3. Fixed `initramfs.toml` manifest:
   - Corrected busybox path: `${toolchain}/busybox-out/${arch}/busybox`

**Phase 4 Cleanup:**

1. Deleted dead code:
   - `scripts/make_initramfs.sh` - referenced non-existent paths
   - `xtask/src/build/initramfs_old.rs` - old implementation
   - `initramfs/lib/ld-musl-x86_64.so.1` - orphaned file
   - `xtask/initrd_resources/test.sh`, `test-core.sh` - moved to initramfs/scripts/

2. Updated `xtask/src/build/mod.rs`:
   - Removed `initramfs_old` module reference

**Test Results:**
- `cargo xtask build initramfs` - SUCCESS (3.2 MB CPIO)
- `cargo xtask test behavior` - PASS (kernel boots to Stage 4)
- `cargo xtask vm exec "echo hello"` - PASS (shell responds)

## Key Decisions

1. **Keep legacy path copy**: The new builder writes to `target/initramfs/{arch}.cpio` but also copies to `initramfs_{arch}.cpio` at repo root for backward compatibility. This can be removed in a future cleanup.

2. **Keep `apps.rs`**: Although APPS array is empty, the module is still referenced by orchestration.rs and iso.rs. Left for future external app support.

3. **Auto-add musl linker**: The builder automatically copies `/lib/ld-musl-{arch}.so.1` if available, supporting dynamic binaries.

4. **Simple TUI fallback**: In CI/non-TTY environments, falls back to simple println! output automatically.

## Gotchas Discovered

1. **Busybox path format**: The manifest had wrong path `busybox-${arch}` but actual path is `${arch}/busybox`. Fixed by correcting to `${toolchain}/busybox-out/${arch}/busybox`.

2. **rm requires -f in scripts**: The rm command prompts for confirmation, use `rm -f` for non-interactive scripts.

## Remaining Work

All Phase 3 and Phase 4 items completed:
- [x] Create CPIO writer module
- [x] Create manifest parser module
- [x] Create builder module with events
- [x] Create TUI dashboard module
- [x] Add ratatui/crossterm dependencies
- [x] Delete dead code files
- [x] Verify behavior tests pass

**Future improvements (not blocking):**
- Remove legacy path copy after all call sites updated
- Add binary type detection (static vs dynamic)
- Add Alpine package integration
- Remove apps.rs if confirmed fully unused

## Handoff Notes

**For next team:**

The initramfs builder refactor is complete. The new system:

1. Uses declarative TOML manifest at `initramfs/initramfs.toml`
2. Pure Rust CPIO writer (no external tools)
3. Non-interactive TUI dashboard (auto-detects TTY)
4. Backward compatible (copies to legacy path)

Key files:
- `xtask/src/build/initramfs/` - New builder module
- `initramfs/initramfs.toml` - Manifest configuration
- `initramfs/files/etc/` - Static config files
- `initramfs/scripts/` - Test scripts

To add new files to initramfs:
1. Edit `initramfs/initramfs.toml`
2. Add to appropriate section (files, symlinks, etc.)
3. Run `cargo xtask build initramfs`

The builder auto-detects TUI capability. Force modes:
- `NO_TUI=1 cargo xtask build initramfs` - Simple output
- TUI is disabled in CI automatically (checks `CI` env var)
