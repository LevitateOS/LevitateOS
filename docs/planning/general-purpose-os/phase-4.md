# Phase 4: Integration - General Purpose OS

**TEAM_400**: General Purpose Unix-Compatible OS
**Created**: 2026-01-10
**Status**: Ready for Implementation

---

## Integration Points

### Syscall Dispatcher Integration

**File**: `crates/kernel/src/syscall/mod.rs`

New syscall numbers to register (from `los_abi`):

| Syscall | x86_64 | AArch64 | Handler |
|---------|--------|---------|---------|
| fork | 57 | 1079* | `sys_fork` |
| execve | 59 | 221 | `sys_execve` |
| uname | 63 | 160 | `sys_uname` |
| umask | 95 | 166 | `sys_umask` |
| chdir | 80 | 49 | `sys_chdir` |
| fchdir | 81 | 50 | `sys_fchdir` |
| chmod | 90 | - | `sys_chmod` |
| fchmodat | 268 | 53 | `sys_fchmodat` |
| chown | 92 | - | `sys_chown` |
| fchownat | 260 | 54 | `sys_fchownat` |
| poll | 7 | - | `sys_poll` |
| select | 23 | - | `sys_select` |

*Note: pselect6 deferred per Q8 decision (YAGNI - add when needed).*

*Note: AArch64 doesn't have `fork` syscall; uses `clone` with specific flags.

```rust
// In syscall dispatcher match
match syscall_num {
    // Existing...

    // New process syscalls
    SyscallNumber::Fork => sys_fork(),
    SyscallNumber::Execve => sys_execve(a0, a1, a2),

    // New fs syscalls
    SyscallNumber::Chdir => sys_chdir(a0),
    SyscallNumber::Fchdir => sys_fchdir(a0 as i32),
    SyscallNumber::Chmod => sys_chmod(a0, a1 as u32),
    SyscallNumber::Fchmodat => sys_fchmodat(a0 as i32, a1, a2 as u32, a3 as i32),
    SyscallNumber::Chown => sys_chown(a0, a1 as u32, a2 as u32),
    SyscallNumber::Fchownat => sys_fchownat(a0 as i32, a1, a2 as u32, a3 as u32, a4 as i32),

    // System info
    SyscallNumber::Uname => sys_uname(a0),
    SyscallNumber::Umask => sys_umask(a0 as u32),

    // I/O multiplexing (Q4: both poll and ppoll for compatibility)
    SyscallNumber::Poll => sys_poll(a0, a1, a2 as i32),
    SyscallNumber::Select => sys_select(a0 as i32, a1, a2, a3, a4),
    // Note: pselect6 deferred per Q8

    // ...
}
```

### ABI Crate Updates

**File**: `crates/abi/src/lib.rs`

Verify all new syscall numbers are present:

```rust
// Ensure los_abi has these variants
pub enum SyscallNumber {
    // Existing...
    Fork = /* verify */,
    Execve = /* verify */,
    Chdir = /* verify */,
    // ...
}
```

### Task Management Integration

**File**: `crates/kernel/src/task/mod.rs`

```rust
impl TaskControlBlock {
    // New method for fork
    pub fn fork(&self) -> Result<Arc<TaskControlBlock>, ProcessError> {
        // Called from sys_fork
        // ...
    }

    // New method for execve
    pub fn exec(&self, path: &str, argv: &[&str], envp: &[&str]) -> Result<(), ProcessError> {
        // Called from sys_execve
        // ...
    }
}
```

### Process Table Integration

**File**: `crates/kernel/src/task/process_table.rs`

**Decisions Applied**:
- **Q3**: Orphan reparenting to PID 1
- **Q10**: getppid returns 1 for reparented orphans

```rust
impl ProcessTable {
    // Ensure these work with fork:
    pub fn register_fork_child(&mut self, parent_pid: usize, child_pid: usize, child: Arc<TCB>);

    // Q3/Q10: Orphan reparenting to PID 1
    pub fn reparent_orphans(&mut self, dead_parent_pid: usize) {
        // Find all children of dead_parent_pid
        // Set their parent_pid to 1 (init)
        // getppid() will now return 1 for these processes
    }
}
```

### Memory Management Integration

**File**: `crates/kernel/src/memory/vmm.rs` or `crates/hal/src/mmu.rs`

```rust
// New public API for fork
pub fn clone_address_space(parent_ttbr0: usize) -> Result<usize, MemoryError>;

// New public API for execve
pub fn destroy_address_space(ttbr0: usize);
pub fn create_fresh_address_space() -> Result<usize, MemoryError>;
```

### VFS Integration

**Files**: `crates/kernel/src/fs/mod.rs`, `crates/kernel/src/fs/vfs.rs`

```rust
// Ensure VFS supports:
pub fn resolve_path(cwd: &str, path: &str) -> Result<String, VfsError>;
pub fn is_directory(path: &str) -> bool;
pub fn file_exists(path: &str) -> bool;

// For execve:
pub fn read_file(path: &str) -> Result<Vec<u8>, VfsError>;
```

---

## Test Strategy

### Unit Tests

#### Location: `crates/kernel/src/syscall/tests/`

```rust
// test_process_syscalls.rs
mod fork_tests {
    #[test]
    fn fork_creates_child_process() { ... }

    #[test]
    fn fork_child_returns_zero() { ... }

    #[test]
    fn fork_parent_returns_child_pid() { ... }

    #[test]
    fn fork_child_inherits_fds() { ... }

    #[test]
    fn fork_child_inherits_cwd() { ... }

    #[test]
    fn fork_enomem_on_oom() { ... }
}

mod execve_tests {
    #[test]
    fn execve_replaces_address_space() { ... }

    #[test]
    fn execve_enoent_on_missing_file() { ... }

    #[test]
    fn execve_enoexec_on_bad_elf() { ... }

    #[test]
    fn execve_closes_cloexec_fds() { ... }  // Q5

    #[test]
    fn execve_passes_argv_correctly() { ... }

    #[test]
    fn execve_e2big_on_large_args() { ... }  // Q9: 256KB limit

    #[test]
    fn execve_resets_signals_except_sig_ign() { ... }  // Q7
}

// test_fs_syscalls.rs
mod chdir_tests {
    #[test]
    fn chdir_updates_cwd() { ... }

    #[test]
    fn chdir_enoent_on_missing() { ... }

    #[test]
    fn chdir_enotdir_on_file() { ... }
}
```

#### Location: Host-side tests with mocks

```rust
// tests/syscall_signatures.rs
#[test]
fn verify_syscall_numbers_match_linux() {
    // Compare los_abi values against linux-raw-sys
}
```

### Behavior Tests

#### New Golden Files

```
tests/golden/
├── x86_64/
│   ├── fork_exec_lifecycle.txt     # NEW
│   ├── chdir_propagation.txt       # NEW
│   └── poll_select_events.txt      # NEW
└── aarch64/
    ├── fork_exec_lifecycle.txt     # NEW
    ├── chdir_propagation.txt       # NEW
    └── poll_select_events.txt      # NEW
```

#### Behavior Test Scenarios

```yaml
# tests/behavior/fork_exec.yaml
name: fork_exec_lifecycle
description: Test fork creates child, exec replaces it
steps:
  - boot kernel with verbose
  - run /bin/fork_test
  - expect: "Parent PID: \d+"
  - expect: "Child PID: \d+"
  - expect: "Child exec'd successfully"
  - expect: "Parent waited, child exit=0"
```

### Integration Tests

#### Static Binary Test

```bash
# tests/integration/static_binary.sh

# Compile test program on host
x86_64-linux-musl-gcc -static -o hello tests/programs/hello.c

# Add to initramfs
cp hello initramfs/bin/

# Boot and run
cargo xtask vm exec "/bin/hello"

# Verify output
assert_output_contains "Hello, World!"
```

#### Fork/Exec Test

```c
// tests/programs/fork_exec_test.c
#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>

int main() {
    pid_t pid = fork();

    if (pid == 0) {
        // Child
        char *argv[] = {"/bin/echo", "child", NULL};
        execve("/bin/echo", argv, NULL);
        _exit(1);  // execve failed
    } else if (pid > 0) {
        // Parent
        int status;
        waitpid(pid, &status, 0);
        printf("Child exited with %d\n", WEXITSTATUS(status));
    } else {
        perror("fork");
        return 1;
    }

    return 0;
}
```

### Regression Tests

#### ABI Compatibility

```rust
// tests/regress/abi_compat.rs

#[test]
fn struct_sizes_match_linux() {
    assert_eq!(size_of::<Utsname>(), 390);  // 6 * 65 bytes
    assert_eq!(size_of::<Pollfd>(), 8);
    assert_eq!(size_of::<Timeval>(), 16);
}

#[test]
fn errno_values_match_linux() {
    assert_eq!(ENOENT, 2);
    assert_eq!(ENOSYS, 38);
    // ...
}
```

---

## Impact Analysis

### Affected Subsystems

| Subsystem | Impact | Risk |
|-----------|--------|------|
| Task scheduler | Low | fork adds tasks |
| Memory manager | High | Page table cloning |
| VFS | Low | Path resolution |
| Syscall dispatcher | Low | New entries |
| Process table | Medium | Fork/orphan handling |
| Signal handling | Medium | Reset on exec |

### Breaking Changes

| Change | Impact | Mitigation |
|--------|--------|------------|
| TCB struct size increase | Binary incompatibility | Rebuild all |
| New syscall numbers | None (additive) | - |
| Memory layout changes | Potential instability | Thorough testing |

### Performance Impact

| Operation | Impact | Notes |
|-----------|--------|-------|
| fork | New overhead | Page table walk + copy |
| execve | Similar to spawn | Already have ELF loader |
| Other syscalls | Negligible | Simple implementations |

---

## Verification Checklist

### Before Merge

- [ ] All new syscalls registered in dispatcher
- [ ] Syscall numbers verified against linux-raw-sys
- [ ] Unit tests pass
- [ ] Behavior tests pass
- [ ] Static binary test works
- [ ] Fork/exec lifecycle test works
- [ ] No regressions in existing tests
- [ ] Both architectures (x86_64, aarch64) tested

### After Merge

- [ ] CI passes on both architectures
- [ ] Golden files updated if behavior changed
- [ ] Documentation updated

---

## Rollback Plan

If integration causes issues:

1. **Immediate**: Revert merge commit
2. **Investigation**: Isolate failing component
3. **Fix forward**: Address specific issue
4. **Re-attempt**: With fixes applied

New syscalls are additive, so rollback primarily affects fork/execve which touch core subsystems.

---

## References

- Phase 3 Implementation: `docs/planning/general-purpose-os/phase-3.md`
- Syscall ABI: `crates/abi/src/lib.rs`
- Existing tests: `tests/golden/`, `crates/kernel/src/**/tests.rs`
