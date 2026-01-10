# Phase 1: Discovery - Disk-Based Root Filesystem

**TEAM_402**: Disk-Based Root Filesystem
**Created**: 2026-01-10
**Status**: Discovery Complete

---

## Feature Summary

### Problem Statement

LevitateOS currently boots with initramfs as permanent root:
- All files live in RAM (initramfs is a CPIO archive)
- No persistence - changes are lost on reboot
- Can't install new software at runtime
- Limited by available RAM, not disk space
- Not a "real" operating system installation

This prevents LevitateOS from being a true **installable operating system**.

### Who Benefits

| User Type | Benefit |
|-----------|---------|
| End Users | Install once, persist across reboots |
| Developers | Install development tools, keep changes |
| Testers | Persistent test environments |
| LevitateOS Project | Closer to production OS |

### The Goal

**Installable OS** = Boot from ISO, install to disk, boot from disk.

**The Test**:
1. Boot ISO → Install to disk → Reboot
2. Boot from disk → Make changes → Reboot
3. Changes persist

---

## Success Criteria

### Milestone 1: Root Filesystem Switch

```bash
# Kernel can switch from initramfs to disk root
# (Automatic during boot if disk has installed OS)

# After boot, root is on disk:
mount | grep "/ "
# /dev/vda1 on / type ext4 (rw)

# Changes persist:
touch /test-file
reboot
ls /test-file  # Still exists!
```

**Acceptance Tests**:
- [ ] `pivot_root` or `switch_root` syscall works
- [ ] Kernel mounts disk partition as new root
- [ ] Old initramfs is unmounted/freed
- [ ] System runs entirely from disk
- [ ] Changes persist across reboot

### Milestone 2: OS Installer

```bash
# Run installer from live ISO
/sbin/levitate-install /dev/vda

# Installer does:
# 1. Partition disk (GPT or MBR)
# 2. Format partition (ext4)
# 3. Copy FHS structure to disk
# 4. Install bootloader (optional)
# 5. Configure /etc/fstab
```

**Acceptance Tests**:
- [ ] Installer partitions disk
- [ ] Installer formats filesystem
- [ ] Installer copies OS files
- [ ] Installed system boots

---

## Current State Analysis

### What Works Today

| Feature | Status | Evidence |
|---------|--------|----------|
| VirtIO Block device | ✅ Working | Initialized at boot |
| FAT32 read/write | ✅ Working | embedded-sdmmc |
| ext4 read-only | ✅ Working | ext4-view crate |
| Mount syscall | ✅ Working | Can mount filesystems |
| Disk image creation | ✅ Working | xtask disk create |

### What's Missing

| Gap | Impact | Effort |
|-----|--------|--------|
| **pivot_root syscall** | Can't switch root | Medium |
| **ext4 write support** | Can't modify installed OS | High |
| **Partition table parsing** | Can't find partitions | Medium |
| **Bootloader installation** | Can't boot from disk directly | Medium |
| **Installer utility** | No way to install OS | Medium |
| **Larger disk support** | 16MB too small | Low |

### Current Disk Architecture

```
Current:
┌─────────────────────────────────────────────────┐
│ ISO (Limine)                                    │
│ ├── boot/levitate-kernel                        │
│ └── boot/initramfs.cpio  ←── permanent root     │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│ Disk Image (16MB FAT32)                         │
│ └── Userspace binaries (duplicate, unused)      │
└─────────────────────────────────────────────────┘
```

```
Target:
┌─────────────────────────────────────────────────┐
│ ISO (Limine) - Live Boot                        │
│ ├── boot/levitate-kernel                        │
│ └── boot/initramfs.cpio  ←── temporary (init)   │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│ Disk (1GB+ ext4) - Installed OS                 │
│ ├── bin/, sbin/, etc/, usr/, var/               │
│ ├── boot/levitate-kernel (optional)             │
│ └── ... full FHS structure                      │
└─────────────────────────────────────────────────┘

Boot Flow:
1. Limine loads kernel + initramfs
2. Initramfs mounts disk
3. pivot_root switches to disk
4. Initramfs freed, disk is new /
```

---

## Technical Background

### pivot_root vs switch_root

| Aspect | pivot_root | switch_root |
|--------|------------|-------------|
| **Syscall** | Yes (Linux 155/syscall) | No (busybox utility) |
| **Mechanism** | Swaps root and put_old | Deletes initramfs, chroots |
| **Old root** | Moved to put_old dir | Deleted entirely |
| **Complexity** | Medium | Uses pivot_root + cleanup |
| **Use case** | Containers, initramfs | initramfs → real root |

**Recommendation**: Implement `pivot_root` syscall, then userspace `switch_root` utility can use it.

### Linux pivot_root Semantics

```c
int pivot_root(const char *new_root, const char *put_old);

// new_root: Path to new root filesystem (must be a mount point)
// put_old: Path where old root will be moved (under new_root)
// Returns: 0 on success, -1 on error

// Example usage:
mount("/dev/vda1", "/mnt/root", "ext4", 0, NULL);
chdir("/mnt/root");
pivot_root(".", "old_root");
// Now / is the disk, /old_root is the initramfs
umount2("/old_root", MNT_DETACH);
rmdir("/old_root");
```

### Boot Sequence with Root Switch

```
1. Limine loads kernel + initramfs
2. Kernel starts, initramfs mounted at /
3. /init (from initramfs) runs:
   a. Load disk drivers (already done by kernel)
   b. Find root partition (/dev/vda1 or by UUID)
   c. Mount disk at /mnt/root
   d. pivot_root to /mnt/root
   e. exec /sbin/init (from disk)
4. Disk init runs, system fully on disk
5. Unmount/free old initramfs
```

---

## Codebase Reconnaissance

### Files to Modify

| Component | Location | Changes Needed |
|-----------|----------|----------------|
| Syscall: pivot_root | `kernel/src/syscall/fs/` | New syscall |
| VFS: root switching | `kernel/src/fs/vfs/` | Support changing root |
| Mount table | `kernel/src/fs/mount.rs` | Root mount updates |
| Init process | `userspace/init/` | Root switch logic |
| Disk tools | `xtask/src/disk/` | Larger disk, ext4 |

### Existing Code to Leverage

| Component | Status | Reuse |
|-----------|--------|-------|
| Mount syscall | Complete | Basis for pivot_root |
| VFS layer | Complete | Path resolution |
| Block device | Complete | Disk access |
| ext4 (read) | Complete | Need write support |

---

## Constraints

### Technical Constraints

1. **ext4 write support** - ext4-view is read-only; need different approach
2. **Single partition** - Start simple, MBR with one partition
3. **No GRUB** - Continue using Limine or direct kernel boot

### Resource Constraints

1. **Disk size** - Need larger default (512MB-1GB minimum)
2. **Memory for initramfs** - Keep initramfs small for root switch

### Design Constraints

1. **Auto-detect from start** - Best UX, detect /sbin/init on disk
2. **Backwards compatible** - Still support initramfs-only boot

---

## Open Questions

| ID | Question | Impact |
|----|----------|--------|
| Q1 | pivot_root or custom mechanism? | Syscall design |
| Q2 | ext4 write: new crate or FAT32? | Filesystem choice |
| Q3 | Bootloader on disk? | Boot independence |
| Q4 | Partition scheme: MBR or GPT? | Disk layout |
| Q5 | Installer: kernel or userspace? | Implementation |

See `docs/questions/TEAM_402_disk_root_filesystem.md` for resolution.

---

## Dependencies

### Prerequisites (Must Complete First)

| Dependency | Team | Status |
|------------|------|--------|
| FHS directory structure | TEAM_401 | Planning |
| devtmpfs | TEAM_401 | Planning |
| fork/exec | TEAM_400 | Planning |

### Why These Prerequisites?

1. **FHS (TEAM_401)**: Disk root needs proper directory structure to copy
2. **devtmpfs**: /dev must work before and after pivot_root
3. **fork/exec**: Installer needs to run child processes

---

## References

- Linux pivot_root(2) man page
- Linux switch_root(8) man page
- Current block device: `crates/kernel/src/block.rs`
- Current ext4: `crates/kernel/src/fs/ext4.rs`
- Current mount: `crates/kernel/src/fs/mount.rs`
