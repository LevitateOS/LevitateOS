# Phase 4 — Cleanup and Verification

**Refactor:** Migrate to linux-raw-sys  
**Team:** TEAM_419  
**Date:** 2026-01-10

---

## Purpose

Final cleanup, verification, and documentation after migration.

---

## Step 1 — Verify Both Architectures Build

```bash
cargo xtask build kernel --arch x86_64
cargo xtask build kernel --arch aarch64
```

**Exit Criteria:** Both build successfully with no errors

---

## Step 2 — Run Full System Test

```bash
./run-term.sh --arch aarch64
```

**Exit Criteria:** System boots, shell works, basic commands run

---

## Step 3 — Remove Dead Code (Rule 6)

Check for any orphaned files or unused imports:
- Deleted `syscall/constants.rs` - DONE
- Deleted fcntl module - DONE
- Any other dead code from refactor

---

## Step 4 — Update Documentation

### Update Team File
```markdown
## Status: Complete
## Files Deleted:
- crates/kernel/src/syscall/constants.rs

## Files Modified:
- All files that imported hardcoded constants

## Dependency Added:
- linux-raw-sys = "0.9"
```

### Update ARCHITECTURE.md (if exists)
Note the use of linux-raw-sys for Linux ABI constants.

---

## Step 5 — Final Verification Checklist

- [ ] x86_64 kernel builds
- [ ] aarch64 kernel builds
- [ ] System boots on QEMU
- [ ] No hardcoded Linux constants remain (grep verify)
- [ ] No shims or compatibility layers exist
- [ ] Team file updated

---

## Grep Verification

```bash
# Should return NO matches in kernel (except comments):
grep -r "pub const CLONE_" crates/kernel/src/
grep -r "pub const RLIMIT_" crates/kernel/src/
grep -r "pub const S_IF" crates/kernel/src/
grep -r "pub const AT_FDCWD" crates/kernel/src/
```

All constants should now come from `linux_raw_sys::*`.

---

## Exit Criteria for Phase 4

- [ ] Both architectures build
- [ ] System boots and runs
- [ ] No hardcoded constants remain
- [ ] Documentation updated
- [ ] Refactor complete
