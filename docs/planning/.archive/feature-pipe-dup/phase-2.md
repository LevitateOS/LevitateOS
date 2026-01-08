# Phase 2: Design

**Parent**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Not Started

## Purpose
Define the solution architecture for pipe and dup syscalls.

---

## Proposed Solution

### Pipe Data Structure

```rust
pub struct Pipe {
    buffer: RingBuffer<u8>,     // Fixed-size circular buffer
    read_open: AtomicBool,      // Is read end still open?
    write_open: AtomicBool,     // Is write end still open?
    read_waiters: WaitQueue,    // Tasks waiting to read
    write_waiters: WaitQueue,   // Tasks waiting to write
}
```

**Buffer Size**: 4096 bytes (one page, simple for MVP)

### Pipe Ends

```rust
pub struct PipeReadEnd(Arc<Pipe>);
pub struct PipeWriteEnd(Arc<Pipe>);
```

Both implement `VfsFile` trait for unified fd handling.

---

## API Design

### sys_pipe2

```rust
pub fn sys_pipe2(pipefd: *mut [i32; 2], flags: u32) -> isize
```

**Behavior**:
1. Create new Pipe object
2. Allocate two fds: `pipefd[0]` = read, `pipefd[1]` = write
3. Handle `O_CLOEXEC` and `O_NONBLOCK` flags
4. Return 0 on success, negative errno on failure

**Syscall Number**: 59 (Linux aarch64)

### sys_dup

```rust
pub fn sys_dup(oldfd: i32) -> isize
```

**Behavior**:
1. Find lowest available fd
2. Copy fd entry from oldfd
3. Return new fd or -EBADF

**Syscall Number**: 23 (Linux aarch64)

### sys_dup3

```rust  
pub fn sys_dup3(oldfd: i32, newfd: i32, flags: u32) -> isize
```

**Behavior**:
1. If newfd open, close it first
2. Copy fd from oldfd to newfd
3. Handle O_CLOEXEC flag
4. Return newfd or negative errno

**Syscall Number**: 24 (Linux aarch64)

---

## Behavioral Decisions

### Q1: What happens when reading from an empty pipe?
**Answer**: Block until data available OR write end closed (return 0 = EOF)

### Q2: What happens when writing to a full pipe?
**Answer**: Block until space available OR read end closed (return -EPIPE)

### Q3: What if O_NONBLOCK is set?
**Answer**: Return -EAGAIN instead of blocking

### Q4: Buffer size?
**Decision**: 4096 bytes for MVP. Linux uses 64KB but page-size is simpler.

### Q5: Should dup increment a reference count?
**Answer**: Yes, dup creates a second reference to the same underlying file/pipe.

---

## Design Alternatives

### Alternative A: Fixed Array Buffer
- Simple implementation
- Fixed size (4KB or 64KB)
- **Chosen for MVP**

### Alternative B: Dynamically Growing Buffer
- More complex
- Better for large data transfers
- **Deferred for future**

---

## Open Questions

None — basic pipe semantics are well-defined by POSIX.

---

## Deliverables
- Pipe struct design
- VfsFile trait implementations for pipe ends
- Syscall signatures and numbers
