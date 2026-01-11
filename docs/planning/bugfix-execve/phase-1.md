# Phase 1: Understanding and Scoping

**Bug**: `execve()` syscall is a stub returning ENOSYS  
**Team**: TEAM_436  
**Created**: 2026-01-11  
**Status**: Ready for Implementation

---

## Bug Summary

The `sys_exec()` function in `crates/kernel/syscall/src/process/lifecycle.rs:200-217` is a stub that returns `ENOSYS`. This means:

- The Linux-standard `execve()` syscall does not work
- fork()+exec() pattern fails - programs cannot spawn other programs using Unix semantics
- The system relies on custom `spawn`/`spawn_args` syscalls (numbers 1000/1001)
- **This blocks Epic 1 (Process Model)** and the "general purpose OS" goal

**Severity**: P0 CRITICAL

---

## Reproduction Status

**Reproducible**: Yes

**Steps**:
1. Build and run LevitateOS
2. Any program calling `execve()` syscall (number 59 on x86_64, 221 on aarch64)
3. Syscall returns -ENOSYS (function not implemented)

**Expected**: Process image replaced with new program, execution continues at new entry point  
**Actual**: Syscall fails with ENOSYS

---

## Context

### Code Areas

| Location | Description |
|----------|-------------|
| `syscall/src/process/lifecycle.rs:200-217` | The stub `sys_exec()` function |
| `syscall/src/lib.rs:75-77` | Dispatcher maps `Exec` → `sys_exec()` |
| `sched/src/fork.rs` | TEAM_432's fork implementation (reference for address space handling) |
| `mm/src/user/page_table.rs` | `copy_user_address_space()` - similar to what exec needs |
| `process/mod.rs` | `spawn_from_elf()` - existing ELF loading infrastructure |

### Related Work

- **TEAM_432**: Implemented `fork()` with full address space copy
- **spawn_from_elf()**: Already loads ELF binaries and creates tasks
- **spawn_args()**: Already handles argv setup on stack

### What Already Works

1. ELF parsing and loading (`process/mod.rs`)
2. Address space creation (`mm/src/user/page_table.rs`)
3. Argv/envp stack setup (in `spawn_args`)
4. Page table management
5. Task creation and scheduling

---

## Constraints

| Constraint | Description |
|------------|-------------|
| **ABI Compatibility** | Must match Linux execve() semantics exactly |
| **Dual Architecture** | Must work on both x86_64 and aarch64 |
| **No Breaking Changes** | Existing spawn syscalls must continue working |
| **Test Coverage** | Must add regression tests |

---

## Open Questions

1. **FD inheritance**: Linux execve() preserves open file descriptors unless O_CLOEXEC is set. Do we track this flag?
2. **Signal handlers**: Should be reset to default on exec. Is this implemented?
3. **Auxiliary vector**: Already set up by spawn_from_elf, but verify it's correct for exec
4. **Current working directory**: Must be preserved across exec

---

## Scope Definition

### In Scope

- Implement `sys_execve()` with Linux-compatible signature
- Load ELF binary and replace current process image
- Set up argv/envp on new stack
- Handle file descriptor inheritance (O_CLOEXEC)
- Reset signal handlers to default
- Preserve pid, ppid, cwd
- Add regression test

### Out of Scope (for this bugfix)

- Script execution (#! shebang handling)
- Dynamic linking (ld.so)
- setuid/setgid handling (requires Epic 4)

---

## Success Criteria

1. `execve("/bin/echo", ["echo", "hello"], NULL)` executes successfully
2. fork()+exec() pattern works
3. Init can be converted from `spawn()` to `fork()`+`execve()`
4. All existing tests pass
5. New execve test passes
