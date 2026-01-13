# Initramfs Builder Behaviors

**File**: `src/builder/initramfs/builder.rs`

**Purpose**: Constructs a CPIO archive from a declarative manifest, converting structured filesystem specifications into bootable initramfs images.

**Event-Driven Pattern**: Uses `BuildEvent` enumeration to emit progress updates during construction, enabling TUI status reporting without coupling builder to display logic.

---

## Table of Contents

- [BuildEvent Enumeration](#buildevent-enumeration)
- [InitramfsBuilder Struct](#initramfsbuilder-struct)
- [Constructor Behaviors](#constructor-behaviors)
- [Build Phases](#build-phases)
- [Event Emission Behaviors](#event-emission-behaviors)
- [Error Handling](#error-handling)
- [Special Cases](#special-cases)
- [Directory Tree Copy Algorithm](#directory-tree-copy-algorithm)

---

## BuildEvent Enumeration

### Definition

```rust
pub enum BuildEvent {
    PhaseStart { name: &'static str, total: usize },
    PhaseComplete { name: &'static str },
    DirectoryCreated { path: String },
    BinaryAdded { path: String, size: u64 },
    SymlinkCreated { link: String, target: String },
    FileAdded { path: String, size: u64 },
    DeviceCreated { path: String },
    TreeCopied { dest: String, files: usize },
    BuildComplete {
        output_path: PathBuf,
        total_size: u64,
        duration: Duration,
    },
    BuildFailed { error: String },
}
```

### Event Semantics

| Event | When Emitted | Purpose | Field Meaning |
|-------|--------------|---------|---------------|
| `PhaseStart` | Before starting a build phase | Signal progress tracking initiation | `total`: expected count of items in phase |
| `PhaseComplete` | After completing a phase | Signal phase finished | Pairs with `PhaseStart` with same `name` |
| `DirectoryCreated` | For each new directory added | Track directory creation | `path`: absolute path in archive (e.g., "etc") |
| `BinaryAdded` | For each executable binary added | Track binary inclusion | `size`: file size in bytes |
| `SymlinkCreated` | For each symbolic link created | Track symlink creation | `link`: source path, `target`: destination |
| `FileAdded` | For each regular file added | Track file inclusion | `size`: file size in bytes |
| `DeviceCreated` | For each device node created | Track device creation | Device nodes (char/block devices) |
| `TreeCopied` | After directory tree copy completes | Track tree recursion | `files`: count of items copied |
| `BuildComplete` | After archive write succeeds | Completion notification | `total_size`: final archive bytes, `duration`: elapsed time |
| `BuildFailed` | Never actually emitted by builder | Placeholder event | For consumer error handling |

### #[allow(dead_code)] Rationale

The `#[allow(dead_code)]` attribute on `BuildEvent` exists because:
- Fields like `name`, `total`, `size` are read via pattern matching by TUI consumers
- Static analysis sees fields read only in match arms external to this module
- The attribute prevents spurious dead code warnings for legitimate event consumers

---

## InitramfsBuilder Struct

### Fields

```rust
pub struct InitramfsBuilder {
    manifest: Manifest,        // Declarative spec of filesystem
    arch: String,              // "x86_64" or "aarch64"
    base_dir: PathBuf,         // Project root directory
}
```

### Field Semantics

| Field | Type | Purpose | Example |
|-------|------|---------|---------|
| `manifest` | `Manifest` | Parsed manifest spec defining layout, binaries, files, devices | From `initramfs/initramfs.toml` |
| `arch` | `String` | Target architecture for path resolution | "x86_64", "aarch64" |
| `base_dir` | `PathBuf` | Project root for resolving relative file paths | "/home/vince/Projects/LevitateOS" |

### Invariants

- `base_dir` must be the project root (used as base for `base_dir/files/` lookups)
- `arch` must match one of the supported architectures
- `manifest` must be pre-validated before builder creation

---

## Constructor Behaviors

### `new(manifest: Manifest, arch: &str, base_dir: &Path) -> Self`

**Purpose**: Create an InitramfsBuilder instance.

**Inputs**:
- `manifest`: Pre-parsed `Manifest` struct from TOML
- `arch`: String slice "x86_64" or "aarch64"
- `base_dir`: Path to project root

**Outputs**: Initialized `InitramfsBuilder` struct

**Guarantees**:
- Constructor always succeeds (no Result type)
- Creates owned copies of String and PathBuf (no lifetime coupling)
- Does not validate inputs (validation should be pre-done on Manifest)

**Side Effects**: None

---

## Build Phases

### Overall Sequence

The builder executes 7 sequential phases:

1. **Create Directories** → `archive.add_directory()`
2. **Add Binaries** → `archive.add_file()`
3. **Create Symlinks** → `archive.add_symlink()`
4. **Add Files** → Regular files + script files + /etc/motd
5. **Create Device Nodes** → `archive.add_char_device()` or `archive.add_block_device()`
6. **Copy Directory Trees** → `copy_tree()` recursive descent
7. **Write Archive** → `archive.write()` to CPIO file

Each phase emits `PhaseStart` before and `PhaseComplete` after.

---

## Phase 1: Create Directories

**Trigger**: `build_with_events()` entry point

**Input**: `self.manifest.layout.directories` (from manifest)

**Process**:
```
for each directory in manifest:
    archive.add_directory(dir, mode=0o755)
    emit DirectoryCreated event
```

**Auto-Created Directories** (implicit, not in manifest):
- `/lib` (musl dynamic linking)
- `/usr`
- `/usr/lib` (musl library location)

**Purpose**:
- `/lib`, `/usr/lib` are added unconditionally for musl compatibility
- Other directories come from manifest layout section

**Guarantees**:
- All directories created with `0o755` permissions (readable by all)
- Symlink `/usr/lib -> ../lib` created later (phase 3)
- Directory creation is idempotent in CPIO format (overwrites create no conflict)

---

## Phase 2: Add Binaries

**Trigger**: After directory creation phase

**Input**: `self.manifest.binaries` HashMap of executable names to specs

**Filter**: Only includes binaries where `b.source.is_empty() == false`
- Empty source means "build from source externally", skip these entries

**Process**:
```
for each binary in manifest (filtered):
    source_path = PathBuf::from(binary.source)
    data = read_file(source_path)  // Can fail here
    mode = parse_mode(binary.mode)
    archive.add_file(binary.dest, data, mode)
    emit BinaryAdded event

    // Special case: if binary name is "busybox"
    if name == "busybox":
        archive.add_file("/init", data, mode)  // Duplicate entry
        emit BinaryAdded for "/init"
```

**Special Case: Busybox Duplication**

When the binary is named "busybox":

1. Added to its manifest destination (e.g., `/bin/busybox`)
2. **Also** added as `/init` as direct file copy

**Rationale** (from code comment):
```
// Copy busybox to /init for custom kernel compatibility
// (Custom LevitateOS kernel can't follow symlinks for init)
// Linux works with either symlink or file, so file is safe for both
```

This handles a quirk where custom kernels couldn't resolve init through symlinks, but the stock Linux kernel doesn't have this limitation. Using a direct file copy is safe for both cases.

**Error Handling**:
- If source file doesn't exist: Returns error with formatted message including binary name and path
- If read fails: Wrapped with context "Failed to read binary '...':"

**Guarantees**:
- Binaries are always readable (added as regular files, not symbolic links)
- File permissions preserved from `binary.mode` field
- Busybox always gets duplicated to `/init` if present

**Side Effects**: None (all writes go to archive, not filesystem)

---

## Phase 3: Create Symlinks

**Trigger**: After binary addition phase

**Input**: `self.manifest.symlinks` HashMap mapping link -> target paths

**Process**:
```
for each (link, target) in symlinks:
    archive.add_symlink(link, target)
    emit SymlinkCreated event

// Auto-created symlink:
archive.add_symlink("usr/lib", "../lib")
```

**Auto-Created Symlinks**:
- `/usr/lib -> ../lib`: Enables library path compatibility (some software expects `usr/lib`, others expect `lib`)

**Guarantees**:
- Symlinks created in CPIO format (not on filesystem)
- Relative symlinks preserved as-is (e.g., `../lib`)
- Auto-created symlink always present regardless of manifest

---

## Phase 4: Add Files

**Trigger**: After symlink creation phase

**Input**:
- `self.manifest.files`: Regular config/data files
- `self.manifest.scripts`: Executable scripts
- Hardcoded `/etc/motd` file

**Process**:

### Regular Files
```
for each (dest, entry) in files:
    match entry:
        FromFile { source, mode }:
            source_path = base_dir / "files" / source
            data = read_file(source_path)
            mode = parse_mode(mode)
        Inline { content, mode }:
            data = content.as_bytes()
            mode = parse_mode(mode)

    archive.add_file(dest, data, mode)
    emit FileAdded event
```

### Script Files
```
for each (dest, entry) in scripts:
    match entry:
        FromFile { source, mode }:
            source_path = base_dir / source  // Note: no "files/" prefix
            data = read_file(source_path)
            mode = parse_mode(mode)
        Inline { content, mode }:
            data = content.as_bytes()
            mode = parse_mode(mode)

    archive.add_file(dest, data, mode)
    emit FileAdded event
```

### MOTD
```
motd = b"Welcome to LevitateOS!\n"
archive.add_file("etc/motd", motd, 0o644)
emit FileAdded event
```

**Path Resolution**:
- Regular files: `base_dir/files/{source}`
- Scripts: `base_dir/{source}` (no subdirectory)
- Allows scripts to be at project root or specific directories

**File Entry Variants**:

| Variant | Purpose | Use Case |
|---------|---------|----------|
| `FromFile { source, mode }` | Reference external file | Config files, init scripts, large data |
| `Inline { content, mode }` | Inline as string | Small configs, shebang scripts |

**Permissions**:
- Parsed from `mode` field (e.g., "0644", "0755")
- Applied to both FromFile and Inline variants

**Error Handling**:
- Missing file: Error message includes file path and type (file vs script)
- Permission parse errors: Handled by `parse_mode()` function

**Guarantees**:
- All files added in phase 4, not earlier
- MOTD always created with content "Welcome to LevitateOS!\n"
- Script file permissions respected (e.g., executable if mode=0755)

---

## Phase 5: Create Device Nodes

**Trigger**: After files added phase

**Input**: `self.manifest.devices` HashMap of device path -> device specs

**Process**:
```
for each (path, device) in devices:
    mode = parse_mode(device.mode)
    if device.dev_type == "c":
        archive.add_char_device(path, mode, device.major, device.minor)
    elif device.dev_type == "b":
        archive.add_block_device(path, mode, device.major, device.minor)
    else:
        skip (no-op)

    emit DeviceCreated event
```

**Device Types**:
- `"c"`: Character device (e.g., `/dev/tty`, `/dev/null`)
- `"b"`: Block device (e.g., `/dev/sda`, `/dev/vda`)
- Other: Silently ignored (no error)

**Device Node Fields**:
```
pub struct Device {
    pub dev_type: String,    // "c" or "b"
    pub mode: String,        // e.g., "0o666"
    pub major: u32,          // Major device number
    pub minor: u32,          // Minor device number
}
```

**Guarantees**:
- Device nodes created in CPIO format (not actual `/dev/` entries)
- Permissions applied from `mode` field
- Major/minor numbers stored for kernel udev/devtmpfs later

---

## Phase 6: Copy Directory Trees

**Trigger**: After devices created phase, only if trees not empty

**Input**: `self.manifest.trees` HashMap of destination -> source paths

**Process**:
```
if trees.is_empty():
    skip phase (emit no events)
else:
    emit PhaseStart
    for each (dest, source) in trees:
        source_path = PathBuf::from(source)
        if source_path.exists():
            files_count = copy_tree(archive, source_path, dest)
            emit TreeCopied event
    emit PhaseComplete
```

**Conditional Execution**:
- Phase completely skipped if no trees in manifest (no events emitted)
- Source existence checked before copying (missing sources don't error, silently skip)

**Recursive Behavior**: Handled by `copy_tree()` helper function (see below)

---

## Phase 7: Write Archive

**Trigger**: After all content phases complete

**Process**:
```
output_dir = PathBuf::from("target/initramfs")
create_dir_all(output_dir)  // Ensures directory exists

output_path = output_dir / "{arch}.cpio"
// e.g., "target/initramfs/x86_64.cpio"

file = File::create(output_path)
total_size = archive.write(file)

duration = Instant.elapsed()
emit BuildComplete event
```

**Output Path**:
- Always: `target/initramfs/{arch}.cpio`
- Architecture-specific to prevent overwrites
- Example: `target/initramfs/x86_64.cpio`

**Size Reporting**:
- `total_size`: Bytes written to CPIO file
- Reported in `BuildComplete` event
- Used for progress display and logging

**Timing**:
- Elapsed duration includes all 7 phases
- Only checked at archive write completion
- Reports wall-clock time, not CPU time

**Guarantees**:
- Output directory created if missing
- File overwrite allowed (no version checking)
- Duration always non-negative

---

## Event Emission Behaviors

### Pattern: Event-Driven Progress

The builder uses a callback-based pattern for event emission:

```rust
pub fn build_with_events<F>(&self, emit: F) -> Result<PathBuf>
where
    F: Fn(BuildEvent),
```

**Callback Contract**:
- `emit` is a closure that receives `BuildEvent`
- Consumer responsible for handling (e.g., TUI rendering, logging)
- No error propagation from `emit()` (fire-and-forget)

### Event Ordering Guarantees

Within each phase:
1. `PhaseStart` emitted once at phase entry (before loop)
2. Multiple item-specific events emitted in loop order
3. `PhaseComplete` emitted once at phase exit (after loop)

**Across phases**: Strict sequential execution (phase N+1 starts after phase N completes)

### Event Reliability

- All events emitted at least once during normal build
- Events never duplicated (except intentional: Busybox adds two `BinaryAdded` events)
- Build failures before completion never emit `BuildComplete`

---

## Error Handling

### Error Sources

| Source | Triggered By | Error Message |
|--------|--------------|---------------|
| Binary read failure | Missing binary source file | "Failed to read binary '{name}': {path}" |
| Binary file I/O | File system errors during read | Error context from `with_context()` |
| Regular file read | Missing file from `files/` dir | "Failed to read file: {path}" |
| Script file read | Missing script from base_dir | "Failed to read script: {path}" |
| Directory tree copy | Missing tree source | Silently skipped (no error) |
| Output dir creation | Permission issues | Error from `create_dir_all()` |
| Archive write | Disk full, permission denied | Error from `archive.write()` |

### Error Propagation

```rust
pub fn build_with_events<F>(&self, emit: F) -> Result<PathBuf>
```

- Returns `Result<PathBuf>` (success path: output archive path)
- Any error in build cancels operation immediately (fail-fast)
- Partial archives not cleaned up on error

### Recovery Strategy

None built-in. Caller must:
1. Handle `Err` result
2. Check `target/initramfs/` for partial files
3. Retry after fixing issue (e.g., missing source file)

---

## Special Cases

### Busybox Initialization

**Behavior**: Busybox binary added twice:
1. To manifest-specified destination (typically `/bin/busybox`)
2. To `/init` as exact duplicate

**Why Duplication**:
- Custom LevitateOS kernel expects `/init` as regular file, not symlink
- Stock Linux kernel resolves symlinks fine
- File copy supports both cases (symlink is stricter)

**Code**:
```rust
if *name == "busybox" {
    archive.add_file("init", &data, mode);
    emit(BuildEvent::BinaryAdded {
        path: "/init".to_string(),
        size,
    });
}
```

**Guarantees**:
- Busybox always present at `/init` if "busybox" entry exists in manifest
- Both files have identical content and permissions
- No other binaries duplicate to `/init`

### Implicit Library Paths

**Auto-Created Directories**:
```rust
archive.add_directory("lib", 0o755);
archive.add_directory("usr", 0o755);
archive.add_directory("usr/lib", 0o755);
```

**Auto-Created Symlink**:
```rust
archive.add_symlink("usr/lib", "../lib");
```

**Purpose**: Ensure musl dynamic linker can find libraries regardless of search path

**Guarantees**:
- Always created, regardless of manifest
- Cannot be overridden by manifest declarations
- Symlink relative (works in any initramfs location)

### MOTD Hardcoding

**Content**: `"Welcome to LevitateOS!\n"`

**Always** created at `/etc/motd` with permissions `0o644`

**Rationale**: Provides user feedback that system initialized

**Guarantees**:
- Always exactly this message
- Cannot be customized via manifest
- Always readable by all users

### Empty Tree Handling

**Behavior**: If `self.manifest.trees.is_empty()`:
- Skip entire phase 6 (no events emitted)
- No error if trees missing

**Code**:
```rust
let trees = &self.manifest.trees;
if !trees.is_empty() {
    emit(BuildEvent::PhaseStart { ... });
    // ... copy logic
    emit(BuildEvent::PhaseComplete { ... });
}
```

**Guarantees**:
- No events emitted if trees section absent from manifest
- Phase 6 completely skipped (efficient for manifests without trees)

---

## Directory Tree Copy Algorithm

### Function Signature

```rust
fn copy_tree(
    &self,
    archive: &mut CpioArchive,
    src: &Path,
    dest_prefix: &str
) -> Result<usize>
```

### Purpose

Recursively copy directory structure from filesystem into CPIO archive.

### Algorithm

```
copy_tree(src, dest_prefix):
    count = 0
    for each entry in read_dir(src):
        path = entry.path()
        name = path.file_name()
        dest = "{dest_prefix}/{name}"

        if path is symlink:
            target = read_link(path)
            archive.add_symlink(dest, target)
            count += 1
        else if path is directory:
            archive.add_directory(dest, mode=0o755)
            count += copy_tree(archive, path, dest)
        else if path is regular file:
            data = read_file(path)
            mode = get_permissions(path)
            archive.add_file(dest, data, mode)
            count += 1

    return count
```

### Recursion

- Descends into subdirectories (depth-first)
- Builds destination paths via string concatenation
- Counts total items added (for progress reporting)

### Permission Handling

```rust
let mode = std::fs::metadata(&path)?.permissions().mode() & 0o7777;
```

- Reads permission bits from source file
- Masks to permission bits only (strips file type bits)
- Preserves executable bit for copied files

### Symlink Handling

```rust
if path.is_symlink() {
    let target = std::fs::read_link(&path)?;
    archive.add_symlink(&dest, &target.to_string_lossy());
    count += 1;
}
```

- Reads link target from filesystem
- Recreates as symlink in archive (preserves relative links)
- Counted as 1 item

### Error Handling

- I/O errors propagate (read_dir, read_file, metadata all Result types)
- No partial recovery (fails on first error)

### Performance

- Linear in tree size (visits each file once)
- Single disk read pass for all files
- String allocations per path traversal

### Guarantees

- All files added to archive (no filtering)
- Symlinks preserved as symlinks
- Directory structure mirrors source
- Permissions preserved from source filesystem

### Example Invocation

```
copy_tree(archive, Path::new("/opt/bin"), "bin")

Source filesystem:
  /opt/bin/
    ├── app1      (file, mode 0o755)
    ├── app2      (file, mode 0o755)
    ├── lib/      (directory)
    │   ├── lib.so (file, mode 0o644)
    │   └── helper (symlink -> /lib/helper.so)
    └── README    (file, mode 0o644)

Result in archive:
  bin/
    ├── app1      (file, 0o755)
    ├── app2      (file, 0o755)
    ├── lib/      (directory, 0o755)
    │   ├── lib.so (file, 0o644)
    │   └── helper (symlink -> /lib/helper.so)
    └── README    (file, 0o644)

Returns: usize = 6 (5 files/symlinks + 1 directory)
```

---

## State Management

### Mutable State During Build

```rust
pub fn build_with_events<F>(&self, emit: F) -> Result<PathBuf>
where
    F: Fn(BuildEvent),
{
    let start = Instant::now();
    let mut archive = CpioArchive::new();

    // ... 7 phases modify archive
    // ... no other mutable state
}
```

### Archive State

- Created empty at function start
- Modified by all 7 phases
- Written to disk once at end
- Never persisted between calls

### Timing State

- Start time recorded at entry
- Duration calculated at exit only
- No per-phase timing tracked

### Guarantees

- Builder is stateless (same instance can build multiple times)
- No side effects between phases (pure accumulation to archive)
- Archive discarded on error (no partial outputs)

---

## Testing Behaviors

### Unit Test Coverage

From `src/tests/unit.rs`:
- Manifest parsing validation
- CPIO generation correctness
- File addition order preservation
- Symlink resolution in archives

### Behavior Test Integration

From `tests/golden_boot_linux_openrc.txt`:
- Golden file captures boot output
- Verifies initramfs contents visible in `/proc/mounts`
- Confirms all phases executed (files, symlinks, devices present)

### Integration Points

- Called by `build_all()` command
- Called by `build initramfs` command
- Called before every `run` command
- Can be tested standalone with `test unit` and `test behavior`

---

## Performance Characteristics

### Time Complexity

- **Directory creation**: O(d) where d = number of directories
- **Binary addition**: O(b × s) where b = binaries, s = average binary size
- **Symlink creation**: O(l) where l = number of symlinks
- **File addition**: O(f × s) where f = files, s = average file size
- **Device creation**: O(v) where v = device count
- **Tree copying**: O(t) where t = total items in trees
- **Archive writing**: O(n) where n = total bytes

### Overall: O(n) where n = total archive size

### Space Complexity

- CPIO archive: O(n) where n = total uncompressed size
- Intermediate buffers: O(s) where s = largest single file

### Typical Metrics (aarch64)

- Build time: 2-5 seconds
- Archive size: 30-50 MB (gzipped ~5-10 MB)
- Files: ~100-200 entries

---

## Thread Safety

- Builder methods take `&self` (immutable)
- `CpioArchive` mutable via `&mut archive` inside build_with_events
- Event callback takes no locks
- **Conclusion**: Safe to instantiate multiple builders, but not safe to call `build_with_events` concurrently on same instance (archive is not Send/Sync for concurrent access)

---

## Related Modules

| Module | Interaction | Purpose |
|--------|-------------|---------|
| `super::manifest` | Uses `Manifest`, `FileEntry`, `parse_mode()` | Manifest parsing |
| `super::cpio` | Uses `CpioArchive` | CPIO archive format |
| `src/main.rs` | Entry point calls builders | CLI orchestration |
| `src/tests/behavior.rs` | Verifies output | Golden file testing |

---

## Future Considerations

### Known Limitations

1. **No incremental building**: Every call rebuilds entire archive
2. **No caching**: File read happens even if unchanged
3. **No compression**: Archive written uncompressed (gzip done externally)
4. **No deduplication**: Same file copied multiple times if in manifest multiple times

### Potential Optimizations

1. Cache file contents by path hash
2. Detect unchanged manifests and skip rebuild
3. Use gzip streaming directly to archive write
4. Detect file duplicates and use hard links in archive

### Design Decisions

| Decision | Rationale | Trade-off |
|----------|-----------|-----------|
| Event callback pattern | Decouple builder from UI | Slightly more code |
| Fail-fast on errors | Catch issues early | No partial recovery |
| Unconditional lib paths | Ensure compatibility | Slightly larger archive |
| Busybox duplication | Support custom kernel | Slight archive inflation |
| Relative symlinks preserved | Allow archive relocation | Assumes stable structure |
