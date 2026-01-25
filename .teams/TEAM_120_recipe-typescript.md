# TEAM_120: Recipe Scripting Language Investigation

## Status: PAUSED FOR REFLECTION

This document captures a deep investigation into replacing Rhai with a different scripting approach for recipe. The investigation revealed fundamental architectural questions that need consideration before proceeding.

---

## The Original Problem

Current Rhai recipes feel ad-hoc:
```rhai
let name = "rocky";
let version = "10.1";

fn acquire() { ... }
fn build() { ... }
fn install() { ... }
```

The user wanted something that feels like **React class components**:
```typescript
class Rocky extends Recipe {
  readonly name = "rocky"
  readonly version = "10.1"

  async acquire() { ... }
  async build() { ... }
  async install() { ... }
}
```

Rhai doesn't support classes. TypeScript does.

---

## Investigation: Do Helpers Need Rust?

### The Question
Before choosing a scripting language, we asked: **Do the helper functions (download, shell, verify_sha256, etc.) need to be in Rust, or could they be pure TypeScript?**

### Analysis of 51 Helper Functions

| Category | Functions | Deno/Bun Native? |
|----------|-----------|------------------|
| File I/O | exists, mkdir, rm, chmod, symlink, read_file | ✅ Yes |
| Hashing | verify_sha256, verify_sha512, verify_blake3 | ✅ Yes (Web Crypto + npm) |
| HTTP | download, http_get, github_latest_release | ✅ Yes (fetch API) |
| Process exec | shell, run_output, extract, git_clone | ✅ Yes (Bun.spawn/Deno.Command) |
| Environment | get_env, set_env | ✅ Yes (actually safer than Rust!) |
| Progress bars | spinners, progress indicators | ✅ Yes (npm packages) |
| Glob | glob_list | ✅ Yes (native in modern runtimes) |

### Conclusion
**There is NO technical reason the helpers must be in Rust.**

The current Rust helpers are just I/O orchestration - reading files, spawning processes, making HTTP requests. This is exactly what JavaScript runtimes are designed for.

---

## Architectural Options Explored

### Option 1: Rust + Embedded TypeScript (rustyscript)
- Embed V8 in Rust binary via rustyscript
- Helpers stay in Rust, exposed to TypeScript
- **Pros**: Single binary, sandboxed
- **Cons**: 20MB+ binary, complex build, bridge layer needed

### Option 2: Rust CLI + Spawn Deno/Bun
- Rust handles CLI, state, locking
- Spawns Deno/Bun to run TypeScript recipes
- **Pros**: Best of both worlds
- **Cons**: Two languages, IPC overhead

### Option 3: Pure TypeScript (Deno)
- Everything in TypeScript
- Deno's permission model for sandboxing
- **Pros**: Simple, native TS, sandboxed
- **Cons**: Requires Deno installed (or compiled binary)

### Option 4: Pure TypeScript (Bun)
- Everything in TypeScript
- Faster startup than Deno
- **Pros**: Fastest, simplest, native TS
- **Cons**: No sandboxing, less mature

### Option 5: Keep Rust + Different Scripting
- Lua (tiny, fast, but no classes)
- Starlark (designed for builds, but limited)
- Python (everyone knows it, but slow startup, dependency hell)

---

## Runtime Comparison

| Factor | Deno | Bun | Node.js |
|--------|------|-----|---------|
| Startup time | ~50ms | ~10ms | ~100ms |
| TypeScript | Native | Native | Needs tsx/ts-node |
| Single binary | `deno compile` | `bun build --compile` | pkg/nexe |
| Sandboxing | ✅ Permissions | ❌ None | ❌ None |
| Maturity | 5+ years | 2 years | 15+ years |
| Binary size | ~100MB | ~50MB | ~50MB |

---

## Decisions Made (Tentative)

During the investigation, the following preferences were expressed:

1. **Runtime**: Bun (fastest startup, simplest)
2. **Sandboxing**: Not needed (trusted recipe sources)
3. **Helper access**: Global functions (no imports needed in recipes)
4. **Location**: Replace Rust in `tools/recipe/`

---

## The Open Question

### Is TypeScript a Respectable Language for an OS Package Manager?

This is the question that needs to settle. Consider:

**Arguments FOR TypeScript:**
- npm, yarn, pnpm are all JavaScript/TypeScript
- Bun itself is a package manager written in Zig+TS
- Many modern CLI tools use TypeScript (Turborepo, etc.)
- Developer experience is excellent (IDE support, types)
- Fast enough for I/O-bound work (which package managers are)

**Arguments AGAINST TypeScript:**
- "Serious" package managers are compiled languages:
  - pacman (C)
  - apt (C++)
  - dnf (Python, but transitioning to C)
  - nix (C++)
  - cargo (Rust)
- JavaScript has a reputation as "not serious"
- Binary size is larger than native code
- Runtime dependency (unless compiled)

**The Middle Ground:**
- TypeScript for recipes (user-facing, needs to be easy to write)
- Rust for core engine (performance-critical, "serious")
- But the investigation showed there's no performance-critical code in recipe...

---

## What Was NOT Done

The following code changes were started but **reverted/abandoned**:

1. Added `rustyscript` to Cargo.toml (reverted)
2. Created `src/runtime/mod.rs` (abandoned)
3. Created `src/runtime/ops.rs` (abandoned)
4. Modified `src/lib.rs` (reverted)
5. Modified `src/core/lifecycle/mod.rs` (reverted)
6. Modified helper files to use new error types (partial, reverted)

**The codebase should be in its original state** with Rhai still in place.

---

## Next Steps (When Ready)

1. **Decide**: Is TypeScript acceptable for LevitateOS's package manager?
2. **If yes**: Proceed with Bun-based implementation per the plan
3. **If no**: Consider alternatives:
   - Keep Rhai but improve recipe structure
   - Use Lua (tiny, embeddable, mature)
   - Use Go (compiled, fast, but no classes)
   - Stay with Rust and accept Rhai's limitations

---

## References

- Plan file: `/home/vince/.claude/plans/typed-gathering-lighthouse.md`
- Current recipe code: `tools/recipe/src/`
- Example Rhai recipe: `leviso/deps/rocky.rhai`

---

## Timeline

- **2026-01-25**: Investigation started
- **2026-01-25**: Helper analysis completed (all can be TypeScript)
- **2026-01-25**: Architecture options explored
- **2026-01-25**: Paused for reflection on language choice
