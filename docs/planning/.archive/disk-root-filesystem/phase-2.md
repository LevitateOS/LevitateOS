# Phase 2: Design - Disk-Based Root Filesystem

**TEAM_402**: Disk-Based Root Filesystem
**Created**: 2026-01-10
**Status**: Design Complete

---

## Proposed Solution

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        BOOT MODES                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Mode A: Live Boot (No Install)      Mode B: Installed Boot      │
│  ┌─────────────────────────────┐    ┌─────────────────────────┐ │
│  │ ISO Boot                    │    │ ISO or Disk Boot        │ │
│  │ ↓                           │    │ ↓                       │ │
│  │ Initramfs = permanent root  │    │ Initramfs = temporary   │ │
│  │ ↓                           │    │ ↓                       │ │
│  │ No disk mount               │    │ Mount /dev/vda1         │ │
│  │ ↓                           │    │ ↓                       │ │
│  │ Run from RAM                │    │ pivot_root to disk      │ │
│  │                             │    │ ↓                       │ │
│  │                             │    │ Run from disk           │ │
│  └─────────────────────────────┘    └─────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Disk Layout

```
Disk (/dev/vda) - 1GB default
┌──────────────────────────────────────────────────────────────┐
│ MBR (512 bytes)                                              │
├──────────────────────────────────────────────────────────────┤
│ Partition 1: Root filesystem (ext4)                          │
│ Offset: 1MB (2048 sectors)                                   │
│ Size: ~1GB                                                   │
│                                                              │
│ Contents (FHS):                                              │
│ ├── bin/ → usr/bin                                           │
│ ├── sbin/                                                    │
│ │   └── init                                                 │
│ ├── etc/                                                     │
│ │   ├── hostname                                             │
│ │   ├── passwd                                               │
│ │   └── fstab                                                │
│ ├── dev/           (mount point for devtmpfs)                │
│ ├── proc/          (mount point for procfs)                  │
│ ├── sys/           (mount point for sysfs)                   │
│ ├── tmp/           (mount point for tmpfs)                   │
│ ├── run/           (mount point for tmpfs)                   │
│ ├── usr/                                                     │
│ │   ├── bin/                                                 │
│ │   │   ├── coreutils                                        │
│ │   │   └── ...                                              │
│ │   └── lib/                                                 │
│ ├── var/                                                     │
│ │   └── log/                                                 │
│ ├── home/                                                    │
│ │   └── user/                                                │
│ └── root/                                                    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Boot Flow Detail

```
┌─────────────────────────────────────────────────────────────────┐
│ 1. BIOS/UEFI loads Limine from ISO                              │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 2. Limine loads kernel + initramfs.cpio                         │
│    - Kernel at 0x40080000 (or wherever)                         │
│    - Initramfs as module in memory                              │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 3. Kernel initializes                                           │
│    - MMU, interrupts, memory                                    │
│    - VirtIO block device (/dev/vda)                             │
│    - Mount initramfs at /                                       │
│    - Spawn /init from initramfs                                 │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 4. /init (initramfs) runs                                       │
│    a. Check if /dev/vda has installed OS                        │
│       - Read partition table                                    │
│       - Look for ext4 signature                                 │
│       - Check for /sbin/init on disk                            │
│                                                                 │
│    b. If installed OS found:                                    │
│       - mount /dev/vda1 /mnt/root                               │
│       - mount --move /dev /mnt/root/dev                         │
│       - cd /mnt/root                                            │
│       - pivot_root . old_root                                   │
│       - exec /sbin/init                                         │
│                                                                 │
│    c. If no installed OS:                                       │
│       - Continue with initramfs as root (live mode)             │
│       - User can run installer                                  │
└─────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────┐
│ 5. /sbin/init (from disk) runs                                  │
│    - umount /old_root (free initramfs memory)                   │
│    - Mount remaining filesystems (/tmp, /run, etc.)             │
│    - Start shell or services                                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## API Design

### pivot_root Syscall

```rust
// crates/kernel/src/syscall/fs/pivot.rs

/// pivot_root - change the root filesystem
///
/// Moves the root filesystem of the calling process to the directory put_old
/// and makes new_root the new root filesystem.
///
/// # Arguments
/// * `new_root` - Path to new root (must be a mount point)
/// * `put_old` - Path under new_root where old root will be moved
///
/// # Returns
/// * 0 on success
/// * -EINVAL if new_root is not a mount point
/// * -EBUSY if new_root or put_old is in use
/// * -ENOTDIR if either path is not a directory
///
/// # Linux Syscall Number
/// * x86_64: 155
/// * aarch64: 41
pub fn sys_pivot_root(new_root: usize, put_old: usize) -> i64 {
    let task = current_task();
    let ttbr0 = task.ttbr0;

    // 1. Read paths from userspace
    let mut new_root_buf = [0u8; PATH_MAX];
    let new_root_path = read_user_cstring(ttbr0, new_root, &mut new_root_buf)?;

    let mut put_old_buf = [0u8; PATH_MAX];
    let put_old_path = read_user_cstring(ttbr0, put_old, &mut put_old_buf)?;

    // 2. Validate new_root is a mount point
    let mount_table = MOUNT_TABLE.write();
    if !mount_table.is_mount_point(new_root_path) {
        return errno::EINVAL;
    }

    // 3. Validate put_old is under new_root
    if !put_old_path.starts_with(new_root_path) {
        return errno::EINVAL;
    }

    // 4. Perform the pivot
    //    - new_root becomes /
    //    - old / moves to put_old
    mount_table.pivot_root(new_root_path, put_old_path)?;

    // 5. Update process root directory
    task.set_root(new_root_path);

    0
}
```

### Mount Table Extensions

```rust
// crates/kernel/src/fs/mount.rs

impl MountTable {
    /// Check if path is a mount point
    pub fn is_mount_point(&self, path: &str) -> bool {
        self.mounts.iter().any(|m| m.mountpoint == path)
    }

    /// Pivot root filesystem
    pub fn pivot_root(&mut self, new_root: &str, put_old: &str) -> Result<(), MountError> {
        // Find the mount for new_root
        let new_mount = self.find_mount(new_root)?;

        // Create entry for old root at put_old
        let old_root_mount = Mount {
            mountpoint: put_old.to_string(),
            fs_type: FsType::Initramfs,  // or whatever current root is
            source: "oldroot".to_string(),
            flags: MountFlags::empty(),
        };

        // Update root mount to point to new filesystem
        self.set_root(new_mount.clone());

        // Add old root at put_old
        self.mounts.push(old_root_mount);

        // Re-sort mounts by path length
        self.mounts.sort_by(|a, b| b.mountpoint.len().cmp(&a.mountpoint.len()));

        Ok(())
    }

    /// Set a new root mount
    fn set_root(&mut self, mount: Mount) {
        // Remove old root mount
        self.mounts.retain(|m| m.mountpoint != "/");

        // Add new root
        let mut root_mount = mount;
        root_mount.mountpoint = "/".to_string();
        self.mounts.push(root_mount);
    }
}
```

### Partition Table Parsing

```rust
// crates/kernel/src/fs/partition.rs

/// MBR Partition Entry
#[repr(C, packed)]
pub struct MbrPartition {
    pub status: u8,           // 0x80 = bootable, 0x00 = not
    pub chs_start: [u8; 3],   // CHS start (legacy)
    pub partition_type: u8,   // 0x83 = Linux, 0x0C = FAT32 LBA
    pub chs_end: [u8; 3],     // CHS end (legacy)
    pub lba_start: u32,       // LBA start sector
    pub sector_count: u32,    // Number of sectors
}

/// Parse MBR partition table
pub fn parse_mbr(block_device: &dyn BlockDevice) -> Result<Vec<Partition>, PartitionError> {
    let mut mbr = [0u8; 512];
    block_device.read_block(0, &mut mbr)?;

    // Check MBR signature
    if mbr[510] != 0x55 || mbr[511] != 0xAA {
        return Err(PartitionError::InvalidMbr);
    }

    let mut partitions = Vec::new();

    // Parse 4 partition entries at offset 446
    for i in 0..4 {
        let offset = 446 + i * 16;
        let entry: MbrPartition = unsafe {
            core::ptr::read_unaligned(mbr[offset..].as_ptr() as *const MbrPartition)
        };

        if entry.partition_type != 0 && entry.sector_count > 0 {
            partitions.push(Partition {
                number: i + 1,
                partition_type: entry.partition_type,
                start_lba: entry.lba_start as u64,
                size_sectors: entry.sector_count as u64,
            });
        }
    }

    Ok(partitions)
}
```

---

## Data Model Changes

### Task Control Block

```rust
// crates/kernel/src/task/mod.rs

pub struct TaskControlBlock {
    // Existing fields...

    /// Root directory for this process (for chroot/pivot_root)
    /// Default: "/" but can be changed
    pub root: IrqSafeLock<String>,

    /// Current working directory (already exists)
    pub cwd: IrqSafeLock<String>,
}

impl TaskControlBlock {
    pub fn set_root(&self, new_root: &str) {
        *self.root.lock() = new_root.to_string();
    }
}
```

### Disk Configuration

```rust
// xtask/src/disk/config.rs

pub struct DiskConfig {
    /// Disk image path
    pub path: PathBuf,

    /// Total size (default: 1GB)
    pub size: u64,

    /// Partition table type
    pub partition_type: PartitionType,

    /// Filesystem type
    pub filesystem: FilesystemType,
}

pub enum PartitionType {
    Mbr,
    Gpt,
}

pub enum FilesystemType {
    Ext4,
    // Fat32 rejected - no symlinks, no permissions
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("levitate_disk.img"),
            size: 1024 * 1024 * 1024,  // 1GB
            partition_type: PartitionType::Mbr,
            filesystem: FilesystemType::Ext4,
        }
    }
}
```

---

## Behavioral Decisions

### Boot Mode Detection

| Condition | Boot Mode | Behavior |
|-----------|-----------|----------|
| No disk attached | Live | Run from initramfs |
| Disk has no partition table | Live | Run from initramfs |
| Disk has no ext4 partition | Live | Run from initramfs |
| Disk ext4 has no /sbin/init | Live | Run from initramfs |
| Disk ext4 has /sbin/init | Install | pivot_root to disk |

### pivot_root Requirements

| Requirement | Behavior |
|-------------|----------|
| new_root must be mount point | Return EINVAL |
| put_old must be under new_root | Return EINVAL |
| put_old must exist | Return ENOENT |
| Caller must be root (uid 0) | Return EPERM |
| Current dir must be under new_root | Move cwd |

### Filesystem Choice

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| ext4 | Standard, journaled | Need write support | **Preferred** |
| FAT32 | Already have write | No permissions, no symlinks | **Rejected** |
| Custom simple FS | Full control | Non-standard | Not recommended |

**Decision**: Use ext4 for disk root. No fallback—FAT32 breaks Unix compatibility.

---

## Implementation Strategy

### Phase 1: Root Switch Infrastructure

1. Implement `pivot_root` syscall
2. Manual root switch from shell:
   ```bash
   mount /dev/vda1 /mnt
   cd /mnt
   pivot_root . old_root
   exec /sbin/init
   ```
3. FAT32 filesystem (existing write support)

### Phase 2: Automatic Detection

1. Init process checks for installed OS
2. Automatic pivot_root if found
3. Fallback to live mode

### Phase 3: Installer

1. `levitate-install` utility
2. Partition disk
3. Format ext4 (or FAT32)
4. Copy FHS structure
5. Write fstab

### Phase 4: Full ext4 Support

1. ext4 write support
2. Journaling (optional)
3. Proper permissions

---

## Open Questions

### Q1: pivot_root or Custom Mechanism?

**Options**:
- **A) Linux-compatible pivot_root** (Recommended)
  - Syscall 155 (x86_64) / 41 (aarch64)
  - Standard semantics
  - Programs expect it

- **B) Custom switch_root**
  - Simpler implementation
  - Non-standard
  - Less flexible

**Recommendation**: A) Linux-compatible pivot_root for compatibility.

### Q2: ext4 Write Support?

**Options**:
- **A) Implement ext4 write**
  - Standard filesystem
  - Complex (journaling, allocation)
  - Long-term correct

- **B) FAT32** (REJECTED)
  - Already have write support
  - No journaling (corruption risk)
  - Quick to implement

- **C) Simple custom filesystem**
  - Full control
  - Non-standard
  - Extra work

**Recommendation**: A) ext4 - this is the cost of being a real Unix OS.

### Q3: Bootloader on Disk?

**Options**:
- **A) ISO boot only**
  - Always boot from ISO
  - Disk is just storage
  - Simpler

- **B) Limine on disk** (Recommended)
  - Install Limine to disk MBR
  - Boot directly from disk
  - "Real" installation

- **C) Custom bootloader**
  - Full control
  - Significant work

**Recommendation**: B) Install Limine to disk for true standalone boot.

### Q4: Partition Scheme?

**Options**:
- **A) MBR** (Recommended)
  - Simple, well-understood
  - 4 primary partitions max
  - 2TB disk limit
  - BIOS compatible

- **B) GPT**
  - Modern standard
  - 128+ partitions
  - Required for >2TB
  - Needs UEFI or hybrid

**Recommendation**: A) MBR for simplicity. GPT can come later.

### Q5: Installer Location?

**Options**:
- **A) Userspace utility** (Recommended)
  - `/sbin/levitate-install`
  - Uses syscalls (mount, write, etc.)
  - Standard approach

- **B) Kernel built-in**
  - Could simplify early boot
  - Unusual design
  - Harder to update

**Recommendation**: A) Userspace installer is standard and flexible.

---

## Dependencies

### Internal Dependencies

| Component | Depends On | Status |
|-----------|------------|--------|
| pivot_root | VFS mount table | ✅ Ready |
| Disk mount | Block device | ✅ Ready |
| Installer | fork/exec | 🔲 TEAM_400 |
| FHS on disk | FHS structure | 🔲 TEAM_401 |

### External Dependencies

| Dependency | Purpose | Status |
|------------|---------|--------|
| ext4 write crate | Full ext4 support | Research needed |
| mkfs.ext4 | Format disk | Host tool |
| Limine installer | Disk bootloader | Host tool |

---

## Test Strategy Preview

### Unit Tests

```rust
#[test]
fn test_pivot_root_changes_root() {
    // Mount tmpfs at /mnt
    // pivot_root /mnt /mnt/old
    // Verify / is now tmpfs
}

#[test]
fn test_pivot_root_invalid_not_mount_point() {
    // pivot_root /nonexistent /old
    // Should return EINVAL
}

#[test]
fn test_mbr_parsing() {
    let mbr = create_test_mbr();
    let partitions = parse_mbr(&mbr);
    assert_eq!(partitions.len(), 1);
}
```

### Integration Tests

```bash
# Test manual root switch
mount /dev/vda1 /mnt
cd /mnt
pivot_root . old_root
echo "Root switched!"
ls /  # Should show disk contents
ls /old_root  # Should show initramfs
```

---

## References

- Phase 1 Discovery: `docs/planning/disk-root-filesystem/phase-1.md`
- Linux pivot_root(2): https://man7.org/linux/man-pages/man2/pivot_root.2.html
- FHS Plan: `docs/planning/filesystem-hierarchy/`
- Current mount: `crates/kernel/src/fs/mount.rs`
