# VFS Implementation Status

**Last Updated:** 2026-01-06  
**Teams:** TEAM_200 (planning), TEAM_201 (Phase 12), TEAM_202 (Phase 13), TEAM_203 (Phase 14-1), TEAM_204 (Phase 14-2), TEAM_205 (Phase 14-3)

---

## Quick Reference

| Phase | Status | Team | Description |
|-------|--------|------|-------------|
| 12 | ✅ DONE | TEAM_201 | VFS Foundation (Prerequisites) |
| 13 | ✅ DONE | TEAM_202 | Core VFS Implementation |
| 14 | 🟢 DOING | TEAM_203 | Filesystem Migration |

---

## ✅ DONE — Completed Work

### Phase 12: VFS Foundation (TEAM_201)

| Component | File | Description |
|-----------|------|-------------|
| **RwLock** | `crates/utils/src/rwlock.rs` | Readers-writer lock for inode access |
| **Path** | `kernel/src/fs/path.rs` | Zero-cost path abstraction |
| **Mount Table** | `kernel/src/fs/mount.rs` | Mount infrastructure with longest-prefix matching |
| **Extended Stat** | `kernel/src/syscall/mod.rs` | Full POSIX stat structure |
| **File Mode Constants** | `kernel/src/fs/mode.rs` | S_IFREG, S_IFDIR, permission bits |

### Phase 13: Core VFS (TEAM_202)

| Component | File | Description |
|-----------|------|-------------|
| **VfsError** | `kernel/src/fs/vfs/error.rs` | Error enum with POSIX errno mapping |
| **InodeOps** | `kernel/src/fs/vfs/ops.rs` | Filesystem operations trait |
| **FileOps** | `kernel/src/fs/vfs/ops.rs` | Open file operations trait |
| **Inode** | `kernel/src/fs/vfs/inode.rs` | In-memory file/dir representation |
| **Superblock** | `kernel/src/fs/vfs/superblock.rs` | Mounted filesystem instance trait |
| **File** | `kernel/src/fs/vfs/file.rs` | Open file handle with offset/flags |
| **Dentry** | `kernel/src/fs/vfs/dentry.rs` | Directory entry cache |
| **VFS Dispatch** | `kernel/src/fs/vfs/dispatch.rs` | vfs_open, vfs_read, vfs_write, etc. |

---

## 🟡 DOING — In Progress

### Phase 14: Filesystem Migration (TEAM_203)

Currently migrating existing filesystems to VFS. Tmpfs is complete.

| Step | Component | Status | Description |
|------|-----------|--------|-------------|
| 1 | **Tmpfs** | ✅ DONE | InodeOps + Superblock for Tmpfs |
| 2 | **Initramfs**| 🔴 TODO | Transition CPIO to VFS |
| 3 | **FdTable** | 🟡 DOING | Replace legacy variants with VfsFile |
| 4 | **Syscalls** | 🟡 DOING | Migrate all FS syscalls to VFS |
| 5 | **Mount** | 🔴 TODO | Implement sys_mount/sys_umount |
| 6 | **Boot** | 🟡 DOING | VFS-based boot initialization |

---

## 🔴 TODO — Remaining Work

### Phase 14: Filesystem Migration

**Goal:** Make existing filesystems implement VFS traits, then update syscalls to use VFS.

#### Step 1: Tmpfs Migration
```
Location: kernel/src/fs/tmpfs.rs
Task: Implement InodeOps + Superblock for TmpfsNode
Status: ✅ DONE (TEAM_203)
```

- [x] Create `TmpfsSuperblock` implementing `Superblock` trait
- [x] Create `TmpfsInodeOps` implementing `InodeOps` trait
- [x] Wire up existing `TmpfsNode` as the `private` data in `Inode`
- [x] Create root inode and register with dentry cache
- [x] **Robustness (TEAM_204)**: Added parent pointers and rename cycle checks

#### Step 2: Initramfs Migration
```
Location: kernel/src/fs/initramfs (new module)
Task: Implement InodeOps + Superblock for initramfs
Status: 🔴 TODO
```

- [ ] Create `InitramfsSuperblock` implementing `Superblock` trait
- [ ] Create `InitramfsInodeOps` implementing `InodeOps` (read-only)
- [ ] Create inodes from CPIO entries
- [ ] Mount at `/` during boot

#### Step 3: FdType Simplification
```
Location: kernel/src/task/fd_table.rs
Task: Replace per-fs FdType variants with Arc<File>
Status: 🔴 TODO
```

**Before:**
```rust
pub enum FdType {
    Stdin,
    Stdout,
    Stderr,
    InitramfsFile { file_index: usize, offset: usize },
    InitramfsDir { dir_index: usize, offset: usize },
    TmpfsFile { node: Arc<Spinlock<TmpfsNode>>, offset: usize },
    TmpfsDir { node: Arc<Spinlock<TmpfsNode>>, offset: usize },
}
```

**After:**
```rust
pub enum FdType {
    Stdin,
    Stdout,
    Stderr,
    File(Arc<vfs::File>),  // All files go through VFS
}
```

#### Step 4: Syscall Migration
```
Location: kernel/src/syscall/fs.rs
Task: Update syscalls to use VFS dispatch
Status: 🔴 TODO
```

- [ ] `sys_openat` → `vfs_open`
- [ ] `sys_read` → `vfs_read`
- [ ] `sys_write` → `vfs_write`
- [ ] `sys_fstat` → `vfs_fstat`
- [ ] `sys_getdents` → `vfs_readdir`
- [ ] `sys_mkdirat` → `vfs_mkdir`
- [ ] `sys_unlinkat` → `vfs_unlink` / `vfs_rmdir`
- [ ] `sys_renameat` → `vfs_rename`
- [ ] `sys_symlinkat` → `vfs_symlink`

#### Step 5: Mount/Umount Syscalls
```
Location: kernel/src/syscall/fs.rs
Task: Add mount/umount syscalls
Status: 🔴 TODO
```

- [ ] Add `sys_mount(source, target, fstype, flags)`
- [ ] Add `sys_umount(target, flags)`
- [ ] Wire up to mount table

#### Step 6: Boot Initialization
```
Location: kernel/src/init.rs
Task: Initialize VFS during boot
Status: 🔴 TODO
```

- [ ] Create initramfs superblock
- [ ] Create root dentry with initramfs root inode
- [ ] Mount tmpfs at `/tmp`
- [ ] Set dcache root

---

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│                  System Calls                    │
│   sys_read, sys_write, sys_openat, sys_stat...   │
└─────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────┐
│                 VFS Dispatch                     │
│   vfs_open, vfs_read, vfs_write, vfs_stat...    │
│   (kernel/src/fs/vfs/dispatch.rs)               │
└─────────────────────────────────────────────────┘
                        │
            ┌───────────┴───────────┐
            ▼                       ▼
┌─────────────────────┐   ┌─────────────────────┐
│    Dentry Cache     │   │    Mount Table      │
│ (path → inode)      │   │ (mount points)      │
│ vfs/dentry.rs       │   │ mount.rs            │
└─────────────────────┘   └─────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────┐
│                    Inode                         │
│   Generic file/dir representation               │
│   (kernel/src/fs/vfs/inode.rs)                  │
└─────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────┐
│                  InodeOps                        │
│   Filesystem-specific operations                │
│   (trait in vfs/ops.rs)                         │
└─────────────────────────────────────────────────┘
            │
    ┌───────┼───────┬───────────┐
    ▼       ▼       ▼           ▼
┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐
│ tmpfs │ │initram│ │ FAT32 │ │ ext4  │
│       │ │  fs   │ │       │ │       │
└───────┘ └───────┘ └───────┘ └───────┘
```

---

## File Locations Reference

### VFS Core (`kernel/src/fs/vfs/`)

| File | Contents |
|------|----------|
| `mod.rs` | Module exports |
| `error.rs` | `VfsError`, `VfsResult` |
| `ops.rs` | `InodeOps`, `FileOps`, `DirEntry`, `SetAttr` |
| `inode.rs` | `Inode`, `InodeRef`, `WeakInodeRef` |
| `superblock.rs` | `Superblock`, `StatFs` |
| `file.rs` | `File`, `OpenFlags`, `FileRef` |
| `dentry.rs` | `Dentry`, `DentryCache`, `dcache()` |
| `dispatch.rs` | `vfs_open`, `vfs_read`, etc. |

### VFS Foundation (`kernel/src/fs/`)

| File | Contents |
|------|----------|
| `path.rs` | `Path`, `PathBuf`, `Component` |
| `mount.rs` | `MountTable`, `Mount`, `FsType` |
| `mode.rs` | `S_IFREG`, `S_IFDIR`, permission bits |

### Utils (`crates/utils/src/`)

| File | Contents |
|------|----------|
| `rwlock.rs` | `RwLock`, `RwLockReadGuard`, `RwLockWriteGuard` |

---

## Patterns for Future Teams

### Pattern 1: Implementing a Filesystem

To make a filesystem work with VFS:

```rust
// 1. Create InodeOps implementation
pub struct MyFsInodeOps;

impl InodeOps for MyFsInodeOps {
    fn lookup(&self, inode: &Inode, name: &str) -> VfsResult<Arc<Inode>> {
        let private = inode.private::<MyFsData>().ok_or(VfsError::IoError)?;
        // ... look up child
    }
    
    fn read(&self, inode: &Inode, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let private = inode.private::<MyFsData>().ok_or(VfsError::IoError)?;
        // ... read data
    }
    
    // ... other ops with default NotSupported for unsupported operations
}

// 2. Create Superblock implementation
pub struct MyFsSuperblock {
    root: Arc<Inode>,
    next_ino: AtomicU64,
}

impl Superblock for MyFsSuperblock {
    fn root(&self) -> Arc<Inode> { self.root.clone() }
    fn fs_type(&self) -> &'static str { "myfs" }
    fn alloc_ino(&self) -> u64 { self.next_ino.fetch_add(1, Ordering::Relaxed) }
}

// 3. Create root inode
fn create_root_inode(sb: Weak<dyn Superblock>) -> Arc<Inode> {
    Arc::new(Inode::new(
        1,                          // ino
        0,                          // dev
        S_IFDIR | 0o755,           // mode
        &MY_FS_INODE_OPS,          // ops (static reference)
        sb,                         // superblock
        Box::new(MyFsRootData {}), // private data
    ))
}
```

### Pattern 2: Static InodeOps

InodeOps must be `&'static` because Inode has unbounded lifetime:

```rust
// Good: static instance
static TMPFS_INODE_OPS: TmpfsInodeOps = TmpfsInodeOps;

impl Inode::new(..., &TMPFS_INODE_OPS, ...)

// Bad: trying to pass owned
impl Inode::new(..., &TmpfsInodeOps {}, ...) // Won't compile
```

### Pattern 3: Private Data Access

Use `Any` downcasting to get filesystem-specific data:

```rust
fn read(&self, inode: &Inode, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
    // Downcast private data to your type
    let data = inode.private::<TmpfsNodeData>()
        .ok_or(VfsError::IoError)?;
    
    // Now use data
    data.read_at(offset, buf)
}
```

### Pattern 4: Weak References for Parent Pointers

Avoid reference cycles with `Weak`:

```rust
pub struct Dentry {
    parent: Option<Weak<Dentry>>,  // Weak to avoid cycle
    // ...
}

pub struct Inode {
    sb: Weak<dyn Superblock>,  // Weak to avoid cycle
    // ...
}
```

---

## Gotchas & Warnings

### Gotcha 1: Dentry Cache vs Filesystem State

The dentry cache is a **cache**, not the source of truth. If you modify filesystem state directly (e.g., create a file in tmpfs), you must also update the dentry cache:

```rust
// After creating in filesystem:
parent_dentry.add_child(new_dentry);

// After deleting from filesystem:
parent_dentry.remove_child(name);
dcache().invalidate(path);
```

### Gotcha 2: Stat Size Must Match

Kernel `Stat` and userspace `Stat` must be identical:

- `kernel/src/syscall/mod.rs` — kernel Stat
- `userspace/libsyscall/src/lib.rs` — userspace Stat

If you add a field to one, add it to both!

### Gotcha 3: FdType Migration Order

When migrating FdType:
1. First implement VFS for all filesystems
2. Then change FdType
3. Then update syscalls

Don't change FdType until all filesystems implement InodeOps.

### Gotcha 4: Boot Order

VFS initialization must happen in correct order:
1. Mount table init
2. Initramfs superblock creation
3. Root dentry creation
4. Tmpfs mount at /tmp

---

## Testing Checklist

Before marking Phase 14 complete:

- [ ] `cat /init` — reads from initramfs
- [ ] `echo test > /tmp/test` — writes to tmpfs
- [ ] `cat /tmp/test` — reads from tmpfs
- [ ] `ls /tmp` — lists tmpfs directory
- [ ] `mkdir /tmp/subdir` — creates tmpfs directory
- [ ] `rm /tmp/test` — removes tmpfs file
- [ ] All existing golden tests pass

---

## Team History

| Team | Phase | Contribution |
|------|-------|--------------|
| TEAM_200 | Planning | Created VFS plan, analyzed reference kernels |
| TEAM_201 | 12 | Implemented RwLock, Path, Mount, Stat, Mode |
| TEAM_202 | 13 | Implemented VFS core (Inode, File, Dentry, dispatch) |
| TBD | 14 | Filesystem migration (TODO) |

---

## Quick Start for Phase 14

```bash
# 1. Read the plan
cat docs/planning/vfs/phase-14.md

# 2. Create your team file
echo "# TEAM_203: Filesystem Migration" > .teams/TEAM_203_fs_migration.md

# 3. Start with tmpfs (simplest)
# Look at: kernel/src/fs/tmpfs.rs
# Implement: TmpfsInodeOps, TmpfsSuperblock

# 4. Build and test
cargo build --release
cargo xtask run
```
