# VFS Reference Analysis: External Kernels

**Team:** TEAM_200  
**Created:** 2026-01-06

Analysis of VFS patterns from `.external-kernels/` for LevitateOS inspiration.

---

## 1. Theseus OS — Trait-Based VFS

### Key Patterns

#### `fs_node` crate — Core Traits

```rust
// File reference types
pub type FileRef = Arc<Mutex<dyn File + Send>>;
pub type DirRef = Arc<Mutex<dyn Directory + Send>>;
pub type WeakDirRef = Weak<Mutex<dyn Directory + Send>>;

// Base trait for all filesystem nodes
pub trait FsNode {
    fn get_name(&self) -> String;
    fn get_parent_dir(&self) -> Option<DirRef>;
    fn set_parent_dir(&mut self, new_parent: WeakDirRef);
    fn get_absolute_path(&self) -> String { /* default impl */ }
}

// File trait extends FsNode + I/O traits
pub trait File: FsNode + ByteReader + ByteWriter + KnownLength {
    fn as_mapping(&self) -> Result<&MappedPages, &'static str>;
}

// Directory trait extends FsNode
pub trait Directory: FsNode {
    fn get(&self, name: &str) -> Option<FileOrDir>;
    fn insert(&mut self, node: FileOrDir) -> Result<Option<FileOrDir>, &'static str>;
    fn remove(&mut self, node: &FileOrDir) -> Option<FileOrDir>;
    fn list(&self) -> Vec<String>;
}
```

#### `FileOrDir` enum — Unified Type

```rust
#[derive(Clone)]
pub enum FileOrDir {
    File(FileRef),
    Dir(DirRef),
}

impl FsNode for FileOrDir { /* delegates to inner */ }
```

### Best Practices from Theseus

| Pattern | Description | Apply to LevitateOS? |
|---------|-------------|---------------------|
| **Arc<Mutex<dyn Trait>>** | Dynamic dispatch with shared ownership | ✅ Yes |
| **WeakRef for parent** | Prevents reference cycles | ✅ Yes |
| **Separate File/Dir traits** | Clear separation of concerns | ✅ Yes |
| **FileOrDir enum** | Unified return type | ✅ Yes |
| **ByteReader/ByteWriter** | Generic I/O traits | ✅ Consider |

---

## 2. Theseus OS — Path Crate

### Key Patterns

```rust
// Path is a transparent wrapper around str (zero-cost)
#[repr(transparent)]
pub struct Path {
    inner: str,
}

// PathBuf owns a String
pub struct PathBuf {
    inner: String,
}

// Component enum for iteration
pub enum Component<'a> {
    RootDir,
    CurDir,      // .
    ParentDir,   // ..
    Normal(&'a str),
}
```

### Best Practices from Theseus Path

| Pattern | Description | Apply to LevitateOS? |
|---------|-------------|---------------------|
| **#[repr(transparent)]** | Zero-cost abstraction over str | ✅ Yes |
| **Path/PathBuf duality** | Like str/String | ✅ Yes |
| **Component enum** | Clean path iteration | ✅ Yes |
| **get() method with cwd** | Path resolution with context | ✅ Yes |

---

## 3. Theseus OS — MemFile (In-Memory File)

### Key Patterns

```rust
pub struct MemFile {
    name: String,
    len: usize,              // Actual file length
    mp: MappedPages,         // Backing memory
    parent: WeakDirRef,      // Parent directory
}

impl ByteReader for MemFile { /* read from mp */ }
impl ByteWriter for MemFile { /* write to mp, realloc if needed */ }
impl File for MemFile { /* as_mapping() */ }
impl FsNode for MemFile { /* get_name, get_parent */ }
```

### Best Practices from MemFile

| Pattern | Description | Apply to LevitateOS? |
|---------|-------------|---------------------|
| **Separate len vs capacity** | Don't confuse file size with backing store | ✅ Yes |
| **Realloc on grow** | Transparent expansion | ✅ Consider |
| **impl separate traits** | Clean composition | ✅ Yes |

---

## 4. Redox OS — Scheme-Based VFS

### Key Patterns

Redox uses a **scheme** abstraction instead of traditional VFS:

```rust
// Scheme trait — filesystem operations
pub trait KernelScheme: Send + Sync + 'static {
    fn kopen(&self, path: &str, flags: usize, ctx: CallerCtx) -> Result<OpenResult>;
    fn kread(&self, id: usize, buf: UserSliceWo, flags: u32) -> Result<usize>;
    fn kwrite(&self, id: usize, buf: UserSliceRo, flags: u32) -> Result<usize>;
    fn kfstat(&self, id: usize, buf: UserSliceWo) -> Result<()>;
    fn close(&self, id: usize) -> Result<()>;
    fn unlinkat(&self, file: usize, path: &str, flags: usize) -> Result<()>;
    fn getdents(&self, id: usize, buf: UserSliceWo) -> Result<usize>;
    // ... many more
}

// Scheme registry
pub struct SchemeList {
    map: HashMap<SchemeId, KernelSchemes>,
    names: HashMap<SchemeNamespace, IndexMap<Box<str>, SchemeId>>,
}
```

#### FileDescription — Open File Handle

```rust
pub struct FileDescription {
    pub offset: u64,           // Current seek position
    pub scheme: SchemeId,      // Which filesystem
    pub number: usize,         // Handle within scheme
    pub flags: u32,            // Open flags
    pub internal_flags: InternalFlags,
}

pub struct FileDescriptor {
    pub description: Arc<RwLock<FileDescription>>,
    pub cloexec: bool,
}
```

### Best Practices from Redox

| Pattern | Description | Apply to LevitateOS? |
|---------|-------------|---------------------|
| **Scheme as trait object** | Filesystem polymorphism | ✅ Similar to InodeOps |
| **SchemeId + number** | Two-level fd lookup | ⚠️ More complex than needed |
| **Arc<RwLock<FileDescription>>** | Shared file state | ✅ Yes |
| **CallerCtx (uid/gid/pid)** | Security context | ✅ Future (Phase 18) |
| **UserSlice types** | Safe userspace I/O | ✅ Yes |
| **Default trait impls return EOPNOTSUPP** | Optional operations | ✅ Yes |

---

## 5. Summary: Recommended Patterns for LevitateOS

### Core Types

```rust
// References
pub type InodeRef = Arc<RwLock<dyn Inode>>;
pub type WeakInodeRef = Weak<RwLock<dyn Inode>>;

// Path (zero-cost wrapper)
#[repr(transparent)]
pub struct Path { inner: str }
pub struct PathBuf { inner: String }

// Open file handle
pub struct File {
    pub inode: InodeRef,
    pub offset: AtomicU64,
    pub flags: u32,
}

// File descriptor wrapper
pub struct FileDescriptor {
    pub file: Arc<File>,
    pub cloexec: bool,
}
```

### Core Traits

```rust
pub trait Inode: Send + Sync {
    fn stat(&self) -> Result<Stat, VfsError>;
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, VfsError>;
    fn write(&self, offset: u64, buf: &[u8]) -> Result<usize, VfsError>;
}

pub trait DirectoryInode: Inode {
    fn lookup(&self, name: &str) -> Result<InodeRef, VfsError>;
    fn create(&self, name: &str, mode: u32) -> Result<InodeRef, VfsError>;
    fn mkdir(&self, name: &str, mode: u32) -> Result<InodeRef, VfsError>;
    fn unlink(&self, name: &str) -> Result<(), VfsError>;
    fn rmdir(&self, name: &str) -> Result<(), VfsError>;
    fn readdir(&self, offset: usize) -> Result<Option<DirEntry>, VfsError>;
}

pub trait SymlinkInode: Inode {
    fn readlink(&self) -> Result<PathBuf, VfsError>;
}
```

### Key Decisions

| Decision | Recommendation | Rationale |
|----------|----------------|-----------|
| **Lock type** | `RwLock` | Readers-writer for inode access |
| **Reference type** | `Arc<RwLock<dyn Trait>>` | Shared ownership + dynamic dispatch |
| **Parent reference** | `Weak<...>` | Prevent cycles |
| **Path type** | `#[repr(transparent)]` wrapper | Zero-cost |
| **Error type** | `VfsError` enum | Clear error cases |
| **Default impls** | Return `EOPNOTSUPP` | Optional operations |

---

## 6. Files to Reference

| File | Pattern |
|------|---------|
| `theseus/kernel/fs_node/src/lib.rs` | Core File/Directory traits |
| `theseus/kernel/vfs_node/src/lib.rs` | Concrete VFS implementation |
| `theseus/kernel/path/src/lib.rs` | Path/PathBuf abstraction |
| `theseus/kernel/memfs/src/lib.rs` | In-memory file (like our tmpfs) |
| `redox-kernel/src/scheme/mod.rs` | KernelScheme trait, SchemeList |
| `redox-kernel/src/context/file.rs` | FileDescription, FileDescriptor |
