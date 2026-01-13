# Phase 4: Implementation and Tests

**Bug**: `execve()` syscall is a stub returning ENOSYS  
**Team**: TEAM_436  
**Status**: ✅ COMPLETE

---

## Implementation Steps

This phase contains the actual implementation work, broken into discrete steps.

---

## Step 1: Add O_CLOEXEC Support to FdTable

**File**: `crates/kernel/sched/src/fd_table.rs`

### Task 1.1: Add close_cloexec() method

```rust
impl FdTable {
    /// Close all file descriptors with O_CLOEXEC flag set.
    /// Called during execve() to implement fd inheritance semantics.
    pub fn close_cloexec(&mut self) {
        // Iterate through all fds and close those with O_CLOEXEC
        // Need to check if O_CLOEXEC flag is tracked in VfsFile
    }
}
```

### Task 1.2: Verify O_CLOEXEC flag tracking

Check if `VfsFile` or fd entry tracks the O_CLOEXEC flag. If not, add it.

**Estimated effort**: 1 UoW

---

## Step 2: Create ELF Loading Helper for Exec

**File**: `crates/kernel/syscall/src/process/exec.rs` (new file)

### Task 2.1: Create exec module

Create a new module for execve implementation to keep lifecycle.rs clean.

### Task 2.2: Implement prepare_exec_image()

```rust
/// Load ELF and prepare new address space for execve.
/// Returns (new_ttbr0, entry_point, initial_sp) on success.
pub fn prepare_exec_image(
    elf_data: &[u8],
    argv: &[&str],
    envp: &[&str],
) -> Result<ExecImage, u32> {
    // 1. Parse ELF headers
    // 2. Create new page table
    // 3. Map ELF segments
    // 4. Set up stack with argv/envp/auxv
    // 5. Return exec image ready to be applied
}

pub struct ExecImage {
    pub ttbr0: PhysAddr,
    pub entry_point: usize,
    pub stack_pointer: usize,
    pub vmas: VmaList,
    pub heap: ProcessHeap,
}
```

**Estimated effort**: 2-3 UoWs (can reuse spawn_from_elf logic)

---

## Step 3: Implement sys_execve()

**File**: `crates/kernel/syscall/src/process/lifecycle.rs`

### Task 3.1: Add sys_execve function

```rust
/// Linux-compatible execve syscall.
/// Replaces current process image with new program.
pub fn sys_execve(
    path_ptr: usize,
    argv_ptr: usize,
    envp_ptr: usize,
) -> SyscallResult {
    // 1. Read path from userspace (null-terminated)
    // 2. Read argv array from userspace
    // 3. Read envp array from userspace (can be NULL)
    // 4. Resolve executable
    // 5. Prepare new exec image
    // 6. Close O_CLOEXEC file descriptors
    // 7. Reset signal handlers
    // 8. Apply exec image to current task
    // 9. Set syscall frame to new entry point
    // 10. Return (never actually returns - jumps to new code)
}
```

### Task 3.2: Add apply_exec_image() helper

```rust
/// Apply exec image to current task, replacing its address space.
fn apply_exec_image(task: &TaskControlBlock, image: ExecImage, frame: &mut SyscallFrame) {
    // 1. Save old ttbr0
    // 2. Swap to new ttbr0
    // 3. Free old address space
    // 4. Update task's vmas, heap
    // 5. Set frame.pc to entry_point
    // 6. Set frame.sp to stack_pointer
    // 7. Clear all other registers (Linux behavior)
}
```

**Estimated effort**: 2 UoWs

---

## Step 4: Update Syscall Dispatcher

**File**: `crates/kernel/syscall/src/lib.rs`

### Task 4.1: Add Execve to dispatcher

The dispatcher currently maps `Exec` → `sys_exec()`. We need to update this to call `sys_execve()` with the correct arguments.

```rust
Some(SyscallNumber::Exec) => process::sys_execve(
    frame.arg0() as usize,  // path
    frame.arg1() as usize,  // argv
    frame.arg2() as usize,  // envp
),
```

**Estimated effort**: 1 UoW (small)

---

## Step 5: Add Tests

**File**: `tests/` or in-kernel test

### Task 5.1: Create execve test program

Create a simple userspace program that:
1. Calls fork()
2. Child calls execve("/echo", ["echo", "execve_works"])
3. Parent waits for child
4. Verify output

### Task 5.2: Add to test suite

Add the test to `cargo xtask test` workflow.

**Estimated effort**: 1-2 UoWs

---

## Step 6: Verify Dual-Architecture Support

### Task 6.1: Build and test x86_64

```bash
cargo xtask --arch x86_64 build all
cargo xtask --arch x86_64 run
```

### Task 6.2: Build and test aarch64

```bash
cargo xtask --arch aarch64 build all
cargo xtask --arch aarch64 run
```

**Estimated effort**: 1 UoW

---

## Total Estimated Effort

| Step | UoWs | Description |
|------|------|-------------|
| Step 1 | 1 | O_CLOEXEC support |
| Step 2 | 2-3 | ELF loading helper |
| Step 3 | 2 | sys_execve implementation |
| Step 4 | 1 | Syscall dispatcher |
| Step 5 | 1-2 | Tests |
| Step 6 | 1 | Dual-arch verification |
| **Total** | **8-10** | |

---

## Implementation Notes

### Critical: Frame Register Handling

When execve succeeds, the syscall frame must be modified to:
- Set PC/RIP to new entry point
- Set SP to new stack pointer
- Clear all general-purpose registers (Linux zeroes them)
- Set up initial register state per ABI

On x86_64:
```rust
frame.set_rip(entry_point);
frame.set_rsp(stack_pointer);
frame.rax = 0; frame.rbx = 0; // etc.
```

On aarch64:
```rust
frame.set_elr(entry_point);
frame.set_sp(stack_pointer);
frame.x0 = 0; frame.x1 = 0; // etc.
```

### Critical: Address Space Cleanup

Must free old pages to prevent memory leak:
```rust
// Before swapping ttbr0
let old_ttbr0 = task.ttbr0;
task.ttbr0 = new_image.ttbr0;
// After swap
free_user_address_space(old_ttbr0, &old_vmas);
```
