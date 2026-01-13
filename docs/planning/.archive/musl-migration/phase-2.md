# Phase 2: Design - Migrate from c-gull to musl

## Proposed Solution

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    LevitateOS Kernel                     │
│                  (Linux syscall ABI)                     │
└─────────────────────────────────────────────────────────┘
                            ▲
                            │ syscalls
                            │
┌─────────────────────────────────────────────────────────┐
│                      musl libc                           │
│            (one libc for everything)                     │
└─────────────────────────────────────────────────────────┘
                            ▲
              ┌─────────────┴─────────────┐
              │                           │
    ┌─────────┴─────────┐       ┌────────┴────────┐
    │   Rust Programs    │       │   C Programs    │
    │ (coreutils, brush) │       │  (dash, etc)    │
    │                    │       │                 │
    │ --target musl      │       │ CC=musl-gcc     │
    └────────────────────┘       └─────────────────┘
```

### musl Sysroot Strategy

**Option A: Use system musl packages**
```bash
# Fedora
sudo dnf install musl-devel musl-gcc

# Ubuntu/Debian
sudo apt install musl-dev musl-tools
```
- Pro: No build step, always up to date
- Con: Requires system packages, version varies

**Option B: Build musl from source**
```bash
git clone https://git.musl-libc.org/git/musl
cd musl && ./configure --prefix=../musl-sysroot && make install
```
- Pro: Reproducible, controlled version
- Con: Build step, maintenance

**Option C: Download prebuilt musl toolchain**
```bash
# From musl.cc
curl -O https://musl.cc/x86_64-linux-musl-cross.tgz
```
- Pro: Fast, includes cross-compiler
- Con: External dependency, large download

**Recommended: Option A (system packages) with Option B fallback**

For CI and most developers, system packages work. For reproducible builds or exotic setups, build from source.

## Build System Design

### New apps.rs (Simplified)

```rust
impl ExternalApp {
    pub fn build(&self, arch: &str) -> Result<()> {
        let target = musl_target(arch);  // "x86_64-unknown-linux-musl"

        // That's it. No special flags.
        let status = Command::new("cargo")
            .current_dir(self.clone_dir())
            .args([
                "build",
                "--release",
                "--target", target,
                "-p", self.package,
            ])
            .status()?;

        // ... copy binary ...
    }
}

fn musl_target(arch: &str) -> &'static str {
    match arch {
        "x86_64" => "x86_64-unknown-linux-musl",
        "aarch64" => "aarch64-unknown-linux-musl",
        _ => panic!("unsupported arch"),
    }
}
```

**What we removed:**
- `+nightly-2025-04-28`
- `-Z build-std=std,panic_abort`
- `-Z build-std-features=panic_immediate_abort`
- All the RUSTFLAGS (panic, relocation-model, link-args)
- `env_remove("RUSTUP_TOOLCHAIN")`

### New c_apps.rs (C Programs)

```rust
pub struct ExternalCApp {
    pub name: &'static str,
    pub repo: &'static str,
    pub binary: &'static str,
    pub configure_args: &'static [&'static str],
}

impl ExternalCApp {
    pub fn build(&self, arch: &str) -> Result<()> {
        self.clone_repo()?;

        let cc = format!("musl-gcc");  // or clang with musl target

        // Standard autotools build
        Command::new("./configure")
            .current_dir(self.clone_dir())
            .env("CC", &cc)
            .args(self.configure_args)
            .status()?;

        Command::new("make")
            .current_dir(self.clone_dir())
            .status()?;

        // Copy binary...
    }
}

pub static C_APPS: &[ExternalCApp] = &[
    ExternalCApp {
        name: "dash",
        repo: "https://git.kernel.org/pub/scm/utils/dash/dash.git",
        binary: "src/dash",
        configure_args: &["--enable-static"],
    },
];
```

### New sysroot.rs (Much Simpler)

```rust
/// Check if musl target is available
pub fn check_musl_target(arch: &str) -> Result<()> {
    let target = musl_target(arch);

    // Check if rustup has the target
    let output = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()?;

    let installed = String::from_utf8_lossy(&output.stdout);
    if !installed.contains(target) {
        println!("Installing musl target...");
        Command::new("rustup")
            .args(["target", "add", target])
            .status()?;
    }

    Ok(())
}

/// Check if musl-gcc is available for C programs
pub fn check_musl_gcc() -> Result<()> {
    if Command::new("musl-gcc").arg("--version").output().is_err() {
        bail!(
            "musl-gcc not found. Install with:\n\
             Fedora: sudo dnf install musl-gcc\n\
             Ubuntu: sudo apt install musl-tools"
        );
    }
    Ok(())
}

// The old build_sysroot() is gone - we use system musl
pub fn sysroot_exists() -> bool {
    true  // System musl, always "exists"
}
```

## Behavioral Decisions

### Edge Cases

| Scenario | Behavior |
|----------|----------|
| musl target not installed | Auto-install via rustup |
| musl-gcc not installed | Error with install instructions |
| Cross-compile aarch64 on x86_64 | Use musl cross toolchain |
| Build fails | Show full compiler output |

### Defaults

- **Linking**: Static (musl default)
- **Optimization**: Release mode (-O2)
- **Target**: Match host arch unless --arch specified

## Migration Path

### Breaking Changes

1. **`cargo xtask build sysroot`** - No longer needed, becomes no-op or removed
2. **toolchain/sysroot/lib/libc.a** - No longer exists (use system musl)
3. **Nightly requirement** - May be able to use stable Rust!

### Compatibility

- **Kernel**: No changes needed (still Linux ABI)
- **Behavior tests**: Should pass unchanged
- **Initramfs**: Same binaries, different build path

## Alternatives Considered

### Alternative 1: Keep c-gull for Rust, add musl for C only
**Rejected**: Two libcs, complexity, "why not just use musl for both?"

### Alternative 2: Make c-gull export C headers
**Rejected**: Massive undertaking, essentially reimplementing musl in Rust

### Alternative 3: Use glibc
**Rejected**: glibc static linking is painful, larger binaries

### Alternative 4: Use cosmopolitan libc
**Considered**: Interesting for portable binaries, but experimental and adds complexity

## Open Questions (RESOLVED)

### Q1: Stable Rust?
~~With musl target, we might not need nightly. Should we try stable first?~~
**DECISION**: Keep nightly. Other parts of the project depend on nightly (kernel build).

### Q2: System musl vs vendored?
**DECISION**: System packages. Document build-from-source for reproducibility.

### Q3: musl-gcc vs clang?
**DECISION**: musl-gcc for simplicity, widely available.

### Q4: What about aarch64 cross-compilation?
**DECISION**: Document both musl-gcc cross and clang paths, test in CI.
