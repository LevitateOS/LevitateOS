# TEAM_171: AcornOS Test Instrumentation

## Status
**Date**: 2026-02-04
**Status**: ✅ Complete

## Task
Implement Phase 3 task 3.15: Test instrumentation - Copy /etc/profile.d/00-test.sh to emit ___SHELL_READY___ marker for install-tests.

## What Was Implemented
Modified AcornOS/src/component/custom/live.rs to copy test instrumentation scripts from profile/live-overlay/etc/profile.d/ to the rootfs during the CreateWelcomeMessage phase.

### Changes Made
1. **File**: AcornOS/src/component/custom/live.rs
   - Extended `create_welcome_message()` to read and copy files from `profile/live-overlay/etc/profile.d/`
   - Now copies 00-acorn-test.sh and live-docs.sh (skips welcome.sh since we create it inline)
   - Sets proper permissions (0o755) on copied scripts

### Files Modified
- `AcornOS/src/component/custom/live.rs` - Extended test script copying

### Verification
- ✅ `cargo check` passes
- ✅ Rootfs rebuilt: `output/rootfs-staging/etc/profile.d/` now contains:
  - `00-acorn-test.sh` - Test marker script with ___SHELL_READY___ emission
  - `live-docs.sh` - Live ISO documentation helper
  - `welcome.sh` - Welcome message
- ✅ Verified ___SHELL_READY___ marker is present in compiled script
- ✅ All scripts have proper executable permissions (755)

## Key Decisions
1. **Copy approach**: Rather than creating a new CustomOp, extended the existing CreateWelcomeMessage operation to also copy the profile.d scripts. This keeps the component system clean and avoids unnecessary duplication.
2. **Script selection**: Skips welcome.sh since we already create it inline with formatted OS_NAME reference.

## How It Works
The 00-acorn-test.sh script:
1. Only activates on serial console (ttyS0) - detects test environment
2. Emits ___SHELL_READY___ marker when shell initialization completes
3. Tracks commands with ___CMD_START___ and ___CMD_END___ markers
4. Provides ___PROMPT___ markers for test harness synchronization
5. Regular users on tty1 see normal behavior with live-docs.sh help

## Blockers
None - task completed successfully.

## Notes
The install-tests framework looks for the ___SHELL_READY___ pattern in console output to detect when the shell is ready for command execution. This is part of Phase 3's testing instrumentation (separate from Phases 1-5 test detection patterns).
