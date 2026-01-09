# TEAM_325 — Review & Implement: debug-tools & shell-session

## Scope
1. Review implementation of debug-tools (TEAM_323) and shell-session (TEAM_324) against their plans.
2. Implement missing debug-tools module.

---

## Phase 1: Implementation Status

### debug-tools (TEAM_323)

| Indicator | Status |
|-----------|--------|
| Team file progress | All phases unchecked (Phase 1-5 pending) |
| Implementation code | **NOT FOUND** |
| `xtask/src/debug/` module | Does not exist |
| Git commits | None for debug commands |

**Status: NOT STARTED**

The planning documents exist (`docs/planning/debug-tools/phase-1.md`, `phase-2.md`) but no implementation has been done. The `debug mem` and `debug regs` commands specified in the plan do not exist.

---

### shell-session (TEAM_324)

| Indicator | Status |
|-----------|--------|
| Team file progress | Phase 1-3 ✅, Phase 4-5 pending |
| Implementation code | **EXISTS** |
| Files | `xtask/src/shell/mod.rs`, `exec.rs`, `session.rs` |
| Build status | ✅ Compiles successfully |
| Unit tests | ✅ 12/12 pass |

**Status: IMPLEMENTATION COMPLETE (Phase 3), TESTING PENDING (Phase 4)**

---

## Phase 2: Gap Analysis

### debug-tools (TEAM_323) — Plan vs Reality

| Planned UoW | Implemented? | Notes |
|-------------|--------------|-------|
| `cargo xtask debug mem <addr>` | ❌ NO | QMP `memsave` wrapper not built |
| `cargo xtask debug regs` | ❌ NO | QMP `human-monitor-command` wrapper not built |
| New `xtask/src/debug/` module | ❌ NO | Module does not exist |

**Gap: 100% unimplemented.** Only planning was done.

---

### shell-session (TEAM_324) — Plan vs Reality

| Planned UoW | Implemented? | Matches Spec? |
|-------------|--------------|---------------|
| `shell start` | ✅ YES | ✅ VNC + QMP socket, PID saved to `.qemu-session.json` |
| `shell send "<text>"` | ✅ YES | ✅ QMP `sendkey`, auto-appends Enter |
| `shell screenshot` | ✅ YES | ✅ QMP `screendump`, PPM→PNG conversion attempted |
| `shell stop` | ✅ YES | ✅ Kills PID, removes session file |
| `shell exec` (from TEAM_323) | ✅ YES | ✅ Ephemeral mode works |
| Session state file | ✅ YES | Schema matches: `pid`, `qmp_socket`, `arch`, `started_at` |
| Keycode translation | ✅ YES | Comprehensive: a-z, A-Z, 0-9, symbols |

**Gap: None.** All planned features are implemented correctly.

**Behavioral contract compliance:**
- Q1 (auto-add Enter): ✅ Implemented
- Q2 (handle crash): ✅ `is_alive()` check implemented
- Q3 (headless): ✅ VNC display, no terminal output

---

## Phase 3: Code Quality Scan

### Search Results

| Pattern | Findings |
|---------|----------|
| `TODO` | 0 |
| `FIXME` | 0 |
| `stub/placeholder` | 0 |
| `unimplemented!` | 0 |
| Empty catch blocks | 0 |

**No incomplete work markers found.** Code is clean.

### Potential Issues

1. **`char_to_qcode()` fallback** (`session.rs:345`):
   ```rust
   _ => ("spc", false), // Default to space for unknown
   ```
   Unknown characters silently become spaces. Could log a warning.

2. **Screenshot conversion** (`session.rs:223`):
   Uses external `convert` command (ImageMagick). Falls back to PPM if not available.

---

## Phase 4: Architectural Assessment

### Rule Compliance

| Rule | Status | Notes |
|------|--------|-------|
| Rule 0 (Quality > Speed) | ✅ | Clean implementation, no shortcuts |
| Rule 5 (Breaking Changes) | ✅ | No compatibility shims |
| Rule 6 (No Dead Code) | ✅ | No unused functions |
| Rule 7 (Modular) | ✅ | Proper separation: `exec.rs`, `session.rs` |

### Architecture Quality

- **Separation of concerns:** ✅ Ephemeral (`exec.rs`) vs persistent (`session.rs`) cleanly split
- **State management:** ✅ Session state properly serialized/deserialized
- **Error handling:** ✅ Proper `Result` propagation, stale session cleanup
- **Resource cleanup:** ✅ Socket and PID cleanup in `stop()`

### Minor Concerns

1. **Code duplication:** Both `exec.rs` and `session.rs` build QEMU args independently. Could consolidate using `QemuBuilder` in both places.

2. **QMP client reuse:** `QmpClient` is generic enough. `memsave` and `human-monitor-command` could be added trivially to support debug-tools.

---

## Phase 5: Direction Check

### shell-session (TEAM_324)

**Recommendation: CONTINUE**

- Implementation is complete and matches plan
- No blockers
- Ready for Phase 4 (Testing)

### debug-tools (TEAM_323)

**Recommendation: RESTART**

- Planning is complete but implementation never started
- Team file incorrectly shows Phase 1-2 as unchecked, but docs exist
- Next team should pick up Phase 3 (Implementation)

---

## Phase 6: Findings Summary

### ✅ shell-session (TEAM_324)

| Aspect | Status |
|--------|--------|
| Implementation | COMPLETE |
| Code Quality | EXCELLENT |
| Architecture | CLEAN |
| Test Status | PENDING |
| Recommendation | Ready for testing (Phase 4) |

### ⚠️ debug-tools (TEAM_323)

| Aspect | Status |
|--------|--------|
| Implementation | NOT STARTED |
| Planning | COMPLETE |
| Recommendation | Needs implementation team |

---

## Recommendations

1. **TEAM_324 (shell-session):** Mark as ready for Phase 4. Manual testing needed:
   - `cargo xtask shell start`
   - `cargo xtask shell send "ls"`
   - `cargo xtask shell screenshot`
   - `cargo xtask shell stop`

2. **TEAM_323 (debug-tools):** Needs new implementation team to:
   - Create `xtask/src/debug/` module
   - Implement `debug mem` using QMP `memsave`
   - Implement `debug regs` using QMP `human-monitor-command`
   - Update team file to reflect actual progress

3. **Minor cleanup:** Consider adding warning for unknown characters in `char_to_qcode()`.

---

---

## Implementation: debug-tools (TEAM_325)

After the review identified debug-tools as NOT STARTED, TEAM_325 implemented the missing module.

### Files Created/Modified

| File | Action | Description |
|------|--------|-------------|
| `xtask/src/debug/mod.rs` | CREATED | New debug module with `regs` and `mem` commands |
| `xtask/src/main.rs` | MODIFIED | Added `mod debug`, `Debug` subcommand, handler |

### Commands Implemented

```
cargo xtask debug regs [--qmp-socket PATH]
cargo xtask debug mem <ADDR> [--len N] [--qmp-socket PATH]
```

### Implementation Details

1. **`debug regs`**: Uses QMP `human-monitor-command` with `info registers`
2. **`debug mem`**: Uses QMP `memsave` to dump memory, displays as hex dump with ASCII
3. **QMP socket auto-detection**: Tries `./qemu-session.sock` then `./qmp.sock`

### Verification

- Build: ✅ Passes
- Tests: ✅ 12/12 pass
- Help: ✅ `cargo xtask debug --help` works

---

## Handoff Checklist

- [x] Project builds cleanly
- [x] All unit tests pass (12/12)
- [x] No TODOs or incomplete work found
- [x] Team file updated
- [x] debug-tools implementation complete
- [x] Findings documented
- [x] Phase 3 testing plan created (docs/planning/debug-tools/phase-3-testing.md)
- [x] Debug tools integration tests implemented (xtask/src/tests/debug_tools.rs)
- [x] Golden files created for both architectures
- [x] `cargo xtask test debug` command working
