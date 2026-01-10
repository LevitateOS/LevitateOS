# Phase 5: Polish - Filesystem Hierarchy Standard

**TEAM_401**: Filesystem Hierarchy Standard Compliance
**Created**: 2026-01-10
**Status**: Pending Phase 4 Completion

---

## Cleanup Tasks

### Code Cleanup

| Task | Location | Priority |
|------|----------|----------|
| Remove hardcoded PTY paths | `syscall/fs/open.rs` | High |
| Consolidate device handling | `fs/devtmpfs/` | Medium |
| Add SAFETY comments | All new unsafe blocks | High |
| Remove old initramfs structure code | `xtask/` | Low |

### Technical Debt

| Item | Description | Action |
|------|-------------|--------|
| procfs not implemented | No /proc filesystem | Document as future work |
| sysfs not implemented | No /sys filesystem | Document as future work |
| Static devices only | No udev-like hotplug | Document limitation |
| Simple RNG | /dev/urandom uses PRNG | Consider proper entropy pool |

### Deprecated Code Removal

```rust
// Remove from syscall/fs/open.rs:
// - Hardcoded "/dev/ptmx" handling
// - Hardcoded "/dev/pts/{n}" handling

// Remove from build scripts:
// - Flat initramfs structure code
// - Root-level binary installation
```

---

## Documentation Updates

### Files to Update

| File | Updates Needed |
|------|----------------|
| `CLAUDE.md` | Document FHS structure |
| `docs/ARCHITECTURE.md` | Add filesystem hierarchy section |
| `README.md` | Note FHS compliance |
| `docs/specs/LINUX_ABI_GUIDE.md` | Add /dev device documentation |

### New Documentation

#### User Guide: Filesystem Layout

```markdown
# LevitateOS Filesystem Layout

## Overview

LevitateOS follows the Filesystem Hierarchy Standard (FHS) 3.0.

## Directory Structure

| Path | Purpose | Filesystem |
|------|---------|------------|
| `/bin` | Essential user commands | initramfs (symlinks) |
| `/sbin` | System commands | initramfs |
| `/etc` | Configuration files | initramfs |
| `/dev` | Device files | devtmpfs |
| `/home` | User home directories | initramfs |
| `/tmp` | Temporary files | tmpfs |
| `/usr` | Secondary hierarchy | initramfs |
| `/var` | Variable data | mixed |

## Device Files

| Device | Path | Purpose |
|--------|------|---------|
| null | `/dev/null` | Discard output |
| zero | `/dev/zero` | Read zeros |
| full | `/dev/full` | Always full |
| urandom | `/dev/urandom` | Random bytes |
| tty | `/dev/tty` | Current terminal |
| ptmx | `/dev/ptmx` | PTY master |

## Configuration Files

| File | Purpose |
|------|---------|
| `/etc/hostname` | System hostname |
| `/etc/passwd` | User accounts |
| `/etc/group` | Groups |
| `/etc/shells` | Valid shells |
| `/etc/profile` | Shell startup |

## Limitations

- No /proc filesystem (planned)
- No /sys filesystem (planned)
- No dynamic device hotplug
- Single-user (all processes run as root)
```

#### Developer Guide: Adding Devices

```markdown
# Adding Device Files to LevitateOS

## Device Node Creation

Devices are created in devtmpfs during boot:

```rust
// In crates/kernel/src/fs/devtmpfs/mod.rs

impl Devtmpfs {
    pub fn add_device(&self) {
        // Standard device numbers (from Linux)
        // Major 1: Memory devices
        // Major 5: TTY devices
        // Major 136: PTY slaves

        self.mknod("mydevice", major, minor, mode)?;
    }
}
```

## Device Operations

Implement device-specific I/O in `devtmpfs/ops.rs`:

```rust
fn device_read(major: u32, minor: u32, buf: &mut [u8]) -> Result<usize> {
    match (major, minor) {
        (MY_MAJOR, MY_MINOR) => {
            // Your device read logic
        }
        _ => Err(VfsError::NotSupported),
    }
}
```

## Testing

```bash
# Verify device exists
ls -la /dev/mydevice

# Test read/write
cat /dev/mydevice
echo test > /dev/mydevice
```
```

---

## Handoff Notes

### What Works

After this feature is complete:

1. **Standard paths**: `/bin/sh`, `/usr/bin/env` work
2. **Device files**: `/dev/null`, `/dev/zero`, `/dev/urandom` work
3. **Configuration**: `/etc/hostname`, `/etc/passwd` exist
4. **Compatibility**: Scripts with `#!/bin/sh` work
5. **PTY**: `/dev/ptmx` and `/dev/pts/*` work

### What Doesn't Work Yet

1. **procfs**: No `/proc` filesystem
2. **sysfs**: No `/sys` filesystem
3. **Dynamic devices**: No hotplug support
4. **Block devices**: No `/dev/sda` style devices

### Known Limitations

| Limitation | Impact | Future Work |
|------------|--------|-------------|
| No procfs | Can't read /proc/self | Implement procfs |
| No sysfs | Can't query hardware | Implement sysfs |
| Simple RNG | /dev/urandom not cryptographic | Add entropy pool |
| No udev | Static device list | Add device manager |

### Recommended Next Steps

1. **procfs Implementation**:
   - `/proc/self/` -> current process
   - `/proc/{pid}/` -> process info
   - `/proc/meminfo` -> memory stats

2. **sysfs Implementation**:
   - `/sys/kernel/` -> kernel parameters
   - `/sys/devices/` -> device tree

3. **Enhanced Devices**:
   - `/dev/fd/` -> file descriptor links
   - `/dev/stdin`, `/dev/stdout`, `/dev/stderr`

---

## Success Criteria Verification

### Milestone 1: Basic FHS Structure

| Criterion | Test | Status |
|-----------|------|--------|
| /bin/ exists with essentials | `ls /bin/sh` | PENDING |
| /usr/bin/ contains utilities | `ls /usr/bin/env` | PENDING |
| /etc/ contains config | `cat /etc/hostname` | PENDING |
| /dev/null works | `echo x > /dev/null` | PENDING |
| /dev/zero works | `head -c 1 /dev/zero` | PENDING |
| /tmp is writable | `touch /tmp/test` | PENDING |

### Sign-off Checklist

- [ ] All FHS directories exist
- [ ] Standard device files work
- [ ] /etc configuration files present
- [ ] No regressions in existing functionality
- [ ] Both architectures tested
- [ ] Documentation complete
- [ ] Code reviewed

---

## Future Milestones Reference

### Milestone 2: procfs

Prerequisites:
- Basic FHS (this milestone)
- Process table introspection

Requirements:
- `/proc/self/` symlink
- `/proc/{pid}/cmdline`
- `/proc/{pid}/status`
- `/proc/meminfo`
- `/proc/cpuinfo`

### Milestone 3: sysfs

Prerequisites:
- procfs complete
- Device model

Requirements:
- `/sys/kernel/`
- `/sys/devices/`
- `/sys/class/`
- Attribute files

### Milestone 4: Full Device Support

Prerequisites:
- sysfs complete
- Block device layer

Requirements:
- `/dev/sda`, `/dev/nvme0n1`
- `/dev/fd/` file descriptor symlinks
- Device permissions

---

## References

- Phase 1-4: `docs/planning/filesystem-hierarchy/phase-*.md`
- [FHS 3.0 Spec](https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html)
- Current VFS: `crates/kernel/src/fs/vfs/`
- Current devtmpfs: `crates/kernel/src/fs/devtmpfs/`
