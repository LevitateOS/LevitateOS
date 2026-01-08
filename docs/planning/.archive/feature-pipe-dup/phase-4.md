# Phase 4: Testing

**Parent**: `docs/planning/feature-pipe-dup/`  
**Team**: TEAM_233  
**Status**: Not Started

## Test Strategy

### Integration Test: pipe_test

**Location**: `userspace/levbox/src/bin/pipe_test.rs`

**Test Cases**:

1. **Basic Pipe Communication**
   - Create pipe with pipe2
   - Write data to write end
   - Read data from read end
   - Verify data matches

2. **EOF Detection**
   - Create pipe
   - Close write end
   - Read should return 0 (EOF)

3. **dup Test**
   - Open a file or stdout
   - dup it
   - Write to both fds
   - Verify both work

4. **dup2/dup3 Redirect Test**
   - Create pipe
   - Use dup3 to redirect stdout to pipe write end
   - Print something via stdout
   - Read from pipe read end
   - Verify output captured

---

## Test Program: pipe_test

```rust
// Pseudocode for pipe_test

fn test_basic_pipe() -> bool {
    let mut fds = [0i32; 2];
    if pipe2(&mut fds, 0) < 0 { return false; }
    
    let msg = b"Hello pipe!";
    write(fds[1], msg);
    
    let mut buf = [0u8; 32];
    let n = read(fds[0], &mut buf);
    
    close(fds[0]);
    close(fds[1]);
    
    &buf[..n] == msg
}

fn test_dup() -> bool {
    let mut fds = [0i32; 2];
    pipe2(&mut fds, 0);
    
    let dup_fd = dup(fds[1]);
    write(dup_fd, b"via dup");
    
    // Read from pipe
    let mut buf = [0u8; 32];
    let n = read(fds[0], &mut buf);
    
    close(fds[0]);
    close(fds[1]);
    close(dup_fd);
    
    n == 7
}
```

---

## Manual Verification Steps

1. **Build the project**:
   ```bash
   cargo xtask build all
   ```

2. **Run in QEMU**:
   ```bash
   ./run.sh
   ```

3. **From shell, run test**:
   ```
   # pipe_test
   ```

4. **Expected output**:
   ```
   [pipe_test] Testing basic pipe... PASS
   [pipe_test] Testing dup... PASS
   [pipe_test] Testing dup3 redirect... PASS
   [pipe_test] All tests passed!
   ```

---

## Deliverables
- [ ] pipe_test binary in levbox
- [ ] All tests passing
