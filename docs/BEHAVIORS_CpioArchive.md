# CPIO Archive Behaviors

**File**: `src/builder/initramfs/cpio.rs`

**Purpose**: Pure Rust implementation of CPIO newc format archive writer, eliminating external `find | cpio` dependency and enabling type-safe archive construction.

**Format**: CPIO newc format (RFC 1413), 110-byte ASCII headers with 4-byte alignment padding

**Team Context**: TEAM_474 replaced shell script cpio calls with pure Rust implementation

---

## Table of Contents

- [CPIO Newc Format Overview](#cpio-newc-format-overview)
- [File Type Constants](#file-type-constants)
- [CpioEntry Struct](#cpioentry-struct)
- [CpioArchive Struct](#cpioarchive-struct)
- [Entry Addition Methods](#entry-addition-methods)
- [Archive Writing Behavior](#archive-writing-behavior)
- [Header Format Specification](#header-format-specification)
- [Path Normalization](#path-normalization)
- [Alignment Strategy](#alignment-strategy)
- [Trailer Handling](#trailer-handling)
- [State Management](#state-management)
- [Error Handling](#error-handling)
- [Testing Behaviors](#testing-behaviors)
- [Performance Characteristics](#performance-characteristics)
- [Interoperability](#interoperability)

---

## CPIO Newc Format Overview

### What is CPIO?

CPIO (Copy In/Out) is a Unix archive format for distributing collections of files. The "newc" variant uses ASCII headers for portability across architectures.

### Key Characteristics

- **Header Format**: ASCII hexadecimal (not binary)
- **Header Size**: Exactly 110 bytes (magic + 13 fields × 8 hex chars)
- **Byte Order**: Big-endian (standard hex)
- **Alignment**: 4-byte padding between entries and data
- **Termination**: "TRAILER!!!" entry marks end of archive

### Newc vs Legacy Formats

| Format | Header Bytes | Header Style | Alignment | Usage |
|--------|--------------|--------------|-----------|-------|
| Binary | 26 | Binary | 2 bytes | Old UNIX (obsolete) |
| ASCII (odc) | 76 | ASCII octal | 2 bytes | POSIX (portable) |
| Newc | 110 | ASCII hex | 4 bytes | Linux kernel (modern) |

**LevitateOS Uses**: Newc (Linux expects this for initramfs)

---

## File Type Constants

### POSIX S_IF* Equivalents

```rust
const S_IFDIR: u32 = 0o040000; // Directory (040000 octal)
const S_IFREG: u32 = 0o100000; // Regular file (100000 octal)
const S_IFLNK: u32 = 0o120000; // Symbolic link (120000 octal)
const S_IFCHR: u32 = 0o020000; // Character device (020000 octal)
const S_IFBLK: u32 = 0o060000; // Block device (060000 octal)
```

### Usage Pattern

These constants are OR'd with permission bits to create complete mode field:

```rust
// Example: Regular file with 0755 permissions
mode = S_IFREG | 0o755  // = 0o100755
// Decoded by kernel as: Regular file, rwxr-xr-x

// Example: Directory with 0755 permissions
mode = S_IFDIR | 0o755  // = 0o040755
// Decoded as: Directory, rwxr-xr-x
```

### Rationale for Constants

- Matches kernel's S_IF* macros from `<sys/stat.h>`
- Enables portable archive reading on any POSIX system
- Kernel uses these bits to determine file type during boot

---

## CpioEntry Struct

### Definition

```rust
pub struct CpioEntry {
    pub path: String,           // Entry pathname
    pub mode: u32,              // Type + permissions
    pub data: Vec<u8>,          // File contents (empty for dirs/devices)
    pub nlink: u32,             // Hard link count
    pub dev_major: u32,         // Device major (for dir/file storage)
    pub dev_minor: u32,         // Device minor (for dir/file storage)
    pub rdev_major: u32,        // Device major (for char/block devices)
    pub rdev_minor: u32,        // Device minor (for char/block devices)
}
```

### Field Semantics

| Field | Type | Purpose | Initialized By |
|-------|------|---------|-----------------|
| `path` | String | Entry name in archive (no leading slash) | `new()`, normalized |
| `mode` | u32 | File type bits + permission bits (e.g., 0o100755) | Method-specific |
| `data` | Vec<u8> | File contents (empty for directories) | Entry type method |
| `nlink` | u32 | Hard link count (1 for most, 2 for dirs) | `new()` default 1 |
| `dev_major` | u32 | Device major (unused in newc) | Default 0 |
| `dev_minor` | u32 | Device minor (unused in newc) | Default 0 |
| `rdev_major` | u32 | **Device major for device nodes** | `add_char_device()`, `add_block_device()` |
| `rdev_minor` | u32 | **Device minor for device nodes** | `add_char_device()`, `add_block_device()` |

### Field Usage by Entry Type

| Type | path | mode | data | nlink | rdev_* |
|------|------|------|------|-------|--------|
| Directory | yes | S_IFDIR + perm | empty | 2 | 0 |
| Regular file | yes | S_IFREG + perm | contents | 1 | 0 |
| Symlink | yes | S_IFLNK + 0777 | target string | 1 | 0 |
| Char device | yes | S_IFCHR + perm | empty | 1 | major/minor |
| Block device | yes | S_IFBLK + perm | empty | 1 | major/minor |

### Invariants

1. `path` is always normalized (no leading `/`, empty becomes ".")
2. `data` is populated only for regular files and symlinks
3. For device nodes, `rdev_major` and `rdev_minor` carry device numbers
4. `nlink` is 2 for directories (. and ..), 1 otherwise

### Constructor

```rust
fn new(path: String, mode: u32) -> Self {
    Self {
        path,
        mode,
        data: Vec::new(),
        nlink: 1,              // Default: 1 (overridden for dirs)
        dev_major: 0,
        dev_minor: 0,
        rdev_major: 0,
        rdev_minor: 0,
    }
}
```

**Contract**:
- Private constructor (only used internally by CpioArchive methods)
- Initializes data to empty Vec
- Sets nlink to 1 (callers override for directories)
- All device fields default to 0

---

## CpioArchive Struct

### Definition

```rust
pub struct CpioArchive {
    entries: Vec<CpioEntry>,    // All entries added so far
    next_ino: u32,              // Next inode number to assign
}
```

### Field Semantics

| Field | Type | Purpose | Initial Value |
|-------|------|---------|----------------|
| `entries` | Vec<CpioEntry> | All entries in archive (order preserved) | Empty vec |
| `next_ino` | u32 | Inode counter for next entry | 1 |

### Invariants

1. `entries` maintains insertion order (first added = first in archive)
2. `next_ino` monotonically increases (no ino reuse)
3. Inode numbers are sequential starting from 1
4. TRAILER entry gets ino 0 (special end-of-archive marker)

### Constructor

```rust
pub fn new() -> Self {
    Self {
        entries: Vec::new(),
        next_ino: 1,
    }
}
```

**Contract**:
- Creates empty archive
- Ready for entry addition immediately
- Inode counter starts at 1 (following convention)

### Default Implementation

```rust
impl Default for CpioArchive {
    fn default() -> Self {
        Self::new()
    }
}
```

Allows `CpioArchive::default()` syntax.

---

## Entry Addition Methods

All entry addition methods follow the same pattern:
1. Normalize path
2. Create CpioEntry with type-specific mode and data
3. Append to entries vector

### add_directory

```rust
pub fn add_directory(&mut self, path: &str, mode: u32) {
    let path = normalize_path(path);
    let mut entry = CpioEntry::new(path, S_IFDIR | (mode & 0o7777));
    entry.nlink = 2;  // Directories have . and ..
    self.entries.push(entry);
}
```

**Purpose**: Add directory entry to archive

**Inputs**:
- `path`: Directory path (e.g., "etc", "usr/bin")
- `mode`: Permission bits (e.g., 0o755)

**Process**:
1. Normalize path (remove leading slash)
2. Create entry with S_IFDIR type flag OR'd with permissions
3. Override nlink to 2 (representing . and .. entries)
4. Append to archive

**Guarantees**:
- Directory always has nlink=2
- Permissions respected from `mode` parameter
- No data field populated (directories have no content in CPIO)

**Example**:
```rust
archive.add_directory("etc", 0o755);
// Creates: mode=0o040755 (directory, rwxr-xr-x)
```

### add_file

```rust
pub fn add_file(&mut self, path: &str, data: &[u8], mode: u32) {
    let path = normalize_path(path);
    let mut entry = CpioEntry::new(path, S_IFREG | (mode & 0o7777));
    entry.data = data.to_vec();
    self.entries.push(entry);
}
```

**Purpose**: Add regular file to archive

**Inputs**:
- `path`: File path (e.g., "bin/init")
- `data`: File contents as byte slice
- `mode`: Permission bits (e.g., 0o755 for executable)

**Process**:
1. Normalize path
2. Create entry with S_IFREG type flag
3. Copy data to entry.data (allocates on heap)
4. Append to archive

**Data Copying**:
- `data.to_vec()` creates owned copy
- Allows caller to drop original slice
- Guarantees archive independent from source

**Guarantees**:
- File data preserved exactly as provided
- Permissions respected
- Data stored in memory until write()

**Example**:
```rust
let content = b"#!/bin/sh\necho hello";
archive.add_file("init", content, 0o755);
// Creates: regular file with executable permissions
```

### add_symlink

```rust
pub fn add_symlink(&mut self, path: &str, target: &str) {
    let path = normalize_path(path);
    let mut entry = CpioEntry::new(path, S_IFLNK | 0o777);
    entry.data = target.as_bytes().to_vec();
    self.entries.push(entry);
}
```

**Purpose**: Add symbolic link to archive

**Inputs**:
- `path`: Symlink path (e.g., "bin/sh")
- `target`: Symlink target (e.g., "busybox")

**Process**:
1. Normalize path (link location)
2. Create entry with S_IFLNK type flag
3. Force mode to 0o777 (symlinks always have full perms in CPIO)
4. Store target as entry.data (symlink target is stored as "file content")
5. Append to archive

**Symlink Target Storage**:
- CPIO format stores symlink target in data field
- Not a separate field in header
- Kernel extracts from data and creates actual symlink

**Permissions**:
- Always 0o777 (symlinks don't have traditional permissions)
- Kernel ignores symlink perms on modern systems

**Guarantees**:
- Target preserved as-is (no validation)
- Both absolute and relative targets supported
- Target can be any string (including broken targets)

**Examples**:
```rust
archive.add_symlink("bin/sh", "busybox");       // Relative
archive.add_symlink("bin/ln", "/bin/busybox");  // Absolute
archive.add_symlink("link", "../somewhere");    // Relative traversal
```

### add_char_device

```rust
pub fn add_char_device(&mut self, path: &str, mode: u32, major: u32, minor: u32) {
    let path = normalize_path(path);
    let mut entry = CpioEntry::new(path, S_IFCHR | (mode & 0o7777));
    entry.rdev_major = major;
    entry.rdev_minor = minor;
    self.entries.push(entry);
}
```

**Purpose**: Add character device node (e.g., /dev/null, /dev/ttyS0)

**Inputs**:
- `path`: Device node path (e.g., "dev/null")
- `mode`: Permission bits (e.g., 0o666 for world-readable)
- `major`: Major device number (identifies driver)
- `minor`: Minor device number (identifies device instance)

**Process**:
1. Normalize path
2. Create entry with S_IFCHR type flag
3. Set rdev_major and rdev_minor (actual device numbers)
4. Append to archive

**Device Numbers**:
- Major: Identifies kernel driver (e.g., 4=TTY, 1=memory)
- Minor: Identifies device instance (e.g., ttyS0 vs ttyS1)
- Kernel uses these to call correct driver when device accessed

**Common Device Numbers**:
```
1:0   /dev/mem       (memory device)
1:3   /dev/null      (null device)
1:5   /dev/zero      (zero device)
4:0   /dev/tty0      (console)
5:0   /dev/ttyS0     (serial port)
```

**Guarantees**:
- No data field populated (device content from driver)
- Major/minor numbers stored in archive
- Permissions control access (e.g., 0o660 for restricted devices)

**Example**:
```rust
archive.add_char_device("dev/null", 0o666, 1, 3);
// Creates: character device /dev/null (1:3) readable/writable by all
```

### add_block_device

```rust
pub fn add_block_device(&mut self, path: &str, mode: u32, major: u32, minor: u32) {
    let path = normalize_path(path);
    let mut entry = CpioEntry::new(path, S_IFBLK | (mode & 0o7777));
    entry.rdev_major = major;
    entry.rdev_minor = minor;
    self.entries.push(entry);
}
```

**Purpose**: Add block device node (e.g., /dev/sda, /dev/vda)

**Inputs**:
- `path`: Device node path (e.g., "dev/sda")
- `mode`: Permission bits (e.g., 0o660 for restricted disk access)
- `major`: Major device number (identifies disk controller)
- `minor`: Minor device number (identifies partition/disk)

**Process**:
- Identical to add_char_device except S_IFBLK flag

**Common Block Device Numbers**:
```
8:0   /dev/sda       (first SATA disk)
253:0 /dev/dm-0      (device mapper, LVM)
254:0 /dev/vda       (QEMU virtio block)
```

**Guarantees**:
- Block devices support random access (vs char devices = stream)
- Major/minor identifying disk controllers preserved
- Permissions control who can mount/access disks

**Example**:
```rust
archive.add_block_device("dev/vda", 0o660, 254, 0);
// Creates: block device /dev/vda (virtio disk) with restricted perms
```

### entry_count

```rust
pub fn entry_count(&self) -> usize {
    self.entries.len()
}
```

**Purpose**: Query total entries in archive (before write)

**Returns**: Exact count of entries added so far (excludes TRAILER)

**Usage**: Progress reporting, debugging

**Example**:
```rust
let count = archive.entry_count();
println!("{} entries will be written", count);
```

---

## Archive Writing Behavior

### write Method

```rust
pub fn write<W: Write>(&mut self, mut writer: W) -> std::io::Result<u64> {
    let mut total_bytes = 0u64;

    for entry in &self.entries {
        // ... write header, name, padding, data, data_padding
    }

    // Write TRAILER entry
    // ...

    Ok(total_bytes)
}
```

**Purpose**: Serialize archive to output writer

**Generic Parameter**: `W: Write` accepts any type implementing std::io::Write
- Files (std::fs::File)
- Buffers (Vec<u8>)
- Network streams
- Compression wrappers

**Return Value**: `Result<u64, std::io::Error>`
- Success: Total bytes written to output
- Error: I/O error with context

**Mutability**: Takes `&mut self` because inode counter is incremented during write

### Write Sequence Per Entry

For each entry in `self.entries`:

#### 1. Assign Inode Number

```rust
let ino = self.next_ino;
self.next_ino += 1;
```

- Sequential inode assignment (1, 2, 3, ...)
- Incremented for each entry
- TRAILER gets ino 0 (special marker)

**Why Inode Numbers**:
- CPIO format includes ino field (matches POSIX inodes)
- Kernel doesn't use these for initramfs (modern kernels ignore)
- Preserved for archive portability and tools

#### 2. Calculate Sizes

```rust
let namesize = entry.path.len() + 1;  // +1 for null terminator
let filesize = entry.data.len();
```

- `namesize`: String length + null byte
- `filesize`: Actual data bytes (0 for dirs/devices)

#### 3. Format and Write Header

```rust
let header = format_header(
    ino,
    entry.mode,
    entry.nlink,
    filesize as u32,
    namesize as u32,
    entry.dev_major,
    entry.dev_minor,
    entry.rdev_major,
    entry.rdev_minor,
);
writer.write_all(header.as_bytes())?;
total_bytes += header.len() as u64;
```

- Header is always 110 bytes (6 char magic + 13 fields × 8 hex chars)
- See [Header Format Specification](#header-format-specification)
- Write succeeds or I/O error propagates

#### 4. Write Filename

```rust
writer.write_all(entry.path.as_bytes())?;
writer.write_all(&[0])?;
total_bytes += namesize as u64;
```

- Path written as UTF-8 bytes
- Null terminator appended (required by CPIO spec)
- Total: `entry.path.len() + 1` bytes

#### 5. Pad Header+Name to 4-Byte Boundary

```rust
let header_name_len = 110 + namesize;
let pad = align_to_4(header_name_len) - header_name_len;
if pad > 0 {
    writer.write_all(&vec![0u8; pad])?;
    total_bytes += pad as u64;
}
```

- CPIO spec requires 4-byte alignment
- Padding consists of zero bytes
- Calculation: (header 110 + namesize) padded to multiple of 4

**Alignment Math Example**:
```
Entry: "init" (4 chars)
  header_name_len = 110 + 5 = 115 bytes
  align_to_4(115) = 116
  pad = 116 - 115 = 1 byte of zeros

Entry: "dev/null" (8 chars)
  header_name_len = 110 + 9 = 119 bytes
  align_to_4(119) = 120
  pad = 120 - 119 = 1 byte of zeros
```

#### 6. Write File Data

```rust
if !entry.data.is_empty() {
    writer.write_all(&entry.data)?;
    total_bytes += entry.data.len() as u64;

    let data_pad = align_to_4(filesize) - filesize;
    if data_pad > 0 {
        writer.write_all(&vec![0u8; data_pad])?;
        total_bytes += data_pad as u64;
    }
}
```

- Writes actual file contents (empty for dirs/devices)
- Pads to 4-byte boundary if needed

**Example - 10-Byte File**:
```
data written: 10 bytes
align_to_4(10) = 12
padding: 2 zero bytes
total in archive: 12 bytes
```

**Example - 4-Byte Aligned File**:
```
data written: 12 bytes
align_to_4(12) = 12
padding: 0 bytes
total in archive: 12 bytes
```

#### 7. Entry Complete

Move to next entry, repeat.

### TRAILER Entry Special Handling

After all entries written, TRAILER entry appended:

```rust
let trailer = "TRAILER!!!";
let namesize = trailer.len() + 1;
let header = format_header(0, 0, 1, 0, namesize as u32, 0, 0, 0, 0);
writer.write_all(header.as_bytes())?;
writer.write_all(trailer.as_bytes())?;
writer.write_all(&[0])?;
total_bytes += header.len() as u64 + namesize as u64;

let trailer_len = 110 + namesize;
let pad = align_to_4(trailer_len) - trailer_len;
if pad > 0 {
    writer.write_all(&vec![0u8; pad])?;
    total_bytes += pad as u64;
}
```

**Purpose**: Mark end of archive

**TRAILER Characteristics**:
- Path: "TRAILER!!!" (exactly this string)
- Inode: 0 (special marker)
- Mode: 0 (ignored)
- Size: 0 (no content)
- nlink: 1

**Kernel Behavior**:
- Kernel reads CPIO entries until hitting TRAILER
- TRAILER marks definitive end (even if file is longer)
- Essential for proper initramfs extraction

### Return Value

```rust
Ok(total_bytes)
```

Total bytes written to writer (including all padding and TRAILER).

**Usage**:
```rust
let mut file = File::create("initramfs.cpio")?;
let bytes_written = archive.write(&mut file)?;
println!("Wrote {} bytes", bytes_written);
```

---

## Header Format Specification

### format_header Function

```rust
fn format_header(
    ino: u32,
    mode: u32,
    nlink: u32,
    filesize: u32,
    namesize: u32,
    dev_major: u32,
    dev_minor: u32,
    rdev_major: u32,
    rdev_minor: u32,
) -> String
```

### Output Format

```
"070701" + 13 fields (each 8 hex chars) = 110 bytes exactly

070701 XXXXXXXX XXXXXXXX XXXXXXXX ... (13 times)
      └─ Magic: "070701" (6 chars) = newc format identifier
```

### Header Fields (In Order)

| # | Name | Bytes | Hex Chars | Purpose | Example |
|---|------|-------|-----------|---------|---------|
| 1 | Magic | 6 | "070701" | Format identifier | 070701 |
| 2 | c_ino | 8 | 8 hex | Inode number | 00000001 |
| 3 | c_mode | 8 | 8 hex | Type + permissions | 00100755 |
| 4 | c_uid | 8 | 00000000 | User ID (always 0=root) | 00000000 |
| 5 | c_gid | 8 | 00000000 | Group ID (always 0=root) | 00000000 |
| 6 | c_nlink | 8 | 8 hex | Link count | 00000001 |
| 7 | c_mtime | 8 | 00000000 | Modification time (always 0) | 00000000 |
| 8 | c_filesize | 8 | 8 hex | File size in bytes | 0000000D |
| 9 | c_devmajor | 8 | 00000000 | Device major (unused) | 00000000 |
| 10 | c_devminor | 8 | 00000000 | Device minor (unused) | 00000000 |
| 11 | c_rdevmajor | 8 | 8 hex | Rdev major (for devices) | 00000001 |
| 12 | c_rdevminor | 8 | 8 hex | Rdev minor (for devices) | 00000003 |
| 13 | c_namesize | 8 | 8 hex | Path + null byte size | 00000005 |
| 14 | c_check | 8 | 00000000 | Checksum (always 0 for newc) | 00000000 |

### Format String (Rust code)

```rust
format!(
    "070701\
     {:08X}\        // c_ino
     {:08X}\        // c_mode
     {:08X}\        // c_uid
     {:08X}\        // c_gid
     {:08X}\        // c_nlink
     {:08X}\        // c_mtime
     {:08X}\        // c_filesize
     {:08X}\        // c_devmajor
     {:08X}\        // c_devminor
     {:08X}\        // c_rdevmajor
     {:08X}\        // c_rdevminor
     {:08X}\        // c_namesize
     {:08X}",       // c_check
    ino, mode, 0, 0, nlink, 0, filesize, dev_major, dev_minor,
    rdev_major, rdev_minor, namesize, 0
)
```

### Fixed Values

- `c_uid`: Always 0 (root)
- `c_gid`: Always 0 (root)
- `c_mtime`: Always 0 (no timestamp info preserved)
- `c_devmajor`: Always 0 (unused in newc)
- `c_devminor`: Always 0 (unused in newc)
- `c_check`: Always 0 (checksum not used in newc)

**Rationale**: Initramfs is ephemeral (booted once), so timestamps unnecessary. Root ownership simplifies permissions.

### Example Header Bytes

```
Entry: directory "etc" with mode 0o40755

Header string:
  070701 00000001 00040755 00000000 00000000 00000002 00000000 00000000 00000000 00000000 00000000 00000000 00000004 00000000

Decoded:
  070701     = newc format
  00000001   = ino 1
  00040755   = mode 0o40755 (dir + rwxr-xr-x)
  00000000   = uid 0 (root)
  00000000   = gid 0 (root)
  00000002   = nlink 2 (directory has . and ..)
  00000000   = mtime 0
  00000000   = filesize 0 (dir has no content)
  00000000   = devmajor 0
  00000000   = devminor 0
  00000000   = rdevmajor 0 (not a device)
  00000000   = rdevminor 0
  00000004   = namesize 4 (includes null byte, "etc\0")
  00000000   = checksum 0
```

---

## Path Normalization

### normalize_path Function

```rust
fn normalize_path(path: &str) -> String {
    let path = path.trim_start_matches('/');
    if path.is_empty() {
        ".".to_string()
    } else {
        path.to_string()
    }
}
```

**Purpose**: Ensure consistent path format in CPIO archive

**Transformation**:

| Input | Output | Reason |
|-------|--------|--------|
| "/etc/init" | "etc/init" | Leading slash removed |
| "etc/init" | "etc/init" | Already normalized |
| "/" | "." | Root becomes current dir |
| "" | "." | Empty becomes current dir |
| "///etc" | "etc" | Multiple slashes trimmed |

### Rationale

CPIO archives expect relative paths without leading slashes:
- Allows archives to be extracted anywhere (portable)
- Kernel expects relative paths in initramfs
- `.` represents current directory (archive root)

### Invariant

All paths in `CpioEntry.path` are guaranteed normalized (no leading `/`, empty converted to `.`)

---

## Alignment Strategy

### align_to_4 Function

```rust
fn align_to_4(n: usize) -> usize {
    (n + 3) & !3
}
```

**Purpose**: Round up value to next 4-byte boundary

**Bitwise Logic**:
- `(n + 3)`: Add 3 (maximum padding needed)
- `& !3`: Bitmask 0xFFFFFFFC (clears low 2 bits)
- Result: Multiple of 4

**Examples**:

| Input | +3 | Binary | & !3 | Result |
|-------|----|---------| ----|----|
| 0 | 3 | 00000011 | & 11111100 | 0 |
| 1 | 4 | 00000100 | & 11111100 | 4 |
| 2 | 5 | 00000101 | & 11111100 | 4 |
| 3 | 6 | 00000110 | & 11111100 | 4 |
| 4 | 7 | 00000111 | & 11111100 | 4 |
| 5 | 8 | 00001000 | & 11111100 | 8 |

### When Applied

1. **After header+name**: Pad so next data starts 4-byte aligned
2. **After file data**: Pad so next entry starts 4-byte aligned
3. **After TRAILER**: Pad final entry to alignment (for completeness)

### Justification

- **Performance**: CPU caches align to 4/8/16 bytes; alignment reduces cache misses
- **Architectures**: Some ARM chips require aligned access
- **Specification**: CPIO newc format mandates 4-byte alignment

### Example Archive Layout

```
Bytes:  0-109      header (110 bytes)
        110-112    "etc\0" (4 bytes)
        113-119    padding (7 bytes) to align next to 120
        120-149    directory data (30 bytes) if any
        150-259    next entry header (110 bytes)
        ...
```

---

## Trailer Handling

### Why TRAILER?

CPIO format requires sentinel entry to mark archive end:
- Kernel reads entries until hitting TRAILER
- Allows archive to be embedded in larger file (kernel ignores content after TRAILER)
- Portable marker (string-based, not magic number)

### TRAILER Entry Specification

```rust
let trailer = "TRAILER!!!";  // Exactly this string (10 chars)
let namesize = 11;           // 10 + null terminator
let header = format_header(0, 0, 1, 0, 11, 0, 0, 0, 0);

// Header with all zeros except:
// ino = 0 (special marker)
// nlink = 1
// namesize = 11
```

### TRAILER Bytes Written

| Field | Value | Bytes |
|-------|-------|-------|
| Header | "070701..." | 110 |
| Filename | "TRAILER!!!" | 10 |
| Null term | \0 | 1 |
| Padding | 0x00 bytes | ? (variable) |
| **Total** | | 121+ |

### Padding After TRAILER

```rust
let trailer_len = 110 + namesize;  // 110 + 11 = 121
let pad = align_to_4(trailer_len) - trailer_len;  // align_to_4(121) = 124, pad = 3
if pad > 0 {
    writer.write_all(&vec![0u8; pad])?;
}
```

Pads to next 4-byte boundary (124 bytes total).

### Kernel Extraction

When kernel extracts initramfs:

```c
// Pseudocode
while (read_cpio_entry(&entry)) {
    if (strcmp(entry.name, "TRAILER!!!") == 0) {
        break;  // Stop extracting
    }
    extract_file(&entry);
}
```

---

## State Management

### Mutable State During write()

```rust
pub fn write<W: Write>(&mut self, mut writer: W) -> std::io::Result<u64> {
    let mut total_bytes = 0u64;

    for entry in &self.entries {
        let ino = self.next_ino;
        self.next_ino += 1;  // Increment inode counter
        // ...
    }
    Ok(total_bytes)
}
```

**Mutation**: `next_ino` incremented for each entry

**Why Mutable**:
- Inode assignment during write (not pre-computed)
- Allows same archive to be written multiple times with fresh inode numbering

### Multiple Writes

```rust
let mut archive = CpioArchive::new();
archive.add_file("file1", b"content1", 0o644);
archive.add_file("file2", b"content2", 0o644);

let mut output1 = Vec::new();
archive.write(&mut output1)?;  // Inode counter: 1, 2

let mut output2 = Vec::new();
archive.write(&mut output2)?;  // Inode counter: 1, 2 (fresh)
```

**Behavior**: Each write starts from ino 1 (counter reset? No - counter continues: 3, 4)

**Wait, Check Code**:
```rust
for entry in &self.entries {
    let ino = self.next_ino;
    self.next_ino += 1;
```

Actually `next_ino` **continues incrementing** across multiple writes (3, 4, 5, ... for second write).

**Implication**: Archive should typically be written once. Multiple writes to same archive produce different inode numbers.

### Entries Immutability

```rust
entries: Vec<CpioEntry>  // Private, immutable after addition
```

Entries cannot be modified after addition (no public methods to change entries).

---

## Error Handling

### Error Type

```rust
pub fn write<W: Write>(&mut self, mut writer: W) -> std::io::Result<u64>
```

Returns `std::io::Result<u64>` (not `Result<u64, anyhow::Error>`)

### Error Sources

| Source | When | How |
|--------|------|-----|
| write_all() | Disk full | I/O error from OS |
| write_all() | Permission denied | I/O error from OS |
| write_all() | Closed pipe | I/O error from OS |

### Error Propagation

```rust
writer.write_all(header.as_bytes())?;  // Propagates I/O error
```

All I/O operations use `?` operator (early return on error).

**Effect**: Single I/O error aborts write immediately (no partial recovery).

### No Validation Errors

Entry addition methods never fail:
- `add_file()` returns `()` (can't fail)
- `add_directory()` returns `()` (can't fail)
- `add_symlink()` returns `()` (can't fail)

Errors only occur during `write()` (I/O phase).

---

## Testing Behaviors

### Test Coverage

Located in `#[cfg(test)]` module at end of file.

#### test_header_format

```rust
#[test]
fn test_header_format() {
    let header = format_header(1, 0o100755, 1, 100, 5, 0, 0, 0, 0);
    assert_eq!(header.len(), 110);
    assert!(header.starts_with("070701"));
}
```

**Verifies**:
- Header always exactly 110 bytes
- Magic "070701" present
- format_header is deterministic

#### test_normalize_path

```rust
#[test]
fn test_normalize_path() {
    assert_eq!(normalize_path("/bin/ls"), "bin/ls");
    assert_eq!(normalize_path("bin/ls"), "bin/ls");
    assert_eq!(normalize_path("/"), ".");
    assert_eq!(normalize_path(""), ".");
}
```

**Verifies**:
- Leading slash removal
- Empty path becomes "."
- Idempotent (normalizing twice = same result)

#### test_align_to_4

```rust
#[test]
fn test_align_to_4() {
    assert_eq!(align_to_4(0), 0);
    assert_eq!(align_to_4(1), 4);
    assert_eq!(align_to_4(2), 4);
    assert_eq!(align_to_4(3), 4);
    assert_eq!(align_to_4(4), 4);
    assert_eq!(align_to_4(5), 8);
}
```

**Verifies**:
- Correct rounding to 4-byte boundary
- Multiples of 4 unchanged
- Edge cases (0, 1, 5)

#### test_basic_archive

```rust
#[test]
fn test_basic_archive() {
    let mut archive = CpioArchive::new();
    archive.add_directory("bin", 0o755);
    archive.add_file("bin/hello", b"Hello, World!", 0o755);
    archive.add_symlink("bin/hi", "hello");

    let mut output = Vec::new();
    let bytes = archive.write(&mut output).unwrap();
    assert!(bytes > 0);
    assert!(output.starts_with(b"070701"));
}
```

**Verifies**:
- End-to-end archive creation
- Multiple entry types
- Output begins with magic
- Positive bytes written

### Test Coverage Gaps

Not tested:
- Device node creation (add_char_device, add_block_device)
- Large files (memory/streaming behavior)
- Invalid characters in paths
- Concurrent access (Send/Sync)
- Archive readback validation

---

## Performance Characteristics

### Time Complexity

- **add_directory**: O(1) (append to vec)
- **add_file**: O(n) where n = file size (copy to vec)
- **add_symlink**: O(m) where m = target length
- **add_char_device**: O(1)
- **add_block_device**: O(1)
- **write()**: O(n) where n = total archive size

### Overall: O(n) linear in total data size

### Space Complexity

- **Storage**: O(n) where n = sum of all data
- **Entry metadata**: O(e) where e = number of entries (negligible)

### Memory Profile

- Entire archive buffered in RAM before write
- For 50MB initramfs: ~50MB heap allocation
- No streaming (write_all() batches all at once per method call)

### Performance Optimizations

1. **Pre-allocated Vec**: Entries vector grows as needed (amortized O(1))
2. **No validation**: No checks before write (validation happens elsewhere)
3. **No compression**: Raw CPIO written (gzip applied externally)
4. **Inode assignment**: Sequential, no lookup overhead

---

## Interoperability

### Archive Reading

Archives created by this code can be read by:
- Linux kernel (extracts during boot)
- Standard CPIO tools: `cpio -i -H newc -F archive.cpio`
- GNU tar: `tar -C extract --cpio -xf archive.cpio`
- Rust cpio crates (3rd party)

### Kernel Compatibility

- **newc format**: All modern Linux kernels support
- **Alignment**: Kernels expect 4-byte alignment (required)
- **TRAILER**: Kernel looks for "TRAILER!!!" to stop reading
- **Root ownership**: Kernel doesn't care about uid/gid (all becomes root)
- **Timestamps**: Kernel ignores mtime (ephemeral filesystem)

### Archive Inspection

```bash
# List contents
cpio -i -H newc -t -F archive.cpio < /dev/null

# Extract
cpio -i -H newc -d -F archive.cpio < /dev/null

# Inspect binary
od -A x -t x1z -v archive.cpio | head -20
```

### Cross-Platform Usage

Archives are portable across:
- x86_64, aarch64, ARM, etc. (ASCII header format)
- Windows (CPIO is text-based format)
- Old systems (newc format standardized since ~1990s)

---

## Known Limitations

### No Incremental Packing

- Entire archive kept in memory until write
- No streaming/partial writes
- Large archives may consume significant RAM

### No Compression

- Archives written uncompressed
- gzip compression applied externally by builder
- CPIO itself is raw binary/text format

### No Optimization

- No deduplication of identical files
- No content-addressing (same content = new entry)
- Possible optimization: detect duplicates and use hard links

### Limited Validation

- No checks for path traversal (e.g., "../../../etc/passwd")
- No enforcement of unique paths
- No symlink target validation

### Timestamps Always Zero

- mtime field always 0
- No way to preserve file timestamps
- Acceptable for ephemeral initramfs

---

## Design Decisions

| Decision | Rationale | Trade-off |
|----------|-----------|-----------|
| Pure Rust implementation | Eliminate shell script dependency | Minor performance vs shell tools |
| Event-driven builder (different module) | Decouple archive format from CLI | Slightly more code |
| newc format | Linux standard for initramfs | Less portable than odc format |
| 4-byte alignment | CPU performance + architecture support | Larger archive vs raw binary |
| ASCII headers | Portable (text-based) | Slightly larger headers (110 vs 26 bytes binary) |
| Root-owned (uid/gid=0) | Simplifies boot process | All files owned by root (acceptable) |
| No timestamps (mtime=0) | Initramfs is ephemeral | Can't preserve timestamps |
| Sequential inode assignment | Simple, deterministic | Not unique across multiple builds |
| fail-fast write | Catch I/O errors early | No partial recovery |

---

## Related Modules

| Module | Interaction | Purpose |
|--------|-------------|---------|
| `super::builder` | Uses CpioArchive | Feeds entries to archive |
| `super::manifest` | Uses path/file data | Manifest defines entries |
| `std::io` | Uses Write trait | Output abstraction |
| `src/main.rs` | Calls through builder | CLI entry point |
| Tests: `src/tests/behavior.rs` | Verifies output | Golden file testing |

---

## Future Enhancements

### Potential Improvements

1. **Streaming API**: `write_entry()` instead of buffering all
2. **Compression Integration**: gzip streaming directly
3. **Deduplication**: Detect identical files, use hard links
4. **Timestamp Preservation**: Add mtime from source files
5. **Path Validation**: Check for traversal, duplicates, invalid chars
6. **CPIO Format Options**: Support odc format, newc checksum variants

### Compatibility Concerns

- Any changes must preserve kernel extraction
- newc format fixed (can't change headers)
- TRAILER sentinel required (can't remove)
- 4-byte alignment assumed (can't change padding)

