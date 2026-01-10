# Phase 5: Hardening and Handoff

**TEAM_411** | Syscall Abstractions Refactor
**Parent**: `phase-4.md`
**Status**: Planning

---

## 1. Final Verification

### 1.1 Linux ABI Compliance Check

Verify all syscalls still match Linux behavior:

| Check | Method |
|-------|--------|
| Syscall numbers | Compare against Linux headers |
| Argument order | Review each syscall signature |
| Return values | Test success and error cases |
| errno values | Verify match Linux constants |
| Struct layouts | Compare sizeof and field offsets |

### 1.2 Full Test Suite

```bash
# 1. Build all targets
cargo build

# 2. Run Eyra behavior tests
./tests/eyra_behavior_test.sh

# 3. Compare golden output (should be identical)
diff tests/eyra_output.txt tests/eyra_output_golden.txt

# 4. Manual boot test
cargo xtask run
# Exercise: ls, cat, echo, mkdir, etc.
```

### 1.3 Architecture Parity

Verify both architectures work:
- [ ] x86_64 build passes
- [ ] aarch64 build passes
- [ ] x86_64 boots and runs
- [ ] aarch64 boots and runs (if testable)

---

## 2. Documentation Updates

### 2.1 Update SYSCALL_REQUIREMENTS.md

Add section documenting new abstractions:
```markdown
## Syscall Helper Abstractions

### VfsError Conversion
All VfsError values now convert to errno via `impl From<VfsError> for i64`.

### UserSlice<T>
Safe wrapper for user-space buffer access. Use instead of raw validate+copy.

### get_fd / get_vfs_file
Helper functions for fd lookup. Return Result<_, i64> with errno.

### resolve_at_path
Proper dirfd resolution for *at() syscalls. Handles AT_FDCWD and relative paths.
```

### 2.2 Update Code Comments

Ensure new helpers have doc comments explaining:
- Purpose
- Usage example
- Error conditions
- Linux ABI compliance notes

### 2.3 Architecture Docs

Update `docs/ARCHITECTURE.md` if syscall module structure changed.

---

## 3. Performance Sanity Check

New abstractions should have zero or minimal overhead:

| Abstraction | Expected Overhead |
|-------------|-------------------|
| VfsError → errno | Zero (compile-time) |
| UserSlice | Zero (same operations, different organization) |
| get_fd | Zero (same lock + clone) |
| resolve_at_path | Minimal (one extra string allocation for path joining) |

If performance concerns arise, measure with:
```bash
# Syscall latency comparison before/after
# (if benchmarking infrastructure exists)
```

---

## 4. Handoff Checklist

### 4.1 Code Quality
- [ ] All new code has doc comments
- [ ] No `unwrap()` on user-provided data
- [ ] All `unsafe` blocks have SAFETY comments
- [ ] No compiler warnings

### 4.2 Testing
- [ ] Build passes (both architectures)
- [ ] Eyra behavior tests pass (golden unchanged)
- [ ] Manual boot test passes
- [ ] No regressions in syscall behavior

### 4.3 Documentation
- [ ] SYSCALL_REQUIREMENTS.md updated
- [ ] New helpers documented
- [ ] Team file updated with completion status

### 4.4 Clean State
- [ ] No TODO(TEAM_411) items remaining (or documented in global TODO)
- [ ] No temporary code left behind
- [ ] Git history clean (logical commits)

---

## 5. Team File Update

Update `.teams/TEAM_411_refactor_syscall_abstractions.md`:

```markdown
## Completion Status

**Status**: Completed
**Date**: YYYY-MM-DD

## Summary of Changes

1. Added `impl From<VfsError> for i64` for consistent error mapping
2. Added `UserSlice<T>` for safe user-space buffer access
3. Added `get_fd()` / `get_vfs_file()` helpers
4. Added `resolve_at_path()` for proper dirfd support
5. Migrated ~X syscalls to use new abstractions
6. Removed ~Y lines of duplicate code

## Files Changed

- `crates/kernel/src/syscall/helpers.rs` (new)
- `crates/kernel/src/syscall/mod.rs` (updated exports)
- `crates/kernel/src/syscall/fs/*.rs` (migrated)
- `crates/kernel/src/fs/vfs/error.rs` (From impl)

## Linux ABI Verification

All syscall signatures and behaviors unchanged. Verified via:
- Eyra behavior tests (golden match)
- Manual boot testing

## Known Limitations

- resolve_at_path requires VfsFile to store path (added in Phase 2)
- SyscallContext abstraction deferred (optional)
```

---

## 6. Future Recommendations

For next teams working on syscalls:

1. **Use the new helpers** — Don't revert to inline patterns
2. **Extend UserSlice** — Add methods as needed (e.g., `read_cstring`)
3. **Add more helpers** — If a pattern repeats 3+ times, abstract it
4. **Maintain Linux ABI** — Always verify against Linux behavior

---

## Exit Criteria

- [ ] All Phase 5 verification passes
- [ ] Documentation updated
- [ ] Team file marked complete
- [ ] Handoff checklist complete
- [ ] No blocking issues for future work

---

## Refactor Complete

This concludes the syscall abstractions refactor plan.
