# Phase 4 — Implementation and Tests

**Team**: TEAM_224
**Bug**: Unsafe User Memory Access in Syscalls
**Status**: Ready for Execution

---

## Overview

This phase contains the concrete implementation work. Each step is a single Unit of Work (UoW) that can be executed by an SLM.

---

## Step 1: Create Helper Function

**File**: `kernel/src/syscall/mod.rs`
**Location**: After `read_from_user` function (around line 380)

### UoW 1.1: Add `copy_user_string` helper

```rust
/// TEAM_224: Copy a string from user space into a kernel buffer.
///
/// Validates the user buffer and copies bytes through kernel-accessible pointers.
/// This is the safe pattern for reading user memory from syscalls.
///
/// # Arguments
/// * `ttbr0` - User page table physical address
/// * `user_ptr` - User virtual address of string
/// * `len` - Length of string to copy
/// * `buf` - Kernel buffer to copy into
///
/// # Returns
/// * `Ok(&str)` - Valid UTF-8 string slice from buffer
/// * `Err(errno)` - EFAULT if copy fails, EINVAL if not valid UTF-8
pub fn copy_user_string<'a>(
    ttbr0: usize,
    user_ptr: usize,
    len: usize,
    buf: &'a mut [u8],
) -> Result<&'a str, i64> {
    let len = len.min(buf.len());
    if mm_user::validate_user_buffer(ttbr0, user_ptr, len, false).is_err() {
        return Err(errno::EFAULT);
    }
    for i in 0..len {
        if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_ptr + i) {
            buf[i] = unsafe { *ptr };
        } else {
            return Err(errno::EFAULT);
        }
    }
    core::str::from_utf8(&buf[..len]).map_err(|_| errno::EINVAL)
}

/// TEAM_224: Copy raw bytes from user space into a kernel buffer.
///
/// Similar to copy_user_string but returns raw bytes without UTF-8 validation.
pub fn copy_user_bytes(
    ttbr0: usize,
    user_ptr: usize,
    len: usize,
    buf: &mut [u8],
) -> Result<usize, i64> {
    let len = len.min(buf.len());
    if mm_user::validate_user_buffer(ttbr0, user_ptr, len, false).is_err() {
        return Err(errno::EFAULT);
    }
    for i in 0..len {
        if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, user_ptr + i) {
            buf[i] = unsafe { *ptr };
        } else {
            return Err(errno::EFAULT);
        }
    }
    Ok(len)
}
```

**Verification**: `cargo build -p levitate-kernel`

---

## Step 2: Fix `sys_spawn`

**File**: `kernel/src/syscall/process.rs`
**Function**: `sys_spawn` (starts around line 43)

### UoW 2.1: Replace unsafe slice with helper

**Before** (lines 44-54):
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();
if mm_user::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
    return errno::EFAULT;
}

let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
let path = match core::str::from_utf8(path_bytes) {
    Ok(s) => s,
    Err(_) => return errno::EINVAL,
};
```

**After**:
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();

// TEAM_224: Use safe copy through kernel pointers
let mut path_buf = [0u8; 256];
let path = match crate::syscall::copy_user_string(task.ttbr0, path_ptr, path_len, &mut path_buf) {
    Ok(s) => s,
    Err(e) => return e,
};
```

**Verification**: Build and test spawn functionality

---

## Step 3: Fix `sys_exec`

**File**: `kernel/src/syscall/process.rs`
**Function**: `sys_exec` (starts around line 97)

### UoW 3.1: Replace unsafe slice with helper

**Before** (lines 98-108):
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();
if mm_user::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
    return errno::EFAULT;
}

let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
let path = match core::str::from_utf8(path_bytes) {
    Ok(s) => s,
    Err(_) => return errno::EINVAL,
};
```

**After**:
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();

// TEAM_224: Use safe copy through kernel pointers
let mut path_buf = [0u8; 256];
let path = match crate::syscall::copy_user_string(task.ttbr0, path_ptr, path_len, &mut path_buf) {
    Ok(s) => s,
    Err(e) => return e,
};
```

---

## Step 4: Fix `sys_spawn_args` (path)

**File**: `kernel/src/syscall/process.rs`
**Function**: `sys_spawn_args` (starts around line 162)

### UoW 4.1: Replace unsafe path slice

**Before** (lines 169-181):
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();
if mm_user::validate_user_buffer(task.ttbr0, path_ptr, path_len, false).is_err() {
    return errno::EFAULT;
}
let path_bytes = unsafe { core::slice::from_raw_parts(path_ptr as *const u8, path_len) };
let path = match core::str::from_utf8(path_bytes) {
    Ok(s) => alloc::string::String::from(s),
    Err(_) => {
        log::debug!("[SYSCALL] spawn_args: Invalid UTF-8 in path");
        return errno::EINVAL;
    }
};
```

**After**:
```rust
let path_len = path_len.min(256);
let task = crate::task::current_task();

// TEAM_224: Use safe copy through kernel pointers
let mut path_buf = [0u8; 256];
let path = match crate::syscall::copy_user_string(task.ttbr0, path_ptr, path_len, &mut path_buf) {
    Ok(s) => alloc::string::String::from(s),
    Err(e) => {
        log::debug!("[SYSCALL] spawn_args: Invalid path: errno={}", e);
        return e;
    }
};
```

---

## Step 5: Fix `sys_spawn_args` (arguments)

**File**: `kernel/src/syscall/process.rs`
**Function**: `sys_spawn_args`, argument parsing loop

### UoW 5.1: Replace unsafe argument slice

**Before** (lines 214-224):
```rust
// Validate arg string
let arg_len = entry.len.min(MAX_ARG_LEN);
if mm_user::validate_user_buffer(task.ttbr0, entry.ptr, arg_len, false).is_err() {
    return errno::EFAULT;
}

let arg_bytes = unsafe { core::slice::from_raw_parts(entry.ptr as *const u8, arg_len) };
match core::str::from_utf8(arg_bytes) {
    Ok(s) => args.push(alloc::string::String::from(s)),
    Err(_) => return errno::EINVAL,
}
```

**After**:
```rust
// TEAM_224: Validate and copy arg string safely
let arg_len = entry.len.min(MAX_ARG_LEN);
let mut arg_buf = [0u8; MAX_ARG_LEN];
match crate::syscall::copy_user_string(task.ttbr0, entry.ptr, arg_len, &mut arg_buf) {
    Ok(s) => args.push(alloc::string::String::from(s)),
    Err(e) => return e,
}
```

---

## Step 6: Fix `sys_write` (console path)

**File**: `kernel/src/syscall/fs/write.rs`
**Function**: `sys_write`, stdout/stderr branch

### UoW 6.1: Replace unsafe slice with safe copy

**Before** (lines 80-93):
```rust
FdType::Stdout | FdType::Stderr => {
    // Write to console
    if mm_user::validate_user_buffer(ttbr0, buf, len, false).is_err() {
        return errno::EFAULT;
    }
    let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    if let Ok(s) = core::str::from_utf8(slice) {
        print!("{}", s);
    } else {
        for byte in slice {
            print!("{:02x}", byte);
        }
    }
    len as i64
}
```

**After**:
```rust
FdType::Stdout | FdType::Stderr => {
    // TEAM_224: Write to console using safe copy
    if mm_user::validate_user_buffer(ttbr0, buf, len, false).is_err() {
        return errno::EFAULT;
    }
    // Copy bytes through kernel-accessible pointers
    let mut kbuf = alloc::vec![0u8; len];
    for i in 0..len {
        if let Some(ptr) = mm_user::user_va_to_kernel_ptr(ttbr0, buf + i) {
            kbuf[i] = unsafe { *ptr };
        } else {
            return errno::EFAULT;
        }
    }
    if let Ok(s) = core::str::from_utf8(&kbuf) {
        print!("{}", s);
    } else {
        for byte in &kbuf {
            print!("{:02x}", byte);
        }
    }
    len as i64
}
```

---

## Step 7: Verification

### UoW 7.1: Build and Test

```bash
# Build kernel
cargo build --release -p levitate-kernel

# Run full test suite
cargo xtask test

# Manual test
./run.sh
# In shell: Run various commands to exercise syscalls
```

### Expected Results

1. Build succeeds with no errors
2. All existing tests pass
3. System boots normally
4. Shell commands work (ls, cat, spawn processes)

---

## Phase 4 Complete Checklist

- [ ] Step 1: Helper functions added to syscall/mod.rs
- [ ] Step 2: sys_spawn fixed
- [ ] Step 3: sys_exec fixed
- [ ] Step 4: sys_spawn_args path fixed
- [ ] Step 5: sys_spawn_args arguments fixed
- [ ] Step 6: sys_write console path fixed
- [ ] Step 7: Build and test verification passed
