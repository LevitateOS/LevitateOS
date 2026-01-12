# Phase 4: Integration & Testing - BusyBox Integration

**Feature:** Replace uutils-coreutils, dash, and custom init with BusyBox  
**Team:** TEAM_449  
**Status:** Ready (after Phase 3)

---

## Testing Strategy

### 1. Build Verification

```bash
# Verify BusyBox builds
cargo xtask build busybox

# Check binary
file toolchain/busybox-out/x86_64/busybox
# Expected: "ELF 64-bit LSB executable... statically linked"

ls -la toolchain/busybox-out/x86_64/busybox
# Expected: ~1-1.5MB

# Test applets work on host
./toolchain/busybox-out/x86_64/busybox --list | head -20
./toolchain/busybox-out/x86_64/busybox echo "test"
./toolchain/busybox-out/x86_64/busybox ash -c "echo hello"
```

### 2. Initramfs Verification

```bash
# Build initramfs
./scripts/make_initramfs.sh x86_64

# Check contents
mkdir -p /tmp/initrd_check
cd /tmp/initrd_check
cpio -idv < /path/to/initramfs.cpio

# Verify structure
ls -la bin/
ls -la sbin/
ls -la etc/
cat etc/inittab
```

### 3. Boot Test

```bash
# Run in QEMU
cargo xtask run --arch x86_64

# Expected boot sequence:
# 1. Kernel boots
# 2. "LevitateOS (BusyBox) starting..."
# 3. Shell prompt appears: "LevitateOS# "
```

### 4. Functional Tests

Run these commands in the booted system:

```bash
# Shell basics
echo "Hello World"
pwd
cd /tmp
pwd

# File operations
touch /tmp/test.txt
echo "content" > /tmp/test.txt
cat /tmp/test.txt
ls -la /tmp
rm /tmp/test.txt

# Text processing
echo "hello" | grep hello
echo -e "b\na\nc" | sort
echo "hello world" | cut -d' ' -f2

# Process info
ps
uname -a

# Editor (if enabled)
vi /tmp/newfile.txt
# (test basic editing)
```

### 5. Job Control Test

```bash
# Background jobs
sleep 100 &
jobs
fg
# Ctrl+C to interrupt

# Ctrl+Z suspend
sleep 100
# Ctrl+Z
jobs
fg
```

### 6. Init Behavior Test

```bash
# Shell respawn test
exit
# Shell should respawn (per inittab ::respawn)

# Shutdown test
halt
# or
reboot
```

---

## Golden Log Updates

### Expected Changes to `tests/golden_boot_x86_64.txt`

Old (custom init):
```
[INIT] PID 1 starting...
[INIT] Spawning dash...
[INIT] Shell spawned as PID 2
```

New (BusyBox init):
```
LevitateOS (BusyBox) starting...
```

### Update Procedure

```bash
# Run with --update flag (SILVER MODE per memories)
cargo xtask test --arch x86_64 --update

# Verify new golden log makes sense
cat tests/golden_boot_x86_64.txt
```

---

## Regression Checklist

| Test | Expected | Status |
|------|----------|--------|
| Kernel boots | Yes | [ ] |
| Init runs | BusyBox init message | [ ] |
| Shell spawns | ash prompt | [ ] |
| `echo` works | Output text | [ ] |
| `ls` works | Lists files | [ ] |
| `cat` works | Shows file contents | [ ] |
| `pwd` works | Shows /root or / | [ ] |
| `cd` works | Changes directory | [ ] |
| `mkdir` works | Creates directory | [ ] |
| `rm` works | Removes files | [ ] |
| `grep` works | Filters text | [ ] |
| Pipes work | `echo hi \| cat` | [ ] |
| Redirects work | `echo hi > file` | [ ] |
| Job control | Ctrl+C, Ctrl+Z | [ ] |
| Shell exit | Respawns (inittab) | [ ] |

---

## Performance Verification

### Binary Size

| Component | Old Size | New Size | Delta |
|-----------|----------|----------|-------|
| init | ~150KB | 0 (removed) | -150KB |
| dash | ~150KB | 0 (removed) | -150KB |
| coreutils | ~2MB | 0 (removed) | -2MB |
| busybox | 0 | ~1MB | +1MB |
| **Total** | ~2.3MB | ~1MB | **-1.3MB** |

### Boot Time

Measure with:
```bash
time cargo xtask run --arch x86_64 --timeout 10
```

Should be similar or faster (fewer binaries to load).

---

## Known Issues to Watch

### 1. TTY Not Ready

If shell doesn't get input, check:
- Kernel TTY initialization
- Foreground process group setting

### 2. Missing Applets

If command not found:
- Check symlink exists in initramfs
- Check applet enabled in BusyBox config
- Run `busybox --list` to see available applets

### 3. Init Respawn Loop

If shell respawns too fast:
- Check shell is actually starting
- Add delay in inittab: `::wait:/bin/sleep 1`

### 4. Signals Not Working

If Ctrl+C doesn't work:
- Verify kernel signal delivery (TEAM_447 work)
- Check terminal is in correct mode

---

## Test Initramfs (Q3 Answer: Separate)

Per user decision, testing uses a **separate test initramfs** rather than embedding test logic in production init.

### Test Initramfs Structure

```
test_initrd_root/
├── init -> bin/busybox
├── bin/
│   ├── busybox
│   ├── sh -> busybox
│   └── ... (minimal applets)
├── test-runner              # Test binary (e.g., libsyscall-tests)
└── etc/
    └── inittab              # Runs test-runner instead of shell
```

### Test Inittab

```
# /etc/inittab for test mode
::sysinit:/bin/echo "LevitateOS Test Mode"
::sysinit:/test-runner
::shutdown:/sbin/poweroff -f
```

### Build Command

```bash
# Production initramfs
./scripts/make_initramfs.sh x86_64

# Test initramfs (separate script)
./scripts/make_test_initramfs.sh x86_64
```

### Implementation Notes

- Test initramfs is smaller (fewer applets needed)
- Test runner is the main process, not shell
- Auto-poweroff after tests complete
- No /proc or /sys mounting needed for basic syscall tests

---

## Phase 4 Checklist

- [ ] Build verification passed
- [ ] Initramfs structure correct
- [ ] Boot test passed
- [ ] Functional tests passed
- [ ] Job control works
- [ ] Golden logs updated
- [ ] Performance verified
- [ ] No regressions
