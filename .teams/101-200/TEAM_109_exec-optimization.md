# TEAM_109: Exec and Login Optimization + Auth Modularization

**Status:** Complete
**Started:** 2026-01-24
**Completed:** 2026-01-24

## Objective

1. Optimize install-tests execution time (~160s savings)
2. Modularize auth subsystem (critical, persistent pain point)
3. Fix root cause of login failures

## Prior Work

- TEAM_105: PAM fix, SSH prep, autologin, pre-reboot verification
- TEAM_107: Shell instrumentation (00-levitate-test.sh), boot detection, pre-built initramfs
- TEAM_108: Root cause analysis - `/usr/bin/login` symlink missing

## Changes Made

### Task 1: Exec Rewrite - Use Shell Markers ✅
**Files:** `testing/install-tests/src/qemu/exec.rs`

- Removed sync_shell() call from exec()
- Added wait_for_prompt() that waits for `___PROMPT___` marker
- Uses 500ms timeout with graceful fallback (proceeds anyway if marker not found)
- Expected savings: ~150 seconds per test run

### Task 2: Login Simplification ✅
**Files:** `testing/install-tests/src/qemu/utils.rs`, `testing/install-tests/src/main.rs`

- Reduced initial settle time from 3000ms to 500ms
- Removed debug eprintln statements
- Removed various 200ms micro-delays
- Simplified post-login warmup from 5-iteration loop to single check
- Reduced post-login settle time from 3000ms to 500ms
- Expected savings: ~10 seconds per test run

### Task 3: UEFI Boot Testing ✅
**Files:** `testing/install-tests/src/steps/phase5_boot.rs`

- Added efibootmgr call after bootctl install
- Creates real EFI boot entry instead of relying on fallback boot path
- Best-effort (warning on failure, not error) since UEFI fallback still works

### Task 4: Auth Subsystem Modularization ✅
**Files:** `testing/install-tests/src/qemu/auth.rs` (NEW), `testing/install-tests/src/qemu/mod.rs`, `testing/install-tests/src/qemu/utils.rs`

Created a dedicated auth module with:
- `AuthMethod` enum: Password, ShellMarker, SSH (future)
- `LoginState` state machine with clear transitions
- `AuthConfig` for retry/timeout configuration
- `AuthResult` with diagnostics for debugging
- `authenticate()` main entry point
- `wait_for_shell_ready()` for instrumented shells
- Shell marker constants in `markers` submodule
- Backward-compatible `login()` method

The old inline login code in utils.rs has been replaced with delegation to the auth module.

### Task 5: Fix /usr/bin/login Symlink ✅
**Files:** `leviso/src/component/definitions.rs`

- Added symlink `usr/bin/login -> ../sbin/login`
- This was the ROOT CAUSE of login failures (TEAM_108 analysis)
- agetty defaults to `/bin/login` (→ `/usr/bin/login`) but login was only at `/usr/sbin/login`

### Task 6: Add efibootmgr to tarball ✅
**Files:** `leviso/src/component/definitions.rs`

- Added `efibootmgr` to SBIN_UTILS
- Enables proper UEFI boot entry creation during installation

## Code Cleanup Completed

Removed all unused code:
- `SyncConfig`, `drain_output`, `sync_shell`, `sync_shell_secondary` from sync.rs
- `exec_streaming` from exec.rs
- `exec_chroot_streaming` from chroot.rs
- `LoginState::Failed` variant (never constructed)
- Unused exports from mod.rs

**Result: Zero warnings.**

## Verification

```bash
# Build new ISO with fixes
cd leviso && cargo run -- build

# Run installation tests
cd testing/install-tests && cargo run
```

**Success criteria:**
- No "WARN: Sync timeout" warnings (Task 1)
- Login phase completes in <2 seconds (Task 2)
- `efibootmgr -v` shows LevitateOS entry or graceful fallback (Task 3)
- Login succeeds on installed system (Task 5 - symlink fix)

## Architecture Notes

### Auth Module Design

```
testing/install-tests/src/qemu/auth.rs
├── AuthMethod (enum)
│   ├── Password { username, password }
│   ├── ShellMarker
│   └── Ssh { username, key_path } [future]
├── LoginState (state machine)
│   ├── WaitingForLoginPrompt
│   ├── WaitingForPasswordPrompt
│   ├── WaitingForShellPrompt
│   ├── VerifyingShell
│   ├── Complete
│   └── Failed
├── AuthConfig (retry/timeout settings)
├── AuthResult (success/diagnostics)
├── markers (shell marker constants)
└── Console impl
    ├── authenticate()
    ├── authenticate_with_config()
    ├── auth_password() [internal]
    ├── auth_shell_marker() [internal]
    ├── login() [backward compat]
    └── wait_for_shell_ready()
```

### Why This Matters

Serial console authentication has been a persistent pain point. By extracting it into a dedicated module:
1. State machine is explicit and debuggable
2. Retry logic is centralized
3. Multiple auth methods can be supported
4. Diagnostics are comprehensive
5. Future SSH support is straightforward to add
