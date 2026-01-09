# TEAM_366 — Eyra/uutils Investigation Questions

**Created:** 2026-01-10  
**Status:** OPEN — Needs proper investigation

---

## Q1: Why does the binary's own object file define `_start`?

The linker error shows:
```
defined at echo.xxx-cgu.0
>>> .../echo.xxx.rcgu.o:(.text._start+0x0)
```

This is the **binary's own compiled code**, not a dependency.

**Investigation needed:**
- What causes Rust to emit `_start` in some binaries but not others?
- Is it the binary name? The dependency graph? A feature flag?
- Compare `nm` output of working vs blocked binaries

---

## Q2: How should Eyra's origin crate provide `_start`?

Eyra uses the `origin` crate to provide a pure-Rust entry point.

**Investigation needed:**
- Read origin crate source code
- Understand how `-Zbuild-std` interacts with origin
- Check if there's a configuration to prevent duplicate `_start`

---

## Q3: Is this a known Eyra issue?

**Investigation needed:**
- Search Eyra's GitHub issues for "uutils", "duplicate symbol", "_start"
- Check if others have integrated uutils with Eyra successfully
- Consider asking Eyra maintainers directly

---

## Q4: What role does the kernel play?

LevitateOS has its own kernel at `crates/kernel/`.

**Investigation needed:**
- Does the ELF loader have specific expectations about `_start`?
- Are there syscalls that uutils needs that aren't implemented?
- Check `crates/kernel/src/loader/elf.rs`

---

## Q5: Can we use linker scripts or flags?

**Investigation needed:**
- Test `--allow-multiple-definition` linker flag
- Check if linker scripts can resolve symbol conflicts
- Understand implications of using these workarounds

---

## Blocked Until Answered

The uutils integration cannot proceed until these questions are investigated properly.

**Do NOT:**
- Write minimal implementations
- Skip utilities
- Use band-aid workarounds

**DO:**
- Investigate the actual root cause
- Read Eyra source code
- Check upstream issues
- Ask maintainers if needed
