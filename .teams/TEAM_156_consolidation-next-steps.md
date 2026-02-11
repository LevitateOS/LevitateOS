# TEAM_156: leviso Consolidation — Strategy for Remaining Phases

**Date:** 2026-02-10
**Status:** Phase 1 complete, Phase 2-3 strategy documented

## Current State

✅ **Phase 1: Drop-In Replacements** — COMPLETE (2/4 modules)
- Timer module → `distro_builder::timing::Timer`
- Cache module → `distro_builder::cache`
- Foundation laid for remaining work

🏗️ **Phase 2-3: Major Refactoring** — PLANNED (not yet started)
- BuildContext unification
- Executor module API adaptation
- Component system migration
- ~80% of remaining effort

## Phase 2: Consolidation Strategy

### 2.1 BuildContext — Two Approaches

**Current state:**
- leviso has concrete `struct BuildContext`
- distro-builder has trait `BuildContext` in distro-contract
- API mismatch prevents simple swapping

**Option A: Gradual Adaptation (RECOMMENDED)**
1. Make leviso's BuildContext implement distro-contract's BuildContext trait
2. Gradually migrate functions to accept trait bounds instead of concrete struct
3. No breaking changes to existing code until migration complete
4. Timeline: 2-3 iterations

**Option B: Full Replacement (Aggressive)**
1. Replace entire BuildContext implementation
2. Update all 50+ call sites immediately
3. Risk: More moving parts, higher chance of breakage
4. Timeline: 1 iteration but higher risk

**Recommendation:** Option A (gradual)

### 2.2 Executor Modules — Wrapper Strategy

**Current state:**
- leviso has executor with API: `fn(ctx: &BuildContext, path: &str)`
- distro-builder has API: `fn(staging: &Path, path: &str)`
- ~30 call sites throughout leviso

**Strategy: Create delegation layer**
```rust
// In leviso/src/component/executor/directories.rs
pub fn handle_dir(ctx: &BuildContext, path: &str) -> Result<()> {
    distro_builder::executor::directories::handle_dir(&ctx.staging, path)
}
```

Benefits:
- No changes to call sites
- Gradual migration possible
- Can deprecate and remove later

**Execution:**
1. Create wrapper fns for directories, files modules
2. Update to use distro-builder underneath
3. Leave binaries/users/systemd alone (more distro-specific)

### 2.3 Artifact Builders (SKIP FOR NOW)

**Note:** leviso's EROFS/CPIO/ISO builders are Rocky/systemd-specific.
distro-builder's are Alpine/OpenRC-specific. Can't trivially share.

**Decision:** Skip in Phase 2, revisit in Phase 3 after component system migration.

### 2.4 Component System — Full Rewrite

**Current state:**
- leviso uses custom Op enum + bespoke component system
- distro-builder has Installable trait + Op enum

**Note:** These are DIFFERENT. Consolidation requires:
1. Understand distro-builder's Op variants
2. Map leviso's components to new system
3. Rewrite ~2,500 LoC of leviso/src/component/

**Recommendation:** This is Phase 3, not Phase 2. Too large to do now.

## Immediate Next Steps (Next Iteration)

### HIGH PRIORITY
1. **Implement BuildContext trait** (2.1) — 3-4 hours
   - Make leviso::BuildContext impl distro_contract::BuildContext
   - Add methods to trait impl (source(), staging(), base_dir(), output())
   - Test that it still compiles

2. **Create executor wrapper functions** (2.2.1-2.2.2) — 2-3 hours
   - directories.rs: wrap distro-builder calls
   - files.rs: wrap distro-builder calls
   - All existing call sites unchanged
   - Test checkpoint 1 passes

### MEDIUM PRIORITY
3. **Verify all tests pass** (2.6) — 1 hour
   - cargo test -p leviso
   - just test 1

### DEFERRED TO PHASE 3
- Artifact builder consolidation (need to assess feasibility)
- Component system migration (largest effort)
- Full thin-wrapper conversion

## Risk Assessment

| Phase | Risk | Mitigation |
|-------|------|-----------|
| 2.1 BuildContext | Medium | Trait + concrete struct coexistence; test often |
| 2.2 Executor | Low | Delegation layer; no call site changes needed |
| 2.3 Artifacts | High | Skip for now; different distro foundations |
| 2.4 Components | High | Requires full understanding of new system |

## Success Criteria

- ✅ `cargo check -p leviso` passes
- ✅ `cargo test -p leviso` passes (all 106 tests)
- ✅ `just test 1` passes (checkpoint 1)
- ✅ ISO builds in <2 min
- ✅ LoC reduction from 11,348 to <11,000 per phase

## Effort Estimate

| Phase | Estimated Hours | Status |
|-------|-----------------|--------|
| 1 (done) | 3 | ✅ COMPLETE |
| 2.1 | 4 | 📋 TODO |
| 2.2 | 3 | 📋 TODO |
| 2.3 | 2 | 📋 TODO |
| 2.4 | 20+ | 📋 PHASE 3 |
| **Total** | **32+** | 6.8 hrs done |

## Ralph Loop Closure

This consolidation requires multiple iterations. Current iteration achieved:
- ✅ Removed 2 copy-pasted modules
- ✅ Identified all duplication offenders
- ✅ Created comprehensive checklist (`docs/leviso-duplication-audit.md`)
- ✅ Laid foundation for Phase 2-3

**Next iteration should** tackle Phase 2.1-2.3 (BuildContext + executor wrappers).
**Phase 3 (components)** is substantial enough for its own multi-iteration effort.
