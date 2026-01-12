# TEAM_444: Dash Shell Open Questions

These questions need user input before implementation proceeds.

## Q1: Cross-compiler choice

**Context**: We need to cross-compile dash (C code) against musl for LevitateOS.

**Options**:
1. **musl-gcc wrapper** - Requires gcc + musl-dev on host
2. **x86_64-linux-musl-gcc** - Standalone musl cross-compiler toolchain
3. **clang with musl sysroot** - LLVM-based, more portable

**Recommendation**: Option 3 (clang) - already installed for kernel builds, more portable across Linux distros.

**Question**: Is clang acceptable, or do you prefer gcc for C userspace programs?

---

## Q2: Shell tier naming

**Context**: We now have multiple shells of different complexity.

**Options**:
1. **T0/T1/T2** - Numeric tiers (T0=none, T1=dash, T2=brush)
2. **simple/full** - Descriptive names (simple=dash, full=brush)
3. **dash/bash** - Just use shell names directly

**Recommendation**: Option 1 - allows future expansion (T0.5 for sash, etc.)

**Question**: How do you want to refer to shell complexity levels?

---

## Q3: aarch64 priority

**Context**: Dash needs to work on both x86_64 and aarch64. musl supports both.

**Options**:
1. **x86_64 first** - Get it working on x86_64, then port to aarch64
2. **Both simultaneously** - Implement for both from the start

**Recommendation**: Option 1 - faster iteration, fewer variables

**Question**: Should we focus on x86_64 first or implement both architectures simultaneously?

---

## Q4: wait3 resource tracking

**Context**: Dash calls wait3() to get resource usage (rusage) for child processes.

**Options**:
1. **Zero rusage** - Return zeros (simpler, most programs don't care)
2. **Track usage** - Actually track CPU time, memory (complex, not needed yet)

**Recommendation**: Option 1 - implement properly later when needed

**Question**: Is returning zero rusage acceptable for now?

---

## Status

- [ ] Q1 answered
- [ ] Q2 answered
- [ ] Q3 answered
- [ ] Q4 answered

Once questions are answered, implementation can proceed.
