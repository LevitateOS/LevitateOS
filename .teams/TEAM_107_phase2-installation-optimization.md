# TEAM_107: Phase 2 - Installation Process Optimization

## Mission
Optimize the installation process to reduce test time by ~150+ seconds.

## Naming Convention (Updated)

Two initramfs files with distinct, self-explanatory names:

| File | Purpose | Size |
|------|---------|------|
| `initramfs-live.img` | Boots the live ISO (mounts squashfs) | ~5MB |
| `initramfs-installed.img` | Boots the daily driver OS (full dracut) | ~30-50MB |

## Completed Work

### Step 2.1: Pre-build initramfs during ISO build - DONE

**Files modified:**
- `distro-spec/src/levitate/paths.rs` - Added `INITRAMFS_INSTALLED_OUTPUT` and `INITRAMFS_INSTALLED_ISO_PATH` constants
- `distro-spec/src/levitate/mod.rs` - Exported new constants
- `leviso/src/artifact/initramfs.rs` - Added `build_install_initramfs()` function
- `leviso/src/artifact/mod.rs` - Exported the new function
- `leviso/src/commands/build.rs` - Added REQUIRED install initramfs build step

**How it works:**
- After squashfs is built, runs dracut in systemd-nspawn/chroot to generate a full initramfs
- The initramfs is generic (no hostonly) so it works on any hardware
- Build FAILS if initramfs generation fails (no fallback - fail fast)

### Step 2.2: Copy initramfs instead of dracut - DONE

**Files modified:**
- `leviso/src/artifact/iso.rs` - REQUIRES initramfs-installed.img on ISO (fails if missing)
- `testing/install-tests/src/steps/phase5_boot.rs` - Copies initramfs from ISO (no dracut fallback)

**Logic:**
```rust
// Simple, no fallback - initramfs MUST exist on ISO
cp /media/cdrom/boot/initramfs-installed.img /mnt/boot/initramfs.img  // ~2 seconds
```

**No fallback by design:** If the ISO is missing initramfs-installed.img, the build failed. Fix the build, don't work around it at install time.

### Step 2.3: Skip redundant config steps - DONE

**Files modified:**
- `testing/install-tests/src/steps/phase4_config.rs` - SetTimezone and ConfigureLocale now check existing values first

**How it works:**
- Before writing timezone symlink, checks `readlink /etc/localtime` for existing value
- Before writing locale.conf, checks `cat /mnt/etc/locale.conf` for existing LANG
- If already correct (e.g., UTC timezone, en_US.UTF-8 locale from squashfs defaults), skips the write
- Still performs write if value differs from expected

**Benefit:** Avoids unnecessary writes when squashfs already has correct defaults.

### Step 2.4: Batch service enablement - DONE

**Files modified:**
- `testing/install-tests/src/steps/phase5_boot.rs` - EnableServices uses single systemctl command

**How it works:**
- First pass: Check which services exist (getent for group, test -f for service files)
- Collect all existing services into a list
- Single `systemctl enable service1 service2 service3...` command
- Individual checks still report which services were enabled

**Before:**
```rust
for service in ENABLED_SERVICES {
    console.exec_chroot("/mnt", &format!("systemctl enable {}", service.name), ...)?;
}
```

**After:**
```rust
let enable_cmd = format!("systemctl enable {}", found_services.join(" "));
console.exec_chroot("/mnt", &enable_cmd, ...)?;
```

**Benefit:** One chroot invocation instead of N, saves ~6 seconds.

## Expected Time Savings

| Step | Before | After | Saved |
|------|--------|-------|-------|
| 2.1+2.2: Initramfs | dracut in chroot: 2-3 min | cp from ISO: 2s | **~150s** |
| 2.3: Config | Write even if correct | Skip if unchanged | ~2s |
| 2.4: Services | N chroot invocations | 1 batch command | ~6s |
| **Total** | | | **~158s** |

## Files Modified Summary

| File | Change |
|------|--------|
| `distro-spec/src/levitate/paths.rs` | Added install initramfs constants |
| `distro-spec/src/levitate/mod.rs` | Exported new constants |
| `leviso/src/artifact/initramfs.rs` | Added `build_install_initramfs()` |
| `leviso/src/artifact/mod.rs` | Exported new function |
| `leviso/src/artifact/iso.rs` | Copy install initramfs to ISO |
| `leviso/src/commands/build.rs` | Call `build_install_initramfs()` |
| `testing/install-tests/src/steps/phase4_config.rs` | Skip timezone/locale if already correct |
| `testing/install-tests/src/steps/phase5_boot.rs` | Pre-built initramfs + batch service enable |

## Testing

```bash
# Build with install initramfs (requires root/nspawn)
cd leviso && sudo cargo run -- build

# Run installation tests
cd testing/install-tests && cargo run
```

## Notes

- The `build_install_initramfs()` function requires root access (systemd-nspawn or chroot)
- Build FAILS without root - no silent fallback, no dracut at install time
- ISO creation FAILS if initramfs-installed.img is missing
- Installation FAILS if initramfs is not on ISO
- Philosophy: Fail fast at build time, not at install time
- This is the biggest single optimization - 150 seconds saved per installation
