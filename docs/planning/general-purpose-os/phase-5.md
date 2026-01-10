# Phase 5: Polish - General Purpose OS

**TEAM_400**: General Purpose Unix-Compatible OS
**Created**: 2026-01-10
**Status**: Ready for Implementation

---

## Cleanup Tasks

### Code Cleanup

| Task | Location | Priority |
|------|----------|----------|
| Remove dead code from old spawn syscall | `syscall/process.rs` | Medium |
| Consolidate path resolution helpers | `syscall/fs/` | Low |
| Unify error handling patterns | `syscall/` | Low |
| Add missing SAFETY comments | All new unsafe blocks | High |

### Technical Debt

| Item | Description | Action | Decision Reference |
|------|-------------|--------|-------------------|
| Eager copy fork | Not memory efficient | Document as future CoW optimization | Q1 |
| chmod/chown no-ops | Don't modify permissions (by design) | Document single-user limitation | Q6 |
| pselect6 missing | Not implemented | Add when program needs it | Q8 (deferred) |

### Deprecated Code Removal

```rust
// Consider removing or deprecating:

// Old spawn syscalls (if execve replaces them)
pub fn sys_spawn(...) -> i64;       // May keep for compatibility
pub fn sys_spawn_args(...) -> i64;  // May keep for compatibility

// If keeping, mark deprecated:
#[deprecated(since = "0.x.0", note = "Use fork+execve instead")]
pub fn sys_spawn(...) -> i64 { ... }
```

---

## Documentation Updates

### Files to Update

| File | Updates Needed |
|------|----------------|
| `README.md` | Add "General Purpose" capability |
| `docs/ROADMAP.md` | Mark milestone complete |
| `docs/ARCHITECTURE.md` | Document fork/exec flow |
| `CLAUDE.md` | Update syscall status table |
| `docs/specs/LINUX_ABI_GUIDE.md` | Add fork/execve gotchas |

### New Documentation

#### User Guide: Running Static Binaries

```markdown
# Running Static Linux Binaries on LevitateOS

## Quick Start

1. Compile your program as static on any Linux system:
   ```bash
   gcc -static -o myprogram myprogram.c
   # OR with musl:
   musl-gcc -static -o myprogram myprogram.c
   ```

2. Add to LevitateOS initramfs:
   ```bash
   cp myprogram initramfs/bin/
   cargo xtask build initramfs
   ```

3. Run:
   ```bash
   cargo xtask run
   # In LevitateOS shell:
   /bin/myprogram
   ```

## Supported Features

- fork/exec process creation
- File I/O (open, read, write, close)
- Memory management (mmap, munmap, brk)
- Signals (basic handling)
- I/O multiplexing (poll, select, epoll)

## Limitations

- Single-user (always runs as root)
- No networking (yet)
- No dynamic linking (yet)
- Limited /proc filesystem
- chmod/chown are no-ops (permissions not enforced)
- Fork uses eager copy (memory inefficient for large processes)
```

#### Developer Guide: Adding Syscalls

```markdown
# Adding New Syscalls to LevitateOS

## Checklist

1. [ ] Verify syscall number in `los_abi`
2. [ ] Implement handler in appropriate `syscall/` module
3. [ ] Add to dispatcher in `syscall/mod.rs`
4. [ ] Write unit tests
5. [ ] Add behavior test if user-visible
6. [ ] Update CLAUDE.md syscall table
7. [ ] Document in LINUX_ABI_GUIDE.md if gotchas exist
```

### CLAUDE.md Syscall Table Update

```markdown
## Syscall Status

| Category | Syscall | Status | Notes |
|----------|---------|--------|-------|
| **Process** | fork | ✅ Done | Eager copy (Q1) |
| | vfork | ✅ Done | Same as fork |
| | execve | ✅ Done | VFS path resolution (Q2), resets signals except SIG_IGN (Q7) |
| | clone (threads) | ✅ Done | TEAM_230 |
| | wait4/waitpid | ✅ Done | TEAM_188 |
| **Filesystem** | chdir/fchdir | ✅ Done | TEAM_400 |
| | chmod/chown | ✅ Done | No-op (Q6: single-user) |
| **I/O** | poll | ✅ Done | TEAM_400 (Q4) |
| | select | ✅ Done | TEAM_400 |
| | pselect6 | ⏳ Deferred | Q8: YAGNI |
| | epoll_* | ✅ Done | TEAM_394 |
| **System** | uname | ✅ Done | TEAM_400 |
| | umask | ✅ Done | TEAM_400 |
| | getppid | ✅ Done | Returns 1 for orphans (Q10) |
```

---

## Handoff Notes

### What Works

After this feature is complete:

1. **Static binary execution**: Any statically-linked Linux binary can run
2. **Fork/exec**: Standard Unix process creation pattern works
3. **Shell scripts**: Basic shell with fork/exec can run pipelines
4. **Build systems**: Make and similar tools can fork compiler processes

### What Doesn't Work Yet

1. **Dynamic linking**: No libc.so, no ld-linux.so
2. **Networking**: No socket syscalls
3. **Advanced process features**:
   - No process groups (partial)
   - No sessions
   - No controlling terminal (partial)
4. **Full /proc**: Only minimal entries

### Known Limitations

| Limitation | Impact | Future Work | Decision |
|------------|--------|-------------|----------|
| Eager copy fork | Memory inefficient for large processes | Implement CoW | Q1 - start simple |
| No vfork optimization | Slower fork+exec | Implement true vfork | vfork = fork for now |
| chmod/chown no-ops | Permissions not stored | Store in VFS if needed | Q6 - single-user OK |
| pselect6 missing | Some programs may need it | Add when needed | Q8 - deferred |
| Single-user | All processes run as root | Multi-user support | Design constraint |
| No swap | Limited by physical RAM | Add swap | Design constraint |

### Recommended Next Steps

1. **Milestone 2 Prep**: Investigate libc.so feasibility
   - Track c-gull cdylib progress
   - Evaluate musl-libc port

2. **Process Improvements**:
   - Implement CoW for fork
   - Add process group support
   - Implement setsid/setpgid

3. **Filesystem Improvements**:
   - Implement /proc filesystem
   - Add /dev/pts for PTY support

---

## Success Criteria Verification

### Milestone 1: Static Binary Compatibility

| Criterion | Test | Status |
|-----------|------|--------|
| gcc -static hello.c runs | Integration test | PENDING |
| musl-gcc -static hello.c runs | Integration test | PENDING |
| Rust +crt-static binary runs | Integration test | PENDING |
| No Eyra dependency required | Verified by test | PENDING |
| No source modification required | Verified by test | PENDING |

### Sign-off Checklist

- [ ] All acceptance tests pass
- [ ] Documentation updated
- [ ] No regressions
- [ ] Performance acceptable
- [ ] Code reviewed
- [ ] TEAM file complete

---

## Team File Reference

See: `.teams/TEAM_400_feature_general_purpose_os.md`

---

## Future Milestones Reference

### Milestone 2: Dynamic Binary Compatibility

Prerequisites from this milestone:
- Working fork/exec
- Stable syscall ABI
- VFS path resolution

New requirements:
- libc.so.6 (c-gull or musl)
- ld-linux.so.2 (dynamic linker)
- /lib directory in filesystem
- Symbol versioning

### Milestone 3: Development Environment

Prerequisites:
- Milestone 2 complete
- Working /proc filesystem

New requirements:
- GCC or Clang port
- binutils port
- make port
- /usr/include headers

---

## References

- Phase 1-4: `docs/planning/general-purpose-os/phase-*.md`
- Original vision: `docs/planning/general-purpose-os/FEATURE_PLAN.md`
- Investigation: `.teams/TEAM_399_investigate_cgull_cdylib_feasibility.md`
- Review: `.teams/TEAM_398_review_general_purpose_os_plan.md`
