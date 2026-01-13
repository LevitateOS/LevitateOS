# Phase 3: Implementation - Disk-Based Root Filesystem

**TEAM_402**: Disk-Based Root Filesystem
**Created**: 2026-01-10
**Status**: Ready for Implementation

---

## Implementation Overview

### Dependency Graph

```
                    ┌─────────────────────┐
                    │ pivot_root syscall  │
                    │ (Medium Effort)     │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
    ┌─────────────────┐ ┌─────────────┐ ┌─────────────┐
    │ Mount table     │ │ Task root   │ │ VFS path    │
    │ pivot support   │ │ tracking    │ │ resolution  │
    └─────────────────┘ └─────────────┘ └─────────────┘

                    ┌─────────────────────┐
                    │ Partition parsing   │
                    │ (Low Effort)        │
                    └──────────┬──────────┘
                               │
              ┌────────────────┴────────────────┐
              ▼                                 ▼
    ┌─────────────────┐               ┌─────────────────┐
    │ MBR parsing     │               │ /dev/vdaN       │
    │                 │               │ device nodes    │
    └─────────────────┘               └─────────────────┘

                    ┌─────────────────────┐
                    │ Init root switch    │
                    │ (Medium Effort)     │
                    └──────────┬──────────┘
                               │
    ┌──────────────────────────┼──────────────────────────┐
    ▼                          ▼                          ▼
┌─────────────┐      ┌─────────────────┐      ┌─────────────────┐
│ Detect      │      │ Mount disk      │      │ pivot_root      │
│ installed   │      │ partition       │      │ and exec        │
│ OS          │      │                 │      │                 │
└─────────────┘      └─────────────────┘      └─────────────────┘

                    ┌─────────────────────┐
                    │ Installer utility   │
                    │ (Medium Effort)     │
                    └──────────┬──────────┘
                               │
    ┌──────────────────────────┼──────────────────────────┐
    ▼                          ▼                          ▼
┌─────────────┐      ┌─────────────────┐      ┌─────────────────┐
│ Partition   │      │ Format FS       │      │ Copy files      │
│ disk        │      │                 │      │                 │
└─────────────┘      └─────────────────┘      └─────────────────┘
```

### Implementation Order

| Order | Component | Files | Effort | Depends On |
|-------|-----------|-------|--------|------------|
| 1 | MBR partition parsing | `kernel/src/fs/partition.rs` | Low | Block device |
| 2 | Partition device nodes | `kernel/src/fs/devtmpfs/` | Low | TEAM_401 devtmpfs |
| 3 | pivot_root syscall | `kernel/src/syscall/fs/pivot.rs` | Medium | Mount table |
| 4 | Task root tracking | `kernel/src/task/mod.rs` | Low | pivot_root |
| 5 | Init root detection | `userspace/init/` | Medium | TEAM_400 fork/exec |
| 6 | Init pivot logic | `userspace/init/` | Medium | pivot_root |
| 7 | Disk image resize | `xtask/src/disk/` | Low | Nothing |
| 8 | Installer utility | `userspace/installer/` | Medium | All above |

---

## Implementation Details

### Unit of Work 1: MBR Partition Parsing

**Files**: `crates/kernel/src/fs/partition.rs` (new)

```rust
//! MBR partition table parsing

use alloc::vec::Vec;

/// Partition table entry
#[derive(Debug, Clone)]
pub struct Partition {
    /// Partition number (1-4 for MBR)
    pub number: u8,
    /// Partition type (0x83 = Linux, 0x0C = FAT32)
    pub partition_type: u8,
    /// Starting LBA sector
    pub start_lba: u64,
    /// Size in sectors
    pub size_sectors: u64,
    /// Size in bytes
    pub size_bytes: u64,
}

/// MBR partition entry (16 bytes)
#[repr(C, packed)]
struct MbrEntry {
    status: u8,
    chs_start: [u8; 3],
    partition_type: u8,
    chs_end: [u8; 3],
    lba_start: u32,
    sector_count: u32,
}

const MBR_SIGNATURE: u16 = 0xAA55;
const SECTOR_SIZE: u64 = 512;

/// Parse MBR partition table from first sector
pub fn parse_mbr(sector: &[u8; 512]) -> Result<Vec<Partition>, PartitionError> {
    // Verify MBR signature at bytes 510-511
    let signature = u16::from_le_bytes([sector[510], sector[511]]);
    if signature != MBR_SIGNATURE {
        return Err(PartitionError::InvalidSignature);
    }

    let mut partitions = Vec::new();

    // Parse 4 partition entries starting at offset 446
    for i in 0..4 {
        let offset = 446 + i * 16;
        let entry_bytes = &sector[offset..offset + 16];

        let entry = MbrEntry {
            status: entry_bytes[0],
            chs_start: [entry_bytes[1], entry_bytes[2], entry_bytes[3]],
            partition_type: entry_bytes[4],
            chs_end: [entry_bytes[5], entry_bytes[6], entry_bytes[7]],
            lba_start: u32::from_le_bytes([
                entry_bytes[8], entry_bytes[9], entry_bytes[10], entry_bytes[11]
            ]),
            sector_count: u32::from_le_bytes([
                entry_bytes[12], entry_bytes[13], entry_bytes[14], entry_bytes[15]
            ]),
        };

        // Skip empty entries
        if entry.partition_type == 0 || entry.sector_count == 0 {
            continue;
        }

        partitions.push(Partition {
            number: (i + 1) as u8,
            partition_type: entry.partition_type,
            start_lba: entry.lba_start as u64,
            size_sectors: entry.sector_count as u64,
            size_bytes: entry.sector_count as u64 * SECTOR_SIZE,
        });
    }

    Ok(partitions)
}

#[derive(Debug)]
pub enum PartitionError {
    InvalidSignature,
    ReadError,
}
```

**Estimated effort**: 2-3 hours

---

### Unit of Work 2: Partition Device Nodes

**Files**: `crates/kernel/src/fs/devtmpfs/` (extend)

```rust
// In devtmpfs initialization, after detecting block devices:

impl Devtmpfs {
    /// Create partition device nodes for a block device
    pub fn create_partition_nodes(&self, device_name: &str, partitions: &[Partition]) {
        // Create base device node (e.g., /dev/vda)
        // Major 253 = virtio-blk
        self.mknod(device_name, 253, 0, 0o660).ok();

        // Create partition nodes (e.g., /dev/vda1, /dev/vda2)
        for part in partitions {
            let part_name = format!("{}{}", device_name, part.number);
            // Minor number encodes partition
            self.mknod(&part_name, 253, part.number as u32, 0o660).ok();
        }
    }
}

// In block device initialization:
pub fn init_block_devices() {
    if let Some(disk) = virtio_blk::get_device() {
        // Read MBR
        let mut mbr = [0u8; 512];
        disk.read_sector(0, &mut mbr).ok();

        // Parse partitions
        if let Ok(partitions) = partition::parse_mbr(&mbr) {
            // Create device nodes
            let devtmpfs = get_devtmpfs();
            devtmpfs.create_partition_nodes("vda", &partitions);

            // Store partition info for later mounting
            PARTITIONS.lock().extend(partitions);
        }
    }
}
```

**Estimated effort**: 2-3 hours

---

### Unit of Work 3: pivot_root Syscall

**Files**: `crates/kernel/src/syscall/fs/pivot.rs` (new)

```rust
//! pivot_root syscall implementation

use crate::fs::mount::MOUNT_TABLE;
use crate::syscall::errno;
use crate::task::current_task;

/// pivot_root - change the root filesystem
///
/// Linux syscall numbers:
/// - x86_64: 155
/// - aarch64: 41
pub fn sys_pivot_root(new_root_ptr: usize, put_old_ptr: usize) -> i64 {
    let task = current_task();
    let ttbr0 = task.ttbr0;

    // Read paths from userspace
    let mut new_root_buf = [0u8; 4096];
    let new_root = match read_user_cstring(ttbr0, new_root_ptr, &mut new_root_buf) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    let mut put_old_buf = [0u8; 4096];
    let put_old = match read_user_cstring(ttbr0, put_old_ptr, &mut put_old_buf) {
        Ok(s) => s,
        Err(_) => return errno::EFAULT,
    };

    // Normalize paths
    let new_root = normalize_path(new_root);
    let put_old = normalize_path(put_old);

    // Validate: new_root must be a mount point
    let mut mount_table = MOUNT_TABLE.write();
    if !mount_table.is_mount_point(&new_root) {
        return errno::EINVAL;
    }

    // Validate: put_old must be under new_root (or equal with ".")
    let put_old_absolute = if put_old.starts_with('/') {
        put_old.clone()
    } else {
        // Relative to new_root
        format!("{}/{}", new_root, put_old)
    };

    if !put_old_absolute.starts_with(&new_root) {
        return errno::EINVAL;
    }

    // Perform the pivot
    match mount_table.pivot_root(&new_root, &put_old_absolute) {
        Ok(()) => {
            // Update task's root and cwd if needed
            let mut task_root = task.root.lock();
            *task_root = "/".to_string();

            // If cwd was under old root, update it
            let mut task_cwd = task.cwd.lock();
            if task_cwd.starts_with(&new_root) {
                // Rebase cwd to new root
                let relative = task_cwd.strip_prefix(&new_root).unwrap_or("");
                *task_cwd = format!("/{}", relative.trim_start_matches('/'));
            }

            0
        }
        Err(_) => errno::EINVAL,
    }
}

// Register in syscall dispatcher
// x86_64: 155
// aarch64: 41
```

**Estimated effort**: 4-6 hours

---

### Unit of Work 4: Mount Table Pivot Support

**Files**: `crates/kernel/src/fs/mount.rs`

```rust
impl MountTable {
    /// Check if a path is a mount point
    pub fn is_mount_point(&self, path: &str) -> bool {
        let normalized = normalize_path(path);
        self.mounts.iter().any(|m| m.mountpoint == normalized)
    }

    /// Pivot root filesystem
    ///
    /// Makes new_root the new / and moves old root to put_old
    pub fn pivot_root(&mut self, new_root: &str, put_old: &str) -> Result<(), MountError> {
        // Find the mount entry for new_root
        let new_mount_idx = self.mounts
            .iter()
            .position(|m| m.mountpoint == new_root)
            .ok_or(MountError::NotMounted)?;

        // Get current root mount
        let old_root_idx = self.mounts
            .iter()
            .position(|m| m.mountpoint == "/")
            .ok_or(MountError::NotMounted)?;

        // Clone the entries we need
        let mut new_mount = self.mounts[new_mount_idx].clone();
        let mut old_mount = self.mounts[old_root_idx].clone();

        // Update mountpoints
        new_mount.mountpoint = "/".to_string();
        old_mount.mountpoint = put_old.to_string();

        // Remove old entries
        self.mounts.retain(|m| m.mountpoint != new_root && m.mountpoint != "/");

        // Add updated entries
        self.mounts.push(new_mount);
        self.mounts.push(old_mount);

        // Re-sort by mountpoint length (longest first for lookup)
        self.mounts.sort_by(|a, b| b.mountpoint.len().cmp(&a.mountpoint.len()));

        // Update any child mounts that were under new_root
        // (e.g., /mnt/root/dev -> /dev)
        for mount in &mut self.mounts {
            if mount.mountpoint.starts_with(new_root) && mount.mountpoint != "/" {
                let suffix = mount.mountpoint.strip_prefix(new_root).unwrap_or("");
                mount.mountpoint = format!("/{}", suffix.trim_start_matches('/'));
            }
        }

        Ok(())
    }
}
```

**Estimated effort**: 3-4 hours

---

### Unit of Work 5: Init Root Detection

**Files**: `crates/userspace/init/src/main.rs`

```rust
//! Init process with root filesystem detection

use std::fs;
use std::path::Path;

const DISK_DEVICE: &str = "/dev/vda1";
const MOUNT_POINT: &str = "/mnt/root";
const DISK_INIT: &str = "/mnt/root/sbin/init";

fn main() {
    // Check if we should switch to disk root
    if should_switch_root() {
        switch_to_disk_root();
    } else {
        // Live mode: continue with initramfs
        run_live_mode();
    }
}

/// Check if disk has an installed OS
fn should_switch_root() -> bool {
    // 1. Check if disk device exists
    if !Path::new(DISK_DEVICE).exists() {
        println!("[init] No disk device found, staying in live mode");
        return false;
    }

    // 2. Try to mount disk
    if mount_disk().is_err() {
        println!("[init] Failed to mount disk, staying in live mode");
        return false;
    }

    // 3. Check if disk has /sbin/init
    let has_init = Path::new(DISK_INIT).exists();

    if !has_init {
        // Unmount and stay in live mode
        umount(MOUNT_POINT).ok();
        println!("[init] Disk has no /sbin/init, staying in live mode");
        return false;
    }

    println!("[init] Found installed OS on disk");
    true
}

fn mount_disk() -> Result<(), i32> {
    // Create mount point
    fs::create_dir_all(MOUNT_POINT).ok();

    // Mount disk partition
    // syscall: mount("/dev/vda1", "/mnt/root", "ext4", 0, NULL)
    unsafe {
        let ret = syscall!(
            SYS_mount,
            DISK_DEVICE.as_ptr(),
            MOUNT_POINT.as_ptr(),
            "ext4\0".as_ptr(),  // or "vfat" for FAT32
            0,
            core::ptr::null::<u8>()
        );
        if ret < 0 { return Err(ret as i32); }
    }
    Ok(())
}
```

**Estimated effort**: 4-5 hours

---

### Unit of Work 6: Init Pivot Logic

**Files**: `crates/userspace/init/src/main.rs` (continued)

```rust
fn switch_to_disk_root() {
    println!("[init] Switching root to disk...");

    // 1. Move /dev mount to new root
    // mount --move /dev /mnt/root/dev
    move_mount("/dev", "/mnt/root/dev").expect("Failed to move /dev");

    // 2. Change to new root directory
    std::env::set_current_dir(MOUNT_POINT).expect("Failed to chdir");

    // 3. Pivot root
    // pivot_root(".", "old_root")
    let old_root = format!("{}/old_root", MOUNT_POINT);
    fs::create_dir_all(&old_root).ok();

    unsafe {
        let ret = syscall!(
            SYS_pivot_root,
            ".\0".as_ptr(),
            "old_root\0".as_ptr()
        );
        if ret < 0 {
            panic!("pivot_root failed: {}", ret);
        }
    }

    println!("[init] Root switched, executing disk init...");

    // 4. Execute the real init from disk
    // This replaces the current process
    let args = ["init\0"];
    let envp = ["PATH=/bin:/sbin:/usr/bin\0"];

    unsafe {
        syscall!(
            SYS_execve,
            "/sbin/init\0".as_ptr(),
            args.as_ptr(),
            envp.as_ptr()
        );
    }

    // If we get here, execve failed
    panic!("Failed to exec /sbin/init from disk");
}

fn move_mount(from: &str, to: &str) -> Result<(), i32> {
    fs::create_dir_all(to).ok();

    unsafe {
        // MS_MOVE = 0x2000
        let ret = syscall!(
            SYS_mount,
            from.as_ptr(),
            to.as_ptr(),
            core::ptr::null::<u8>(),
            0x2000,  // MS_MOVE
            core::ptr::null::<u8>()
        );
        if ret < 0 { return Err(ret as i32); }
    }
    Ok(())
}

fn run_live_mode() {
    println!("[init] Running in live mode (initramfs root)");
    println!("[init] To install, run: /sbin/levitate-install /dev/vda");

    // Start shell
    // ... existing shell spawn code
}
```

**Estimated effort**: 4-5 hours

---

### Unit of Work 7: Larger Disk Image

**Files**: `xtask/src/disk/image.rs`

```rust
// Update disk configuration

pub const DEFAULT_DISK_SIZE: u64 = 1024 * 1024 * 1024;  // 1GB (was 16MB)
pub const DEFAULT_DISK_PATH: &str = "levitate_disk.img";

pub fn create_disk_image(size: Option<u64>) -> Result<()> {
    let size = size.unwrap_or(DEFAULT_DISK_SIZE);
    let path = DEFAULT_DISK_PATH;

    // Create sparse file
    let file = File::create(path)?;
    file.set_len(size)?;

    // Create MBR partition table
    create_mbr_partition(path, size)?;

    // Format partition as ext4
    format_partition(path)?;

    Ok(())
}

fn create_mbr_partition(path: &str, total_size: u64) -> Result<()> {
    // Use fdisk or sfdisk
    let script = format!(
        "label: dos\n\
         start=2048, type=83\n"  // type 83 = Linux
    );

    Command::new("sfdisk")
        .arg(path)
        .stdin(Stdio::piped())
        .spawn()?
        .stdin.unwrap()
        .write_all(script.as_bytes())?;

    Ok(())
}

fn format_partition(path: &str) -> Result<()> {
    // Format as ext4
    // Offset 1MB (2048 * 512 = 1048576)
    Command::new("mkfs.ext4")
        .args(["-F", "-E", "offset=1048576", path])
        .status()?;

    Ok(())
}
```

**Estimated effort**: 2-3 hours

---

### Unit of Work 8: Installer Utility

**Files**: `crates/userspace/installer/` (new crate)

```rust
//! LevitateOS installer utility

use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: levitate-install <device>");
        eprintln!("Example: levitate-install /dev/vda");
        std::process::exit(1);
    }

    let device = &args[1];

    println!("LevitateOS Installer");
    println!("====================");
    println!("Target device: {}", device);
    println!();
    println!("WARNING: This will ERASE all data on {}!", device);
    println!("Press Enter to continue or Ctrl+C to abort...");

    // Wait for confirmation
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();

    // Run installation
    if let Err(e) = install(device) {
        eprintln!("Installation failed: {:?}", e);
        std::process::exit(1);
    }

    println!();
    println!("Installation complete!");
    println!("You can now reboot to start from disk.");
}

fn install(device: &str) -> Result<(), InstallError> {
    // 1. Create partition table
    println!("[1/5] Creating partition table...");
    create_partitions(device)?;

    // 2. Format filesystem
    println!("[2/5] Formatting filesystem...");
    let partition = format!("{}1", device);  // /dev/vda1
    format_filesystem(&partition)?;

    // 3. Mount partition
    println!("[3/5] Mounting filesystem...");
    fs::create_dir_all("/mnt/install")?;
    mount(&partition, "/mnt/install")?;

    // 4. Copy files
    println!("[4/5] Copying system files...");
    copy_system_files("/mnt/install")?;

    // 5. Unmount
    println!("[5/5] Finalizing...");
    umount("/mnt/install")?;

    Ok(())
}

fn copy_system_files(target: &str) -> Result<(), InstallError> {
    // Create FHS directories
    let dirs = [
        "bin", "sbin", "etc", "dev", "home", "home/user",
        "lib", "mnt", "opt", "proc", "root", "run", "sys",
        "tmp", "usr", "usr/bin", "usr/lib", "usr/share",
        "var", "var/log", "var/tmp",
    ];

    for dir in dirs {
        fs::create_dir_all(format!("{}/{}", target, dir))?;
    }

    // Copy binaries from initramfs
    copy_dir("/usr/bin", &format!("{}/usr/bin", target))?;
    copy_dir("/sbin", &format!("{}/sbin", target))?;

    // Create symlinks in /bin
    let bin_links = ["sh", "cat", "ls", "cp", "mv", "rm", "mkdir", "echo"];
    for link in bin_links {
        let target_path = format!("{}/bin/{}", target, link);
        let source = format!("../usr/bin/{}", link);
        std::os::unix::fs::symlink(&source, &target_path).ok();
    }

    // Create /etc files
    fs::write(format!("{}/etc/hostname", target), "levitate\n")?;
    fs::write(format!("{}/etc/passwd", target),
        "root:x:0:0:root:/root:/bin/sh\n")?;
    fs::write(format!("{}/etc/group", target),
        "root:x:0:\n")?;
    fs::write(format!("{}/etc/fstab", target),
        "# /etc/fstab - filesystem table\n\
         /dev/vda1  /     ext4  defaults  0 1\n\
         devtmpfs   /dev  devtmpfs  rw    0 0\n\
         tmpfs      /tmp  tmpfs     rw    0 0\n\
         tmpfs      /run  tmpfs     rw    0 0\n")?;

    Ok(())
}
```

**Estimated effort**: 6-8 hours

---

## File Change Summary

| File | Changes | Priority |
|------|---------|----------|
| `kernel/src/fs/partition.rs` | New: MBR parsing | P0 |
| `kernel/src/fs/devtmpfs/` | Partition device nodes | P1 |
| `kernel/src/syscall/fs/pivot.rs` | New: pivot_root syscall | P0 |
| `kernel/src/syscall/mod.rs` | Register pivot_root | P0 |
| `kernel/src/fs/mount.rs` | Pivot support | P0 |
| `kernel/src/task/mod.rs` | Task root tracking | P1 |
| `userspace/init/src/main.rs` | Root detection & switch | P1 |
| `userspace/installer/` | New: installer utility | P2 |
| `xtask/src/disk/` | Larger disk, ext4 format | P1 |

---

## Resolved Blockers

All questions answered per kernel development rules.

| Question | Decision | Status |
|----------|----------|--------|
| Q1: pivot_root design | **Linux-compatible** | ✅ Resolved |
| Q2: ext4 vs FAT32 | **ext4** | ✅ Resolved |
| Q3: Partition table | **MBR only** | ✅ Resolved |
| Q4: Disk bootloader | **ISO only** | ✅ Resolved |
| Q5: Boot mode | **Automatic** | ✅ Resolved |
| Q6: Installer scope | **Standard** | ✅ Resolved |

### External Dependencies (Still Pending)

| Dependency | Team | Status |
|------------|------|--------|
| fork/exec | TEAM_400 | Required for installer |
| FHS/devtmpfs | TEAM_401 | Required for disk structure |

See `docs/questions/TEAM_402_disk_root_filesystem.md` for full rationale.

---

## References

- Phase 2 Design: `docs/planning/disk-root-filesystem/phase-2.md`
- Linux pivot_root(2): https://man7.org/linux/man-pages/man2/pivot_root.2.html
- Current mount: `crates/kernel/src/fs/mount.rs`
- Current block: `crates/kernel/src/block.rs`
