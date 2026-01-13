# Phase 5: Hardening - Initramfs Builder Refactor

## Final Verification

### Test Matrix

| Test | Command | Expected |
|------|---------|----------|
| Unit tests | `cargo test -p xtask` | All pass |
| Build kernel | `cargo xtask build kernel` | Success |
| Build initramfs | `cargo xtask build initramfs` | Creates `target/initramfs/{arch}.cpio` |
| Build all | `cargo xtask build all` | Kernel + initramfs + ISO |
| Build ISO | `cargo xtask build iso` | ISO with embedded initramfs |
| Run x86_64 | `cargo xtask run` | Boots to BusyBox shell |
| Run aarch64 | `cargo xtask --arch aarch64 run` | Boots to BusyBox shell |
| Behavior tests | `cargo xtask test behavior` | All pass |
| Regression tests | `cargo xtask test regress` | All pass |

### CPIO Contents Verification

After refactor, verify initramfs contains expected files:

```bash
mkdir -p /tmp/verify
cd /tmp/verify
cpio -idv < $PROJECT/target/initramfs/x86_64.cpio
```

Expected contents:
```
.
./bin
./bin/busybox
./bin/sh -> busybox
./bin/ash -> busybox
./bin/cat -> busybox
... (all symlinks)
./sbin
./sbin/init -> ../bin/busybox
./sbin/reboot -> ../bin/busybox
... (sbin symlinks)
./etc
./etc/inittab
./etc/passwd
./etc/group
./etc/profile
./etc/motd
./dev
./proc
./sys
./tmp
./root
./root/hello.txt
./init
./test.sh
./test-core.sh
```

### Behavior Test Verification

```bash
cargo xtask test behavior
```

Must match `tests/golden_boot_x86_64.txt` exactly (or update if behavior intentionally changed).

## Documentation Updates

### Update CLAUDE.md

Add section about initramfs builder:

```markdown
### Initramfs Builder

The initramfs is built from a declarative manifest at `initramfs/initramfs.toml`.

**Directory structure:**
```
initramfs/
├── initramfs.toml      # What goes in the CPIO
├── files/              # Static files to include
│   └── etc/            # Config files
└── scripts/            # Test scripts
```

**Build commands:**
```bash
cargo xtask build initramfs    # Build initramfs only
cargo xtask build all          # Includes initramfs
```

**Adding files:**
Edit `initramfs/initramfs.toml`:
```toml
[files]
"/my/file.txt" = { source = "initramfs/files/my/file.txt", mode = 0o644 }
```

**Adding symlinks:**
```toml
[symlinks]
"/bin/newcmd" = "busybox"
```
```

### Update README or ARCHITECTURE.md

If these reference initramfs, update to reflect new structure.

### Add Module Documentation

Each new file should have module-level docs:

```rust
//! Pure Rust CPIO archive writer (newc format)
//!
//! This module implements CPIO archive creation without external dependencies.
//! The newc format uses 110-byte ASCII headers.
//!
//! # Example
//! ```
//! let mut archive = CpioArchive::new();
//! archive.add_directory("bin", 0o755);
//! archive.add_file("bin/hello", b"#!/bin/sh\necho hello", 0o755);
//! archive.write(file)?;
//! ```
```

## Error Message Quality

Ensure all error paths have helpful messages:

```rust
// Good
Manifest::load("initramfs/initramfs.toml", arch)
    .context("Failed to load initramfs manifest. Does initramfs/initramfs.toml exist?")?;

// Good
std::fs::read(&source_path)
    .with_context(|| format!(
        "Failed to read file '{}' referenced in initramfs.toml",
        source_path.display()
    ))?;

// Good
if !busybox_path.exists() {
    bail!(
        "BusyBox binary not found at {}.\n\
         Run 'cargo xtask build busybox' first.",
        busybox_path.display()
    );
}
```

## Handoff Notes

### What Changed

1. **New declarative system**: Initramfs contents defined in `initramfs/initramfs.toml`
2. **Pure Rust CPIO**: No external `cpio` or `find` binaries required
3. **Clean workspace**: Build artifacts now in `target/initramfs/`
4. **Externalized scripts**: Test scripts in `initramfs/scripts/`, not embedded

### What Stayed the Same

1. **Public API**: `create_busybox_initramfs(arch)` still works
2. **CPIO format**: Same newc format, kernel-compatible
3. **Contents**: Same files, symlinks, directories as before

### How to Add Things

**Add a new BusyBox applet symlink:**
```toml
# initramfs/initramfs.toml
[symlinks]
"/bin/newcmd" = "busybox"
```

**Add a new config file:**
```toml
# initramfs/initramfs.toml
[files]
"/etc/newconfig" = { source = "initramfs/files/etc/newconfig", mode = 0o644 }
```

Then create `initramfs/files/etc/newconfig`.

**Add inline content:**
```toml
"/etc/hostname" = { content = "levitateos\n", mode = 0o644 }
```

### Known Limitations

1. **No dynamic content**: Manifest is static TOML, no scripting
2. **BusyBox-centric**: Designed for BusyBox symlink model
3. **No compression**: CPIO is uncompressed (kernel handles decompression if needed)

### Future Improvements (Deferred)

1. **Conditional entries**: `[files.x86_64]` sections for arch-specific
2. **Auto-generate symlinks**: Query `busybox --list` instead of static list
3. **Compression support**: Optional gzip/lz4 of CPIO
4. **Multiple profiles**: `initramfs-test.toml`, `initramfs-minimal.toml`

## Final Checklist

- [ ] All tests pass (`cargo xtask test`)
- [ ] `cargo xtask build all` works
- [ ] `cargo xtask run` boots successfully
- [ ] `cargo xtask run --arch aarch64` boots successfully (if supported)
- [ ] No CPIO files at repo root
- [ ] No `initrd_root/` directory at repo root
- [ ] `initramfs/initramfs.toml` is readable and documented
- [ ] Module docs in all new files
- [ ] Error messages are helpful
- [ ] CLAUDE.md updated with new workflow
- [ ] Team file completed with handoff notes
- [ ] Old dead code removed (Phase 4 checklist complete)

## Success Metrics

| Metric | Before | After | Target |
|--------|--------|-------|--------|
| External tool deps | 2 (find, cpio) | 0 | 0 |
| Lines of Rust | ~800 | ~380 | <500 |
| Time to add file | ~5 min (edit, compile) | ~30 sec (edit TOML) | <1 min |
| Build artifacts at root | 3+ files | 0 | 0 |
| Embedded strings | 2 (test scripts) | 0 | 0 |
