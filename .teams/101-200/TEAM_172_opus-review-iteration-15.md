# TEAM_172: Opus Review (after Iteration 15)

**Date:** 2026-02-04
**Status:** Complete
**Type:** Quality review of haiku iterations 13-15

## Scope Reviewed

Reviewed all commits from the last 3 haiku iterations:

- **AcornOS** (3 commits): SSH setup (ssh.rs), Tier 0-2 package installation (main.rs), test instrumentation (live.rs)
- **distro-builder** (3 commits): Installable trait + Op enum tests, cargo fmt, executor extraction

## Verification Results

- `cargo check --workspace`: Clean (only pre-existing leviso/fsdbg warnings)
- `cargo test -p acornos`: 31 tests pass
- `cargo test`: 4 tests pass
- `cargo test -p distro-builder`: 60 tests pass
- `cargo test -p distro-spec`: 73 tests pass

## Bugs Found and Fixed

### 1. sshd_config commented-line detection bug (AcornOS ssh.rs)

**Severity:** Medium — would cause SSH root login to not work on the live ISO if Alpine's default sshd_config has settings commented out.

**Root cause:** `configure_sshd()` used `config.contains("PermitRootLogin yes")` to check if the setting was already active. However, `String::contains()` matches substrings, so `"#PermitRootLogin yes"` (a commented-out line) also matches. This meant the function would skip adding the uncommented setting.

**Fix:** Changed to line-by-line checking with `config.lines().any(|line| line.trim() == target)` to only match active (uncommented) settings.

**Commit:** `fix(acorn): fix sshd_config commented-line detection bug`

## Code Quality Observations (No Action Needed)

- **SSH key comment hardcoded**: `ssh.rs:47` uses `-C "root@acornos"` instead of pulling from distro-spec. This is cosmetic (SSH key comments are not functional) and not worth changing.
- **live.rs copies profile.d scripts**: The implementation is clean — copies all files from `profile/live-overlay/etc/profile.d/` except `welcome.sh` (which is generated inline). Good separation of concerns.
- **Package installation in download phase**: `main.rs:414` calls `packages()` during `cmd_download_alpine()`. This is the right place — packages need to be installed into rootfs before EROFS build.

## No Blocked Tasks

No tasks were marked BLOCKED by haiku in iterations 13-15. All PRD tasks through 3.15 are correctly marked [x].

## Files Modified

- `AcornOS/src/component/custom/ssh.rs` — sshd_config detection bug fix
