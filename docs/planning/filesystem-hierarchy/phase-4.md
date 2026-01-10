# Phase 4: Integration - Filesystem Hierarchy Standard

**TEAM_401**: Filesystem Hierarchy Standard Compliance
**Created**: 2026-01-10
**Status**: Pending Phase 3 Completion

---

## Integration Points

### VFS Integration

**File**: `crates/kernel/src/fs/vfs/dispatch.rs`

devtmpfs must integrate with VFS path resolution:

```rust
// VFS dispatch should route /dev/* paths to devtmpfs
fn resolve_filesystem(path: &str) -> Result<&dyn Filesystem, VfsError> {
    let mount_table = MOUNT_TABLE.read();

    // Longest-prefix matching already handles this
    // /dev/null -> matches /dev mount -> devtmpfs
    // /dev/pts/0 -> matches /dev/pts mount -> devpts

    mount_table.lookup(path)
}
```

### Mount Table Updates

**File**: `crates/kernel/src/fs/mount.rs`

```rust
// Add new filesystem types
pub enum FsType {
    Initramfs,
    Tmpfs,
    Fat32,
    Ext4,
    Devtmpfs,  // NEW
    Devpts,    // NEW
}

impl FsType {
    pub fn create_superblock(&self) -> Result<Arc<dyn Superblock>, MountError> {
        match self {
            FsType::Devtmpfs => {
                let fs = devtmpfs::init();
                Ok(Arc::new(devtmpfs::DevtmpfsSuperblock::new(fs)))
            }
            FsType::Devpts => {
                Ok(Arc::new(devpts::DevptsSuperblock::new()))
            }
            // ... existing cases
        }
    }
}
```

### Syscall Updates

**File**: `crates/kernel/src/syscall/fs/open.rs`

Remove hardcoded PTY handling, let VFS route:

```rust
pub fn sys_openat(dirfd: i32, path_ptr: usize, flags: i32, mode: u32) -> i64 {
    // Remove this block:
    // if path == "/dev/ptmx" {
    //     return handle_ptmx_open();
    // }

    // VFS handles everything, including /dev/ptmx
    vfs_open(resolved_path, flags, mode)
}
```

### xtask Build Integration

**File**: `xtask/src/build/commands.rs`

```rust
pub fn build_initramfs(arch: &str) -> Result<()> {
    let root = PathBuf::from(format!("initrd_root_{}", arch));

    // Clean previous build
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }

    // 1. Create FHS directory structure
    create_fhs_directories(&root)?;

    // 2. Install binaries to proper locations
    install_binaries_to_fhs(&root, arch)?;

    // 3. Create symlinks in /bin
    create_bin_symlinks(&root)?;

    // 4. Create /etc configuration files
    create_etc_files(&root)?;

    // 5. Create CPIO archive
    create_cpio_archive(&root, arch)?;

    Ok(())
}
```

---

## Test Strategy

### Unit Tests

```rust
// tests/fs/devtmpfs_tests.rs

mod devtmpfs_tests {
    #[test]
    fn devtmpfs_creates_standard_devices() {
        let fs = Devtmpfs::new();
        fs.init_standard_devices().unwrap();

        assert!(fs.lookup("null").is_ok());
        assert!(fs.lookup("zero").is_ok());
        assert!(fs.lookup("urandom").is_ok());
        assert!(fs.lookup("ptmx").is_ok());
        assert!(fs.lookup("pts").is_ok());
    }

    #[test]
    fn dev_null_returns_eof() {
        let mut buf = [0u8; 100];
        let result = device_read(1, 3, &mut buf, 0);
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn dev_null_accepts_writes() {
        let result = device_write(1, 3, b"test data", 0);
        assert_eq!(result, Ok(9));
    }

    #[test]
    fn dev_zero_fills_zeros() {
        let mut buf = [0xFF; 100];
        device_read(1, 5, &mut buf, 0).unwrap();
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn dev_full_returns_enospc() {
        let result = device_write(1, 7, b"test", 0);
        assert_eq!(result, Err(VfsError::NoSpace));
    }

    #[test]
    fn dev_urandom_fills_buffer() {
        let mut buf1 = [0u8; 32];
        let mut buf2 = [0u8; 32];
        device_read(1, 9, &mut buf1, 0).unwrap();
        device_read(1, 9, &mut buf2, 0).unwrap();
        // Very unlikely to be equal
        assert_ne!(buf1, buf2);
    }
}
```

### Behavior Tests

#### New Golden Files

```
tests/golden/
├── x86_64/
│   ├── fhs_structure.txt      # NEW: ls output of FHS directories
│   ├── dev_devices.txt        # NEW: ls -la /dev output
│   └── etc_files.txt          # NEW: /etc content verification
└── aarch64/
    ├── fhs_structure.txt
    ├── dev_devices.txt
    └── etc_files.txt
```

#### Behavior Test Scenarios

```yaml
# tests/behavior/fhs_structure.yaml
name: fhs_directory_structure
description: Verify FHS directories exist
steps:
  - boot kernel with verbose
  - run: ls -la /
  - expect: "bin"
  - expect: "sbin"
  - expect: "etc"
  - expect: "dev"
  - expect: "usr"
  - expect: "var"
  - expect: "tmp"

# tests/behavior/dev_devices.yaml
name: device_files
description: Verify /dev devices work
steps:
  - boot kernel
  - run: echo test > /dev/null
  - expect: (no output, success)
  - run: head -c 4 /dev/zero | od -x
  - expect: "0000 0000"
  - run: head -c 4 /dev/urandom | wc -c
  - expect: "4"
```

### Integration Tests

```bash
#!/bin/bash
# tests/integration/fhs_test.sh

echo "=== FHS Structure Test ==="

# Test directories exist
for dir in bin sbin etc dev home usr var tmp; do
    if [ -d "/$dir" ]; then
        echo "PASS: /$dir exists"
    else
        echo "FAIL: /$dir missing"
        exit 1
    fi
done

# Test /usr subdirectories
for dir in bin lib share; do
    if [ -d "/usr/$dir" ]; then
        echo "PASS: /usr/$dir exists"
    else
        echo "FAIL: /usr/$dir missing"
        exit 1
    fi
done

# Test /bin symlinks
if [ -L /bin/cat ]; then
    echo "PASS: /bin/cat is symlink"
else
    echo "FAIL: /bin/cat should be symlink"
fi

# Test /etc files
if [ -f /etc/hostname ]; then
    HOSTNAME=$(cat /etc/hostname)
    echo "PASS: hostname is '$HOSTNAME'"
else
    echo "FAIL: /etc/hostname missing"
fi

# Test device files
echo "test" > /dev/null && echo "PASS: /dev/null accepts writes"

if head -c 1 /dev/zero | od -c | grep -q '\\0'; then
    echo "PASS: /dev/zero returns zeros"
fi

echo "=== All FHS tests passed ==="
```

### Regression Tests

```rust
// tests/regress/fhs_compat.rs

#[test]
fn standard_paths_exist() {
    // These paths must exist for Unix compatibility
    let required_dirs = [
        "/bin", "/sbin", "/etc", "/dev", "/home",
        "/tmp", "/usr", "/usr/bin", "/var",
    ];

    for path in required_dirs {
        assert!(Path::new(path).is_dir(), "{} must be a directory", path);
    }
}

#[test]
fn bin_contains_essentials() {
    let essentials = ["sh", "cat", "ls", "echo", "test"];

    for cmd in essentials {
        let path = format!("/bin/{}", cmd);
        assert!(Path::new(&path).exists(), "{} must exist", path);
    }
}

#[test]
fn dev_contains_standard_devices() {
    let devices = ["null", "zero", "urandom", "tty"];

    for dev in devices {
        let path = format!("/dev/{}", dev);
        assert!(Path::new(&path).exists(), "{} must exist", path);
    }
}
```

---

## Impact Analysis

### Affected Subsystems

| Subsystem | Impact | Risk |
|-----------|--------|------|
| VFS | Low | New filesystem type |
| Mount table | Low | Additional mounts |
| Syscall dispatch | Medium | PTY routing change |
| Build system | Medium | New initramfs structure |
| Existing programs | Low | Paths still work |

### Breaking Changes

| Change | Impact | Mitigation |
|--------|--------|------------|
| Binary locations | Programs expecting `/shell` | Add compatibility symlinks |
| PTY path | Code using hardcoded paths | Route through VFS |
| Initramfs structure | Build system | Update xtask |

### Compatibility Matrix

| Program | Before | After | Status |
|---------|--------|-------|--------|
| Shell | `/shell` | `/bin/sh`, `/usr/bin/shell` | ✅ Symlink |
| cat | `/cat` | `/bin/cat` | ✅ Symlink |
| init | `/init` | `/sbin/init` | ✅ Move |
| Scripts | `#!/bin/sh` | `#!/bin/sh` | ✅ Works |

---

## Verification Checklist

### Before Merge

- [ ] All FHS directories created in initramfs
- [ ] /bin/ symlinks point to /usr/bin/
- [ ] /etc/ contains hostname, passwd, group
- [ ] devtmpfs mounts at /dev/
- [ ] /dev/null, /dev/zero, /dev/urandom work
- [ ] PTY still works via /dev/ptmx
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Both architectures tested

### After Merge

- [ ] CI passes
- [ ] Golden files updated
- [ ] Documentation updated

---

## Rollback Plan

If integration causes issues:

1. **Immediate**: Revert initramfs structure changes in xtask
2. **PTY issues**: Re-enable hardcoded PTY handling
3. **Mount issues**: Revert to old mount sequence

Changes are largely additive, rollback should be straightforward.

---

## References

- Phase 3 Implementation: `docs/planning/filesystem-hierarchy/phase-3.md`
- Current VFS: `crates/kernel/src/fs/vfs/`
- Current mount: `crates/kernel/src/fs/mount.rs`
