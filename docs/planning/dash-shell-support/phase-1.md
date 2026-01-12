# Phase 1: Discovery - Dash Shell Support

## Feature Summary

### Problem
Brush (bash-compatible Rust shell) is complex and makes debugging kernel issues difficult. When brush fails, it's hard to know if the problem is in the kernel's syscall implementation or in brush's complexity.

### Who Benefits
- Kernel developers debugging syscall implementations
- Anyone testing new kernel features incrementally
- CI/CD pipelines that need fast, reliable shell tests

### Why Dash
| Shell | Language | Lines of Code | Dependencies | Complexity |
|-------|----------|---------------|--------------|------------|
| dash  | C        | ~10-15k       | libc only    | Low        |
| brush | Rust     | ~50k+         | Many crates  | High       |
| bash  | C        | ~150k+        | readline, etc| Very High  |

Dash is the Debian `/bin/sh` - proven, minimal, POSIX-compliant.

## Success Criteria

1. **Build**: `cargo xtask build dash` compiles dash for LevitateOS
2. **Boot**: Dash starts and displays prompt when run on LevitateOS
3. **Execute**: Dash can execute `/coreutils echo hello`
4. **Pipes**: Dash can execute `echo hello | cat`
5. **Test Parity**: Any test that passes with dash should pass with brush

## Current State Analysis

### Toolchain Status
- **c-gull sysroot**: Builds `libc.a` for Rust programs (via Eyra pattern)
- **apps.rs registry**: Builds Rust crates (coreutils, brush) against sysroot
- **C support**: NOT IMPLEMENTED - no way to build C programs

### c-gull libc Coverage for Dash

| Function Group | Required by Dash | c-gull Status |
|----------------|------------------|---------------|
| Process: fork, exec, wait, waitpid | Yes | IMPLEMENTED |
| Process: wait3, wait4 | Yes (job control) | TODO STUB |
| Signals: sigaction, sigprocmask | Yes | IMPLEMENTED |
| Terminal: tcgetpgrp, tcsetpgrp | Yes | IMPLEMENTED |
| Terminal: isatty | Yes | IMPLEMENTED |
| Jump: setjmp, longjmp | Yes (error handling) | IMPLEMENTED |
| File: open, read, write, close | Yes | IMPLEMENTED |
| File: dup, dup2, pipe | Yes | IMPLEMENTED |
| Kill: kill, killpg | Yes | IMPLEMENTED |
| Pattern: glob, fnmatch | Optional | NOT CHECKED |

### Key Problem

**c-gull is not a C libc.** It's a Rust libc implementation that exposes the Linux syscall ABI to Rust programs. It does NOT provide:
- C header files (`<stdio.h>`, `<unistd.h>`, etc.)
- A traditional `libc.a` that C compilers can link against
- The C runtime startup (`crt0.o`, `crti.o`, etc.)

## Codebase Reconnaissance

### Files Affected
- `xtask/src/build/apps.rs` - Would need C program support
- `xtask/src/build/commands.rs` - New `Dash` command
- `xtask/src/build/mod.rs` - Module exports
- `toolchain/` - New musl or C toolchain files

### APIs/Patterns to Follow
- `ExternalApp` struct pattern for consistency
- Clone-on-demand pattern (like coreutils, brush)
- Fail-fast with helpful error messages

### Tests Affected
- Behavior tests may need shell-agnostic variants
- New dash-specific smoke tests needed

## Constraints

1. **No bespoke shells** - Must use unmodified upstream dash
2. **C toolchain required** - Need cross-compiler + libc
3. **musl preferred** - Proven dash compatibility, static linking
4. **wait3/wait4 missing** - May need to implement or work around

## Open Questions for Phase 2

1. **Libc choice**: Use musl-libc for C programs, or invest in c-gull C export?
2. **Toolchain**: Which cross-compiler? `x86_64-linux-musl-gcc` or clang?
3. **Static vs dynamic**: Static linking only (no ld.so)?
4. **wait3 workaround**: Can dash be patched to use waitpid, or must we implement wait3?
