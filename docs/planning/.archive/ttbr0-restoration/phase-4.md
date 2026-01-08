# Phase 4: Integration & Testing — TTBR0 Restoration

## Test Strategy

### 1. Regression Tests

| Test | Command | Expected |
|------|---------|----------|
| Behavior test | `cargo xtask test behavior` | Pass, no crash |
| Unit tests | `cargo xtask test unit` | Pass |
| USER EXCEPTION check | (in behavior test) | No crashes detected |

### 2. New Functional Tests

#### Test A: Yield During Syscall

**Scenario:**
1. Shell (PID=2) calls `sys_read`
2. No input available → yields to init (PID=1)
3. Timer fires → scheduler runs
4. Shell scheduled again → resumes syscall
5. Input available → returns to userspace

**Expected:** Shell works normally, accepts input.

#### Test B: Multiple Concurrent Tasks

**Scenario:**
1. Init spawns shell
2. Shell spawns background task
3. Shell waits for input (yields)
4. Background task runs
5. Input arrives → shell resumes

**Expected:** Both tasks run, no crashes.

---

## Verification Checklist

- [ ] `cargo check` passes
- [ ] `cargo xtask test behavior` passes
- [ ] Shell accepts keyboard input
- [ ] No USER EXCEPTION detected
- [ ] GPU flush count reasonable (42-50)
- [ ] No kernel panics during I/O wait

---

## Performance Validation

| Metric | Before | After | Acceptable? |
|--------|--------|-------|-------------|
| Syscall overhead | Baseline | +2 instructions | Yes |
| TLB flushes | Per switch | Same | Yes |
| Context switch time | Baseline | Same | Yes |

---

## Rollback Plan

If issues found:
1. Remove yield_now from sys_read (revert to TEAM_145 fix)
2. Keep TTBR0 save/restore for future use
