# Phase 5: Polish - Dash Shell Support

## Cleanup Tasks

### Code Quality
- [ ] Run `cargo clippy` on xtask changes
- [ ] Ensure no `unwrap()` in new build code
- [ ] Add doc comments to public functions
- [ ] Remove any debug println! statements

### Error Messages
- [ ] Verify all error paths have actionable messages
- [ ] Test error case: musl-sysroot missing
- [ ] Test error case: clang not installed
- [ ] Test error case: dash clone fails

## Documentation Updates

### CLAUDE.md Updates
Add to Build Commands section:
```markdown
# Build musl C library sysroot
cargo xtask build musl-sysroot

# Build dash shell
cargo xtask build dash
```

Add to Architecture section:
```markdown
### Shell Tiers
| Tier | Shell | Complexity | Use Case |
|------|-------|------------|----------|
| T1   | dash  | Low        | Basic shell tests |
| T2   | brush | High       | Full bash compat  |
```

### README.md
No changes needed (build instructions in CLAUDE.md)

### docs/testing/behavior-inventory.md
Add new behavior IDs:
- `[DASH1]` Dash shell boots
- `[DASH2]` Dash executes commands
- `[DASH3]` Dash handles pipes
- `[DASH4]` Dash exits cleanly

## Handoff Notes

### For Future Teams

**What's Implemented:**
- musl sysroot build for C programs
- dash shell build via musl cross-compilation
- wait3/wait4 kernel syscalls
- Optional inclusion in initramfs

**What's NOT Implemented:**
- Dynamic linking (still static only)
- Other C programs (only dash)
- Job control (wait3 returns zero rusage)

**Known Limitations:**
1. Dash uses internal glob/fnmatch (not libc versions)
2. Resource usage tracking incomplete (rusage always zero)
3. aarch64 may need additional testing

**Debugging Tips:**
- If dash crashes, check syscall trace first
- If dash hangs, check terminal/signal handling
- If pipes fail, verify pipe/dup2 syscalls

### Dependencies for This Feature

**Host Dependencies:**
- clang (C compiler)
- musl-dev or musl-tools (C library)
- autoconf, automake (dash build system)

**Runtime Dependencies:**
- Kernel with wait3/wait4 support
- Working fork/exec/wait
- Working pipe/dup2

### Future Improvements

1. **Add more C apps**: sash (Stand-Alone Shell) for even simpler testing
2. **Resource tracking**: Implement proper rusage in wait3/wait4
3. **Dynamic linking**: Add ld.so for dynamically linked C programs
4. **libedit**: Add line editing to dash for better interactive use

## Team File Location

`.teams/TEAM_444_feature_dash_shell_support.md`

## Plan File Location

`docs/planning/dash-shell-support/`
- phase-1.md (Discovery)
- phase-2.md (Design)
- phase-3.md (Implementation)
- phase-4.md (Integration)
- phase-5.md (This file)
