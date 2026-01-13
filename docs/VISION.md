# LevitateOS Vision

## 🎯 Mission Statement

**LevitateOS is a General Purpose Unix-Compatible Operating System.**

To build a **modern, secure, and performant** operating system written in Rust that can **run any Unix program without modification**.

### What "General Purpose" Means

A General Purpose OS lets **arbitrary users run arbitrary programs** they didn't write:

| Requirement | Description |
|-------------|-------------|
| **No Source Modification** | Programs compiled for Linux just work |
| **Standard ABI** | Linux syscall interface, not a custom ABI |
| **libc Compatibility** | Provide libc.so that existing binaries link against |
| **POSIX Semantics** | fork, exec, pipes, signals, file descriptors work as expected |

**The Test**: Can a user download a Linux binary and run it? If yes, we're general purpose.

### What We Are NOT

- ❌ NOT a hobby/educational OS (we aim for production use)
- ❌ NOT an embedded/single-purpose OS (we run arbitrary programs)
- ❌ NOT a research OS (we prioritize compatibility over novelty)

LevitateOS aims to prove that a clean-slate kernel, built with modern language guarantees (Rust), can support the vast existing ecosystem of Linux applications without sacrificing safety or architectural integrity.

## 🏛️ Core Principles

1. **Safety by Default**: Leverage Rust's ownership and type system to enforce memory safety and eliminate entire classes of bugs (e.g., Use-After-Free, Data Races) at compile time.
2. **Linux ABI Compatibility**: Prioritize compatibility with the Linux system call interface. This allows running unmodified Linux binaries (starting with static Rust applications like `uutils`) and enables the use of the standard Rust `std` library.
3. **Modern Pure-Rust Userspace**: Utilize the [Eyra](https://github.com/sunfishcode/eyra) ecosystem (via `rustix` and `linux-raw-sys`) to provide a Linux-compatible runtime that is entirely C-free.
4. **Modular "Worse is Better" Architecture**: Prioritize simple, verifiable implementations over "perfect" but complex ones. Follow the rule of simplicity (Rule 20).
5. **Silence is Golden**: The kernel should be silent in success and loud in failure (Rule 4).
6. **Modern Hardware First**: Targets modern architectures (AArch64, x86_64) and hardware (Pixel 6, Intel NUC) with a focus on energy efficiency and scalability.

## 🚀 Long-Term Goal

**Run any Unix program without modification.**

This breaks down into concrete milestones:

1. ✅ Linux syscall ABI compatibility (in progress)
2. ✅ Static libc support via musl (TEAM_444)
3. 🔲 Dynamic linker (ld-linux.so equivalent)
4. 🔲 Full POSIX compliance for common utilities

## 🛠️ Strategy

### Path to General Purpose

| Phase | Goal | Status |
|-------|------|--------|
| Foundation (1-14) | HAL, MMU, Multitasking, VFS | ✅ Complete |
| Compatibility (15-17) | Linux syscall layer, TTY | 🟡 In Progress |
| **Static libc** | musl libc (static linking) | 🟢 Complete (TEAM_444) |
| **Dynamic Linker** | ld-linux.so.2 equivalent | 🔲 Next Milestone |
| Security (18-20) | Identity, authentication, hardening | 🔲 Future |

### The libc Strategy

**Current (musl static)**: Static musl binaries work now!
- Rust programs: `--target x86_64-unknown-linux-musl`
- C programs: `musl-gcc`

```
Static Linux Binary (musl) → Linux syscalls → LevitateOS kernel
```

**Next milestone**: Dynamic linker for dynamically-linked binaries.
