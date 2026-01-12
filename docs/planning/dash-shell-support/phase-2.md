# Phase 2: Design - Dash Shell Support

## Proposed Solution

### User-Facing Behavior
```bash
# Build dash shell
cargo xtask build dash

# Include dash in initramfs
# Automatically included when built, like brush

# Run dash interactively
$ dash
$ echo hello
hello
$ exit
```

### System Behavior
1. xtask clones dash source from kernel.org
2. xtask cross-compiles dash using musl-gcc
3. Built binary placed in `toolchain/dash-out/`
4. Binary included in initramfs alongside brush

## Architecture Decision: musl-libc for C Programs

### Why Not Extend c-gull?

c-gull is a Rust library, not a C library. Making it work for C would require:
- Generating C header files from Rust code
- Building crt0.o/crti.o/crtn.o startup objects
- Handling C-specific ABI quirks (varargs, struct passing, etc.)
- Significant ongoing maintenance burden

This is effectively reimplementing musl in Rust, which is not our goal.

### Why musl?

| Criterion | musl | glibc |
|-----------|------|-------|
| Static linking | Native support | Complex |
| Binary size | Small (~100KB) | Large (~2MB) |
| Dash proven | Alpine uses musl+dash | Yes but larger |
| Cross-compile | Simple | Complex |
| Dependencies | None | Many |

**Decision**: Use musl-libc for all C programs. Keep c-gull for Rust programs.

## Toolchain Design

### Directory Structure
```
toolchain/
├── c-ward/              # Existing (Rust libc)
├── libc-levitateos/     # Existing (c-gull wrapper)
├── sysroot/lib/libc.a   # Existing (Rust programs)
├── musl/                # NEW: musl source (cloned)
├── musl-sysroot/        # NEW: musl install prefix
│   ├── lib/libc.a       # musl static library
│   └── include/         # C headers
├── dash/                # NEW: dash source (cloned)
└── dash-out/            # NEW: built dash binary
```

### Build Flow

```
1. cargo xtask build musl-sysroot
   └── Clone musl → Configure → Build → Install to toolchain/musl-sysroot/

2. cargo xtask build dash
   └── Clone dash → ./configure with musl → make → Copy to dash-out/
```

## API Design

### New ExternalCApp Struct

```rust
/// A C application built against musl
pub struct ExternalCApp {
    pub name: &'static str,
    pub repo: &'static str,
    pub binary: &'static str,
    pub configure_args: &'static [&'static str],
    pub required: bool,
}

pub static C_APPS: &[ExternalCApp] = &[
    ExternalCApp {
        name: "dash",
        repo: "https://git.kernel.org/pub/scm/utils/dash/dash.git",
        binary: "dash",
        configure_args: &[
            "--enable-static",
            "--disable-fnmatch",  // Use internal implementation
            "--disable-glob",     // Use internal implementation
        ],
        required: false,  // Optional shell
    },
];
```

### New Build Commands

```rust
pub enum BuildCommands {
    // ... existing ...
    /// Build musl C library sysroot
    MuslSysroot,
    /// Build dash shell
    Dash,
}
```

## Data Model Changes

None required. This extends the build system, not kernel data structures.

## Behavioral Decisions

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| musl-sysroot missing when building dash | Error with message to run `cargo xtask build musl-sysroot` |
| Cross-compiler not installed | Error with install instructions |
| Dash clone fails | Retry with full clone (not shallow) |
| Dash configure fails | Show full configure output for debugging |
| wait3 called by dash | Return ENOSYS (dash should fall back to waitpid) |

### Error Handling

All build failures are fatal with clear messages:
```
Error: musl-sysroot not found.
Run 'cargo xtask build musl-sysroot' first.
```

### Defaults

- Architecture: inherit from `--arch` flag (default x86_64)
- Optimization: `-Os` for size
- Linking: static only (no dynamic linker)

## Design Alternatives Considered

### Alternative 1: Build dash with glibc
**Rejected**: glibc requires dynamic linking by default, and static glibc is complex.

### Alternative 2: Use cosmopolitan libc
**Rejected**: Experimental, unclear compatibility, adds complexity.

### Alternative 3: Rewrite dash in Rust
**Rejected**: Defeats the purpose of testing with external unmodified software.

### Alternative 4: Use busybox ash instead of dash
**Considered**: Could work, but busybox is larger and has more dependencies. Dash is simpler.

## Open Questions

### Q1: Cross-compiler choice?
**Options**:
1. `musl-gcc` wrapper (requires gcc + musl-dev on host)
2. `x86_64-linux-musl-gcc` (standalone toolchain)
3. `clang` with musl sysroot (llvm-based, more portable)

**Recommended**: Option 3 (clang) - already installed for kernel builds, more portable.

### Q2: How to handle wait3/wait4?

Dash's `jobs.c` uses `wait3()` for rusage information. Options:
1. **Stub in kernel**: Return ENOSYS, hope dash falls back
2. **Patch dash**: Modify to use waitpid instead (violates "no modification" rule)
3. **Implement wait3**: Add to kernel syscall table

**Recommended**: Option 3 - implement wait3/wait4 in kernel. It's a standard syscall we'll eventually need anyway.

### Q3: Where to store musl source?

Options:
1. `toolchain/musl/` - alongside c-ward
2. Git submodule
3. Download tarball on demand

**Recommended**: Option 1 (clone on demand) - matches coreutils/brush pattern.

### Q4: CI considerations?

Need to install:
- clang (already present for kernel)
- musl headers (may need package)

### Q5: aarch64 support?

Need musl aarch64 cross-compile. Same pattern, different target triple.
- x86_64: `x86_64-linux-musl`
- aarch64: `aarch64-linux-musl`

## Shell Tier System

With dash added, we have a clear progression:

| Tier | Shell | Use Case |
|------|-------|----------|
| T0 | None | Pure syscall tests via init |
| T1 | dash | Basic shell tests, simple commands |
| T2 | brush | Full bash compatibility |

Testing progression:
1. Get T1 working (dash boots, runs commands)
2. Debug any kernel issues at T1 level
3. Move to T2 (brush) with confidence kernel is correct
