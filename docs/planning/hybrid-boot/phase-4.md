# Phase 4: Verification — Hybrid Boot Specification

## Verification Strategy
We will use a combination of automated regression tests and manual QEMU checks to ensure the boot sequence and terminal interactions behave as specified.

## Automated Tests
### Step 1: Behavioral Regression
- **UoW 1**: Create `tests/behavior/terminal_wrap.rs` to verify that `0x08` correctly wraps to the previous line when at Column 0.
- **UoW 2**: Add a golden file check for the new stage logs: `[BOOT] Stage 1 ... [BOOT] SYSTEM_READY`.

### Step 2: Unit Testing
- **UoW 1**: Test `BootStage` transition logic in `kernel/src/main.rs` (using `cfg(test)` mocks if needed) to ensure invalid transitions are caught.

## Manual Verification
### Step 1: Fallback Scenarios
- **UoW 1**: Disable the VirtIO GPU in `xtask` (or via QEMU args) and verify that the system still boots to Stage 5 using UART fallback (SPEC-1).
- **UoW 2**: Temporarily remove the initrd and verify transition to Maintenance Shell (SPEC-4).

## hardware Verification (Out of Scope for Implementation)
- **Note**: Physical Pixel 6 verification requires USB-C SBU debug cable and and unlocked bootloader. Documentation in Phase 1 provides the necessary mapping for future teams.
