# Phase 1: Discovery and Safeguards

**Status**: Ready for execution
**TEAM**: TEAM_415
**Source**: TEAM_414 Panic Mitigation Checklist

---

## Refactor Summary

Replace unsafe `unwrap()`, `expect()`, and `panic!()` calls in kernel-critical paths with proper error handling. Focus on code paths that can be triggered by userspace input (syscalls) where a panic would crash the entire kernel.

### Pain Points

1. **Syscall paths use `unwrap()` on user memory operations** - A validation bug could crash the kernel
2. **`Tmpfs::root()` panics** if called before initialization - Should return `Option` or `Result`
3. **`current_task()` panics** before scheduler init - API should be safer
4. **`unimplemented!()` in x86_64** - Will panic if called

### Motivation

A general-purpose OS cannot panic on malformed userspace input. Every syscall path must handle errors gracefully and return appropriate error codes.

---

## Success Criteria

### Before

```rust
// Syscall handler - panics on validation bug
let dest = mm_user::user_va_to_kernel_ptr(task.ttbr0, addr).unwrap();
```

### After

```rust
// Syscall handler - returns EFAULT on validation bug
let dest = mm_user::user_va_to_kernel_ptr(task.ttbr0, addr)
    .ok_or(SyscallError::Fault)?;
```

---

## Behavioral Contracts

| Contract | Requirement |
|----------|-------------|
| Syscall error codes | All syscalls must return proper errno on failure, never panic |
| Kernel stability | Malformed userspace input cannot crash the kernel |
| API compatibility | Public function signatures may change (breaking changes allowed per Rule 5) |

---

## Golden/Regression Tests

| Test | Location | Purpose |
|------|----------|---------|
| Eyra behavior tests | `tests/eyra_behavior_test.sh` | Verify syscall behavior unchanged |
| Build verification | `cargo build --release` | Must compile |

**Note**: Silver mode is active - golden logs can be updated if behavior improves.

---

## Current Architecture

### Syscall Error Handling Pattern

Current pattern in `crates/kernel/src/syscall/`:

```rust
// 1. Validate user buffer
validate_user_buffer(task, addr, size)?;

// 2. Get kernel pointer (UNSAFE - unwrap)
let ptr = mm_user::user_va_to_kernel_ptr(task.ttbr0, addr).unwrap();

// 3. Perform operation
unsafe { core::ptr::copy(...) }
```

The gap: `validate_user_buffer` returns `Result`, but `user_va_to_kernel_ptr` returns `Option`. The `unwrap()` assumes validation was sufficient.

### Affected Files

| File | `unwrap()` count | Priority |
|------|------------------|----------|
| `src/syscall/process.rs` | 2 | P0 |
| `src/syscall/time.rs` | 3 | P0 |
| `src/syscall/sys.rs` | 1 | P0 |
| `src/syscall/fs/stat.rs` | 1 | P0 |
| `src/syscall/fs/fd.rs` | 3 | P0 |
| `src/syscall/fs/dir.rs` | 1 | P0 |
| `src/syscall/fs/statx.rs` | 1 | P0 |
| `src/syscall/fs/read.rs` | 1 | P0 |
| `src/syscall/fs/write.rs` | 5 | P0 |

---

## Constraints

1. **No behavior changes** - Syscalls must return same results for valid inputs
2. **Error codes must be correct** - Invalid pointers → `EFAULT`
3. **Performance** - Error path is cold; optimize for success path
4. **Breaking changes allowed** - Per Rule 5, prefer clean breaks over compatibility hacks

---

## Open Questions

None - TEAM_414 already audited all panic locations.

---

## Steps

### Step 1: Verify Current Test Baseline

**File**: `phase-1-step-1.md` (inline - small enough)

1. Run `cargo build --release` - must pass
2. Run eyra behavior tests - capture baseline
3. Document any pre-existing failures

### Step 2: Design Error Handling Strategy

**File**: `phase-1-step-2.md` (inline - small enough)

1. Define `SyscallError` enum with proper errno mapping
2. Define pattern for `user_va_to_kernel_ptr` error handling
3. Document the replacement pattern for all syscall files

---

## Exit Criteria

- [x] Build passes
- [ ] Error handling strategy documented
- [ ] Replacement pattern defined
- [ ] Ready for Phase 2 implementation
