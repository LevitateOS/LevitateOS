# Phase 4: Integration - Dash Shell Support

## Integration Points

### 1. xtask CLI

New commands integrate with existing pattern:
```bash
cargo xtask build musl-sysroot  # Build musl C library
cargo xtask build dash          # Build dash shell
cargo xtask build all           # Now includes dash (if musl-sysroot present)
```

### 2. Initramfs

Dash binary included alongside existing binaries:
```
initrd_root/
├── init           # Bare-metal init (existing)
├── shell          # Bare-metal shell (existing)
├── coreutils      # uutils coreutils (existing)
├── brush          # Rust bash shell (existing)
├── dash           # NEW: C dash shell
└── hello.txt
```

### 3. Kernel Syscall Table

New syscalls:
- `wait3` (x86_64: 61, aarch64: uses wait4)
- `wait4` (x86_64: 61 via wait3, aarch64: 260)

## Test Strategy

### Smoke Tests

**T1: Dash boots**
```bash
# In VM
$ /dash
$
# Expected: prompt appears without crash
```

**T2: Dash executes command**
```bash
$ /dash -c "echo hello"
hello
# Expected: prints "hello"
```

**T3: Dash runs coreutils**
```bash
$ /dash -c "/coreutils cat /hello.txt"
Hello from initramfs!
```

**T4: Dash pipe**
```bash
$ /dash -c "echo test | /coreutils cat"
test
```

**T5: Dash exit**
```bash
$ /dash -c "exit 42"
# Expected: exit code 42
```

### Behavioral Tests

Add to `tests/golden/`:
- `dash-boot.golden` - Dash startup output
- `dash-echo.golden` - Echo command output

### Regression Tests

- Verify dash binary is statically linked: `file dash | grep static`
- Verify dash binary size < 200KB
- Verify no glibc symbols: `nm dash | grep -v musl`

## CI Integration

### Required Packages

Add to CI workflow:
```yaml
- name: Install musl toolchain
  run: |
    sudo apt-get update
    sudo apt-get install -y musl-dev musl-tools clang
```

### Build Matrix

```yaml
strategy:
  matrix:
    arch: [x86_64, aarch64]
    shell: [brush, dash]
```

## Impact Analysis

### What Changes

| Component | Impact |
|-----------|--------|
| Kernel | New wait3/wait4 syscalls |
| xtask | New build commands |
| CI | New dependencies |
| Initramfs | Larger (adds ~120KB) |

### What Doesn't Change

- Existing Rust app builds
- c-gull sysroot
- brush functionality
- coreutils builds

### Backward Compatibility

- `cargo xtask build all` still works (dash is optional)
- Existing initramfs works without dash
- Tests continue to pass

## Rollout Plan

1. **Phase A**: Implement wait3/wait4 in kernel (can test independently)
2. **Phase B**: Add musl sysroot build (can test independently)
3. **Phase C**: Add dash build (depends on A, B)
4. **Phase D**: Add to CI (after manual verification)

## Verification Checklist

Before declaring complete:
- [ ] `cargo xtask build musl-sysroot` succeeds
- [ ] `cargo xtask build dash` succeeds
- [ ] Dash binary is static and < 200KB
- [ ] Dash boots in QEMU
- [ ] Dash executes `echo hello`
- [ ] Dash executes piped commands
- [ ] Dash exits cleanly
- [ ] CI builds pass for x86_64
- [ ] CI builds pass for aarch64
- [ ] All existing tests still pass
