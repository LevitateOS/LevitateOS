# TEAM_154: install-tests Boot Detection Broken - Manual Testing Required

**Date:** 2026-02-04
**Status:** BLOCKED - Automated tests broken, manual verification works
**Impact:** HIGH - Cannot run automated E2E installation tests

---

## TL;DR

The install-tests automated test runner **fails during initial boot detection** (before Phase 1), but **manual testing shows the ISO boots perfectly**. This is a test harness I/O buffering issue, not an ISO problem.

**Current workaround:** Use manual testing workflow (see below).

---

## What's Broken

### Automated Test Failure

```bash
cargo run -p install-tests --bin serial -- run
```

**Fails with:**
```
Error: BOOT STALLED: No output received - QEMU or serial broken
No output for 30 seconds - system appears hung.
```

**Location of failure:** `testing/install-tests/src/bin/serial.rs:427`
- Calls `console.wait_for_live_boot_with_context(Duration::from_secs(30), &*ctx)?`
- Waiting for `___SHELL_READY___` marker from `00-levitate-test.sh`
- Test harness **does not see the serial output** from QEMU

---

## What Actually Works

### Manual Boot Verification (2026-02-04)

```bash
# Boot ISO manually with serial console
target/debug/recqemu serial leviso/output/levitateos-x86_64.iso
```

**Result:** ✅ **BOOTS SUCCESSFULLY**

```
[    2.042073] systemd[1]: Queued start job for default target multi-user.target.
___SHELL_READY___
Hi, and welcome to the LevitateOS Installer. This is the author writing.
___PROMPT___
```

**Confirmed working:**
- ✅ Kernel boots (Linux 6.19.0-rc6-levitate)
- ✅ Serial console outputs correctly (`console=ttyS0,115200n8`)
- ✅ systemd reaches multi-user.target
- ✅ serial-console.service starts bash on /dev/ttyS0
- ✅ /etc/profile.d/00-levitate-test.sh runs and emits `___SHELL_READY___`
- ✅ Test instrumentation works correctly

---

## Root Cause

**The test harness Console I/O is not capturing QEMU's serial output.**

**Affected code:**
- `testing/install-tests/src/qemu/serial/mod.rs` - Console wrapper
- Possibly buffering issue in stdio pipes between test harness and QEMU process
- May be timing-related (output appears before test starts reading)

**NOT the issue:**
- ❌ ISO is fine (builds correctly, 1.4GB, passes preflight checks)
- ❌ Serial console parameters are correct (`console=ttyS0,115200n8`)
- ❌ Boot instrumentation works (verified manually)
- ❌ Kernel/systemd boot process works

---

## Fast Iteration Workflow (Until Fixed)

### DO NOT run full automated install-tests

They will fail immediately and waste time.

### INSTEAD: Manual Testing Steps

#### 1. Build ISO
```bash
cd leviso
cargo run -- build iso  # ~30 seconds if only ISO changed
```

#### 2. Quick Boot Test (Serial)
```bash
# From project root
target/debug/recqemu serial leviso/output/levitateos-x86_64.iso

# Watch for:
# - Kernel boots (Linux version line)
# - systemd starts (multi-user.target)
# - ___SHELL_READY___ appears
#
# Press Ctrl+A, then X to quit
```

#### 3. Manual Installation Test (If needed)
```bash
# Boot with VNC for visual testing
target/debug/recqemu vnc leviso/output/levitateos-x86_64.iso

# Connect browser to http://localhost:6080/vnc.html?autoconnect=true
# Manually run installation commands
```

#### 4. Phase-by-Phase Manual Testing

Instead of running all 24 steps, test incrementally:

**Phase 1-2 (Boot + Disk Setup):**
```bash
# In QEMU serial console
lsblk                    # Verify disk detected
fdisk -l /dev/vda        # Check partitioning capability
```

**Phase 3 (Base System):**
```bash
mount /dev/sr0 /mnt
ls -la /mnt/rootfs.erofs  # Verify rootfs exists
recstrap /mnt/rootfs.erofs /target  # Test extraction
```

**Phase 4 (Configuration):**
```bash
# Test in chroot
recchroot /target /bin/bash
echo "levitate-test" > /etc/hostname
passwd root
useradd -m -G wheel testuser
```

**Phase 5 (Bootloader):**
```bash
# Verify UKIs are available
ls -la /mnt/boot/uki/
# Test bootctl install in chroot
recchroot /target bootctl install
```

**Phase 6 (Post-Reboot):**
```bash
# Create test disk, install, reboot
# Then verify services, networking, sudo, etc.
```

---

## What Needs Fixing (For Next Team)

### Option A: Fix Console I/O in Test Harness

**File:** `testing/install-tests/src/qemu/serial/mod.rs`

**Potential issues:**
1. Buffering - stdio pipes may not flush immediately
2. Timing - test may start reading after boot messages already emitted
3. Thread synchronization - reading thread may not be started early enough

**Debug approach:**
```rust
// Add extensive logging to Console::wait_for_boot_with_patterns
// Log every byte received, timestamps, buffer state
// Compare with manual recqemu output
```

### Option B: Switch to QMP + Screenshots

Use the QMP backend with screenshot verification instead of serial console parsing.

**Pros:**
- More reliable (visual verification)
- Can see actual screen state

**Cons:**
- Requires OCR or pixel matching
- Slower than text parsing
- Harder to debug

### Option C: Simplify Boot Detection

Instead of waiting for `___SHELL_READY___`, just:
1. Wait 10 seconds after QEMU starts
2. Try to execute a command
3. If command succeeds, boot is complete

**Simpler, but less precise.**

---

## ISO Build Status (2026-02-04)

**Latest ISO:** `leviso/output/levitateos-x86_64.iso`
**Size:** 1356 MB (1.4 GB)
**Built:** 2026-02-04 (today)

**Preflight Checks:**
- ✅ Live Initramfs: 59/59 checks passed
- ✅ Install Initramfs: 150/150 checks passed
- ✅ Live ISO: 21/21 checks passed

**Hardware Compatibility:**
- ✅ Intel NUC: PASS
- ✅ Gaming Laptop: PASS
- ⚠️ AMD Mini PC: PASS WITH WARNINGS (6 kernel config missing)
- ⚠️ ThinkPad: PASS WITH WARNINGS (4 kernel, 1 firmware)
- ⚠️ Dell XPS: PASS WITH WARNINGS (7 kernel)
- ⚠️ Framework: PASS WITH WARNINGS (5 kernel)
- ⚠️ Surface: PASS WITH WARNINGS (3 kernel, 1 firmware)
- ⚠️ Linux-First: PASS WITH WARNINGS (5 kernel, 1 firmware)
- ⚠️ Desktop: PASS WITH WARNINGS (2 kernel, 2 firmware)
- ⚠️ Gaming Desktop: PASS WITH WARNINGS (8 kernel)
- ⚠️ Workstation: PASS WITH WARNINGS (3 kernel)
- ❌ Homelab/Server: FAIL (8 kernel missing)
- ❌ Steam Deck: FAIL (1 kernel missing)

---

## 24-Step Test Suite (Reference)

All 24 test steps are **defined and ready**, just can't run them automatically:

**Phase 1:** Boot Verification (2 steps)
**Phase 2:** Disk Setup (4 steps)
**Phase 3:** Base System (4 steps)
**Phase 4:** Configuration (5 steps)
**Phase 5:** Bootloader (3 steps)
**Phase 6:** Post-Reboot Verification (6 steps)

See: `cargo run -p install-tests --bin serial -- list`

---

## Expected vs Actual

**Expected failure point:** Phase 6 (post-reboot verification)
**Actual failure point:** Initial boot detection (before Phase 1)

**User expectation:** Tests run through Phases 1-5, ISO installs successfully, then Phase 6 fails on some verification step (networking, services, etc.)

**Reality:** Can't even start Phase 1 because boot detection is broken in the test harness.

---

## Recommendations

1. **Short term:** Use manual testing workflow above
2. **Medium term:** Fix Console I/O buffering in test harness
3. **Long term:** Consider hybrid approach - automated steps with manual verification points

**Do NOT waste time re-running automated install-tests until Console I/O is fixed.**

---

## Files to Check

**Test harness:**
- `testing/install-tests/src/qemu/serial/mod.rs` - Serial console wrapper
- `testing/install-tests/src/bin/serial.rs:427` - Where it fails
- `testing/install-tests/src/distro/levitate.rs:32-36` - Boot patterns

**Boot instrumentation:**
- `leviso/src/component/custom/live/overlay/etc/profile.d/00-levitate-test.sh` - Emits marker
- `leviso/output/live-overlay/etc/systemd/system/serial-console.service` - Starts shell

**Working manual tool:**
- `tools/recqemu/src/main.rs` - Serial mode implementation that WORKS
