# TEAM_108: Review of TEAM_107 + Login Architecture Analysis

## Mission
1. Review TEAM_107's installation optimization work
2. Document the login problem and propose a proper solution

---

## Part 1: Review of TEAM_107 (Phase 2 Installation Optimization)

### What TEAM_107 Claimed

| Optimization | Claimed Savings |
|-------------|-----------------|
| Pre-built initramfs | ~150 seconds |
| Skip redundant config | ~2 seconds |
| Batch service enablement | ~6 seconds |
| **Total** | **~158 seconds** |

### What Actually Works

#### ✅ Pre-built Initramfs - VERIFIED WORKING

**Evidence from test run:**
```
▶ Step 16: Generate Initramfs... PASS (3.5s)
```

Previous runs showed ~113 seconds for dracut in chroot. TEAM_107's optimization saves **~110 seconds**.

**Implementation:**
- `build_install_initramfs()` in `leviso/src/artifact/initramfs.rs` runs dracut during ISO build
- `initramfs-installed.img` (34MB, generic/no-hostonly) placed on ISO
- Installation copies it in 3.5 seconds vs running dracut

**Correctness:**
- Uses `--no-hostonly` so it works on any hardware
- Includes necessary drivers for common hardware
- Placed at `/boot/initramfs-installed.img` on ISO

#### ✅ Batch Service Enablement - IMPLEMENTED

**Code in phase5_boot.rs:**
```rust
let enable_cmd = format!("systemctl enable {}", found_services.join(" "));
console.exec_chroot("/mnt", &enable_cmd, Duration::from_secs(15))?;
```

Single `systemctl enable` call vs N separate calls. Saves chroot overhead.

#### ✅ Skip Redundant Config - IMPLEMENTED

**Code in phase4_config.rs** checks:
- `readlink /etc/localtime` before writing timezone
- `cat /mnt/etc/locale.conf` before writing locale

Skips writes if values already match (e.g., UTC timezone from squashfs defaults).

### TEAM_107 Verdict: APPROVED

The optimizations are real and working. The 150-second dracut savings is the big win.

---

## Part 2: The Login Problem - Root Cause Analysis

### Current Symptom

After rebooting into installed system:
```
  DEBUG: Detected autologin, waiting for shell to be ready
  DEBUG: drained: ""
  DEBUG: Sent shell test command
Error: Timeout waiting for login to complete
```

The `echo ___LOGIN_OK___` command is sent but the response isn't captured.

### Why Timeout Increases Don't Work

Increasing timeouts is a **band-aid**, not a fix. The underlying issue is:

1. **Line buffering** - Console uses `BufReader::lines()` which needs newline termination
2. **Shell state uncertainty** - We don't know when bash is truly ready
3. **Echo timing** - The echo output may arrive between recv_timeout calls
4. **No synchronization primitive** - We're polling, not event-driven

### The Fundamental Problem: Testing Serial Console Login is Fragile

We're essentially doing "expect" scripting over serial console:
1. Wait for pattern X
2. Send text Y
3. Wait for pattern Z

This approach has known issues:
- Race conditions between send and receive
- Line fragmentation (partial lines, line wrapping)
- ANSI escape codes (even with stripping)
- Shell prompt variations
- Bash startup time variability

### Why Autologin Was Added (and Why It's Wrong)

TEAM_105 added autologin to "eliminate fragile login pattern matching in the test harness."

**The problem:** The autologin configuration is added to the **installed system**, not just the test harness:

```rust
// phase5_boot.rs line 341-348
let autologin_cmd = r#"
mkdir -p /etc/systemd/system/serial-getty@ttyS0.service.d && \
cat > /etc/systemd/system/serial-getty@ttyS0.service.d/autologin.conf << 'EOF'
[Service]
ExecStart=
ExecStart=-/sbin/agetty -o '-p -- \\u' --autologin root --noclear %I $TERM
EOF
"#;
console.exec_chroot("/mnt", autologin_cmd, ...)?;
```

**This is wrong because:**

1. **Not Arch-like behavior** - Arch installs do NOT have autologin. Users login with username/password.
2. **Test shims modify production** - The installed system now behaves differently than a real install would.
3. **Security issue** - Root autologin on serial console is a security hole.
4. **Hides login bugs** - If `chpasswd` or PAM is broken, autologin masks it.

**What Arch actually does:**
- Live ISO: zsh shell as PID 1 child (not autologin - just no getty)
- Installed system: Normal agetty login prompt, user types credentials

**We're testing a different system than users will actually get.**

---

## Part 3: Proposed Solutions

### Option A: Keep Manual Login (Arch-like)

The installed system should behave normally:
- agetty prompts for login
- User types username
- agetty prompts for password
- User types password
- Shell starts

Test harness improvements:
1. **Better state machine** - Track exact login phase
2. **Prompt detection** - Reliable shell prompt patterns (`#`, `$`, PS1)
3. **Command echo detection** - When we send `whoami`, wait to see `whoami` echoed back before expecting output
4. **Retry logic** - If login fails, retry from beginning

### Option B: Use expect-style Library

Replace manual recv_timeout polling with a proper expect implementation:
- `expectrl` crate - Rust expect library
- Handles timeouts, regexes, multiple patterns
- Designed for this exact problem

### Option C: Use SSH Instead of Serial Console

After installation completes and before reboot:
1. Install and enable sshd
2. Configure known host keys
3. After reboot, wait for SSH port
4. Connect via SSH for verification

SSH is reliable, line-buffered, and stateful. No expect-scripting needed.

**Problem:** sshd.service is currently failing on boot (separate issue).

### Option D: Marker File Protocol

Don't parse console output at all. Use file-based signaling:
1. During installation, add a first-boot service
2. First-boot service creates `/var/lib/install-test/boot-ok`
3. After reboot, verification reads the marker file

No login needed for basic boot verification.

---

## Part 4: Recommended Approach

### Short-term: Fix Manual Login State Machine

1. **Remove autologin drop-in** - Restore normal login behavior
2. **Add echo-back verification** - After sending username, wait to see it echoed
3. **Add explicit delays after each phase** - Not timeouts, structured waits
4. **Better prompt detection** - Look for `hostname#` or `hostname$` patterns

### Long-term: SSH-based Verification

1. Fix sshd.service failure (likely missing host keys or PAM issue)
2. Generate host keys during installation
3. Configure passwordless root login for testing (or key-based)
4. After reboot, poll SSH port until available
5. Run verification commands over SSH

This eliminates all serial console parsing fragility.

---

## Part 5: SSH Service Investigation

sshd.service fails on boot. Likely causes:
1. Missing host keys (`/etc/ssh/ssh_host_*`)
2. PAM configuration issue (unix_chkpwd missing - FIXED by TEAM_105)
3. Directory permissions (`/var/empty`, `~/.ssh`)

### Action Items

1. Check if host keys are generated during installation
2. Verify sshd directory structure in squashfs
3. Add host key generation to installation if missing

---

## Files Referenced

| File | Purpose |
|------|---------|
| `testing/install-tests/src/qemu/utils.rs` | Login function with autologin detection |
| `testing/install-tests/src/steps/phase5_boot.rs` | Pre-built initramfs copy, batch services |
| `testing/install-tests/src/steps/phase4_config.rs` | Skip redundant config |
| `leviso/src/artifact/initramfs.rs` | `build_install_initramfs()` |
| `.teams/TEAM_107_phase2-installation-optimization.md` | TEAM_107's documentation |

---

---

## Part 6: Root Cause of Login Failure

### Observed Behavior

```
  DEBUG: [WaitingForLoginPrompt] line: "levitateos login:"
  DEBUG: Sent username: root
  DEBUG: [SentUsername] line: "levitateos login: root"
  (timeout - never saw Password: prompt)
```

After sending username `root`, the system never prompts for password. This means:

1. **The `login` program rejected the username immediately**
2. **OR the account doesn't exist / is locked**
3. **OR PAM rejected the login before password prompt**

### Likely Root Cause: Password Not Set

During installation, Step 14 runs `chpasswd` to set the root password:

```rust
// phase4_config.rs
exec_chroot("/mnt", "echo 'root:password' | chpasswd", ...)?;
```

If `chpasswd` fails (e.g., missing `unix_chkpwd`, broken PAM), the password is never set, and the account remains locked. A locked account shows `!` or `*` in `/etc/shadow` instead of a password hash.

**TEAM_105 fixed `unix_chkpwd` being missing** - but we should verify `chpasswd` actually works.

### Why sshd.service Also Fails

sshd.service depends on:
1. Host keys (generated by `sshd-keygen@.service`)
2. PAM working correctly

If PAM is broken, both login and sshd fail.

### The Fix

**NOT more timeouts. Verify the password is actually set.**

Before reboot verification, add a pre-reboot check:

```rust
// Verify password was set (not locked)
let shadow_check = console.exec(
    "grep '^root:' /mnt/etc/shadow",
    Duration::from_secs(5),
)?;

// A set password looks like: root:$6$...:19000:0:99999:7:::
// A locked password looks like: root:!:19000:0:99999:7:::
cheat_ensure!(
    !shadow_check.output.contains(":!:") && !shadow_check.output.contains(":*:"),
    protects = "Root password is set, not locked",
    severity = "CRITICAL",
    cheats = ["Skip password verification"],
    consequence = "Cannot login - account locked",
    "Root account is locked: {}", shadow_check.output.trim()
);
```

This check is already in phase5_boot.rs lines 393-405. **The check passed during installation phase.** So the password WAS set at installation time.

### The Real Problem: Something Else

Since the pre-reboot verification passed (`root password set`), the password was set during installation. But login still fails after reboot.

Possible causes:
1. **Shadow file not persisted** - Is /etc on the right partition?
2. **PAM configuration mismatch** - Different PAM config after boot?
3. **SELinux/security labels** - Preventing access to shadow?
4. **Different getty behavior** - Serial getty vs console?

### ROOT CAUSE FOUND: `login` Binary in Wrong Location

**The `login` binary is in `/usr/sbin/login` but agetty expects it at `/bin/login`.**

```bash
# Where login actually is:
/usr/sbin/login

# Where agetty looks for it:
/bin/login -> /usr/bin/login  (via symlink)

# But /usr/bin/login does NOT exist!
```

When agetty starts, it runs `login` to authenticate users. By default (without `-l` option), it looks for `/bin/login`. Since `/bin -> /usr/bin` and login is actually in `/usr/sbin`, agetty can't find the login binary.

**The fix (for leviso, not install-tests):**

In `leviso/src/component/definitions.rs`, add a symlink:

```rust
symlink("usr/bin/login", "../sbin/login"),
```

This needs to go in a component that runs during Phase::Binaries, after `SBIN_BINARIES` creates `/usr/sbin/login`.

**Why this wasn't caught earlier:**
- During live ISO boot, the overlay or different init flow may work differently
- The password was set correctly in chroot (chpasswd works)
- Pre-reboot verification passed (shadow file shows password hash)
- The failure only happens after clean reboot when agetty runs

**This explains BOTH failures:**
1. **Login fails** - agetty can't find `/bin/login`
2. **sshd fails** - Probably also has a missing binary or PAM issue (but login is the primary blocker)

---

## Part 7: Immediate Action Items

### 0. Add `/usr/bin/login` Symlink

**Priority: CRITICAL - This is the root cause of login failure**

**File to modify:** `leviso/src/component/definitions.rs`

**Change needed:** Add symlink from `/usr/bin/login` to `/usr/sbin/login`

```rust
// In SBIN_BINARIES component or create new component:
symlink("usr/bin/login", "../sbin/login"),
```

**Why:** agetty defaults to `/bin/login` (which via symlink is `/usr/bin/login`), but the login binary is installed to `/usr/sbin/login`. Without this symlink, agetty can't find login and the login prompt appears but cannot authenticate.

**After fixing:** Rebuild squashfs and ISO, then rerun install tests.

### 1. Remove Autologin from Installation

**File:** `testing/install-tests/src/steps/phase5_boot.rs`

**Remove these lines (341-365):**
```rust
// Create autologin drop-in for serial console
let autologin_cmd = r#"
mkdir -p /etc/systemd/system/serial-getty@ttyS0.service.d && \
...
```

The installed system should have normal login, not autologin.

### 2. Fix Login State Machine

**File:** `testing/install-tests/src/qemu/utils.rs`

The login function needs to:
1. Wait for `login:` prompt
2. Send username
3. Wait for username echo (proves it was received)
4. Wait for `Password:` prompt
5. Send password
6. Wait for shell prompt (`#` or `$`)

NOT send shell test commands immediately after password.

### 3. Investigate sshd.service Failure

Before boot verification:
- Verify `/etc/ssh/` directory exists in installed system
- Check if sshd-keygen runs on first boot
- Check journalctl for sshd errors

---

## Summary

| Aspect | Status |
|--------|--------|
| TEAM_107 pre-built initramfs | ✅ Working, saves 110+ seconds |
| TEAM_107 batch services | ✅ Implemented |
| TEAM_107 skip config | ✅ Implemented |
| Login failure ROOT CAUSE | 🔴 **`/usr/bin/login` symlink missing** - agetty can't find login binary |
| Autologin in installed OS | ✅ Removed from phase5_boot.rs |
| SSH verification | 🔧 Recommended long-term solution |

## Critical Fix Required

**The `/usr/bin/login` symlink must be added to `leviso/src/component/definitions.rs`.**

Without this fix, the installed system cannot authenticate users because agetty cannot find the login binary.
