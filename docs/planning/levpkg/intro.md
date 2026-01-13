# levpkg: Rust-Native Package Manager

> **Status**: Future work. First milestone is APK compatibility.

## Vision

`levpkg` is a Rust-native package manager for LevitateOS - simple, fast, and built for source-based distributions.

**Philosophy**: What Alpine does with `apk` (C, 15,000+ LOC), we do with Rust in ~2,500 LOC.

---

## Why Build Our Own?

| apk (Alpine) | levpkg (LevitateOS) |
|--------------|---------------------|
| Binary packages | Build from source |
| Complex C codebase | Simple Rust |
| Repository servers | Local + remote recipes |
| 14,000 packages | Curated minimal set |

We don't need the complexity of a full distribution package manager. We need a **build-from-source recipe system** with caching.

---

## Design Principles

1. **Source-first**: All packages build from source (reproducible)
2. **Static linking**: Default to musl static binaries
3. **Declarative recipes**: TOML manifests, not shell scripts
4. **Content-addressed cache**: Same inputs = same outputs = cache hit
5. **Minimal deps**: Each package declares exactly what it needs

---

## Recipe Format

```toml
# recipes/nano.toml
[package]
name = "nano"
version = "8.0"
description = "Simple text editor"
license = "GPL-3.0"
homepage = "https://nano-editor.org"

[source]
url = "https://nano-editor.org/dist/v8/nano-${version}.tar.xz"
sha256 = "c17f43fc0e37336b33ee50a209c701d5beb808adc2d9f089ca831b40539c9ac4"

[dependencies]
build = ["musl", "ncurses"]
runtime = []  # Static linking, no runtime deps

[build]
configure = "./configure --prefix=/usr --enable-utf8 --disable-nls"
make = "make -j${JOBS}"
install = "make DESTDIR=${OUT} install"

[install]
bins = ["bin/nano"]
```

---

## Commands

```bash
# Build & install
levpkg install nano           # Build nano + deps, add to system
levpkg install nano vim       # Multiple packages

# Query
levpkg search editor          # Search recipes
levpkg info nano              # Show package info
levpkg list                   # List installed packages
levpkg deps nano              # Show dependency tree

# Maintenance
levpkg remove nano            # Uninstall package
levpkg update                 # Rebuild outdated packages
levpkg clean                  # Clear build cache

# Development
levpkg build nano             # Build but don't install
levpkg recipe nano            # Show recipe file
levpkg check nano             # Verify recipe builds
```

---

## Architecture

```
levpkg/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── recipe.rs         # Recipe parsing (TOML)
│   ├── resolver.rs       # Dependency resolution
│   ├── builder.rs        # Build execution
│   ├── cache.rs          # Content-addressed cache
│   ├── db.rs             # Installed package database
│   └── fetch.rs          # Source downloading
│
├── recipes/              # Package recipes
│   ├── core/             # Core system (musl, busybox, openrc)
│   ├── editors/          # nano, vim, etc.
│   ├── shells/           # ash, dash, bash
│   ├── network/          # curl, wget, openssh
│   └── dev/              # git, make, gcc
│
└── cache/                # Build cache (content-addressed)
    └── sha256-xxxx/      # Cached build outputs
```

---

## Implementation Phases

### Phase 0: APK Compatibility (Current Goal)
- Get Alpine's `apk` working in LevitateOS
- Learn from real-world package management
- Validate our init system works with packages

### Phase 1: Recipe System (~500 LOC)
- Parse TOML recipes
- Execute build steps
- No caching, no deps

### Phase 2: Dependency Resolution (~800 LOC)
- Topological sort of dependencies
- Build in correct order
- Detect cycles

### Phase 3: Caching (~400 LOC)
- Content-addressed storage
- Skip unchanged builds
- Cache invalidation

### Phase 4: Package Database (~400 LOC)
- Track installed packages
- Install/remove operations
- File ownership tracking

### Phase 5: Polish (~400 LOC)
- Search functionality
- Update checking
- Nice CLI output

**Total estimate**: ~2,500 LOC

---

## Comparison

| Feature | apk | apt | pacman | levpkg |
|---------|-----|-----|--------|--------|
| Language | C | C++ | C | Rust |
| Binary packages | Yes | Yes | Yes | No |
| Source packages | No | Separate | AUR | Yes |
| Static linking | Optional | No | No | Default |
| Complexity | Medium | High | Medium | Low |
| Package count | 14,000+ | 60,000+ | 13,000+ | ~50 (curated) |

---

## Non-Goals

- **Not a universal package manager**: Only for LevitateOS
- **Not binary distribution**: Always build from source
- **Not comprehensive**: Curated packages only
- **Not compatible with apk/apt/pacman**: Own format

---

## Open Questions

1. **Recipe repository**: Git repo? Separate from main repo?
2. **Versioning**: How to handle multiple versions?
3. **Patches**: How to apply distro-specific patches?
4. **Cross-compilation**: aarch64 support?
5. **Verification**: GPG signing of recipes?

---

## Inspiration

- [Cargo](https://github.com/rust-lang/cargo) - Rust's package manager
- [Brioche](https://brioche.dev/) - Nix-like, Rust, TypeScript recipes
- [pkgsrc](https://pkgsrc.org/) - NetBSD's source-based system
- [Portage](https://wiki.gentoo.org/wiki/Portage) - Gentoo's build system
- [apk-tools](https://gitlab.alpinelinux.org/alpine/apk-tools) - Alpine's package manager

---

## Timeline

| Milestone | Prerequisite | Est. Effort |
|-----------|--------------|-------------|
| APK working | Refactor complete | 1 week |
| Recipe parser | APK working | 2 days |
| Basic builder | Recipe parser | 3 days |
| Dependency resolver | Basic builder | 3 days |
| Caching | Dep resolver | 2 days |
| Package database | Caching | 3 days |
| **levpkg v1.0** | All above | **~3 weeks** |

---

## Next Steps

1. Complete refactor (TEAM_476)
2. Get APK working with LevitateOS
3. Document pain points with APK
4. Start Phase 1: Recipe parser
