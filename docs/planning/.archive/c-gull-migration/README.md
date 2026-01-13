# C-Gull Migration Plan: Removing Eyra Dependency

**Created**: 2026-01-11
**Status**: Planning
**Goal**: Run unmodified uutils/coreutils without Eyra

---

## Current Architecture (Eyra)

```
┌─────────────────────────────────────────────────────────────┐
│                    sunfishcode/coreutils                     │
│            (forked uutils with Eyra integration)            │
├─────────────────────────────────────────────────────────────┤
│   Cargo.toml: std = { package = "eyra", version = "0.22" }  │
├─────────────────────────────────────────────────────────────┤
│                         Eyra                                 │
│    (Rust std replacement - makes syscalls directly)         │
├─────────────────────────────────────────────────────────────┤
│                    Linux Syscalls                            │
├─────────────────────────────────────────────────────────────┤
│                    LevitateOS Kernel                         │
└─────────────────────────────────────────────────────────────┘
```

**Problem**: Every app needs `std = { package = "eyra" }` in Cargo.toml

---

## Target Architecture (c-gull)

```
┌─────────────────────────────────────────────────────────────┐
│              uutils/coreutils (UNMODIFIED)                   │
│                   No source changes needed                   │
├─────────────────────────────────────────────────────────────┤
│                     Rust std (stock)                         │
│                  Links against "libc"                        │
├─────────────────────────────────────────────────────────────┤
│                    c-gull (as libc)                          │
│     Provides C ABI libc functions, backed by Rust            │
├─────────────────────────────────────────────────────────────┤
│                    Linux Syscalls                            │
├─────────────────────────────────────────────────────────────┤
│                    LevitateOS Kernel                         │
└─────────────────────────────────────────────────────────────┘
```

**Benefit**: Any Linux program just works - no modifications needed

---

## Understanding the Projects

| Project | What It Is | Use Case |
|---------|-----------|----------|
| **Eyra** | Rust std replacement | Rust programs (requires Cargo.toml change) |
| **c-gull** | Rust libc implementation | C programs or libc-based std |
| **c-ward** | Parent project of c-gull | Contains c-gull + c-scape |
| **c-scape** | Low-level libc subset | Used by c-gull |
| **origin** | Program startup in Rust | Replaces crt1.o |
| **Mustang** | Build system for c-gull | Custom targets + build-std |

**Key Insight**: Eyra internally uses c-gull! They share the same syscall backend.

---

## Syscall Coverage Status

**cgull-test** (in `crates/userspace/eyra/cgull-test/`) tests the syscalls both Eyra and c-gull need.

### Test Results: 19/19 PASS

| Tier | Syscalls | Status |
|------|----------|--------|
| Basic I/O | write, writev | PASS |
| Memory | brk, mmap, munmap | PASS |
| Time | clock_gettime, nanosleep | PASS |
| Random | getrandom | PASS |
| Process | getpid, getuid | PASS |
| Environment | args, env, getcwd | PASS |
| Files | open, read, close, stat, mkdir, readdir | PASS |
| Pipes | pipe2 | PASS |
| Signals | sigprocmask | PASS |

### How to Run cgull-test

```bash
# 1. Build the test binary (if not already built)
cd crates/userspace/eyra
cargo build --release --target x86_64-unknown-linux-gnu -p cgull-test

# 2. Copy to initramfs (should already be there)
cp target/x86_64-unknown-linux-gnu/release/cgull-test ../../initrd_root/

# 3. Rebuild initramfs
cargo xtask build initramfs

# 4. Run in VM and execute test
cargo xtask run
# At shell prompt: cgull-test
```

Expected output:
```
╔══════════════════════════════════════════════════════════════╗
║        C-GULL / EYRA SYSCALL COMPATIBILITY TEST              ║
╚══════════════════════════════════════════════════════════════╝

── TIER 1: Basic I/O ──────────────────────────────────────────
[PASS] write() - you're reading this
[PASS] writev() - println! works
...
╔══════════════════════════════════════════════════════════════╗
║                         SUMMARY                              ║
╠══════════════════════════════════════════════════════════════╣
║  Passed:  19                                                 ║
║  Failed:   0                                                 ║
║  Total:   19                                                 ║
╚══════════════════════════════════════════════════════════════╝

🎉 All tests passed! LevitateOS is ready for c-gull programs.
```

---

## Migration Path: Pre-built Sysroot

**The only approach that requires ZERO source modifications.**

See [SYSROOT_BUILD.md](SYSROOT_BUILD.md) for detailed implementation plan.

### How It Works

1. **Build c-gull as libc.a** - Static library providing all libc symbols
2. **Create custom target spec** - `x86_64-levitateos.json` with linker config
3. **Build std from source** - Using `-Z build-std` against our libc.a
4. **Package as sysroot** - Distributable directory with pre-compiled std
5. **Build any program** - Just specify `--target` and `--sysroot`

```bash
# Build UNMODIFIED uutils/coreutils
git clone https://github.com/uutils/coreutils
cd coreutils
cargo +nightly build --release \
    --target x86_64-levitateos \
    --sysroot $LEVITATEOS_SYSROOT

# That's it. No Cargo.toml changes. No macros. Nothing.
```

### Comparison

| Approach | Source Changes | Build Command Changes |
|----------|---------------|----------------------|
| **Eyra** | Cargo.toml modification | None |
| **Mustang** | One macro in main.rs | `--target`, `-Z build-std` |
| **Pre-built Sysroot** | **NONE** | `--target`, `--sysroot` |

### How Redox Does It

Redox OS uses this exact approach:
- [relibc](https://github.com/redox-os/relibc) - Their Rust libc implementation
- `x86_64-unknown-redox` - Official Rust target
- Programs built for Redox **automatically link against relibc**

We're doing the same thing, but with c-gull instead of relibc, and a custom target instead of an upstream one.

---

## Implementation Phases

### Phase 1: Verify Syscall Coverage (DONE)
- [x] cgull-test passes 19/19
- [x] All Eyra-required syscalls implemented

### Phase 2: Build c-gull as libc.a
- [ ] Configure c-gull for staticlib output
- [ ] Include origin startup code
- [ ] Test linking a minimal program

### Phase 3: Create Target Spec + Sysroot
- [ ] Write `x86_64-levitateos.json`
- [ ] Build std with `-Z build-std`
- [ ] Package sysroot

### Phase 4: Build Original Coreutils
- [ ] Clone upstream uutils/coreutils
- [ ] Build with our sysroot
- [ ] Test on LevitateOS

### Phase 5: Automation
- [ ] `cargo xtask build sysroot` command
- [ ] CI/CD for sysroot builds
- [ ] Remove Eyra from tree

---

## Files to Remove (After Migration)

Once c-gull/Mustang migration is complete:

```
crates/userspace/eyra/
├── cgull-test/        # KEEP - useful for testing
├── coreutils/         # REPLACE with Mustang-built version
├── brush/             # UPDATE to use Mustang
├── eyra-hello/        # REMOVE - no longer needed
├── eyra-test-runner/  # REMOVE
├── libsyscall/        # KEEP - raw syscalls still useful
├── libsyscall-tests/  # KEEP
└── syscall-conformance/ # KEEP
```

---

## References

- [c-ward](https://github.com/sunfishcode/c-ward) - Rust libc implementation
- [c-gull](https://github.com/sunfishcode/c-ward/tree/main/c-gull) - libc ABI layer
- [Eyra](https://github.com/sunfishcode/eyra) - Rust std replacement (uses c-gull)
- [Mustang](https://github.com/sunfishcode/mustang) - Build system for c-gull programs
- [origin](https://github.com/sunfishcode/origin) - Program startup in Rust

---

## Team Log

| Date | Team | Action |
|------|------|--------|
| 2026-01-11 | TEAM_432 | Created migration plan document |
