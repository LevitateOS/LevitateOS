# I02 — Isolated recfstab source/layout inputs

Date: 2026-11-05. Owner lane: Integration (I02 only).
Scope: this note only. No edits to `tools/recipe`, `tools/ginux`,
`AcornOS`, `tools/recfstab`, or any other submodule; no commits.
`AcornOS/deps/recfstab.rhai` was read, not modified (adaptation decision:
none — see §5).

Read first: `docs/ginux-recipe-implementation-plan.md` (tasks I01, I02, R05),
`.artifacts/ginux-env/PRODUCED.md`, `AcornOS/deps/recfstab.rhai`, and the R05
`prepare_invocation` path in `tools/recipe` (`src/inputs/mod.rs`,
`src/inputs/discover.rs`, `src/inputs/config.rs`, `src/inputs/env.rs`,
`examples/prepare_inputs_demo.rs` — reference only).

## 1. Checkout-relative paths (a)

Recipe file: `AcornOS/deps/recfstab.rhai` (3124 B, mode 0644,
sha256 `91118aaf…0af91` — full hashes in §2). Lives in the `AcornOS`
submodule (`8de77da`, not tracked in the superproject index), so it is
out of I02 write scope.

Paths the recipe derives at runtime from the `BUILD_DIR` scope constant:

| Role | Host derivation | Host target |
| --- | --- | --- |
| Source dir | `dirname(dirname(BUILD_DIR))/tools/recfstab` | `tools/recfstab/` |
| Workspace manifest | `dirname(dirname(BUILD_DIR))/Cargo.toml` | `Cargo.toml` (workspace root) |
| Build output | `<monorepo>/target/release/recfstab` | `target/release/recfstab` |
| Install staging | `<acornos>/output/staging` | `AcornOS/output/staging/` |
| Installed file | `<staging>/usr/bin/recfstab` | `AcornOS/output/staging/usr/bin/recfstab` |

`BUILD_DIR` itself is pushed by the engine (`actions.rs`, `flow.rs`); in the
domain the R05 allowlisted env fixes `BUILD_DIR=/work/build`
(`src/inputs/env.rs`). The double-`dirname` derivation therefore resolves to
`/work` and `/` inside the domain — recorded as open gap H1 for I03 (§6),
not papered over here.

## 2. Cargo workspace manifest + source closure (a)

- Workspace manifest: superproject `Cargo.toml`
  (sha256 `673cdec2…40ca023`), `members` includes `tools/recfstab`;
  workspace lock `Cargo.lock` (118973 B) pins `recfstab 0.1.5` + `clap`.
- Package manifest: `tools/recfstab/Cargo.toml`
  (sha256 `aed4de6d…01ae`): `recfstab 0.1.5`, edition 2021, rust 1.93,
  one dependency `clap 4.4` (feature `derive`); dev-dependency
  `tempfile 3` (tests only, not needed for the build).
- Package source (submodule `tools/recfstab` @ `98926aa`, `v0.1.5-2-g98926aa`):
  `src/device.rs`, `src/error.rs`, `src/filter.rs`, `src/fstab.rs`,
  `src/lib.rs`, `src/main.rs`, `src/mount.rs`, `src/swap.rs` (951 lines total),
  `tests/integration.rs` (dev-only), standalone `tools/recfstab/Cargo.lock`
  (4905 B, pins `clap 4.5.54` from crates.io). Full per-file sha256 in the
  verification log (§7); e.g. `src/main.rs → 6ad9c52d…53c3c`.
- Registry: `clap` source is `registry+https://github.com/rust-lang/crates.io-index`
  (standalone lock line 58). There is **no** `vendor/` directory and no
  vendored registry in the checkout or `.artifacts`; `.cargo/config.toml`
  holds only an `xtask` alias. Offline resolution of `clap` is gap H4 (§6).
- R05 discovery over the recipe: **no `//! extends:` line, no `import "..."`
  lines** (`grep` confirms both absent), so discovery stages exactly the main
  recipe. Re-ran `prepare_inputs_demo` this session: 1 input,
  `/inputs/main.rhai`, sha256 `91118aaf…0af91` (matches the recipe file hash
  above), size 3124, `referenced_as main.rhai`; 7-entry allowlisted env;
  config 704 B (limit 4096); `no-leak: PASS`; over-limit/absolute-extends/
  missing-import rejections fire; ginux arm fails explicitly `not wired yet
  (R06 owns it)` with no host fallback. `R05-DEMO: PASS`.

## 3. Build/install locations: known + initially empty (b)

- Build output `target/release/recfstab`: **absent**
  (`ls: cannot access 'target/release/recfstab'`).
- Install staging `AcornOS/output/staging` (and
  `.../usr/bin/recfstab`): **absent** (`ls: cannot access` both).
- `tools/recfstab/.gitignore` ignores `/target`; no build artifacts are
  tracked. I05 starts from empty outputs as required.

## 4. Acquisition, cache, and toolchain inputs — honest record (c)

- Local acquisition behavior: `is_acquired`/`acquire` only test
  `is_file(<source>/Cargo.toml)` and set `ctx.source_path`. **No download,
  no clone, no hash verification happens.** The R05 1-input staging proves
  local-source *discovery*, not acquisition — no HTTP acquisition is claimed
  and none was exercised.
- I01 staged inputs (`.artifacts/ginux-env/`, `PRODUCED.md`): Rocky 10.2
  rootfs + index (re-verified pins at I01), `inputs/recipe` (`recipe 0.1.4`,
  debug 242 MB), gcc/cpp/glibc-devel/kernel-headers/libmpc/make RPMs,
  Rust 1.99.0 tarball (sha matches the published `.sha256`). Staged as files,
  **not** yet assembled into a consumable `/inputs` tree — that is R05/R06
  work, unchanged by I02.
- Caches: no cargo registry cache, no sccache, no pre-seeded `CARGO_HOME`
  was staged by I01 and none exists in the checkout; host `~/.cargo` must
  never enter the domain (R05 rejects secret/host paths). How the offline
  `cargo build` resolves `clap` is unresolved → H4.
- Worker binary: `.artifacts/ginux-env/inputs/` still contains only `recipe`;
  the domain `init` path `/inputs/recipe-worker` has no staged binary → H5
  (owned by R02/R06 follow-up, not I02).

## 5. Adaptation decision: NO adaptation (d)

No change to `AcornOS/deps/recfstab.rhai`. Rationale:

1. Write scope forbids it: the file lives in the `AcornOS` submodule and the
   task restricts writes to superproject `docs/`/`.artifacts` notes while
   prohibiting edits to any submodule.
2. Nothing in I02's done criteria needs it: locations are known and empty
   (§3), behavior and inputs are documented honestly (§4). A domain run is
   I03/I05 work and is unwired (R06/G09/G12a pending), so any edit now would
   be untestable.
3. No edit is required for R05: discovery succeeds with 1 input, no defines
   exist in the recipe, and it uses no network. R05 already records layout
   reconstruction data (`sources.txt`, `manifest.txt`, per-file extras) that
   R06/I03 will consume.
4. Any future adaptation must stay recipe-owned with no required defines and
   no Ginux-specific package logic; the observed host couplings are recorded
   as named I03 gaps below instead of being fixed blind.

## 6. Open gaps for I03 (handoff)

- H1 (recipe layout): double-`dirname(BUILD_DIR)` derivation, monorepo
  `target/` write, `AcornOS/output/staging` write — all assume a full host
  checkout; domain equivalents (`/inputs` closure, `/work/build`, `/out` +
  G13 export) undetermined.
- H2 (buildable closure): minimal declared per-file extra set that makes
  `cargo build --release -p recfstab` work from staged `/inputs` content
  (root manifest? workspace membership? which lockfile?) undetermined.
- H3 (workspace build): recipe invokes the whole-workspace manifest build;
  domain build must not require the full checkout.
- H4 (offline registry): `clap 4.5.54` has no vendored source or staged
  cargo cache; host `~/.cargo` must not leak.
- H5 (worker init): `/inputs/recipe-worker` binary not staged (R02/R06).
- H6 (storage/launch): retained backing, generation checkpoints, launch
  wiring pending (G11/G12a/R06) — I03 inventories against that state.

## 7. Verification (read-only commands + outputs)

All commands run from `/home/vince/Projects/LevitateOS`; nothing under
`docs/`, `.artifacts/`, `tools/`, or submodules was modified by them
(`cargo run` writes only `target/` (gitignored) and process tempdirs).

- `grep -n -E 'extends|import' AcornOS/deps/recfstab.rhai` →
  `NO extends/import matches`.
- `find tools/recfstab -type f -not -path '*/target/*' -not -path '*/.git/*' | sort`
  → `AGENTS.md Cargo.lock Cargo.toml CLAUDE.md .github/workflows/ci.yml
  .gitignore LICENSE README.md src/{device,error,filter,fstab,lib,main,mount,swap}.rs
  tests/integration.rs`.
- `ls -la target/release/recfstab` / `ls -la AcornOS/output/staging…` →
  both `cannot access … No such file or directory` (initially empty).
- `git ls-files AcornOS/deps/recfstab.rhai` → empty (submodule-owned);
  `git -C tools/recfstab rev-parse HEAD` → `98926aa…`;
  `git submodule status tools/recfstab tools/recipe` →
  `98926aa tools/recfstab (v0.1.5-2-g98926aa)`,
  `+a32a227 tools/recipe (0.1.3-22-ga32a227)`.
- `sha256sum AcornOS/deps/recfstab.rhai tools/recfstab/Cargo.toml
  tools/recfstab/src/*.rs tools/recfstab/tests/integration.rs Cargo.toml` →
  recorded in §1–§2 (recipe `91118aaf…0af91`, root manifest `673cdec2…40ca023`).
- `cargo run -p levitate-recipe --example prepare_inputs_demo` (full) →
  `R05-DEMO: PASS` (704 B config, `no-leak: PASS`, all three rejections,
  ginux arm `not wired yet`, 1 input staged); `-q … | head -18` re-showed the
  staged set: `/inputs/main.rhai`, 3124 B, digest matching the file hash.

Files created: `docs/ginux-i02-recfstab-inputs.md` (this note).
Files changed: none besides this note.
