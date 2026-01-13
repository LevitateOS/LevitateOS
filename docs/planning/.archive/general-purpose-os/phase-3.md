# Phase 3: Implementation - General Purpose OS

**TEAM_400**: General Purpose Unix-Compatible OS
**Created**: 2026-01-10
**Status**: Ready for Implementation

---

## Implementation Overview

### Dependency Graph

```
                    ┌─────────────────┐
                    │ sys_fork        │
                    │ (High Priority) │
                    └────────┬────────┘
                             │ depends on
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
    ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
    │ Page Table  │  │ FD Table    │  │ Process     │
    │ Duplication │  │ Clone       │  │ Table       │
    └─────────────┘  └─────────────┘  └─────────────┘
              │
              ▼
    ┌─────────────────┐
    │ sys_execve      │
    │ (High Priority) │
    └────────┬────────┘
             │ uses
             ▼
    ┌─────────────────┐
    │ ELF Loader      │
    │ (Existing)      │
    └─────────────────┘

    ┌─────────────────┐     ┌─────────────────┐
    │ sys_chdir       │     │ sys_poll        │
    │ (Low Effort)    │     │ (Medium)        │
    └─────────────────┘     └─────────────────┘
            │                       │
            ▼                       ▼
    ┌─────────────────┐     ┌─────────────────┐
    │ VFS Path        │     │ sys_ppoll       │
    │ (Existing)      │     │ (Existing)      │
    └─────────────────┘     └─────────────────┘
```

### Implementation Order

| Order | Component | Files | Effort | Depends On |
|-------|-----------|-------|--------|------------|
| 1 | Simple syscalls (uname, umask) | `syscall/process.rs`, `syscall/fs/` | Low | Nothing |
| 2 | chdir/fchdir | `syscall/fs/dir.rs` | Low | VFS |
| 3 | chmod/chown stubs | `syscall/fs/` | Low | VFS |
| 4 | poll (wrapper around ppoll) | `syscall/sync.rs` | Low | ppoll |
| 5 | select | `syscall/sync.rs` | Medium | ppoll |
| 6 | **fork** | `syscall/process.rs`, `memory/`, `task/` | High | Page tables |
| 7 | **execve** | `syscall/process.rs`, `task/process.rs` | High | ELF loader, fork |
| 8 | c-gull staticlib integration | `xtask/`, userspace | Medium | All above |

---

## Design Reference

See: `docs/planning/general-purpose-os/phase-2.md`

### Resolved Design Decisions

| ID | Decision | Rationale |
|----|----------|-----------|
| **Q1** | Fork uses **eager copy** | Simplicity first (Rule 20), optimize to CoW later |
| **Q2** | execve uses **VFS path resolution** | Standard behavior, shell handles PATH searching |
| **Q3** | Orphans **reparent to PID 1** | Standard Unix semantics |
| **Q4** | Implement **both poll() and ppoll()** | Maximum compatibility |
| **Q5** | **Track FD_CLOEXEC per-fd** | Close on exec, prevents FD leaks |
| **Q6** | chmod/chown are **no-op** (just succeed) | Single-user OS, permissions not enforced |
| **Q7** | Reset signals **except SIG_IGN** on exec | Linux behavior, nohup compatibility |
| **Q8** | pselect6 **deferred** | YAGNI, add when needed |
| **Q9** | argv+envp max **256KB** | Matches Linux ARG_MAX |
| **Q10** | getppid returns **1 for orphans** | Consistent with Q3 reparenting |

---

## Implementation Details

### Unit of Work 1: Simple Syscalls

**Files**: `crates/kernel/src/syscall/process.rs`

```rust
// sys_uname implementation
pub fn sys_uname(buf: usize) -> i64 {
    let task = current_task();

    // Validate buffer
    let size = core::mem::size_of::<Utsname>();
    if validate_user_buffer(task.ttbr0, buf, size, true).is_err() {
        return errno::EFAULT;
    }

    let utsname = Utsname {
        sysname: str_to_array("LevitateOS"),
        nodename: str_to_array("levitate"),
        release: str_to_array(env!("CARGO_PKG_VERSION")),
        version: str_to_array(concat!(env!("CARGO_PKG_VERSION"), " ", env!("BUILD_DATE"))),
        machine: str_to_array(MACHINE_NAME),  // "x86_64" or "aarch64"
        domainname: str_to_array("(none)"),
    };

    // Copy to user
    write_to_user(task.ttbr0, buf, &utsname);
    0
}

// sys_umask implementation
pub fn sys_umask(mask: u32) -> i64 {
    let task = current_task();
    let old = task.umask.swap(mask & 0o777, Ordering::SeqCst);
    old as i64
}
```

**Estimated effort**: 1-2 hours

---

### Unit of Work 2: chdir/fchdir

**Files**: `crates/kernel/src/syscall/fs/dir.rs`

```rust
pub fn sys_chdir(path_ptr: usize) -> i64 {
    let task = current_task();

    // Read path from user
    let mut buf = [0u8; PATH_MAX];
    let path = match read_user_cstring(task.ttbr0, path_ptr, &mut buf) {
        Ok(p) => p,
        Err(e) => return e,
    };

    // Resolve path and verify it's a directory
    let resolved = match resolve_path(&task.cwd.lock(), path) {
        Ok(p) => p,
        Err(_) => return errno::ENOENT,
    };

    // Check if directory exists (via VFS)
    if !is_directory(&resolved) {
        return errno::ENOTDIR;
    }

    // Update CWD
    *task.cwd.lock() = resolved;
    0
}

pub fn sys_fchdir(fd: i32) -> i64 {
    let task = current_task();
    let fd_table = task.fd_table.lock();

    let entry = match fd_table.get(fd as usize) {
        Some(e) => e,
        None => return errno::EBADF,
    };

    // Get path from fd (if it's a directory)
    let path = match &entry.fd_type {
        FdType::VfsFile(vfs_file) => {
            if !vfs_file.is_directory() {
                return errno::ENOTDIR;
            }
            vfs_file.path().to_string()
        }
        _ => return errno::ENOTDIR,
    };

    drop(fd_table);
    *task.cwd.lock() = path;
    0
}
```

**Estimated effort**: 2-4 hours

---

### Unit of Work 3: chmod/chown (No-op per Q6)

**Files**: `crates/kernel/src/syscall/fs/mod.rs` (new file or extend existing)

**Decision**: Per Q6, chmod/chown are **no-op** implementations. LevitateOS is single-user (always root) and doesn't enforce permissions, so storing modes adds complexity with no functional benefit.

```rust
pub fn sys_chmod(pathname: usize, mode: u32) -> i64 {
    let task = current_task();

    let mut buf = [0u8; PATH_MAX];
    let path = match read_user_cstring(task.ttbr0, pathname, &mut buf) {
        Ok(p) => p,
        Err(e) => return e,
    };

    // Verify file exists (Q6: no-op but validate path)
    match resolve_path(&task.cwd.lock(), path) {
        Ok(_) => 0,  // Success - no actual mode change (single-user OS)
        Err(_) => errno::ENOENT,
    }
}

pub fn sys_chown(pathname: usize, owner: u32, group: u32) -> i64 {
    let task = current_task();

    let mut buf = [0u8; PATH_MAX];
    let path = match read_user_cstring(task.ttbr0, pathname, &mut buf) {
        Ok(p) => p,
        Err(e) => return e,
    };

    // Verify file exists (Q6: no-op but validate path)
    match resolve_path(&task.cwd.lock(), path) {
        Ok(_) => 0,  // Success - no actual ownership change (single-user OS)
        Err(_) => errno::ENOENT,
    }
}

// fchmod, fchown similar - verify fd exists and succeed
```

**Estimated effort**: 1-2 hours

---

### Unit of Work 4: poll (Wrapper)

**Files**: `crates/kernel/src/syscall/sync.rs`

```rust
// poll is essentially ppoll with simpler timeout handling
pub fn sys_poll(fds: usize, nfds: usize, timeout_ms: i32) -> i64 {
    // Convert ms timeout to ppoll format, then call sys_ppoll
    // timeout_ms: -1 = infinite, 0 = non-blocking, >0 = ms

    sys_ppoll(fds, nfds, 0 /* tmo_ptr - ignored */, 0 /* sigmask - ignored */)
    // Note: Current ppoll ignores timeout anyway, so this is equivalent
}
```

**Estimated effort**: 1 hour

---

### Unit of Work 5: select

**Files**: `crates/kernel/src/syscall/sync.rs`

```rust
pub fn sys_select(
    nfds: i32,
    readfds: usize,
    writefds: usize,
    exceptfds: usize,
    timeout: usize,
) -> i64 {
    // select is more complex than poll
    // Need to convert fd_set bitmasks to pollfd array

    let task = current_task();
    let ttbr0 = task.ttbr0;

    // Read fd_sets from user space
    let read_set = if readfds != 0 {
        read_fdset(ttbr0, readfds, nfds)?
    } else {
        FdSet::empty()
    };

    // ... similar for writefds, exceptfds

    // Convert to poll and call existing infrastructure
    // ...

    // Write back modified fd_sets
    // ...
}
```

**Estimated effort**: 4-6 hours

---

### Unit of Work 6: fork (High Complexity)

**Files**:
- `crates/kernel/src/syscall/process.rs`
- `crates/kernel/src/memory/vmm.rs` (or page_table.rs)
- `crates/kernel/src/task/mod.rs`

**Decisions Applied**:
- **Q1**: Eager copy (not CoW) - copy all writable pages immediately
- **Q3**: Child registered with parent_pid for later reparenting to PID 1 if orphaned
- **Q5**: FD table clone includes per-fd flags (CLOEXEC)

```rust
pub fn sys_fork() -> i64 {
    let parent = current_task();

    // 1. Allocate new PID
    let child_pid = allocate_pid();

    // 2. Clone page tables (Q1: eager copy, not CoW)
    let child_ttbr0 = match clone_page_tables_eager(parent.ttbr0) {
        Ok(ttbr) => ttbr,
        Err(_) => return errno::ENOMEM,
    };

    // 3. Clone FD table (Q5: includes per-fd CLOEXEC flags)
    let child_fds = parent.fd_table.lock().clone_with_flags();

    // 4. Clone signal handlers
    let child_signals = parent.signal_handlers.lock().clone();

    // 5. Create child TCB
    let child = TaskControlBlock {
        id: TaskId(child_pid),
        pid: ProcessId(child_pid),
        ttbr0: child_ttbr0,
        fd_table: IrqSafeLock::new(child_fds),
        signal_handlers: IrqSafeLock::new(child_signals),
        cwd: IrqSafeLock::new(parent.cwd.lock().clone()),
        umask: AtomicU32::new(parent.umask.load(Ordering::Relaxed)),
        // ... copy other fields

        // Child-specific (Q3: track parent for orphan reparenting):
        parent_pid: AtomicUsize::new(parent.id.0),
    };

    // 6. Set child's return value to 0
    child.set_syscall_return(0);

    // 7. Register in process table (Q3: enables reparenting on parent exit)
    register_process(child_pid, parent.id.0, Arc::new(child.clone()));

    // 8. Add child to scheduler
    SCHEDULER.add_task(Arc::new(child));

    // 9. Return child PID to parent
    child_pid as i64
}

// Helper: Clone page tables (eager copy)
fn clone_page_tables(parent_ttbr0: usize) -> Result<usize, MemoryError> {
    // Allocate new page table root
    let child_ttbr0 = allocate_page_table()?;

    // Walk parent's page tables
    // For each mapped page:
    //   - Allocate new physical page
    //   - Copy contents
    //   - Map in child's page table with same permissions

    // ... implementation details

    Ok(child_ttbr0)
}
```

**Estimated effort**: 16-24 hours (most complex syscall)

---

### Unit of Work 7: execve (High Complexity)

**Files**:
- `crates/kernel/src/syscall/process.rs`
- `crates/kernel/src/task/process.rs`

**Decisions Applied**:
- **Q2**: VFS path resolution (shell handles PATH searching)
- **Q5**: Close FD_CLOEXEC file descriptors
- **Q7**: Reset signal handlers except SIG_IGN
- **Q9**: Check argv+envp total size < 256KB (return E2BIG)

```rust
pub fn sys_execve(pathname: usize, argv: usize, envp: usize) -> i64 {
    let task = current_task();

    // 1. Read pathname
    let mut path_buf = [0u8; PATH_MAX];
    let path = match read_user_cstring(task.ttbr0, pathname, &mut path_buf) {
        Ok(p) => p,
        Err(e) => return e,
    };

    // 2. Read argv array (Q9: track size)
    let (args, args_size) = match read_string_array_with_size(task.ttbr0, argv) {
        Ok(a) => a,
        Err(e) => return e,
    };

    // 3. Read envp array (Q9: track size)
    let (env, env_size) = match read_string_array_with_size(task.ttbr0, envp) {
        Ok(e) => e,
        Err(e) => return e,
    };

    // Q9: Check combined size limit (256KB = Linux ARG_MAX)
    const ARG_MAX: usize = 256 * 1024;
    if args_size + env_size > ARG_MAX {
        return errno::E2BIG;
    }

    // 4. Find and load ELF (Q2: VFS path resolution)
    let elf_data = match vfs_resolve_and_read(path, &task.cwd.lock()) {
        Ok(data) => data,
        Err(_) => return errno::ENOENT,
    };

    // 5. Validate ELF
    if !is_valid_elf(&elf_data) {
        return errno::ENOEXEC;
    }

    // 6. Close FD_CLOEXEC file descriptors (Q5)
    close_cloexec_fds(&mut task.fd_table.lock());

    // 7. Reset signal handlers (Q7: all except SIG_IGN)
    reset_signal_handlers_except_ignored(&mut task.signal_handlers.lock());

    // 8. Destroy old address space
    destroy_user_address_space(task.ttbr0);

    // 9. Load new program (reuse existing ELF loader)
    let new_ttbr0 = match load_elf_and_setup_stack(&elf_data, &args, &env) {
        Ok(ttbr) => ttbr,
        Err(_) => {
            // Fatal: can't recover, must exit
            task_exit();
            unreachable!()
        }
    };

    // 10. Update task
    task.ttbr0 = new_ttbr0;

    // 11. Return to new entry point (does not return here)
    // Set up registers and jump to entry
    jump_to_user(entry_point, stack_pointer);

    unreachable!()
}
```

**Estimated effort**: 12-20 hours

---

### Unit of Work 8: c-gull Staticlib Integration

**Files**:
- `xtask/src/build.rs`
- New: `crates/userspace/libc-test/` (test programs)

```bash
# Build c-gull as staticlib
cd external/c-gull
cargo build --release --features "take-charge,malloc-via-crates"

# Result: target/release/libc_gull.a

# Create sysroot
mkdir -p sysroot/lib
cp libc_gull.a sysroot/lib/libc.a

# Compile test program
gcc -static -nostdlib -L sysroot/lib -lc -o hello hello.c
```

**Estimated effort**: 8-12 hours (includes debugging linking issues)

---

## File Change Summary

| File | Changes | Priority |
|------|---------|----------|
| `crates/kernel/src/syscall/mod.rs` | Add syscall dispatcher entries | P0 |
| `crates/kernel/src/syscall/process.rs` | Add fork, execve, uname, umask | P0 |
| `crates/kernel/src/syscall/fs/dir.rs` | Add chdir, fchdir | P1 |
| `crates/kernel/src/syscall/fs/mod.rs` | Add chmod, chown stubs | P1 |
| `crates/kernel/src/syscall/sync.rs` | Add poll, select | P1 |
| `crates/kernel/src/memory/vmm.rs` | Page table cloning | P0 |
| `crates/kernel/src/task/mod.rs` | TCB extensions | P0 |
| `crates/kernel/src/task/process.rs` | execve integration | P0 |
| `xtask/src/build.rs` | c-gull build commands | P2 |

---

## Test Approach

### Per-Syscall Tests

```rust
// In kernel test suite
#[test]
fn test_uname() {
    let mut buf = Utsname::default();
    let ret = sys_uname(&mut buf as *mut _ as usize);
    assert_eq!(ret, 0);
    assert!(buf.sysname.starts_with(b"LevitateOS"));
}

#[test]
fn test_chdir_valid() {
    sys_chdir("/tmp\0".as_ptr() as usize);
    // Verify CWD changed
}

#[test]
fn test_fork_child_gets_zero() {
    let ret = sys_fork();
    if ret == 0 {
        // Child
        sys_exit(42);
    } else {
        // Parent
        let mut status = 0;
        sys_waitpid(ret as i32, &mut status as *mut _ as usize);
        assert_eq!(status, 42);
    }
}
```

### Integration Test

```c
// hello.c - Compile with gcc -static
#include <unistd.h>
#include <sys/wait.h>

int main() {
    pid_t pid = fork();
    if (pid == 0) {
        // Child
        write(1, "Child\n", 6);
        _exit(0);
    } else {
        // Parent
        wait(NULL);
        write(1, "Parent\n", 7);
    }
    return 0;
}
```

---

## Resolved Blockers

All Phase 2 design questions have been answered. Implementation can proceed.

| Question | Decision | Status |
|----------|----------|--------|
| Q1: Eager vs CoW | **Eager Copy** | ✅ Resolved |
| Q2: execve path resolution | **VFS resolution** | ✅ Resolved |
| Q3: Orphan reparenting | **Yes, to PID 1** | ✅ Resolved |
| Q4: poll vs ppoll | **Both** | ✅ Resolved |
| Q5: FD_CLOEXEC | **Track per-fd flag** | ✅ Resolved |
| Q6: chmod metadata | **No-op** | ✅ Resolved |
| Q7: Signal reset | **All except SIG_IGN** | ✅ Resolved |
| Q8: pselect6 | **Deferred** | ✅ Resolved |
| Q9: argv+envp limit | **256KB** | ✅ Resolved |
| Q10: getppid for orphans | **Return 1** | ✅ Resolved |

See `docs/questions/TEAM_400_general_purpose_os.md` for full rationale.

---

## References

- Phase 2 Design: `docs/planning/general-purpose-os/phase-2.md`
- Existing clone implementation: `crates/kernel/src/syscall/process.rs:481`
- Existing ELF loader: `crates/kernel/src/task/process.rs`
- Linux syscall reference: man pages
