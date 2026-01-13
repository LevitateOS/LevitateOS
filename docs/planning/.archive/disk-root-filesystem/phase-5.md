# Phase 5: Polish - Disk-Based Root Filesystem

**TEAM_402**: Disk-Based Root Filesystem
**Created**: 2026-01-10
**Status**: Pending Phase 4 Completion

---

## Cleanup Tasks

### Code Cleanup

| Task | Location | Priority |
|------|----------|----------|
| Consolidate mount helpers | `fs/mount.rs` | Medium |
| Add SAFETY comments | All new unsafe blocks | High |
| Error message improvements | `userspace/init/`, `userspace/installer/` | Low |
| Remove debug prints | Throughout | Medium |

### Technical Debt

| Item | Description | Action |
|------|-------------|--------|
| FAT32 root filesystem | Not ideal (no permissions) | Document as temporary, plan ext4 write |
| No disk bootloader | Still requires ISO boot | Document as future work |
| Single partition only | No swap, no separate /home | Document limitation |
| No resize support | Can't grow filesystem | Future work |

### Deprecated Code Removal

```rust
// None expected - this is new functionality
// However, may need to update old init code paths
```

---

## Documentation Updates

### Files to Update

| File | Updates Needed |
|------|----------------|
| `CLAUDE.md` | Document disk boot capability |
| `docs/ARCHITECTURE.md` | Add boot flow diagram |
| `README.md` | Installation instructions |
| `docs/specs/LINUX_ABI_GUIDE.md` | pivot_root documentation |

### New Documentation

#### User Guide: Installing LevitateOS

```markdown
# Installing LevitateOS to Disk

## Overview

LevitateOS can run in two modes:
1. **Live Mode**: Boot from ISO, run from RAM (no persistence)
2. **Installed Mode**: Boot from ISO, run from disk (persistent)

## Requirements

- Disk with at least 512MB free space
- Currently attached as VirtIO block device (/dev/vda)

## Installation Steps

### 1. Boot the Live ISO

```bash
cargo xtask run
# Or boot from ISO in VM/hardware
```

### 2. Run the Installer

```bash
# In LevitateOS shell:
/sbin/levitate-install /dev/vda
```

### 3. Confirm Installation

The installer will:
- Create partition table (MBR)
- Format partition as ext4 (or FAT32)
- Copy system files
- Create configuration

### 4. Reboot

```bash
reboot
```

On next boot, the system will:
- Detect installed OS on disk
- Switch root to disk
- Run from persistent storage

## Verifying Installation

```bash
# Check current root filesystem
mount | grep "/ "

# Should show:
# /dev/vda1 on / type ext4 (rw)

# Test persistence
touch /test-file
reboot
ls /test-file  # File still exists!
```

## Troubleshooting

### System boots to live mode despite installation

Check:
1. Disk is attached: `ls /dev/vda*`
2. Partition exists: `cat /proc/partitions` (future)
3. Filesystem valid: `mount /dev/vda1 /mnt && ls /mnt/sbin/init`

### Installation fails

- Ensure disk is not mounted
- Check disk size (minimum 512MB)
- Check for errors in output
```

#### Developer Guide: Boot Modes

```markdown
# LevitateOS Boot Modes

## Boot Flow

```
Limine (ISO/Disk)
       ↓
Load kernel + initramfs
       ↓
Kernel init
       ↓
Mount initramfs as /
       ↓
Spawn /init (from initramfs)
       ↓
┌──────────────────────────────────────┐
│ Init checks for installed OS:        │
│                                      │
│ 1. Does /dev/vda1 exist?            │
│ 2. Can we mount it?                  │
│ 3. Does /sbin/init exist on it?     │
│                                      │
│ YES to all → Switch to disk root    │
│ NO to any  → Stay in live mode      │
└──────────────────────────────────────┘
       ↓                    ↓
   Disk Boot            Live Boot
       ↓                    ↓
pivot_root to disk    Run from initramfs
       ↓                    ↓
exec /sbin/init       Start shell
(from disk)           (installer available)
```

## Adding Boot Mode Detection

To change boot mode detection logic, modify:
- `crates/userspace/init/src/main.rs`

To add new filesystem support:
- `crates/kernel/src/fs/` - Add filesystem driver
- `crates/kernel/src/fs/partition.rs` - Add partition type
```

---

## Handoff Notes

### What Works

After this feature is complete:

1. **Disk boot detection**: Init checks for installed OS
2. **Root switching**: pivot_root moves root to disk
3. **Persistence**: Changes survive reboot
4. **Installation**: Installer utility populates disk
5. **Fallback**: Live mode if no installation found

### What Doesn't Work Yet

1. **Direct disk boot**: Still requires ISO for kernel
2. **ext4 write**: May be using FAT32 workaround
3. **Multiple partitions**: Only single root partition
4. **Disk resize**: No filesystem growth
5. **Recovery mode**: No boot options menu

### Known Limitations

| Limitation | Impact | Future Work |
|------------|--------|-------------|
| ISO boot required | Need ISO even for installed system | Install Limine to disk |
| FAT32 root (if used) | No Unix permissions | ext4 write support |
| Single partition | No swap, no /home | GPT + multiple partitions |
| No UUID support | Disk order dependent | Parse filesystem UUID |
| No encryption | Data not protected | LUKS support |

### Recommended Next Steps

1. **Disk Bootloader**:
   - Install Limine to disk MBR
   - Boot directly from disk without ISO

2. **ext4 Write Support**:
   - Research ext4 write crates
   - Implement journal handling
   - Full read-write support

3. **Multiple Partitions**:
   - GPT partition table support
   - Swap partition
   - Separate /home

4. **Boot Menu**:
   - Choose boot mode at startup
   - Recovery options
   - Kernel selection

---

## Success Criteria Verification

### Milestone 1: Root Filesystem Switch

| Criterion | Test | Status |
|-----------|------|--------|
| pivot_root syscall works | Unit test | PENDING |
| Init detects installed OS | Behavior test | PENDING |
| Root switches to disk | Integration test | PENDING |
| Old root at /old_root | Verification | PENDING |
| System runs from disk | Mount check | PENDING |

### Milestone 2: OS Installer

| Criterion | Test | Status |
|-----------|------|--------|
| Installer partitions disk | Manual test | PENDING |
| Installer formats filesystem | Manual test | PENDING |
| Installer copies files | Manual test | PENDING |
| Installed system boots | Integration test | PENDING |
| Changes persist | Reboot test | PENDING |

### Sign-off Checklist

- [ ] pivot_root works on both architectures
- [ ] Boot detection works correctly
- [ ] Root switch completes successfully
- [ ] Installer creates working system
- [ ] Persistence verified
- [ ] Documentation complete
- [ ] No regressions in live mode

---

## Future Milestones Reference

### Milestone 3: Disk Bootloader

Prerequisites:
- This milestone complete
- Limine installation tools

Requirements:
- Install Limine to disk MBR/ESP
- Configure Limine for disk boot
- Remove ISO dependency

### Milestone 4: ext4 Write Support

Prerequisites:
- This milestone complete

Requirements:
- ext4 write operations
- Journal handling
- fsck integration (optional)

### Milestone 5: Advanced Partitioning

Prerequisites:
- ext4 write complete

Requirements:
- GPT partition table
- Multiple partitions
- Swap partition
- /home separation

---

## References

- Phase 1-4: `docs/planning/disk-root-filesystem/phase-*.md`
- FHS Plan: `docs/planning/filesystem-hierarchy/`
- Linux pivot_root(2): https://man7.org/linux/man-pages/man2/pivot_root.2.html
- Limine: https://github.com/limine-bootloader/limine
