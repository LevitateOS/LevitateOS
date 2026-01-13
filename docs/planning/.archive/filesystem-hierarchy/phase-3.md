# Phase 3: Implementation - Filesystem Hierarchy Standard

**TEAM_401**: Filesystem Hierarchy Standard Compliance
**Created**: 2026-01-10
**Status**: Ready for Implementation

---

## Implementation Overview

### Dependency Graph

```
                    ┌─────────────────────┐
                    │ Initramfs Structure │
                    │ (Low Effort)        │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
    ┌─────────────────┐ ┌─────────────┐ ┌─────────────┐
    │ /bin symlinks   │ │ /etc files  │ │ /usr/bin/   │
    │ /sbin binaries  │ │ config      │ │ binaries    │
    └─────────────────┘ └─────────────┘ └─────────────┘

                    ┌─────────────────────┐
                    │ devtmpfs            │
                    │ (Medium Effort)     │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
    ┌─────────────────┐ ┌─────────────┐ ┌─────────────┐
    │ /dev/null,zero  │ │ /dev/ptmx   │ │ /dev/pts/   │
    │ /dev/urandom    │ │ migration   │ │ migration   │
    └─────────────────┘ └─────────────┘ └─────────────┘

                    ┌─────────────────────┐
                    │ Additional Mounts   │
                    │ (Low Effort)        │
                    └──────────┬──────────┘
                               │
              ┌────────────────┴────────────────┐
              ▼                                 ▼
    ┌─────────────────┐               ┌─────────────────┐
    │ /run (tmpfs)    │               │ /var/log        │
    │                 │               │ (tmpfs)         │
    └─────────────────┘               └─────────────────┘
```

### Implementation Order

| Order | Component | Files | Effort | Depends On |
|-------|-----------|-------|--------|------------|
| 1 | Initramfs FHS structure | `xtask/src/build/` | Low | Nothing |
| 2 | /etc config files | `xtask/src/build/` | Low | Order 1 |
| 3 | devtmpfs filesystem | `kernel/src/fs/devtmpfs/` | Medium | VFS |
| 4 | Device operations | `kernel/src/fs/devtmpfs/ops.rs` | Medium | Order 3 |
| 5 | Boot mount sequence | `kernel/src/fs/mount.rs` | Low | Order 3, 4 |
| 6 | PTY migration | `kernel/src/syscall/fs/` | Medium | Order 3 |
| 7 | Additional mounts | `kernel/src/fs/mount.rs` | Low | Order 5 |

---

## Implementation Details

### Unit of Work 1: Initramfs FHS Structure

**Files**: `xtask/src/build/commands.rs`

```rust
// Add to build_initramfs() or create new function

fn create_fhs_directories(root: &Path) -> Result<()> {
    let directories = [
        // Root-level FHS directories
        "bin", "sbin", "etc", "dev", "home", "lib",
        "mnt", "opt", "proc", "root", "run", "sys",
        "tmp", "usr", "var",
        // Subdirectories
        "home/user",
        "usr/bin", "usr/lib", "usr/share", "usr/share/misc",
        "var/log", "var/tmp",
    ];

    for dir in directories {
        let path = root.join(dir);
        fs::create_dir_all(&path)?;
        log::debug!("Created directory: {}", path.display());
    }

    Ok(())
}

fn install_binaries_to_fhs(root: &Path, arch: &str) -> Result<()> {
    let usr_bin = root.join("usr/bin");
    let sbin = root.join("sbin");

    // Install coreutils multi-call binary to /usr/bin
    let coreutils_src = format!("crates/userspace/eyra/coreutils/target/{}-unknown-linux-gnu/release/coreutils", arch);
    if Path::new(&coreutils_src).exists() {
        fs::copy(&coreutils_src, usr_bin.join("coreutils"))?;
    }

    // Install shell to /usr/bin
    let shell_src = format!("crates/userspace/target/{}-unknown-linux-gnu/release/shell", arch);
    if Path::new(&shell_src).exists() {
        fs::copy(&shell_src, usr_bin.join("shell"))?;
        // Create sh symlink
        std::os::unix::fs::symlink("shell", usr_bin.join("sh"))?;
    }

    // Install init to /sbin
    let init_src = format!("crates/userspace/target/{}-unknown-linux-gnu/release/init", arch);
    if Path::new(&init_src).exists() {
        fs::copy(&init_src, sbin.join("init"))?;
    }

    Ok(())
}

fn create_bin_symlinks(root: &Path) -> Result<()> {
    let bin = root.join("bin");

    // Standard utilities that should be in /bin
    let bin_utilities = [
        "cat", "cp", "dd", "echo", "false", "kill", "ln", "ls",
        "mkdir", "mv", "pwd", "rm", "rmdir", "sh", "sleep",
        "test", "touch", "true", "uname",
    ];

    for util in bin_utilities {
        let target = format!("../usr/bin/{}", util);
        let link = bin.join(util);
        if !link.exists() {
            std::os::unix::fs::symlink(&target, &link)?;
        }
    }

    Ok(())
}
```

**Estimated effort**: 2-4 hours

---

### Unit of Work 2: /etc Config Files

**Files**: `xtask/src/build/commands.rs`

```rust
fn create_etc_files(root: &Path) -> Result<()> {
    let etc = root.join("etc");

    // hostname
    fs::write(etc.join("hostname"), "levitate\n")?;

    // passwd (minimal stub)
    fs::write(etc.join("passwd"), "\
root:x:0:0:root:/root:/bin/sh
user:x:1000:1000:user:/home/user:/bin/sh
nobody:x:65534:65534:nobody:/nonexistent:/bin/false
")?;

    // group (minimal stub)
    fs::write(etc.join("group"), "\
root:x:0:
user:x:1000:
nogroup:x:65534:
")?;

    // shells
    fs::write(etc.join("shells"), "\
/bin/sh
/usr/bin/sh
")?;

    // profile (shell initialization)
    fs::write(etc.join("profile"), "\
# LevitateOS shell profile
export PATH=/usr/bin:/bin:/sbin
export HOME=/root
export USER=root
export PS1='\\u@\\h:\\w\\$ '
")?;

    // fstab (informational)
    fs::write(etc.join("fstab"), "\
# LevitateOS filesystem table
# Mounts are handled by kernel, this is informational only
#
# <device>     <mount>     <type>    <options>   <dump> <pass>
devtmpfs       /dev        devtmpfs  rw          0      0
tmpfs          /tmp        tmpfs     rw,nosuid   0      0
tmpfs          /run        tmpfs     rw,nosuid   0      0
")?;

    Ok(())
}
```

**Estimated effort**: 1-2 hours

---

### Unit of Work 3: devtmpfs Filesystem

**Files**: New `crates/kernel/src/fs/devtmpfs/`

```rust
// crates/kernel/src/fs/devtmpfs/mod.rs

mod node;
mod ops;
mod superblock;

use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use spin::RwLock;

pub use node::DevtmpfsNode;
pub use ops::{DevtmpfsFileOps, DevtmpfsDirOps};
pub use superblock::DevtmpfsSuperblock;

/// Device filesystem for /dev
pub struct Devtmpfs {
    root: Arc<DevtmpfsNode>,
    nodes: RwLock<BTreeMap<u64, Arc<DevtmpfsNode>>>,
    next_ino: AtomicU64,
}

impl Devtmpfs {
    pub fn new() -> Arc<Self> {
        let root = Arc::new(DevtmpfsNode::new_dir(1, 0o755));
        let mut nodes = BTreeMap::new();
        nodes.insert(1, root.clone());

        Arc::new(Self {
            root,
            nodes: RwLock::new(nodes),
            next_ino: AtomicU64::new(2),
        })
    }

    /// Create a device node
    pub fn mknod(&self, name: &str, major: u32, minor: u32, mode: u32) -> Result<(), VfsError> {
        let ino = self.next_ino.fetch_add(1, Ordering::Relaxed);
        let node = Arc::new(DevtmpfsNode::new_device(ino, mode, major, minor));

        self.root.add_child(name, node.clone())?;
        self.nodes.write().insert(ino, node);
        Ok(())
    }

    /// Create a directory
    pub fn mkdir(&self, name: &str) -> Result<(), VfsError> {
        let ino = self.next_ino.fetch_add(1, Ordering::Relaxed);
        let node = Arc::new(DevtmpfsNode::new_dir(ino, 0o755));

        self.root.add_child(name, node.clone())?;
        self.nodes.write().insert(ino, node);
        Ok(())
    }

    /// Initialize standard devices
    pub fn init_standard_devices(&self) -> Result<(), VfsError> {
        // Null devices
        self.mknod("null", 1, 3, 0o666)?;
        self.mknod("zero", 1, 5, 0o666)?;
        self.mknod("full", 1, 7, 0o666)?;

        // Random devices
        self.mknod("random", 1, 8, 0o666)?;
        self.mknod("urandom", 1, 9, 0o444)?;

        // TTY devices
        self.mknod("tty", 5, 0, 0o666)?;
        self.mknod("console", 5, 1, 0o600)?;
        self.mknod("ptmx", 5, 2, 0o666)?;

        // PTY slave directory
        self.mkdir("pts")?;

        Ok(())
    }
}

/// Global devtmpfs instance
static DEVTMPFS: Once<Arc<Devtmpfs>> = Once::new();

pub fn init() -> Arc<Devtmpfs> {
    DEVTMPFS.call_once(|| {
        let fs = Devtmpfs::new();
        fs.init_standard_devices().expect("Failed to init devices");
        fs
    }).clone()
}
```

**Estimated effort**: 6-8 hours

---

### Unit of Work 4: Device Operations

**Files**: `crates/kernel/src/fs/devtmpfs/ops.rs`

```rust
// Device-specific I/O operations

use crate::random::get_random_bytes;

/// Read from a device
pub fn device_read(major: u32, minor: u32, buf: &mut [u8], _offset: u64) -> Result<usize, VfsError> {
    match (major, minor) {
        // Memory devices (major 1)
        (1, 3) => {
            // /dev/null: always returns EOF
            Ok(0)
        }
        (1, 5) => {
            // /dev/zero: fills buffer with zeros
            buf.fill(0);
            Ok(buf.len())
        }
        (1, 7) => {
            // /dev/full: reads zeros
            buf.fill(0);
            Ok(buf.len())
        }
        (1, 8) | (1, 9) => {
            // /dev/random, /dev/urandom: random bytes
            fill_random(buf);
            Ok(buf.len())
        }

        // TTY devices (major 5)
        (5, 0) => {
            // /dev/tty: read from controlling terminal
            read_controlling_tty(buf)
        }
        (5, 1) => {
            // /dev/console: read from system console
            read_console(buf)
        }
        (5, 2) => {
            // /dev/ptmx: open returns fd, can't read directly
            Err(VfsError::NotSupported)
        }

        // PTY slaves (major 136)
        (136, n) => {
            read_pty_slave(n as usize, buf)
        }

        _ => Err(VfsError::NotSupported),
    }
}

/// Write to a device
pub fn device_write(major: u32, minor: u32, buf: &[u8], _offset: u64) -> Result<usize, VfsError> {
    match (major, minor) {
        // Memory devices (major 1)
        (1, 3) => {
            // /dev/null: discards all data
            Ok(buf.len())
        }
        (1, 5) => {
            // /dev/zero: discards all data
            Ok(buf.len())
        }
        (1, 7) => {
            // /dev/full: always returns ENOSPC
            Err(VfsError::NoSpace)
        }
        (1, 8) | (1, 9) => {
            // /dev/random, /dev/urandom: could seed entropy, just discard
            Ok(buf.len())
        }

        // TTY devices (major 5)
        (5, 0) => {
            // /dev/tty: write to controlling terminal
            write_controlling_tty(buf)
        }
        (5, 1) => {
            // /dev/console: write to system console
            write_console(buf)
        }
        (5, 2) => {
            // /dev/ptmx: can't write directly
            Err(VfsError::NotSupported)
        }

        // PTY slaves (major 136)
        (136, n) => {
            write_pty_slave(n as usize, buf)
        }

        _ => Err(VfsError::NotSupported),
    }
}

/// Simple PRNG for /dev/urandom (non-cryptographic)
fn fill_random(buf: &mut [u8]) {
    // Use architecture-specific random if available
    #[cfg(target_arch = "x86_64")]
    {
        use core::arch::x86_64::_rdrand64_step;
        let mut i = 0;
        while i + 8 <= buf.len() {
            let mut val: u64 = 0;
            unsafe {
                if _rdrand64_step(&mut val) == 1 {
                    buf[i..i+8].copy_from_slice(&val.to_le_bytes());
                }
            }
            i += 8;
        }
        // Fill remaining bytes
        if i < buf.len() {
            let mut val: u64 = 0;
            unsafe { _rdrand64_step(&mut val); }
            for j in i..buf.len() {
                buf[j] = (val >> ((j - i) * 8)) as u8;
            }
        }
        return;
    }

    // aarch64: timestamp-based PRNG (RDRAND not available)
    #[allow(unreachable_code)]
    {
        let mut state = crate::timer::get_timestamp_ns();
        for byte in buf.iter_mut() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *byte = (state >> 33) as u8;
        }
    }
}
```

**Estimated effort**: 4-6 hours

---

### Unit of Work 5: Boot Mount Sequence

**Files**: `crates/kernel/src/fs/mount.rs`

```rust
// Update mount::init() to include devtmpfs and other mounts

pub fn init() -> Result<(), MountError> {
    let mut table = MOUNT_TABLE.write();

    // 1. Root filesystem (initramfs)
    table.mount(Mount {
        mountpoint: "/".into(),
        fs_type: FsType::Initramfs,
        source: "initramfs".into(),
        flags: MountFlags::READONLY,
    })?;

    // 2. Device filesystem
    table.mount(Mount {
        mountpoint: "/dev".into(),
        fs_type: FsType::Devtmpfs,
        source: "devtmpfs".into(),
        flags: MountFlags::empty(),
    })?;

    // 3. PTY filesystem (inside /dev)
    table.mount(Mount {
        mountpoint: "/dev/pts".into(),
        fs_type: FsType::Devpts,
        source: "devpts".into(),
        flags: MountFlags::empty(),
    })?;

    // 4. Temporary filesystem
    table.mount(Mount {
        mountpoint: "/tmp".into(),
        fs_type: FsType::Tmpfs,
        source: "tmpfs".into(),
        flags: MountFlags::NOSUID | MountFlags::NODEV,
    })?;

    // 5. Runtime data
    table.mount(Mount {
        mountpoint: "/run".into(),
        fs_type: FsType::Tmpfs,
        source: "tmpfs".into(),
        flags: MountFlags::NOSUID | MountFlags::NODEV,
    })?;

    // 6. Variable log (optional, for log files)
    table.mount(Mount {
        mountpoint: "/var/log".into(),
        fs_type: FsType::Tmpfs,
        source: "tmpfs".into(),
        flags: MountFlags::empty(),
    })?;

    Ok(())
}
```

**Estimated effort**: 2-3 hours

---

### Unit of Work 6: PTY Migration

**Files**: `crates/kernel/src/syscall/fs/open.rs`, `crates/kernel/src/fs/devtmpfs/`

```rust
// Current: PTY hardcoded in syscall/fs/open.rs
// After: PTY routed through devtmpfs

// In open.rs, change:
fn open_special_file(path: &str, flags: i32) -> Option<i64> {
    // Old: hardcoded path matching
    // New: let VFS route to devtmpfs

    // Remove this special case:
    // if path == "/dev/ptmx" { ... }

    // Instead, open goes through VFS to devtmpfs
    // devtmpfs handles /dev/ptmx open specially
    None
}

// In devtmpfs, handle ptmx open:
impl InodeOps for DevtmpfsPtmxOps {
    fn open(&self, flags: i32) -> Result<OpenResult, VfsError> {
        // Allocate new PTY pair
        let pty_num = allocate_pty()?;

        // Create /dev/pts/{n} node
        let devtmpfs = get_devtmpfs();
        let pts_dir = devtmpfs.lookup("pts")?;
        pts_dir.mknod(&format!("{}", pty_num), 136, pty_num as u32, 0o620)?;

        // Return master fd
        Ok(OpenResult::PtyMaster(pty_num))
    }
}
```

**Estimated effort**: 4-6 hours

---

## File Change Summary

| File | Changes | Priority |
|------|---------|----------|
| `xtask/src/build/commands.rs` | FHS directory structure, /etc files | P0 |
| `crates/kernel/src/fs/devtmpfs/mod.rs` | New devtmpfs filesystem | P0 |
| `crates/kernel/src/fs/devtmpfs/node.rs` | Device node types | P0 |
| `crates/kernel/src/fs/devtmpfs/ops.rs` | Device I/O operations | P0 |
| `crates/kernel/src/fs/devtmpfs/superblock.rs` | VFS integration | P0 |
| `crates/kernel/src/fs/mount.rs` | Updated mount sequence | P1 |
| `crates/kernel/src/fs/mod.rs` | Add devtmpfs module | P0 |
| `crates/kernel/src/syscall/fs/open.rs` | PTY migration | P1 |

---

## Test Approach

### Unit Tests

```rust
#[test]
fn test_devtmpfs_mknod() {
    let fs = Devtmpfs::new();
    fs.mknod("test", 1, 1, 0o666).unwrap();
    assert!(fs.root.lookup("test").is_ok());
}

#[test]
fn test_dev_null_behavior() {
    assert_eq!(device_read(1, 3, &mut [0; 100], 0), Ok(0));
    assert_eq!(device_write(1, 3, &[1, 2, 3], 0), Ok(3));
}

#[test]
fn test_dev_zero_fills() {
    let mut buf = [0xFF; 100];
    device_read(1, 5, &mut buf, 0).unwrap();
    assert!(buf.iter().all(|&b| b == 0));
}

#[test]
fn test_dev_full_enospc() {
    assert_eq!(device_write(1, 7, &[1], 0), Err(VfsError::NoSpace));
}
```

### Integration Tests

```bash
# Run after boot
/bin/test -d /bin && echo "PASS: /bin exists"
/bin/test -d /usr/bin && echo "PASS: /usr/bin exists"
/bin/test -c /dev/null && echo "PASS: /dev/null is char device"
/bin/echo test > /dev/null && echo "PASS: write to /dev/null"
/bin/cat /etc/hostname && echo "PASS: /etc/hostname readable"
```

---

## Resolved Blockers

All questions answered. Decisions prioritize modern standards and compatibility.

| Question | Decision | Status |
|----------|----------|--------|
| Q1: Traditional vs merged /usr | **Merged /usr** | ✅ Resolved |
| Q2: devtmpfs or static /dev | **devtmpfs** | ✅ Resolved |
| Q3: Essential /dev nodes | **Extended (13+)** | ✅ Resolved |
| Q4: procfs scope | **Minimal /proc/self** | ✅ Resolved |
| Q5: /etc content | **Standard (5)** | ✅ Resolved |
| Q6: Binary location | **/usr/bin** | ✅ Resolved |
| Q7: /var structure | **Standard** | ✅ Resolved |
| Q8: Random quality | **RDRAND (x86_64)** | ✅ Resolved |

See `docs/questions/TEAM_401_filesystem_hierarchy.md` for full rationale.

---

## References

- Phase 2 Design: `docs/planning/filesystem-hierarchy/phase-2.md`
- Current tmpfs: `crates/kernel/src/fs/tmpfs/` (model)
- Current VFS: `crates/kernel/src/fs/vfs/`
