# TEAM_448: Investigate BusyBox Integration

**Date:** 2026-01-12  
**Status:** Complete  
**Task:** Investigate replacing uutils-coreutils with BusyBox

---

## Summary

Investigated how to integrate BusyBox to replace uutils-coreutils. Created comprehensive investigation document at `docs/planning/busybox-integration/INVESTIGATION.md`.

## Key Findings

### Why BusyBox > uutils-coreutils

| Factor | uutils-coreutils | BusyBox |
|--------|------------------|---------|
| Binary size | ~2MB (8 utils) | ~1MB (300+ utils) |
| Shell included | ❌ No | ✅ Yes (ash) |
| Build complexity | Rust toolchain | C + musl-gcc (already have) |
| Feature coverage | 8 utilities | 300+ utilities |

### Build Approach

BusyBox can be built statically with musl using existing toolchain:

```bash
# Already have musl-gcc for dash
git clone https://git.busybox.net/busybox toolchain/busybox
cd toolchain/busybox
make defconfig
# Disable musl-incompatible features
make CC=musl-gcc LDFLAGS="-static" -j$(nproc)
# Result: ~1MB static binary
```

### Syscall Compatibility

BusyBox uses standard POSIX syscalls. All required syscalls are already implemented in LevitateOS:
- fork/execve/waitpid ✅
- pipe/dup2 ✅
- open/read/write/stat ✅
- termios/ioctl ✅
- signals ✅

## Deliverables

- `docs/planning/busybox-integration/INVESTIGATION.md` - Full investigation document with:
  - Current state analysis
  - BusyBox build requirements
  - Implementation plan
  - Code examples for xtask integration
  - Risk assessment

## Recommendation

**Switch to BusyBox.** Benefits:
1. Single binary replaces both coreutils AND dash
2. 300+ utilities vs current 8
3. Smaller total size
4. Simpler build (reuse existing musl-gcc)
5. Battle-tested (Alpine Linux, embedded systems)

## Next Steps (for future team)

1. Create `xtask/src/build/busybox.rs`
2. Add `cargo xtask build busybox` command
3. Test on LevitateOS
4. Update initramfs creation
5. Remove uutils-coreutils and dash

---

## Handoff Checklist

- [x] Investigation complete
- [x] Documentation created
- [x] Team file updated
- [ ] Implementation (future team)
