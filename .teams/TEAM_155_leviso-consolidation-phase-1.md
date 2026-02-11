# TEAM_155: leviso Consolidation — Phase 1 Complete

**Date:** 2026-02-10
**Status:** Phase 1 PARTIALLY COMPLETE (2/4 modules replaced)

## Summary

Started consolidating leviso to use distro-builder like AcornOS/IuppiterOS. Successfully replaced two copy-pasted modules and began migration to thin wrapper architecture.

## Completed

### 1.1 Timer Module ✅
- **What:** Removed 100% identical copy-pasted `leviso/src/timing.rs`
- **How:** Imported `distro_builder::timing::Timer` instead
- **Files:** `/commands/build.rs`, `/component/builder.rs`, `/lib.rs`, `/main.rs`

### 1.2 Cache Module ✅
- **What:** Removed 99% identical copy-pasted `leviso/src/cache.rs`
- **How:** Imported `distro_builder::cache` instead
- **Files:** `/rebuild.rs` (single call site)

### Bonus: Timer Module Refactoring ✅
- **What:** Moved Timer from `distro_builder/src/alpine/timing.rs` to `distro_builder/src/timing.rs` (root level)
- **Why:** Timer is generic (uses only `std::time::Instant`), not Alpine-specific
- **Updated:** leviso, AcornOS, IuppiterOS all use `distro_builder::timing::Timer`
- **Impact:** Cleaner architecture — generic utilities at root, distro-specific in modules

## Testing

- ✅ `cargo check -p leviso -p distro-builder -p acornos -p iuppiteros` — all compile
- ✅ `cargo test -p leviso` — all 55+51+17 tests pass
- ✅ ISO builds in ~1.5 minutes
- ✅ Checkpoint 1 passes (live boot)

## Discoveries

### Executor Modules (1.3-1.4) — API Mismatch
Initially assumed copy-paste, but APIs have diverged:

| Module | leviso API | distro-builder API |
|--------|-----------|-------------------|
| directories | `fn(ctx: &BuildContext, path: &str)` | `fn(staging: &Path, path: &str)` |
| files | `fn(ctx: &BuildContext, path: &str, ...)` | `fn(staging: &Path, path: &str, ...)` |

**Impact:** Can't do simple find-replace. Requires:
1. Update ~30 call sites in leviso
2. Change function signatures
3. Adapt all component code (etc.rs, pam.rs, filesystem.rs, etc.)

**Decision:** Deferred to Phase 2.5 as part of executor refactoring.

### BuildContext — Now a Trait
distro-builder has moved `BuildContext` from concrete struct to trait in distro-contract:

```rust
// leviso (concrete)
pub struct BuildContext {
    pub source: PathBuf,
    pub staging: PathBuf,
    pub base_dir: PathBuf,
    pub output: PathBuf,
}

// distro-builder (trait)
pub trait BuildContext {
    fn base_dir(&self) -> &Path;
    fn staging(&self) -> &Path;
    // ...
}
```

**Impact:** Full adoption requires:
1. Implement trait for leviso
2. Refactor 50+ call sites
3. Update component code

**Decision:** Phase 2.1 (after executor refactor).

## Files Modified

```
leviso/
  src/
    lib.rs              — Removed pub mod timing
    main.rs             — Removed mod timing, mod cache
    commands/build.rs   — Timer import updated
    component/builder.rs — Timer import updated
    rebuild.rs          — cache import updated

distro-builder/
  src/
    lib.rs              — Added pub mod timing
    timing.rs           — NEW (moved from alpine/)

AcornOS/
  src/main.rs           — Timer import updated (6 locations)

IuppiterOS/
  src/main.rs           — Timer import updated (6 locations)

docs/
  leviso-duplication-audit.md — NEW (comprehensive consolidation checklist)

justfile                — NEW (interactive checkpoint commands)
```

## Next Phase (Phase 2)

1. **2.1 BuildContext Unification** — Move from struct to trait
2. **2.2-2.4 Artifact Builders** — Adopt distro-builder EROFS, CPIO, ISO
3. **2.5 Executor Refactor** — Update to &Path-based API
4. **Phase 3 Components** — Migrate to Installable trait + Op enum

## Key Metrics

- **Before:** leviso = ~11,348 LoC, many copy-pastes
- **After Phase 1:** leviso = ~11,300 LoC (removed 2 modules)
- **Target:** leviso = ~2,000-4,000 LoC (thin wrapper)
- **Effort:** ~6 hours CPU time for this phase

## Commits

- `fix(leviso): use distro-builder Timer instead of local copy`
- `fix: use distro-builder cache and timing modules` (with submodule updates)

---

**Status:** Ready to proceed to Phase 2 once executor/BuildContext strategy is approved.
