# TEAM_363 — Review & Implement Eyra Migration Plan

**Created:** 2026-01-09  
**Plan:** `docs/planning/eyra-migration/`  
**Status:** 🟠 In Progress (Phase 2 Complete)

## Review Scope

Reviewing the Eyra migration plan created by TEAM_361/362 for:
- Questions and answers alignment
- Scope and complexity
- Architecture alignment
- Global rules compliance
- Verification of claims

## Phase 1: Questions and Answers Audit

### Questions Files Found

| File | Status |
|------|--------|
| `TEAM_349_eyra_integration.md` | ✅ All 7 questions answered |
| `TEAM_359_eyra_syscalls_questions.md` | ⚠️ Q1 (ppoll blocking) awaits confirmation |

### Q&A Plan Alignment

| Question | Answer | Reflected in Plan? |
|----------|--------|--------------------|
| Q1: getrandom | Hardware + PRNG fallback | ✅ Not plan-relevant (kernel work) |
| Q2: clone3 | Defer (clone only) | ✅ Correct |
| Q3: /proc/self/exe | Return error | ✅ Correct |
| Q4: arch_prctl | Implement | ✅ Already done (TEAM_360) |
| Q5: Signal queues | Immediate delivery | ✅ Correct |
| Q6: fcntl | F_GETFD/SETFD/GETFL/SETFL | ✅ Correct |
| Q7: MAP_FIXED | Implement | ✅ Correct |

### Open Questions

**TEAM_359 Q1 (ppoll blocking)** — Still awaiting user confirmation:
- Plan proceeds assuming non-blocking is OK
- Risk: May need revisiting if std I/O requires blocking

### Phase 1 Verdict: ✅ PASS (minor)
- All major questions answered and reflected
- One open question does not block initial phases

---

## Phase 2: Scope and Complexity Check

### Structure Analysis

| Item | Count | Assessment |
|------|-------|------------|
| Phases | 5 | ✅ Appropriate for scope |
| Apps to migrate | 12 | ✅ Reasonable |
| Time estimate | 14 hours | ✅ Realistic |

### Overengineering Signals

| Signal | Found? | Notes |
|--------|--------|-------|
| Too many phases | ❌ No | 5 phases appropriate |
| Unnecessary abstractions | ❌ No | Direct migration, no new layers |
| Premature optimization | ❌ No | Size concerns acknowledged but not blocking |
| Speculative features | ⚠️ Minor | Shell "tab completion" mentioned as optional |
| Excessive UoW splitting | ❌ No | Each app is one UoW |

### Oversimplification Signals

| Signal | Found? | Notes |
|--------|--------|-------|
| Missing phases | ❌ No | Has cleanup and hardening phases |
| Vague UoWs | ❌ No | Each step is concrete |
| Ignored edge cases | ⚠️ Minor | See below |
| No regression protection | ❌ No | Golden tests mentioned |
| Handwavy handoff | ❌ No | Phase 5 has clear handoff |

### Edge Cases Not Addressed

1. **Test binaries migration**: levbox has 9 test binaries (clone_test, mmap_test, etc.) — not mentioned in migration plan
2. **repro_crash, systest crates**: Listed in workspace but not in migration plan

### Phase 2 Verdict: ⚠️ PASS with concerns
- Missing: Test binaries migration strategy
- Missing: Decision on repro_crash/systest crates

---

## Phase 3: Architecture Alignment

### Critical Discrepancy: Directory Structure

Plan proposes:
```
crates/userspace/apps/
├── init/
├── shell/
├── cat/
...
```

Existing eyra-hello is at:
```
userspace/eyra-hello/  (NOT in crates/userspace/)
```

**Issue:** Two different locations for Eyra apps. Need consistency.

### Cargo.toml Pattern Discrepancy

Plan specifies:
```toml
[dependencies.std]
package = "eyra"
version = "0.22"
```

Existing eyra-hello uses:
```toml
[dependencies]
eyra = { version = "0.22", features = ["experimental-relocate"] }

# NO [dependencies.std] block
```

**Issue:** Plan's Cargo.toml pattern differs from working example.

### Build Configuration Discrepancy

Plan specifies:
```toml
opt-level = "z"
```

Existing eyra-hello uses:
```toml
opt-level = "s"
strip = true
```

**Issue:** Minor, but should be consistent.

### Current Broken State

levbox Cargo.toml still references:
```toml
ulib = { path = "../ulib", features = ["entry"] }
```

But `crates/userspace/ulib/` directory **does not exist** (correctly deleted).

**Issue:** Workspace is currently broken — levbox won't build.

### Phase 3 Verdict: ⚠️ NEEDS CORRECTIONS

1. **Decide app location:** Either `crates/userspace/apps/` OR `userspace/`
2. **Update Cargo.toml template** to match working eyra-hello
3. **Acknowledge broken state** in Phase 1 (already noted but could be clearer)

---

## Phase 4: Global Rules Compliance

| Rule | Status | Notes |
|------|--------|-------|
| Rule 0 (Quality > Speed) | ✅ Pass | Clean migration, no hacks |
| Rule 1 (SSOT) | ✅ Pass | Plan in docs/planning/ |
| Rule 2 (Team Registration) | ✅ Pass | TEAM_361, TEAM_362 files exist |
| Rule 3 (Before Starting Work) | ✅ Pass | Prereqs listed |
| Rule 4 (Regression Protection) | ✅ Pass | Golden tests mentioned |
| Rule 5 (Breaking Changes) | ✅ Pass | Clean cutover, no adapters |
| Rule 6 (No Dead Code) | ✅ Pass | Phase 4 is cleanup |
| Rule 7 (Modular Refactoring) | ⚠️ Check | New structure needs review |
| Rule 8 (Ask Questions Early) | ✅ Pass | Questions files exist |
| Rule 9 (Maximize Context) | ✅ Pass | Work batched by app |
| Rule 10 (Before Finishing) | ✅ Pass | Phase 5 has handoff |
| Rule 11 (TODO Tracking) | ⚠️ Unclear | No TODOs documented yet |

### Phase 4 Verdict: ✅ PASS

---

## Phase 5: Verification and References

### Claims Verified

| Claim | Verified? | Method |
|-------|-----------|--------|
| ulib already deleted | ✅ Yes | `find_by_name` returned 0 results |
| eyra-hello exists as template | ✅ Yes | File exists at `userspace/eyra-hello/` |
| Eyra 0.22 supports experimental-relocate | ✅ Yes | Seen in working Cargo.toml |
| Syscalls implemented (TEAM_360) | ✅ Assumed | Referenced in phase-1.md |
| levbox has 10 utilities | ✅ Yes | Confirmed in levbox/Cargo.toml |

### Claims Needing Correction

| Claim | Issue |
|-------|-------|
| "levbox → ulib" still works | ❌ No — ulib deleted, levbox broken |
| Cargo.toml pattern | ❌ Plan pattern differs from eyra-hello |
| Apps location | ❌ Plan says `crates/userspace/apps/`, existing is `userspace/` |

### Phase 5 Verdict: ⚠️ NEEDS CORRECTIONS

---

## Phase 6: Summary of Required Corrections

### Critical (blocks work)

1. **Fix Cargo.toml template** — Update plan to match working eyra-hello pattern:
   - Remove `[dependencies.std]` block
   - Add `extern crate eyra;` requirement note
   - Add `strip = true` for smaller binaries

2. **Decide app location** — Choose ONE of:
   - A) `crates/userspace/apps/` (plan's proposal)
   - B) `userspace/` (where eyra-hello currently lives)
   
   **Recommendation:** Option B — less restructuring, eyra-hello already there

### Important (improves quality)

3. **Add test binaries decision** — What happens to:
   - `suite_test_core`, `clone_test`, `mmap_test`, `pipe_test`, etc.
   - `repro_crash`, `systest` crates
   
   **Recommendation:** Add to Phase 4 cleanup — either migrate or remove

4. **Confirm ppoll question** — Q1 from TEAM_359 still open

### Minor (nice to have)

5. **Clarify toolchain sharing** — Each app needing its own `rust-toolchain.toml` is redundant; consider shared location

6. **Add `extern crate eyra;` note** — Required for -Zbuild-std compatibility (see eyra-hello)

## Changes Applied

### plan.md
- Fixed Cargo.toml template to match working eyra-hello pattern
- Added `extern crate eyra;` requirement note
- Added `strip = true` and `[unstable]` section

### phase-2.md
- Changed directory structure from `crates/userspace/apps/` to `userspace/`
- Updated all path references
- Fixed Cargo.toml template

### phase-4.md
- Added test binaries decision table (9 binaries)
- Added `repro_crash/` and `systest/` to deletion list

## Remaining Open Items

1. **User confirmation needed:** TEAM_359 Q1 (ppoll blocking) still awaiting answer
2. **Optional:** Consider shared rust-toolchain.toml in `userspace/` root

## Progress Log

### 2026-01-09
- Created team file
- Completed 6-phase review
- Applied corrections to plan.md, phase-2.md, phase-4.md
- Review complete
- User approved: ppoll non-blocking (Q1), shared toolchain
- Created shared `userspace/rust-toolchain.toml`
- **Fixed kernel bug:** Stack alignment for static-pie (Eyra) binaries with >1 arg
  - File: `crates/kernel/src/memory/user.rs:323-344`
  - Root cause: 16-byte alignment not guaranteed for x86-64 ABI
- **Migrated cat utility to Eyra/std** (326KB)
  - Location: `crates/userspace/eyra/cat/`
  - Tested: `cat /hello.txt` works
- Phase 2 complete, ready for Phase 3
- **Migrated pwd** - working
- **Migrated mkdir** - working (read-only initramfs limitation)
- Phase 3 in progress, utilities using std idioms
- **ls blocked:** requires getdents64 syscall (217) - not implemented in kernel
- Migrated utilities: cat, pwd, mkdir (3/10 core utilities)
- **Refactored directory structure:** Moved all Eyra apps from `userspace/` to `crates/userspace/eyra/`
- Updated xtask paths and plan documents
