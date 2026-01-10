# TEAM_326: xtask Command Refactor

**Created:** 2026-01-09  
**Status:** In Progress

---

## Scope

Refactor all xtask commands for clarity and better organization:

1. Merge `shell` + `debug` → `vm` (all VM interaction in one place)
2. Rename `image` → `disk` (clearer naming)
3. Simplify `run` to use flags instead of subcommands
4. Rename `preflight` → `check`
5. Remove duplicate `screenshot` commands
6. Update shell scripts in root
7. Update all READMEs

---

## New Command Structure

```bash
# Most common
cargo xtask run                    # Build + run with GUI
cargo xtask build                  # Build all
cargo xtask test                   # Run all tests

# Run variations (flags)
cargo xtask run --gdb              # With GDB server
cargo xtask run --headless         # No display
cargo xtask run --term             # Terminal mode
cargo xtask run --vnc              # VNC display
cargo xtask run --profile pixel6   # Pixel 6 profile

# VM session & debug (merged)
cargo xtask vm start               # Start persistent session
cargo xtask vm stop                # Stop session
cargo xtask vm send "ls"           # Send command
cargo xtask vm exec "ls"           # One-shot execution
cargo xtask vm screenshot          # Take screenshot
cargo xtask vm regs                # Dump registers
cargo xtask vm mem 0x1000          # Dump memory

# Disk (renamed from image)
cargo xtask disk create
cargo xtask disk install
cargo xtask disk status

# Utilities
cargo xtask check                  # Renamed from preflight
cargo xtask clean
cargo xtask kill
```

---

## Progress

- [x] Verify tests pass before changes
- [x] Refactor main.rs command structure
- [x] Rename device → disk module
- [x] Create vm module (merge shell + debug)
- [x] Simplify run commands (flags instead of subcommands)
- [x] Rename preflight → check
- [x] Update shell scripts (run.sh, run-test.sh, etc.)
- [x] Update READMEs (vm/, disk/)
- [x] Delete old shell/ and debug/ directories
- [x] Verify all tests pass after changes

---

## Additional Updates

- Updated `xtask/src/support/README.md` with new commands
- Updated `xtask/src/tests/README.md` with screenshot test docs
- Updated `.agent/rules/behavior-testing.md` with complete command reference (Section VII)

---

## Files Changed

### New
- `xtask/src/vm/mod.rs` - VM interaction commands
- `xtask/src/vm/session.rs` - Persistent session management
- `xtask/src/vm/exec.rs` - One-shot command execution
- `xtask/src/vm/debug.rs` - Register/memory inspection
- `xtask/src/vm/README.md` - Documentation

### Renamed
- `xtask/src/device/` → `xtask/src/disk/`

### Deleted
- `xtask/src/shell/` (merged into vm)
- `xtask/src/debug/` (merged into vm)

### Modified
- `xtask/src/main.rs` - Refactored command structure
- `xtask/src/build/commands.rs` - Updated imports
- `xtask/src/run.rs` - Updated imports
- `xtask/src/tests/keyboard_input.rs` - Updated imports
- `run.sh`, `run-test.sh`, `run-pixel6.sh`, `run-vnc.sh`, `screenshot.sh` - Updated

---

## Notes

