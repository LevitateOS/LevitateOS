# Phase 4: Threading (clone, TLS, set_tid_address) (P1)

**Parent**: `docs/planning/std-support/`  
**Team**: TEAM_222  
**Status**: Not Started  
**Priority**: P1 — Unlocks `std::thread`

## Purpose
Implement Linux-compatible threading primitives that `std::thread` requires.

## Success Criteria
- `sys_clone` creates threads sharing address space
- TLS via `TPIDR_EL0` works correctly
- `sys_set_tid_address` registers clear-on-exit TID
- `std::thread::spawn` works

## Reference
- `origin` crate: `src/threads.rs` for TLS and thread creation
- `rustix::thread` for clone flags
- Linux clone(2) man page

---

## Step 1: TLS Infrastructure

### UoW 4.1.1: Add TPIDR_EL0 to Thread Context
**File**: `phase-4-step-1-uow-1.md`

**Objective**: Save/restore thread-local storage pointer on context switch.

**Tasks**:
1. Find thread/task context structure
2. Add field: `tls_base: u64` (or `tpidr_el0: u64`)
3. In context save (asm):
   ```asm
   mrs x0, tpidr_el0
   str x0, [sp, #OFFSET_TLS]
   ```
4. In context restore (asm):
   ```asm
   ldr x0, [sp, #OFFSET_TLS]
   msr tpidr_el0, x0
   ```

**Exit Criteria**: TPIDR_EL0 saved/restored, existing tests pass.

---

### UoW 4.1.2: Add arch_prctl or Equivalent
**File**: `phase-4-step-1-uow-2.md`

**Objective**: Allow userspace to set TLS base.

**Tasks**:
1. On aarch64, TLS is set via clone or dedicated syscall
2. Option A: Support `CLONE_SETTLS` flag in clone
3. Option B: Add `sys_arch_prctl` for TLS setting
4. Store value in thread's `tls_base` field

**Exit Criteria**: Userspace can set TLS base.

---

## Step 2: Implement sys_clone

### UoW 4.2.1: Define Clone Flags
**File**: `phase-4-step-2-uow-1.md`

**Objective**: Define Linux clone flags.

**Tasks**:
1. Add constants:
   ```rust
   pub const CLONE_VM: u64 = 0x00000100;
   pub const CLONE_FS: u64 = 0x00000200;
   pub const CLONE_FILES: u64 = 0x00000400;
   pub const CLONE_SIGHAND: u64 = 0x00000800;
   pub const CLONE_THREAD: u64 = 0x00010000;
   pub const CLONE_SETTLS: u64 = 0x00080000;
   pub const CLONE_PARENT_SETTID: u64 = 0x00100000;
   pub const CLONE_CHILD_CLEARTID: u64 = 0x00200000;
   pub const CLONE_CHILD_SETTID: u64 = 0x01000000;
   ```
2. Add syscall number: `SYS_CLONE = 220`

**Exit Criteria**: Constants defined.

---

### UoW 4.2.2: Implement Basic sys_clone
**File**: `phase-4-step-2-uow-2.md`

**Objective**: Implement clone for thread creation.

**Tasks**:
1. Create `sys_clone(flags, stack, parent_tid, tls, child_tid)`:
   ```rust
   pub fn sys_clone(
       flags: u64,
       stack: usize,
       parent_tid: *mut i32,
       tls: usize,
       child_tid: *mut i32,
   ) -> isize
   ```
2. For thread case (`CLONE_VM | CLONE_THREAD`):
   - Create new thread in same address space
   - Share: memory, files, fs, signal handlers
   - New: thread ID, stack, registers
   - Set child's stack pointer to `stack`
3. If `CLONE_SETTLS`: set child's `tls_base = tls`
4. Return child TID to parent, 0 to child

**Exit Criteria**: Clone creates thread, both run.

---

### UoW 4.2.3: Implement SETTID/CLEARTID
**File**: `phase-4-step-2-uow-3.md`

**Objective**: Support TID address features.

**Tasks**:
1. If `CLONE_PARENT_SETTID`:
   - Write child TID to `*parent_tid`
2. If `CLONE_CHILD_SETTID`:
   - Write child TID to `*child_tid`
3. If `CLONE_CHILD_CLEARTID`:
   - Store `child_tid` address in thread struct
   - On thread exit: write 0 to that address
   - Wake futex at that address

**Exit Criteria**: TID address features work.

---

### UoW 4.2.4: Wire Clone to Syscall Dispatch
**File**: `phase-4-step-2-uow-4.md`

**Objective**: Make clone callable.

**Tasks**:
1. Add `SYS_CLONE` to dispatch
2. Extract 5 arguments
3. Call `sys_clone`
4. Handle return to both parent and child correctly

**Exit Criteria**: Clone callable from userspace.

---

## Step 3: Implement sys_set_tid_address

### UoW 4.3.1: Implement set_tid_address
**File**: `phase-4-step-3-uow-1.md`

**Objective**: Allow thread to register clear-on-exit address.

**Tasks**:
1. Create `sys_set_tid_address(tidptr)`:
   - Store `tidptr` in current thread's struct
   - Return current TID
2. Add syscall number: `SYS_SET_TID_ADDRESS = 96`
3. On thread exit:
   - If tidptr set: write 0 to `*tidptr`
   - Wake futex at tidptr

**Exit Criteria**: set_tid_address works.

---

## Step 4: Thread Exit Handling

### UoW 4.4.1: Implement Thread Exit
**File**: `phase-4-step-4-uow-1.md`

**Objective**: Handle thread exit properly.

**Tasks**:
1. On thread exit:
   - If `clear_child_tid` set:
     - Write 0 to `*clear_child_tid`
     - `futex_wake(clear_child_tid, 1)`
   - Free thread resources (but not process resources)
   - If last thread: exit process
2. Ensure parent's `pthread_join` (via futex) wakes up

**Exit Criteria**: Thread exit notifies joiners.

---

## Step 5: Userspace Integration

### UoW 4.5.1: Add Clone to libsyscall
**File**: `phase-4-step-5-uow-1.md`

**Objective**: Add userspace wrapper.

**Tasks**:
1. Add to libsyscall:
   ```rust
   pub fn clone(flags: u64, stack: usize, parent_tid: *mut i32, tls: usize, child_tid: *mut i32) -> isize
   pub fn set_tid_address(tidptr: *mut i32) -> isize
   ```
2. Add clone flag constants

**Exit Criteria**: Wrappers compile.

---

### UoW 4.5.2: Add Threading Test
**File**: `phase-4-step-5-uow-2.md`

**Objective**: Test thread creation and joining.

**Tasks**:
1. Create test that:
   - Allocates stack for child
   - Calls clone with thread flags
   - Child writes to shared memory
   - Parent waits via futex on clear_child_tid
   - Verifies child ran
2. Add to test suite

**Exit Criteria**: Threading test passes.

---

## Deliverables
- TPIDR_EL0 in context switch
- `sys_clone` with thread support
- `sys_set_tid_address`
- Thread exit with futex wake
- libsyscall wrappers
- Threading test
