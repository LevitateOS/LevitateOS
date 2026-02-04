# TEAM_208: Opus Review after Iteration 39

**Date:** 2026-02-04
**Status:** Complete
**Type:** Review (iterations 37-39)

## Scope Reviewed

Last 3 iterations' commits across all submodules:

- **AcornOS** (3 commits): APK database init + --usermode removal, ISO_LABEL constant in UKI cmdlines, UKI building implementation
- **IuppiterOS** (3 commits): duplicate console= parameter fix in UKI, udev rule for mq-deadline scheduler, operator user creation after passwd/group files
- **distro-spec** (3 commits): sdparm removal from REFURBISHMENT_PACKAGES, cargo fmt, IuppiterOS variant skeleton
- **distro-builder** (3 commits): Installable trait tests, cargo fmt, executor extraction

## Verification Results

- `cargo check --workspace`: Clean (only pre-existing leviso warnings)
- `cargo test -p acornos --lib`: 34 tests pass
- `cargo test -p iuppiteros --lib`: 22 tests pass
- `cargo test -p distro-builder`: 60+ tests pass
- `cargo test -p distro-spec --lib`: 73 tests pass
- `cargo check` for install-tests: Clean (1 pre-existing unused import warning in fsdbg)

## Bugs Found

**None.** The last 3 iterations' code is clean:

1. **AcornOS UKI builder** (uki.rs): Correctly uses ISO_LABEL constant, VGA_CONSOLE and SERIAL_CONSOLE in base cmdline (AcornOS entries have empty extra_cmdline). APK database init fix is sound.
2. **IuppiterOS UKI builder** (uki.rs): Correctly sources console parameters from distro-spec extra_cmdline only, no duplication.
3. **IuppiterOS operator user** (users.rs): Clean implementation. ensure_group/ensure_user use `starts_with("name:")` pattern. add_user_to_group properly parses and reconstructs group lines.
4. **IuppiterOS udev rule**: Single correct rule targeting rotational block devices.
5. **distro-spec sdparm removal**: Correct (verified not in Alpine v3.23).
6. **No remaining `distro_spec::acorn` imports in IuppiterOS** — verified with grep.

## BLOCKED Tasks Assessment

Tasks 8.3-8.7 (AcornOS install-tests Phases 2-6) and 8.12-8.13 (IuppiterOS Phases 1-5, 6) remain unchecked. These require running the full install-tests QEMU sequence which depends on:
1. TEAM_154 boot detection I/O buffering (known harness issue)
2. TCG emulation being slow and timing-sensitive

The install-tests infrastructure is now unblocked (ISO paths fixed in iteration 38), but actual test execution is a runtime concern, not a code bug. These tasks need haiku to attempt running the tests, not an opus code review.

## Files Modified

None. No bugs found, no changes needed.

## Key Observations

- The iuppiter-engine placeholder binary PID file issue (`$$` in subshell) was already noted as acceptable in the iteration 33 opus review.
- The fsdbg `ESSENTIAL_UNITS` unused import warning is pre-existing LevitateOS infrastructure, not from recent iterations.
- IuppiterOS copy-paste audit is now clean — all previous reviews successfully caught and fixed AcornOS references.
