# TEAM_436: Investigate Missing Crucial Components

## Objective
Investigate whether LevitateOS is missing any crucial components for its goal of becoming a general-purpose Unix-compatible OS.

## Status: INVESTIGATION COMPLETE

## Progress Log
### Session 1 (2026-01-11)
- Registered as TEAM_436
- Read CLAUDE.md and MASTER_PLAN_GENERAL_PURPOSE_OS.md
- Reviewed syscall dispatcher (los_syscall/src/lib.rs)
- Checked fork implementation (TEAM_432)
- Checked exec implementation - **FOUND CRITICAL GAP**
- Checked devtmpfs implementation
- Checked ext4 implementation
- Reviewed FHS compliance status

---

## CRITICAL FINDINGS

### BUG #1: `execve()` is a STUB (CRITICAL - BLOCKING)

**Location**: `@/crates/kernel/syscall/src/process/lifecycle.rs:200-217`

**Evidence**:
```rust
pub fn sys_exec(path_ptr: usize, path_len: usize) -> SyscallResult {
    // ... validation ...
    log::warn!("[SYSCALL] exec is currently a stub");
    Err(ENOSYS)
}
```

**Impact**: 
- **fork()+exec() pattern DOES NOT WORK**
- Programs cannot spawn other programs using standard Unix semantics
- The system relies on custom `spawn`/`spawn_args` syscalls (numbers 1000/1001)
- This breaks the entire "general purpose OS" goal

**Root Cause**: exec was never fully implemented. The comment in arch files says "temporary, until clone/execve work" but execve was never completed.

**Severity**: P0 CRITICAL - Blocks Epic 1 (Process Model)

---

### BUG #2: `fchdir()` is a STUB

**Location**: `@/crates/kernel/syscall/src/fs/fd.rs:374-377`

**Evidence**:
```rust
pub fn sys_fchdir(_fd: usize) -> SyscallResult {
    // TODO: Implement when directory fd tracking is added
    Err(ENOSYS)
}
```

**Impact**: Programs using fchdir() will fail. Some coreutils may use this.

**Severity**: P2 Medium

---

### BUG #3: ext4 is READ-ONLY (CRITICAL - BLOCKING)

**Location**: `@/crates/kernel/fs/ext4/src/lib.rs:1-5`

**Evidence**:
```rust
//! TEAM_032: Uses ext4-view for read-only ext4 support.
//! This is designed for the root filesystem partition.
```

The ext4 implementation uses `ext4-view` crate which only supports reading. There's a `write_block` method in the trait but it's never used.

**Impact**:
- Cannot create files on ext4 filesystem
- Cannot install to disk
- Cannot persist changes across reboot
- **Blocks Epic 3 (Disk-Based Root Filesystem)**

**Root Cause**: The master plan notes "ext4 write support is a prerequisite" and "This is blocking work" but it was never implemented.

**Severity**: P0 CRITICAL - Blocks Epic 3

---

### BUG #4: Missing /dev/stdin, /dev/stdout, /dev/stderr

**Location**: devtmpfs only creates null, zero, full, urandom

**Evidence** from `@/crates/kernel/fs/devtmpfs/src/devices/mod.rs:88-94`:
```rust
pub fn register_builtin_devices() {
    register_char_device(MEM_MAJOR, NULL_MINOR, &null::NULL_DEVICE);
    register_char_device(MEM_MAJOR, ZERO_MINOR, &zero::ZERO_DEVICE);
    register_char_device(MEM_MAJOR, FULL_MINOR, &full::FULL_DEVICE);
    register_char_device(MEM_MAJOR, URANDOM_MINOR, &urandom::URANDOM_DEVICE);
}
```

**Impact**: 
- /dev/stdin, /dev/stdout, /dev/stderr don't exist
- /dev/fd/ directory doesn't exist
- Many shell scripts and programs expect these

**Master Plan says**: "Extended (13+) - /dev/stdin, stdout, stderr widely used"

**Severity**: P1 High - Blocks Epic 2 FHS compliance

---

### BUG #5: No /proc filesystem

**Evidence**: grep for "procfs" and "/proc/self" returns no implementation

**Impact**:
- `std::env::current_exe()` fails (needs /proc/self/exe)
- Many programs introspect /proc/self/fd, /proc/self/maps, etc.

**Master Plan says**: "Minimal /proc/self - /proc/self/exe needed for std::env::current_exe()"

**Severity**: P1 High - Blocks Epic 2

---

### BUG #6: No pivot_root() syscall

**Evidence**: grep for "pivot_root" returns no results

**Impact**: Cannot switch root filesystem for disk boot

**Master Plan says**: "pivot_root - Linux-compatible syscall (155/41)"

**Severity**: P1 High - Blocks Epic 3

---

### BUG #7: No setuid/setgid/setgroups syscalls

**Evidence**: grep for "setuid|setgid|setgroups" returns no implementations

**Impact**:
- Cannot change process credentials
- No multi-user support
- No privilege escalation (su)

**Master Plan says**: Epic 4 (Users & Permissions) depends on these

**Severity**: P1 High - Blocks Epic 4

---

### BUG #8: Init uses custom syscalls, not fork/exec

**Location**: `@/crates/userspace/init/src/main.rs:10,126`

**Evidence**:
```rust
use libsyscall::{spawn, spawn_args, ...};
// ...
let shell_pid = spawn("brush");
```

**Impact**: 
- Init cannot demonstrate that fork/exec works
- The "process model" isn't actually being exercised
- This masks the fact that exec() is broken

**Severity**: P2 Medium - symptom of BUG #1

---

## Summary Table

| Bug | Component | Severity | Blocks |
|-----|-----------|----------|--------|
| #1 | execve() stub | P0 CRITICAL | Epic 1 |
| #3 | ext4 read-only | P0 CRITICAL | Epic 3 |
| #4 | Missing /dev/stdin,stdout,stderr | P1 High | Epic 2 |
| #5 | No /proc filesystem | P1 High | Epic 2 |
| #6 | No pivot_root() | P1 High | Epic 3 |
| #7 | No setuid/setgid | P1 High | Epic 4 |
| #2 | fchdir() stub | P2 Medium | - |
| #8 | Init uses spawn not fork/exec | P2 Medium | - |

---

## Recommendations

### Immediate Priority (P0)

1. **Implement execve()** - This is the most critical gap. Without it, fork() is useless.
   - Must load ELF, set up new address space, transfer control
   - Should handle argv/envp on stack
   - TEAM_432 implemented fork, execve should follow same patterns

2. **Implement ext4 write support** OR **switch to simpler writable FS**
   - ext4 write is complex (journaling, extent trees, etc.)
   - Alternative: Use minixfs or simple custom FS first
   - Or implement FAT32 write (simpler, already have FAT read)

### High Priority (P1)

3. **Create /dev/stdin, /dev/stdout, /dev/stderr** as symlinks to /proc/self/fd/0,1,2
   - But this requires /proc first!
   - Alternative: Create them as special device nodes that redirect to current task's fd 0,1,2

4. **Implement minimal /proc/self**
   - /proc/self/exe - symlink to executable path
   - /proc/self/fd/ - directory of fd symlinks
   - /proc/self/cwd - symlink to cwd

5. **Implement pivot_root()** - Relatively straightforward syscall

6. **Implement setuid/setgid family** - Need credential infrastructure first

---

## Handoff Notes

The investigation reveals that LevitateOS has significant gaps between the master plan and actual implementation:

- **Epic 1 (Process Model)**: fork() works, but exec() is a stub. The epic is NOT complete.
- **Epic 2 (FHS)**: devtmpfs works with 4 devices, but missing /dev/stdin,stdout,stderr and /proc
- **Epic 3 (Disk Root)**: Completely blocked by ext4 read-only limitation
- **Epic 4 (Users/Permissions)**: Not started (as documented)

The master plan's checkmarks (✅) for fork() and devtmpfs are accurate, but the system still cannot run standard Unix programs because exec() doesn't work.

**Next team should prioritize implementing execve() - it's the single most important missing piece.**

---

## Bugfix Plan Created

Created comprehensive bugfix plan at `docs/planning/bugfix-execve/`:

| Phase | File | Status |
|-------|------|--------|
| Phase 1 | `phase-1.md` | Understanding and Scoping ✅ |
| Phase 2 | `phase-2.md` | Root Cause Analysis ✅ |
| Phase 3 | `phase-3.md` | Fix Design and Validation Plan ✅ |
| Phase 4 | `phase-4.md` | Implementation Steps (8-10 UoWs) ✅ |
| Phase 5 | `phase-5.md` | Cleanup and Handoff ✅ |

**Ready for implementation.**

---

## Implementation Complete (Session 2)

### Files Modified

| File | Change |
|------|--------|
| `crates/kernel/arch/aarch64/src/lib.rs` | Added `set_pc()` method to SyscallFrame |
| `crates/kernel/levitate/src/process.rs` | Added `ExecImage` struct and `prepare_exec_image()` function |
| `crates/kernel/levitate/src/init.rs` | Registered `PREPARE_EXEC_IMAGE_HOOK` |
| `crates/kernel/syscall/src/process/lifecycle.rs` | Implemented `sys_execve()`, `execve_internal()`, string parsing helpers |
| `crates/kernel/syscall/src/process/mod.rs` | Exported `sys_execve` and `PREPARE_EXEC_IMAGE_HOOK` |
| `crates/kernel/syscall/src/lib.rs` | Updated dispatcher to call `sys_execve` with frame |
| `crates/kernel/mm/src/heap.rs` | Added `reset()` method to ProcessHeap |

### Implementation Summary

1. **ExecImage struct**: Contains prepared address space state (ttbr0, entry_point, stack_pointer, initial_brk, tls_base)
2. **prepare_exec_image()**: Loads ELF, creates page tables, sets up stack with argv/envp/auxv, returns ExecImage
3. **sys_execve()**: Linux-compatible syscall that:
   - Reads null-terminated path, argv, envp from userspace
   - Calls prepare_exec_image via hook
   - Switches TTBR0/CR3 to new page table
   - Updates TLS register
   - Resets heap state
   - Modifies syscall frame PC/SP to jump to new entry point
4. **Hook mechanism**: Follows existing pattern to avoid circular dependencies between los_syscall and levitate

### Remaining TODOs

- [ ] `TODO(TEAM_436): Close O_CLOEXEC file descriptors` - FdTable needs O_CLOEXEC tracking
- [ ] `TODO(TEAM_436): Reset signal handlers to default` - Signal infrastructure needed
- [ ] Free old address space pages (currently just switch, may leak)

### Verification

- ✅ x86_64 builds successfully
- ✅ aarch64 builds successfully  
- ✅ Behavior tests pass
- ✅ No new regressions introduced

### Handoff Checklist

- [x] Project builds cleanly (both architectures)
- [x] All tests pass
- [x] Team file updated
- [ ] O_CLOEXEC handling (deferred - requires FdTable enhancement)
- [ ] Signal reset on exec (deferred - requires signal infrastructure)
