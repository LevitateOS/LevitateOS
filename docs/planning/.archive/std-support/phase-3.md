# Phase 3: mmap/munmap/mprotect Implementation (P0)

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: P0 — Required for std allocator

## Purpose
Implement memory mapping syscalls that Rust's allocator needs for large allocations and thread stacks.

## Success Criteria
- `sys_mmap` supports anonymous mappings
- `sys_munmap` frees mapped regions
- `sys_mprotect` changes page permissions
- Allocator can use mmap for large allocations

## Reference
- `rustix::mm` for syscall interface
- Linux mmap(2), munmap(2), mprotect(2) man pages

---

## Step 1: Virtual Memory Infrastructure

### UoW 3.1.1: Audit Current VM State
**File**: `phase-3-step-1-uow-1.md`

**Objective**: Understand current virtual memory management.

**Tasks**:
1. Find VM-related code (page tables, address space management)
2. Document:
   - How user address space is managed
   - Current allocation strategy
   - Free region tracking (if any)
3. Identify what's missing for mmap

**Exit Criteria**: VM architecture documented with gaps identified.

---

### UoW 3.1.2: Add VMA (Virtual Memory Area) Tracking
**File**: `phase-3-step-1-uow-2.md`

**Objective**: Add structure to track mapped regions per process.

**Tasks**:
1. Define VMA structure:
   ```rust
   pub struct Vma {
       pub start: usize,
       pub end: usize,
       pub prot: u32,      // PROT_READ | PROT_WRITE | PROT_EXEC
       pub flags: u32,     // MAP_PRIVATE | MAP_ANONYMOUS | etc
   }
   ```
2. Add `vmas: Vec<Vma>` or similar to process struct
3. Add helper: `find_free_region(size) -> Option<usize>`

**Exit Criteria**: VMA structure exists, compiles.

---

## Step 2: Implement sys_mmap

### UoW 3.2.1: Add mmap Constants
**File**: `phase-3-step-2-uow-1.md`

**Objective**: Define mmap-related constants.

**Tasks**:
1. Add protection flags:
   ```rust
   pub const PROT_NONE: u32 = 0;
   pub const PROT_READ: u32 = 1;
   pub const PROT_WRITE: u32 = 2;
   pub const PROT_EXEC: u32 = 4;
   ```
2. Add map flags:
   ```rust
   pub const MAP_SHARED: u32 = 0x01;
   pub const MAP_PRIVATE: u32 = 0x02;
   pub const MAP_FIXED: u32 = 0x10;
   pub const MAP_ANONYMOUS: u32 = 0x20;
   ```
3. Add syscall number: `SYS_MMAP = 222` (Linux aarch64)

**Exit Criteria**: Constants defined, compiles.

---

### UoW 3.2.2: Implement Anonymous mmap
**File**: `phase-3-step-2-uow-2.md`

**Objective**: Implement mmap for anonymous memory.

**Tasks**:
1. Create `sys_mmap(addr, len, prot, flags, fd, offset)`:
   ```rust
   pub fn sys_mmap(
       addr: usize,
       len: usize,
       prot: u32,
       flags: u32,
       fd: i32,
       offset: usize,
   ) -> isize
   ```
2. For MVP, only support `MAP_ANONYMOUS | MAP_PRIVATE`:
   - Find free region of `len` bytes
   - Allocate physical pages
   - Map pages with requested protection
   - Add VMA entry
   - Return start address
3. Return `-ENOMEM` on failure

**Exit Criteria**: Anonymous mmap works for simple cases.

---

### UoW 3.2.3: Add mmap to Syscall Dispatch
**File**: `phase-3-step-2-uow-3.md`

**Objective**: Wire mmap into syscall handler.

**Tasks**:
1. Add `SYS_MMAP` case to syscall dispatch
2. Extract all 6 arguments from registers
3. Call `sys_mmap`
4. Return result

**Exit Criteria**: mmap callable from userspace.

---

## Step 3: Implement sys_munmap

### UoW 3.3.1: Implement munmap
**File**: `phase-3-step-3-uow-1.md`

**Objective**: Implement memory unmapping.

**Tasks**:
1. Create `sys_munmap(addr, len)`:
   - Find VMA containing `addr`
   - Unmap pages in range
   - Free physical pages
   - Remove/split VMA entry
2. Add syscall number: `SYS_MUNMAP = 215`
3. Wire into syscall dispatch

**Exit Criteria**: munmap frees mmap'd memory.

---

## Step 4: Implement sys_mprotect

### UoW 3.4.1: Implement mprotect
**File**: `phase-3-step-4-uow-1.md`

**Objective**: Implement protection change.

**Tasks**:
1. Create `sys_mprotect(addr, len, prot)`:
   - Find VMA containing range
   - Update page table entries with new protection
   - Update VMA prot field
2. Add syscall number: `SYS_MPROTECT = 226`
3. Wire into syscall dispatch

**Exit Criteria**: mprotect changes page permissions.

---

## Step 5: Userspace Integration

### UoW 3.5.1: Add mmap/munmap to libsyscall
**File**: `phase-3-step-5-uow-1.md`

**Objective**: Add userspace wrappers.

**Tasks**:
1. Add to libsyscall:
   ```rust
   pub fn mmap(addr: usize, len: usize, prot: u32, flags: u32, fd: i32, offset: usize) -> isize
   pub fn munmap(addr: usize, len: usize) -> isize
   pub fn mprotect(addr: usize, len: usize, prot: u32) -> isize
   ```
2. Add corresponding constants

**Exit Criteria**: Wrappers compile.

---

### UoW 3.5.2: Add mmap Test Program
**File**: `phase-3-step-5-uow-2.md`

**Objective**: Create test verifying mmap works.

**Tasks**:
1. Create test that:
   - mmap's anonymous region
   - Writes to it
   - Reads back
   - munmap's it
   - Verifies no crash
2. Add to test suite

**Exit Criteria**: Test passes.

---

## Deliverables
- VMA tracking in process struct
- `sys_mmap`, `sys_munmap`, `sys_mprotect` implementations
- libsyscall wrappers
- mmap test program
