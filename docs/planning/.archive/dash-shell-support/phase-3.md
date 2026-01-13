# Phase 3: Implementation - Dash Shell Support

## Implementation Overview

### Order of Implementation

1. **Kernel: wait3/wait4 syscalls** (prerequisite)
2. **Build system: musl sysroot support**
3. **Build system: C app registry**
4. **Build system: dash build command**
5. **Integration: initramfs inclusion**

## Step 1: Kernel wait3/wait4 Support

**Files:**
- `crates/kernel/src/syscall/mod.rs` - Add syscall numbers
- `crates/kernel/src/syscall/process.rs` - Implement handlers

**Implementation:**
```rust
// wait3(status, options, rusage) → wait4(-1, status, options, rusage)
// wait4(pid, status, options, rusage)
pub fn sys_wait4(
    pid: i32,
    status: *mut i32,
    options: i32,
    rusage: *mut Rusage,
) -> SyscallResult {
    // rusage can be null - if provided, fill with zeros for now
    let result = sys_waitpid(pid, status, options)?;
    if !rusage.is_null() {
        // Zero-fill rusage (we don't track resource usage yet)
        unsafe { (*rusage) = Rusage::zeroed(); }
    }
    Ok(result)
}
```

**Estimated units of work:** 3

## Step 2: musl Sysroot Build

**Files:**
- `xtask/src/build/musl.rs` (NEW)
- `xtask/src/build/mod.rs` - Add module

**Implementation:**
```rust
// xtask/src/build/musl.rs
const MUSL_REPO: &str = "https://git.musl-libc.org/git/musl";

pub fn build_musl_sysroot(arch: &str) -> Result<()> {
    // 1. Clone musl if not present
    clone_musl()?;

    // 2. Configure for target
    let target = match arch {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => bail!("Unsupported architecture"),
    };

    // 3. Build and install
    // CC=clang ./configure --prefix=../musl-sysroot --target=x86_64
    // make && make install

    Ok(())
}
```

**Estimated units of work:** 5

## Step 3: C App Registry

**Files:**
- `xtask/src/build/c_apps.rs` (NEW)

**Implementation:**
```rust
pub struct ExternalCApp {
    pub name: &'static str,
    pub repo: &'static str,
    pub binary: &'static str,
    pub configure_args: &'static [&'static str],
    pub required: bool,
}

impl ExternalCApp {
    pub fn build(&self, arch: &str) -> Result<()> {
        // 1. Ensure musl sysroot exists
        if !musl_sysroot_exists() {
            bail!("Run 'cargo xtask build musl-sysroot' first");
        }

        // 2. Clone if needed
        self.clone_repo()?;

        // 3. Run autoreconf if needed
        // 4. Configure with musl
        // 5. Make
        // 6. Copy binary to output

        Ok(())
    }
}
```

**Estimated units of work:** 4

## Step 4: Dash Build Command

**Files:**
- `xtask/src/build/commands.rs` - Add Dash variant
- `xtask/src/build/c_apps.rs` - Add dash to registry

**Implementation:**
```rust
// In c_apps.rs
pub static C_APPS: &[ExternalCApp] = &[
    ExternalCApp {
        name: "dash",
        repo: "https://git.kernel.org/pub/scm/utils/dash/dash.git",
        binary: "src/dash",
        configure_args: &[
            "--enable-static",
            "--disable-fnmatch",
            "--disable-glob",
        ],
        required: false,
    },
];
```

**Estimated units of work:** 3

## Step 5: Initramfs Integration

**Files:**
- `xtask/src/build/commands.rs` - Update create_initramfs

**Implementation:**
```rust
// In create_initramfs()
// After adding Rust apps, add C apps
for app in c_apps::C_APPS {
    if app.exists(arch) {
        let src = app.output_path(arch);
        std::fs::copy(&src, root.join(app.binary))?;
        println!("  📦 Added {} (C)", app.name);
    }
}
```

**Estimated units of work:** 1

## Design Reference

See [Phase 2](./phase-2.md) for:
- Architecture decisions (musl over c-gull for C)
- Directory structure
- Error handling patterns

## Dependencies

### External Dependencies
- `clang` - C compiler (already installed for kernel)
- `musl-dev` or musl headers (may need package install)
- `autoconf`, `automake` - For dash autoreconf

### Internal Dependencies
- Step 2 depends on Step 1 (kernel wait3 for dash job control)
- Step 3 depends on Step 2 (musl sysroot)
- Step 4 depends on Step 3 (C app registry)
- Step 5 depends on Step 4 (dash binary)

## Total Estimated Work

| Step | Units |
|------|-------|
| 1. Kernel wait3/wait4 | 3 |
| 2. musl sysroot | 5 |
| 3. C app registry | 4 |
| 4. Dash build | 3 |
| 5. Initramfs integration | 1 |
| **Total** | **16** |
