# Phase 3: Implementation

**Parent**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Not Started

## Implementation Overview

### Files to Create
| File | Description |
|------|-------------|
| `kernel/src/fs/pipe.rs` | Pipe struct and ring buffer |
| `kernel/src/syscall/pipe.rs` | sys_pipe2 handler |
| `kernel/src/syscall/fd.rs` | sys_dup, sys_dup3 handlers |

### Files to Modify
| File | Change |
|------|--------|
| `kernel/src/fs/mod.rs` | Add `pub mod pipe` |
| `kernel/src/syscall/mod.rs` | Add syscall numbers and dispatch |
| `kernel/src/task/fd_table.rs` | Add dup operations |
| `userspace/libsyscall/src/lib.rs` | Add pipe2, dup, dup3 wrappers |

### Order of Implementation
1. Step 1: Pipe kernel object
2. Step 2: sys_pipe2 syscall
3. Step 3: sys_dup and sys_dup3 syscalls
4. Step 4: Userspace wrappers

---

## Step 1: Pipe Kernel Object

### UoW 1.1: Implement Ring Buffer

**File**: `kernel/src/fs/pipe.rs`

**Objective**: Create simple ring buffer for pipe data.

**Tasks**:
1. Create `RingBuffer<T>` struct with fixed capacity
2. Implement `push()`, `pop()`, `is_empty()`, `is_full()`
3. Use atomic indices for thread safety

**Exit Criteria**: RingBuffer compiles and unit tests pass.

---

### UoW 1.2: Implement Pipe Struct

**File**: `kernel/src/fs/pipe.rs`

**Objective**: Create Pipe object with read/write ends.

**Tasks**:
1. Create `Pipe` struct:
   ```rust
   pub struct Pipe {
       buffer: IrqSafeLock<RingBuffer<u8>>,
       read_open: AtomicBool,
       write_open: AtomicBool,
   }
   ```
2. Create `PipeReadEnd` and `PipeWriteEnd` wrappers
3. Implement `read()`, `write()`, `close_read()`, `close_write()`

**Exit Criteria**: Pipe object compiles.

---

### UoW 1.3: Implement VfsFile for Pipe Ends

**File**: `kernel/src/fs/pipe.rs`

**Objective**: Make pipe ends work with VFS file operations.

**Tasks**:
1. Implement `VfsFile` for `PipeReadEnd`:
   - `read()` → reads from pipe buffer
   - `write()` → returns error (read-only)
   - `close()` → marks read end closed
2. Implement `VfsFile` for `PipeWriteEnd`:
   - `read()` → returns error (write-only)
   - `write()` → writes to pipe buffer
   - `close()` → marks write end closed

**Exit Criteria**: VfsFile implementations compile.

---

## Step 2: sys_pipe2 Syscall

### UoW 2.1: Implement sys_pipe2

**File**: `kernel/src/syscall/pipe.rs`

**Objective**: Create pipe2 syscall handler.

**Tasks**:
1. Add syscall number: `SYS_PIPE2 = 59`
2. Implement `sys_pipe2(pipefd, flags)`:
   - Create Pipe
   - Allocate two fds in current task's fd table
   - Write fds to user memory at pipefd
   - Handle O_CLOEXEC, O_NONBLOCK flags
3. Wire into syscall dispatch

**Exit Criteria**: pipe2 callable from userspace.

---

## Step 3: sys_dup and sys_dup3 Syscalls

### UoW 3.1: Implement sys_dup

**File**: `kernel/src/syscall/fd.rs`

**Objective**: Implement basic fd duplication.

**Tasks**:
1. Add syscall number: `SYS_DUP = 23`
2. Implement `sys_dup(oldfd)`:
   - Get file handle from fd table
   - Find lowest available fd
   - Clone reference to same file
3. Wire into syscall dispatch

**Exit Criteria**: dup creates fd copy.

---

### UoW 3.2: Implement sys_dup3

**File**: `kernel/src/syscall/fd.rs`

**Objective**: Implement dup to specific fd.

**Tasks**:
1. Add syscall number: `SYS_DUP3 = 24`
2. Implement `sys_dup3(oldfd, newfd, flags)`:
   - If oldfd == newfd, return -EINVAL
   - If newfd open, close it
   - Copy file from oldfd to newfd
   - Handle O_CLOEXEC
3. Wire into syscall dispatch

**Exit Criteria**: dup3 works for redirection.

---

## Step 4: Userspace Wrappers

### UoW 4.1: Add Wrappers to libsyscall

**File**: `userspace/libsyscall/src/lib.rs`

**Objective**: Add userspace syscall wrappers.

**Tasks**:
1. Add constants: `SYS_PIPE2 = 59`, `SYS_DUP = 23`, `SYS_DUP3 = 24`
2. Add flag: `O_CLOEXEC = 0x80000`
3. Add wrappers:
   ```rust
   pub fn pipe2(pipefd: &mut [i32; 2], flags: u32) -> isize
   pub fn dup(oldfd: i32) -> isize
   pub fn dup2(oldfd: i32, newfd: i32) -> isize  // via dup3
   pub fn dup3(oldfd: i32, newfd: i32, flags: u32) -> isize
   ```

**Exit Criteria**: Wrappers compile.

---

## Deliverables
- [ ] Ring buffer implementation
- [ ] Pipe kernel object with VfsFile trait
- [ ] sys_pipe2 syscall
- [ ] sys_dup syscall
- [ ] sys_dup3 syscall
- [ ] libsyscall wrappers
