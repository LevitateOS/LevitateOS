# Phase 3: Migration - Initramfs Builder Refactor

## Prerequisites

Before starting, ensure these dependencies are added to xtask:

```toml
# xtask/Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "0.8"
ratatui = "0.29"
crossterm = "0.28"
atty = "0.2"
```

## Migration Order

### Step 1: Create Pure Rust CPIO Writer
**File:** `xtask/src/build/initramfs/cpio.rs`
**Lines:** ~150
**Dependencies:** None (can be developed in isolation)

```bash
# Test independently
cargo test -p xtask cpio
```

Key functions:
- `CpioArchive::new()` - Create empty archive
- `add_directory(path, mode)` - Add directory entry
- `add_file(path, data, mode)` - Add regular file
- `add_symlink(link, target)` - Add symbolic link
- `write(writer)` - Write newc format CPIO

### Step 2: Create Manifest Parser
**File:** `xtask/src/build/initramfs/manifest.rs`
**Lines:** ~150
**Dependencies:** serde, toml

Key types:
- `Manifest` - Root struct with layout, binaries, symlinks, files
- `Binary` - Binary definition with source, install path
- `FileEntry` - Either from file or inline content
- `Manifest::load(path, arch)` - Load and substitute `${arch}`
- `Manifest::validate()` - Check all referenced files exist

### Step 3: Create Static Files Directory
**Files to create:**

```
initramfs/
├── initramfs.toml           # Manifest (Step 4)
├── files/
│   └── etc/
│       ├── inittab          # Extract from initramfs.rs
│       ├── passwd           # root:x:0:0:root:/root:/bin/sh
│       ├── group            # root:x:0:
│       └── profile          # Shell profile
└── scripts/
    ├── test.sh              # Move from xtask/initrd_resources/
    └── test-core.sh         # Move from xtask/initrd_resources/
```

### Step 4: Create Manifest
**File:** `initramfs/initramfs.toml`

Translate hardcoded values from current `initramfs.rs` to TOML.
See `phase-2.md` for full manifest format.

### Step 5: Create Builder Module (with Event Emission)
**File:** `xtask/src/build/initramfs/builder.rs`
**Lines:** ~200
**Dependencies:** cpio.rs, manifest.rs

Key functions:
- `InitramfsBuilder::new(manifest, arch)` - Create builder
- `build()` - Build without events (simple mode)
- `build_with_events(callback)` - Build with event emission for TUI

The builder emits `BuildEvent` variants as it works:
- `PhaseStart { name, total }` - Starting a phase
- `DirectoryCreated { path }` - Directory added
- `BinaryAdded { path, size }` - Binary added
- `SymlinkCreated { link, target }` - Symlink created
- `FileAdded { path, size }` - File added
- `BuildComplete { output_path, total_size, duration }` - Done
- `BuildFailed { error }` - Error occurred

### Step 6: Create TUI Dashboard (Non-Interactive)
**File:** `xtask/src/build/initramfs/tui.rs`
**Lines:** ~300
**Dependencies:** ratatui, crossterm, atty

See `phase-0-tui-design.md` for full specification.

Key points:
- **Non-interactive**: Display-only, no keyboard input
- **Auto-detect TTY**: Falls back to simple output in CI
- **Event-driven**: Receives events from builder thread
- **60fps max**: Smooth updates without busy-looping

Key functions:
- `should_use_tui()` - Check if TUI should be enabled
- `run_build_with_tui(arch)` - Run build with dashboard
- `run_build_simple(arch)` - Run build with line output
- `Dashboard::handle_event(event)` - Update state
- `Dashboard::render(frame)` - Render to terminal

### Step 7: Create Module Entry Point
**File:** `xtask/src/build/initramfs/mod.rs`
**Lines:** ~80

```rust
mod builder;
mod cpio;
mod manifest;
mod tui;

pub use builder::InitramfsBuilder;
pub use manifest::Manifest;
pub use tui::BuildEvent;

/// Build initramfs with automatic TUI detection
pub fn build_initramfs(arch: &str) -> anyhow::Result<std::path::PathBuf> {
    if tui::should_use_tui() {
        tui::run_build_with_tui(arch)
    } else {
        tui::run_build_simple(arch)
    }
}

/// Backward compatibility wrapper (copies to legacy path)
pub fn create_busybox_initramfs(arch: &str) -> anyhow::Result<()> {
    let output = build_initramfs(arch)?;
    let legacy_path = format!("initramfs_{arch}.cpio");
    std::fs::copy(&output, &legacy_path)?;
    Ok(())
}
```

### Step 8: Update Build Module
**File:** `xtask/src/build/mod.rs`

The existing `initramfs.rs` file becomes an `initramfs/` directory.
Rename old file temporarily while developing:

```bash
mv xtask/src/build/initramfs.rs xtask/src/build/initramfs_old.rs
mkdir -p xtask/src/build/initramfs
```

Update mod.rs to use new module.

### Step 9: Verify Output Matches

```bash
# Build with old system first
USE_OLD_INITRAMFS=1 cargo xtask build initramfs
mv initramfs_x86_64.cpio initramfs_old.cpio

# Build with new system
cargo xtask build initramfs

# Compare contents
mkdir -p /tmp/old /tmp/new
cd /tmp/old && cpio -idv < $PROJECT/initramfs_old.cpio
cd /tmp/new && cpio -idv < $PROJECT/initramfs_x86_64.cpio
diff -r /tmp/old /tmp/new
```

### Step 10: Run Tests

```bash
# Run behavior tests
cargo xtask test behavior

# Run with VM
cargo xtask vm exec "ls -la /bin"
```

## Call Site Inventory

All call sites that need to continue working after migration:

### Direct Calls to `create_busybox_initramfs`

| File | Line | Context |
|------|------|---------|
| `xtask/src/main.rs` | 304 | `BuildCommands::All` match arm |
| `xtask/src/main.rs` | 306 | `BuildCommands::Initramfs` match arm |
| `xtask/src/run.rs` | 263 | Before running QEMU |
| `xtask/src/build/orchestration.rs` | 37 | Part of `build_all()` |
| `xtask/src/build/iso.rs` | 41 | Before building ISO |

### References to Output Path (`initramfs_{arch}.cpio`)

| File | Line | Pattern |
|------|------|---------|
| `xtask/src/tests/behavior.rs` | 122 | `format!("initramfs_{arch}.cpio")` |
| `xtask/src/tests/serial_input.rs` | 60 | `"initramfs_aarch64.cpio"` |
| `xtask/src/tests/shutdown.rs` | 67 | `"initramfs_aarch64.cpio"` |
| `xtask/src/tests/keyboard_input.rs` | 65 | `"initramfs_aarch64.cpio"` |
| `xtask/src/qemu/builder.rs` | 120-121 | Arch-specific paths |
| `xtask/src/build/iso.rs` | 60 | `format!("initramfs_{arch}.cpio")` |
| `xtask/src/vm/exec.rs` | 156 | `"initramfs_aarch64.cpio"` |
| `xtask/src/support/clean.rs` | 35-37 | Cleanup list |

## Backward Compatibility During Migration

The wrapper function ensures no call sites break:

```rust
// xtask/src/build/initramfs/mod.rs

/// Build initramfs using new declarative system
pub fn build_initramfs(arch: &str) -> Result<PathBuf> {
    let manifest = Manifest::load("initramfs/initramfs.toml", arch)?;
    manifest.validate()?;

    let builder = InitramfsBuilder::new(manifest, arch);
    builder.build()
}

/// Backward compatibility wrapper
///
/// Builds initramfs and copies to legacy location.
/// All existing call sites use this function.
pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    let output = build_initramfs(arch)?;

    // Copy to legacy location at repo root
    let legacy_path = format!("initramfs_{arch}.cpio");
    std::fs::copy(&output, &legacy_path)?;

    let size_kb = std::fs::metadata(&legacy_path)?.len() / 1024;
    println!("Initramfs created: {legacy_path} ({size_kb} KB)");

    Ok(())
}
```

## Rollback Plan

If issues are discovered after migration:

### Immediate Rollback
```bash
# Revert to old initramfs.rs (git has history)
git checkout HEAD~1 -- xtask/src/build/initramfs.rs

# Adjust mod.rs to use file instead of directory
# mod initramfs;  (file)
# not
# mod initramfs;  (directory with mod.rs)
```

### Keeping Both Systems
During development, can keep both:
```rust
mod initramfs_old;  // Current file
mod initramfs;      // New directory

pub fn create_busybox_initramfs(arch: &str) -> Result<()> {
    // Toggle between implementations
    if std::env::var("USE_NEW_INITRAMFS").is_ok() {
        initramfs::create_busybox_initramfs(arch)
    } else {
        initramfs_old::create_busybox_initramfs(arch)
    }
}
```

### Feature Flag Alternative
```toml
# xtask/Cargo.toml
[features]
new-initramfs = []
```

```rust
#[cfg(feature = "new-initramfs")]
mod initramfs;

#[cfg(not(feature = "new-initramfs"))]
#[path = "initramfs_old.rs"]
mod initramfs;
```

## Breaking Changes (Rule 5)

**NO compatibility shims.** The API signature is preserved:

```rust
// Before
pub fn create_busybox_initramfs(arch: &str) -> Result<()>

// After (same signature, different implementation)
pub fn create_busybox_initramfs(arch: &str) -> Result<()>
```

The only "shim" is the copy-to-legacy-path behavior, which is temporary scaffolding removed in Phase 4.

## Migration Checklist

- [ ] Create `xtask/src/build/initramfs/cpio.rs`
- [ ] Add unit tests for CPIO writer
- [ ] Add `serde`, `toml`, `ratatui`, `crossterm` to xtask dependencies
- [ ] Create `xtask/src/build/initramfs/manifest.rs`
- [ ] Add unit tests for manifest parser
- [ ] Create `initramfs/files/etc/` directory with config files
- [ ] Move `xtask/initrd_resources/*.sh` → `initramfs/scripts/`
- [ ] Create `initramfs/initramfs.toml`
- [ ] Create `xtask/src/build/initramfs/builder.rs` with event emission
- [ ] Create `xtask/src/build/initramfs/tui.rs` (live dashboard)
- [ ] Create `xtask/src/build/initramfs/mod.rs`
- [ ] Rename old `xtask/src/build/initramfs.rs` → `initramfs_old.rs` (temporary)
- [ ] Update `xtask/src/build/mod.rs` imports
- [ ] Test TUI dashboard works in terminal
- [ ] Verify fallback works in non-TTY (CI)
- [ ] Run behavior tests to verify boot works
- [ ] Compare CPIO contents (old vs new)
- [ ] Remove `initramfs_old.rs` after verification
