# Phase 4: Integration - Disk-Based Root Filesystem

**TEAM_402**: Disk-Based Root Filesystem
**Created**: 2026-01-10
**Status**: Pending Phase 3 Completion

---

## Integration Points

### Syscall Dispatcher Integration

**File**: `crates/kernel/src/syscall/mod.rs`

```rust
// Add pivot_root to syscall dispatcher

match syscall_num {
    // ... existing syscalls ...

    // Filesystem - root switching
    #[cfg(target_arch = "x86_64")]
    SyscallNumber::PivotRoot => sys_pivot_root(a0, a1),  // 155

    #[cfg(target_arch = "aarch64")]
    SyscallNumber::PivotRoot => sys_pivot_root(a0, a1),  // 41

    // ...
}
```

### ABI Crate Updates

**File**: `crates/abi/src/lib.rs`

```rust
// Ensure pivot_root syscall number is defined

#[cfg(target_arch = "x86_64")]
pub const SYS_PIVOT_ROOT: usize = 155;

#[cfg(target_arch = "aarch64")]
pub const SYS_PIVOT_ROOT: usize = 41;
```

### Block Device Integration

**File**: `crates/kernel/src/block.rs`

```rust
// Add partition-aware block access

impl BlockDevice {
    /// Read from a specific partition
    pub fn read_partition(&self, partition: u8, offset: u64, buf: &mut [u8]) -> Result<()> {
        let part_info = self.partitions.get(partition as usize - 1)
            .ok_or(BlockError::InvalidPartition)?;

        let absolute_offset = part_info.start_lba * 512 + offset;
        self.read(absolute_offset, buf)
    }

    /// Write to a specific partition
    pub fn write_partition(&self, partition: u8, offset: u64, buf: &[u8]) -> Result<()> {
        let part_info = self.partitions.get(partition as usize - 1)
            .ok_or(BlockError::InvalidPartition)?;

        let absolute_offset = part_info.start_lba * 512 + offset;
        self.write(absolute_offset, buf)
    }
}
```

### VFS Integration for Partitions

**File**: `crates/kernel/src/fs/mod.rs`

```rust
// Mount by partition device path

pub fn mount_partition(device: &str, mountpoint: &str, fstype: &str) -> Result<(), MountError> {
    // Parse device path: /dev/vda1 -> device=vda, partition=1
    let (device_name, partition_num) = parse_device_path(device)?;

    // Get block device
    let block_dev = get_block_device(device_name)?;

    // Get partition info
    let partition = block_dev.partitions.get(partition_num - 1)
        .ok_or(MountError::InvalidPartition)?;

    // Create filesystem superblock with partition offset
    let superblock = match fstype {
        "ext4" => ext4::mount_partition(block_dev, partition)?,
        "vfat" => fat::mount_partition(block_dev, partition)?,
        _ => return Err(MountError::UnsupportedFs),
    };

    // Add to mount table
    let mut table = MOUNT_TABLE.write();
    table.mount(Mount {
        mountpoint: mountpoint.to_string(),
        fs_type: FsType::from_str(fstype)?,
        source: device.to_string(),
        superblock,
        flags: MountFlags::empty(),
    })?;

    Ok(())
}
```

### Init Process Integration

**File**: `crates/userspace/init/Cargo.toml`

```toml
[package]
name = "init"
version = "0.1.0"
edition = "2021"

[dependencies]
std = { package = "eyra", version = "0.22", features = ["experimental-relocate"] }

# For syscalls
libc = "0.2"
```

---

## Test Strategy

### Unit Tests

```rust
// tests/partition_tests.rs

#[test]
fn test_mbr_parsing_single_partition() {
    let mbr = create_test_mbr_with_partition(
        1,      // partition 1
        0x83,   // Linux type
        2048,   // start at 1MB
        2097152 // 1GB in sectors
    );

    let partitions = parse_mbr(&mbr).unwrap();
    assert_eq!(partitions.len(), 1);
    assert_eq!(partitions[0].number, 1);
    assert_eq!(partitions[0].partition_type, 0x83);
    assert_eq!(partitions[0].start_lba, 2048);
}

#[test]
fn test_mbr_parsing_no_signature() {
    let mut mbr = [0u8; 512];
    // No 0xAA55 signature
    assert!(parse_mbr(&mbr).is_err());
}

#[test]
fn test_pivot_root_success() {
    // Setup: mount tmpfs at /mnt/test
    mount("tmpfs", "/mnt/test", "tmpfs", 0, None).unwrap();

    // Create put_old directory
    mkdir("/mnt/test/old", 0o755).unwrap();

    // Change to new root
    chdir("/mnt/test").unwrap();

    // Pivot
    let result = pivot_root(".", "old");
    assert_eq!(result, Ok(()));

    // Verify: / is now the tmpfs
    // old root accessible at /old
}

#[test]
fn test_pivot_root_not_mount_point() {
    // Try to pivot to non-mount-point
    let result = pivot_root("/tmp/not-a-mount", "old");
    assert_eq!(result, Err(EINVAL));
}
```

### Behavior Tests

#### New Golden Files

```
tests/golden/
├── x86_64/
│   ├── pivot_root_success.txt       # NEW
│   ├── disk_boot_detection.txt      # NEW
│   └── installer_output.txt         # NEW
└── aarch64/
    ├── pivot_root_success.txt
    ├── disk_boot_detection.txt
    └── installer_output.txt
```

#### Behavior Test Scenarios

```yaml
# tests/behavior/pivot_root.yaml
name: pivot_root_basic
description: Test pivot_root syscall
steps:
  - boot kernel with disk attached
  - mount disk at /mnt
  - create /mnt/old directory
  - chdir to /mnt
  - call pivot_root(".", "old")
  - verify / is disk filesystem
  - verify /old contains initramfs

# tests/behavior/boot_detection.yaml
name: boot_mode_detection
description: Test automatic boot mode detection
steps:
  - boot kernel with installed disk
  - expect: "Found installed OS on disk"
  - expect: "Switching root to disk"
  - expect: "Root switched"
  - verify: / is ext4 on /dev/vda1
```

### Integration Tests

```bash
#!/bin/bash
# tests/integration/disk_boot_test.sh

echo "=== Disk Boot Integration Test ==="

# Create test disk with installed OS
create_test_disk() {
    dd if=/dev/zero of=test_disk.img bs=1M count=512
    sfdisk test_disk.img << EOF
label: dos
start=2048, type=83
EOF
    mkfs.ext4 -F -E offset=1048576 test_disk.img

    # Mount and populate
    mkdir -p /mnt/test
    mount -o loop,offset=1048576 test_disk.img /mnt/test
    mkdir -p /mnt/test/{bin,sbin,etc,dev,tmp}
    cp /sbin/init /mnt/test/sbin/
    echo "levitate" > /mnt/test/etc/hostname
    umount /mnt/test
}

# Boot with disk
boot_with_disk() {
    cargo xtask run --disk test_disk.img --timeout 30
}

# Verify output
verify_boot() {
    # Check for expected messages
    grep -q "Found installed OS" boot.log || fail "No detection"
    grep -q "Switching root" boot.log || fail "No switch"
    grep -q "Root switched" boot.log || fail "Switch failed"
}

create_test_disk
boot_with_disk > boot.log 2>&1
verify_boot

echo "=== Test Passed ==="
```

---

## Impact Analysis

### Affected Subsystems

| Subsystem | Impact | Risk |
|-----------|--------|------|
| Mount table | High | Root switching is fundamental |
| Task management | Medium | Root/cwd tracking |
| VFS | Medium | Path resolution with new root |
| Block device | Low | Partition support |
| Init process | High | New boot flow logic |

### Breaking Changes

| Change | Impact | Mitigation |
|--------|--------|------------|
| New syscall | None (additive) | - |
| Init behavior | May change boot flow | Fallback to live mode |
| Disk format | Existing disks incompatible | Re-create disk |

### Compatibility Matrix

| Scenario | Before | After |
|----------|--------|-------|
| No disk | Initramfs root | Initramfs root (unchanged) |
| Empty disk | Initramfs root | Initramfs root (unchanged) |
| Installed disk | N/A (new) | Disk root |
| Disk without init | N/A | Initramfs root |

---

## Verification Checklist

### Before Merge

- [ ] pivot_root syscall implemented for both architectures
- [ ] MBR partition parsing works
- [ ] Partition device nodes created in /dev
- [ ] Init detects installed OS correctly
- [ ] Root switch works (pivot_root + exec)
- [ ] Fallback to live mode works
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Both architectures tested

### After Merge

- [ ] CI passes
- [ ] Golden files updated
- [ ] Documentation updated
- [ ] xtask disk commands work with new size

---

## Rollback Plan

If integration causes issues:

1. **pivot_root issues**: Disable in init, stay in live mode
2. **Partition issues**: Fall back to whole-disk access
3. **Boot issues**: Boot without disk attached

New code is largely additive; existing initramfs boot path unchanged.

---

## References

- Phase 3 Implementation: `docs/planning/disk-root-filesystem/phase-3.md`
- Current mount: `crates/kernel/src/fs/mount.rs`
- Current block: `crates/kernel/src/block.rs`
- Init process: `crates/userspace/init/`
