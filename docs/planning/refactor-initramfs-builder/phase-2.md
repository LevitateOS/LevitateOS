# Phase 2: Structural Extraction - Initramfs Builder Refactor

## Target Design

### New Directory Layout

```
initramfs/                          # Declarative initramfs definition
├── initramfs.toml                  # Manifest: what goes in the CPIO
├── files/                          # Static files to include
│   ├── etc/
│   │   ├── inittab
│   │   ├── passwd
│   │   ├── group
│   │   └── profile
│   └── root/
│       └── hello.txt
└── scripts/                        # Test scripts (not embedded)
    ├── test.sh
    └── test-core.sh

xtask/src/build/
├── initramfs/                      # NEW: Initramfs builder module
│   ├── mod.rs                      # Public API: build_initramfs()
│   ├── cpio.rs                     # Pure Rust CPIO writer (~150 lines)
│   ├── manifest.rs                 # TOML manifest parser (~100 lines)
│   └── builder.rs                  # Archive builder logic (~150 lines)
├── busybox.rs                      # KEEP: BusyBox build (not initramfs-specific)
├── mod.rs                          # Update re-exports
└── ...

target/initramfs/                   # Build artifacts (not repo root!)
├── staging/                        # Temporary staging directory
├── x86_64.cpio                     # Output for x86_64
└── aarch64.cpio                    # Output for aarch64
```

### Module Responsibilities

| Module | Responsibility | Lines (est) |
|--------|---------------|-------------|
| `initramfs/mod.rs` | Public API, orchestration | ~50 |
| `initramfs/cpio.rs` | CPIO newc format writer | ~150 |
| `initramfs/manifest.rs` | TOML parsing, validation | ~100 |
| `initramfs/builder.rs` | Archive construction logic | ~150 |

**Total: ~450 lines** (vs ~800+ current)

## Manifest Format (`initramfs.toml`)

```toml
# LevitateOS Initramfs Manifest
# This file declares what goes into the initramfs CPIO archive.

[meta]
# Manifest format version (for future compatibility)
version = 1

# =============================================================================
# Directory Structure
# =============================================================================
[layout]
# Standard FHS directories to create (empty)
directories = [
    "bin",
    "sbin",
    "etc",
    "dev",
    "proc",
    "sys",
    "tmp",
    "root",
]

# =============================================================================
# Binaries
# =============================================================================
[binaries.busybox]
# Source binary (${arch} is substituted)
source = "toolchain/busybox-out/${arch}/busybox"
# Install location in initramfs
install = "/bin/busybox"
# Also copy to /init (kernel entry point - can't be symlink)
copy_as_init = true

# =============================================================================
# Symlinks
# =============================================================================
[symlinks]
# Format: "link_path" = "target"
# All relative to initramfs root

# Init system (sbin -> ../bin/busybox)
"/sbin/init" = "../bin/busybox"
"/sbin/halt" = "../bin/busybox"
"/sbin/poweroff" = "../bin/busybox"
"/sbin/reboot" = "../bin/busybox"

# Shell (bin -> busybox)
"/bin/sh" = "busybox"
"/bin/ash" = "busybox"

# Coreutils (bin -> busybox)
"/bin/cat" = "busybox"
"/bin/cp" = "busybox"
"/bin/echo" = "busybox"
"/bin/ls" = "busybox"
"/bin/mkdir" = "busybox"
"/bin/mv" = "busybox"
"/bin/pwd" = "busybox"
"/bin/rm" = "busybox"
"/bin/rmdir" = "busybox"
"/bin/touch" = "busybox"
"/bin/ln" = "busybox"
"/bin/chmod" = "busybox"
"/bin/chown" = "busybox"
"/bin/head" = "busybox"
"/bin/tail" = "busybox"
"/bin/true" = "busybox"
"/bin/false" = "busybox"
"/bin/test" = "busybox"
"/bin/[" = "busybox"
"/bin/stat" = "busybox"
"/bin/wc" = "busybox"

# Text processing
"/bin/grep" = "busybox"
"/bin/sed" = "busybox"
"/bin/awk" = "busybox"
"/bin/sort" = "busybox"
"/bin/uniq" = "busybox"
"/bin/cut" = "busybox"
"/bin/tr" = "busybox"
"/bin/tee" = "busybox"

# Search
"/bin/find" = "busybox"
"/bin/xargs" = "busybox"
"/bin/which" = "busybox"

# Archives
"/bin/tar" = "busybox"
"/bin/gzip" = "busybox"
"/bin/gunzip" = "busybox"
"/bin/zcat" = "busybox"

# Editor
"/bin/vi" = "busybox"

# Process
"/bin/ps" = "busybox"
"/bin/kill" = "busybox"
"/bin/killall" = "busybox"
"/bin/sleep" = "busybox"

# Filesystem
"/bin/mount" = "busybox"
"/bin/umount" = "busybox"
"/bin/df" = "busybox"
"/bin/du" = "busybox"

# Misc
"/bin/date" = "busybox"
"/bin/clear" = "busybox"
"/bin/reset" = "busybox"
"/bin/env" = "busybox"
"/bin/printenv" = "busybox"
"/bin/uname" = "busybox"
"/bin/hostname" = "busybox"
"/bin/id" = "busybox"
"/bin/whoami" = "busybox"

# =============================================================================
# Static Files
# =============================================================================
[files]
# Format: "dest_in_initramfs" = { source = "path", mode = 0o644 }
# Or inline: "dest" = { content = "...", mode = 0o644 }

# Config files from initramfs/files/
"/etc/inittab" = { source = "initramfs/files/etc/inittab", mode = 0o644 }
"/etc/passwd" = { source = "initramfs/files/etc/passwd", mode = 0o644 }
"/etc/group" = { source = "initramfs/files/etc/group", mode = 0o644 }
"/etc/profile" = { source = "initramfs/files/etc/profile", mode = 0o644 }
"/etc/motd" = { content = "Welcome to LevitateOS!\n", mode = 0o644 }

# Sample files
"/root/hello.txt" = { content = "Hello from LevitateOS initramfs!\n", mode = 0o644 }

# Test scripts (executable)
"/test.sh" = { source = "initramfs/scripts/test.sh", mode = 0o755 }
"/test-core.sh" = { source = "initramfs/scripts/test-core.sh", mode = 0o755 }

# =============================================================================
# Shared Libraries (TEAM_471: Future - requires kernel file-backed mmap)
# =============================================================================
# NOTE: Dynamic linking is currently BLOCKED by kernel limitations.
# The kernel's mmap only supports MAP_ANONYMOUS, not MAP_PRIVATE with fd.
# These sections are documented for future use.

# [libraries]
# # Format: "dest" = { source = "path" }
# # Libraries will be installed to /lib/ automatically
# "libncursesw.so.6" = { source = "toolchain/alpine-packages/lib/libncursesw.so.6" }

# [libraries.config]
# # musl library search path
# ld_path = ["/lib"]

# =============================================================================
# Terminal Info (TEAM_471: For curses-based applications)
# =============================================================================
# NOTE: Basic terminfo is needed for apps like vi, nano, less
# Include minimal set: linux, vt100, xterm

# [terminfo]
# # Source directory containing terminfo files
# source = "initramfs/files/etc/terminfo"
# # Install to /etc/terminfo (musl default search path)
# install = "/etc/terminfo"
```

## Rust Types

### `cpio.rs` - Pure Rust CPIO Writer

```rust
//! Pure Rust CPIO archive writer (newc format)
//!
//! The newc format uses ASCII headers (110 bytes each) followed by
//! filename and data. No external tools required.

use std::io::Write;

/// A single entry in the CPIO archive
pub struct CpioEntry {
    pub path: String,
    pub mode: u32,
    pub data: Vec<u8>,
}

/// CPIO archive builder
pub struct CpioArchive {
    entries: Vec<CpioEntry>,
}

impl CpioArchive {
    pub fn new() -> Self { ... }

    /// Add a directory entry
    pub fn add_directory(&mut self, path: &str, mode: u32) { ... }

    /// Add a regular file
    pub fn add_file(&mut self, path: &str, data: &[u8], mode: u32) { ... }

    /// Add a symbolic link
    pub fn add_symlink(&mut self, path: &str, target: &str) { ... }

    /// Write the archive to a writer (adds TRAILER automatically)
    pub fn write<W: Write>(&self, writer: W) -> std::io::Result<()> { ... }
}

// Internal: Format newc header (110 bytes ASCII)
fn format_header(
    ino: u32,
    mode: u32,
    nlink: u32,
    filesize: u32,
    namesize: u32
) -> String { ... }
```

### `manifest.rs` - TOML Parser

```rust
//! Initramfs manifest parser

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub meta: Meta,
    pub layout: Layout,
    pub binaries: HashMap<String, Binary>,
    pub symlinks: HashMap<String, String>,
    pub files: HashMap<String, FileEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub version: u32,
}

#[derive(Debug, Deserialize)]
pub struct Layout {
    pub directories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    pub source: String,
    pub install: String,
    #[serde(default)]
    pub copy_as_init: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileEntry {
    FromFile { source: String, mode: u32 },
    Inline { content: String, mode: u32 },
}

impl Manifest {
    /// Load manifest from file, substituting ${arch}
    pub fn load(path: &str, arch: &str) -> Result<Self> { ... }

    /// Validate all referenced files exist
    pub fn validate(&self) -> Result<()> { ... }
}

// TEAM_471: Validation enhancements discovered during nano exercise
impl Manifest {
    /// Check if a binary is dynamically linked (needs kernel mmap support)
    pub fn check_binary_type(&self, path: &Path) -> BinaryType {
        // Use `file` command or parse ELF header
        // Returns Static, Dynamic, or Unknown
    }

    /// Warn about dynamic binaries that won't work without kernel support
    pub fn validate_with_warnings(&self) -> Result<Vec<Warning>> {
        let mut warnings = Vec::new();

        for (name, binary) in &self.binaries {
            let source = self.resolve_path(&binary.source);
            if !source.exists() {
                return Err(anyhow!("Binary '{}' not found: {}", name, source.display()));
            }

            match Self::check_binary_type(&source) {
                BinaryType::Dynamic => {
                    warnings.push(Warning::DynamicBinary {
                        name: name.clone(),
                        note: "Requires kernel file-backed mmap (not yet implemented)",
                    });
                }
                _ => {}
            }
        }

        // Check that referenced source files exist
        for (dest, entry) in &self.files {
            if let FileEntry::FromFile { source, .. } = entry {
                if !Path::new(source).exists() {
                    return Err(anyhow!("File '{}' not found for {}", source, dest));
                }
            }
        }

        Ok(warnings)
    }
}

#[derive(Debug)]
pub enum BinaryType {
    Static,
    Dynamic,
    Unknown,
}

#[derive(Debug)]
pub enum Warning {
    DynamicBinary { name: String, note: &'static str },
    MissingLibrary { binary: String, library: String },
}
```

### `builder.rs` - Archive Builder

```rust
//! Initramfs builder - constructs CPIO from manifest

use super::{cpio::CpioArchive, manifest::Manifest};
use std::path::Path;

pub struct InitramfsBuilder {
    manifest: Manifest,
    arch: String,
}

impl InitramfsBuilder {
    pub fn new(manifest: Manifest, arch: &str) -> Self { ... }

    /// Build the initramfs, returning path to CPIO
    pub fn build(&self) -> Result<PathBuf> {
        let mut archive = CpioArchive::new();

        // 1. Create directories
        for dir in &self.manifest.layout.directories {
            archive.add_directory(dir, 0o755);
        }

        // 2. Add binaries
        for (name, binary) in &self.manifest.binaries {
            let source = binary.source.replace("${arch}", &self.arch);
            let data = std::fs::read(&source)?;
            archive.add_file(&binary.install, &data, 0o755);

            if binary.copy_as_init {
                archive.add_file("/init", &data, 0o755);
            }
        }

        // 3. Add symlinks
        for (link, target) in &self.manifest.symlinks {
            archive.add_symlink(link, target);
        }

        // 4. Add files
        for (dest, entry) in &self.manifest.files {
            let (data, mode) = match entry {
                FileEntry::FromFile { source, mode } => {
                    (std::fs::read(source)?, *mode)
                }
                FileEntry::Inline { content, mode } => {
                    (content.as_bytes().to_vec(), *mode)
                }
            };
            archive.add_file(dest, &data, mode);
        }

        // 5. Write to target/
        let output = PathBuf::from(format!("target/initramfs/{}.cpio", self.arch));
        std::fs::create_dir_all(output.parent().unwrap())?;
        let file = std::fs::File::create(&output)?;
        archive.write(file)?;

        Ok(output)
    }
}
```

### `mod.rs` - Public API

```rust
//! Initramfs builder module
//!
//! Builds initramfs CPIO archives from declarative TOML manifest.

mod builder;
mod cpio;
mod manifest;

pub use builder::InitramfsBuilder;
pub use manifest::Manifest;

/// Build initramfs for the given architecture
///
/// Loads `initramfs/initramfs.toml` and produces `target/initramfs/{arch}.cpio`
pub fn build_initramfs(arch: &str) -> anyhow::Result<std::path::PathBuf> {
    let manifest = Manifest::load("initramfs/initramfs.toml", arch)?;
    manifest.validate()?;

    let builder = InitramfsBuilder::new(manifest, arch);
    let output = builder.build()?;

    println!("Initramfs built: {}", output.display());
    Ok(output)
}

/// Backward compatibility wrapper
///
/// TODO: Remove after migration complete
pub fn create_busybox_initramfs(arch: &str) -> anyhow::Result<()> {
    let output = build_initramfs(arch)?;

    // Copy to legacy location for existing call sites
    let legacy_path = format!("initramfs_{arch}.cpio");
    std::fs::copy(&output, &legacy_path)?;

    Ok(())
}
```

## Live Dashboard TUI

**See `phase-0-tui-design.md` for complete TUI specification.**

### Key Points

- **Non-interactive**: Display-only, no user input required
- **Auto TTY detection**: Falls back to simple output in CI/pipes
- **Event-driven**: Builder emits events, TUI renders them
- **60fps max**: Smooth progress without busy-looping

### Dependencies

```toml
# xtask/Cargo.toml
[dependencies]
ratatui = "0.29"
crossterm = "0.28"
atty = "0.2"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

### Layout

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  LEVITATE INITRAMFS BUILDER                                       x86_64    │
├──────────────────────────────────────────────────────────────────────────────┤
│  Phase: Adding symlinks                                    [████████░░] 80%  │
├──────────────────────────────────────────────────────────────────────────────┤
│  ACTIVITY                                                                    │
│   ✓  + /bin/busybox                                              1.2 MB     │
│   ✓  + /init                                                     1.2 MB     │
│   ✓  → /bin/sh -> busybox                                                   │
│   ◉  → /bin/ls -> busybox                                                   │
├──────────────────────────────────────────────────────────────────────────────┤
│  STATISTICS                                                                  │
│  Directories    8          Symlinks    47/60         Total Size   2.4 MB    │
│  Binaries       2          Files        5/7          Elapsed      0.3s      │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Module: `tui.rs`

```rust
//! Non-interactive TUI dashboard for initramfs build
//!
//! Key design: NO user input. Display-only progress and status.

use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};
use std::collections::VecDeque;
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Build events emitted by the builder
#[derive(Clone)]
pub enum BuildEvent {
    PhaseStart { name: &'static str, total: usize },
    PhaseComplete { name: &'static str },
    DirectoryCreated { path: String },
    BinaryAdded { path: String, size: u64 },
    SymlinkCreated { link: String, target: String },
    FileAdded { path: String, size: u64 },
    BuildComplete { output_path: std::path::PathBuf, total_size: u64, duration: Duration },
    BuildFailed { error: String },
}

/// Non-interactive dashboard state
pub struct Dashboard {
    arch: String,
    current_phase: String,
    phase_progress: (usize, usize),
    activity: VecDeque<ActivityItem>,
    stats: BuildStats,
    start_time: Instant,
    complete: bool,
    error: Option<String>,
}

struct ActivityItem {
    icon: char,
    text: String,
    size: Option<u64>,
    status: ItemStatus,
}

#[derive(Clone, Copy, PartialEq)]
enum ItemStatus { Pending, InProgress, Done }

#[derive(Default)]
struct BuildStats {
    directories: usize,
    binaries: usize,
    binary_bytes: u64,
    symlinks_done: usize,
    symlinks_total: usize,
    files_done: usize,
    files_total: usize,
}

impl Dashboard {
    pub fn new(arch: &str) -> Self { /* ... */ }
    pub fn handle_event(&mut self, event: BuildEvent) { /* ... */ }
    pub fn render(&self, frame: &mut Frame) { /* ... */ }
    pub fn is_complete(&self) -> bool { self.complete }
}

/// Auto-detect if TUI should be used
pub fn should_use_tui() -> bool {
    atty::is(atty::Stream::Stdout)
        && std::env::var("NO_TUI").is_err()
        && std::env::var("CI").is_err()
}

/// Run build with non-interactive TUI dashboard
pub fn run_build_with_tui(arch: &str) -> anyhow::Result<std::path::PathBuf> {
    let mut terminal = ratatui::init();
    crossterm::terminal::enable_raw_mode()?;

    let mut dashboard = Dashboard::new(arch);
    let (tx, rx) = mpsc::channel();

    // Build in separate thread
    let arch_owned = arch.to_string();
    let build_handle = std::thread::spawn(move || {
        super::builder::build_with_events(&arch_owned, |event| {
            tx.send(event).ok();
        })
    });

    // Render loop (non-blocking)
    loop {
        // Process available events
        while let Ok(event) = rx.try_recv() {
            dashboard.handle_event(event);
        }

        terminal.draw(|f| dashboard.render(f))?;

        if dashboard.is_complete() {
            std::thread::sleep(Duration::from_millis(500)); // Brief pause to show result
            break;
        }

        std::thread::sleep(Duration::from_millis(16)); // ~60fps
    }

    // Cleanup
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;

    build_handle.join().unwrap()
}
```

### Integration with Builder

```rust
// builder.rs - emit events during build

impl InitramfsBuilder {
    pub fn build_with_events<F>(&self, emit: F) -> Result<PathBuf>
    where
        F: Fn(BuildEvent),
    {
        let mut archive = CpioArchive::new();

        // 1. Directories
        emit(BuildEvent::PhaseStart { name: "Creating directories", total: dirs.len() });
        for dir in &self.manifest.layout.directories {
            archive.add_directory(dir, 0o755);
            emit(BuildEvent::DirectoryCreated { path: dir.clone() });
        }

        // 2. Binaries
        emit(BuildEvent::PhaseStart { name: "Adding binaries", total: bins.len() });
        for (_, binary) in &self.manifest.binaries {
            let data = std::fs::read(&source)?;
            let size = data.len() as u64;
            archive.add_file(&binary.install, &data, 0o755);
            emit(BuildEvent::BinaryAdded { path: binary.install.clone(), size });
        }

        // 3. Symlinks
        emit(BuildEvent::PhaseStart { name: "Creating symlinks", total: links.len() });
        for (link, target) in &self.manifest.symlinks {
            archive.add_symlink(link, target);
            emit(BuildEvent::SymlinkCreated { link: link.clone(), target: target.clone() });
        }

        // 4. Files
        emit(BuildEvent::PhaseStart { name: "Adding files", total: files.len() });
        // ... similar pattern

        emit(BuildEvent::Complete { total_size, duration: start.elapsed() });
        Ok(output)
    }
}
```

### CLI Flag

```rust
// main.rs - add --tui flag

#[derive(Parser)]
struct Args {
    /// Show live dashboard during build
    #[arg(long)]
    tui: bool,
}

// In build initramfs handler:
if args.tui {
    initramfs::build_with_tui(arch)?;
} else {
    initramfs::build_initramfs(arch)?;
}
```

### Fallback for Non-TTY

When stdout is not a TTY (CI, pipes), fall back to simple line output:

```rust
pub fn build_initramfs(arch: &str) -> Result<PathBuf> {
    if atty::is(atty::Stream::Stdout) && std::env::var("NO_TUI").is_err() {
        build_with_tui(arch)
    } else {
        build_simple(arch)  // Just println! for each step
    }
}
```

### Module Responsibilities (Updated)

| Module | Responsibility | Lines (est) |
|--------|---------------|-------------|
| `initramfs/mod.rs` | Public API, orchestration | ~50 |
| `initramfs/cpio.rs` | CPIO newc format writer | ~150 |
| `initramfs/manifest.rs` | TOML parsing, validation | ~100 |
| `initramfs/builder.rs` | Archive construction + events | ~200 |
| `initramfs/tui.rs` | Live dashboard rendering | ~250 |

**Total: ~750 lines** (still reasonable, each module focused)

## Extraction Strategy

### Order of Implementation

1. **Create `cpio.rs`** (no dependencies on existing code)
   - Pure Rust CPIO writer
   - Unit tests for format correctness

2. **Create `manifest.rs`** (no dependencies)
   - TOML parsing with serde
   - Validation logic

3. **Create `builder.rs`** (depends on 1 & 2)
   - Archive construction
   - Integration with busybox.rs (for binary source)

4. **Create `initramfs/` directory structure**
   - Move `xtask/initrd_resources/*.sh` → `initramfs/scripts/`
   - Create `initramfs/files/etc/*` from embedded strings
   - Create `initramfs/initramfs.toml`

5. **Update `mod.rs`** to export new API
   - Keep old `create_busybox_initramfs` as wrapper initially

6. **Verify output matches** (golden test)
   - Extract both old and new CPIOs
   - Compare file listings and contents

### Coexistence Strategy

During migration, both old and new systems work:

```rust
// Phase 1: New system, copies to legacy path
pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    let new_path = new_initramfs::build_initramfs(arch)?;
    std::fs::copy(&new_path, format!("initramfs_{arch}.cpio"))?;
    Ok(())
}
```

After verification, remove legacy copy step.

## Rule 7 Compliance

| Module | Lines | Private State | Deep Imports |
|--------|-------|---------------|--------------|
| `cpio.rs` | ~150 | `entries: Vec<CpioEntry>` | None |
| `manifest.rs` | ~100 | Parsed structs | None |
| `builder.rs` | ~150 | `manifest`, `arch` | Uses cpio, manifest |
| `mod.rs` | ~50 | None | Re-exports only |

All modules:
- Own their state privately
- Expose intentional APIs
- No cross-module field access
- Under 500 lines each

## Files Created in This Phase

| File | Purpose |
|------|---------|
| `xtask/src/build/initramfs/mod.rs` | Module entry, public API |
| `xtask/src/build/initramfs/cpio.rs` | CPIO writer |
| `xtask/src/build/initramfs/manifest.rs` | TOML parser |
| `xtask/src/build/initramfs/builder.rs` | Builder logic |
| `initramfs/initramfs.toml` | Manifest |
| `initramfs/files/etc/inittab` | Init config |
| `initramfs/files/etc/passwd` | User database |
| `initramfs/files/etc/group` | Group database |
| `initramfs/files/etc/profile` | Shell profile |
| `initramfs/scripts/test.sh` | Moved from xtask |
| `initramfs/scripts/test-core.sh` | Moved from xtask |

## Future Enhancements (TEAM_471)

Based on pain points discovered during the nano exercise, these features are documented for future implementation once kernel support is available.

### Kernel Prerequisites

| Feature | Kernel Requirement | Status |
|---------|-------------------|--------|
| Dynamic binaries | File-backed mmap (`MAP_PRIVATE` with fd) | Not implemented |
| Symlink following | `readlink` syscall (#89) | Not implemented |
| Shared libraries | File-backed mmap | Not implemented |

### Phase 2b: Dynamic Linking Support

Once the kernel supports file-backed mmap, add:

```toml
# initramfs.toml additions

[libraries]
# Shared libraries to include
"libncursesw.so.6" = { source = "toolchain/libs/libncursesw.so.6" }
"libc.musl-x86_64.so.1" = { source = "/lib/libc.musl-x86_64.so.1" }

[libraries.config]
# Generate /etc/ld-musl-x86_64.path automatically
ld_path = ["/lib", "/usr/lib"]
```

Builder changes:
1. Auto-detect dynamic binaries using ELF header parsing
2. Warn if dependencies aren't in manifest
3. Generate `/etc/ld-musl-{arch}.path` from config

### Phase 2c: Alpine Package Integration

For easy inclusion of pre-built Alpine packages:

```toml
[alpine]
# Auto-download and extract Alpine packages
packages = ["nano", "htop", "less"]
# Automatically resolve dependencies
resolve_deps = true
# Architecture (x86_64, aarch64)
arch = "${arch}"
```

This would:
1. Download .apk files from Alpine mirrors
2. Extract binaries and libraries
3. Validate all dependencies are satisfied
4. Warn about missing kernel features

### Phase 2d: Terminfo Database

```toml
[terminfo]
# Include these terminal definitions
terminals = ["linux", "vt100", "xterm", "xterm-256color"]
# Or include a directory
source = "initramfs/files/etc/terminfo"
```

### Validation Enhancements

Add to `manifest.rs`:

```rust
/// Analyze a binary and report its requirements
pub fn analyze_binary(path: &Path) -> BinaryAnalysis {
    BinaryAnalysis {
        binary_type: detect_elf_type(path),
        interpreter: parse_elf_interp(path),
        libraries: parse_elf_needed(path),
        size: path.metadata().map(|m| m.len()).ok(),
    }
}

/// Validate manifest with detailed diagnostics
pub fn validate_detailed(&self) -> ValidationReport {
    let mut report = ValidationReport::default();

    for binary in &self.binaries {
        let analysis = analyze_binary(&binary.source);
        if analysis.binary_type == BinaryType::Dynamic {
            report.warnings.push(format!(
                "{} is dynamically linked, requires: {:?}",
                binary.name, analysis.libraries
            ));
        }
    }

    report
}
```

### TUI Enhancements

Show validation warnings in the dashboard:

```
┌─────────────────────────────────────────────────────────────────────┐
│  LevitateOS Initramfs Builder                          x86_64      │
├─────────────────────────────────────────────────────────────────────┤
│  ⚠️  WARNINGS                                                        │
│  ├─ nano is dynamically linked (needs kernel mmap support)          │
│  └─ Missing library: libncursesw.so.6                               │
├─────────────────────────────────────────────────────────────────────┤
│  Phase: Adding binaries                              [████░░░░░░] 40% │
└─────────────────────────────────────────────────────────────────────┘
```
