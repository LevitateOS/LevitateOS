# Phase 6: Process Orchestration (pipe2, dup) (P2)

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: P2 — Required for `std::process::Command`

## Purpose
Implement pipe and file descriptor duplication for process communication and I/O redirection.

## Success Criteria
- `sys_pipe2` creates pipe pairs
- `sys_dup`, `sys_dup3` duplicate file descriptors
- Shell can do `cmd1 | cmd2` style piping
- `std::process::Command` with piped stdin/stdout works

## Reference
- `relibc`: `src/header/unistd/` for pipe/dup
- Linux pipe2(2), dup2(2) man pages

---

## Step 1: Pipe Implementation

### UoW 6.1.1: Design Pipe Data Structure
**File**: `phase-6-step-1-uow-1.md`

**Objective**: Design kernel pipe structure.

**Tasks**:
1. Design pipe structure:
   ```rust
   pub struct Pipe {
       buffer: RingBuffer<u8>,  // or fixed array
       read_open: bool,
       write_open: bool,
       // waiters for read/write blocking
   }
   ```
2. Decide buffer size (typically 64KB or 4KB for simplicity)
3. Document blocking behavior:
   - Read blocks when empty (unless O_NONBLOCK)
   - Write blocks when full

**Exit Criteria**: Design documented.

---

### UoW 6.1.2: Implement Pipe Object
**File**: `phase-6-step-1-uow-2.md`

**Objective**: Implement pipe kernel object.

**Tasks**:
1. Implement `Pipe` struct with:
   - `read(&mut buf) -> isize` — read from buffer
   - `write(&buf) -> isize` — write to buffer
   - `close_read()` — mark read end closed
   - `close_write()` — mark write end closed (triggers SIGPIPE/EPIPE)
2. Handle EOF: read returns 0 when write end closed and buffer empty

**Exit Criteria**: Pipe object compiles.

---

### UoW 6.1.3: Implement sys_pipe2
**File**: `phase-6-step-1-uow-3.md`

**Objective**: Implement pipe2 syscall.

**Tasks**:
1. Create `sys_pipe2(pipefd, flags)`:
   ```rust
   pub fn sys_pipe2(pipefd: *mut [i32; 2], flags: u32) -> isize
   ```
2. Implementation:
   - Create new Pipe object
   - Allocate two file descriptors: read_fd, write_fd
   - Write [read_fd, write_fd] to `*pipefd`
   - Handle flags (O_CLOEXEC, O_NONBLOCK)
3. Add syscall number: `SYS_PIPE2 = 59`

**Exit Criteria**: pipe2 creates working pipe.

---

### UoW 6.1.4: Wire Pipe to File Operations
**File**: `phase-6-step-1-uow-4.md`

**Objective**: Make pipe work with read/write syscalls.

**Tasks**:
1. Integrate Pipe into file descriptor table
2. When `sys_read(pipe_fd)` called → delegate to Pipe::read
3. When `sys_write(pipe_fd)` called → delegate to Pipe::write
4. When `sys_close(pipe_fd)` called → call close_read or close_write

**Exit Criteria**: Pipes work with standard read/write.

---

## Step 2: File Descriptor Duplication

### UoW 6.2.1: Implement sys_dup
**File**: `phase-6-step-2-uow-1.md`

**Objective**: Implement basic dup.

**Tasks**:
1. Create `sys_dup(oldfd)`:
   - Find lowest available fd number
   - Copy file descriptor entry from oldfd
   - Both fds now refer to same underlying file/pipe
2. Add syscall number: `SYS_DUP = 23`

**Exit Criteria**: dup creates copy of fd.

---

### UoW 6.2.2: Implement sys_dup3
**File**: `phase-6-step-2-uow-2.md`

**Objective**: Implement dup3 (dup2 with flags).

**Tasks**:
1. Create `sys_dup3(oldfd, newfd, flags)`:
   ```rust
   pub fn sys_dup3(oldfd: i32, newfd: i32, flags: u32) -> isize
   ```
2. Implementation:
   - If newfd already open, close it first
   - Copy file descriptor from oldfd to newfd
   - Handle O_CLOEXEC flag
3. Add syscall number: `SYS_DUP3 = 24`
4. Note: `dup2` is often implemented via `dup3(old, new, 0)`

**Exit Criteria**: dup3 works for redirection.

---

## Step 3: Userspace Integration

### UoW 6.3.1: Add Pipe/Dup to libsyscall
**File**: `phase-6-step-3-uow-1.md`

**Objective**: Add userspace wrappers.

**Tasks**:
1. Add to libsyscall:
   ```rust
   pub fn pipe2(pipefd: &mut [i32; 2], flags: u32) -> isize
   pub fn dup(oldfd: i32) -> isize
   pub fn dup2(oldfd: i32, newfd: i32) -> isize
   pub fn dup3(oldfd: i32, newfd: i32, flags: u32) -> isize
   ```
2. Add flag constants (O_CLOEXEC = 0x80000)

**Exit Criteria**: Wrappers compile.

---

### UoW 6.3.2: Add Pipe Test
**File**: `phase-6-step-3-uow-2.md`

**Objective**: Test pipe functionality.

**Tasks**:
1. Create test that:
   - Creates pipe with pipe2
   - Spawns child process
   - Parent writes to pipe
   - Child reads from pipe
   - Verifies data transferred correctly
2. Test EOF behavior (close write end, read returns 0)

**Exit Criteria**: Pipe test passes.

---

### UoW 6.3.3: Add Dup/Redirect Test
**File**: `phase-6-step-3-uow-3.md`

**Objective**: Test fd duplication.

**Tasks**:
1. Create test that:
   - Opens file for writing
   - Uses dup2 to redirect stdout to file
   - Prints something
   - Verifies output went to file

**Exit Criteria**: Dup/redirect test passes.

---

## Deliverables
- Pipe kernel object
- `sys_pipe2` implementation
- `sys_dup`, `sys_dup3` implementations
- libsyscall wrappers
- Pipe and redirect tests
