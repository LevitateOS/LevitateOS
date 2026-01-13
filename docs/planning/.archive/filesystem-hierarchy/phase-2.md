# Phase 2: Design - Filesystem Hierarchy Standard

**TEAM_401**: Filesystem Hierarchy Standard Compliance
**Created**: 2026-01-10
**Status**: Design Complete

---

## Proposed Solution

### Target Directory Structure

```
/
├── bin/                    # Essential user binaries (symlinks to coreutils)
│   ├── sh -> /usr/bin/sh
│   ├── cat -> /usr/bin/cat
│   ├── ls -> /usr/bin/ls
│   └── ...
├── sbin/                   # System binaries
│   └── init
├── dev/                    # Device files (devtmpfs)
│   ├── null               # Discard output
│   ├── zero               # Infinite zeros
│   ├── full               # Always full (ENOSPC on write)
│   ├── random             # Random bytes (alias for urandom)
│   ├── urandom            # Random bytes
│   ├── tty                # Current TTY
│   ├── console            # System console
│   ├── ptmx               # PTY master
│   └── pts/               # PTY slaves
│       ├── 0
│       └── ...
├── etc/                    # Configuration
│   ├── hostname           # System hostname
│   ├── passwd             # User database (stub)
│   ├── group              # Group database (stub)
│   ├── profile            # Shell profile
│   └── shells             # Valid shells
├── home/                   # User home directories
│   └── user/              # Default user
├── lib/                    # Libraries (empty - static only)
├── mnt/                    # Temporary mounts
├── opt/                    # Optional packages
├── proc/                   # Process info (procfs - deferred)
├── root/                   # Root home directory
├── run/                    # Runtime data (tmpfs)
│   └── utmp               # Login records (stub)
├── sys/                    # Kernel info (sysfs - deferred)
├── tmp/                    # Temporary files (tmpfs)
├── usr/                    # Secondary hierarchy
│   ├── bin/               # User binaries
│   │   ├── coreutils      # Multi-call binary
│   │   ├── env
│   │   ├── sh -> shell
│   │   └── ...
│   ├── lib/               # Libraries (empty)
│   └── share/             # Architecture-independent data
│       └── misc/
└── var/                    # Variable data
    ├── log/               # Log files (tmpfs)
    ├── tmp/               # Persistent temp (tmpfs)
    └── run -> /run        # Symlink for compatibility
```

### Mount Layout

```
Mountpoint      Filesystem    Source          Flags
/               initramfs     initramfs.cpio  ro
/tmp            tmpfs         tmpfs           rw,nosuid,nodev
/run            tmpfs         tmpfs           rw,nosuid,nodev
/var/log        tmpfs         tmpfs           rw
/dev            devtmpfs      devtmpfs        rw
/dev/pts        devpts        devpts          rw
/proc           procfs        proc            rw (deferred)
/sys            sysfs         sysfs           rw (deferred)
```

---

## API Design

### Devtmpfs Implementation

```rust
// crates/kernel/src/fs/devtmpfs/mod.rs

/// Device types supported by devtmpfs
pub enum DeviceType {
    CharDevice { major: u32, minor: u32 },
    BlockDevice { major: u32, minor: u32 },
}

/// Standard device numbers (Linux compatible)
pub mod devices {
    // Character devices
    pub const DEV_NULL: (u32, u32) = (1, 3);      // /dev/null
    pub const DEV_ZERO: (u32, u32) = (1, 5);      // /dev/zero
    pub const DEV_FULL: (u32, u32) = (1, 7);      // /dev/full
    pub const DEV_RANDOM: (u32, u32) = (1, 8);    // /dev/random
    pub const DEV_URANDOM: (u32, u32) = (1, 9);   // /dev/urandom
    pub const DEV_TTY: (u32, u32) = (5, 0);       // /dev/tty
    pub const DEV_CONSOLE: (u32, u32) = (5, 1);   // /dev/console
    pub const DEV_PTMX: (u32, u32) = (5, 2);      // /dev/ptmx
    pub const DEV_PTS: (u32, u32) = (136, 0);     // /dev/pts/N base
}

/// Devtmpfs filesystem
pub struct Devtmpfs {
    root: Arc<DevtmpfsNode>,
    inodes: RwLock<BTreeMap<u64, Arc<DevtmpfsNode>>>,
}

impl Devtmpfs {
    /// Create device node
    pub fn mknod(&self, path: &str, dev_type: DeviceType, mode: u32) -> Result<(), VfsError>;

    /// Create directory (for /dev/pts, etc.)
    pub fn mkdir(&self, path: &str) -> Result<(), VfsError>;

    /// Initialize standard devices
    pub fn init_standard_devices(&self) -> Result<(), VfsError> {
        self.mknod("null", CharDevice { major: 1, minor: 3 }, 0o666)?;
        self.mknod("zero", CharDevice { major: 1, minor: 5 }, 0o666)?;
        self.mknod("full", CharDevice { major: 1, minor: 7 }, 0o666)?;
        self.mknod("random", CharDevice { major: 1, minor: 8 }, 0o666)?;
        self.mknod("urandom", CharDevice { major: 1, minor: 9 }, 0o444)?;
        self.mknod("tty", CharDevice { major: 5, minor: 0 }, 0o666)?;
        self.mknod("console", CharDevice { major: 5, minor: 1 }, 0o600)?;
        self.mknod("ptmx", CharDevice { major: 5, minor: 2 }, 0o666)?;
        self.mkdir("pts")?;
        Ok(())
    }
}
```

### Device Operations

```rust
// crates/kernel/src/fs/devtmpfs/ops.rs

/// Device-specific read/write operations
pub fn device_read(major: u32, minor: u32, buf: &mut [u8], offset: u64) -> Result<usize, VfsError> {
    match (major, minor) {
        (1, 3) => Ok(0),                           // /dev/null: EOF
        (1, 5) => { buf.fill(0); Ok(buf.len()) }   // /dev/zero: zeros
        (1, 7) => { buf.fill(0); Ok(buf.len()) }   // /dev/full: zeros on read
        (1, 8) | (1, 9) => fill_random(buf),       // /dev/random, urandom
        (5, 0) => read_current_tty(buf),           // /dev/tty
        (5, 1) => read_console(buf),               // /dev/console
        (5, 2) => Err(VfsError::NotSupported),     // /dev/ptmx: open-only
        (136, n) => read_pty_slave(n, buf),        // /dev/pts/N
        _ => Err(VfsError::NotSupported),
    }
}

pub fn device_write(major: u32, minor: u32, buf: &[u8], offset: u64) -> Result<usize, VfsError> {
    match (major, minor) {
        (1, 3) => Ok(buf.len()),                   // /dev/null: discard
        (1, 5) => Ok(buf.len()),                   // /dev/zero: discard
        (1, 7) => Err(VfsError::NoSpace),          // /dev/full: ENOSPC
        (1, 8) | (1, 9) => Ok(buf.len()),          // /dev/random: discard
        (5, 0) => write_current_tty(buf),          // /dev/tty
        (5, 1) => write_console(buf),              // /dev/console
        (136, n) => write_pty_slave(n, buf),       // /dev/pts/N
        _ => Err(VfsError::NotSupported),
    }
}
```

### Random Number Generation

```rust
// Integration with existing RNG or simple PRNG

/// Fill buffer with random bytes
fn fill_random(buf: &mut [u8]) -> Result<usize, VfsError> {
    // Use existing kernel RNG if available, or simple PRNG
    // Use RDRAND on x86_64, fallback to PRNG on aarch64
    let mut state = get_timestamp_ns();
    for byte in buf.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        *byte = (state >> 33) as u8;
    }
    Ok(buf.len())
}
```

---

## Data Model Changes

### Mount Table Extensions

```rust
// crates/kernel/src/fs/mount.rs

pub enum FsType {
    Initramfs,
    Tmpfs,
    Fat32,
    Ext4,
    Devtmpfs,   // NEW
    Devpts,     // NEW (for /dev/pts)
    Procfs,     // NEW (deferred)
    Sysfs,      // NEW (deferred)
}
```

### Initramfs Content Changes

```rust
// xtask/src/build/commands.rs

fn create_fhs_structure(root: &Path) -> Result<()> {
    // Create FHS directories
    let dirs = [
        "bin", "sbin", "etc", "dev", "home", "home/user",
        "lib", "mnt", "opt", "proc", "root", "run", "sys",
        "tmp", "usr", "usr/bin", "usr/lib", "usr/share",
        "var", "var/log", "var/tmp",
    ];

    for dir in dirs {
        fs::create_dir_all(root.join(dir))?;
    }

    // Create /bin symlinks
    for util in COREUTILS_BINARIES {
        symlink(format!("../usr/bin/{}", util), root.join("bin").join(util))?;
    }

    // Create /etc files
    fs::write(root.join("etc/hostname"), "levitate\n")?;
    fs::write(root.join("etc/passwd"), "root:x:0:0:root:/root:/bin/sh\n")?;
    fs::write(root.join("etc/group"), "root:x:0:\n")?;
    fs::write(root.join("etc/shells"), "/bin/sh\n")?;

    Ok(())
}
```

---

## Behavioral Decisions

### Directory Layout Decision

| Option | Pros | Cons |
|--------|------|------|
| **A) Traditional** | Simple, explicit | More symlinks |
| **B) Merged /usr** | Modern standard | Requires symlinks at / |

**Decision**: **B) Merged /usr** - Start modern. Every major distro has moved here.

### /dev Implementation Decision

| Option | Pros | Cons |
|--------|------|------|
| **A) Static in initramfs** | Simple | Can't add devices at runtime |
| **B) devtmpfs** | Dynamic, proper | More code |

**Decision**: **B) devtmpfs** - Needed for PTY allocation anyway.

### Essential Devices

| Device | Path | Behavior | Priority |
|--------|------|----------|----------|
| null | /dev/null | Read: EOF, Write: discard | P0 |
| zero | /dev/zero | Read: zeros, Write: discard | P0 |
| full | /dev/full | Read: zeros, Write: ENOSPC | P1 |
| urandom | /dev/urandom | Read: random, Write: discard | P0 |
| random | /dev/random | Same as urandom | P1 |
| tty | /dev/tty | Current controlling TTY | P1 |
| console | /dev/console | System console | P1 |
| ptmx | /dev/ptmx | PTY master allocator | P0 (exists) |
| pts/N | /dev/pts/N | PTY slaves | P0 (exists) |

### /etc Content

| File | Content | Purpose |
|------|---------|---------|
| hostname | `levitate` | System hostname |
| passwd | `root:x:0:0:root:/root:/bin/sh` | User database |
| group | `root:x:0:` | Group database |
| shells | `/bin/sh` | Valid shells |
| profile | Shell initialization | Environment setup |

---

## Migration Path

### Phase 1: Restructure Initramfs

1. Create FHS directories in initramfs build
2. Move binaries to `/usr/bin/`
3. Create symlinks in `/bin/`
4. Add `/etc/` config files
5. Update shell to use new paths

### Phase 2: Add devtmpfs

1. Implement devtmpfs filesystem
2. Mount at `/dev/` during boot
3. Create standard device nodes
4. Migrate PTY handling to devfs

### Phase 3: Additional Mounts

1. Mount tmpfs at `/run/`
2. Mount tmpfs at `/var/log/`
3. Create symlink `/var/run` -> `/run`

### Phase 4: Pseudo-filesystems (Deferred)

1. Implement procfs
2. Implement sysfs
3. Mount at `/proc/` and `/sys/`

---

## Open Questions

### Q1: Traditional vs Merged /usr?

**Options**:
- **A) Traditional** (Recommended)
  - `/bin/` contains essential binaries
  - `/usr/bin/` contains additional binaries
  - Clear separation

- **B) Merged /usr**
  - `/bin -> /usr/bin` symlink
  - `/sbin -> /usr/sbin` symlink
  - Modern Linux default

**Recommendation**: B) Merged /usr - we're building a new OS, start modern.

### Q2: devtmpfs or Static /dev?

**Options**:
- **A) Static in initramfs**
  - Device nodes created at build time
  - Can't add devices at runtime
  - Simpler

- **B) devtmpfs** (Recommended)
  - Dynamic device creation
  - Proper for PTY allocation
  - More flexible

**Recommendation**: B) devtmpfs - necessary for proper PTY support.

### Q3: Essential /dev Nodes?

**Options**:
- **A) Minimal**: null, zero, urandom, ptmx, pts/
- **B) Standard** (Recommended): Add tty, console, random, full
- **C) Extended**: Add fd, stdin, stdout, stderr symlinks

**Recommendation**: B) Standard - covers most program needs.

### Q4: procfs Scope?

**Options**:
- **A) Defer entirely** (Recommended for M1)
- **B) Minimal**: /proc/self only
- **C) Basic**: /proc/{pid}/ directories
- **D) Full**: meminfo, cpuinfo, etc.

**Recommendation**: B) Minimal /proc/self - /proc/self/exe is needed for program self-location.

---

## Dependencies

### Internal Dependencies

| Component | Depends On | Status |
|-----------|------------|--------|
| devtmpfs | VFS layer | ✅ Ready |
| devtmpfs | Tmpfs (model) | ✅ Ready |
| /dev/pts | PTY subsystem | ✅ Ready |
| Initramfs structure | xtask build | ✅ Ready |

### External Dependencies

None - all can be implemented with existing infrastructure.

---

## Test Strategy Preview

### Unit Tests

```rust
#[test]
fn test_dev_null_read_eof() {
    let null = open("/dev/null", O_RDONLY);
    let mut buf = [0u8; 100];
    assert_eq!(read(null, &mut buf), Ok(0)); // EOF
}

#[test]
fn test_dev_null_write_discard() {
    let null = open("/dev/null", O_WRONLY);
    assert_eq!(write(null, b"test"), Ok(4));
}

#[test]
fn test_dev_zero_fills() {
    let zero = open("/dev/zero", O_RDONLY);
    let mut buf = [0xFFu8; 100];
    read(zero, &mut buf);
    assert!(buf.iter().all(|&b| b == 0));
}

#[test]
fn test_dev_full_enospc() {
    let full = open("/dev/full", O_WRONLY);
    assert_eq!(write(full, b"test"), Err(ENOSPC));
}
```

### Integration Tests

```bash
# Verify FHS structure
test -d /bin && echo "PASS: /bin exists"
test -d /usr/bin && echo "PASS: /usr/bin exists"
test -d /etc && echo "PASS: /etc exists"

# Verify symlinks
test -L /bin/cat && echo "PASS: /bin/cat is symlink"

# Verify devices
test -c /dev/null && echo "PASS: /dev/null is char device"
echo test > /dev/null && echo "PASS: /dev/null accepts writes"

# Verify config
cat /etc/hostname | grep -q levitate && echo "PASS: hostname set"
```

---

## References

- Phase 1 Discovery: `docs/planning/filesystem-hierarchy/phase-1.md`
- Current VFS: `crates/kernel/src/fs/vfs/`
- Current tmpfs: `crates/kernel/src/fs/tmpfs/`
- [FHS 3.0 Spec](https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html)
