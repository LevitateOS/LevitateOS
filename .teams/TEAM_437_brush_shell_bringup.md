# TEAM_437: Brush Shell Bringup

## Objective

Get the brush shell (POSIX-compatible shell) running on LevitateOS so users can run commands.

## Status: IN PROGRESS - BLOCKED

**Blocking Issue**: Brush crashes immediately on startup with INVALID OPCODE (ud2) before making any syscalls.

## Context

- LevitateOS aims to be a general-purpose Unix-compatible OS
- User explicitly stated: "NO BRUSH!! NO USERS!!! THIS IS A GENERAL PURPOSE OS!!!"
- Brush is critical infrastructure - no shell means the OS is unusable for interactive work

## Progress Summary

### What Was Done

1. **Added missing libc stubs to c-gull** (`toolchain/libc-levitateos/src/lib.rs`):
   - `getpwnam_r()` - returns ENOENT (no users in LevitateOS)
   - `getgrgid_r()` - returns ENOENT (no groups)
   - `getgrouplist()` - returns single group (primary gid)
   - `ttyname_r()` - returns ENOENT

2. **Fixed coreutils to be PIE** (`xtask/src/build/external.rs`):
   - Changed `-C link-arg=-static` to `-C link-arg=-static-pie`
   - Added `-C relocation-model=pic`
   - Coreutils now loads correctly at kernel's 0x10000 base address

3. **Added scheduler affinity syscalls** (for sysinfo crate used by brush):
   - `sys_sched_getaffinity()` - returns single-CPU mask
   - `sys_sched_setaffinity()` - accepts any mask (single-CPU system)
   - Added to `crates/kernel/syscall/src/process/sched.rs`
   - Added dispatcher entries in `crates/kernel/syscall/src/lib.rs`
   - Added to both x86_64 and aarch64 arch modules

4. **Init process updated** (`crates/userspace/init/src/main.rs`):
   - Changed to spawn "brush" instead of the no_std shell
   - Brush spawns successfully as PID 2

### Current Crash Analysis

**Crash Details**:
- Location: VA 0x6b6555 (brush binary loaded at 0x10000, crash at +0x6a6555 file offset)
- Exception: INVALID OPCODE (ud2 instruction)
- Pattern: Syscall + test rax + jnz + ud2 panic handler

**Key Observation**:
- Brush makes ZERO syscalls before crashing
- Init syscalls are logged (spawn, write, waitpid)
- No syscalls from PID 2 (brush) appear in logs
- The crash happens during early initialization, before any syscall

**Disassembly at crash site** (file offset 0x6a5548):
```asm
mov eax, 0xa        ; syscall 10 = mprotect
mov edx, 1          ; prot = PROT_READ
syscall
test rax, rax
jnz success         ; if rax != 0, skip panic
ud2                 ; rax == 0, panic
```

**Contradiction**:
- The crash site has a mprotect syscall
- Our mprotect returns 0 on success (correct Linux behavior)
- But NO mprotect syscall appears in the syscall log
- This means brush crashes BEFORE reaching the syscall instruction

## What Needs Investigation

1. **Why is brush crashing before any syscalls?**
   - The crash site shows syscall code but it's never executed
   - Something earlier is jumping directly to the ud2 panic
   - Possible causes:
     - Initialization failure in origin/c-gull startup
     - TLS setup issue
     - Relocation failure
     - Stack issue

2. **Entry point to crash location analysis**:
   - Entry: 0x69d56c
   - Crash: 0x6b6555
   - Distance: ~128KB of code
   - Need to trace execution path

3. **Possible test**: Run brush under strace on a real Linux system to see what syscalls it makes at startup. Compare with what LevitateOS logs.

## Files Modified in This Session

| File | Change |
|------|--------|
| `toolchain/libc-levitateos/src/lib.rs` | Added user/group stub functions |
| `xtask/src/build/external.rs` | Changed to static-pie linking |
| `crates/kernel/syscall/src/process/sched.rs` | NEW: scheduler affinity syscalls |
| `crates/kernel/syscall/src/process/mod.rs` | Added sched module |
| `crates/kernel/syscall/src/lib.rs` | Added dispatcher entries + debug logging |
| `crates/kernel/syscall/src/mm.rs` | Added mprotect debug print (remove when done) |
| `crates/kernel/arch/x86_64/src/lib.rs` | Added SchedSetaffinity/SchedGetaffinity |
| `crates/kernel/arch/aarch64/src/lib.rs` | Added SchedSetaffinity/SchedGetaffinity |
| `crates/userspace/init/src/main.rs` | Changed to spawn brush |

## Debug Code to Remove

When the issue is resolved, remove these debug additions:
1. In `syscall/src/lib.rs`: Remove the `log::info!` syscall logging before/after dispatch
2. In `syscall/src/mm.rs`: Remove the `los_hal::println!` in sys_mprotect

## Next Steps for Future Team

1. **Investigate brush startup code path**:
   - What does origin (c-gull's startup code) do before main()?
   - Where is the conditional jump that lands on ud2?
   - Use addr2line or objdump to trace back from crash

2. **Compare with Linux**:
   - Run brush on real Linux with strace to see startup syscalls
   - Example: `strace -f ./brush -c 'exit 0'`

3. **Simplify test**:
   - Try running hello-cgull (simpler c-gull test program)
   - If it works, brush issue is brush-specific
   - If it crashes similarly, issue is in c-gull/origin startup

4. **Check relocation**:
   - Verify PIE relocation is working correctly
   - Check if any relocations point to unmapped memory

## Handoff Notes

- Brush is built and in initramfs (7.3MB static-pie binary)
- Init spawns brush successfully as PID 2
- Brush crashes immediately with ud2 before making any syscalls
- The syscall dispatch logging works (init syscalls are logged)
- The crash appears to be in brush's early initialization, not kernel
- Need fresh perspective on why code jumps to ud2 without making syscall

## Related Team Files

- TEAM_435: Removed Eyra, set up c-gull sysroot
- TEAM_433/434: Planned the Eyra removal
