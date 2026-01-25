# Knowledge: Login Prompt Debugging

## The Problem Pattern

Install tests fail at login with:
```
Error: Authentication failed: Timeout after 15s while waiting for login prompt
State: WaitingForLoginPrompt
Last output:
LevitateOS Live - ttyS0
```

## THE ISSUE IS NOT TIMEOUTS

If you're reading this because "login prompt timeout", **DO NOT increase timeouts**.
The timeout is a symptom, not the cause.

## What's Actually Happening

1. System boots successfully to multi-user.target
2. getty.target is reached
3. Serial getty starts and prints /etc/issue banner: "LevitateOS Live - ttyS0"
4. **BUT**: the "login:" prompt never appears after the banner
5. Test waits 15s, never sees "login:", times out

This is NOT a timing problem. The "login:" prompt simply never gets printed.

## Current Observations (2026-01-24)

From the test output:
```
[  OK  ] Reached target getty.target - Login Prompts.
[  OK  ] Reached target multi-user.target - Multi-User System.
[!p]104[?7h[6n[32766;32766H[6n[!p]104[?7h[6n[32766;32766H[6n
LevitateOS Live - ttyS0
```

Key observations:
1. Terminal escape sequences `[6n` = cursor position queries being sent
2. These appear BEFORE the banner (or interspersed with it)
3. Banner prints successfully
4. "login:" never follows

The `[6n` sequences are VT100 "Device Status Report" requests that expect a response.
The test harness pipe may not be responding to these, but this doesn't explain
why the banner prints but login: doesn't.

## ROOT CAUSE IDENTIFIED (2026-01-24)

### Live ISO vs Installed System

**Live ISO (works)**:
- Uses custom `serial-console.service` from live-overlay
- Has `Environment=TERM=vt100` explicitly set
- `ExecStart=/bin/bash --login` - runs bash directly, NO agetty/login
- This completely bypasses the standard getty flow

**Installed System (fails)**:
- Uses standard `serial-getty@ttyS0.service`
- No TERM set: `${TERM}` in ExecStart is empty
- `ExecStart=-/sbin/agetty ... ${TERM}` - agetty with empty TERM
- When TERM is empty, agetty tries to auto-detect terminal type
- Auto-detection sends escape sequences and waits for responses
- Test harness pipe doesn't respond → agetty hangs

### The Smoking Gun

In `/usr/lib/systemd/system/serial-getty@.service`:
```ini
ExecStart=-/sbin/agetty -o '-- \\u' --noreset --noclear --keep-baud 115200,57600,38400,9600 - ${TERM}
```

The `${TERM}` is empty because there's no `Environment=TERM=...` line.

Compare to live-overlay's `/etc/systemd/system/serial-console.service`:
```ini
Environment=TERM=vt100
ExecStart=/bin/bash --login
```

### Fix Options

1. **Add drop-in to set TERM for serial-getty** (squashfs change):
   Create `/etc/systemd/system/serial-getty@.service.d/term.conf`:
   ```ini
   [Service]
   Environment=TERM=vt100
   ```

2. **Use same autologin approach as live ISO** (not recommended for installed system)

3. **Fix test harness to respond to terminal queries** (complex)

### Why This Wasn't Caught Before

The live ISO testing always passed because it uses the autologin service that
bypasses agetty. Only the installed system verification (Phase 6) uses standard
getty, which is why the failure only appears after reboot.

## Common Root Causes

### 1. PAM Authentication Issues
If PAM is misconfigured, getty may fail silently before outputting "login:".
Check:
- `/etc/pam.d/login` exists and is valid
- `/etc/pam.d/system-auth` exists and is valid
- `pam_unix.so` library is present

### 2. /etc/issue vs /etc/issue.net
The installed system shows "LevitateOS Live" but should show "LevitateOS" (not Live).
This suggests `/etc/issue` was copied from live system incorrectly.

### 3. Serial Console Getty Not Starting
Check if `serial-getty@ttyS0.service` actually starts:
```bash
systemctl status serial-getty@ttyS0.service
journalctl -u serial-getty@ttyS0.service
```

### 4. Missing /bin/login or /sbin/agetty
If these binaries are missing, getty can't prompt for login.

## How to Debug

1. **Boot the installed system manually** (`leviso run` from disk, not ISO)
2. **Check serial console output in QEMU window** - not through the test harness
3. **Add debug output to /etc/profile or /etc/issue**
4. **Check systemd journal** for getty/login errors

## Previous False Solutions (DO NOT REPEAT)

| Wrong Fix | Why It's Wrong |
|-----------|----------------|
| Increase timeout from 3s to 10s | "login:" never appears - waiting longer doesn't help |
| Increase timeout from 10s to 30s | Same issue - the prompt is missing, not delayed |
| Add retry logic | You can't retry something that never happens |
| Change auth config | Auth only matters AFTER "login:" appears |

## Actual Investigation Steps

1. Check if agetty process is running: `ps aux | grep agetty`
2. Check if /bin/login exists: `ls -la /bin/login`
3. Check if PAM config is valid: `pamtester login root authenticate`
4. Check systemd journal for getty: `journalctl -u serial-getty@ttyS0`
5. Check /etc/issue content: `cat /etc/issue`

## When This Is Fixed

The install test should see output like:
```
LevitateOS - ttyS0

levitateos login:
```

NOT:
```
LevitateOS Live - ttyS0
[escape sequences with no login prompt]
```
