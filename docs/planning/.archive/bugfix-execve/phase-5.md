# Phase 5: Cleanup, Regression Protection, and Handoff

**Bug**: `execve()` syscall is a stub returning ENOSYS  
**Team**: TEAM_436  
**Status**: Pending (after Phase 4)

---

## Cleanup Tasks

### Task 1: Remove Stub Warning

Remove the warning log from the old stub code:
```rust
// Remove this line
log::warn!("[SYSCALL] exec is currently a stub");
```

### Task 2: Update Code Comments

Update comments in arch files that reference "temporary, until clone/execve work":
- `arch/aarch64/src/lib.rs:122-126`
- Any other references

### Task 3: Consider Deprecating Custom Spawn Syscalls

After execve works, the custom `spawn` (1000) and `spawn_args` (1001) syscalls become redundant for Unix programs. Options:
- Keep for backwards compatibility
- Mark as deprecated
- Eventually remove (breaking change)

**Decision**: Keep for now, document that fork()+execve() is preferred.

---

## Regression Protection

### New Tests to Maintain

| Test | Purpose |
|------|---------|
| `test_execve_basic` | Basic execve works |
| `test_execve_argv` | Arguments passed correctly |
| `test_execve_cloexec` | O_CLOEXEC fds closed |
| `test_fork_exec` | Full fork()+exec() pattern |

### Golden File Updates

If behavior tests output changes (e.g., init now uses fork+exec), update golden files:
```bash
cargo xtask test behavior --update
```

### CI Integration

Ensure all new tests are included in:
- `cargo xtask test` workflow
- Any CI pipelines

---

## Documentation Updates

### Task 1: Update CLAUDE.md

Add execve to the list of implemented syscalls.

### Task 2: Update Master Plan

Mark Epic 1 Phase 2 (execve) as complete in:
- `docs/planning/MASTER_PLAN_GENERAL_PURPOSE_OS.md`

### Task 3: Update Team File

Complete the TEAM_436 team file with:
- Implementation summary
- Key decisions made
- Any gotchas discovered
- Handoff notes

---

## Handoff Checklist

Before marking this bugfix complete:

- [ ] `sys_execve()` implemented and working
- [ ] Both x86_64 and aarch64 build successfully
- [ ] All existing tests pass
- [ ] New execve tests pass
- [ ] O_CLOEXEC handling works
- [ ] Signal handlers reset on exec
- [ ] No memory leaks (old address space freed)
- [ ] Code reviewed for safety
- [ ] Documentation updated
- [ ] Team file completed

---

## Future Work (Out of Scope)

Items identified during this bugfix that should be addressed later:

1. **Shebang (#!) execution**: Scripts starting with `#!/bin/sh` should invoke the interpreter
2. **Dynamic linking**: Load and execute dynamically linked ELFs via ld.so
3. **setuid/setgid on exec**: Honor setuid bit when execve'ing (requires Epic 4)
4. **AT_EXECFN in auxv**: Provide executed filename in auxiliary vector

These should be filed as separate issues or added to the roadmap.

---

## Success Verification

Final verification steps:

1. **Manual test**:
   ```
   # In LevitateOS shell
   echo "testing execve"  # This should work via fork+exec internally
   ```

2. **Automated test**:
   ```bash
   cargo xtask test behavior
   cargo xtask test unit
   ```

3. **Build verification**:
   ```bash
   cargo xtask build all  # Both architectures
   ```

When all checks pass, the bugfix is complete.
