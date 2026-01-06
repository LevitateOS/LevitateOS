# LevitateOS Architecture

> ⚠️ **CURRENT STATE (2026-01-05):** System boots to an interactive shell.

**TEAM_009: Workspace Refactoring**

LevitateOS uses a modular **Cargo Workspace** structure, inspired by **Tock OS** and **Redox**. This ensures clear separation of concerns between core kernel logic, hardware abstraction, and shared utilities.

## Workspace Structure

The project root defines the workspace members in `Cargo.toml`:

```toml
[workspace]
members = [
    "kernel",
    "crates/hal",
    "crates/utils",
    "crates/term",
    "crates/virtio",
    "crates/pci",
    "crates/gpu",
    "crates/error",
    "xtask",
]
```

### 1. Core Kernel (`levitate-kernel`)
- **Location**: `kernel/`
- **Purpose**: High-level OS logic, task scheduling, memory management, and device coordination.
- **Dependencies**: Depends on `los_hal`, `los_utils`, and other `los_*` crates.
- **Note**: This is the binary crate (`main.rs`) that produces the kernel executable.

### 2. Library Crates (`crates/`)

All library crates use the `los_` prefix:

| Crate | Location | Purpose |
|-------|----------|----------|
| `los_hal` | `crates/hal/` | Hardware abstraction (GIC, MMU, Timer, UART, VirtIO HAL) |
| `los_utils` | `crates/utils/` | Shared primitives (Spinlock, RingBuffer, CPIO) |
| `los_term` | `crates/term/` | ANSI terminal emulator |
| `los_virtio` | `crates/virtio/` | VirtIO transport layer |
| `los_pci` | `crates/pci/` | PCI bus enumeration |
| `los_gpu` | `crates/gpu/` | VirtIO GPU driver |
| `los_error` | `crates/error/` | Error handling macros |

## Build System

- **Toolchain**: `aarch64-unknown-none`
- **Runner**: `run.sh`
  - Builds the workspace (`cargo build --release`).
  - Extracts the binary from `target/aarch64-unknown-none/release/levitate-kernel`.
  - Converts it to a raw binary (`objcopy`).
  - Launches QEMU with specific device flags (`-device virtio-gpu`, etc.).

## Gotchas & Notes

- **Strict Alignment**: AArch64 requires strict alignment. We use `strict-align` target feature (or similar) where possible, but `levitate-utils` may generate warnings about it being unstable.
- **QEMU Bus**: VirtIO devices in QEMU (legacy/MMIO) are order-sensitive or specific to the command line arguments. Check `run.sh` vs `virtio.rs` scanning logic if devices aren't found.
- **External Kernels**: Reference implementations are stored in `.external-kernels/` which is excluded from VS Code analysis to improve performance.

## Error Handling

LevitateOS uses typed error enums with numeric codes for debugging.

### Defining New Error Types

Use the `define_kernel_error!` macro for error types:

```rust
use los_error::define_kernel_error;

define_kernel_error! {
    /// My subsystem errors (0x10xx)
    pub enum MyError(0x10) {
        /// Something went wrong
        SomethingWrong = 0x01 => "Something went wrong",
        /// Nested error example
        Other(InnerError) = 0x02 => "Nested error occurred",
    }
}
```

### Error Code Format

```
0xSSCC where:
  SS = Subsystem (e.g., 0x01 for MMU, 0x03 for Spawn)
  CC = Error code within subsystem (01-FF)
```

### Subsystem Allocation

See `docs/planning/error-macro/phase-1.md` for the current subsystem list.

## Userspace & ABI

LevitateOS is transitioning from a minimal custom syscall ABI to full **Linux AArch64 ABI Compatibility** (Phase 10). This strategy enables the use of the standard Rust library (`std`) and existing UNIX toolchains. For the definitive target specification, see [userspace-abi.md](file:///home/vince/Projects/LevitateOS/docs/specs/userspace-abi.md).
