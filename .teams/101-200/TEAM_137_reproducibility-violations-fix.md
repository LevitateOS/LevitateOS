# TEAM_137: Fix Reproducibility Violations

**Status:** COMPLETED
**Date:** 2026-01-27
**Scope:** Fix manual agent interventions that weren't codified in the build system

## Problem Statement

From the reproducibility-violations audit, 7 cases were found where agents ran manual commands that were never codified. A fresh `cargo run -- build` would not reproduce these fixes.

## Changes Made

### 1. Password Setting (install-tests)

**Problem:** `chpasswd` via PAM silently fails in chroot environments. The command returns exit 0 but doesn't actually modify the shadow file.

**Solution:** Use `openssl passwd -6` to generate SHA-512 hash and `sed` to directly edit `/etc/shadow`.

**Files Changed:**
- `testing/install-tests/src/steps/phase4_config.rs` - SetRootPassword and CreateUser steps

**Before:**
```rust
let password_cmd = format!("echo 'root:{}' | chpasswd", ctx.default_password());
executor.exec_chroot("/mnt", &password_cmd, ...)?;
```

**After:**
```rust
// Generate hash with openssl
let hash_cmd = format!("openssl passwd -6 '{}'", password);
let hash = executor.exec(&hash_cmd, ...)?;

// Edit shadow directly with sed
let sed_cmd = format!("sed -i 's|^root:[^:]*:|root:{}:|' /mnt/etc/shadow", hash);
executor.exec(&sed_cmd, ...)?;
```

### 2. SSH Host Key Generation (leviso)

**Problem:** `sshd-keygen@.service` doesn't always start correctly, leaving sshd unable to accept connections.

**Solution:** Pre-generate SSH host keys during the rootfs build.

**Files Changed:**
- `leviso/src/component/custom/etc.rs` - `create_ssh_host_keys()` function

**Before:**
```rust
// Just creates /etc/ssh directory, relies on sshd-keygen@.service
fs::create_dir_all(&ssh_dir)?;
println!("  Created /etc/ssh (keys will be generated on first boot)");
```

**After:**
```rust
// Generate all three key types
for (key_type, bits) in [("rsa", 3072), ("ecdsa", 256), ("ed25519", 0)] {
    Command::new("ssh-keygen")
        .arg("-t").arg(key_type)
        .arg("-f").arg(&key_path)
        .arg("-N").arg("")
        .status()?;
}
println!("  SSH host keys ready (sshd can start immediately)");
```

### 3. Cache Invalidation (leviso)

**Problem:** Profile files (`profile/etc/shadow`, etc.) weren't tracked in rebuild hashes. Changes to these files didn't trigger rebuilds.

**Solution:** Add profile files to the hash tracking in `rebuild.rs`.

**Files Changed:**
- `leviso/src/rebuild.rs` - `rootfs_needs_rebuild()` and `cache_rootfs_hash()`

**Added to tracking:**
- `profile/etc/shadow`
- `profile/etc/passwd`
- `profile/etc/group`
- `profile/etc/gshadow`
- `profile/etc/sudoers`
- `profile/etc/motd`

Also added live-overlay files to ISO rebuild detection:
- `profile/live-overlay/etc/shadow`
- `profile/live-overlay/etc/systemd/system/console-autologin.service`

### 4. Automated Verification (fsdbg)

**Problem:** Manual initramfs extraction was needed to verify SSH keys existed.

**Solution:** Add SSH host keys to the rootfs checklist so builds fail if keys are missing.

**Files Changed:**
- `testing/fsdbg/src/checklist/rootfs.rs` - Added to `ETC_FILES`:

```rust
"etc/ssh/ssh_host_rsa_key",
"etc/ssh/ssh_host_rsa_key.pub",
"etc/ssh/ssh_host_ecdsa_key",
"etc/ssh/ssh_host_ecdsa_key.pub",
"etc/ssh/ssh_host_ed25519_key",
"etc/ssh/ssh_host_ed25519_key.pub",
```

## Dependencies

No new dependencies added. Password hashing uses `openssl` which is available on all build systems.

## Bug Fixes (Proactive Review)

After initial implementation, proactive bug hunting found additional issues:

### 5. SSH Key Generation - Public Key Permissions

**Problem:** Public key permissions weren't being set, and partial generation (private key without public) wasn't handled.

**Solution:**
- Set public key permissions to 0o644 (standard for SSH public keys)
- Check both private AND public key exist before skipping
- Remove partial state before regenerating
- Verify both files exist after generation

### 6. Password Escaping - Shell Injection Prevention

**Problem:** Single quotes in passwords would break the shell command. Also, sed escaping was incomplete.

**Solution:**
- Added `shell_escape()` function to handle single quotes (`'` -> `'\''`)
- Added `escape_for_sed()` function to handle special characters (`$`, `&`, `\`)
- Use `printf '%s' 'password' | openssl passwd -6 -stdin` instead of inline password

### 7. Cache Invalidation - Additional PAM and Recipe Files

**Problem:** Many profile files included via `include_str!()` weren't tracked.

**Solution:** Added tracking for:
- PAM files: `system-auth`, `login`, `sshd`, `sudo`, `su`, `passwd`, `chpasswd`
- Security: `limits.conf`
- Recipe: `recipe.conf`, `recipe.sh`
- Live overlay: `serial-console.service`, `live-docs.sh`, `00-levitate-test.sh`

### 8. SSH Key Regeneration for Installed Systems (recstrap)

**Problem:** All installed systems shared the same SSH host keys as the live ISO, enabling MITM attacks.

**Solution:** Add post-extraction step in recstrap to regenerate SSH keys for each installed system.

**Files Changed:**
- `tools/recstrap/src/helpers.rs` - Added `regenerate_ssh_host_keys()` function
- `tools/recstrap/src/main.rs` - Added Phase 7: Security Hardening

**After extraction now includes:**
```rust
// SECURITY: Regenerate SSH host keys to prevent MITM attacks.
if let Err(e) = regenerate_ssh_host_keys(&target, args.quiet) {
    eprintln!("recstrap: warning: SSH key regeneration failed: {}", e);
}
```

### 9. OpenSSL Availability in Live ISO (packages.rhai)

**Problem:** `install-tests` uses `openssl passwd -6` for password hashing, but openssl wasn't explicitly in the package list.

**Solution:** Add openssl to packages.rhai package list with verification.

**Files Changed:**
- `leviso/deps/packages.rhai` - Added `"openssl"` to packages array and `usr/bin/openssl` to verification

## Testing

```bash
cargo check -p leviso -p fsdbg -p install-tests -p recstrap  # All pass
cargo test -p install-tests --lib                            # 4/4 pass
```

## References

- `KNOWLEDGE_install-test-debugging.md` - Original documentation of the issues (R4, W2)
- `TEAM_112_install-test-login-fix.md` - Previous attempt at fixing login issues
- `APOLOGY_TO_TEAM_092.md` - Context on panic git checkout issue (not addressed here)
