# LevitateOS: Fedora ISO as Reproducible Source

## Problem Statement

Currently, LevitateOS has two problems:

1. **Copies from host system** - glibc, PAM, etc. copied from `/lib64/` - not reproducible
2. **Builds Rust alternatives** - uutils, brush, findutils, diffutils, sudo-rs - unnecessary complexity

**Solution:** Use a pinned Fedora ISO as the single source for ALL binaries and libraries.

---

## The Big Simplification

### REMOVE from vendor/ (Rust apps we don't need)

| Directory | What it is | Fedora replacement |
|-----------|------------|-------------------|
| `vendor/uutils/` | Rust coreutils | `coreutils` (GNU) |
| `vendor/brush/` | Rust shell | `bash` |
| `vendor/findutils/` | Rust find/xargs | `findutils` (GNU) |
| `vendor/diffutils/` | Rust diff/cmp | `diffutils` (GNU) |
| `vendor/sudo-rs/` | Rust sudo | `sudo` |
| `vendor/helix/` | Rust editor | `nano` + `vim` |
| `vendor/procps-ng/` | Process tools | `procps-ng` (same) |
| `vendor/iproute2/` | Network tools | `iproute` (same) |
| `vendor/iputils/` | ping, etc. | `iputils` (same) |
| `vendor/util-linux/` | mount, login, etc. | `util-linux` (same) |
| `vendor/systemd/` | Init system | `systemd` ✓ |

### KEEP in vendor/

| Directory | Why |
|-----------|-----|
| `vendor/linux/` | Kernel - need custom config |
| `vendor/images/` | Source ISOs |

### Everything else comes from Fedora ISO

---

## Source ISO

```
Filename: Fedora-Sway-Live-x86_64-43-1.6.iso
Size:     2.1 GB
SHA256:   41d08d2e5b99a9f255eddc9aa7c310fea5436de1cc90085e57f5520c937d8bd6

Structure:
ISO → LiveOS/squashfs.img → Full Fedora root filesystem
```

---

## Verified: Fedora Has Everything We Need

```
✓ bash, sh                    (shells)
✓ ls, cat, cp, mv, rm, etc.   (coreutils)
✓ find, xargs                 (findutils)
✓ diff, cmp                   (diffutils)
✓ sudo                        (setuid)
✓ ps, free, top, htop         (procps-ng)
✓ ip, ss, bridge              (iproute2)
✓ ping, tracepath             (iputils)
✓ mount, login, agetty, etc.  (util-linux)
✓ systemd, systemctl          (init)
✓ nano, vi                    (editors)
✓ less, sed, gawk             (text processing)
✓ tar, gzip, xz, zstd         (compression)
✓ curl, ssh, rsync            (network)
✓ glibc, PAM, all libs        (system)
```

---

## New Architecture

### Before (complex)
```
vendor/
├── linux/          # Build kernel
├── systemd/        # Build from source
├── util-linux/     # Build from source
├── uutils/         # Build Rust coreutils
├── brush/          # Build Rust shell
├── findutils/      # Build Rust find
├── diffutils/      # Build Rust diff
├── sudo-rs/        # Build Rust sudo
├── helix/          # Build Rust editor
├── procps-ng/      # Build from source
├── iproute2/       # Build from source
├── iputils/        # Build from source
└── images/         # ISOs
```

### After (simple)
```
vendor/
├── linux/          # Build kernel (custom config)
├── fedora-root/    # Extracted from ISO (read-only source)
└── images/         # Source ISOs
    └── Fedora-Sway-Live-x86_64-43-1.6.iso
```

---

## Implementation Plan

### Phase 1: Create Fedora extraction

New file: `crates/builder/src/builder/fedora.rs`

```rust
const FEDORA_ISO: &str = "Fedora-Sway-Live-x86_64-43-1.6.iso";
const FEDORA_SHA256: &str = "41d08d2e5b99a9f255eddc9aa7c310fea5436de1cc90085e57f5520c937d8bd6";
const FEDORA_URL: &str = "https://download.fedoraproject.org/pub/fedora/linux/releases/43/Sway/x86_64/iso/...";

pub fn ensure_fedora_root() -> Result<PathBuf>;  // Download ISO if needed, extract squashfs
pub fn fedora_bin(name: &str) -> PathBuf;        // e.g., fedora_bin("bash") -> vendor/fedora-root/usr/bin/bash
pub fn fedora_lib(name: &str) -> PathBuf;        // e.g., fedora_lib("libc.so.6") -> vendor/fedora-root/usr/lib64/libc.so.6
```

### Phase 2: Simplify components

Delete these component files:
- `components/uutils.rs`
- `components/brush.rs`
- `components/findutils.rs`
- `components/diffutils.rs`
- `components/sudo_rs.rs`
- `components/helix.rs`
- `components/procps.rs`
- `components/iproute2.rs`
- `components/iputils.rs`
- `components/util_linux.rs`
- `components/systemd.rs`

Create single new approach:
- `components/fedora.rs` - Declares all binaries/libs to copy from Fedora

### Phase 3: Simplify initramfs.rs

Instead of complex per-component logic:
```rust
// Copy binaries from Fedora
let binaries = [
    ("usr/bin/bash", "bin/bash"),
    ("usr/bin/ls", "bin/ls"),
    ("usr/bin/cat", "bin/cat"),
    // ... etc
];

for (src, dest) in binaries {
    copy_from_fedora(src, dest)?;
}
```

### Phase 4: Clean up vendor/

```bash
rm -rf vendor/uutils
rm -rf vendor/brush
rm -rf vendor/findutils
rm -rf vendor/diffutils
rm -rf vendor/sudo-rs
rm -rf vendor/helix
rm -rf vendor/procps-ng
rm -rf vendor/iproute2
rm -rf vendor/iputils
rm -rf vendor/util-linux
rm -rf vendor/systemd
```

Keep only:
- `vendor/linux/` (kernel)
- `vendor/images/` (ISOs)

---

## New Build Flow

```
cargo run --bin builder -- initramfs

1. Ensure vendor/images/Fedora-Sway-Live-x86_64-43-1.6.iso exists
   └── Download if missing, verify SHA256

2. Ensure vendor/fedora-root/ exists
   └── Extract squashfs if missing

3. Build kernel from vendor/linux/

4. Create initramfs:
   └── Copy binaries from vendor/fedora-root/usr/bin/
   └── Copy libraries from vendor/fedora-root/usr/lib64/
   └── Copy systemd units from vendor/fedora-root/usr/lib/systemd/
   └── Create config files (passwd, shadow, etc.)

5. Package as initramfs.cpio.gz
```

---

## What This Means

### Pros
- **Simpler codebase** - No building 10+ projects from source
- **Faster builds** - Just copy, no compile
- **Reproducible** - Pinned ISO version
- **Battle-tested** - Fedora's binaries, not experimental Rust rewrites
- **Smaller repo** - Remove ~500MB+ of vendor source code

### Cons
- **Larger ISO download** - 2.1 GB (one-time)
- **Fedora-dependent** - Tied to their release cycle
- **Less "pure"** - Not everything built from source

---

## Files to Delete

### Components (crates/builder/src/builder/components/)
```
- uutils.rs
- brush.rs
- findutils.rs
- diffutils.rs
- sudo_rs.rs
- helix.rs
- procps.rs
- iproute2.rs
- iputils.rs
- util_linux.rs
- systemd.rs
```

### Vendor directories
```
- vendor/uutils/
- vendor/brush/
- vendor/findutils/
- vendor/diffutils/
- vendor/sudo-rs/
- vendor/helix/
- vendor/procps-ng/
- vendor/iproute2/
- vendor/iputils/
- vendor/util-linux/
- vendor/systemd/
```

---

## Files to Create/Modify

### New
- `crates/builder/src/builder/fedora.rs` - ISO download, extraction, path helpers

### Modify
- `crates/builder/src/builder/components/mod.rs` - Simplify to just Linux + Fedora
- `crates/builder/src/builder/components/registry.rs` - Remove old components
- `crates/builder/src/builder/initramfs.rs` - Copy from Fedora root instead of vendor builds
- `crates/builder/src/builder/components/glibc.rs` - Use Fedora paths

---

## Legal Compliance

Fedora is 100% open source (GPL, LGPL, MIT, etc.)

Required:
1. Include attribution: "Contains software from the Fedora Project"
2. Link to source: koji.fedoraproject.org for F43 packages
3. Include licenses: Copy /usr/share/licenses/ or reference Fedora

NOT allowed:
- Use Fedora trademarks/logo
- Claim to be "Fedora" or official Fedora derivative

---

## Questions to Decide

1. **Extraction method:**
   - `unsquashfs` (needs squashfs-tools installed)
   - `mount` (needs sudo)
   - Extract at build time vs pre-extracted in repo?

2. **What about custom kernel modules?**
   - Keep building kernel from source (vendor/linux/)
   - Or use Fedora's kernel too?

3. **CI/CD:**
   - Cache the 2.1 GB ISO?
   - Cache the extracted root (~4-5 GB)?

---

## Tomorrow's Tasks

1. Create `fedora.rs` with ISO download/extract logic
2. Delete all the vendor source directories (except linux/)
3. Delete all the component files (except linux.rs)
4. Create new simple `fedora.rs` component
5. Refactor `initramfs.rs` to copy from Fedora
6. Test build
7. Boot and verify everything works

---

## Size Estimates

### Current initramfs
- ~64 MB compressed (includes 235 MB Helix runtime)

### After refactor
- ~25-35 MB compressed (estimate)
- Much smaller because Fedora binaries are stripped and optimized

### Repo size change
- Remove: ~500 MB+ of vendor source code
- Add: Nothing (ISO downloaded at build time, not in repo)
