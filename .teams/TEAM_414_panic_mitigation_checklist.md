# TEAM_414: Panic Mitigation Checklist

**Created**: 2026-01-10
**Purpose**: Audit all panic-inducing code paths in kernel-critical code for mitigation

---

## Summary

This checklist documents **all locations** where the kernel can panic. The focus is on kernel-critical code (kernel crate and HAL crate), excluding userspace test code.

**Total panic points found:**
- **unwrap()**: 53 instances in kernel, additional in HAL
- **expect()**: 10 instances in kernel, additional in HAL  
- **panic!()**: 4 instances in kernel, 6 in HAL
- **unreachable!()**: 1 instance
- **unimplemented!()**: 1 instance

---

## 🔴 CRITICAL: Explicit panic!() Calls

These are intentional panics that should be reviewed for proper error handling.

### Kernel Crate

- [ ] **`src/main.rs:205`** - OOM handler
  ```rust
  panic!("out of memory");
  ```
  **Mitigation**: This is the global allocator OOM handler - panic is appropriate here, but consider logging before panic.

- [ ] **`src/init.rs:143`** - Manual panic from maintenance shell
  ```rust
  panic!("Manual panic triggered from maintenance shell");
  ```
  **Mitigation**: Intentional debug feature - acceptable.

- [ ] **`src/memory/mod.rs:181`** - Failed to allocate mem_map
  ```rust
  panic!("Failed to allocate physical memory for mem_map!");
  ```
  **Mitigation**: Early boot failure - panic is appropriate, system cannot continue.

- [ ] **`src/fs/tmpfs/superblock.rs:113`** - Tmpfs root not initialized
  ```rust
  panic!("Tmpfs::root called before vfs_root was initialized");
  ```
  **Mitigation**: ⚠️ **SHOULD FIX** - Return `Option<Arc<Inode>>` or `VfsResult` instead.

### HAL Crate

- [ ] **`src/x86_64/cpu/exceptions.rs:193`** - Divide error handler
- [ ] **`src/x86_64/cpu/exceptions.rs:197`** - Debug handler
- [ ] **`src/x86_64/cpu/exceptions.rs:245-248`** - Invalid opcode handler
- [ ] **`src/x86_64/cpu/exceptions.rs:252-255`** - Double fault handler
- [ ] **`src/x86_64/cpu/exceptions.rs:259-262`** - General protection fault handler
- [ ] **`src/x86_64/cpu/exceptions.rs:270-272`** - Page fault handler
  **Mitigation**: Exception handlers - panic is appropriate for unrecoverable CPU exceptions.

- [ ] **`src/x86_64/mem/mmu.rs:565`** - Translation failed (test code)
  **Mitigation**: Test code - acceptable.

---

## 🟠 HIGH PRIORITY: expect() Calls

These have explicit error messages but will still panic.

### Kernel Crate

- [ ] **`src/main.rs:164`** - Boot info initialization
  ```rust
  crate::boot::boot_info().expect("AArch64 must have BootInfo initialized from DTB");
  ```
  **Mitigation**: Early boot - panic appropriate if boot info missing.

- [ ] **`src/init.rs:369`** - Boot info for userspace init
  ```rust
  crate::boot::boot_info().expect("BootInfo must be available for userspace init");
  ```
  **Mitigation**: Same as above - boot info is required.

- [ ] **`src/logger.rs:41`** - Logger initialization
  ```rust
  log::set_logger(&LOGGER).expect("Failed to set logger");
  ```
  **Mitigation**: Single-call initialization - panic appropriate if called twice.

- [ ] **`src/task/mod.rs:106`** - current_task() before scheduler init
  ```rust
  .expect("current_task() called before scheduler init")
  ```
  **Mitigation**: ⚠️ **SHOULD FIX** - Return `Option<Arc<Task>>` for safer API.

- [ ] **`src/arch/x86_64/mod.rs:592`** - Boot info must be set
  ```rust
  crate::boot::boot_info().expect("Boot info must be set");
  ```
  **Mitigation**: Early boot - panic appropriate.

- [ ] **`src/memory/mod.rs:69`** - Reserved memory checked None above
  ```rust
  reserved[next_idx].as_ref().expect("Checked None above");
  ```
  **Mitigation**: Logic invariant - code checks for None before this. Consider using `unwrap_unchecked` with safety comment or restructure.

### HAL Crate

- [ ] **`src/virtio.rs:35-36`** - DMA allocation layout
  ```rust
  .expect("TEAM_130: Layout creation failed - invalid page count");
  ```
  **Mitigation**: Layout from constant values - should never fail. Consider `unwrap_unchecked`.

- [ ] **`src/virtio.rs:39`** - DMA allocation OOM
  ```rust
  NonNull::new(ptr).expect("TEAM_130: VirtIO DMA allocation failed - OOM");
  ```
  **Mitigation**: OOM is unrecoverable in kernel context - panic appropriate.

- [ ] **`src/virtio.rs:48`** - DMA dealloc layout
  ```rust
  .expect("TEAM_130: Layout creation failed - invalid page count");
  ```
  **Mitigation**: Same as allocation.

- [ ] **`src/virtio.rs:59`** - MMIO phys_to_virt null
  ```rust
  NonNull::new(vaddr as *mut u8).expect("TEAM_130: MMIO phys_to_virt returned null")
  ```
  **Mitigation**: Indicates corrupted address translation - panic appropriate.

- [ ] **`src/allocator/buddy.rs:87-88`** - Pop from non-empty list
  ```rust
  .expect("TEAM_135: List was not empty but pop_front failed");
  ```
  **Mitigation**: Invariant violation - indicates memory corruption. Panic appropriate.

- [ ] **`src/allocator/buddy.rs:98`** - Buddy page must exist
  ```rust
  .expect("TEAM_130: Buddy page must exist - corrupted allocator state");
  ```
  **Mitigation**: Invariant violation - panic appropriate.

- [ ] **`src/allocator/buddy.rs:148`** - Page must exist in free
  ```rust
  .expect("TEAM_130: Page must exist - invalid PA passed to free");
  ```
  **Mitigation**: Caller error - consider returning Result instead.

- [ ] **`src/allocator/buddy.rs:174`** - mem_map not initialized
  ```rust
  .expect("TEAM_130: mem_map must be set - allocator not initialized");
  ```
  **Mitigation**: Initialization order bug - panic appropriate.

- [ ] **`src/allocator/intrusive_list.rs:114`** - NonNull was null
  ```rust
  .expect("TEAM_135: NonNull was null - impossible");
  ```
  **Mitigation**: Type system invariant - should be impossible. Consider `unwrap_unchecked`.

---

## 🟡 MEDIUM PRIORITY: unwrap() Calls in Syscall Paths

These are in hot paths and could crash the kernel on malformed input.

### Syscall User Memory Operations

- [ ] **`src/syscall/process.rs:1036`** - user_va_to_kernel_ptr in getrusage
- [ ] **`src/syscall/process.rs:1122`** - user_va_to_kernel_ptr in getrlimit
- [ ] **`src/syscall/time.rs:91`** - user_va_to_kernel_ptr in clock_gettime
- [ ] **`src/syscall/time.rs:146`** - user_va_to_kernel_ptr in gettimeofday
- [ ] **`src/syscall/time.rs:189`** - user_va_to_kernel_ptr in clock_getres
- [ ] **`src/syscall/sys.rs:129`** - user_va_to_kernel_ptr in getrandom
- [ ] **`src/syscall/fs/stat.rs:82`** - user_va_to_kernel_ptr in fstat
- [ ] **`src/syscall/fs/fd.rs:158`** - user_va_to_kernel_ptr in pipe2
- [ ] **`src/syscall/fs/fd.rs:580`** - user_va_to_kernel_ptr in pread64
- [ ] **`src/syscall/fs/fd.rs:645`** - user_va_to_kernel_ptr in pwrite64
- [ ] **`src/syscall/fs/dir.rs:117`** - user_va_to_kernel_ptr in getcwd
- [ ] **`src/syscall/fs/statx.rs:185`** - user_va_to_kernel_ptr in statx
- [ ] **`src/syscall/fs/read.rs:31`** - user_va_to_kernel_ptr in readv
- [ ] **`src/syscall/fs/write.rs:40,91,115,133,163`** - user_va_to_kernel_ptr in writev

**Mitigation Strategy**: All these follow the pattern:
```rust
// SAFETY: validate_user_buffer confirmed buffer is accessible
let dest = mm_user::user_va_to_kernel_ptr(task.ttbr0, addr).unwrap();
```
The `validate_user_buffer` is called before, but `unwrap()` is still dangerous if validation has a bug. 
⚠️ **SHOULD FIX**: Change to `?` operator with proper error propagation, or at minimum use `expect()` with context.

---

## 🟢 LOW PRIORITY: Build-time / Boot unwrap()

### Build Script

- [ ] **`build.rs:4`** - CARGO_CFG_TARGET_ARCH
- [ ] **`build.rs:5`** - CARGO_MANIFEST_DIR
  **Mitigation**: Build-time env vars - always present during cargo build.

### Boot/Initialization

- [ ] **`src/virtio.rs:60`** - VirtIO header NonNull
  ```rust
  core::ptr::NonNull::new(addr as *mut ...).unwrap();
  ```
  **Mitigation**: Address is computed from constants - should be valid.

### AArch64 MMU Boot Mappings

- [ ] **`src/arch/aarch64/boot.rs:54,62,70,78,86,94,102,110,111`** - mmu::map_range().unwrap()
  **Mitigation**: Boot-time mapping failures are unrecoverable - panic is appropriate. Could add better error messages via expect().

---

## 🔵 TEST CODE: unwrap() (Acceptable)

The following are in test code and are acceptable:

- `src/memory/vma.rs` - VmaList tests
- `src/memory/user.rs` - User memory tests  
- `src/loader/elf.rs:119-133` - ELF header parsing (try_into().unwrap() on fixed-size slices)
- `src/fs/initramfs.rs:196` - Root inode lock
- `src/fs/tmpfs/dir_ops.rs:268` - Directory rename (found_idx.unwrap() after confirming Some)

---

## 🟣 SPECIAL: unreachable!() and unimplemented!()

- [ ] **`src/gpu.rs:141`** - unreachable!() in VirtIO display error handling
  ```rust
  d.draw_iter(pixels).map_err(|_| unreachable!())
  ```
  **Mitigation**: VirtIO returns Infallible - truly unreachable. Consider using `match` with `Infallible` type instead.

- [ ] **`src/arch/x86_64/mod.rs:564`** - unimplemented!() for x86_64 exception_return
  ```rust
  unimplemented!("x86_64 exception_return");
  ```
  **Mitigation**: ⚠️ **SHOULD FIX** - This will panic if called. Either implement or make it compile-time error.

---

## Recommended Mitigation Priority

### Phase 1: Critical Path Safety (Syscalls)
1. Replace all `user_va_to_kernel_ptr().unwrap()` with proper error handling
2. Add `?` propagation or explicit `expect()` with context

### Phase 2: Filesystem Safety
1. Fix `Tmpfs::root()` to return `Option` or `Result`
2. Review initramfs root_inode access

### Phase 3: Task System Safety
1. Make `current_task()` return `Option<Arc<Task>>`
2. Add `current_task_unchecked()` for performance-critical paths

### Phase 4: Cleanup
1. Replace obvious invariant `expect()` with `unwrap_unchecked()` where safe
2. Add `#[track_caller]` to panic-prone functions for better backtraces

---

## Handoff Notes

- Total identified: ~70+ panic points in kernel-critical code
- Most critical: syscall path unwrap() calls (15+ locations)
- Most dangerous: `current_task().expect()` - called frequently
- Acceptable: Boot-time panics, OOM panics, exception handlers
