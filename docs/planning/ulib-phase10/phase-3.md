# Phase 10: Userspace Standard Library (`ulib`) — Phase 3: Implementation

**Team:** TEAM_164  
**Created:** 2026-01-06  
**Status:** READY FOR IMPLEMENTATION (Phase 2 approved by TEAM_165)  
**Parent:** `phase-2.md`

---

## 1. Implementation Overview

Phase 10 implementation is divided into steps, each with one or more Units of Work (UoW) that can be executed by a single team.

### 1.1 Step Breakdown

| Step | Name | Description | Dependencies |
|------|------|-------------|--------------|
| 1 | Kernel: sbrk | Fix sbrk syscall stub | None |
| 2 | ulib: Allocator | Global allocator using sbrk | Step 1 |
| 3 | Kernel: File Syscalls | Add openat, close, fstat | None |
| 4 | ulib: File Abstractions | File, OpenOptions, io::Error | Step 3 |
| 5 | Kernel: Arg/Env Passing | Stack-based argc/argv | None |
| 6 | ulib: Environment | args(), vars() | Step 5 |
| 7 | Kernel: Time Syscalls | nanosleep, clock_gettime | None |
| 8 | ulib: Time | Instant, Duration, sleep | Step 7 |
| 9 | Integration | Demo program, shell enhancement | Steps 2,4,6,8 |

---

## 2. Step 1: Kernel sbrk Implementation

**Goal:** Make `sys_sbrk` functional so userspace can grow its heap.

### 2.1 Current State
- `sys_sbrk` in `kernel/src/syscall.rs` returns `errno::ENOSYS`
- Process has `ttbr0` (page table base) but no heap tracking

### 2.2 Required Changes

1. Add heap tracking to process/task structure
2. Implement page allocation for heap growth
3. Map new pages into TTBR0

### 2.3 UoW Files

#### `phase-3-step-1-uow-1.md`: Add Process Heap State

**Tasks:**
1. Add `ProcessHeap` struct to `kernel/src/task/process.rs`
2. Initialize heap with base address (e.g., 0x0000_0001_0000_0000)
3. Add heap field to Process structure
4. Update process creation to initialize heap

**Expected Output:**
- Heap state exists but sbrk still returns ENOSYS

---

#### `phase-3-step-1-uow-2.md`: Implement sys_sbrk

**Tasks:**
1. Calculate new program break from increment
2. Allocate physical pages if needed
3. Map pages into process TTBR0
4. Update heap.current
5. Return previous break (or new break on success)

**Expected Output:**
- `sys_sbrk(n)` allocates n bytes of heap
- Userspace can use `libsyscall::sbrk()` successfully

---

## 3. Step 2: ulib Allocator

**Goal:** Create a global allocator backed by sbrk.

### 3.1 UoW Files

#### `phase-3-step-2-uow-1.md`: Create ulib Crate

**Tasks:**
1. Create `userspace/ulib/Cargo.toml`
2. Create `userspace/ulib/src/lib.rs`
3. Add ulib to workspace
4. Add libsyscall as dependency
5. Create basic module structure

**Expected Output:**
- `ulib` compiles, shell can depend on it

---

#### `phase-3-step-2-uow-2.md`: Implement Global Allocator

**Tasks:**
1. Create `userspace/ulib/src/alloc.rs`
2. Implement `LosAllocator` struct with linked-list or bump allocator
3. Implement `GlobalAlloc` trait
4. Use `sbrk` to grow heap
5. Export `#[global_allocator]` static

**Expected Output:**
- `Vec`, `Box`, `String` work in userspace programs

---

## 4. Step 3: Kernel File Syscalls

**Goal:** Add file-related syscalls to kernel.

### 4.1 UoW Files

#### `phase-3-step-3-uow-1.md`: File Descriptor Table

**Tasks:**
1. Create `kernel/src/task/fd_table.rs`
2. Implement `FdTable` struct
3. Pre-populate fd 0/1/2 for stdin/stdout/stderr
4. Add FdTable to process/task
5. Update existing read/write to use FdTable

**Expected Output:**
- Existing fd 0/1/2 operations still work
- FdTable infrastructure ready

---

#### `phase-3-step-3-uow-2.md`: Implement openat/close

**Tasks:**
1. Add syscall numbers 9 (openat) and 10 (close)
2. Implement `sys_openat`: lookup path in initramfs, allocate fd
3. Implement `sys_close`: release fd
4. Add wrappers to libsyscall

**Expected Output:**
- Can open files from initramfs
- Can close file descriptors

---

#### `phase-3-step-3-uow-3.md`: Implement fstat

**Tasks:**
1. Add syscall number 11 (fstat)
2. Implement `sys_fstat`: return file size, type
3. Define `Stat` structure in libsyscall
4. Add wrapper to libsyscall

**Expected Output:**
- Can query file metadata

---

## 5. Step 4: ulib File Abstractions

**Goal:** Create idiomatic Rust file types.

### 5.1 UoW Files

#### `phase-3-step-4-uow-1.md`: io Module

**Tasks:**
1. Create `userspace/ulib/src/io.rs`
2. Define `Error` type with errno conversion
3. Define `Result<T>` type alias
4. Define `Read` and `Write` traits

**Expected Output:**
- Error handling infrastructure ready

---

#### `phase-3-step-4-uow-2.md`: File Type

**Tasks:**
1. Create `userspace/ulib/src/fs.rs`
2. Implement `File` struct (wraps fd)
3. Implement `File::open()`, `File::create()`
4. Implement `Read` trait for `File`
5. Implement `Drop` for `File` (calls close)

**Expected Output:**
- Can open and read files idiomatically

---

## 6. Step 5: Kernel Arg/Env Passing

**Goal:** Pass arguments to userspace via stack.

### 6.1 UoW Files

#### `phase-3-step-5-uow-1.md`: Stack Argument Setup

**Tasks:**
1. Modify ELF loader to accept argv/envp
2. Set up stack layout: argc, argv[], NULL, envp[], NULL
3. Update spawn/exec syscalls to accept arguments
4. Update init to pass arguments to shell

**Expected Output:**
- Process stack contains argc/argv on entry

---

## 7. Step 6: ulib Environment

**Goal:** Parse and expose argc/argv to userspace.

### 7.1 UoW Files

#### `phase-3-step-6-uow-1.md`: Args and Vars

**Tasks:**
1. Create `userspace/ulib/src/env.rs`
2. Parse argc/argv from stack at startup
3. Implement `args() -> Args` iterator
4. Implement `vars() -> Vars` iterator (if envp supported)

**Expected Output:**
- `for arg in ulib::env::args()` works

---

## 8. Step 7: Kernel Time Syscalls

**Goal:** Add time-related syscalls.

### 8.1 UoW Files

#### `phase-3-step-7-uow-1.md`: nanosleep

**Tasks:**
1. Add syscall number 13 (nanosleep)
2. Implement `sys_nanosleep`: sleep for specified duration
3. Use timer or yield-based approach
4. Add wrapper to libsyscall

**Expected Output:**
- Process can sleep for specified duration

---

## 9. Step 8: ulib Time

**Goal:** Time abstractions for userspace.

### 9.1 UoW Files

#### `phase-3-step-8-uow-1.md`: Instant and Duration

**Tasks:**
1. Create `userspace/ulib/src/time.rs`
2. Implement `Duration` struct
3. Implement `Instant` struct (uses monotonic counter)
4. Implement `sleep(Duration)` using nanosleep syscall

**Expected Output:**
- `Instant::now()`, `Instant::elapsed()`, `sleep()` work

---

## 10. Step 9: Integration

**Goal:** Demonstrate all components working together.

### 10.1 UoW Files

#### `phase-3-step-9-uow-1.md`: Demo Program

**Tasks:**
1. Create `userspace/demo/` crate
2. Demonstrate heap allocation (Vec, String)
3. Demonstrate file reading
4. Demonstrate argument parsing
5. Demonstrate timing
6. Add to initramfs

**Expected Output:**
- Running `demo` shows all ulib features working

---

#### `phase-3-step-9-uow-2.md`: Shell Enhancement (Optional)

**Tasks:**
1. Update shell to use ulib allocator
2. Add `cat` command (reads files)
3. Test all existing shell functionality

**Expected Output:**
- Shell benefits from ulib without regressions

---

## 11. Implementation Order

**Critical Path:**
```
Step 1 (sbrk) → Step 2 (allocator) → Step 9 (integration)
```

**Parallel Tracks (after Step 1):**
```
Track A: Steps 3-4 (file syscalls + abstractions)
Track B: Steps 5-6 (arg/env passing)
Track C: Steps 7-8 (time syscalls + abstractions)
```

**Recommended Order:**
1. Step 1: Kernel sbrk
2. Step 2: ulib Allocator
3. Step 3: Kernel File Syscalls
4. Step 4: ulib File Abstractions
5. Step 5-6: Arg/Env (can parallel with 3-4)
6. Step 7-8: Time (can parallel with 3-6)
7. Step 9: Integration

---

## 12. Testing Strategy

### 12.1 Unit Tests
- Allocator: allocation/deallocation cycles
- File: open/read/close sequences
- Time: sleep accuracy (within tolerance)

### 12.2 Integration Tests
- Demo program exercises all features
- Shell regression tests

### 12.3 Golden File Tests
- Boot sequence with new syscall messages
- Demo program expected output

---

## 13. Risk Mitigation

| Risk | Mitigation |
|------|------------|
| sbrk complexity | Start with simple bump allocator, refine later |
| Page table bugs | Extensive testing, kernel logging |
| Breaking shell | Feature-flag new code, test after each step |

---

## 14. Estimated Effort

| Step | UoWs | Estimated Teams |
|------|------|-----------------|
| 1 | 2 | 1-2 |
| 2 | 2 | 1 |
| 3 | 3 | 2-3 |
| 4 | 2 | 1-2 |
| 5 | 1 | 1 |
| 6 | 1 | 1 |
| 7 | 1 | 1 |
| 8 | 1 | 1 |
| 9 | 2 | 1-2 |

**Total:** ~10-14 team sessions

---

## 15. Status

**READY:** Phase 2 questions resolved by TEAM_165 (2026-01-06). Implementation can begin.

### Decision Summary (from Phase 2)
| Question | Decision | Rationale |
|----------|----------|-----------|
| Q1: Heap size | Start with 0, grow 4KB | Rule 20 (Simplicity) |
| Q2: FD allocation | Lowest available | Rule 18 (Least Surprise) |
| Q3: OOM behavior | Return null | Rule 14 (Fail Fast) |
| Q4: Initramfs | Read-only | Rule 20 (Simplicity) |
| Q5: Args | Stack-based | Rule 18 (Least Surprise) |
| Q6: Sleep | Timer-based wakeup | Rule 16 (Energy Awareness) |
| Q7: Errno | Linux values | Rule 18 (Least Surprise) |
