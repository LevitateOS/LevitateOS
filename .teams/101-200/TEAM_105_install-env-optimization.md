# TEAM_105: Install Environment Optimization

## Mission
Fix root causes of test failures - NO WORKAROUNDS.

## Summary of REAL Fixes

### Fix 1: PAM - Missing `unix_chkpwd` binary

**Root Cause:** `pam_unix.so` has a hardcoded path to `/usr/sbin/unix_chkpwd`. This binary was in the source rootfs but NOT being copied to the squashfs. When `chpasswd` ran, PAM silently failed.

**Real Fix:** Added `unix_chkpwd` to `AUTH_SBIN` in `definitions.rs`

**File:** `leviso/src/component/definitions.rs`
```rust
const AUTH_SBIN: &[&str] = &["visudo", "unix_chkpwd"];
```

**Result:** `chpasswd` now works properly in chroot.

### Fix 2: SSH - Directory preparation (NOT pre-generated keys)

**Original Problem:** sshd.service failed on first boot.

**WRONG approach (removed):** Pre-generate SSH host keys during build. This is a **security vulnerability** - all installations would share the same keys!

**Real Fix:** Just ensure `/etc/ssh` directory exists with correct permissions. The `sshd-keygen@.service` units will generate unique keys on first boot.

**File:** `leviso/src/component/custom/etc.rs`
```rust
pub fn create_ssh_host_keys(ctx: &BuildContext) -> Result<()> {
    let ssh_dir = ctx.staging.join("etc/ssh");
    fs::create_dir_all(&ssh_dir)?;
    fs::set_permissions(&ssh_dir, fs::Permissions::from_mode(0o755))?;
    Ok(())
}
```

### Fix 3: Autologin for serial console (test infrastructure)

**Purpose:** Test infrastructure only - eliminates fragile login pattern matching.

**File:** `testing/install-tests/src/steps/phase5_boot.rs`

Creates systemd drop-in for `serial-getty@ttyS0.service` with autologin.

### Fix 4: Pre-reboot verification (good practice)

**Purpose:** Catch installation issues BEFORE rebooting, saving debug time.

**File:** `testing/install-tests/src/steps/phase5_boot.rs`

Checks before unmount:
- Kernel exists at /mnt/boot/vmlinuz
- Initramfs exists at /mnt/boot/initramfs.img
- Root password is set (not locked)
- fstab has /boot entry

### Fix 5: Shell instrumentation (test infrastructure)

**Purpose:** Faster boot detection via `___SHELL_READY___` marker.

**Files:**
- `leviso/profile/live-overlay/etc/profile.d/00-levitate-test.sh` (NEW)
- `leviso/src/component/custom/live.rs`
- `testing/install-tests/src/qemu/boot.rs`
- `testing/install-tests/src/qemu/sync.rs`

## Workaround Audit

| Change | Workaround? | Status |
|--------|-------------|--------|
| Add `unix_chkpwd` to AUTH_SBIN | **NO** - Real fix | Done |
| SSH directory creation | **NO** - Real fix | Done |
| Pre-generated SSH keys | **YES** - Removed | Security fix |
| Autologin drop-in | No - Test infra | Done |
| Pre-reboot verification | No - Good practice | Done |
| Shell instrumentation | No - Test infra | Done |

## Files Modified

| File | Change |
|------|--------|
| `leviso/src/component/definitions.rs` | Added `unix_chkpwd` to AUTH_SBIN |
| `leviso/src/component/custom/etc.rs` | SSH directory prep (no key generation) |
| `testing/install-tests/src/steps/phase4_config.rs` | Use proper `chpasswd`, verify password set |
| `testing/install-tests/src/steps/phase5_boot.rs` | Autologin, pre-reboot verification |
| `leviso/src/component/custom/live.rs` | Include test mode script |
| `leviso/profile/live-overlay/etc/profile.d/00-levitate-test.sh` | NEW - Test mode instrumentation |
| `testing/install-tests/src/qemu/boot.rs` | ___SHELL_READY___ detection |
| `testing/install-tests/src/qemu/sync.rs` | New marker filtering |

## Testing

```bash
cd leviso && cargo run -- build    # Rebuild squashfs with unix_chkpwd
cd testing/install-tests && cargo run   # Run installation tests
```

## Is This An Arch Contender Now?

After these fixes:
- PAM works properly (like Arch)
- SSH keys are generated on first boot (like Arch)
- No security-compromising workarounds
- systemd services work as designed

**YES** - these are real fixes, not workarounds.
