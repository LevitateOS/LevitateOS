# Phase 4: Testing

**Parent**: `docs/planning/feature-clone-thread/`  
**Team**: TEAM_228  
**Status**: Complete (TEAM_231)

## Test Strategy

### Unit Tests
Not applicable — threading requires full kernel integration.

### Integration Test: clone_test

**Location**: `userspace/levbox/src/bin/clone_test.rs` or standalone binary

**Test Program**:
```rust
// clone_test - Test thread creation via clone syscall

fn main() -> i32 {
    // 1. Allocate stack for child thread (using mmap)
    let stack_size = 4096 * 4; // 16KB
    let stack = libsyscall::mmap(
        0, stack_size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1, 0
    );
    if stack < 0 {
        return 1;
    }
    
    // 2. Set up futex for join
    let mut tid: i32 = 0;
    
    // 3. Set up child stack with return address
    let stack_top = (stack as usize) + stack_size;
    // Child needs: return addr, then args
    // For simple test, just use stack_top
    
    // 4. Call clone
    let flags = CLONE_VM | CLONE_THREAD | CLONE_CHILD_CLEARTID;
    let child_tid = libsyscall::clone(
        flags,
        stack_top,
        core::ptr::null_mut(), // parent_tid
        0, // tls
        &mut tid as *mut i32, // child_tid
    );
    
    if child_tid == 0 {
        // Child path
        // Write magic value to shared memory
        // Then exit
        unsafe { *SHARED_FLAG = 42; }
        libsyscall::exit(0);
    } else if child_tid > 0 {
        // Parent path
        // Wait for child via futex
        libsyscall::futex_wait(&tid as *const i32 as usize, child_tid as usize);
        
        // Verify shared memory was modified
        if unsafe { *SHARED_FLAG } == 42 {
            println!("PASS: Thread created and ran successfully");
            return 0;
        }
    }
    
    println!("FAIL: Thread test failed");
    1
}

static mut SHARED_FLAG: i32 = 0;
```

### Manual Verification Steps

1. **Build the project**:
   ```bash
   cargo xtask build all
   ```

2. **Run in QEMU**:
   ```bash
   ./run-term.sh
   ```

3. **From shell, run test**:
   ```
   $ clone_test
   ```

4. **Expected output**:
   ```
   PASS: Thread created and ran successfully
   ```

5. **Verify no crash or hang**.

---

## Test Coverage

| Test Case | Coverage |
|-----------|----------|
| Clone creates thread | UoW 2.1 |
| Child returns 0, parent returns tid | UoW 2.1 |
| Child runs in shared address space | UoW 1.1 |
| Thread exit clears tid | UoW 3.1 |
| Futex wake on thread exit | UoW 3.1 |
