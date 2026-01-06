//! TEAM_194: Tmpfs — In-memory writable filesystem for LevitateOS.
//!
//! Provides a writable scratch space at `/tmp` for levbox utilities.
//!
//! Design decisions (from phase-2.md):
//! - Mount point: `/tmp` only
//! - Max file size: 16MB
//! - Max total size: 64MB
//! - Locking: Global lock
//! - Hard links/symlinks: Deferred (EOPNOTSUPP)

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use los_utils::Spinlock;

use crate::fs::mode;
use crate::fs::vfs::error::{VfsError, VfsResult};
use crate::fs::vfs::inode::Inode;
use crate::fs::vfs::ops::{DirEntry, InodeOps};
use crate::fs::vfs::superblock::{StatFs, Superblock};

/// TEAM_194: Maximum file size (16MB)
pub const MAX_FILE_SIZE: usize = 16 * 1024 * 1024;

/// TEAM_194: Maximum total tmpfs size (64MB)
pub const MAX_TOTAL_SIZE: usize = 64 * 1024 * 1024;

/// TEAM_194: Tmpfs error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmpfsError {
    /// File or directory not found
    NotFound,
    /// File or directory already exists
    AlreadyExists,
    /// Not a directory
    NotADirectory,
    /// Not a file
    NotAFile,
    /// Directory not empty
    NotEmpty,
    /// No space left
    NoSpace,
    /// File too large
    FileTooLarge,
    /// Invalid path
    InvalidPath,
    /// Operation not supported
    NotSupported,
    /// Permission denied (cross-filesystem)
    CrossDevice,
}

/// TEAM_194: Node type enumeration
/// TEAM_198: Added Symlink variant
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TmpfsNodeType {
    File,
    Directory,
    Symlink,
}

/// TEAM_194: A node in the tmpfs tree (file or directory)
/// Note: No Debug derive to avoid Spinlock<TmpfsNode> Debug requirement in FdType
pub struct TmpfsNode {
    /// Unique inode number
    pub ino: u64,
    /// Node name (not full path)
    pub name: String,
    /// Node type
    pub node_type: TmpfsNodeType,
    /// File content (for files only)
    pub data: Vec<u8>,
    /// Child nodes (for directories only)
    pub children: Vec<Arc<Spinlock<TmpfsNode>>>,
    /// TEAM_198: Access time (seconds since boot)
    pub atime: u64,
    /// TEAM_198: Modification time (seconds since boot)
    pub mtime: u64,
    /// TEAM_198: Creation time (seconds since boot)
    pub ctime: u64,
    /// Parent node (Weak reference to avoid cycles)
    pub parent: Weak<Spinlock<TmpfsNode>>,
}

impl TmpfsNode {
    /// TEAM_194: Create a new file node
    /// TEAM_198: Added timestamp fields
    pub fn new_file(ino: u64, name: &str) -> Self {
        let now = crate::syscall::time::uptime_seconds();
        Self {
            ino,
            name: String::from(name),
            node_type: TmpfsNodeType::File,
            data: Vec::new(),
            children: Vec::new(),
            atime: now,
            mtime: now,
            ctime: now,
            parent: Weak::new(),
        }
    }

    /// TEAM_194: Create a new directory node
    /// TEAM_198: Added timestamp fields
    pub fn new_dir(ino: u64, name: &str) -> Self {
        let now = crate::syscall::time::uptime_seconds();
        Self {
            ino,
            name: String::from(name),
            node_type: TmpfsNodeType::Directory,
            data: Vec::new(),
            children: Vec::new(),
            atime: now,
            mtime: now,
            ctime: now,
            parent: Weak::new(),
        }
    }

    /// TEAM_198: Create a new symlink node
    pub fn new_symlink(ino: u64, name: &str, target: &str) -> Self {
        let now = crate::syscall::time::uptime_seconds();
        Self {
            ino,
            name: String::from(name),
            node_type: TmpfsNodeType::Symlink,
            data: target.as_bytes().to_vec(), // Store target path in data
            children: Vec::new(),
            atime: now,
            mtime: now,
            ctime: now,
            parent: Weak::new(),
        }
    }

    /// TEAM_194: Check if this is a file
    pub fn is_file(&self) -> bool {
        self.node_type == TmpfsNodeType::File
    }

    /// TEAM_194: Check if this is a directory
    pub fn is_dir(&self) -> bool {
        self.node_type == TmpfsNodeType::Directory
    }

    /// TEAM_198: Check if this is a symlink
    pub fn is_symlink(&self) -> bool {
        self.node_type == TmpfsNodeType::Symlink
    }

    /// TEAM_198: Get symlink target (returns None if not a symlink)
    pub fn symlink_target(&self) -> Option<&[u8]> {
        if self.is_symlink() {
            Some(&self.data)
        } else {
            None
        }
    }

    /// TEAM_194: Get file size
    pub fn is_lnk(&self) -> bool {
        self.node_type == TmpfsNodeType::Symlink
    }

    /// TEAM_204: Check if this node is a descendant of the given node
    pub fn is_descendant_of(&self, other_ino: u64) -> bool {
        if self.ino == other_ino {
            return true;
        }

        let mut curr_node = self.parent.upgrade();
        while let Some(node) = curr_node {
            let locked = node.lock();
            if locked.ino == other_ino {
                return true;
            }
            curr_node = locked.parent.upgrade();
        }

        false
    }
}

/// TEAM_203: Shared logic for creating nodes
fn tmpfs_add_child(
    parent: &Arc<Spinlock<TmpfsNode>>,
    child: Arc<Spinlock<TmpfsNode>>,
) -> VfsResult<()> {
    let mut parent_node = parent.lock();
    if !parent_node.is_dir() {
        return Err(VfsError::NotADirectory);
    }
    let name = child.lock().name.clone();
    for existing in &parent_node.children {
        if existing.lock().name == name {
            return Err(VfsError::AlreadyExists);
        }
    }
    child.lock().parent = Arc::downgrade(parent);
    parent_node.children.push(child);
    Ok(())
}

/// TEAM_194: The tmpfs filesystem state
pub struct Tmpfs {
    /// Root directory node
    root: Arc<Spinlock<TmpfsNode>>,
    /// Next inode number
    next_ino: AtomicU64,
    /// Total bytes used
    bytes_used: AtomicUsize,
    /// VFS root inode (cached)
    vfs_root: Spinlock<Option<Arc<Inode>>>,
}

impl Tmpfs {
    /// TEAM_194: Create a new tmpfs instance
    pub fn new() -> Self {
        Self {
            root: Arc::new(Spinlock::new(TmpfsNode::new_dir(1, ""))),
            next_ino: AtomicU64::new(2),
            bytes_used: AtomicUsize::new(0),
            vfs_root: Spinlock::new(None),
        }
    }

    /// TEAM_194: Allocate a new inode number
    fn alloc_ino(&self) -> u64 {
        self.next_ino.fetch_add(1, Ordering::SeqCst)
    }

    /// TEAM_194: Get total bytes used
    pub fn bytes_used(&self) -> usize {
        self.bytes_used.load(Ordering::SeqCst)
    }

    /// TEAM_203: Convert a TmpfsNode to a VFS Inode
    pub fn make_inode(
        &self,
        node: Arc<Spinlock<TmpfsNode>>,
        sb: Weak<dyn Superblock>,
    ) -> Arc<Inode> {
        let node_locked = node.lock();
        let ino = node_locked.ino;
        let node_type = node_locked.node_type;
        let mode = match node_type {
            TmpfsNodeType::File => mode::S_IFREG | 0o666,
            TmpfsNodeType::Directory => mode::S_IFDIR | 0o777,
            TmpfsNodeType::Symlink => mode::S_IFLNK | 0o777,
        };
        let size = node_locked.data.len() as u64;
        let atime = node_locked.atime;
        let mtime = node_locked.mtime;
        let ctime = node_locked.ctime;
        drop(node_locked);

        let ops: &'static dyn InodeOps = match node_type {
            TmpfsNodeType::File => &TMPFS_FILE_OPS,
            TmpfsNodeType::Directory => &TMPFS_DIR_OPS,
            TmpfsNodeType::Symlink => &TMPFS_SYMLINK_OPS,
        };

        let inode = Arc::new(Inode::new(
            ino,
            0, // dev id
            mode,
            ops,
            sb,
            Box::new(node),
        ));

        inode.size.store(size, Ordering::Relaxed);
        inode.atime.store(atime, Ordering::Relaxed);
        inode.mtime.store(mtime, Ordering::Relaxed);
        inode.ctime.store(ctime, Ordering::Relaxed);

        inode
    }
}

impl Superblock for Tmpfs {
    fn root(&self) -> Arc<Inode> {
        let root_cache = self.vfs_root.lock();
        if let Some(ref root) = *root_cache {
            return Arc::clone(root);
        }

        // We need a Weak<dyn Superblock> to self.
        // This is tricky for Tmpfs because it's usually inside an Arc.
        // For now, let's assume we can get it from the mount system later or just use Dummy Weak.
        // Actually, the caller of root() usually has the Arc<Superblock>.
        // But we are implementing root(&self).

        // Let's use a Dummy Weak for now if we don't have a way to get self-arc.
        // Or we can initialize it during mount.
        panic!("Tmpfs::root called before vfs_root was initialized");
    }

    fn statfs(&self) -> VfsResult<StatFs> {
        Ok(StatFs {
            f_type: 0x01021994, // Tmpfs magic
            f_bsize: 4096,
            f_blocks: (MAX_TOTAL_SIZE / 4096) as u64,
            f_bfree: ((MAX_TOTAL_SIZE - self.bytes_used()) / 4096) as u64,
            f_bavail: ((MAX_TOTAL_SIZE - self.bytes_used()) / 4096) as u64,
            f_files: 1024, // Arbitrary
            f_ffree: 1024,
            f_namelen: 255,
            f_frsize: 4096,
            f_flags: 0,
        })
    }

    fn fs_type(&self) -> &'static str {
        "tmpfs"
    }

    fn alloc_ino(&self) -> u64 {
        self.alloc_ino()
    }
}

/// TEAM_203: Tmpfs File Operations
struct TmpfsFileOps;
static TMPFS_FILE_OPS: TmpfsFileOps = TmpfsFileOps;

impl InodeOps for TmpfsFileOps {
    fn read(&self, inode: &Inode, offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        // We can't easily get Tmpfs instance here to call read_file.
        // But Tmpfs methods just lock the node and read.
        let node_inner = node.lock();
        if !node_inner.is_file() {
            return Err(VfsError::IsADirectory);
        }

        if offset >= node_inner.data.len() as u64 {
            return Ok(0);
        }

        let available = node_inner.data.len() - offset as usize;
        let to_read = buf.len().min(available);
        buf[..to_read]
            .copy_from_slice(&node_inner.data[offset as usize..offset as usize + to_read]);

        Ok(to_read)
    }

    fn write(&self, inode: &Inode, offset: u64, buf: &[u8]) -> VfsResult<usize> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let mut node_inner = node.lock();
        if !node_inner.is_file() {
            return Err(VfsError::IsADirectory);
        }

        let offset = offset as usize;
        let new_size = offset.saturating_add(buf.len());

        // Check max file size
        if new_size > MAX_FILE_SIZE {
            return Err(VfsError::FileTooLarge);
        }

        // Tmpfs from Superblock
        let _sb_arc = inode.sb.upgrade().ok_or(VfsError::IoError)?;
        let tmpfs_lock = TMPFS.lock();
        let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;

        // Check total space
        let old_size = node_inner.data.len();
        let size_delta = if new_size > old_size {
            new_size - old_size
        } else {
            0
        };
        let current_used = tmpfs.bytes_used.load(Ordering::SeqCst);

        if current_used + size_delta > MAX_TOTAL_SIZE {
            return Err(VfsError::NoSpace);
        }

        // Extend file if needed
        if new_size > node_inner.data.len() {
            node_inner.data.resize(new_size, 0);
            tmpfs.bytes_used.fetch_add(size_delta, Ordering::SeqCst);
        }

        // Write data
        node_inner.data[offset..offset + buf.len()].copy_from_slice(buf);
        node_inner.mtime = crate::syscall::time::uptime_seconds();
        inode
            .size
            .store(node_inner.data.len() as u64, Ordering::Relaxed);
        inode.mtime.store(node_inner.mtime, Ordering::Relaxed);

        Ok(buf.len())
    }

    fn truncate(&self, inode: &Inode, size: u64) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let mut node_inner = node.lock();
        if !node_inner.is_file() {
            return Err(VfsError::IsADirectory);
        }

        let new_size = size as usize;

        // Check max file size
        if new_size > MAX_FILE_SIZE {
            return Err(VfsError::FileTooLarge);
        }

        let _sb_arc = inode.sb.upgrade().ok_or(VfsError::IoError)?;
        let tmpfs_lock = TMPFS.lock();
        let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;

        let old_size = node_inner.data.len();

        if new_size < old_size {
            // Shrink file
            let freed_bytes = old_size - new_size;
            node_inner.data.truncate(new_size);
            tmpfs.bytes_used.fetch_sub(freed_bytes, Ordering::SeqCst);
        } else if new_size > old_size {
            // Extend file
            let added_bytes = new_size - old_size;
            let current_used = tmpfs.bytes_used.load(Ordering::SeqCst);
            if current_used + added_bytes > MAX_TOTAL_SIZE {
                return Err(VfsError::NoSpace);
            }
            node_inner.data.resize(new_size, 0);
            tmpfs.bytes_used.fetch_add(added_bytes, Ordering::SeqCst);
        }

        node_inner.mtime = crate::syscall::time::uptime_seconds();
        inode
            .size
            .store(node_inner.data.len() as u64, Ordering::Relaxed);
        inode.mtime.store(node_inner.mtime, Ordering::Relaxed);

        Ok(())
    }

    fn setattr(&self, inode: &Inode, attr: &crate::fs::vfs::ops::SetAttr) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;
        let mut node_inner = node.lock();

        if let Some(mode) = attr.mode {
            inode.mode.store(mode, Ordering::Relaxed);
            node_inner.mtime = crate::syscall::time::uptime_seconds();
            node_inner.ctime = node_inner.mtime;
        }

        if let Some(atime) = attr.atime {
            node_inner.atime = atime;
            inode.atime.store(atime, Ordering::Relaxed);
        }

        if let Some(mtime) = attr.mtime {
            node_inner.mtime = mtime;
            node_inner.ctime = mtime;
            inode.mtime.store(mtime, Ordering::Relaxed);
            inode.ctime.store(mtime, Ordering::Relaxed);
        }

        if let Some(size) = attr.size {
            drop(node_inner);
            self.truncate(inode, size)?;
        }

        Ok(())
    }
}

/// TEAM_203: Tmpfs Directory Operations
struct TmpfsDirOps;
static TMPFS_DIR_OPS: TmpfsDirOps = TmpfsDirOps;

impl InodeOps for TmpfsDirOps {
    fn lookup(&self, inode: &Inode, name: &str) -> VfsResult<Arc<Inode>> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let node_inner = node.lock();
        if !node_inner.is_dir() {
            return Err(VfsError::NotADirectory);
        }

        for child in &node_inner.children {
            let child_node = child.lock();
            if child_node.name == name {
                let sb = inode.sb.upgrade().ok_or(VfsError::IoError)?;
                let tmpfs_lock = TMPFS.lock();
                let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;
                return Ok(tmpfs.make_inode(Arc::clone(child), Arc::downgrade(&sb)));
            }
        }

        Err(VfsError::NotFound)
    }

    fn readdir(&self, inode: &Inode, offset: usize) -> VfsResult<Option<DirEntry>> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let node_inner = node.lock();
        if !node_inner.is_dir() {
            return Err(VfsError::NotADirectory);
        }

        // offsets 0 and 1 are . and ..
        if offset == 0 {
            return Ok(Some(DirEntry {
                ino: node_inner.ino,
                name: ".".to_string(),
                file_type: mode::S_IFDIR,
            }));
        }
        if offset == 1 {
            let parent_ino = if let Some(p) = node_inner.parent.upgrade() {
                p.lock().ino
            } else {
                node_inner.ino // root's parent is itself
            };
            return Ok(Some(DirEntry {
                ino: parent_ino,
                name: "..".to_string(),
                file_type: mode::S_IFDIR,
            }));
        }

        let child_idx = offset - 2;
        if child_idx < node_inner.children.len() {
            let child = &node_inner.children[child_idx];
            let child_node = child.lock();
            let de = DirEntry {
                ino: child_node.ino,
                name: child_node.name.clone(),
                file_type: mode::file_type(match child_node.node_type {
                    TmpfsNodeType::File => mode::S_IFREG,
                    TmpfsNodeType::Directory => mode::S_IFDIR,
                    TmpfsNodeType::Symlink => mode::S_IFLNK,
                }),
            };
            Ok(Some(de))
        } else {
            Ok(None)
        }
    }

    fn create(&self, inode: &Inode, name: &str, _mode: u32) -> VfsResult<Arc<Inode>> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let sb = inode.sb.upgrade().ok_or(VfsError::IoError)?;
        let tmpfs_lock = TMPFS.lock();
        let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;

        let ino = tmpfs.alloc_ino();
        let new_node = Arc::new(Spinlock::new(TmpfsNode::new_file(ino, name)));
        tmpfs_add_child(node, Arc::clone(&new_node))?;

        Ok(tmpfs.make_inode(new_node, Arc::downgrade(&sb)))
    }

    fn mkdir(&self, inode: &Inode, name: &str, _mode: u32) -> VfsResult<Arc<Inode>> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let sb = inode.sb.upgrade().ok_or(VfsError::IoError)?;
        let tmpfs_lock = TMPFS.lock();
        let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;

        let ino = tmpfs.alloc_ino();
        let new_node = Arc::new(Spinlock::new(TmpfsNode::new_dir(ino, name)));
        tmpfs_add_child(node, Arc::clone(&new_node))?;

        Ok(tmpfs.make_inode(new_node, Arc::downgrade(&sb)))
    }

    fn symlink(&self, inode: &Inode, name: &str, target: &str) -> VfsResult<Arc<Inode>> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let sb = inode.sb.upgrade().ok_or(VfsError::IoError)?;
        let tmpfs_lock = TMPFS.lock();
        let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;

        let ino = tmpfs.alloc_ino();
        let new_node = Arc::new(Spinlock::new(TmpfsNode::new_symlink(ino, name, target)));
        tmpfs_add_child(node, Arc::clone(&new_node))?;

        Ok(tmpfs.make_inode(new_node, Arc::downgrade(&sb)))
    }

    fn rename(
        &self,
        old_dir: &Inode,
        old_name: &str,
        new_dir: &Inode,
        new_name: &str,
    ) -> VfsResult<()> {
        let old_node = old_dir
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;
        let new_node = new_dir
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        // TEAM_204: Rename cycle check
        {
            let old_node_locked = old_dir
                .private::<Arc<Spinlock<TmpfsNode>>>()
                .ok_or(VfsError::IoError)?
                .lock();
            let mut to_move = None;
            for child in &old_node_locked.children {
                if child.lock().name == old_name {
                    to_move = Some(child.clone());
                    break;
                }
            }
            if let Some(child) = to_move {
                let child_locked = child.lock();
                if child_locked.is_dir() {
                    let new_dir_node = new_node.lock();
                    if new_dir_node.is_descendant_of(child_locked.ino) {
                        return Err(VfsError::InvalidArgument); // Moving dir into its own subdir
                    }
                }
            } else {
                return Err(VfsError::NotFound);
            }
        }

        if Arc::ptr_eq(&old_node, &new_node) {
            let mut locked = old_node.lock();
            if !locked.is_dir() {
                return Err(VfsError::NotADirectory);
            }

            let mut found_idx = None;
            for (idx, child) in locked.children.iter().enumerate() {
                if child.lock().name == old_name {
                    found_idx = Some(idx);
                    break;
                }
            }
            let idx = found_idx.ok_or(VfsError::NotFound)?;

            // Check if target exists
            let mut target_idx = None;
            for (t_idx, child) in locked.children.iter().enumerate() {
                if child.lock().name == new_name {
                    target_idx = Some(t_idx);
                    break;
                }
            }

            if let Some(t_idx) = target_idx {
                if t_idx == idx {
                    // Renaming to same name, nothing to do
                    return Ok(());
                }
                let existing = locked.children.remove(t_idx);
                if existing.lock().is_dir() && !existing.lock().children.is_empty() {
                    locked.children.insert(t_idx, existing);
                    return Err(VfsError::DirectoryNotEmpty);
                }
                // Update bytes_used if it was a file/symlink
                let tmpfs_lock = TMPFS.lock();
                let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;
                if !existing.lock().is_dir() {
                    tmpfs
                        .bytes_used
                        .fetch_sub(existing.lock().data.len(), Ordering::SeqCst);
                }

                // Adjust index if needed since we removed an element
                let final_idx = if t_idx < idx { idx - 1 } else { idx };
                let to_move = locked.children.remove(final_idx);
                to_move.lock().name = new_name.to_string();
                locked.children.insert(t_idx, to_move); // Insert at the target's old position
            } else {
                let to_move = locked.children.remove(idx);
                to_move.lock().name = new_name.to_string();
                locked.children.push(to_move);
            }
        } else {
            let mut old_locked = old_node.lock();
            let mut new_locked = new_node.lock();

            if !old_locked.is_dir() || !new_locked.is_dir() {
                return Err(VfsError::NotADirectory);
            }

            let mut found_idx = None;
            for (idx, child) in old_locked.children.iter().enumerate() {
                if child.lock().name == old_name {
                    found_idx = Some(idx);
                    break;
                }
            }
            let to_move_arc = old_locked
                .children
                .remove(found_idx.ok_or(VfsError::NotFound)?);

            // Check if target exists and remove it
            let mut target_idx = None;
            for (idx, child) in new_locked.children.iter().enumerate() {
                if child.lock().name == new_name {
                    target_idx = Some(idx);
                    break;
                }
            }
            if let Some(idx) = target_idx {
                let existing_child = new_locked.children.remove(idx);
                // If it's a directory, it must be empty
                if existing_child.lock().is_dir() && !existing_child.lock().children.is_empty() {
                    // Put it back and return error
                    new_locked.children.insert(idx, existing_child);
                    old_locked.children.insert(found_idx.unwrap(), to_move_arc); // Put back original
                    return Err(VfsError::DirectoryNotEmpty);
                }
                // If it's a file/symlink, or an empty directory, it's replaced.
                // Update bytes_used if it was a file/symlink
                let tmpfs_lock = TMPFS.lock();
                let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;
                if !existing_child.lock().is_dir() {
                    tmpfs
                        .bytes_used
                        .fetch_sub(existing_child.lock().data.len(), Ordering::SeqCst);
                }
            }

            to_move_arc.lock().name = new_name.to_string();
            new_locked.children.push(to_move_arc);
        }

        Ok(())
    }

    fn unlink(&self, inode: &Inode, name: &str) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let mut parent_node = node.lock();
        let mut found_idx = None;
        for (idx, child) in parent_node.children.iter().enumerate() {
            let child_node = (**child).lock();
            if child_node.name == name {
                if child_node.is_dir() {
                    return Err(VfsError::IsADirectory);
                }
                found_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = found_idx {
            let child = parent_node.children.remove(idx);
            let tmpfs_lock = TMPFS.lock();
            let tmpfs = tmpfs_lock.as_ref().ok_or(VfsError::IoError)?;
            tmpfs
                .bytes_used
                .fetch_sub(child.lock().data.len(), Ordering::SeqCst);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn rmdir(&self, inode: &Inode, name: &str) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;

        let mut parent_node = node.lock();
        let mut found_idx = None;
        for (idx, child) in parent_node.children.iter().enumerate() {
            let child_node = (**child).lock();
            if child_node.name == name {
                if !child_node.is_dir() {
                    return Err(VfsError::NotADirectory);
                }
                if !child_node.children.is_empty() {
                    return Err(VfsError::DirectoryNotEmpty);
                }
                found_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = found_idx {
            parent_node.children.remove(idx);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn setattr(&self, inode: &Inode, attr: &crate::fs::vfs::ops::SetAttr) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;
        let mut node_inner = node.lock();

        if let Some(mode) = attr.mode {
            inode.mode.store(mode, Ordering::Relaxed);
            node_inner.mtime = crate::syscall::time::uptime_seconds();
            node_inner.ctime = node_inner.mtime;
        }

        if let Some(atime) = attr.atime {
            node_inner.atime = atime;
            inode.atime.store(atime, Ordering::Relaxed);
        }

        if let Some(mtime) = attr.mtime {
            node_inner.mtime = mtime;
            node_inner.ctime = mtime;
            inode.mtime.store(mtime, Ordering::Relaxed);
            inode.ctime.store(mtime, Ordering::Relaxed);
        }

        Ok(())
    }
}

/// TEAM_203: Tmpfs Symlink Operations
struct TmpfsSymlinkOps;
static TMPFS_SYMLINK_OPS: TmpfsSymlinkOps = TmpfsSymlinkOps;

impl InodeOps for TmpfsSymlinkOps {
    fn readlink(&self, inode: &Inode) -> VfsResult<String> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;
        let node_inner = node.lock();
        if !node_inner.is_symlink() {
            return Err(VfsError::InvalidArgument);
        }
        Ok(String::from_utf8_lossy(&node_inner.data).to_string())
    }

    fn setattr(&self, inode: &Inode, attr: &crate::fs::vfs::ops::SetAttr) -> VfsResult<()> {
        let node = inode
            .private::<Arc<Spinlock<TmpfsNode>>>()
            .ok_or(VfsError::IoError)?;
        let mut node_inner = node.lock();

        if let Some(mode) = attr.mode {
            inode.mode.store(mode, Ordering::Relaxed);
            node_inner.mtime = crate::syscall::time::uptime_seconds();
            node_inner.ctime = node_inner.mtime;
        }

        if let Some(atime) = attr.atime {
            node_inner.atime = atime;
            inode.atime.store(atime, Ordering::Relaxed);
        }

        if let Some(mtime) = attr.mtime {
            node_inner.mtime = mtime;
            node_inner.ctime = mtime;
            inode.mtime.store(mtime, Ordering::Relaxed);
            inode.ctime.store(mtime, Ordering::Relaxed);
        }

        Ok(())
    }
}

impl Default for Tmpfs {
    fn default() -> Self {
        Self::new()
    }
}

/// TEAM_194: Global tmpfs instance
pub static TMPFS: Spinlock<Option<Arc<Tmpfs>>> = Spinlock::new(None);

/// TEAM_194: Initialize the tmpfs
pub fn init() {
    let mut tmpfs_lock = TMPFS.lock();
    let tmpfs = Arc::new(Tmpfs::new());

    // Initialize VFS root
    let root_inode = tmpfs.make_inode(
        Arc::clone(&tmpfs.root),
        Arc::downgrade(&(Arc::clone(&tmpfs) as Arc<dyn Superblock>)),
    );
    *tmpfs.vfs_root.lock() = Some(root_inode);

    *tmpfs_lock = Some(tmpfs);
}

/// TEAM_194: Check if a path is under /tmp
pub fn is_tmpfs_path(path: &str) -> bool {
    let normalized = path.trim_start_matches('/');
    normalized.starts_with("tmp/") || normalized == "tmp"
}

/// TEAM_194: Strip /tmp prefix from path
pub fn strip_tmp_prefix(path: &str) -> &str {
    let normalized = path.trim_start_matches('/');
    if normalized == "tmp" {
        ""
    } else if let Some(rest) = normalized.strip_prefix("tmp/") {
        rest
    } else {
        normalized
    }
}
