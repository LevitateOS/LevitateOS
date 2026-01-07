# Phase 5: I/O (writev/readv) (P1)

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: P1 — Required for `println!`

## Purpose
Implement vectored I/O syscalls that Rust's standard I/O uses.

## Success Criteria
- `sys_writev` writes multiple buffers atomically
- `sys_readv` reads into multiple buffers
- `println!` works in std programs

## Reference
- `rustix::io` for vectored I/O
- Linux writev(2), readv(2) man pages

---

## Step 1: Define iovec Structure

### UoW 5.1.1: Add iovec to Kernel
**File**: `phase-5-step-1-uow-1.md`

**Objective**: Define vectored I/O structure in kernel.

**Tasks**:
1. Add structure (may already exist in libsyscall):
   ```rust
   #[repr(C)]
   pub struct IoVec {
       pub iov_base: *const u8,
       pub iov_len: usize,
   }
   ```
2. Add syscall numbers:
   ```rust
   pub const SYS_READV: u64 = 65;
   pub const SYS_WRITEV: u64 = 66;
   ```

**Exit Criteria**: Structure and constants defined.

---

## Step 2: Implement sys_writev

### UoW 5.2.1: Implement writev Core Logic
**File**: `phase-5-step-2-uow-1.md`

**Objective**: Implement vectored write.

**Tasks**:
1. Create `sys_writev(fd, iov, iovcnt)`:
   ```rust
   pub fn sys_writev(fd: usize, iov: *const IoVec, iovcnt: usize) -> isize
   ```
2. Implementation:
   - Validate `iov` pointer is in user space
   - For each iovec entry:
     - Validate `iov_base` is in user space
     - Write `iov_len` bytes to fd
   - Return total bytes written
3. Handle partial writes correctly

**Exit Criteria**: writev writes all buffers.

---

### UoW 5.2.2: Wire writev to Syscall Dispatch
**File**: `phase-5-step-2-uow-2.md`

**Objective**: Make writev callable.

**Tasks**:
1. Add `SYS_WRITEV` to dispatch
2. Extract 3 arguments
3. Call `sys_writev`
4. Return result

**Exit Criteria**: writev callable from userspace.

---

## Step 3: Implement sys_readv

### UoW 5.3.1: Implement readv Core Logic
**File**: `phase-5-step-3-uow-1.md`

**Objective**: Implement vectored read.

**Tasks**:
1. Create `sys_readv(fd, iov, iovcnt)`:
   ```rust
   pub fn sys_readv(fd: usize, iov: *mut IoVec, iovcnt: usize) -> isize
   ```
2. Implementation:
   - Validate pointers
   - For each iovec entry:
     - Read up to `iov_len` bytes from fd into `iov_base`
   - Return total bytes read
3. Handle EOF correctly

**Exit Criteria**: readv reads into all buffers.

---

### UoW 5.3.2: Wire readv to Syscall Dispatch
**File**: `phase-5-step-3-uow-2.md`

**Objective**: Make readv callable.

**Tasks**:
1. Add `SYS_READV` to dispatch
2. Extract 3 arguments
3. Call `sys_readv`
4. Return result

**Exit Criteria**: readv callable from userspace.

---

## Step 4: Userspace Integration

### UoW 5.4.1: Verify libsyscall Wrappers
**File**: `phase-5-step-4-uow-1.md`

**Objective**: Ensure userspace wrappers exist.

**Tasks**:
1. Check libsyscall for existing writev/readv (TEAM_217 added these)
2. If missing, add:
   ```rust
   pub fn writev(fd: usize, iov: &[IoVec]) -> isize
   pub fn readv(fd: usize, iov: &mut [IoVec]) -> isize
   ```
3. Verify IoVec structure matches kernel

**Exit Criteria**: Wrappers present and correct.

---

### UoW 5.4.2: Add Vectored I/O Test
**File**: `phase-5-step-4-uow-2.md`

**Objective**: Test vectored I/O.

**Tasks**:
1. Create test that:
   - Creates 3 separate buffers with different strings
   - Calls writev to write all 3 to stdout
   - Verifies output is concatenated correctly
2. Test readv with stdin or file

**Exit Criteria**: Vectored I/O test passes.

---

## Deliverables
- `sys_writev` implementation
- `sys_readv` implementation
- Verified libsyscall wrappers
- Vectored I/O test
