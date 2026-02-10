# Recipe Engine Audit — 2026-02-10

Bug and gap audit of `tools/recipe/` and related recipe infrastructure.

## Critical

- [x] **stdout corruption** — `shell()` in `helpers.rs` uses `.status()` which inherits parent stdout. Recipe JSON output gets mixed with shell command output. `run_recipe_json_with_defines()` parses stdout as JSON, so any recipe that prints to stdout causes parse failures.
  - File: `tools/recipe/src/helpers/util/shell.rs`
  - Fix: Redirect shell stdout → stderr via `/dev/stderr` fd duplication. `shell_output()` unchanged (captures stdout intentionally).
  - **FIXED** — `stderr_as_stdout()` helper, applied to `shell()`, `shell_in()`, `shell_status()`, `shell_status_in()`.

- [x] **No Rhai engine limits** — No `set_max_operations()`, `set_max_call_levels()`, or `set_max_string_size()` configured. A malicious or buggy recipe can infinite-loop or OOM the host.
  - File: `tools/recipe/src/lib.rs`
  - **FIXED** — `set_max_operations(2_000_000)`, `set_max_call_levels(64)`, `set_max_string_size(64MB)`.

- [ ] **Hardcoded absolute paths** — `rocky.rhai` and `alpine.rhai` ctx blocks contain `/home/vince/Projects/LevitateOS/...` paths baked in via `ctx::persist`. Breaks on any other machine.
  - Files: Various `.rhai` recipe files
  - Fix: Use relative paths or `RECIPE_DIR`/`BUILD_DIR` constants instead.
  - **Planning effort: 3/5** — Need to audit every .rhai file for absolute paths, change ctx::persist to write relative paths, ensure all consumers resolve relative→absolute at load time. ~15 files affected.

- [ ] **Shell command injection** — Recipe `.rhai` files build shell commands via string concatenation (`"tar xf " + path`). Paths with spaces or special characters result in command injection.
  - Files: All `.rhai` recipes using `shell()` with concatenation
  - Fix: Add a `shell_args()` helper that takes an argv array, or quote paths.
  - **Planning effort: 4/5** — Requires new Rhai helper (`shell_argv` or `shell_quoted`), then auditing and rewriting every `shell()` call across all recipes. ~50+ call sites. Risk of breaking working recipes.

- [x] **Temp dir leak** — When no `--build-dir` is specified, executor creates a tempdir then calls `temp.keep()`, leaking it on disk. Every failed recipe run leaves debris in `/tmp`.
  - File: `tools/recipe/src/bin/recipe.rs`
  - Note: `keep()` is the intended API (`into_path()` is deprecated). The temp dir is cleaned by the recipe's `cleanup()` phase. The real leak is on failed runs — but fixing that requires restructuring the engine lifetime. Documented as accepted behavior.
  - **WONTFIX (accepted)** — cleanup() phase handles it; catastrophic failures leave one dir per run in /tmp, which is normal OS behavior.

## High

- [ ] **Silent shell failures** — `packages.rhai` ignores `shell_status()` and `shell_status_in()` return values. Failed package installs silently produce a broken rootfs.
  - File: `AcornOS/deps/packages.rhai`, `IuppiterOS/deps/packages.rhai`
  - Fix: Check return values and bail on non-zero exit.
  - **Planning effort: 2/5** — Grep for `shell_status` in .rhai files, add `if status != 0 { throw "..."; }` after each. Straightforward but tedious.

- [ ] **Symlink attacks in extraction** — tar/zip extraction doesn't check for symlinks escaping the build directory. A crafted tarball could write outside the intended path.
  - File: `tools/recipe/src/helpers/acquire/extract.rs` (or wherever tar extraction lives)
  - Fix: Validate extracted paths stay within build dir, or use `--no-same-owner --no-overwrite-dir` flags.
  - **Planning effort: 3/5** — Need to understand the native tar/zip extraction code, add path canonicalization checks. Must not break legitimate symlinks within packages.

- [x] **Lock file stale timeout too long** — 24-hour stale timeout means a build crash leaves locks that block the next run for a full day.
  - File: `tools/recipe/src/core/lock.rs`
  - **FIXED** — Reduced `STALE_LOCK_AGE_SECS` from 86400 (24h) to 7200 (2h).

- [x] **No git clone timeout** — `git clone` in recipes can hang forever on network issues with no timeout.
  - File: `tools/recipe/src/helpers/acquire/git.rs`
  - **FIXED** — Added `http.lowSpeedLimit=1000` + `http.lowSpeedTime=600` (10 min) via `git -c` flags on both `git_clone()` and `git_clone_depth()`.

- [ ] **Zero E2E extends tests** — The `//! extends:` mechanism has unit tests for parsing but no integration test that runs a base+child recipe through the full executor pipeline.
  - File: `tools/recipe/tests/`
  - Fix: Add an integration test with a real base.rhai + child.rhai that verifies function override, scope inheritance, and ctx persistence.
  - **Planning effort: 3/5** — Write two temp .rhai files (base + child), run through RecipeEngine, assert ctx values. Main complexity is setting up the test fixture correctly.

- [x] **AST `+=` merge semantics undocumented** — The extends merge relies on Rhai's `+=` operator letting child `let ctx` shadow base `let ctx`. This is implementation detail, not an API guarantee. A Rhai version bump could silently break extends.
  - File: `tools/recipe/Cargo.toml`
  - **FIXED** — Pinned Rhai to `=1.19.0` with comment documenting the dependency on AST merge semantics. Regression test should still be added (see E2E extends tests above).

## Low

- [ ] **FileModuleResolver vs extends conflict** — Two independent module resolution mechanisms (Rhai's `import` and `//! extends:`) that could produce confusing behavior if both are used in the same recipe.
  - Fix: Document that `import` and `extends` should not be mixed, or unify them.
  - **Planning effort: 1/5** — Just add a doc comment or error check.

- [ ] **Parallel race on `.packages-version`** — Shared state file written by multiple recipe runs with no file locking. Concurrent builds could corrupt it.
  - Fix: Use advisory file locks or atomic writes.
  - **Planning effort: 2/5** — Add `fs2::FileExt::lock_exclusive()` around the read-modify-write of the file. Already have `fs2` in deps.
