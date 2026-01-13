# Phase 5: Cleanup, Regression Protection, and Handoff

**Bug:** Brush shell crash - rt_sigaction format mismatch  
**Team:** TEAM_438  
**Parent:** `docs/planning/brush-requirements/`

## Cleanup Tasks

### Code Cleanup

1. Remove any debug logging added during investigation
2. Ensure consistent code style with existing codebase
3. Add proper TEAM comments to modified code
4. Update any outdated comments in signal.rs

### Documentation Cleanup

1. Update `BRUSH_REQUIREMENTS.md` with fix status
2. Archive investigation notes if no longer needed
3. Update team file with completion status

## Regression Protection

### Tests to Add

1. **Signal Handler Test**
   - Register signal handler via sigaction
   - Verify handler is called when signal delivered
   - Verify old action is returned correctly

2. **Edge Case Tests**
   - NULL act pointer (query only)
   - NULL oldact pointer (set only)
   - Invalid signal numbers
   - SIGKILL/SIGSTOP rejection

### Golden Log Updates

If boot behavior changes:
1. Run `cargo xtask run --term --headless --update`
2. Review changes to golden logs
3. Commit updated golden logs with fix

### Behavioral Verification

```bash
# Full test suite
cargo test --workspace

# Boot test
timeout 15 cargo xtask run --term --headless

# Verify brush makes more progress
# Expected: Gets past signal setup, may hit other issues
```

## Handoff Notes

### What Was Fixed

1. `sys_sigaction` now properly reads `struct sigaction` from userspace pointers
2. All sigaction fields (handler, flags, restorer, mask) are parsed and stored
3. Old action is written back to oldact pointer if provided

### What Remains

After fixing rt_sigaction, brush may encounter other issues:

| Feature | Priority | Notes |
|---------|----------|-------|
| waitid syscall | HIGH | For job control |
| Terminal ioctls | MEDIUM | TCGETS, TCSETS, TIOCGPGRP |
| /proc/self/fd | LOW | FD enumeration |
| /etc/passwd | LOW | User info |

### Files Modified

| File | Change |
|------|--------|
| `crates/kernel/syscall/src/signal.rs` | Rewrote sys_sigaction |
| `crates/kernel/syscall/src/lib.rs` | Updated dispatcher args |
| `crates/kernel/sched/src/lib.rs` | Added SignalAction struct (if needed) |

### How to Verify Fix

```bash
# Quick verification
cargo xtask build kernel && cargo xtask build iso
timeout 15 cargo xtask run --term --headless 2>&1 | grep -E "sigaction|SIGCHLD|EXCEPTION"

# Expected: No crash at 0x6aa71f, brush makes more syscalls
```

### Known Limitations

1. Signal mask is stored but not fully enforced during signal delivery
2. SA_SIGINFO three-argument handlers not fully implemented
3. Realtime signals (32-64) not tested

## Completion Checklist

- [ ] Phase 4 implementation complete
- [ ] All tests pass
- [ ] Golden logs updated (if needed)
- [ ] Team file updated with completion status
- [ ] Brush makes more progress (verified)
- [ ] Handoff notes complete

---

## Phase 5 Status: PENDING

Waiting for Phase 4 implementation to complete.
