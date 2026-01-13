# Phase 2: Root Cause Analysis

**Bug**: `execve()` syscall is a stub returning ENOSYS  
**Team**: TEAM_436  
**Status**: Analysis Complete

---

## Root Cause

The root cause is straightforward: **execve() was never implemented**.

```rust
// @crates/kernel/syscall/src/process/lifecycle.rs:200-217
pub fn sys_exec(path_ptr: usize, path_len: usize) -> SyscallResult {
    let path_len = path_len.min(256);
    let task = los_sched::current_task();
    let mut path_buf = [0u8; 256];
    let path = crate::copy_user_string(task.ttbr0, path_ptr, path_len, &mut path_buf)?;
    log::trace!("[SYSCALL] exec('{}')", path);
    let _elf_data = resolve_initramfs_executable(path)?;
    log::warn!("[SYSCALL] exec is currently a stub");
    Err(ENOSYS)  // <-- STUB: Returns error instead of executing
}
```

The code:
1. Reads the path from userspace ✓
2. Resolves the executable from initramfs ✓
3. **Then does nothing with it and returns ENOSYS** ✗

---

## Why It Was Left as a Stub

Historical context from code comments:
- `arch/aarch64/src/lib.rs:122-126`: "Custom LevitateOS syscalls (temporary, until clone/execve work)"
- The `spawn` and `spawn_args` syscalls were implemented as temporary workarounds
- TEAM_432 implemented `fork()` but the corresponding `exec()` was never completed

---

## What Needs to Happen

execve() must:

1. **Load the new executable** (already done: `resolve_initramfs_executable()`)
2. **Tear down current address space** (new code needed)
3. **Create new address space with ELF segments** (exists in `spawn_from_elf`)
4. **Set up new stack with argv/envp/auxv** (exists in spawn code)
5. **Handle file descriptors** (close O_CLOEXEC fds)
6. **Reset signal handlers** (new code needed)
7. **Update task state** (pc, sp, registers)
8. **Return to userspace at new entry point** (critical - different from normal syscall return)

---

## Key Insight: Reuse spawn_from_elf Infrastructure

The existing `spawn_from_elf()` function in `process/mod.rs` already:
- Parses ELF headers
- Maps segments into address space
- Sets up auxiliary vector
- Configures initial stack with argv

The difference for execve:
- `spawn_from_elf()` creates a **new task**
- `execve()` **replaces the current task's** address space and state

---

## Implementation Strategy

Two approaches:

### Approach A: Modify Current Task In-Place (Recommended)

1. Load ELF and prepare new page table (reuse spawn infrastructure)
2. Close O_CLOEXEC file descriptors
3. Reset signal handlers to default
4. Swap current task's ttbr0 to new page table
5. Set frame registers (pc, sp, etc.) to new entry point
6. Return from syscall → lands in new program

**Pros**: Preserves pid, ppid, fd table, cwd naturally  
**Cons**: Must carefully tear down old address space

### Approach B: Create New Task, Transfer Identity

1. Create new task via spawn_from_elf
2. Copy over pid, ppid, fd_table, cwd, etc.
3. Terminate old task
4. Insert new task with same pid

**Pros**: Simpler - reuses more existing code  
**Cons**: More complex pid/identity management

**Decision**: Use Approach A - modify current task in-place.

---

## Files to Modify

| File | Changes |
|------|---------|
| `syscall/src/process/lifecycle.rs` | Replace stub with full implementation |
| `mm/src/user/page_table.rs` | Add `destroy_user_address_space()` if needed |
| `sched/src/fd_table.rs` | Add `close_cloexec()` method |
| `sched/src/lib.rs` | Possibly add task mutation methods |

---

## Dependencies

- ELF loading: ✅ Already works (`spawn_from_elf`)
- Page table creation: ✅ Already works
- Argv stack setup: ✅ Already works (in spawn_args)
- Auxv setup: ✅ Already works

No external dependencies need to be added.
