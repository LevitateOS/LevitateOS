# Install Test Debugging Session - 2026-01-24

> **NOTE (2026-01-27):** This document references squashfs which has been replaced by EROFS.
> File references like `leviso/src/artifact/squashfs.rs` are now `leviso/src/artifact/erofs.rs`.
> The general debugging approaches and learnings still apply.

## Summary

Debugging session attempting to get E2E installation tests passing. Tests pass all 18 installation steps but fail in the verification phase due to:

1. **sshd.service failing to start** on the installed system
2. **Login timeout** - Password prompt never appears after entering username

---

## Issues Found and Fixed

### Issue 1: Wrong Kernel Path
**Problem:** Test was using Rocky kernel from downloads which lacks built-in virtio drivers.
**Fix:** Changed kernel path to LevitateOS kernel in `output/iso-root/boot/vmlinuz`.
**Files:** `testing/install-tests/src/main.rs`, `testing/install-tests/src/bin/boot-test.rs`

### Issue 2: recstrap refusing non-empty target
**Problem:** `recstrap /mnt` failed with E009 because ext4 format creates `lost+found`.
**Fix:** Added `--force` flag to recstrap command.
**File:** `testing/install-tests/src/steps/phase3_base.rs`

### Issue 3: Missing systemd services in distro-spec
**Problem:** Test expected systemd-networkd/resolved/timesyncd but Rocky 10 uses NetworkManager/chronyd.
**Fix:** Updated `distro-spec/src/levitate/services.rs` to use correct services.

### Issue 4: File ownership in squashfs
**Problem:** Files in squashfs owned by build user (vince) not root.
**Fix:** Added `-all-root` flag to mksquashfs.
**File:** `leviso/src/artifact/squashfs.rs`

### Issue 5: Missing /run/sshd directory
**Problem:** sshd needs `/run/sshd` for privilege separation, but `/run` is tmpfs.
**Fix:** Added tmpfiles.d config to create `/run/sshd` on boot.
**File:** `leviso/src/component/custom/etc.rs`

---

## Issues Still Unresolved

### Issue A: sshd.service Still Failing
**Symptom:** `[FAILED] Failed to start sshd.service - OpenSSH server daemon.`
**Status:** tmpfiles.d config IS in squashfs, but sshd still fails.
**Hypothesis:** Unknown - need to capture journalctl output from installed system.

**What was checked:**
- ssh-keygen binary exists at `/usr/bin/ssh-keygen`
- sshd-keygen script exists at `/usr/libexec/openssh/sshd-keygen`
- sshd binary exists at `/usr/sbin/sshd`
- All PAM modules are present
- tmpfiles.d config is included in squashfs
- Host keys are NOT present (correct - should be generated on first boot)

### Issue B: Login Timeout - No Password Prompt
**Symptom:** After sending username "root", no "Password:" prompt appears.

**Debug Output:**
```
LOGIN_RX: "levitateos login:" (clean: "levitateos login:")
DEBUG: Sent username: root
LOGIN_RX: "levitateos login: root" (clean: "levitateos login: root")
Error: Timeout waiting for login to complete
```

**Root Cause (likely):** The root account in `/etc/shadow` is locked with `!`:
```
root:!:19000:0:99999:7:::
```

The installation step 14 "Set Root Password" passes (chpasswd returns exit 0), but the password is NOT actually being set. The chpasswd command runs in chroot via recchroot:
```bash
recchroot '/mnt' /bin/bash -c 'echo '\''root:levitate'\'' | chpasswd'
```

**Hypothesis:** Either:
1. chpasswd in chroot is silently failing (wrong PAM config? missing libs?)
2. Something is overwriting /etc/shadow after password is set
3. The shadow file is read-only or on a read-only layer

---

## Recommendations for Installation Environment Developers

### R1: Add Password Verification Step
After setting root password with chpasswd, add a verification:
```bash
# After chpasswd, verify the password was actually set
grep '^root:' /mnt/etc/shadow | grep -v '!'
```
This would catch the "chpasswd succeeds but password not set" issue.

### R2: Create Debug Mode for recchroot
Add a `--debug` flag to recchroot that logs:
- All bind mounts created
- All commands executed
- Exit codes and outputs
This would help debug why chpasswd appears to work but doesn't.

### R3: Add sshd Health Check to ISO Build
After building squashfs, run:
```bash
# Verify all sshd dependencies are present
ldd /path/to/squashfs/usr/sbin/sshd
# Verify PAM config
cat /path/to/squashfs/etc/pam.d/sshd
# Verify tmpfiles.d
cat /path/to/squashfs/usr/lib/tmpfiles.d/sshd.conf
```

### R4: Pre-generate SSH Host Keys in Squashfs
Instead of relying on sshd-keygen@ services on first boot, generate host keys during ISO build:
```bash
ssh-keygen -t rsa -f /path/to/staging/etc/ssh/ssh_host_rsa_key -N ''
ssh-keygen -t ecdsa -f /path/to/staging/etc/ssh/ssh_host_ecdsa_key -N ''
ssh-keygen -t ed25519 -f /path/to/staging/etc/ssh/ssh_host_ed25519_key -N ''
```
This eliminates dependency on sshd-keygen services working correctly.

### R5: Add Serial Console Login Without Password (for testing)
Create a test-only serial-getty configuration that auto-logs in as root:
```ini
[Service]
ExecStart=-/sbin/agetty -o '-p -- \\u' --autologin root --noclear %I $TERM
```
Or enable root login without password temporarily for testing.

### R6: Include Debug Tool in Live ISO
Add a simple script `/usr/bin/diagnose-sshd`:
```bash
#!/bin/bash
echo "=== sshd status ==="
systemctl status sshd --no-pager
echo "=== sshd journal ==="
journalctl -u sshd --no-pager -n 50
echo "=== /run/sshd ==="
ls -la /run/sshd 2>&1 || echo "NOT FOUND"
echo "=== Host keys ==="
ls -la /etc/ssh/ssh_host_* 2>&1 || echo "NONE"
echo "=== tmpfiles ==="
cat /usr/lib/tmpfiles.d/sshd.conf 2>&1 || echo "NOT FOUND"
```

### R7: Better Error Pattern Handling
The current test fails immediately on "Failed to start" pattern. Consider:
1. For live ISO boot: Keep fail-fast (current behavior)
2. For installed system boot: Track failures, complete boot, capture diagnostics, THEN fail

This was partially implemented in this session - see `testing/install-tests/src/qemu/boot.rs`.

---

## Code Changes Made

### Files Modified

1. **testing/install-tests/src/main.rs**
   - Changed kernel path to use LevitateOS kernel
   - Added diagnostic capture when services fail during boot

2. **testing/install-tests/src/bin/boot-test.rs**
   - Changed kernel path to use LevitateOS kernel

3. **testing/install-tests/src/steps/phase3_base.rs**
   - Added `--force` flag to recstrap

4. **distro-spec/src/levitate/services.rs**
   - Updated to use NetworkManager/chronyd instead of systemd-networkd/timesyncd

5. **leviso/src/component/definitions.rs**
   - Added ssh-host-keys-migration.service to OPENSSH_SVC units

6. **leviso/src/artifact/squashfs.rs**
   - Added `-all-root` flag to mksquashfs

7. **leviso/src/component/custom/etc.rs**
   - Added sshd tmpfiles.d configuration

8. **testing/install-tests/src/qemu/patterns.rs**
   - Split error patterns into CRITICAL_BOOT_ERRORS and SERVICE_FAILURE_PATTERNS
   - Added new patterns for tracking service failures without immediate bail

9. **testing/install-tests/src/qemu/boot.rs**
   - Added `track_service_failures` parameter to wait_for_boot_with_patterns
   - Added `failed_services()` method

10. **testing/install-tests/src/qemu/console.rs**
    - Added `failed_services` field to Console struct

11. **testing/install-tests/src/qemu/utils.rs**
    - Added debug logging for login process

---

## Next Steps for Debugging

1. **Verify chpasswd works in live environment:**
   Boot live ISO, run `chpasswd` manually, verify with `passwd -S root`

2. **Check if /etc/shadow is on writable layer:**
   During installation, after chpasswd, run `cat /mnt/etc/shadow | grep root`

3. **Test sshd manually:**
   Boot installed system (if login works), run `/usr/sbin/sshd -d -e` for debug output

4. **Check systemd-tmpfiles timing:**
   Verify systemd-tmpfiles-setup.service runs before sshd.service

5. **Capture full journal on boot:**
   Add kernel param `systemd.log_level=debug` for more boot logs

---

## Wasteful Actions to Streamline

### W1: Initramfs Generation (123 seconds - 40% of install time!)
**Current:** Every installation runs `dracut` in chroot to generate initramfs.
**Problem:** The initramfs is nearly identical to what's already on the live ISO - same kernel, same modules.
**Solution:** Pre-generate initramfs during ISO build, copy during install. Or copy from ISO directly:
```bash
cp /media/cdrom/boot/initramfs /mnt/boot/initramfs
```

### W2: chpasswd via PAM (silently fails)
**Current:** `echo 'root:levitate' | chpasswd` - goes through full PAM stack.
**Problem:** Can return success without setting password if PAM is misconfigured in chroot.
**Solution:** Direct shadow manipulation:
```bash
# Generate hash and write directly
HASH=$(openssl passwd -6 'levitate')
sed -i "s|^root:[^:]*:|root:$HASH:|" /mnt/etc/shadow
```
This bypasses PAM entirely and is more reliable in chroot environments.

### W3: Two Full QEMU Boots
**Current:** Boot live ISO → install → shutdown → boot installed system → verify
**Problem:** Second boot just for verification adds 30+ seconds.
**Potential:** Could we verify more before rebooting? Check:
- /mnt/etc/shadow has valid password hash
- /mnt/boot/vmlinuz and initramfs exist
- /mnt/etc/fstab is correct
- Basic file permissions
Only boot installed system for "does it actually boot" verification.

### W4: Serial Console Limitations
**Current:** Login via serial console with pattern matching, timeouts, retries.
**Problem:** Fragile, slow, can't see password prompts if account is locked.
**Solution:** For testing, use autologin on serial console:
```ini
# Override serial-getty@ttyS0.service for installed system
ExecStart=-/sbin/agetty -o '-p -- \\u' --autologin root --noclear %I $TERM
```
Then the test doesn't need login logic at all.

### W6: UEFI Boot Testing is Incomplete
**Current:** Installation uses `-kernel` direct boot (bypasses UEFI), verification relies on UEFI fallback.
**Problem:**
- Installation phase: `QemuBuilder::new().kernel().initrd()` = QEMU bypasses UEFI entirely
- `bootctl install --no-variables` = No EFI boot entries created
- Verification: Uses UEFI fallback scan for `\EFI\BOOT\BOOTX64.EFI`
- OVMF_VARS is never modified, verification uses empty template

**What's NOT tested:**
- Real EFI variable creation
- Boot entry priorities
- Boot fallback behavior
- UEFI secure boot

**Solution:** For verification, we should create proper EFI boot entries:
```bash
# After installation, before reboot, run efibootmgr
efibootmgr --create --disk /dev/vda --part 1 --label "LevitateOS" --loader /EFI/systemd/systemd-bootx64.efi
```
Or don't use `--no-variables` in bootctl and ensure EFI variables can be written.

### W5: Squashfs Extraction (58-95 seconds)
**Current:** `unsquashfs` with default settings.
**Potential optimizations:**
- Use `-processors N` for parallel extraction
- Use faster compression (lz4 instead of zstd) for test builds
- Consider using a tarball instead of squashfs for testing

---

## Key Learnings

1. **chpasswd exit 0 != password set**: The command can succeed without actually setting the password if something in the chroot environment is wrong.

2. **tmpfiles.d ordering matters**: Even with correct config, the service might start before tmpfiles runs if dependencies aren't set correctly.

3. **Serial console buffering**: Password prompts may not appear if the login process rejects the user before asking for password (e.g., account locked).

4. **Rocky 10 vs other distros**: NetworkManager and chronyd are the defaults, not systemd-networkd/timesyncd.

5. **squashfs ownership**: Without `-all-root`, files are owned by the build user, breaking services like sshd that check ownership.
