# Directory Iteration Feature — Phase 2: Design

**Team:** TEAM_175  
**Created:** 2026-01-06  
**Status:** QUESTIONS PENDING  
**Parent:** `phase-1.md`

---

## 1. Proposed Solution

### 1.1 High-Level Description

Add directory enumeration capability through three layers:

1. **Kernel**: `sys_getdents(fd, buf, len)` syscall that reads directory entries from an open directory fd into a userspace buffer
2. **libsyscall**: Thin wrapper around the syscall
3. **ulib**: `ReadDir` iterator that provides ergonomic Rust API

### 1.2 User-Facing Behavior

```rust
use ulib::fs::{ReadDir, read_dir};

// Iterate over directory contents
for entry in read_dir("/")? {
    let entry = entry?;
    println!("{} (type: {:?})", entry.file_name(), entry.file_type());
}

// Or with explicit construction
let dir = ReadDir::open("/")?;
for entry in dir {
    // ...
}
```

### 1.3 System Behavior

**Flow:**
1. User calls `read_dir("/path")`
2. ulib calls `openat("/path", O_DIRECTORY)` to get directory fd
3. ulib allocates internal buffer (size TBD - see Q1)
4. ulib calls `getdents(fd, buf, len)` via libsyscall
5. Kernel reads directory entries from initramfs into buffer
6. Kernel returns number of bytes written (or negative error)
7. ulib parses `Dirent64` records from buffer
8. Each `next()` call returns parsed `DirEntry` or fetches more
9. When done, `drop(ReadDir)` closes the directory fd

---

## 2. API Design

### 2.1 Kernel Syscall

```rust
// kernel/src/syscall/mod.rs
pub const SYS_GETDENTS: u64 = 14;

// kernel/src/syscall/sys.rs
/// Read directory entries into buffer.
/// 
/// # Arguments
/// * `fd` - Directory file descriptor (must be opened with O_DIRECTORY)
/// * `buf` - Userspace buffer to write Dirent64 records
/// * `buf_len` - Size of buffer in bytes
///
/// # Returns
/// * `> 0` - Number of bytes written to buffer
/// * `0` - End of directory (no more entries)
/// * `< 0` - Error code (EBADF, ENOTDIR, EFAULT, EINVAL)
fn sys_getdents(fd: usize, buf: *mut u8, buf_len: usize) -> isize;
```

### 2.2 libsyscall Wrapper

```rust
// userspace/libsyscall/src/lib.rs

/// Dirent64 structure matching Linux ABI.
#[repr(C)]
pub struct Dirent64 {
    pub d_ino: u64,       // Inode number
    pub d_off: i64,       // Offset to next entry
    pub d_reclen: u16,    // Length of this record
    pub d_type: u8,       // File type
    // d_name follows (null-terminated, variable length)
}

/// File type constants
pub mod d_type {
    pub const DT_UNKNOWN: u8 = 0;
    pub const DT_FIFO: u8 = 1;
    pub const DT_CHR: u8 = 2;
    pub const DT_DIR: u8 = 4;
    pub const DT_BLK: u8 = 6;
    pub const DT_REG: u8 = 8;
    pub const DT_LNK: u8 = 10;
    pub const DT_SOCK: u8 = 12;
}

pub const SYS_GETDENTS: u64 = 14;

/// Read directory entries.
#[inline]
pub fn getdents(fd: usize, buf: &mut [u8]) -> isize;
```

### 2.3 ulib Types

```rust
// userspace/ulib/src/fs.rs

/// Iterator over directory entries.
pub struct ReadDir {
    fd: usize,
    buf: Vec<u8>,       // Internal buffer
    pos: usize,         // Current position in buffer
    end: usize,         // End of valid data in buffer
    finished: bool,     // No more entries from kernel
}

/// A single directory entry.
pub struct DirEntry {
    name: String,
    ino: u64,
    file_type: FileType,
}

/// File type from directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Other,
}

impl ReadDir {
    /// Open a directory for iteration.
    pub fn open(path: &str) -> Result<Self>;
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
    fn next(&mut self) -> Option<Self::Item>;
}

impl DirEntry {
    /// Get the file name (without path).
    pub fn file_name(&self) -> &str;
    
    /// Get the file type.
    pub fn file_type(&self) -> FileType;
    
    /// Get the inode number.
    pub fn ino(&self) -> u64;
}

/// Convenience function to read a directory.
pub fn read_dir(path: &str) -> Result<ReadDir>;
```

---

## 3. Data Model Changes

### 3.1 Kernel Changes

**Initramfs directory support:**
```rust
// kernel/src/fs/initramfs.rs

/// Directory entry in initramfs
struct InitramfsEntry {
    name: &'static str,
    kind: EntryKind,
    data: Option<&'static [u8]>,  // For files
}

enum EntryKind {
    File,
    Directory,
}

/// Get entries for a directory path
fn get_directory_entries(path: &str) -> Option<&[InitramfsEntry]>;
```

**File descriptor tracking:**
```rust
// Extend FileKind to track directory state
enum FileKind {
    Console(ConsoleKind),
    InitramfsFile { data: &'static [u8], offset: usize },
    InitramfsDir { path: &'static str, offset: usize },  // NEW
}
```

### 3.2 No Migration Needed

- New functionality only
- Existing file operations unchanged

---

## 4. Behavioral Decisions (QUESTIONS)

### Q1: ReadDir Internal Buffer Size

**Question:** What should be the default buffer size for `ReadDir`?

**Options:**
- A) 256 bytes (minimal, frequent syscalls)
- B) 1024 bytes (balance)
- C) 4096 bytes (one page, efficient)
- D) Configurable via `ReadDir::with_capacity()`

**Recommendation:** Option C (4096) - one page is efficient and handles most directories in single syscall.

**Trade-offs:**
- Smaller = less memory, more syscalls
- Larger = more memory, fewer syscalls
- Most directories have < 100 entries, 4KB handles ~50-100 entries

---

### Q2: Handling "." and ".." Entries

**Question:** Should `ReadDir` include `.` (current dir) and `..` (parent dir) entries?

**Options:**
- A) Include them (matches raw syscall, POSIX behavior)
- B) Filter them out (matches Rust `std::fs::read_dir` behavior)
- C) Make configurable

**Recommendation:** Option B - Rust std behavior filters them, reduces surprise for Rust developers.

---

### Q3: Error on Non-Directory

**Question:** What error when user calls `read_dir()` on a regular file?

**Options:**
- A) `ErrorKind::NotADirectory` (new error kind)
- B) `ErrorKind::InvalidArgument` (generic)
- C) `ErrorKind::NotFound` (misleading but simple)

**Recommendation:** Option A - explicit error is clearer for debugging.

---

### Q4: Empty Directory Behavior

**Question:** How should empty directories behave?

**Options:**
- A) Return iterator that immediately yields `None`
- B) Return `Ok(ReadDir)` where first `next()` returns `None`
- C) Return error (empty dir is "not found")

**Recommendation:** Option B - consistent with non-empty directories, empty is valid state.

---

### Q5: Initramfs Directory Structure

**Question:** Does the current initramfs support directories, or is it flat?

**Context:** Investigated `crates/utils/src/cpio.rs` - CPIO format has `c_mode` field that encodes file type (directories have mode `0o040000`). However, the current API only provides `get_file()` and flat iteration.

**Finding:** CPIO format supports directories, but kernel API needs enhancement:
- Need to parse `c_mode` to detect file type
- Need directory enumeration by prefix matching (e.g., list all entries starting with "subdir/")

**Options:**
- A) Enhance CpioArchive with `is_directory()` and `list_directory()` methods
- B) Implement directory support only in kernel syscall layer
- C) Flatten everything - no real directories, just path prefixes

**Recommendation:** Option A - add helpers to CpioArchive, reusable and testable.

---

### Q6: Syscall Number Assignment

**Question:** Should we use syscall number 14 (next sequential) or align with Linux `getdents64` (NR 61)?

**Options:**
- A) Use NR 14 (next sequential in our table)
- B) Use NR 61 (Linux compatible)
- C) Use custom high number to avoid future conflicts

**Recommendation:** Option A - our syscall numbers are already custom (not Linux-aligned), consistency is better than partial alignment.

**Note:** Phase 10 design doc suggested NR 12, but that's now used by `nanosleep`. NR 14 is next available.

---

### Q7: DirEntry Path vs Name

**Question:** Should `DirEntry` store full path or just filename?

**Options:**
- A) Filename only (what kernel returns)
- B) Full path (convenience, matches std::fs)
- C) Both (flexible)

**Recommendation:** Option A for MVP - avoids allocation and path construction. User can construct path if needed.

**Trade-off:** Rust `std::fs::DirEntry::path()` returns full path, but requires storing the base path. We can add this later.

---

## 5. Design Alternatives Considered

### 5.1 Stream-Based API (Rejected)

```rust
// Alternative: callback-based
fn read_dir_each(path: &str, f: impl FnMut(DirEntry)) -> Result<()>;
```

**Rejected because:** Iterator pattern is more ergonomic and composable in Rust.

### 5.2 Fixed-Size Entry Array (Rejected)

```rust
// Alternative: return array
fn read_dir(path: &str) -> Result<Vec<DirEntry>>;
```

**Rejected because:** Requires reading entire directory upfront, bad for large directories. Iterator is lazy.

### 5.3 Use Linux Syscall Numbers (Rejected)

**Rejected because:** Our syscall numbers are already custom (SYS_READ=0, not 63). Mixing would be confusing.

---

## 6. Open Questions Summary

| ID | Question | Options | Recommendation | Status |
|----|----------|---------|----------------|--------|
| Q1 | Buffer size | 256/1024/4096/Config | 4096 | ⏳ Pending |
| Q2 | Include . and .. | Yes/No/Config | No (filter) | ⏳ Pending |
| Q3 | Non-directory error | NotADir/Invalid/NotFound | NotADirectory | ⏳ Pending |
| Q4 | Empty dir behavior | None/Ok+None/Error | Ok + None | ⏳ Pending |
| Q5 | Initramfs dirs | Supported/Flat/Unknown | **Investigate** | ⏳ Pending |
| Q6 | Syscall number | 14/61/High | 14 | ⏳ Pending |
| Q7 | Entry path vs name | Name/Path/Both | Name only | ⏳ Pending |

---

## 7. Implementation Phases (Preview)

Once questions are answered, implementation will follow:

1. **Phase 3-Step 1**: Add `SYS_GETDENTS` to kernel (with initramfs support)
2. **Phase 3-Step 2**: Add `getdents()` to libsyscall
3. **Phase 3-Step 3**: Add `ReadDir`/`DirEntry` to ulib
4. **Phase 4**: Integration testing with `ls` command

---

## Next Steps

1. **User answers questions Q1-Q7**
2. **Investigate Q5** (initramfs directory support)
3. Update this document with decisions
4. Proceed to Phase 3 (Implementation)
