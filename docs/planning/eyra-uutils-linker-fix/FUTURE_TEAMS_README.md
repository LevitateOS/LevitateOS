# CRITICAL: Eyra/uutils Linker Conflict — Future Teams Read This

**TEAM_366** | 2026-01-10

---

## ⚠️ DO NOT TAKE SHORTCUTS

Previous attempts tried:
- ❌ `default-features = false` — ICU still pulled in
- ❌ Renaming binaries — Symbol conflict persists
- ❌ Minimal implementations — Defeats purpose of using uutils

**These are band-aids, not fixes.**

---

## The Real Problem

When building some uutils crates with Eyra, the linker reports:

```
duplicate symbol: _start
duplicate symbol: __dso_handle
```

### What We Know

1. **Working utilities:** cat, pwd, mkdir, ls
2. **Blocked utilities:** echo, env, true, false, rm, cp, mv, ln, touch, rmdir

3. **The `_start` symbol comes from the binary's own object file**, not a dependency:
   ```
   defined at echo.xxx-cgu.0
   >>> .../echo.xxx.rcgu.o:(.text._start+0x0)
   ```

4. **Eyra provides `_start` via the `origin` crate** for its pure-Rust libc

5. **The conflict is NOT just about ICU dependencies** — disabling them doesn't fix it

---

## What Needs Investigation

### 1. Why do some binaries define `_start` and others don't?

The binary name itself (`true`, `false`, `echo`) is causing the compiler to generate `_start` in the binary's object file. Working utilities (cat, pwd, mkdir, ls) don't have this.

**Hypothesis:** There's something special about these binary names in Rust/LLVM codegen.

### 2. How does Eyra's entry point work?

Eyra uses the `origin` crate to provide `_start`. When `-Zbuild-std` rebuilds std, it should use Eyra's entry point. But something is causing BOTH Eyra's `_start` AND another `_start` to be generated.

**Files to investigate:**
- Eyra's origin crate source
- How `-Zbuild-std` handles entry points
- LLVM codegen for binary names

### 3. What's different about the dependency graph?

```bash
# Compare working vs blocked
cd crates/userspace/eyra/cat && cargo tree > /tmp/cat.tree
cd ../echo && cargo tree > /tmp/echo.tree
diff /tmp/cat.tree /tmp/echo.tree
```

### 4. Can linker flags help?

Try:
```toml
[build]
rustflags = ["-C", "link-args=-Wl,--allow-multiple-definition"]
```

But understand WHY before using this — it may cause runtime issues.

---

## The Kernel Angle

LevitateOS has a full kernel. The syscall layer is at:
- `crates/kernel/src/syscall/`

Before giving up on uutils, verify:
1. Are all required syscalls implemented?
2. Is the issue actually in how the kernel loads/runs these binaries?
3. Is there a configuration in Eyra that needs kernel support?

---

## What NOT To Do

1. **Don't write minimal implementations** — We want GNU-compatible uutils, not toys
2. **Don't skip utilities** — If 10 of 14 don't work, the approach is broken
3. **Don't blame uutils** — cat/pwd/mkdir/ls work fine
4. **Don't loop on the same fix** — If it didn't work, investigate deeper

---

## Recommended Next Steps

1. **Read Eyra's documentation** on how it handles binary entry points
2. **Check if there's an Eyra issue** about uutils compatibility
3. **Compare object files** between working and blocked utilities:
   ```bash
   nm working_binary.o | grep _start
   nm blocked_binary.o | grep _start
   ```
4. **Ask upstream** — Eyra maintainers may know about this issue

---

## Files Created by TEAM_364-366

- `crates/userspace/eyra/cat/` — ✅ Works (uutils)
- `crates/userspace/eyra/pwd/` — ✅ Works (uutils)
- `crates/userspace/eyra/mkdir/` — ✅ Works (uutils)
- `crates/userspace/eyra/ls/` — ✅ Works (uutils)
- `crates/userspace/eyra/echo/` — ❌ Blocked (linker conflict)
- `crates/userspace/eyra/env/` — ❌ Blocked
- `crates/userspace/eyra/touch/` — ❌ Blocked
- `crates/userspace/eyra/rm/` — ❌ Blocked
- `crates/userspace/eyra/rmdir/` — ❌ Blocked
- `crates/userspace/eyra/ln/` — ❌ Blocked
- `crates/userspace/eyra/cp/` — ❌ Blocked
- `crates/userspace/eyra/mv/` — ❌ Blocked
- `crates/userspace/eyra/coreutils-true/` — ❌ Blocked
- `crates/userspace/eyra/coreutils-false/` — ❌ Blocked

---

## Summary

**The fix is NOT in disabling features or writing minimal code.**

**The fix is in understanding why Eyra's entry point conflicts with certain binaries.**

This requires deeper investigation into:
- Eyra's origin crate
- Rust's binary codegen
- The kernel's ELF loader

Future teams: DO NOT repeat the shortcut attempts. Investigate the root cause.
