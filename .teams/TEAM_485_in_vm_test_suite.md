# TEAM_485: In-VM Test Suite

## Goal
Create an automated test suite that runs inside LevitateOS VM.

## Status: BLOCKED - Terminal Escape Code Bug

---

## BUG REPORT: Cursor Position Query Flooding

### Symptoms
When running `cargo xtask test quick`, the user's terminal gets flooded with:
```
[47;6R[47;6R[47;6R[47;6R[47;6R[47;6R[47;6R...
```

This is the ANSI cursor position response (row 47, column 6). Something in the VM boot is sending `ESC[6n` (cursor position query) and the user's host terminal is responding.

### Reproduction Steps

1. Build the test binary:
   ```bash
   cd tools/levitate-test && cargo build --release
   ```

2. Build initramfs:
   ```bash
   cargo run --bin builder -- initramfs
   ```

3. Run the quick test:
   ```bash
   cargo xtask test quick --no-build
   ```

4. Observe terminal flooding with `[47;6R` responses

### What We Tried (All Failed)

1. **Created Rust binary test runner** - `tools/levitate-test/` (353k ELF binary)
   - Verified it's in initramfs: `build/initramfs/bin/levitate-test`
   - NOT a shell script - pure Rust, no shell dependency

2. **Used custom systemd target** - `levitate-test.target`
   - Boots directly to test service, not multi-user
   - Service runs `/bin/levitate-test` directly

3. **Masked getty services** via kernel cmdline:
   ```
   systemd.mask=serial-getty@ttyS0.service systemd.mask=getty@tty1.service
   ```
   - Still getting cursor queries

4. **Tried rescue.target** - same issue

### Root Cause Analysis

The cursor position query `ESC[6n` is being sent by SOMETHING during boot. Candidates:

1. **systemd itself** - May query terminal capabilities during init
2. **agetty** - Even if masked, generator might run before mask applies
3. **Some library** - libsystemd or libc terminal detection
4. **Kernel** - Console initialization?

The problem is NOT brush (shell) because:
- We're running a Rust binary directly
- Getty services are masked
- The binary has no shell dependencies

### Files Created/Modified

1. `tools/levitate-test/` - Standalone Rust test binary (isolated from workspace)
2. `tools/levitate-test/src/main.rs` - Test implementation
3. `crates/builder/src/builder/initramfs.rs` - Updated to copy binary
4. `xtask/src/test/helpers.rs` - Boot with levitate-test.target + masked gettys
5. `xtask/src/test/mod.rs` - Quick test command

### Current Kernel Cmdline
```
console=ttyS0 rw quiet systemd.unit=levitate-test.target systemd.mask=serial-getty@ttyS0.service systemd.mask=getty@tty1.service
```

### Next Steps to Investigate

1. **Boot without serial console** - Use `-nographic` differently or don't use `console=ttyS0`
2. **Check systemd source** - Does systemd query terminal on boot?
3. **Use virtio-console** instead of serial
4. **Write test output to file** - Don't rely on console at all, read file after VM exits

### Workaround Ideas

1. **File-based output**: Write test results to `/test-results.txt`, mount the initramfs after VM exits, read results
2. **QMP-based**: Use QEMU Machine Protocol to inject commands
3. **9p filesystem**: Share a directory between host and VM for results

---

## What Was Built (Working Parts)

### Rust Test Binary
- Location: `tools/levitate-test/`
- Size: 353k (stripped, LTO, panic=abort)
- Tests: coreutils, findutils, diffutils, procps, network, auth, editor, system files
- No dependencies except std

### Systemd Integration
- `levitate-test.target` - Custom boot target
- `levitate-test.service` - Runs test binary, then poweroff
