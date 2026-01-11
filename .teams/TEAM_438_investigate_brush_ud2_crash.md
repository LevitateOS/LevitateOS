# TEAM_438: Investigate Brush ud2 Crash

## Objective

Continue investigation of TEAM_437's blocking issue: Brush shell crashes with INVALID OPCODE (ud2).

## Status: MULTIPLE FIXES APPLIED - CRASH PERSISTS (DIFFERENT LOCATION)

## Summary of Progress

The original crash (GLOB_DAT relocation) was fixed. Brush now makes many syscalls successfully, including:
- mprotect, mmap, arch_prctl, sigaction, clone, getuid, etc.

**Current crash**: After clone() creates a child thread (TID 3) and parent calls getuid(), the parent crashes at ud2 (0x6aa71f) without calling exit_group through our syscall handler.

## Fixes Implemented

### 1. GLOB_DAT Relocation Fix (Original Crash)
- **File**: `toolchain/libc-levitateos/src/lib.rs`
- **Change**: Added `posix_spawn_file_actions_addchdir` stub forwarding to `_np` version

### 2. fcntl F_DUPFD_CLOEXEC (1030)
- **File**: `crates/kernel/syscall/src/fs/fd.rs`
- **Change**: Added handling for F_DUPFD_CLOEXEC command

### 3. socketpair syscall (53)
- **Files**: `crates/kernel/arch/x86_64/src/lib.rs`, `crates/kernel/syscall/src/lib.rs`, `crates/kernel/syscall/src/sync.rs`
- **Change**: Added Socketpair syscall number and implementation using pipe pair

### 4. x86_64 TLS Setup Before Userspace Entry
- **File**: `crates/kernel/sched/src/lib.rs`
- **Change**: Added FS_BASE MSR write in `user_task_entry_wrapper` for x86_64 (was only done for aarch64)

### 5. x86_64 exception_return Implementation
- **File**: `crates/kernel/arch/x86_64/src/lib.rs`
- **Change**: Implemented `exception_return` (was `unimplemented!()`) - restores SyscallFrame and uses sysretq

### 6. Thread Context Fix for x86_64
- **File**: `crates/kernel/sched/src/thread.rs`
- **Change**: Set `context.rip` directly to `exception_return` on x86_64 (bypassing `task_entry_trampoline` which corrupts RSP)

## Current Crash Analysis

**Symptom**: 
- Clone creates child thread (TID 3) successfully
- Parent continues with getuid → returns 0
- Parent crashes at ud2 (0x6aa71f) without exit_group syscall logged
- Child thread is NEVER scheduled before crash

**Crash Location**: 0x6aa71f (file offset 0x69a71f)
```asm
69a719:       syscall        ; exit_group
69a71b:       ud2
69a71d:       ud2
69a71f:       ud2            ; <- CRASH HERE (third ud2, not first!)
```

**Key Observation**: Crash is at the THIRD ud2, not the first after syscall. This means brush is JUMPING to this location, not reaching it via syscall return.

## Remaining Investigation Needed

1. **Why does brush jump to the ud2?** - Something in brush's async runtime or thread initialization is panicking
2. **Why doesn't exit_group reach our handler?** - The crash might be a Rust panic abort that goes directly to ud2
3. **Child thread setup** - The child thread (TID 3) was never scheduled; exception_return might still have issues

## Files Modified

| File | Change |
|------|--------|
| `toolchain/libc-levitateos/src/lib.rs` | Added `posix_spawn_file_actions_addchdir` stub |
| `crates/kernel/syscall/src/fs/fd.rs` | Added F_DUPFD_CLOEXEC handling |
| `crates/kernel/arch/x86_64/src/lib.rs` | Added Socketpair enum, implemented exception_return |
| `crates/kernel/syscall/src/lib.rs` | Added Socketpair dispatcher |
| `crates/kernel/syscall/src/sync.rs` | Added sys_socketpair implementation |
| `crates/kernel/sched/src/lib.rs` | Added x86_64 TLS setup |
| `crates/kernel/sched/src/thread.rs` | Fixed context.rip for x86_64 threads |

## Handoff Notes

- Original GLOB_DAT crash is FIXED - brush now makes many syscalls
- Several kernel improvements made for threading support

## ROOT CAUSE IDENTIFIED (Session 2)

**The crash is caused by `rt_sigaction` syscall format mismatch!**

### The Problem

Linux `rt_sigaction` expects **pointers to sigaction structs**:
```c
int rt_sigaction(int signum, 
                 const struct sigaction *act,    // POINTER
                 struct sigaction *oldact,       // POINTER
                 size_t sigsetsize);
```

Our implementation treats the args as **direct values**:
```rust
pub fn sys_sigaction(sig: i32, handler_addr: usize, restorer_addr: usize)
```

### Why Brush Crashes

1. Tokio creates multi-thread runtime, spawns worker via clone() ✅
2. Tokio calls rt_sigaction to set up SIGCHLD handler
3. We interpret struct pointer as handler address → garbage stored
4. Tokio's signal setup silently fails
5. Brush async runtime panics → jumps to ud2

### Fix Required

Rewrite `sys_sigaction` to:
1. Read `struct sigaction` from userspace pointer (arg1)
2. Parse `sa_handler`, `sa_flags`, `sa_restorer`, `sa_mask`
3. Write old sigaction to `oldact` pointer (arg2) if non-null
4. Handle `SA_RESTORER` flag properly

See detailed analysis: `docs/planning/brush-requirements/BRUSH_REQUIREMENTS.md`

