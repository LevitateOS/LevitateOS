
# Userspace Development Guide

> **TEAM_090**: Documented build process and linker workarounds for userspace applications.

## 1. Linker Script Conflict Workaround

**Problem:**
The root `.cargo/config.toml` adds `-Tlinker.ld` to rustflags for ALL `aarch64-unknown-none` builds (intended for the kernel). Userspace applications typically use their own `link.ld` (mapped at 0x10000). Cargo merges these flags, causing the linker to attempt using BOTH scripts, resulting in overlapping sections or "Cannot allocate memory" errors.

**Solution:**
Every userspace crate (e.g., `userspace/shell`, `userspace/hello`) **MUST** include an empty `linker.ld` file in its root directory.

File: `userspace/<app>/linker.ld`
```ld
/* TEAM_090: Empty linker script to satisfy root config's -Tlinker.ld */
/* This is intentionally empty - the actual linker script is link.ld */
/* to prevent conflicts with the kernel's linker.ld settings */
```

Config: `userspace/<app>/.cargo/config.toml`
```toml
[target.aarch64-unknown-none]
rustflags = ["-C", "link-arg=-Tlink.ld"]
```

## 2. Syscall ABI

> [!WARNING]
> **Legacy ABI Note**: The current kernel (Phase 8/9) implementation uses a simple custom syscall ABI (Read=0, Write=1, etc.). This is considered **DEPRECATED** and is being replaced by the Linux-compatible ABI in Phase 10.

For the finalized target ABI, see [userspace-abi.md](file:///home/vince/Projects/LevitateOS/docs/specs/userspace-abi.md).

### Legacy ABI (Current MVP)
- **Instruction:** `svc #0`
- **Syscall Number:** `x8` register
- **Arguments:** `x0` - `x5`
- **Return Value:** `x0`

| Syscall | NR | Description |
|---------|----|-------------|
| Read | 0 | Read from fd 0 |
| Write | 1 | Write to fd 1/2 |
| Exit | 2 | Terminate process |
| GetPid | 3 | Get current PID |
| Sbrk | 4 | Adjust heap |
| Spawn | 5 | Spawn program |
| Exec | 6 | Replace process |
| Yield | 7 | Yield CPU |

### Implementation Example
```rust
pub fn exit(code: i32) -> ! {
    unsafe {
        core::arch::asm!(
            "svc #0",
            in("x8") 2, // SYS_EXIT
            in("x0") code,
            options(noreturn, nostack)
        );
    }
}
```

## 3. Building and Packaging

Userspace apps are built separately and packaged into the initramfs.

### Step 1: Build the App
```bash
cd userspace/shell
cargo build --release
```
Output binary: `target/aarch64-unknown-none/release/shell` (ELF)

### Step 2: Package into Initramfs
Use `cpio` to create the archive.

```bash
mkdir -p build/initramfs
cp userspace/shell/target/.../release/shell build/initramfs/lsh
cd build/initramfs
find . | cpio -o --format=newc > ../../initramfs.cpio
```

### Step 3: Run from Kernel
The kernel launches userspace programs using `run_from_initramfs`:

```rust
// kernel/src/main.rs
task::process::run_from_initramfs("lsh", &archive);
```
