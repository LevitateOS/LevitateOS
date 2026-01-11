# Phase 3: Fix Design and Validation Plan

**Bug**: `execve()` syscall is a stub returning ENOSYS  
**Team**: TEAM_436  
**Status**: Design Complete

---

## Root Cause Summary

`sys_exec()` in `syscall/src/process/lifecycle.rs` is a stub that returns ENOSYS after loading the executable but before actually executing it.

---

## Fix Strategy

### High-Level Approach

Implement execve by **modifying the current task in-place**:

1. Load ELF binary (already works)
2. Create new page table with ELF segments mapped
3. Set up new stack with argv/envp/auxv (reuse spawn infrastructure)
4. Close file descriptors with O_CLOEXEC flag
5. Reset signal handlers to SIG_DFL
6. Swap task's page table (ttbr0) to new one
7. Free old address space
8. Set syscall frame registers to new entry point
9. Return from syscall → execution continues in new program

### Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Approach | Modify task in-place | Naturally preserves pid, ppid, cwd, fds |
| Stack setup | Reuse spawn_args code | Avoid duplication |
| FD handling | Add close_cloexec() | Clean separation of concerns |
| Signal reset | Clear handler map | Simple, matches Linux behavior |

---

## API Design

### New/Modified Functions

```rust
// In syscall/src/process/lifecycle.rs
pub fn sys_execve(
    path_ptr: usize,      // Pointer to path string
    argv_ptr: usize,      // Pointer to argv array (null-terminated)
    envp_ptr: usize,      // Pointer to envp array (null-terminated)
) -> SyscallResult;

// In sched/src/fd_table.rs
impl FdTable {
    /// Close all file descriptors with O_CLOEXEC flag
    pub fn close_cloexec(&mut self);
}

// In mm/src/user/page_table.rs (if needed)
pub fn free_user_address_space(ttbr0: PhysAddr);
```

### Syscall Signature Change

Current stub:
```rust
pub fn sys_exec(path_ptr: usize, path_len: usize) -> SyscallResult
```

Linux-compatible:
```rust
pub fn sys_execve(path_ptr: usize, argv_ptr: usize, envp_ptr: usize) -> SyscallResult
```

**Note**: The current `sys_exec` takes a length-prefixed path (LevitateOS custom). Linux execve takes a null-terminated path. We should support both:
- Keep `sys_exec` for backwards compatibility (custom syscall 1000?)
- Add `sys_execve` for Linux ABI compatibility (syscall 59/221)

---

## Reversal Strategy

If the fix causes issues:

1. **Revert syscall dispatcher** to call stub again
2. **Keep new helper functions** (they don't break anything)
3. **Signal**: Any panic or hang during execve indicates need to revert

The fix is isolated to syscall handling - it doesn't change core data structures.

---

## Test Strategy

### New Tests

1. **Basic execve test**: fork() → execve("/echo", ["echo", "test"]) → verify output
2. **Argv passing test**: Verify arguments reach the new program
3. **FD inheritance test**: Open file → execve → verify fd still open
4. **O_CLOEXEC test**: Open with O_CLOEXEC → execve → verify fd closed
5. **CWD preservation test**: chdir → execve → verify cwd unchanged

### Existing Tests

All existing tests must continue to pass:
```bash
cargo xtask test behavior
cargo xtask test unit
```

---

## Impact Analysis

### API Changes

| Change | Impact |
|--------|--------|
| New `sys_execve` function | Additive - no breakage |
| `FdTable::close_cloexec()` | Additive - no breakage |
| Syscall dispatcher update | Maps new syscall number |

### Behavior Changes

| Before | After |
|--------|-------|
| execve returns ENOSYS | execve replaces process image |
| Programs use spawn() | Programs can use fork()+exec() |

### Risks

| Risk | Mitigation |
|------|------------|
| Memory leak from old address space | Explicitly free old pages |
| Register state corruption | Carefully set all frame registers |
| Dual-arch issues | Test on both x86_64 and aarch64 |

---

## Implementation Order

1. **Step 1**: Add `close_cloexec()` to FdTable
2. **Step 2**: Create helper to load ELF and prepare new address space
3. **Step 3**: Create helper to set up argv/envp stack
4. **Step 4**: Implement `sys_execve()` tying it all together
5. **Step 5**: Update syscall dispatcher
6. **Step 6**: Add tests
7. **Step 7**: Verify on both architectures
