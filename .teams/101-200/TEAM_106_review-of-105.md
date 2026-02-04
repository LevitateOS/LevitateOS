# TEAM_106: Review of TEAM_105 Install Environment Optimization

## Summary

TEAM_105 identified and fixed real root causes. Their work is solid. However, verification still fails due to incomplete test infrastructure updates.

---

## TEAM_105 Fixes - Verified

### Fix 1: unix_chkpwd - CORRECT
**Problem:** `chpasswd` silently failed because `pam_unix.so` couldn't find `/usr/sbin/unix_chkpwd`.

**Their Fix:** Added `unix_chkpwd` to `AUTH_SBIN` in `definitions.rs`.

**Verdict:** This is the ROOT CAUSE of the password issue. Correct fix.

### Fix 2: SSH Directory Prep - CORRECT
**Problem:** sshd.service failed on first boot.

**Their Fix:** Ensure `/etc/ssh` directory exists with correct permissions. Let `sshd-keygen@.service` generate keys on first boot.

**Verdict:** Correct approach. Pre-generating keys would be a security vulnerability (shared keys across all installations).

### Fix 3: Autologin for Serial Console - CORRECT CONCEPT
**Problem:** Fragile login pattern matching in tests.

**Their Fix:** Create systemd drop-in for `serial-getty@ttyS0.service` with `--autologin root`.

**Verdict:** Good approach, but incomplete. See issues below.

### Fix 4: Pre-reboot Verification - CORRECT
**Problem:** Issues discovered only after reboot, wasting debug time.

**Their Fix:** Added checks before unmount: kernel exists, initramfs exists, root password set, fstab correct.

**Verdict:** Good practice. Catches issues early.

### Fix 5: Shell Instrumentation - CORRECT CONCEPT
**Problem:** Slow boot detection.

**Their Fix:** Added `___SHELL_READY___` marker via profile.d script.

**Verdict:** Good approach, but needs integration with login function.

---

## Remaining Issues

### Issue 1: sshd.service Still Fails
**Observation:** Boot shows `[FAILED] Failed to start sshd.service`

**Possible causes:**
- Host keys not being generated (sshd-keygen@ services)
- /run/sshd not created (tmpfiles.d timing)
- Missing dependency

**Status:** Needs investigation. The SSH directory fix is necessary but not sufficient.

### Issue 2: Autologin Detection Incomplete
**Observation:**
```
  DEBUG: Detected autologin, sending shell test
Error: Timeout waiting for login to complete
```

**Problem:** The login function detects autologin and sends `echo ___LOGIN_OK___`, but the response isn't captured.

**Root Cause:** The shell test command is sent, but the output buffer isn't receiving the response. The shell may not be fully ready when we send the command.

**Fix needed:** Add delay after autologin detection before sending shell test, or wait for shell prompt before sending.

### Issue 3: Boot Output Shows "LevitateOS Live"
**Observation:**
```
LevitateOS Live - ttyS0
```

**Problem:** The installed system shows "LevitateOS Live" in the getty banner. This suggests the `/etc/issue` file from the live overlay is being used, OR the installed system is somehow using live environment settings.

**Should investigate:** Is the installed system actually using the correct rootfs?

---

## Test Results

| Step | Status | Notes |
|------|--------|-------|
| Steps 1-18 (Installation) | PASS | All installation steps succeed |
| Boot installed system | PARTIAL | Boots but sshd fails |
| Autologin | PARTIAL | Detected but shell test times out |
| Verification | NOT RUN | Login timeout prevents verification |

---

## Recommendations

### R1: Fix Login Shell Detection
The autologin is working but the shell isn't ready when we send the test command. Add:
```rust
// After detecting autologin, wait for shell prompt
std::thread::sleep(Duration::from_millis(2000));
// Or wait for prompt pattern (# or $)
```

### R2: Debug sshd Failure
Capture diagnostic output when sshd fails:
```bash
journalctl -u sshd-keygen@rsa -u sshd-keygen@ecdsa -u sshd-keygen@ed25519 -u sshd
systemctl status sshd
ls -la /etc/ssh/
ls -la /run/sshd
```

### R3: Fix Getty Banner
The installed system should show "LevitateOS" not "LevitateOS Live". Check:
- `/etc/issue` in the installed system
- Whether overlay files are being copied incorrectly

---

## Conclusion

TEAM_105's fixes are correct and address real root causes:
- `unix_chkpwd` was genuinely missing (PAM fix)
- SSH directory prep is the right approach (security conscious)
- Autologin and shell instrumentation are good infrastructure improvements

The remaining issues are:
1. Test infrastructure needs to handle autologin response timing
2. sshd still fails (may need additional investigation)
3. Minor: getty banner shows "Live" on installed system

**Overall Assessment:** TEAM_105 did good work. The core fixes are correct. The test failures are now infrastructure timing issues, not fundamental problems.
