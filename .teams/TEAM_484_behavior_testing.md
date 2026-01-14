# TEAM_484: Behavior Testing Suite

## Goal
Create a behavior testing system that boots LevitateOS and verifies components work correctly inside the VM.

## What We're Building
- `cargo xtask test behavior` command
- BehaviorTest trait for defining tests
- Minimal test suite (~10 tests): boot, shell, coreutils, auth, process, network
- Phased execution: boot tests must pass before others run

## Files Changed
- `xtask/src/test/helpers.rs` - Add `start_levitate()` method
- `xtask/src/test/mod.rs` - Add `Behavior` command variant
- `xtask/src/test/behavior/mod.rs` - BehaviorTest trait, TestRunner
- `xtask/src/test/behavior/registry.rs` - Test collection
- `xtask/src/test/behavior/tests/*.rs` - Individual tests

## Design Decisions
- Tests defined as Rust structs implementing BehaviorTest trait
- Phased execution (0=boot, 1=shell, 2=general) - earlier phases abort on failure
- Pattern matching via regex for expected output
- Uses existing TestVm infrastructure with new `start_levitate()` method

## Status
- [x] Team file created
- [x] Add start_levitate() to TestVm
- [x] Create behavior module
- [x] Implement tests (10 tests across 5 categories)
- [ ] Test the suite

## TODO - Next Steps

1. **Fix terminal/shell issue** - Login fails because brush shell tries to detect cursor position on serial console
   - Error: `The cursor position could not be read within a normal duration`
   - Tried: Adding `TERM=dumb` to kernel cmdline (didn't help)
   - Try: Check if brush has `--no-editing` flag, or set TERM in shell env, or configure agetty

2. **Verify login credentials** - Confirm root:root works or if passwordless login is needed

3. **Run full test suite** - `cargo xtask test behavior --verbose`

4. **Fix any failing tests** - Adjust patterns/timeouts as needed

5. **Clean up compiler warnings** - Remove unused imports/functions

## Known Issue
The shell (brush) queries terminal capabilities even on dumb terminals, causing login to fail on serial console.
