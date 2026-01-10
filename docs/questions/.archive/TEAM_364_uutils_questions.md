# TEAM_364 — uutils-coreutils Questions

## Q1: Binary Size Tradeoff

uutils utilities are feature-complete GNU replacements. This means:
- More code = larger binaries
- Our hand-written cat: ~326KB
- uu_cat with full features: likely 500KB-1MB

**Question:** Is increased binary size acceptable for full GNU compatibility?

**Options:**
- A) Yes, GNU compatibility is worth the size ✅ **SELECTED**
- B) No, prefer minimal hand-written utilities
- C) Hybrid: use uutils for complex utilities (ls, cp, find), hand-write simple ones (cat, pwd)

**Answer:** A — Full GNU compatibility is the priority.

---

## Q2: Multicall vs Individual Binaries

**Options:**
- A) **Multicall (BusyBox-style):** Single `coreutils` binary + symlinks
  - Pro: Smaller total size, simpler deployment
  - Con: All-or-nothing, harder to debug
  
- B) **Individual binaries:** Separate `cat`, `ls`, `cp` binaries ✅ **SELECTED**
  - Pro: Flexible, can mix with hand-written utilities
  - Con: Larger total size, more files

**Answer:** B — Individual binaries.

---

## Q3: Which Utilities to Include?

uutils has 100+ utilities. Which subset for LevitateOS?

**Minimum viable:**
- cat, cp, mv, rm, mkdir, rmdir, ls, ln, touch, pwd, echo, env, true, false

**Extended (shell-useful):**
- head, tail, wc, sort, uniq, grep (separate crate), find, xargs

**Full coreutils:**
- Everything

**Question:** Which tier to target initially?

**Answer:** Minimum viable — cat, cp, mv, rm, mkdir, rmdir, ls, ln, touch, pwd, echo, env, true, false

---

## Q4: Missing Syscall Strategy

uutils will likely need syscalls we haven't implemented yet.

**Options:**
- A) Implement all required syscalls first, then integrate uutils
- B) Integrate incrementally — stub missing syscalls, implement as needed
- C) Fork uutils and patch to work with current syscall support

**Recommendation:** Option B — lets us make progress while discovering requirements.

**Answer:** B (by implication from Q5-A)

---

## Q5: Priority vs Current Migration

TEAM_363 is mid-migration of hand-written utilities to Eyra.

**Options:**
- A) Stop current migration, pivot to uutils immediately
- B) Complete current migration, then evaluate uutils for v2
- C) Continue migration for simple utilities, use uutils for complex ones (ls, cp, find)

**Recommendation:** Option C — ls is already blocked, uutils could unblock it.

**Answer:** A — Stop current migration, pivot to uutils immediately. Full commitment.
