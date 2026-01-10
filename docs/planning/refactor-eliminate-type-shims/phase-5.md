# Phase 5 — Hardening

**Refactor:** Eliminate Type Shims - Make linux-raw-sys the Canonical Source
**Team:** TEAM_420
**Date:** 2026-01-10

---

## Final Verification

### Build Verification
```bash
cargo xtask build kernel --arch x86_64
cargo xtask build kernel --arch aarch64
```

### Runtime Verification (if available)
```bash
cargo xtask run --term
# Test: spawn process, check errno on failure, epoll operations
```

### Static Analysis
```bash
# No shims
grep -c "pub const.*= .*as i" crates/kernel/src/syscall/mod.rs  # Should be 0
grep -c "pub const.*= .*as u" crates/kernel/src/syscall/process/mod.rs  # Should be 0

# Direct imports only
grep -l "use linux_raw_sys::" crates/kernel/src/syscall/*.rs  # Should list all syscall files
```

---

## Documentation Updates

### Update CLAUDE.md
Add to "Key Architectural Patterns" section:

```markdown
### linux-raw-sys Usage

LevitateOS uses `linux-raw-sys` as the canonical source for all Linux ABI constants.

**IMPORTANT: NO SHIMS**
- Import constants directly: `use linux_raw_sys::errno::ENOENT`
- Never create wrapper modules that re-export with type casts
- If types don't match, change YOUR code to match linux-raw-sys

**Errno handling:**
```rust
// Correct: explicit negation at use site
return -(linux_raw_sys::errno::ENOENT as i64);

// WRONG: shim module
pub mod errno { pub const ENOENT: i64 = -2; }
```
```

### Update error.rs header
```rust
//! TEAM_202: VFS Error Types
//! TEAM_420: Uses linux_raw_sys::errno directly, no shims.
```

---

## Handoff Notes

### For Future Developers

1. **Adding new errno usage**: Import directly from `linux_raw_sys::errno`
   ```rust
   use linux_raw_sys::errno::ENEWCODE;
   return -(ENEWCODE as i64);
   ```

2. **Adding new constants**: Import from `linux_raw_sys::general`
   ```rust
   use linux_raw_sys::general::NEW_FLAG;
   ```

3. **Type mismatches**: If linux-raw-sys has a different type than expected:
   - **DO**: Change your function signature to match
   - **DON'T**: Create a shim constant with `as` cast

4. **Grep for violations**:
   ```bash
   grep -rn "pub const.*linux_raw_sys.*as" crates/kernel/src/
   # Should return 0 results
   ```

---

## Lessons Learned

1. **Shims are debt**: They seem helpful but add indirection and hide the real types
2. **The library is canonical**: When integrating an external source of truth, adapt to IT
3. **Let the compiler help**: Breaking changes reveal all callsites; fix them don't hide them
4. **Explicit > Implicit**: `-(ENOENT as i64)` is better than `errno::ENOENT` shim magic

---

## Exit Criteria for Phase 5

- [ ] Both architectures build successfully
- [ ] No shim patterns in grep output
- [ ] CLAUDE.md updated with linux-raw-sys guidelines
- [ ] Future developer notes documented
- [ ] Team file marked complete

---

## Final Checklist

- [ ] All phases complete
- [ ] Builds pass (x86_64, aarch64)
- [ ] Zero shims remain
- [ ] Documentation updated
- [ ] `.teams/TEAM_420_refactor_eliminate_type_shims.md` marked COMPLETE
