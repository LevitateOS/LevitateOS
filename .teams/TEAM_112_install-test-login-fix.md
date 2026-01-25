# TEAM_112: Install Test Login Prompt Fix

## Session: 2026-01-24

## Summary

Attempting to fix install test login failure. The installed system boots but the
"login:" prompt never appears after the banner.

## What Was Done

### 1. Build Atomicity Fix (squashfs.rs) - COMPLETED
- Implemented Gentoo-style work directory pattern
- Build now uses `.work` files, only swaps to final on success
- File: `leviso/src/artifact/squashfs.rs`
- Status: ✅ Working, build is now atomic

### 2. Root Cause Analysis - COMPLETED
- Identified that live ISO works because it uses custom `serial-console.service`
  with `Environment=TERM=vt100` and direct bash exec (no agetty)
- Installed system uses standard `serial-getty@.service` which has `${TERM}` empty
- Empty TERM causes agetty to send terminal queries (`[6n`) that hang the login flow
- Documented in `.teams/KNOWLEDGE_login-prompt-debugging.md`

### 3. Serial Getty TERM Fix - ATTEMPTED BUT NOT WORKING
- Added drop-in file creation to GETTY component in `definitions.rs`:
  ```rust
  dirs(&["etc/systemd/system/serial-getty@.service.d"]),
  write_file(
      "etc/systemd/system/serial-getty@.service.d/term.conf",
      SERIAL_GETTY_TERM_CONF,  // sets Environment=TERM=vt100
  ),
  ```
- Verified drop-in exists in squashfs-root:
  `/etc/systemd/system/serial-getty@.service.d/term.conf`
- Rebuilt squashfs and ISO
- Test still fails with same symptoms (escape sequences still present)

## What's Not Working

The drop-in is in the squashfs but the terminal escape sequences still appear:
```
[!p]104[?7h[6n[32766;32766H[6n[!p]104[?7h[6n[32766;32766H[6n
LevitateOS Live - ttyS0
```

Possible reasons:
1. Drop-in not being read by systemd (permissions? syntax?)
2. Drop-in needs `systemctl daemon-reload` after extraction
3. The escape sequences are coming from somewhere else (kernel? login?)
4. The recstrap extraction doesn't preserve the drop-in directory

## Current Status (Latest)

### Second Fix Attempt: Add `-L` flag (CLOCAL)
- Root cause: agetty on serial ports waits for modem carrier detect (DCD)
- QEMU virtual serial ports don't provide DCD signal
- Fix: Override ExecStart to add `-L` (local line - ignore modem control)
- Drop-in file: `etc/systemd/system/serial-getty@.service.d/local.conf`
- Status: **Deployed but NOT YET VERIFIED**

### New Blocker: efibootmgr failure
The test now fails at step 17 (Install Bootloader) with:
```
efibootmgr failed: EFI variables are not supported on this system.
```

This is caused by recent commit `3c70ce2` which changed tests to boot through
real UEFI instead of direct kernel boot. Even though efivars exists, it's
not writable from the live environment.

This is a **test infrastructure issue**, not a LevitateOS bug. The `-L` fix
cannot be verified until the efibootmgr issue is resolved.

### Options to Proceed

1. **Fix efibootmgr issue**: Investigate why efivars isn't writable in QEMU
   - May need OVMF configuration changes
   - May need to mount efivars read-write

2. **Revert to direct kernel boot temporarily**: Remove the UEFI requirement
   for testing until properly configured

3. **Skip efibootmgr step**: Fallback boot path works, efibootmgr is nice-to-have

## Files Modified

- `leviso/src/artifact/squashfs.rs` - atomicity fix
- `leviso/src/component/definitions.rs` - serial-getty drop-in with `-L` flag
- `.teams/KNOWLEDGE_login-prompt-debugging.md` - root cause documentation

## Drop-in Content (in squashfs)

```ini
[Service]
# Override ExecStart to add -L (local line - ignore modem control signals).
ExecStart=
ExecStart=-/sbin/agetty -L -o '-- \\u' --noreset --noclear --keep-baud 115200,57600,38400,9600 %I vt100
```

## Commands

```bash
# Verify drop-in in squashfs
cat leviso/output/squashfs-root/etc/systemd/system/serial-getty@.service.d/local.conf

# Force rebuild and test
rm -rf leviso/output/squashfs-root leviso/output/filesystem.squashfs
cargo run -- build squashfs
cargo run -- build iso
cd testing/install-tests && cargo run --bin install-tests -- run
```
