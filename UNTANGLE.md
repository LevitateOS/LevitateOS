# Untangle log

## Scope

- **Target:** the install UI for LevitateOS and AcornOS, with its wiring, taken as one system:
  - `tui/` (submodule): `kit/core`, `kit/presets-install`, `kit/testing`, `apps/live-tools/install-docs`, `apps/install/disk-plan`, `apps/runtime/recab`.
  - `distro/builder/install-split-pane` (the split-pane helper, `levitate-install-docs-split`).
  - The builder's install-UX payload code: `distro/builder/src/pipeline/live_tools.rs`, `src/live_ux.rs`, `src/artifact/live_overlay.rs` (UX parts), `src/recipe/tools.rs` and `recipes/install-split-pane.rhai` where they build UI binaries.
  - Contract types for install UX: `distro/contract/src/{schema,variant}.rs` (`InstallExperience`, `InstallDocsFrontend`, `RuntimeActionContract::InstallModePayload`, `live_tools_ux`).
  - Variant install profiles: `distro/variants/{levitate,acorn}/ring2/products.toml` (`live_tools_*` runtime profiles), `distro/variants/acorn/ring2/install-guide.txt`, live overlays.
  - xtask tasks that drive the UI: `xtask/src/tasks/{tui,inspect_tui,inspect_docs,check,generate}.rs`.
- **Out of scope as targets** (links to them still count as findings): the rec* tools themselves (finished untangle in `docs/history/UNTANGLE.md`), `docs/content` (the docs data the install-docs app renders), `docs/website`, `testing/*` harnesses other than where they drive the UI.
- **Additions chosen:** Comments (remove comments that only restate the code) and Tests (remove tests that wouldn't catch a real bug, and fix the ones that fail without one).
  - AGENTS.md prohibits unit tests outside ginux, so "Tests" applies to the PTY/integration tests (`install-viewer.chrome.pty.test.ts`), snapshot checks (`inspect:check`), and the `kit/testing` package.
- **Repo rules that bind the refactor:** commit on the checked-out branch (no new branches), submodules first then superproject pointers, no unit tests. Another session is concurrently editing `distro/contract`, `distro/builder/src/pipeline/products.rs`, `xtask/` (rpm_dvd → rpm_set work): commit only our own hunks.
- **Tooling for later phases:** `.artifacts/untangle-ui/` (gitignored)
  - `scripts/skeleton.py` — body-stripped TS/TSX/Rust skeleton of the scope → `.artifacts/untangle-ui/skeleton/`.
  - `scripts/depgraph.py` — TS import graph (file and package level), cycles, fan-in → `.artifacts/untangle-ui/map/`.
  - `scripts/history.sh` — churn and co-change for `tui/` and the builder UX files → `.artifacts/untangle-ui/map/`.
  - `baseline/` — command outputs and render snapshots listed below.

## Baseline

Taken 2026-11-06.

- **Superproject:** `master` @ `90229932fddab1213dd0595153abb3e7ab923afb`. Unrelated dirty files from another session: `Cargo.lock`, `xtask/{Cargo.toml,src/app.rs,src/cli/*,src/tasks/mod.rs,src/tasks/base.rs}`, `distro/variants/levitate/ring3/{sources.toml,keys/}`, `tools/recipe` pointer.
- **Submodules** (all on `codex/streamline-development`):

  | repo | SHA | state |
  |---|---|---|
  | tui | cfb5a5a | clean |
  | distro/builder | 3963bf9 | dirty (other session: `products.rs`) |
  | distro/contract | bb90e68 | dirty (other session: rpm_set) |
  | docs/content | bd9e103 | clean |

- **Typecheck** (`bun run typecheck`): pass for `tui/kit/core`, `tui/apps/install/disk-plan`, `tui/apps/live-tools/install-docs`, `tui/apps/runtime/recab`. `cargo check -p install-split-pane`: pass.
- **Lint** (`bun run lint`): pass for disk-plan and recab; **fail** for kit/core and install-docs — their scripts point at `.oxlintrc.json` at the repo root, which does not exist.
- **Format** (`bun run format:check`): **fail** in all four packages. kit/core and install-docs point at a missing `.oxfmtrc.json`; disk-plan and recab run with oxfmt defaults and report unformatted files.
- **install-docs `inspect:check`** (`scripts/verify-inspect-snapshots.sh`): **fail** — `001-installation-disk.txt` differs from `snapshots/inspect/120x36/` (docs content moved on; snapshots stale).
- **install-docs `test:e2e`** (`install-viewer.chrome.pty.test.ts`): **0 pass, 3 fail** — the test runs `xtask docs inspect`, renamed to `xtask inspect docs` in tui `60669a0`; `unrecognized subcommand 'docs'`.
- **Renders** (`.artifacts/untangle-ui/baseline/`):
  - `docs/run-*/`: `cargo xtask inspect docs --columns 120 --rows 36` → all 35 nav pages render (exit 0).
  - `disk-plan/run-*/`: `cargo xtask inspect tui install-disk-plan --columns 120 --rows 36` → welcome page renders (10-page flow, disk `/dev/sda`, mode `ab`).
  - `recab/run-*/`: `cargo xtask inspect tui --cwd tui/apps/runtime/recab --command 'bun run main.ts'` → **crashes**: "Invalid hook call … more than one copy of React", then `Unhandled TUI error: null is not an object (evaluating 'dispatcher.useContext')`. recab is not in the root `package.json` workspaces.
- **Live ISO:** not built or booted. QEMU tools under `.artifacts/tools/current` point into root-owned `.artifacts/ginux-export`, unreadable by this user; the user chose to skip boot checks. The install-UX profile scripts, entrypoint and split pane are therefore only verified by compilation.

## Map

Views (rerun with the scripts in Scope): skeleton `.artifacts/untangle-ui/skeleton/` (~15k → ~5k lines), import graph `.artifacts/untangle-ui/map/{deps,fanin,packages,cycles}.txt`, history `.artifacts/untangle-ui/map/{churn,cochange}-*.txt`.

| Area | Size | What it is | Depends on |
|---|---|---|---|
| A. `tui/kit/core` | 85 files, 5.5k lines | Ink/React runtime, primitives, components, "chrome" box-drawing renderer, theme, hooks, patterns (wizard, two-pane, review) | ink, react |
| A'. `tui/kit/presets-install`, `tui/kit/testing` | 7 + 6 files, 1 line each | Stubs: every file is an empty barrel; `build` echoes "stub" | — |
| B. `tui/apps/live-tools/install-docs` | 58 files, 4.3k lines | LevitateOS docs/installer guide viewer (sidebar nav + rich page renderer), bundled with Bun onto the LevitateOS ISO | `@levitate/tui-kit` (28 imports), `@levitate/docs-content` (26) |
| C. `tui/apps/install/disk-plan` | 11 files, 1.7k lines | 10-page partition-planning wizard over `recpart` JSON; not shipped on any ISO | tui-kit (2), generated `contract/recpart.ts` |
| C'. `tui/apps/runtime/recab` | 13 files, 0.5k lines | A/B slot status TUI over `recab`; not shipped; crashes at baseline (duplicate React; not in root workspaces) | tui-kit (2) |
| D. Split pane | `install-split-pane/src/main.rs`, 0.8k lines | Rust ratatui+vt100+PTY two-pane host: shell left, docs right; `--smoke` | crossterm, portable-pty, vt100 |
| E. Builder UX payload | `live_tools.rs` (0.7k), `live_ux.rs`, `live_overlay.rs` (0.7k) | Generates entrypoint, profile.d hook, docs launcher scripts as Rust format strings; bundles the Bun app; overlay autologin | contract, recipe tools |
| F. Contract + variants | `schema.rs`/`variant.rs` install types; `products.toml` `live_tools_*` profiles | Declares install experience, docs frontend, split-pane binary per distro | — |
| G. xtask | `tui.rs`, `inspect_tui.rs`, `inspect_docs.rs`, `check.rs`, `generate.rs` | Launch/render/typecheck the apps; generate `recpart.ts` | tui paths by string |

**Hotspots.** tui history was re-rooted (17 commits total), so churn is thin: `disk-plan-screen.tsx` (7), `runtime-recab-screen.tsx` (4), `contract/recpart.ts` (4). In the builder, `live_overlay.rs` (23) and `live_tools.rs` (22) are the volatile files, and the superproject's tui pointer moves together with `xtask/src/tasks/{tui,docs}/inspect.rs` (4×) — xtask and the TUI change together across a repo boundary.

**Graph.** No import cycles. Fan-in concentrates on `kit/core/src/utils/clamp.ts` (20), `theme.ts` (17), `app/app-provider.tsx` (17). 24 files nothing imports: entrypoints, plus all of `presets-install` and `testing` (dead), `install-docs/.../primitives/index.ts`, and recab's three entry/barrel files. Each app reaches `@levitate/tui-kit` only through the barrel; install-docs is the only heavy kit consumer.

**Where leverage looks highest.** (1) The seam between Rust and the UI: shell scripts emitted from Rust format strings, env-var protocols (`LEVITATE_INSTALL_*`, `LEVITATE_DOCS_PLAIN`), command names resolved by string at three layers (profile hook → entrypoint → split pane → docs). (2) Two distros, two different docs frontends (Bun TUI vs `less`). (3) kit/core's size vs its two real consumers. (4) Dead/stub packages and broken checks (lint/format configs missing, e2e and snapshot checks failing).

## Problems

Totals: **649 entries — 364 hidden, 47 needless, 130 surplus, 108 special** (636 from the area passes after dropping 9 cross-area duplicates, plus 13 from the completeness pass). The special-case count, 108, is the number the refactor works to bring down.

**Biggest clusters** (detail under `### Found during the refactor

- [hidden] N001 `docs/content` highlighted code ends a backslash-continued line with `\[[/]]`; the `[[fg=]]` markup reads `\[[` as an escaped `[[`, so code blocks show a literal `[[/]]` (seen in cli-reference's "Inputs and Results"). The producer's escaping and the parser's grammar disagree, and nothing checks them against each other.

## Fixes`):
1. **The launch chain from login to install screen has no owner.** tty selection, command choice, shell choice and env (`LEVITATE_INSTALL_*`, `LEVITATE_DOCS_PLAIN`, TERM/COLORTERM/FORCE_COLOR/NO_COLOR) are decided separately in the profile hook, serial autologin, entrypoint, split pane, docs launcher scripts and xtask, all as Rust format strings or copies; paths differ per distro and two are dead (systemd agetty has no `-l`; Acorn's ttyS0 branch can't run).
2. **The UI can't fit where it's shown and lies about colour.** The docs app needs 88 columns but gets half the console (verified: "Terminal too small"); ansi256 colours are emitted as bare numbers Ink ignores, hidden by forcing `FORCE_COLOR=3` in five places; `LEVITATE_DOCS_PLAIN` is never read.
3. **Checks that can't catch anything.** lint/format point at missing configs; the PTY test calls a renamed xtask subcommand; snapshots embed the page count; `inspect tui --input` writes keys to the screen, not the app; split-pane `--smoke` passes on tty echo; recab is typechecked but crashes on launch (kit lists React as a dependency, recab isn't a workspace member).
4. **Duplicated facts across the TS/Rust/repo boundaries.** recpart/recab modes, tokens, command lines and labels copied into the apps; the app list in six places; the split-pane name in ~9; theme colours in three RGB tables; block layout computed twice (measure vs render) in install-docs.
5. **Surplus kit and app scaffolding.** All of `kit/core/patterns`, ~20 components, ~25 helpers, two stub packages, ~20 barrels, plugin/registry/profile layers with one implementation (the `acorn` docs profile can't be selected), docs/README/architecture files that describe code that no longer exists.
6. **Distro-blind UI.** LevitateOS gets a Bun TUI, AcornOS a `less` text file; disk-plan defaults to `ab` and isn't shipped on either ISO; LevitateOS branding and `/usr/local/share/levitate` paths ship on AcornOS.

Real bugs found while auditing (from reading code unless noted): recab TUI crash (run); `inspect tui --input` dead (run); docs pane "Terminal too small" at half width (run); `recipe/tools.rs` adds `recpart` to one of two package maps; live-net-setup awk programs rejected by gawk (run on host gawk); split pane PTYs one row taller than drawn; `truncate_for_error` can panic on a UTF-8 boundary; any failure in the UX `exec` chain ends the login with no fallback shell; `rich-text.tsx:45` returns before hooks; `AsyncWizardFlow` ignores `defaultIndex`; QA-answer links selectable but never highlighted; recab mutation result overwritten by the status refresh; disk-plan Next on result-handoff lands on failure-diagnostics.

| Area | hidden | needless | surplus | special |
|---|---|---|---|---|
| Cross-area | 7 | 0 | 0 | 2 |
| A. kit | 95 | 18 | 43 | 32 |
| B. install-docs | 97 | 15 | 44 | 23 |
| C. disk-plan + recab | 92 | 4 | 25 | 16 |
| D. Rust/shell wiring | 71 | 10 | 15 | 27 |

**Every entry has moved to `## Fixes`**, under the fix that solves it or under *Leave as is*, prefixed with an ID: `X` cross-area, `A` kit, `B` install-docs, `C` disk-plan, recab and their xtask code, `D` Rust/shell wiring, `R` completeness pass, numbered in the order they were logged. The area notes below keep the path shorthands those entries use (cross-area entries use full paths).

### A — tui/kit (core, presets-install, testing)

Path shorthand: `core/…` = `tui/kit/core/…`; `presets/…` = `tui/kit/presets-install/…`; `testing/…` = `tui/kit/testing/…`; `install-docs/…` = `tui/apps/live-tools/install-docs/src/…`; `disk-plan/…` = `tui/apps/install/disk-plan/src/…`; `recab/…` = `tui/apps/runtime/recab/…`. Consumers were read only to judge how kit exports are used.

### B — install-docs findings

Area: `tui/apps/live-tools/install-docs` plus the xtask code that drives it and its ISO bundling.
Path convention: `install-docs/...` = `tui/apps/live-tools/install-docs/...`; `src/...` = `install-docs/src/...`; `ink/...` = `src/presentation/ink/...`. Every other path is relative to the repo root.

### C. disk-plan + recab TUIs (and the xtask code that drives them)

Scope: `tui/apps/install/disk-plan/**`, `tui/apps/runtime/recab/**`, `xtask/src/tasks/{tui,inspect_tui,check,generate}.rs`, `xtask/src/cli/types.rs` (TuiApp). Paths below are relative to the repo root; `dp/` = `tui/apps/install/disk-plan/`, `rc/` = `tui/apps/runtime/recab/`.

Checked live: `cargo xtask inspect tui install-disk-plan --input 'n'` and `--input 'j'` both leave the app on the welcome page, and the keys show up as printed text (`nn`) at the bottom of the screen (see the inspect entries). `oxfmt --check` fails on `dp/src/contract/recpart.ts`, `dp/src/presentation/disk-plan-screen.tsx` and `rc/domain/contract.ts`. `rc/node_modules/@levitate/tui-kit/node_modules/react` is a second React next to `rc/node_modules/react`.

- Verified done, nothing left: B010 (`CONFIRM_TOKEN` generated, `dp/src/contract/recpart.ts:71`), B013 (schema versions generated), B015/B016 (no inventory/preflight re-implementation; `listDisks` and the dry-run are used), B021 (no `REPO_ROOT`), B039/B040 (no postinstall-tools, no kit shim), B060/B046/B047 (no hand validators), B080–B083 (removed), B130 (`exitCode: null`). Kept items B019, B023, B126, B127, B128 are still present and are listed above.

---

Counts: [hidden] 94, [needless] 4, [surplus] 25, [special] 16. Total 139 (plus one "verified done" note).

### D — Rust/shell wiring: live login → install screen

Scope: `distro/builder/install-split-pane/`, `recipes/install-split-pane.rhai`, `src/recipe/tools.rs`,
`src/pipeline/{live_tools,config,overlay,products}.rs`, `src/live_ux.rs`, `src/artifact/live_overlay.rs`,
contract `schema.rs`/`variant.rs`, variant `products.toml`/`scenarios.toml`/`install-guide.txt`/`overlays/`,
and consumers in `testing/install-tests` and `xtask`. Audit only, read from the code; nothing was built or booted,
so every runtime claim below comes from reading the code. Paths are relative to the repo root; `builder/` = `distro/builder/`.

---

Counts: hidden 72, needless 10, surplus 15, special 27 (124 entries). Every runtime claim (respawn loop, tty1 drop-in precedence, the recpart build failure, smoke vacuity) is inferred from code; nothing was booted. The two awk programs were run through host gawk.

### Completeness pass

Two fresh agents that had not seen this list re-audited the highest-churn files: R1 the builder payload (`live_tools.rs`, `live_overlay.rs`; 101 entries) and R2 the three screens (`disk-plan-screen.tsx`, install-docs `presentation/ink`, `runtime-recab-screen.tsx`; 134 entries). Both saw a few lines of this file through a repo-wide grep; each says it re-verified overlapping items against the code. Compared by file:line overlap (`.artifacts/untangle-ui/scripts/novel.py`), then by hand.

- R1 added 11. The pattern: the first pass read `live_overlay.rs` only for its install-UX parts and missed the generic live-overlay special cases. R1 read both files whole, which closes that gap.
- R2 added 2 (about 1.5%), so the screens' list has converged.

## Fixes

**24 fixes solve 639 of the 649 problems; 10 are left as they are (7 of them special cases).** Every entry from `## Problems` sits exactly once below, under its fix or under *Leave as is*, with its ID.

**Not reviewed.** The user asked for this section to be written and committed without a review stop, so these fixes went in on the plan below. The calls the code couldn't settle are listed under *Decisions*; change any of them here before running untangle-refactor.

Line numbers are the audit baseline's. Since then `distro/builder` (`products.rs`, 13 lines removed), `distro/contract` (`schema.rs`, `variant.rs`, rpm_set additions) and `xtask/src/cli/types.rs` (18 lines added) have moved a little; `tui` hasn't changed.

### Decisions taken without review

- **Kit membership rule (F8, F11).** The kit holds generic, vocabulary-free code that more than one screen can use; anything that names docs, sidebars or one app's screens lives in that app. Generic code now in install-docs (the styled-run word-wrapper) moves into the kit, and docs-only code in the kit (the markup parser, pager status format, docs nav defaults) moves out.
- **Two docs frontends stay (F19).** LevitateOS keeps the Bun TUI and AcornOS the text guide, but both get one argv contract and one lifetime. Putting the TUI on AcornOS needs Bun on musl and the docs content made distro-aware, which is a product choice for the overhaul.
- **Whether disk-plan ships is left to the overhaul (F22).** After F22 it's one line in the variant. Until then the READMEs say where each app ships.
- **Guest command names become distro-neutral (F19):** `install-ux`, `install-split`, `install-docs`, `/usr/share/install-ux/`. The `levitate-*` names go from both ISOs.
- **The login → install-screen chain moves from a profile.d hook to getty autologin (F19).** The getty runs the launcher directly, and the launcher falls back to the login shell.
- **The bun PTY test is deleted, not repaired (F4).** Full-screen snapshots at the real widths cover what it checked.
- **recpart and recab get small output additions (F14):** a handoff mount root and next steps, a recab `schema_version` and clap doc text exposed to the generator. These touch the `tools/` submodules.
- **Generic live-overlay special cases stay out (Leave as is).** The live-overlay fix (F21) covers only the files the install UX depends on.

### Requirements the special cases reveal

Read together, the special cases describe what the system needs and never stated. Each fix below is built to meet these:

- A failure anywhere in the launch chain leaves the user a working shell; it never ends the login or loops through getty. (`D042`, `D044`, `D079`, `D080`)
- The UI fits the space it is shown in: the docs app works at the split pane's right-pane width, and measured rows equal drawn rows. (`X009`, `A079`, `A111`, `A112`, `A113`, `A114`, `B063`, `A184`, `A187`)
- The program that owns the terminal decides what the terminal is; nobody forces colour, and colour code works at every level. (`X004`, `A084`, `A085`, `A093`)
- Distro differences come from the variant, not from init-system code paths or copied strings. (`D006`, `D030`, `D031`, `D046`, `D112`)
- Tests run the shipped path, and shipped code carries no test-only branches; checks wait on state, not timers. (`D008`, `D043`, `D092`, `D098`, `B147`, `B150`, `B151`, `B152`, `B156`, `B160`, `B161`, `C104`)
- Each keypress goes to exactly one handler, chosen by focus; key hints come from the bindings they describe; the host and its children agree on key encodings. (`A137`, `A138`, `A139`, `B104`, `B113`, `C058`, `C062`, `D082`, `D083`)
- Bad input fails fast with a message instead of falling back silently. (`A085`, `B084`, `B085`, `B086`, `B088`, `B089`, `B090`, `B092`, `C048`, `D081`, `D087`, `R012`)
- A render error exits non-zero with its stack and restores the terminal; hooks run in the same order every render. (`A162`, `A175`, `A181`, `B075`)
- A headless driver never touches a real device. (`C102`)
- Host state never leaks into the ISO: no host binaries, no host DNS config, no network installs during a build. (`D067`, `D068`, `D072`)
- Each guest file has one writer, and one shape per job: one fallback label, one empty-list text, one way to compute pane geometry. (`A176`, `A185`, `B030`, `C080`, `R006`, `R013`)

### Order

Fixes that others build on come first; among the rest, the most problems for the least risk. After F1–F4, the TypeScript track (F5–F16) and the Rust track (F17–F24) are independent, except that F13's launcher side lands with F19 and F20 (stop forcing colour) waits for F6 (colour that works without forcing).

| # | Fix | Problems | Special | Depends on |
|---|---|---|---|---|
| F1 | One TUI workspace: one React, one lockfile, one toolchain, one package shape. | 43 | 1 | — |
| F2 | One app manifest: each app declares its name, entry and needs once, and xtask and the builder read it. | 15 | 0 | F1 |
| F3 | One xtask TUI driver for run, inspect and capture. | 23 | 5 | F2 |
| F4 | Checks that fail on real bugs: `cargo xtask check tui`. | 22 | 7 | F1, F3 |
| F5 | Delete the kit surface no app uses. | 49 | 9 | F4 |
| F6 | One colour pipeline from theme intent to Ink. | 39 | 5 | F5 |
| F7 | Keymap as data, with one dispatcher and focus scopes. | 30 | 5 | F5 |
| F8 | No app vocabulary in the kit. | 17 | 0 | F5, F6 |
| F9 | One frame and width model, and one app shell that fits the space it gets. | 50 | 9 | F5, F6, F7 |
| F10 | The kit owns the app lifecycle: args, TTY, errors, exit codes and processes. | 27 | 5 | F5, F7 |
| F11 | One generic component per job in the kit. | 19 | 7 | F5, F8, F9 |
| F12 | install-docs lays each block out once: block → lines. | 72 | 3 | F9, F11 |
| F13 | Apps get distro identity from their launcher, and install-docs fails fast. | 22 | 7 | F10 |
| F14 | Facts owned by recpart and recab reach the TUIs only through `cargo xtask gen`. | 27 | 1 | F1, F2 |
| F15 | disk-plan as an explicit flow graph, with IO kept apart from pages. | 40 | 6 | F7, F10, F14 |
| F16 | recab TUI: results stay visible, and status is a separate panel. | 7 | 3 | F10, F14 |
| F17 | Guest scripts are files, with one placeholder scheme that is checked. | 4 | 0 | — |
| F18 | The install experience is declared once, in the variant's products.toml. | 13 | 0 | — |
| F19 | One owner for the chain from login to install screen. | 43 | 6 | F17, F18 |
| F20 | The program that owns the terminal sets TERM, and nothing forces colour. | 6 | 1 | F6, F3, F19 |
| F21 | Each live-ISO file has one writer, and init and distro differences come from config. | 21 | 5 | F17, F19 |
| F22 | ISO payload is built by recipes and one generic TUI-app action. | 18 | 3 | F2, F18 |
| F23 | The split pane is a robust terminal host. | 16 | 8 | F19 |
| F24 | Install tests exercise the real launch, not a smoke stand-in. | 16 | 5 | F19, F23 |
| — | Leave as is | 10 | 7 | — |

### F1. One TUI workspace: one React, one lockfile, one toolchain, one package shape.

Solves 43 problems (20 hidden · 22 surplus · 1 special).

`@levitate/tui-kit` lists `ink` and `react` as `peerDependencies`. Every app (install-docs, disk-plan, recab) is a member of the root workspace and depends on the kit with `workspace:*`. One root `bun.lock` replaces the per-package lockfiles, and xtask runs one `bun install --frozen-lockfile` at the root before any TUI command: no per-package installs and no "node_modules exists" probe. The two stub packages (`presets-install`, `testing`) are deleted. One `oxlint`/`oxfmt` config sits at the repo root, both tools are pinned once in the root `package.json`, and every tui package's `lint`/`format` scripts use them. Packages emit nothing: no `build`/`clean` scripts, `dist/`, `declaration`/`outDir` or `.turbo` entries. turbo no longer runs the TUI's typecheck or lint (F4 defines the UI check), and biome stays only for docs/content. Kit and apps typecheck against `@types/bun`, so there's no hand-written `globals.d.ts`. Every app has the same shape: a `src/main.ts` entry, a `bin`, and no barrel `index.ts`. Stray files go: the subdirectory `ci.yml`, `design_goal.png`, the `levitate-docs` ignore line, `pre-commit.sh`, and the stale disk-plan lockfile.

**Done when:** recab launches without the "Invalid hook call" crash; `find tui -name bun.lock` finds nothing; `kit/presets-install` and `kit/testing` are gone; lint and format-check pass in every tui package with the shared config; no tui package has a `build` script or emit settings.

**Depends on:** nothing.

- `X005` [hidden] Root `package.json:13-19` runs `turbo dev|build|typecheck|lint|check` (`turbo.json`) while `xtask/src/tasks/check.rs` runs its own typecheck loop over a different package list; AGENTS.md says workflows go through `cargo xtask`. Two definitions of "check the UI".
- `X006` [hidden] Root `package.json:21` pins `@biomejs/biome` (used only by `docs/content/biome.json`), while every tui package lints/formats with `oxlint`/`oxfmt`; kit/core and install-docs point `--config` at root `.oxlintrc.json`/`.oxfmtrc.json` that don't exist, disk-plan and recab run oxlint/oxfmt with defaults. Three lint/format setups across one UI.
- `A001` [hidden] `core/package.json:23-26` declares `ink` and `react` as `dependencies`, not peers, so `tui/kit/core/node_modules/react` exists. Bun resolves kit source files to their real path, so the kit's `import "react"` resolves from `tui/kit/core/node_modules` (root store `react@19.3.1`). recab is not a workspace member (root `package.json:4-12`), so its own `bun install` gives it `recab/node_modules/react@19.2.4`, and its `@levitate/tui-kit` copy (`recab/node_modules/@levitate/tui-kit/`, per-file symlinks back into `tui/kit/core`) still reads kit React from the kit's folder. That makes two Reacts and the "Invalid hook call" crash. disk-plan and install-docs work only because, as workspace members, their `react` symlinks point at the same root store path as the kit's. Whether a kit consumer works depends on where it sits in the workspace, and nothing in the kit says so.
- `A002` [hidden] `recab/package.json:19-23`, `disk-plan` `package.json` and `install-docs` `package.json` each list `ink` and `react` again next to `@levitate/tui-kit`, and the apps import `ink` directly (`disk-plan/presentation/disk-plan-screen.tsx:1` `Box, Text, useApp, useInput`; `recab/presentation/runtime-recab-screen.tsx:1`; `install-docs/presentation/ink/screens/install-viewer.tsx:1`). The kit and each app must resolve the same ink/React instance, but each manifest pins its own range.
- `A003` [hidden] `core/package.json` consumers use `"file:../../../kit/core"` (`recab/package.json:20`, disk-plan, install-docs). install-docs uses `"workspace:*"` for docs-content in the same manifest. The `file:` protocol makes a non-workspace app take a copy, while a workspace app gets a symlink, and the code doesn't show that difference.
- `A004` [hidden] `core/bun.lock:128` pins `react@19.2.4`, but the kit runs on the root store `react@19.3.1` (`tui/kit/core/node_modules/react` symlink). The kit lockfile is a standalone leftover that doesn't describe what is installed. The repo root has no `bun.lock`, and `disk-plan/../bun.lock` and `recab/bun.lock` are separate lockfiles too.
- `A005` [hidden] `xtask/src/tasks/check.rs:8-14,45,56-60`: `ensure_node_modules` runs a bare `bun install` inside `tui/apps/runtime/recab`, which creates the separate `node_modules` that causes the crash. The `typecheck` step then passes, because types don't detect two runtime React copies. The check that sets up recab is what breaks it, and it can't catch the breakage.
- `A006` [hidden] `core/src/globals.d.ts:1-26` hand-declares `process` (with `stdout.on?`, `off?`, `removeListener?` optional) because the kit has no `@types/node` (`core/node_modules/@types` has only `react`). When an app compiles kit source, `@types/node` or `@types/bun` supplies `process` instead. The same code is typechecked against two different `process` shapes, and the fallbacks at `core/src/app/app-provider.tsx:73-77` exist only for the hand-written one.
- `A009` [hidden] `turbo.json` `lint` plus root `package.json` `"lint": "turbo lint"` runs the kit's broken `lint`, so the root lint fails because of a path in the kit's manifest.
- `A010` [surplus] `core/package.json:13-14`: `clean` and `build` emit `dist/` with declarations, but `exports` (`:8-11`) point at `./src/*.ts`, so nothing consumes `dist/`. `turbo.json` `typecheck.dependsOn: ["^build"]` makes every app typecheck run this unused build first.
- `A011` [surplus] `core/scripts/pre-commit.sh:1-4` repeats `core/package.json:21` (`git diff --cached --check`), and nothing references the script (no hook, no package script).
- `A012` [surplus] `core/package.json:29-30` keeps its own `oxfmt`/`oxlint` devDependencies, while the root `package.json` also pins `oxlint 1.39.0`. Two pins of the same tool must be bumped together.
- `A013` [hidden] `presets/package.json:7` `build` is `echo 'stub: presets-install build'`, so `turbo build` reports success for a package that builds nothing. This check can never fail.
- `A014` [hidden] `presets/tsconfig.json` and `testing/tsconfig.json` set `"types": ["node"]`, but neither package depends on `@types/node`. Any `tsc` run there would fail. Neither has a `typecheck` script, so nothing notices.
- `A017` [surplus] `presets/src/index.ts:1` exports only `TUI_PRESETS_INSTALL_STUB = true`, and nothing imports it (fan-in 0).
- `A018` [surplus] `presets/src/{apply-confirm,command-preview,evidence-panels,layout-presets,session-shell,status-model}/index.ts:1` are each `// stub`, empty files nothing imports. These are extension points for things that don't exist.
- `A019` [surplus] `testing/src/index.ts:1` exports only `TUI_TESTING_STUB = true`, and nothing imports it.
- `A020` [surplus] `testing/src/{asserts,fixtures,matchers,pty,snapshots}/index.ts:1` are each `// stub`, and nothing imports them. The package is described as a "test harness" (`testing/README.md:3`) in a repo that bans unit tests.
- `A021` [surplus] `testing/package.json:6` has `"scripts": {}`, and root `package.json:10-11` still lists both stub packages as workspaces.
- `A022` [hidden] `tui/README.md:14-15` says `kit/presets-install` holds "install UX compositions shared by the apps" and `kit/testing` holds "test harness and snapshot helpers". Both are empty stubs.
- `A028` [hidden] `core/README.md:46-47` lists `bun run lint` and `bun run format:check` as working scripts. Both fail (missing config).
- `B164` [surplus] `install-docs/.github/workflows/ci.yml:1-18` sits in a subdirectory of the `tui` repository, which has no root `.github/workflows`. GitHub never runs it, and the reusable `bun-ci.yml` it calls would only typecheck anyway.
- `B165` [surplus] `install-docs/tsconfig.json:11-12` (`declaration`, `outDir: dist`) configure emit for a package that only runs `tsc --noEmit` and `bun build`.
- `B166` [surplus] `install-docs/.gitignore:5` ignores a `levitate-docs` binary that nothing produces.
- `B167` [surplus] `install-docs/design/design_goal.png` is referenced by no doc or code.
- `C067` [surplus] `dp/src/index.ts:3-5` re-exports `buildPlanCommand`, `runRecpartPlan`, `cliHelpText`, `parseCliArgs` and `runCli`. No package imports `@levitate/tui-install-disk-plan`, and `package.json` declares no `main`/`exports`.
- `C068` [hidden] `dp/src/index.ts:1,9-11` is both the library barrel and the executable (`import.meta.main`). recab splits the same roles into `rc/index.ts` + `rc/main.ts`, and install-docs uses `src/main.ts`. xtask has to know each app's launch form (`tui.rs:19,41`, `inspect_tui.rs:122,127`).
- `C069` [surplus] `dp/package.json:7` `build` (tsc emit to `dist/`) and `dp/tsconfig.json:10` `outDir`: nothing runs `dist/`. Bun runs the sources.
- `C070` [surplus] `dp/tsconfig.json:15` excludes `src/**/*.test.ts(x)`. No tests exist, and unit tests are prohibited.
- `C071` [surplus] `dp/bun.lock:1-28` is a stale lockfile for a package named `recpart-frontend` depending on `blessed`. disk-plan is a root workspace member, so the file is never used and describes a different app.
- `C072` [surplus] `dp/.gitignore:3` `.turbo/`. disk-plan defines no turbo-specific outputs.
- `C086` [hidden] `package.json:4-12` (root workspaces) omits `tui/apps/runtime/recab`, so recab installs its own tree. Its `file:../../../kit/core` dependency (`rc/package.json:20`) is copied with the kit's own `node_modules/react` (`tui/kit/core/package.json` lists `react` under `dependencies`, not `peerDependencies`). That gives two Reacts and the launch crash. The workspace list, the `file:` link and the kit's dependency kind must agree, and nothing shows it.
- `C087` [hidden] `rc/bun.lock:1-177` pins a private dependency tree for recab alone, while disk-plan and install-docs resolve through the root lockfile. React/ink versions can drift per app.
- `C088` [surplus] `rc/index.ts:1-6` barrel re-exports the runner, CLI, domain and flows. Nothing imports it, and `rc/package.json` has no `main`/`exports`.
- `C089` [surplus] `rc/app/index.ts:1-2` and `rc/presentation/index.ts:1` are barrels nothing imports (`bootstrap.tsx:2` and `rc/index.ts:1-2` import the files directly).
- `C090` [surplus] `rc/domain/index.ts:1-2` is a pass-through barrel that `rc/flows/index.ts:1` and `rc/domain/recab.ts:1` bypass by importing `./contract` directly.
- `C091` [surplus] `rc/package.json:5-7` `"bin": {"levitate-runtime-recab": "./main.ts"}`. Nothing installs or links the package.
- `C092` [surplus] `rc/tsconfig.json:10-11` `resolveJsonModule` (no JSON imports) and `declaration: true` (an app nobody imports), plus `rc/package.json:10` `build`, which emits `dist/` that nothing runs.
- `C112` [hidden] `xtask/src/tasks/check.rs:56-61` `ensure_node_modules` runs a bare `bun install` in any package dir lacking `node_modules`. For recab (outside the workspace) that is exactly the install that produces the duplicate-React tree.
- `C113` [special] `xtask/src/tasks/check.rs:57` treats "node_modules exists" as "dependencies installed", so stale or partial installs, like recab's nested React, are never refreshed.
- `C121` [hidden] `dp/package.json:13-14` and `rc/package.json:16-17`: `format:check`, and therefore `check`, always fails. The failing files are the generated contracts plus `disk-plan-screen.tsx`, so `bun run check` and the root `turbo check` (`package.json:14`, which reaches disk-plan only) cannot pass.
- `C122` [hidden] `dp/package.json:10-13` and `rc/package.json:13-16` call `bunx oxlint`/`bunx oxfmt` without a devDependency, so the tool version is whatever `bunx` resolves (root's `oxlint 1.39.0` for disk-plan, the kit copy's or a download for recab). The kit pins both.
- `C128` [hidden] `tui/README.md:14` "kit/presets-install: install UX compositions shared by the apps". disk-plan, the install app, does not depend on it (`dp/package.json:16-20`).

### F2. One app manifest: each app declares its name, entry and needs once, and xtask and the builder read it.

Solves 15 problems (15 hidden).

Each app's `package.json` is the only record of the app. The directory name is the package-name suffix, the `bin` name and the guest command (`install-docs`, `disk-plan`, `recab`). The entry comes from `bin`. A `levitate` block lists the Rust tools the app needs on PATH (`recpart`, `recab`), where its generated contracts go, and its inspect presets (args and fixtures). xtask discovers apps from the root workspace list. `TuiApp` becomes a validated name, and `TS_PACKAGES` plus the per-app arms in `tui.rs`, `inspect_tui.rs`, `check.rs` and `generate.rs` read the manifest, as does the builder's bundling step (F22). The spellings `@levitate/docs-tui`, `docs-tui`, `levitate-install-docs`, `install-disk-plan` and `levitate-runtime-recab` go. `tui/README.md` points at the workspace for the app list and gives one way to run any app.

**Done when:** Adding an app means a workspace entry and its `package.json`, with no edit in xtask or the builder; `rg 'apps/live-tools/install-docs|install-disk-plan|docs-tui' xtask distro` finds nothing; recab runs and renders through xtask like the others.

**Depends on:** F1.

- `X001` [hidden] The set of TUI packages is listed separately in `package.json:4-12` (root workspaces: install-docs, disk-plan, kit/core, presets-install, testing — no recab), `xtask/src/tasks/check.rs:8-14` (`TS_PACKAGES`: kit/core, disk-plan, install-docs, recab — no presets/testing), `xtask/src/cli/types.rs:337-339` (`TuiApp`: install-docs, install-disk-plan), `xtask/src/tasks/tui.rs:15-33` (paths per app), `xtask/src/tasks/inspect_tui.rs:~120-130` (presets: install-docs, install-disk-plan), `tui/README.md` (lists all three apps, "run an app with `cargo xtask tui <app>`"). Adding or moving an app needs all six; they already disagree (recab is typechecked but not a workspace member, cannot be launched or inspected by xtask, and crashes on launch with a duplicate React).
- `X002` [hidden] App names differ by place: `install-docs` (dir, xtask) vs `@levitate/docs-tui` (package name) vs `levitate-install-docs` (bin, ISO command) vs `docs-tui` (ISO alias) vs `install-disk-plan` (xtask) vs `disk-plan` (dir). Nothing ties them.
- `A015` [hidden] `xtask/src/tasks/tui.rs:14-44` and `inspect_tui.rs:117-128` have no recab target, yet `tui/README.md:9,17` says every app runs with `cargo xtask tui <app>`. The recab launch crash is invisible to every repo command.
- `B133` [hidden] `distro/builder/src/pipeline/live_tools.rs:330` bundles `src/main.ts`, the same entry hard-coded in `xtask/src/tasks/inspect_docs.rs:226`, `xtask/src/tasks/inspect_tui.rs:122`, `xtask/src/tasks/tui.rs:317`, `install-docs/bin/levitate-install-docs-split:45` and `install-docs/package.json:7`, `:10`.
- `B134` [hidden] The app directory path `tui/apps/live-tools/install-docs` is hard-coded in `distro/builder/src/pipeline/live_tools.rs:279`, `:282`, `xtask/src/tasks/inspect_docs.rs:24`, `xtask/src/tasks/tui.rs:314`, `xtask/src/tasks/inspect_tui.rs:121`, `xtask/src/tasks/check.rs:361` and `install-docs/scripts/verify-inspect-snapshots.sh:5`.
- `B137` [hidden] Naming drift for one artifact: package `@levitate/docs-tui` (`install-docs/package.json:2`), directory `install-docs`, bin `levitate-install-docs` (`package.json:7`), bundle dir `docs-tui` (`live_tools.rs:319`), alias `docs-tui` (`live_tools.rs:435`) and `.gitignore` entry `levitate-docs` (`install-docs/.gitignore:5`).
- `B171` [hidden] `tui/README.md:7` says install-docs is "the live ISO's install guide and split-pane install session". The split-pane session lives in `distro/builder/install-split-pane`.
- `B179` [hidden] `install-docs/README.md:1` and `CLAUDE.md:1` use the name `docs-tui`, while the directory, bin and live command are `install-docs`/`levitate-install-docs`.
- `C093` [hidden] `rc/cli/parse-args.ts:44-46` help gives only `bun run main.ts` usages. There is no `cargo xtask tui` route for recab (`xtask/src/cli/types.rs:336-342`), unlike disk-plan (`dp/src/cli/parse-args.ts:65-66`).
- `C097` [hidden] App identity strings are repeated per app: `"levitate-install-disk-plan"` (`dp/src/app/bootstrap.tsx:11`), `"install-disk-plan"` (`disk-plan-screen.tsx:651`, `xtask/src/cli/types.rs:339`, `xtask/src/tasks/inspect_tui.rs:125`), `"@levitate/tui-install-disk-plan"` (`dp/package.json:2`). recab has the same spread: `"levitate-runtime-recab"` (`rc/app/bootstrap.tsx:9`, `rc/package.json:6`) and `"runtime-recab"` (`runtime-recab-screen.tsx:174`).
- `C099` [hidden] App directory paths are hard-coded separately in `xtask/src/tasks/tui.rs:16,32`, `xtask/src/tasks/inspect_tui.rs:121,126`, `xtask/src/tasks/check.rs:8-14`, `xtask/src/tasks/generate.rs:96,102` and root `package.json:7`. Moving or renaming an app needs five edits across the superproject/submodule boundary.
- `C100` [hidden] Launch commands are duplicated between `xtask/src/tasks/tui.rs:19,41` (`bun src/main.ts`, `bun run start --`) and `xtask/src/tasks/inspect_tui.rs:122,127` (`bun src/main.ts`, `bun run start -- --disk /dev/sda`). Both depend on each app's `package.json` `start` script or file layout.
- `C111` [hidden] `xtask/src/cli/types.rs:336-342` `TuiApp` lists install-docs and install-disk-plan only. recab, a typechecked app (`check.rs:13`) with a generated contract (`generate.rs:98-102`), cannot be launched or inspected by preset. That is why the crash went unseen.
- `C127` [hidden] `tui/README.md:17-18` "Run an app with `cargo xtask tui <app>`, render one headlessly with `cargo xtask inspect tui <app>`". Neither works for recab (`xtask/src/cli/types.rs:336-342`), and inspect `--input` drives nothing.
- `D066` [hidden] `live_tools.rs:279,295,330` hard-code the tui app's layout: the directory `tui/apps/live-tools/install-docs`, the entry `src/main.ts` and `node_modules/@levitate/tui-kit/package.json` as the "deps installed" probe. Moving or renaming the tui package or its dependency breaks the builder.

### F3. One xtask TUI driver for run, inspect and capture.

Solves 23 problems (14 hidden · 4 surplus · 5 special).

`xtask/src/tasks/tui/` holds one launch path that `tui run`, `tui inspect` and the checks all use. It reads the manifest (F2), builds the app's required Rust tools with one target-dir helper and puts them first on PATH, runs the root install from F1, and spawns the app. `inspect` drives the app inside `pty_capture`, which writes `--input` keys to the PTY master (the app's stdin) and waits for the screen to settle instead of a fixed timer. There is one screen parser and one run-dir and transcript naming scheme, and errors propagate instead of `let _ =`. `inspect docs` becomes `inspect install-docs --page <slug>` or `--all-pages`, with the page list coming from the app (`install-docs --pages`) rather than from a regex over generated files. A destructive app never sees a real device: disk-plan's preset puts a fixture `recpart` (named in the manifest) first on PATH, which answers list-disks, plan and dry-run from recorded JSON and refuses `apply`. The app's own runner checks for a TTY; xtask doesn't. The capture host sets its environment only as the terminal it emulates (see F20); it shares no copied setup with the split pane.

**Done when:** `cargo xtask tui inspect disk-plan --input n` shows the second page; run and inspect behave the same for every app on a fresh checkout; `inspect_docs.rs` is gone; xtask has no `/dev/sda` and no write to `/dev/tty`.

**Depends on:** F2.

- `X007` [hidden] Superproject history: the `tui` pointer moves with `xtask/src/tasks/{tui,docs}/inspect.rs` in 4 commits and with xtask tooling files 3× each (`.artifacts/untangle-ui/map/cochange-super-tui.txt`). The TUI's render/inspect harness lives in xtask (another repo), its PTY test lives in the app and shells out to xtask by subcommand name — the rename `docs inspect` → `inspect docs` broke it silently.
- `X008` [hidden] `xtask/src/util/pty_capture.rs` (PTY spawn, kill process group, env) and `install-split-pane/src/main.rs` (PTY spawn, env, vt100 screen) are two PTY hosts for the same apps in two repos with copied setup.
- `B093` [hidden] `src/domain/navigation/nav-model.ts:3-21` (`docsSlugFromHref`, `/docs/<slug>`) and `xtask/src/tasks/inspect_docs.rs:157` (regex `"/docs/([a-z0-9][a-z0-9\-]*)"`) each derive slugs from hrefs. `inspect_docs.rs:174-175` validates a third slug grammar the app itself never enforces.
- `B138` [hidden] `xtask/src/tasks/inspect_docs.rs:187-298` copies `create_run_dir`, `transcript_path`, `parse_plain_screen`, `parse_formatted_screen`, `parse_screen`, `trim_line_trailing_spaces`, `strip_script_bookends`, `find_subslice` and `unix_timestamp` from `xtask/src/tasks/inspect_tui.rs:200-322`.
- `B139` [hidden] `xtask/src/tasks/inspect_docs.rs:38-44` and `xtask/src/tasks/inspect_tui.rs:39-45` each clamp `seconds == 0 → 1` and `columns/rows.max(1)`, which `xtask/src/util/pty_capture.rs:24-25` and `:56` clamp a third time.
- `B140` [surplus] `xtask/src/tasks/inspect_docs.rs` (`inspect docs`) and `xtask/src/tasks/inspect_tui.rs:119-123` (`inspect tui install-docs`) are two commands that render the same app headlessly.
- `B141` [hidden] `xtask/src/tasks/inspect_docs.rs:150-170` scrapes `docs/content/src/generated/index.ts` with a regex on `"href": "/docs/…"`. It depends on the generated file's JSON-ish formatting and on every `/docs/` href in that file being a nav entry, while the app gets nav from the `docsNav` export.
- `B142` [surplus] `xtask/src/tasks/inspect_docs.rs:133-143` loops over a one-element tool list `["bun"]`.
- `B143` [hidden] `xtask/src/tasks/tui.rs:315` runs `ensure_node_modules` before launching, but `xtask/src/tasks/inspect_docs.rs:20-36` and `inspect_tui.rs:29-37` do not. Inspect fails on a fresh checkout where `tui` works.
- `B144` [special] `xtask/src/tasks/inspect_docs.rs:91` and `inspect_tui.rs:85` discard the `remove_file` error. `inspect_docs.rs:296` and `inspect_tui.rs:320` map a clock error to `0`.
- `B145` [hidden] `xtask/src/tasks/inspect_docs.rs:63-64` passes the unclamped `options.columns`/`options.rows` into `transcript_path`, while capture uses the clamped values (`inspect_docs.rs:43-44`). `inspect_tui.rs:51-52` does the same.
- `C098` [hidden] The TTY requirement is checked twice for disk-plan: by xtask (`xtask/src/tasks/tui.rs:9-12` `require_terminal`) and by the app (`dp/src/cli/run.ts:15-18`), with different messages and exit paths.
- `C101` [hidden] `xtask/src/tasks/inspect_tui.rs:124-128` runs disk-plan without what `tui.rs:23-42` does first: building recpart, putting `target/debug` on PATH and `ensure_node_modules`. The inspect snapshot runs whatever `recpart` is on the host PATH (none here, so listing fails), and the two commands drift.
- `C102` [special] `xtask/src/tasks/inspect_tui.rs:127` hard-codes `--disk /dev/sda` into the preset. That is where the "default disk /dev/sda" in the baseline comes from: a real device path on many dev hosts, injected into a destructive tool's UI.
- `C103` [hidden] `xtask/src/tasks/inspect_tui.rs:240-243` delivers `--input` keys with `printf '%b' … > /dev/tty`. Inside the PTY that writes to the terminal's output, not the app's stdin, so the keys are drawn on screen (`nn` in the snapshot) and never reach the app. `--input` cannot drive any app, and the documented `cargo xtask inspect tui install-disk-plan --input 'n'` (`xtask/README.md:34`) does nothing. The prior untangle noticed "keys do not navigate" (`docs/history/UNTANGLE.md:1031`) but not why.
- `C104` [special] `xtask/src/tasks/inspect_tui.rs:242` sends the same key sequence twice (`sleep N; printf …; sleep 1; printf …`), a sleep-and-retry for timing.
- `C105` [special] `xtask/src/tasks/inspect_tui.rs:39-43` maps `--seconds 0` to 1, and `:85` swallows the transcript removal error (`let _ =`).
- `C106` [surplus] `xtask/src/tasks/inspect_tui.rs:287-315` `strip_script_bookends`/`find_subslice` strip `script(1)` "Script started/done" banners, but capture now uses `portable_pty` (`xtask/src/util/pty_capture.rs:20-40`), which emits none.
- `C107` [hidden] `xtask/src/tasks/inspect_tui.rs:200-322` (`create_run_dir`, `transcript_path`, `parse_plain_screen`, `parse_formatted_screen`, `parse_screen`, `trim_line_trailing_spaces`, `strip_script_bookends`, `find_subslice`, `unix_timestamp`) are copies of the same functions in `xtask/src/tasks/inspect_docs.rs:187-293`.
- `C108` [surplus] `xtask/src/tasks/inspect_tui.rs:73,77,218` prefix the single snapshot with `001-`, a numbering scheme with only one entry.
- `C109` [special] `xtask/src/tasks/tui.rs:23-44`: only the disk-plan arm builds a Rust tool and edits PATH. The "app needs a binary from this checkout" rule is an `if` for one app, and recab (needing `recab`) has no arm.
- `C110` [hidden] `xtask/src/tasks/tui.rs:34-36` and `xtask/src/tasks/generate.rs:133-135` (also `xtask/src/util/distro_builder.rs:24`) each re-derive `CARGO_TARGET_DIR`-or-`target` + `debug`.
- `C135` [hidden] `docs/history/UNTANGLE.md:1031` "TUI page navigation could not be driven by `xtask tui inspect` (keys do not navigate, also before this change)" was left as a curiosity. The cause is `xtask/src/tasks/inspect_tui.rs:242` writing keys to the PTY output, which is still there.

### F4. Checks that fail on real bugs: `cargo xtask check tui`.

Solves 22 problems (14 hidden · 1 surplus · 7 special).

For every manifest app, `cargo xtask check tui` runs typecheck, lint and format-check (F1 config), then a render check through the F3 driver at full console size and at the size the split pane really gives the right pane (taken from the split-pane layout for 80- and 100-column consoles). The render check fails on a crash, a non-zero exit, a React error or the "Terminal too small" screen. install-docs keeps a small snapshot set listed in one manifest that xtask reads: no shell script, no `SECONDS`, and the diff is printed on mismatch. Snapshots leave out the docs-content page count, so editing docs doesn't fail the viewer. disk-plan is walked page by page with `--input` against the fixture recpart, asserting each page title, including both apply outcomes. recab renders against a fixture recab. The bun PTY test is deleted: full-screen snapshots at the real widths cover its frame checks, and it shells out to xtask by subcommand name. install-docs' `check` script and README point at `cargo xtask check tui`. The narrow-width render is expected to fail until F9 lands.

**Done when:** The check passes on a clean tree after F9. It fails when recab has two Reacts, when a disk-plan transition lands on the wrong page, or when install-docs doesn't fit the right pane. It still passes after an unrelated docs-content nav change.

**Depends on:** F1, F3.

- `B146` [hidden] `xtask/src/tasks/check.rs:383-397` "check" runs only typecheck for install-docs, while `install-docs/package.json:18` defines a different `check` (typecheck, lint, format, inspect). Root `package.json` `turbo check` runs the latter.
- `B147` [special] `src/install-viewer.chrome.pty.test.ts:51-52` invokes `levitate-xtask docs inspect`, which was renamed to `inspect docs` (`xtask/src/cli/types.rs:346`), so `test:e2e` fails 3/3 (`baseline/install-docs-test_e2e.log`).
- `B148` [hidden] `install-docs/snapshots/inspect/120x36/001-installation-disk.txt:36` embeds the total docs page count (`page 3/37`, now `3/35`) from the status bar. Any docs-content nav change unrelated to the viewer breaks `inspect:check`, which is the current failure (`baseline/install-docs-inspect_check.log`).
- `B149` [hidden] `install-docs/scripts/verify-inspect-snapshots.sh:8-18` hand-copies columns, rows, seconds, slugs and the expected `NNN-<slug>.txt` names. `snapshots/inspect/120x36/manifest.txt:1-5` holds the same values and nothing reads it. The file numbering depends on `xtask/src/tasks/inspect_docs.rs:79`.
- `B150` [special] `install-docs/scripts/verify-inspect-snapshots.sh:10` assigns bash's special `SECONDS` variable (the shell's elapsed-time counter), so the value passed at `:30` is `2 +` elapsed seconds.
- `B151` [special] `install-docs/scripts/verify-inspect-snapshots.sh:9-10` and `:27-36` (with `xtask/src/tasks/inspect_docs.rs:218-237`) rely on a fixed 2-second capture window: the snapshot is whatever Ink drew before a timed kill.
- `B152` [special] `install-docs/scripts/verify-inspect-snapshots.sh:36` discards xtask stdout. `:54` runs `diff -u … >/dev/null`, so a mismatch reports only a filename, never the diff.
- `B153` [hidden] `install-docs/scripts/verify-inspect-snapshots.sh:4` resolves the repo root by `../../../../..` from its own location, coupling it to the directory depth.
- `B154` [hidden] The inspect snapshots are plain text of the first screen at 120x36 with navigation focus. They cannot catch colour or theme regressions (always-black command rows, uncoloured list markers, hex note colours under NO_COLOR), content-focus behaviour, link activation, scrolling and measure drift below the fold, or the real split-pane width where `minColumns: 88` applies. The status line is also truncated at 120 columns (`001-installation-disk.txt:36`).
- `B155` [hidden] `src/install-viewer.chrome.pty.test.ts:35` derives the repo root from `process.cwd()`, so the test only works when run from the app directory. `:99` hard-codes `target/debug/levitate-xtask`, ignoring `CARGO_TARGET_DIR` (which `xtask/src/tasks/tui.rs:332-334` honours).
- `B156` [special] `src/install-viewer.chrome.pty.test.ts:36` runs `cargo build -p levitate-xtask` at module import time.
- `B157` [hidden] `src/install-viewer.chrome.pty.test.ts:92` depends on xtask's transcript naming `{index:03}-{slug}.transcript` (`xtask/src/tasks/inspect_docs.rs:206`). `:204` splits frames on `ESC[H`, which depends on Ink's redraw escape sequence.
- `B158` [hidden] `src/install-viewer.chrome.pty.test.ts:144` matches code fences with `/┌\s+[A-Z][A-Z0-9_. -]*\s+/`, coupled to `ink/blocks/code-block.tsx:107-114` (single glyph set, upper-cased language label). `:131`, `:175`, `:179` and `:359-361` hard-code single-weight glyphs although `:29-33` accepts every weight. Changing `theme.chrome.borderGlyphSet` (`ink/theme/index.ts:81`) fails the test without any bug.
- `B159` [hidden] `src/install-viewer.chrome.pty.test.ts:396-401` depends on the literal pane title "Installation Docs" (`ink/screens/install-viewer.tsx:501`). `:415` depends on docs-content page `helpers-commands` having a complete code fence in its first screen.
- `B160` [special] `src/install-viewer.chrome.pty.test.ts:28` and `:295`: when the structural title row is not found, the fallback title search uses `/[A-Za-z]/`, which matches almost any line, so the title check cannot fail.
- `B161` [special] `src/install-viewer.chrome.pty.test.ts:217` inspects only the first two frames.
- `B162` [surplus] `src/install-viewer.chrome.pty.test.ts:380-381` and `:397-398` capture the same `installation` run twice per width. `:38-44` (`runPtyCapture`, `runPtyCaptureSlug`) are wrappers over `runDocsInspectCapture`.
- `B163` [hidden] The PTY test only checks frame glyph shapes at 96 and 120 columns. It would not catch wrong content, wrong scroll math, broken focus/actions, missing colour, or the narrow split-pane case.
- `B169` [hidden] `install-docs/README.md:29-35` advertises `test:e2e`, `inspect:check` and `check` as the development loop, and all three are broken (see above).
- `C114` [hidden] `xtask/src/tasks/check.rs:43-48`: "typecheck recab" passes while the app crashes on its first render. The only TS check xtask runs cannot catch a broken dependency graph, and nothing renders recab.
- `C123` [hidden] `dp/README.md:28-30` and `xtask/README.md:34` are the documented ways to exercise disk-plan. The inspect example cannot send keys (see inspect entry), and the interactive one requires a human. No scripted check ever walks past the welcome page, so the flow branches above are unverified by anything.
- `C131` [hidden] `docs/history/UNTANGLE.md:1031` (F21) records the recab TUI moving to the real `@levitate/tui-kit`, verified by "typecheck and lint" and a runner test only. The TUI was never rendered after the switch and now crashes at launch (duplicate React; see packaging). The claimed "one recab TUI" is one TUI that does not run.

### F5. Delete the kit surface no app uses.

Solves 49 problems (14 hidden · 3 needless · 23 surplus · 9 special).

Delete `patterns/`, the unused components, hooks, helpers, aliases, palette keys and intents listed below, `ThemeProvider`, `disposeTuiApp` and the context `title`. One `src/index.ts` exporting what the apps import replaces the folder barrels, and there's no `./theme` subpath. Theme code lives only in `theme/`: no `theme.ts` that is both barrel and owner, and no `typography.ts` re-export. `core/README.md` and `core/AGENTS.md` describe the folders that remain, with a Quick Start that runs. Anything a later fix still needs (`useAsyncTask`, list and grid navigation) stays.

**Done when:** The import graph shows no kit file with fan-in 0 except `index.ts`; every export in `index.ts` is used by an app; `core/README.md` matches the tree.

**Depends on:** F4.

- `A024` [hidden] `core/README.md:9-14` lists the folders as app, runtime, primitives, components, patterns and theme. It omits `surfaces/`, `chrome/`, `hooks/` and `utils/`, which are what the apps actually use (`SurfaceFrame`, chrome glyphs, hooks).
- `A025` [hidden] `core/README.md:14` lists "typography" under theme. `core/src/theme/typography.ts:1` is only a re-export of string helpers from `utils/strings`.
- `A026` [hidden] `core/README.md:25-37`: the Quick Start renders `TwoPane` with `footer="q quit"` through `renderApp`, which defaults `exitOnCtrlC: false` (`core/src/app/render-app.tsx:44`). Nothing in `TwoPane` binds `q`, so the documented example can't be exited. It also showcases `TwoPane`, which no app uses.
- `A027` [hidden] `core/README.md:21` and `core/AGENTS.md:7` advertise `@levitate/tui-kit/theme` as a public entry. No consumer imports it.
- `A030` [hidden] `core/src/patterns/wizard-flow.tsx:26` says the aliases are "Backward-compatible helper names for tests and callsites during the cutover". The tests were removed (commit `1e2ca92`) and no call site uses `pageClampIndex` or `formatPageCard`.
- `A031` [hidden] `core/src/hooks/use-hotkeys.ts:22` says "Alias retained as a stable generic input hook name". Nothing uses `useTuiInput`.
- `A035` [surplus] `core/src/index.ts:1-10` is a pure re-export barrel over ten folder barrels, and every app import goes through it.
- `A036` [surplus] Folder barrels that only re-export: `core/src/app/index.ts:1-4`, `runtime/index.ts:1-2`, `runtime/ink/index.ts:1-3`, `runtime/terminal/index.ts:1-2`, `hooks/index.ts:1-7`, `primitives/index.ts:1-2`, `primitives/layout/index.ts:1-4`, `primitives/display/index.ts:1-7`, `components/index.ts:1-4`, `components/{data,feedback,forms}/index.ts:1-3`, `components/navigation/index.ts:1-6`, `patterns/index.ts:1-4`, `surfaces/index.ts:1`, `surfaces/frame/index.ts:1-4`, `chrome/index.ts:1-5`, `utils/index.ts:1-3`.
- `A037` [surplus] `core/src/theme.ts:29-34` already does `export *` from `./theme/tokens` and `./theme/colors`, and `:182-201` exports the same names again (`defaultThemeChrome`, `detectColorRuntime`, `resolveIntentColor`, `ColorIntent`, …). That's two export lists to keep in step.
- `A038` [needless] `core/src/theme.ts` is both a barrel and the owner of `createTheme`, `normalizeTheme*` and `footerHint` (`:36-180`), next to a `theme/` folder that holds the rest of the theme. Theme logic is split between `theme.ts` and `theme/*.ts` for no reason the layout shows.
- `A039` [surplus] `core/src/theme/typography.ts:1` re-exports four `utils/strings` functions under the theme namespace. Those names reach the root barrel twice (`index.ts:9` through theme, `:10` through utils).
- `A040` [surplus] `core/src/hooks/use-hotkeys.ts:18-20` re-exports `keyNamesFromInput`, `normalizeKeySpec` and five types that `runtime/ink/index.ts:1` already exports through the root barrel.
- `A041` [surplus] `core/package.json:10` `"./theme": "./src/theme.ts"` is a second entry point for the same symbols the root barrel exports, and nobody imports it.
- `A042` [surplus] Exports kept only for the unit tests removed in `1e2ca92`, and now unused anywhere outside their own file: `createFocusManager` (`core/src/runtime/ink/focus-manager.ts:11`), `toggleFocusPlane` (`hooks/use-focus-plane.ts:19`, used only internally), `transitionGridIndex` (`hooks/use-grid-navigation.ts:33`), `resolveDividerGlyph` (`primitives/display/divider.tsx:11`), `keyNamesFromInput`/`normalizeKeySpec` exports (`runtime/ink/use-input.ts:60,68`), `groupGridRows` (`primitives/layout/grid.tsx:67`), `progressRatio`/`renderProgress`/`renderProgressBar` (`components/feedback/progress.ts:15,26,31`), `normalizeTerminalSize`/`readScreenSize`/`DEFAULT_TERMINAL_SIZE`/`terminalMeetsMinimum` exports (`runtime/terminal/screen-size.ts:8,17,27,52`), `renderSectionList` (`components/navigation/section-list.tsx:25`), `mapSelectableGridItems` (`components/navigation/selectable-grid.tsx:23`), `flattenStatusSegments`/`formatPagedStatusLine` (`components/feedback/status-line.tsx:38,167`), `footerHint` (`theme.ts:173`), `buildTreeNavRows` (`components/navigation/tree-nav.tsx:74`), `resolveTwoPaneGeometry` (`patterns/two-pane.tsx:70`), `formatWizardCard`/`wizardClampIndex` (`patterns/wizard-flow.tsx:15,19`), `createChromeGrid` (`chrome/grid.ts:276`).
- `A043` [surplus] Components nothing renders: `Stack` (`primitives/layout/stack.tsx:12`), `Inline` (`primitives/layout/inline.tsx:12`), `Grid`/`GridItem` outside `SelectableGrid` (`primitives/layout/grid.tsx:82,90`), `ScrollView` outside `ReviewScreen` (`primitives/layout/scroll-view.tsx:40`), `Heading` (`primitives/display/heading.tsx:8`), `Divider` (`primitives/display/divider.tsx:15`), `Panel` (`primitives/display/panel.tsx:15`), `UiLink` (`primitives/display/link.tsx:51`), `SyntaxTokenLine` (`primitives/display/syntax-token-line.tsx:102`), `EmptyState` (`components/data/empty-state.tsx:8`), `Table` (`components/data/table.tsx:45`), `Spinner` (`components/feedback/spinner.tsx:11`), `ProgressBar` (`components/feedback/progress.ts:39`), `StatusLine` (`components/feedback/status-line.tsx:12`), `Confirm` (`components/forms/confirm.tsx:12`), `Select` (`components/forms/select.tsx:14`), `TextField` outside `TextEntryScreen` (`components/forms/text-field.tsx:23`), `List` (`components/navigation/list.tsx:5`), `Pager` (`components/navigation/pager.tsx:33`), `Tabs` (`components/navigation/tabs.tsx:18`), `ErrorBoundary` as an export (`app/error-boundary.tsx:14`), `ThemeProvider` (`theme/theme-provider.tsx:10`), `AppProvider` as an export (`app/app-provider.tsx:18`).
- `A044` [surplus] The whole `patterns/` folder is unused by every app: `PageFrame` (`patterns/page-frame.tsx:12`), `TwoPane` and `ScreenFrame` (`patterns/two-pane.tsx:88,136`), `ReviewScreen` (`patterns/review-screen.tsx:17`), `WizardFlow`, `AsyncWizardFlow`, `TextEntryScreen` and `ReviewScreenFlow` (`patterns/wizard-flow.tsx:41,123,234,272`). That's about 500 lines that change only when the kit changes.
- `A045` [surplus] Unused hooks and runtime helpers: `useMeasure` (`hooks/use-measure.ts:5`), `useAsyncTask` outside `AsyncWizardFlow` (`hooks/use-async-task.ts:13`), `useTuiInput` (`hooks/use-hotkeys.ts:23`), `useKeyInput` as an export (`runtime/ink/use-input.ts:163`), `useTuiApp` outside the kit (`app/app-provider.tsx:22`), `useThemeTokens` (`theme/theme-provider.tsx:14`), `createExitController` (`runtime/ink/exit-controller.ts:8`), `getTerminalSize` (`runtime/terminal/screen-size.ts:42`), `detectTtyColors` and `ttySupportsColor` (`runtime/terminal/tty-capabilities.ts:10,14`).
- `A046` [surplus] Unused theme and utility exports: `spacing`/`resolveSpacing`/`SpacingToken` (`theme/spacing.ts:1-17`; the kit hard-codes `paddingX={1}` instead), `PaletteKey` (`theme/palette.ts:27`), palette keys `gray100`, `gray300`, `gray700`, `slate700`, `blue800`, `blue700`, `amber900` (`theme/palette.ts:4,5,7,10,16,17,18`, never read), `resolveIntentMono` (`theme/colors.ts:349`), `resolveColorReference` (`theme/colors.ts:353`), `resolveRgbLiteral` (`theme/colors.ts:431`), `normalizeThemeColorMode` (`theme/colors.ts:417`), `chunkArray` (`utils/arrays.ts:11`), `toPositiveIntOrFallback`/`toInt`/`toFiniteNumber` exports (`utils/clamp.ts:1,9,17`), `renderChromeRowStrings` (`chrome/renderer.ts:50`), `applyChromeInstruction` export (`chrome/renderer.ts:4`), `ChromeRule` and `ChromeSplit` (`chrome/components.tsx:115,117`), `ChromeGrid` class export (`chrome/grid.ts:84`).
- `A047` [surplus] `core/src/app/create-app.tsx:38` `disposeTuiApp(_app) {}` is an empty function nothing calls.
- `A048` [surplus] `core/src/app/app-provider.tsx:7` `TuiAppContextValue.title` is set by `create-app.tsx:32`, and the three apps pass titles (`disk-plan/app/bootstrap.tsx:11`, `recab/app/bootstrap.tsx:9`, `install-docs/app/bootstrap.tsx:30`), but nothing reads `title` from context.
- `A050` [surplus] `core/src/theme/tokens.ts:60,64` `ThemeChrome.titleStyle` and `sidebarHeaderMode` are normalised (`theme.ts:92-105`) and set by install-docs (`install-docs/presentation/ink/theme/index.ts:82,86`), but no renderer reads them.
- `A051` [surplus] `core/src/theme/tokens.ts:21,31` intents `headerAccentBackground` and `warningBackground` are defined and overridden by install-docs, but no kit or app component renders with them.
- `A052` [surplus] `core/src/surfaces/frame/types.ts:4,20` `SurfacePaneTitleStyle`/`titleStyle` prop is accepted but never read in `surfaces/frame/pane.tsx`.
- `A053` [surplus] `core/src/primitives/display/link.tsx:12,59` `UiLink` accepts `onActivate` and discards it with `void onActivate`.
- `A078` [surplus] `core/src/theme/theme-provider.tsx:4-16` is a second theme context. Every kit component reads the theme from `AppProvider` (`app/app-provider.tsx:30-32`), so a theme passed to `ThemeProvider` reaches nothing. There are two ways to provide a theme, and only one is wired.
- `A099` [hidden] `core/src/patterns/review-screen.tsx:23-26` derives body height as `viewport.rows - headerHeight - footerHeight - 4`. The `4` is a hand copy of the frame/title row budget that `resolveSurfaceFrameGeometry` computes.
- `A104` [surplus] `core/src/patterns/page-frame.tsx:12-22` `PageFrame` copies props into `TwoPane` and adds only the default `sidebar ?? "tui-kit"`.
- `A105` [surplus] `core/src/patterns/two-pane.tsx:134-138` `ScreenFrame`/`ScreenFrameProps` are aliases that pass every prop to `TwoPane`.
- `A106` [needless] `core/src/patterns/two-pane.tsx:26-86` `TwoPaneGeometry`/`resolveTwoPaneGeometry` rename every `SurfaceFrameGeometry` field (`left*`→`sidebar*`, `right*`→`content*`). Each change to surface geometry needs a matching rename map.
- `A115` [hidden] `core/src/primitives/display/panel.tsx:29,37,41` hard-codes `borderStyle="single"` and `paddingX={1}`, ignoring `theme.chrome.borderGlyphSet`/`panePaddingX`, which `SurfacePane` honours. Two pane components follow different theme rules.
- `A116` [hidden] `core/src/primitives/display/divider.tsx:19-20` defaults to the full `viewport.columns` width and the `footerSeparator` intent, so a divider inside a pane overflows, and the general divider is tied to footer vocabulary.
- `A134` [hidden] `core/src/patterns/review-screen.tsx:47` says `"Arrows/PgUp/PgDn …"`, while `core/src/patterns/wizard-flow.tsx:287` says `"Arrows/PageUp/PageDown …"` for the same `ReviewScreen` bindings.
- `A135` [hidden] The cancel binding `["escape","q"]` is copied in `components/forms/confirm.tsx:31`, `components/forms/select.tsx:39`, `components/forms/text-field.tsx:45` and `patterns/wizard-flow.tsx:73,175`. The up/down binding `["up","k"]`/`["down","j"]` is copied in `select.tsx:23-29`, `review-screen.tsx:33-34` and `wizard-flow.tsx:57-63,145-157`.
- `A137` [special] `core/src/components/forms/confirm.tsx:19,23` binds `["y","Y"]`/`["n","N"]`, but `Y`/`N` are redundant because of the lowercase expansion.
- `A147` [hidden] `core/src/components/navigation/pager.tsx:10-25` `computePagerWindow` and `core/src/primitives/layout/scroll-view.tsx:19-38` `computeScrollWindow` are two line-window computations. install-docs has its own third one (`rendering/pipeline/viewport.ts:73`).
- `A148` [surplus] `core/src/components/navigation/list.tsx:3-7` `List`/`ListProps` pass everything through to `Select`.
- `A149` [surplus] `core/src/patterns/wizard-flow.tsx:15-17` `wizardClampIndex` only calls `clampIndex`, and `:27-28` alias it and `formatWizardCard` again.
- `A151` [special] `core/src/components/forms/select.tsx:21`: `selected` is never re-clamped when `items` shrinks (unlike `wizard-flow.tsx:53-55`), so the marker can point past the list until a key is pressed.
- `A152` [special] `core/src/patterns/wizard-flow.tsx:137`: `wizardClampIndex(defaultIndex, 1)` always yields `0`, so `AsyncWizardFlow`'s `defaultIndex` prop is ignored.
- `A155` [special] `core/src/primitives/layout/scroll-view.tsx:25` and `core/src/components/navigation/pager.tsx:15` wrap `toPositiveInt` in a redundant `Math.max(1, …)`.
- `A156` [special] `core/src/components/data/table.tsx:13,26` hard-codes `maxWidth = 120` and a 40-column cap per column.
- `A164` [special] `core/src/primitives/display/heading.tsx:5,9`: `level` accepts `1 | 2 | 3`, but 2 and 3 render identically.
- `A165` [special] `core/src/components/feedback/progress.ts:43` is the one component built with `createElement` in a `.ts` file instead of TSX.
- `A168` [surplus] `core/src/hooks/use-measure.ts:5-7` `useMeasure` only returns `useTuiViewport()`. Its name suggests element measurement, but it returns the terminal size.
- `A178` [hidden] `core/src/patterns/wizard-flow.tsx:41-101` `WizardFlow` and `:123-220` `AsyncWizardFlow` duplicate the selection state, the up/down/enter/cancel bindings and the card rendering (`:77-90` vs `:181-209`), and both duplicate `Select` (`components/forms/select.tsx`).
- `A179` [hidden] `core/src/patterns/wizard-flow.tsx:159` makes the refresh key configurable (`refreshKeys`), but `:193,200` hard-code "Press r to retry/refresh", so the text drifts from the binding.
- `A182` [special] `core/src/patterns/wizard-flow.tsx:146,153,164`: the same `task.loading || task.error || items.length === 0` guard is repeated in three handlers instead of using `isActive`.
- `A183` [needless] `core/src/patterns/wizard-flow.tsx` holds four unrelated screens (`WizardFlow`, `AsyncWizardFlow`, `TextEntryScreen`, `ReviewScreenFlow`) in one file named after one of them.
- `A186` [special] `core/src/components/feedback/spinner.tsx:4` uses ASCII frames `- \ | /`, separate from the kit's glyph sets.

### F6. One colour pipeline from theme intent to Ink.

Solves 39 problems (27 hidden · 3 needless · 4 surplus · 5 special).

A theme maps each intent to one value: a palette name, a hex, or `inherit`. There are no per-mode triples. One kit resolver turns that value into an Ink colour string for the current colour level: it derives ansi256 and ansi16 from the hex with one table, emits forms Ink's `colorize` accepts (`ansi256(n)`, names, `#hex`), and resolves `inherit` to no colour. The level comes from one detector, chalk's `supports-color`, which honours `NO_COLOR` and `FORCE_COLOR` as the user set them. The kit-side `detectColorRuntime`, the `enabled` flag and the `mono` hints go. Every coloured element goes through the resolver: syntax spans, literal colours, the error boundary, note admonitions (their colours become docs-theme intents), list markers and role labels, and background helpers, so `withBackgroundIntent` applies the background. Inline code gets its own intent. App themes are typed partial overrides of the kit theme, so normalising them needs no hand-kept list. `style-map.md` describes this pipeline.

**Done when:** install-docs shows colour under `TERM=xterm-256color` with no `COLORTERM`/`FORCE_COLOR` (an F4 render that keeps colour); `NO_COLOR=1` gives no colour anywhere, notes and code included; hex literals in `tui/apps` appear only in the docs theme file.

**Depends on:** F5.

- `A049` [surplus] `core/src/theme/tokens.ts:41` `ColorValue.mono` (`normal|bold|dim`) is given for every intent and normalised (`theme/colors.ts:404-407`), but no renderer applies it. In mono mode the hints are ignored.
- `A065` [hidden] `core/src/theme.ts:123-157` `normalizeThemeColors` lists all 30 intents by hand, a copy of the `ColorIntent` union at `core/src/theme/tokens.ts:5-35` and the keys at `:73-254`. Adding an intent means editing three places, and a miss silently drops the override.
- `A066` [hidden] `core/src/theme.ts:84-105` `normalizeThemeChrome` repeats the literal unions of `ThemeChrome` (`theme/tokens.ts:58-65`) as runtime `===` chains.
- `A067` [hidden] `core/src/theme/tokens.ts:73-254`: each intent hand-writes `truecolor`, `ansi256` and `ansi16` triples that must describe the same colour (for example `text` `#e5e7eb`/`15`/`white`, `sectionHeading` `#e2e8f0`/`255`). `colorOverrideToDefaults` (`theme/colors.ts:369-384`) can derive them, but the defaults don't use it, so the three columns drift independently.
- `A068` [hidden] `core/src/theme/tokens.ts:93,99,237`: `accent`, `linkText` and `sectionSubheading` are the same literal `#7dd3fc`/`117`/`cyan` three times.
- `A069` [hidden] `core/src/theme/tokens.ts:105,189`: `linkActiveText` and `sidebarItemActiveText` repeat `#0f172a`/`17`/`black`, which is also `palette.slate900` (`theme/palette.ts:8`) written as a literal.
- `A070` [hidden] `core/src/theme/tokens.ts:111,195`: `linkActiveBackground` and `sidebarItemActiveBackground` repeat `#cbd5e1`/`251`/`white`.
- `A071` [hidden] `core/src/theme/tokens.ts:123`: `warning` uses the literal `#facc15`, which equals `palette.yellow` (`theme/palette.ts:22`). `:74-79`/`:200-205` (`border`/`sidebarBorder`) and `:212-217`/`:248-253` (`commandBarBackground`/`cardBackground`) are identical pairs too.
- `A072` [hidden] `core/src/theme/tokens.ts`: about half the intents use `palette.*` and half use inline hex not in the palette (`#e5e7eb`, `#94a3b8`, `#7dd3fc`, `#93c5fd`, `#f87171`, `#86efac`, `#dbeafe`, `#bfdbfe`, `#fef08a`, `#e2e8f0`, `#64748b`). A palette change reaches only part of the theme.
- `A073` [hidden] `core/src/theme/colors.ts:23-34` (`ANSI16_RGB`, VS Code-style RGB), `:36-47` (`NAMED_COLORS`, Tailwind hex from the palette, plus magenta `#d946ef` inline at `:46`) and `:85-102` (xterm base-16 RGB) are three different tables for the same 16 colour names. `red` is 205,49,49 in one, `#ef4444` in another and 128,0,0 or 255,0,0 in the third, and which one applies depends on the code path.
- `A074` [hidden] `install-docs/presentation/ink/theme/index.ts:6,17-21,66,68,72` sets backgrounds to `"default"`. The kit has no "inherit" colour: in truecolor mode `resolveLiteralColor` passes the unknown string through (`theme/colors.ts:291`) and Ink ignores it, but in ansi256/ansi16 mode it falls back to the kit default (`232`/`black`). `install-docs/presentation/ink/blocks/shared/intent-color.ts:3-14` keeps a hand-copied set of those same ten intents to force `undefined`. Theme file, intent-color file and kit colour code must change together, and none of them shows it.
- `A080` [hidden] `core/src/theme/colors.ts:294-305,334-335`: in `ansi256` mode the kit returns bare digits (`"117"`). Ink's `colorize` (`ink/build/colorize.js`) accepts only chalk names, `#hex`, `ansi256(n)` or `rgb(…)`, and returns unknown strings uncoloured. So on a 256-colour terminal without `COLORTERM`, every theme colour vanishes. The kit's colour string format has to match Ink's parser, and nothing ties them together.
- `A081` [hidden] `core/src/theme/colors.ts:283-290`: in truecolor mode, named colours come back as names (`"gray"`) and numeric strings as bare indices, so a truecolor runtime can emit ANSI16 names or invalid digit strings. The mode contract depends on which input form a theme author used.
- `A082` [hidden] `core/src/theme/colors.ts:218-266` `detectColorRuntime` re-implements `NO_COLOR`/`FORCE_COLOR`/`COLORTERM`/`TERM` detection, while chalk (used by Ink for every coloured `Text`) runs its own `supports-color` detection on the same variables and downsamples hex itself. Two detectors read the same globals and can disagree.
- `A083` [hidden] `core/src/theme/colors.ts:230-231` defaults to the globals `process.env` and `process.stdout.isTTY`. `core/src/app/create-app.tsx:26-29` calls it with no env, so app colour mode comes from globals at `createTuiApp` time, and nothing in the signatures shows it.
- `A084` [special] `core/src/theme/colors.ts:233-238`: `enabled: true` forces `ansi16` (or mono when not a TTY), ignoring `COLORTERM`/`TERM`. Asking for colour gives less colour than asking for nothing.
- `A085` [special] `core/src/theme/colors.ts:192-212` `parseForcedColor` is typed `| null` but never returns null, so `if (forced)` at `:246` always passes. Empty and non-numeric `FORCE_COLOR` each get their own `ansi16` branch (`:194-201`).
- `A086` [hidden] `core/src/theme/colors.ts:49-51` defines `TRUECOLOR_RUNTIME`/`ANSI256_RUNTIME`/`ANSI16_RUNTIME`, yet `:195,201,206,209,211,257,262,265` rebuild the same objects as literals.
- `A087` [hidden] `core/src/theme/colors.ts:214-216` (`runtimeModeEnabled`), `:269` and `:326` (`!runtime.enabled || runtime.mode === "mono"`), and `core/src/runtime/terminal/tty-capabilities.ts:14-16` (`ttySupportsColor`) are four copies of the "is colour on" rule.
- `A088` [hidden] `core/src/primitives/display/syntax-token-line.tsx:13-26` `normalizeHexColor` copies `core/src/theme/colors.ts:53-63` `normalizeHex` (plus an 8-digit case).
- `A089` [hidden] `core/src/primitives/display/syntax-token-line.tsx:115` and `core/src/primitives/display/rich-text.tsx:27-29` pass raw `#hex`/`literalColor` straight to Ink, bypassing `resolveLiteralColor`. Under `NO_COLOR` or mono mode, syntax and literal colours still print, while intent colours obey the runtime.
- `A090` [hidden] `core/src/app/error-boundary.tsx:38` hard-codes `color="red"`, bypassing the theme and colour runtime (so it ignores mono and `NO_COLOR`).
- `A091` [surplus] `core/src/runtime/terminal/tty-capabilities.ts:3-12`: `DetectTtyColorsOptions` copies `DetectColorRuntimeOptions` (`theme/colors.ts:10-15`) field for field, and `detectTtyColors` only calls `detectColorRuntime(options)`.
- `A092` [needless] `core/src/runtime/terminal/tty-capabilities.ts:1` makes the low-level runtime layer import the `theme` barrel, so terminal capability code changes when theme exports change.
- `A093` [special] `core/src/theme/colors.ts:425` `normalizeThemeColorMode` accepts `"none"` as an alias for `"mono"`, a value nothing sends (the function itself is unused).
- `B057` [needless] `ink/blocks/shared/rich-text-renderer.tsx:100-102` renders inline `code` with the `warning` intent, so recolouring warnings recolours inline code.
- `B061` [surplus] `ink/primitives/rich-text-runs.ts:63-68` (`withBackgroundIntent`) ignores its `_backgroundIntent` and returns a clone. Callers that rely on it for backgrounds (`interactive-block.tsx:29`, `:45`; `prefixed-rich-list-item.tsx:40`; `rich-table-row.tsx:31`) get no background.
- `B062` [surplus] `ink/primitives/rich-text-runs.ts:38` and `rich-text-runs.ts:74` accept `_backgroundIntent` and never use it. `ink/primitives/rich-table-row.tsx:34` and `:75` pass it anyway.
- `B064` [hidden] `ink/primitives/prefixed-rich-list-item.tsx:44` passes a `ColorIntent` name (`"sectionSubheading"`, `"dimText"`, `"info"`, `"accent"`, `"text"`) to Ink's `color` prop. Ink's colorize (`node_modules/ink/build/colorize.js`) treats unknown names as no colour, so list markers and conversation role labels (`conversation-block.tsx:24`, `:33`) render uncoloured and nothing reports it.
- `B068` [needless] `ink/blocks/shared/syntax-line.tsx:9-10` accepts both `backgroundIntent` and `backgroundColor`, and `backgroundColor` always wins (`syntax-line.tsx:34-39`). Its only caller, `ink/primitives/command-line-row.tsx:35-36`, passes both.
- `B071` [hidden] `ink/blocks/note-block.tsx:24-47` hard-codes note label, foreground, background and border hex colours outside the theme. They bypass the colour runtime (passed raw to Ink at `note-block.tsx:72-73` and through `AdmonitionFrame`), so mono/NO_COLOR/ansi16 modes still emit hex colours.
- `B072` [special] `ink/blocks/note-block.tsx:66-67` falls back to `cardBackground` when `palette.backgroundColor` is empty, but every palette entry is non-empty, so the fallback never runs.
- `B073` [hidden] `ink/primitives/command-line-row.tsx:16` and `:28`: `useIntentColor("commandBarBackground")` always returns `undefined`, because `commandBarBackground` is in the inherit set at `ink/blocks/shared/intent-color.ts:11`. The `#000000` fallback is therefore always used, and the theme's `commandBarBackground` (`ink/theme/index.ts:66`) never has an effect.
- `B074` [hidden] `ink/blocks/shared/intent-color.ts:3-14` hand-lists the intents that "inherit terminal background", duplicating the `"default"` values in `ink/theme/index.ts:6`, `:17-21`, `:66`, `:68`, `:72`. The lists have already diverged: `sidebarItemActiveBackground` is in the set but the theme gives it `#1f4acc` (`theme/index.ts:37-41`).
- `B075` [special] `ink/blocks/shared/intent-color.ts:17-19` returns before calling `useTuiTheme`/`useTuiColors`, so the hook count depends on the intent value. `ink/blocks/shared/syntax-line.tsx:37-39` calls `useIntentColor` only when `backgroundIntent` is set. `ink/blocks/table-block.tsx:52-58` calls `useIntentColor` after an early `return null`. All three call hooks conditionally.
- `B076` [hidden] `ink/theme/index.ts:8-16`: `border` and `sidebarBorder` have identical values. `theme/index.ts:44-53`: `accent` and `linkText` are identical. `theme/index.ts:37-41` vs `:59-63`: active backgrounds share ansi256 `26`. These are copies with no shared source.
- `B129` [hidden] Every launcher forces `FORCE_COLOR=3` (truecolor). In ansi256 mode the kit returns bare numbers like `"111"` (`tui/kit/core/src/theme/colors.ts:334-335`, `:294-304`), and Ink's colorize ignores them, so colour vanishes. The forced env silently hides this.
- `B176` [hidden] `install-docs/style-map.md:10` and `:47` ("Change colors globally: edit theme intents") are contradicted by the hex colours in `ink/blocks/note-block.tsx:24-47` and `ink/primitives/command-line-row.tsx:16`.
- `B177` [hidden] `install-docs/style-map.md:31` ("full-row background") and `style-map.md:35` ("pad/truncate/background") describe background behaviour that `ink/primitives/rich-text-runs.ts:63-68` turns into a no-op and that `command-line-row.tsx:28` forces to black.

### F7. Keymap as data, with one dispatcher and focus scopes.

Solves 30 problems (19 hidden · 4 needless · 2 surplus · 5 special).

A screen declares its keys as data: `{keys, label, when?, run}`. One kit dispatcher, typed on Ink's `Key`, matches key names exactly: Ctrl-C is `C-c`, never `c`; `G` isn't `g`; Shift-Tab is `S-tab`. It sends each keypress to the innermost active scope only. A text-entry scope takes printable keys, and panes are scopes, so `isActive` flags, inline focus checks and nav-lock refs go. Every app quits on `C-c` unless its keymap's `quit` entry says otherwise. disk-plan and recab both use `quit.when` with a visible "can't quit: apply running" message. The footer, the status bar and the key section of `--help` render from the same keymap, so hints can't drift, and READMEs stop listing keys. Pane focus (`FocusPlane`) is one kit scope that install-docs and disk-plan both use. The hand-kept VT escape tables go once F23 makes the split pane forward standard xterm sequences.

**Done when:** No app calls Ink `useInput` directly; no footer, status or help string names a key outside a keymap; typing `q` into disk-plan's token field doesn't quit; Ctrl-C quits every app the same way; install-docs' nav-lock refs are gone.

**Depends on:** F5.

- `A008` [hidden] `core/src/runtime/ink/use-input.ts:7-24,178` redeclares Ink's `Key` as `InkKeyLike` and casts `key as InkKeyLike`. If Ink adds or renames key flags (for example `super`), the names table at `:94-132` won't follow.
- `A029` [hidden] `core/AGENTS.md:5` says "The kit owns … terminal input, focus". disk-plan reads raw input with Ink's `useInput` (`disk-plan/presentation/disk-plan-screen.tsx:293`) and keeps its own pane focus (`:170`), because the kit has no input capture.
- `A062` [needless] `core/src/hooks/use-focus-plane.ts:4` `FocusPlane = "navigation" | "content"` is used only by install-docs (`install-viewer.tsx:214`, `components/status-bar.tsx:7`). disk-plan re-invents the same thing as `"nav" | "content"` (`disk-plan/presentation/disk-plan-screen.tsx:170`).
- `A077` [needless] `core/src/theme.ts:173-180` `footerHint` builds the hint string `"q quit | arrows navigate"` inside the theme module. It is key-binding text, unrelated to colour or layout, so the theme changes whenever keymap wording does.
- `A128` [hidden] `core/src/runtime/ink/use-input.ts:72-75` always adds the raw and lowercased input as names, even when `key.ctrl`/`key.meta` is set (Ink reports Ctrl-C as input `"c"`). So a binding for `"c"` also fires on Ctrl-C (`install-docs/presentation/ink/screens/install-viewer.tsx:467` copy plus `:223` quit both run), and `"q"` fires on `Q`. Every binding depends on this name-expansion rule, and none can opt out.
- `A129` [hidden] `core/src/runtime/ink/use-input.ts:74`: lowercasing means `"G"` also matches handlers bound to `"g"`. `core/src/patterns/review-screen.tsx:37-38` and `install-viewer.tsx:453-466` bind `["g","home"]` and `["G","end","S-g"]`. Pressing `G` runs both, and it lands on "end" only because the `G` handler was registered second.
- `A130` [hidden] `core/src/runtime/ink/use-input.ts:163-198` `useKeyInput` registers a global Ink listener per call with no focus or capture layer. `TextField` (`components/forms/text-field.tsx:45,49-67`) binds `"q"` to cancel and also appends every printable key, so typing `q` both cancels and inserts. disk-plan bypasses the kit with raw `useInput` (`disk-plan/presentation/disk-plan-screen.tsx:293-313`) for its confirm token while its unconditional `useHotkeys(["q",…], quit)` (`:526`) still listens.
- `A131` [hidden] `core/src/app/render-app.tsx:44` defaults `exitOnCtrlC: false`. Every app must then bind `"C-c"` itself (`install-viewer.tsx:223`, `disk-plan-screen.tsx:526`, `recab/presentation/runtime-recab-screen.tsx:141`) and also passes `exitOnCtrlC: false` again (`disk-plan/app/bootstrap.tsx:14`, `recab/app/bootstrap.tsx:12`, `install-docs/app/bootstrap.tsx:45`). Callers just have to know this convention.
- `A132` [hidden] `core/src/hooks/use-focus-plane.ts:31` binds `tab`/`S-tab` globally unless `bindTabKeys: false`. An app that also binds tab (as `disk-plan-screen.tsx:527` does with its own pane focus) would toggle twice.
- `A133` [hidden] Hotkey hints are hand-written strings, separate from the bindings they describe: `core/src/theme.ts:175`, `core/src/components/feedback/status-line.tsx:185`, `core/src/patterns/review-screen.tsx:47`, `core/src/patterns/wizard-flow.tsx:96,215,249,287`, and in apps `install-docs/presentation/ink/components/status-bar.tsx:26-40`. `hotkeyStatusSegment` (`status-line.tsx:113`) and `useHotkeys` share no data, so a rebinding leaves the hint wrong. For example, `review-screen.tsx:47` omits the `g/G/b/space` keys it binds at `:35-38`.
- `A136` [surplus] `core/src/hooks/use-hotkeys.ts:10-16` `useHotkeys` passes every argument straight to `useKeyInput`, and `:23` `useTuiInput` is a third name for the same function.
- `A138` [special] `core/src/runtime/ink/use-input.ts:38-45,134-158` keeps hand-written VT escape-sequence tables "for some terminals", in parallel with Ink's own key parsing.
- `A139` [special] `core/src/runtime/ink/use-input.ts:113` emits `"S-tab"` instead of `"tab"` when shift is held. That's a one-off name scheme: shift+letter emits both `x` and `S-x` (`:73-82`), but shift+tab never emits `tab`.
- `B102` [hidden] The keymap is written four times: `ink/screens/install-viewer.tsx:409-469` (bindings), `ink/components/status-bar.tsx:26-40` (labels), `src/cli/help.ts:11-21` (help) and `install-docs/README.md:19-25`. They already disagree: `b`/`space`/arrows are bound but absent from help, and README lacks Tab/m/[ ]/Enter/c/y.
- `B103` [needless] `ink/screens/install-viewer.tsx:441-466` gates `pageup`/`pagedown`/`g`/`G` with inline `if (!focus.isContent) return`, while `install-viewer.tsx:412-440` and `:467-469` use the `isActive` option. Two mechanisms do one job.
- `B104` [special] `ink/screens/install-viewer.tsx:215-216` and `:290-318` keep refs (`contentNavLock`, `contentNavAllowedIndex`) that undo page changes made while content focus is active, with a runtime note "blocked navigation mutation…". This guards against a state the `isActive`-gated hotkeys should already prevent.
- `B105` [hidden] `ink/screens/install-viewer.tsx:401-402` must set `contentNavAllowedIndex.current` before `navigation.setPage`, or the lock effect at `install-viewer.tsx:300-318` reverts the jump. Nothing states this call-order rule.
- `B110` [needless] `ink/components/status-bar.tsx:11-18` (`installStatusBar`) takes six positional parameters, two optional with defaults the only caller always overrides (`status-bar.tsx:21`; caller `install-viewer.tsx:480-487`).
- `B111` [hidden] `ink/components/status-bar.tsx:45` and `ink/screens/install-viewer.tsx:380-381` each format the `i/n` action counter.
- `B112` [surplus] `ink/components/status-bar.tsx:34` adds a second `tab` segment ("tab content") next to `status-bar.tsx:27` ("tab toggle pane").
- `B113` [special] `src/app/bootstrap.tsx:45` sets `exitOnCtrlC: false` only because `ink/screens/install-viewer.tsx:223` binds `C-c` itself.
- `B168` [hidden] `install-docs/README.md:22-25` says `j`/`k` scroll and `PgUp`/`PgDn`/`g`/`G` work unconditionally. In content focus `j`/`k` cycle actions (`ink/screens/install-viewer.tsx:426-437`), and PgUp/PgDn/g/G only work in content focus (`install-viewer.tsx:441-466`).
- `C054` [hidden] Footer hotkey strings (`disk-plan-screen.tsx:648-684`) are written separately from the `useHotkeys` bindings (`:526-645`) and are wrong in nav focus. "back" is shown as `tab`, but `tab` toggles the pane and back is `h`/`k`/`←`/`↑`. `mode m/h/l` and `pick enter/h/j/k/l` are shown in nav focus, where `h`/`l`/`j`/`k` move between pages. Confirmed on the rendered welcome footer (`j/k/h/l next | tab back`).
- `C055` [hidden] `dp/README.md:34-37` keybinds ("Enter / n / → / l: next page", "p / ← / h: previous page") match neither binding. `n`/`p` act only in content focus (`disk-plan-screen.tsx:534-545`), Enter in nav focus opens the content pane (`:614-620`), and tab/j/k/`F` are missing.
- `C056` [hidden] `dp/src/presentation/disk-plan-screen.tsx:293-312`: on the confirm page every letter typed goes into the token and also reaches `useHotkeys` (`n`, `p`, `m`, `h`/`j`/`k`/`l`, `q`, `F`). The generated `CONFIRM_TOKEN` (`DESTROY`) happens to share no letter with those keys. A token change in recpart (`tools/recpart/src/types.rs:12`) could make typing it navigate or quit.
- `C057` [hidden] Enter on the confirm page fires both `useInput` → `startApply` (`:301-303`) and `useHotkeys(["enter"])` → `onNext` or the pane switch (`:614-628`). Correctness depends on both handlers' early returns.
- `C059` [hidden] `rc/cli/parse-args.ts:53-56` help lists "j/k, arrows", "enter" and "q, esc, ctrl-c" separately from the bindings in `rc/presentation/runtime-recab-screen.tsx:141-157` and the footer (`:181-188`). Only the per-action hotkeys come from the table.
- `C060` [hidden] `rc/flows/index.ts:37` derives each set-next hotkey from the slot letter (`a`, `b`), next to fixed `r`/`c`/`x` (`:29,45,53`) and quit `q` (`runtime-recab-screen.tsx:141`). A new slot letter colliding with an action key is not detected.
- `C061` [hidden] `rc/presentation/runtime-recab-screen.tsx:158-161` registers hooks in a loop over `RUNTIME_RECAB_ACTIONS`. It is valid only while the table is static, which only a comment says.
- `C062` [special] `dp/src/presentation/disk-plan-screen.tsx:452-458`: `quit` silently ignores q/Esc/Ctrl-C while apply runs, with no message. recab's `quit` (`runtime-recab-screen.tsx:141`) exits during a running mutating action. The two apps follow different policies for the same situation.

### F8. No app vocabulary in the kit.

Solves 17 problems (7 hidden · 9 needless · 1 surplus).

The kit theme has generic intents (text, dimText, accent, border, selection, link, warning, error, success, …) and no layout dimensions for one screen. install-docs extends it with its own intents (sidebar*, commandPrompt, card*, section*) and layout. Docs-specific code moves into install-docs: the `[[fg=…]]` markup parser (as the only copy, so the app's duplicate token helpers go), `linkRuns` (one link-text rule), the pager status format, and the docs defaults and mode type of the nav list (one name for the mode). The rule is written in `core/package.json`, `core/README.md` and install-docs' `CLAUDE.md`: the kit holds generic, vocabulary-free code, and anything that names docs, sidebars or one app's screens belongs to that app.

**Done when:** `rg -i 'docs|sidebar|commandBar|card' tui/kit/core/src` finds nothing app-specific; install-docs' sidebar passes only what differs from the nav-list defaults.

**Depends on:** F5, F6.

- `A023` [hidden] `core/README.md:16` says "Docs-domain rendering/navigation no longer lives in `tui-kit`". `core/src/components/navigation/tree-nav.tsx:52` (`"Docs"`), `:96` (`"(no docs pages)"`), `core/src/primitives/display/syntax-token-line.tsx:44` (the docs `[[fg=…]]` markup), `core/src/primitives/display/link.tsx:21` (`linkRuns`) and `core/src/components/feedback/status-line.tsx:167` (pager keymap) are docs-viewer code still in the kit.
- `A034` [hidden] `core/package.json:5` says "Reusable terminal UI primitives". The package also carries install-docs vocabulary (see below) and workflow patterns (`WizardFlow`, `ReviewScreen`) that no app uses.
- `A054` [needless] `core/src/theme/tokens.ts:5-35` `ColorIntent` includes `sidebarSectionText`, `sidebarItem*`, `commandBarBackground`, `commandPrompt`, `sectionHeading`, `sectionSubheading` and `card*`. These name install-docs regions. `commandPrompt`/`commandBarBackground` are read only by `install-docs/presentation/ink/primitives/command-line-row.tsx:23-24`. Every change to the docs viewer's palette roles changes the kit's central type.
- `A055` [needless] `core/src/theme/tokens.ts:50-56,256-262` `ThemeLayout.sidebarWidth`, `headerHeight` and `footerHeight` describe one two-pane screen shape, so the general theme carries one layout's dimensions.
- `A056` [needless] `core/src/theme/tokens.ts:64` `sidebarHeaderMode: "current-section-title" | "all-section-headers"` names the docs sidebar's modes. The real modes live in `components/navigation/tree-nav.tsx:6` and again in `install-docs/presentation/ink/components/sidebar.tsx:5`.
- `A057` [needless] `core/src/components/navigation/tree-nav.tsx:52,96` default section `"Docs"` and empty label `"(no docs pages)"`. Its only consumer is `install-docs/presentation/ink/components/sidebar.tsx:23-43`, which overrides every option. `TreeNav` changes whenever the docs sidebar does.
- `A058` [needless] `core/src/components/navigation/tree-nav.tsx:174-178` defaults its colours to `sidebar*` intents, so the general tree is tied to "sidebar" theme roles.
- `A059` [needless] `core/src/primitives/display/syntax-token-line.tsx:44-94` `parseSyntaxTokenLine` parses an ad-hoc `[[fg=#hex]]…[[/]]` markup that only the docs content pipeline produces. Its only consumers are install-docs (`blocks/code-block.tsx:44`, `blocks/shared/syntax-line.tsx:18,40`). The markup format is owned by the docs producer but parsed in the kit.
- `A060` [needless] `core/src/primitives/display/link.tsx:21-49` `linkRuns` (with `includeHrefWhenDifferent`) is used only by `install-docs/presentation/ink/blocks/shared/rich-text-renderer.tsx:89`.
- `A061` [needless] `core/src/components/feedback/status-line.tsx:157-191` `formatPagedStatusLine` hard-codes the pager keymap `"q quit | h/l page | j/k scroll | g/G top/end"` and page/lines counters for one docs pager.
- `A076` [hidden] `core/src/theme/tokens.ts:257` `sidebarWidth: 30` is repeated as the default `maxWidth ?? 30` in `components/navigation/section-list.tsx:34` and `tree-nav.tsx:78`.
- `A143` [hidden] `core/src/components/navigation/tree-nav.tsx:6` `TreeNavMode` is copied as `SidebarMode` in `install-docs/presentation/ink/components/sidebar.tsx:5` and named differently in `ThemeChrome.sidebarHeaderMode` (`core/src/theme/tokens.ts:64`). That's three names for one setting.
- `A160` [hidden] `core/src/primitives/display/link.tsx:21-49` `linkRuns` and `:51-82` `UiLink` each re-implement the selected/unselected intent choice and the href-suffix rule (`:22-25` vs `:70`), with different underline and bold defaults.
- `A161` [hidden] `core/src/primitives/display/syntax-token-line.tsx:32-42` `pushToken` is copied as `pushSyntaxToken` in `install-docs/presentation/ink/blocks/code-block.tsx:27-37`.
- `B066` [hidden] `ink/blocks/code-block.tsx:18-21` re-declares the kit's exported `SyntaxToken` type (`syntax-token-line.tsx:8-11`). `code-block.tsx:27-37` (`pushSyntaxToken`) copies the kit's private `pushToken` (`syntax-token-line.tsx:32-42`).
- `B079` [surplus] `ink/components/sidebar.tsx:33-36` and `:39-42` pass `emptyLabel="(no docs pages)"`, `▾`, `▸`, `▸`, `dimText`, `sidebarItemText`, `sidebarItemActiveText` and `sidebarItemActiveBackground`. These are identical to the kit `TreeNav` defaults (`tui/kit/core/src/components/navigation/tree-nav.tsx:85-87`, `:96`, `:175-178`).
- `B080` [needless] `tui/kit/core/src/components/navigation/tree-nav.tsx:96` uses the kit default empty label `"(no docs pages)"`. That is install-docs vocabulary inside a generic kit component.

### F9. One frame and width model, and one app shell that fits the space it gets.

Solves 50 problems (33 hidden · 2 needless · 6 surplus · 9 special).

The kit has one way to draw a box: a frame renderer that produces border rows for a known width from one glyph table (single, double, round, bold; one `BorderStyle` type). Ink `borderStyle`, the `ChromeLayer` instruction renderer and install-docs' hand-joined glyph strings (code blocks, admonitions, tables, markers) all give way to it. Text width is measured one way everywhere, with `string-width` (Ink's model), and there's one positive-integer helper. One `AppShell` (nav pane, content pane, footer) replaces the pane config the three apps copy. It computes geometry once (title rows, borders, padding from the theme) and hands each pane its inner size through a hook, so apps never call `resolveSurfaceFrameGeometry` or pass title rows and widths. Below the app's two-pane width the shell collapses to one pane with the nav on a toggle key, and it only refuses to draw below a hard minimum of about 40 columns. That way the docs app works at the split pane's right-pane width, which is the contract between the split pane's layout and the theme. The viewport comes only from Ink's `useStdout` and resize events. `chrome-architecture.md` is replaced by a short section in `core/README.md`, and disk-plan's grid breakpoints and recab's widths derive from the shell's pane sizes.

**Done when:** `cargo xtask tui inspect install-docs --columns 60 --rows 24` shows docs content; the kit has one glyph table and one width function; no app passes title rows, padding or widths to the shell; F4's right-pane-width render passes.

**Depends on:** F5, F6, F7.

- `X009` [special] `tui/kit/core/src/theme.ts:50-66` minimum 88 columns for the docs layout, while the split pane gives the docs app about half the console. Verified: `cargo xtask inspect docs --slug installation --columns 60 --rows 24` renders only "Terminal too small: got 60x24, need 88x24". On a typical framebuffer console (100–128 columns) the LevitateOS install screen's right pane shows nothing but that message. The width the docs app gets is decided by the split pane (Rust, builder repo) and the width it needs by the kit theme (TS, tui repo); nothing connects them.
- `A032` [hidden] `install-docs/../chrome-architecture.md:4,34-40` says chrome is "rendered from deterministic coordinates, not content layout" and that `SurfacePane` frames are chrome-owned. `core/src/surfaces/frame/pane.tsx:97-126` draws only the top three rows with `ChromeLayer`. The sides and bottom come from Ink's `borderStyle` (`:110-115`, cli-boxes glyphs). The doc describing kit internals lives in an app, and `:45` still names the old `docs/tui` path.
- `A033` [hidden] `install-docs/../chrome-architecture.md:21` says "No ASCII `-` fallback is used for structural chrome". `core/src/utils/strings.ts:45,50` (`horizontalRule` defaults to `-`) and `core/src/components/data/table.tsx:37` (`-`/`-+-` separators) draw rules in ASCII.
- `A063` [needless] `core/src/utils/strings.ts:110-144` `normalizeTextWidth`, `truncateBoundedLine`, `wrapBoundedText` and `prefixWrappedText` default `minimum = 20`. Every external caller passes `1` (`install-docs/blocks/command-block.tsx:28,31,74,78`, `blocks/list-block.tsx:98`, `blocks/table-block.tsx:56`, `rendering/plan/line-metrics.ts`), and inside the kit `section-list.tsx:34,50,59` and `tree-nav.tsx:96,117,138` pass `1` too. The `20` default serves no caller.
- `A075` [hidden] `core/src/theme/tokens.ts:256-262` `defaultThemeLayout.minColumns: 80, minRows: 24` repeats `DEFAULT_TERMINAL_SIZE` (`core/src/runtime/terminal/screen-size.ts:8-11`) and the `80` fallback at `core/src/primitives/layout/grid.tsx:54`.
- `A079` [special] `core/src/theme.ts:60-68` `normalizeThemeLayout` silently raises `minRows`/`minColumns` to fit header, footer and sidebar, so a caller's explicit minimum is overwritten for one layout shape.
- `A094` [hidden] There are four separate ways to draw boxes and rules: Ink `borderStyle` (`core/src/primitives/display/panel.tsx:29`, `surfaces/frame/pane.tsx:110,138`); the instruction renderer `ChromeLayer` (`chrome/components.tsx:120`, used only at `pane.tsx:97-104`); raw glyph-string concatenation from `resolveChromeGlyphSet` (`primitives/display/divider.tsx:12`, `install-docs/presentation/ink/primitives/admonition-frame.tsx:31-38`, `blocks/code-block.tsx:107`); and ASCII `horizontalRule` (`utils/strings.ts:45`, `components/data/table.tsx:37`). A border-style change has to be made in each.
- `A095` [hidden] `core/src/surfaces/frame/pane.tsx:97-126` joins a `ChromeLayer` top (kit glyphs from `chrome/glyph-sets.ts`) to an Ink-bordered body (cli-boxes glyphs) with `borderTop={false}`. The seam lines up only while the kit's glyph tables and cli-boxes agree for each style name.
- `A096` [hidden] `core/src/surfaces/frame/pane.tsx:74` (`chromeHeaderRows = 3`) and `core/src/surfaces/frame/surface-frame.tsx:48` (`paneTitleRows` returns `2`, plus `BORDER_ROWS = 2` at `:18`) each encode the titled-pane row budget. They agree only by arithmetic, and nothing links them.
- `A097` [hidden] `core/src/surfaces/frame/surface-frame.tsx:16-26`: `HORIZONTAL_PADDING_COLUMNS = 2` and `paddingY` `0` (`:129,131`) assume `theme.chrome.panePaddingX = 1`/`panePaddingY = 0`, while `pane.tsx:64-65` renders with the theme values. Changing theme padding desynchronises the text-area geometry from what is drawn.
- `A098` [hidden] `core/src/surfaces/frame/surface-frame.tsx:172-184` computes geometry internally but doesn't expose it to pane bodies. So `install-docs/presentation/ink/screens/install-viewer.tsx:252-262` recomputes it with the same inputs copied by hand, and hard-codes `leftTitleRows: 2, rightTitleRows: 2` instead of `paneTitleRows`. The caller has to mirror SurfaceFrame's internals.
- `A100` [hidden] Three places set the left/right pane intent defaults independently: `core/src/surfaces/frame/surface-frame.tsx:217-231` (sidebar defaults), `core/src/patterns/two-pane.tsx:113-127` (the same values again), and `core/src/surfaces/frame/pane.tsx:49-52` (`border`/`background`/`text`/`sectionHeading`).
- `A101` [hidden] All three apps pass the same left-pane block (`title`, `titleMode: "inline"`, `borderIntent: "sidebarBorder"`, `textIntent: "sidebarItemText"`, `titleIntent: "sidebarSectionText"`), the same right-pane block (`borderIntent: "cardBorder"`, `textIntent: "text"`, `titleIntent: "sectionHeading"`) and `showHeader={false}`: `install-viewer.tsx:226-248,495-523`, `disk-plan/presentation/disk-plan-screen.tsx:1153-1170`, `recab/presentation/runtime-recab-screen.tsx:232-252`. The shared shell is copied rather than owned by the kit.
- `A102` [surplus] `core/src/surfaces/frame/surface-frame.tsx:137,187-192`: `showHeader` defaults to `true` and the `title` header with `"tui"` fallback exists, but every app passes `showHeader={false}`, so `title` is never shown.
- `A103` [surplus] `core/src/surfaces/frame/types.ts:5` `SurfacePaneTitleMode` has `"inline"`, which every app passes, but `pane.tsx:63,69` only distinguishes `"none"`, so `"inline"` behaves like `"slot"`.
- `A107` [surplus] `core/src/surfaces/frame/frame-canvas.tsx:16-59` `FrameCanvas` has one caller (`surface-frame.tsx:195`), which passes 16 props one by one from the geometry object.
- `A108` [hidden] `core/src/surfaces/frame/frame-canvas.tsx:5-14` and `core/src/surfaces/frame/pane.tsx:15-25` are identical `renderNode` helpers.
- `A109` [hidden] `core/src/surfaces/frame/pane.tsx:34-39` `paneTitleText` and `core/src/surfaces/frame/surface-frame.tsx:41-46` repeat the same string/number/bigint title coercion.
- `A110` [hidden] `core/src/surfaces/frame/pane.tsx:27-32` `normalizeDimension`, `surface-frame.tsx:51-56` `normalizeMinWidth`, `runtime/terminal/screen-size.ts:13-15` `normalizeDimension` (a different rule under the same name), and `utils/clamp.ts:13-15` `toPositiveInt` are four copies of "finite, floored, ≥1".
- `A111` [special] `core/src/surfaces/frame/surface-frame.tsx:185` calls `resolveLayoutGuards` a second time after `resolveSurfaceFrameGeometry` already did (`:91`), because geometry doesn't return the resolved guards.
- `A112` [special] `core/src/surfaces/frame/surface-frame.tsx:87` `bodyRows = Math.max(3, …)` sets a floor of 3 for one layout. It can exceed the real space after the minimum-size check passes.
- `A113` [special] `core/src/surfaces/frame/pane.tsx:80` `innerColumns` falls back to `40` when width is unknown, but that branch is only reached when `canRenderChromeTitle` is false, where `titleCell` isn't used.
- `A114` [special] `core/src/surfaces/frame/pane.tsx:84-85` maps `"none"` to `"single"` for `chromeLineWeight`, but the chrome path only runs when `hasBorder` is true, so the branch is dead.
- `A117` [hidden] The union `"single" | "double" | "round" | "bold"` is written out separately at `core/src/theme/tokens.ts:59`, `core/src/chrome/types.ts:4`, `core/src/primitives/display/divider.tsx:11`, `core/src/patterns/two-pane.tsx:19-20`, `core/src/surfaces/frame/surface-frame.tsx:30` and (with `"none"`) `core/src/surfaces/frame/types.ts:14`, plus the runtime check at `core/src/theme.ts:86-89`. `pane.tsx:85` casts between them. It also has to match Ink's `borderStyle` names (`pane.tsx:110`).
- `A118` [hidden] `core/src/surfaces/frame/types.ts:4` `SurfacePaneTitleStyle` copies `ThemeChrome.titleStyle` (`core/src/theme/tokens.ts:60`).
- `A119` [hidden] `core/src/surfaces/frame/types.ts:15-16` and `core/src/primitives/display/panel.tsx:9` each restrict border and background intents to hand-picked subsets of `ColorIntent`.
- `A120` [hidden] `core/src/chrome/components.tsx:32-40,114-118`: `ChromeFrame`, `ChromeSeam` and the others render `null` and work only when `ChromeLayer` walks its children (`:95-112`). Used anywhere else they silently draw nothing, and nothing in their types says so.
- `A121` [hidden] `core/src/chrome/components.tsx:42-93` `childToInstruction` re-reads each node's props with casts, a second copy of the `ChromeInstruction` shapes at `core/src/chrome/types.ts:27-61` that typecheck can't link.
- `A122` [surplus] `core/src/chrome/types.ts:33-46`: the `rule` and `seam` instruction kinds have identical shapes. `chrome/renderer.ts:13-21` handles them in two branches that do the same thing, `chrome/grid.ts:195-201` `drawSeam`/`drawSplit` only forward to `drawHorizontal`/`drawVertical`, and `ChromeRuleProps` equals `ChromeSeamProps` (`types.ts:90-109`).
- `A123` [surplus] `core/src/chrome/grid.ts:276-278` `createChromeGrid` only calls `new ChromeGrid`.
- `A124` [hidden] `core/src/chrome/grid.ts:24-26` defines its own `clamp`, a copy of `core/src/utils/clamp.ts:29-33` `clampNumber`. `grid.ts:17-22` `normalizeCoordinate` is another finite/floor helper.
- `A125` [special] `core/src/chrome/components.tsx:140-149` memoizes on `children`, which is a new object every render, so the memo never hits.
- `A126` [needless] `core/src/chrome/components.tsx:10` imports `UiText` from `primitives`, while `core/src/primitives/display/divider.tsx:2` imports from `chrome`. The two folders depend on each other.
- `A127` [hidden] `core/src/chrome/glyph-sets.ts:45-57` `ROUND_GLYPHS` repeats every non-corner glyph of `SINGLE_GLYPHS` (`:3-15`).
- `A153` [hidden] Three width models: `core/src/utils/strings.ts:15-108` measures with `.length` (UTF-16 units), `core/src/chrome/grid.ts:209` with `Array.from` (code points, one cell each), and Ink lays out with `string-width`. Truncation, padding and chrome placement disagree with Ink for wide or combining characters.
- `A154` [hidden] `core/src/utils/clamp.ts:13-23`: `toPositiveInt` clamps to ≥1 and `toPositiveIntOrFallback` falls back instead. Nothing in the names says which one a caller gets. Inline copies of `toNonNegativeInt` appear at `core/src/theme.ts:74-82`, `core/src/theme/spacing.ts:13`, `core/src/primitives/layout/grid.tsx:102-103` and `core/src/surfaces/frame/pane.tsx:119-120,143,148-149`, and of `toPositiveInt` at `components/navigation/tree-nav.tsx:78`, `primitives/layout/grid.tsx:39` and `chrome/components.tsx:130-131`.
- `A166` [hidden] `core/src/app/app-provider.tsx:40-43` reads `process.stdout.columns/rows` directly on every render as a fallback, next to Ink's `useStdout()`. `core/src/runtime/terminal/screen-size.ts:42-50` `getTerminalSize` is a third reader of the same global.
- `A167` [hidden] `core/src/app/app-provider.tsx:48-54` and `:61-67` are the same compare-and-set block, copied.
- `A169` [surplus] `core/src/runtime/terminal/screen-size.ts:13-15` `normalizeDimension` only calls `toPositiveIntOrFallback`.
- `A184` [special] `core/src/components/navigation/tree-nav.tsx:210,217,228`, `core/src/primitives/display/rich-text.tsx:46,63`: each renders `" "` in place of empty text, the same Ink workaround repeated at each site.
- `B025` [hidden] `ink/screens/install-viewer.tsx:252-263` calls `resolveSurfaceFrameGeometry` itself with hard-coded `leftTitleRows: 2`, `rightTitleRows: 2`, `hasFooter: true` and `hasHeader: false`. These copy what `tui/kit/core/src/surfaces/frame/surface-frame.tsx:172-184` derives internally via `paneTitleRows()`. If the kit's title-row or guard rules change, the app's content row and column counts diverge from the actual pane.
- `B070` [hidden] `ink/blocks/code-block.tsx:107-158` and `ink/primitives/admonition-frame.tsx:31-71` assemble box frames by string concatenation of glyphs instead of using the kit chrome (`ChromeFrame`/`ChromeGrid`). That contradicts `install-docs/chrome-architecture.md:4`.
- `B077` [hidden] `ink/blocks/code-block.tsx:107` hard-codes `resolveChromeGlyphSet("single")` and `ink/primitives/admonition-frame.tsx:31` hard-codes `"round"`, ignoring `theme.chrome.borderGlyphSet` (`ink/theme/index.ts:81`).
- `B078` [hidden] `ink/blocks/table-block.tsx:17` (`"─┼─"`, `"─"`) and `ink/primitives/rich-table-row.tsx:19` (`" │ "`) hand-copy box glyphs instead of using the kit glyph set.
- `B081` [hidden] `ink/document/section-heading-item.tsx:17` (`◆`/`›`), `ink/blocks/command-block.tsx:48` (`▸ ` selection prefix) and `ink/components/sidebar.tsx:34-36` (`▾`/`▸`) are hand-picked glyphs with no shared table.
- `B114` [hidden] `ink/theme/index.ts:76` (`minColumns: 88`) together with `sidebarWidth: 32` (`theme/index.ts:75`) assumes a wide pane. The live split pane gives the docs app 50% of the console minus borders (`distro/builder/install-split-pane/src/main.rs:31-32`, `:708-716`). Below 88 columns, `SurfaceFrame` replaces the viewer with "Terminal too small" (`tui/kit/core/src/surfaces/frame/surface-frame.tsx:153-170`), and no check runs at the real pane width.
- `B175` [hidden] `install-docs/style-map.md:3` and `install-docs/chrome-architecture.md:45` refer to the old path `docs/tui`. `chrome-architecture.md:45` names the component `InstallViewer`; it is `InstallViewerScreen` (`ink/screens/install-viewer.tsx:198`).
- `B178` [hidden] `install-docs/chrome-architecture.md:1-49` documents `@levitate/tui-kit` internals (`ChromeGrid`, `ChromeLayer`, glyph sets, the `SurfacePane` contract) from the app folder, so it changes with the kit, not with this app. `chrome-architecture.md:4` ("rendered from deterministic coordinates, not content layout") does not hold for the app's own frames (`ink/blocks/code-block.tsx:107-158`, `ink/primitives/admonition-frame.tsx:31-71`).
- `C052` [special] `dp/src/presentation/disk-plan-screen.tsx:179-188` disk-grid breakpoints `110`/`180` and `leftWidth={38}` (`:1156`) are free layout numbers.
- `C085` [hidden] `rc/presentation/runtime-recab-screen.tsx:201,235`: the sidebar `maxWidth={36}` must fit inside `leftWidth={40}`. Two coupled numbers with nothing linking them.

### F10. The kit owns the app lifecycle: args, TTY, errors, exit codes and processes.

Solves 27 problems (13 hidden · 1 needless · 8 surplus · 5 special).

`runApp({name, args, render})` is the only entry an app needs. It parses argv from a declared spec (required values, aliases, and help text generated from the spec plus the F7 keymap), checks for a TTY, and mounts one provider; the theme is readable without an app, so atoms render anywhere. It renders with Ink's own types, and on a render error it restores the terminal and exits 1 with the stack on stderr instead of swallowing the error. `runCommand(argv)` is the one process runner: Bun.spawn, an inherited environment stated once, and a typed error result for a missing binary. `errorMessage(e)` is the one error idiom. `useAsyncTask` catches synchronous throws and takes an explicit key instead of relying on a memoised `load`. Gone: the `createTuiApp` + `renderApp` double path, the `exitOnCtrlC` convention, `REMOVED_FLAGS`, `AppError.code`, `--recab-bin` (tools come from PATH, which F3 sets), and the fallback-label strings.

**Done when:** Each app's `main.ts` is one `runApp` call; the three `cli/run.ts` runners and two `parse-args.ts` loops are gone; a thrown render error exits 1 with a stack; every app's `--help` comes from its spec.

**Depends on:** F5, F7.

- `A007` [hidden] `core/src/app/render-app.tsx:12-16,45` redeclares Ink's render handle as `InkRenderHandle` and casts `render(...) as unknown as InkRenderHandle`. Ink's `Instance` type can change without the kit's typecheck noticing. `rerender?.` at `:64` is optional only because of this local type.
- `A170` [needless] Layout and display primitives (`UiText`, `Divider`, `Panel`, `RichTextLine`, `SyntaxTokenLine`, `UiLink`, `Grid`) all call `useTuiApp`-backed hooks (`core/src/primitives/display/text.tsx:13-14`, etc.), which throw outside `AppProvider` (`core/src/app/app-provider.tsx:24-26`). The lowest-level atoms depend on the app shell, and only `renderApp` mounts that provider.
- `A171` [surplus] `core/src/app/create-app.tsx:24-36` plus `core/src/app/render-app.tsx:33-41`: `renderApp` can build the app itself from the same options, yet all three apps call `createTuiApp` and then pass `{ app }` (`disk-plan/app/bootstrap.tsx:11-15`, `recab/app/bootstrap.tsx:9-13`, `install-docs/app/bootstrap.tsx:29-46`). There are two ways to do one step.
- `A172` [hidden] `core/src/app/render-app.tsx:7-10,34-41`: `RenderAppOptions` takes both `app` and `title`/`theme`/`colorMode`, and when `app` is given the others are silently ignored.
- `A173` [surplus] `core/src/app/create-app.tsx:17` `TuiApp` is only an alias of `TuiAppContextValue`.
- `A174` [surplus] `core/src/app/render-app.tsx:60-62` wraps `handle.waitUntilExit()` in an async function that returns the same promise result.
- `A175` [special] `core/src/app/render-app.tsx:28` plus `core/src/app/error-boundary.tsx:29-39` wrap every app in an `ErrorBoundary` with no `onError`. A render crash is swallowed into a red one-line message, the stack is lost, and the process keeps running in raw mode instead of exiting non-zero.
- `A176` [special] `core/src/app/create-app.tsx:21`, `core/src/patterns/page-frame.tsx:15`, `core/src/surfaces/frame/surface-frame.tsx:191`, `core/src/theme.ts:174` and `core/src/components/feedback/status-line.tsx:100` each have their own fallback label (`"tui-kit"`, `"tui-kit"`, `"tui"`, `"tui"`, `"tui"`).
- `A177` [special] `core/src/app/app-provider.tsx:73-77` falls back from `stdout.off` to `removeListener?.` for a stream shape that only the hand-written `globals.d.ts` allows.
- `A180` [hidden] `core/src/patterns/wizard-flow.tsx:138` `useAsyncTask(load, [])` and `core/src/hooks/use-async-task.ts:66` put `load` in effect deps. Callers must memoise `load` (and `toCard`, `:179`) or the task reloads every render, and nothing tells them.
- `A181` [special] `core/src/hooks/use-async-task.ts:38`: `Promise.resolve(load(...))` doesn't catch a synchronous throw from `load`, which escapes the effect instead of landing in `error`.
- `B083` [hidden] `src/cli/parse-args.ts:29-34`, `parse-args.ts:41-46` and `src/domain/navigation/slug-guard.ts:9-12` each check for an empty slug with the identical message `"Flag '--slug' requires a non-empty page slug."`.
- `B084` [special] `src/cli/parse-args.ts:41` rejects any `--slug` value that starts with `-`.
- `B120` [surplus] `src/app/errors.ts:3` and `:8` (`AppError.code`) are never read. `errors.ts:5` defaults `exitCode = 2`, and `src/app/bootstrap.tsx:26` passes `2` explicitly anyway.
- `B121` [hidden] `src/app/bootstrap.tsx:25` inlines the body of `formatErrorMessage` (`src/app/errors.ts:13-19`).
- `B125` [surplus] `src/cli/parse-args.ts:1` keeps a `REMOVED_FLAGS` list (`--list`, `-l`, `--page`, `-p`, `--all`, `-a`, `--tmux-mode`), and `parse-args.ts:20-25` prints a special message for them.
- `B126` [surplus] `src/cli/help.ts:23` and `install-docs/README.md:17` still describe removed legacy flags.
- `B127` [hidden] `src/cli/parse-args.ts:39` accepts `-s` as an alias that `src/cli/help.ts:5-9` and README do not mention.
- `C018` [hidden] `dp/src/app/plan-runner.ts:27,35,84` run a bare `recpart` from PATH. `rc/cli/parse-args.ts:10,22-30` take `--recab-bin` with default `recab` from PATH. Two binary-discovery conventions for sibling apps, and disk-plan only works against this checkout because `xtask/src/tasks/tui.rs:34-42` prepends `target/debug` to PATH.
- `C019` [hidden] `dp/src/app/plan-runner.ts:44-58` (`Bun.spawn`; a missing binary throws, so the screen gets an `error` state) and `rc/app/recab-runner.ts:5-48` (`node:child_process.spawn`; a missing binary becomes `exitCode: null` plus a stderr message) are two process runners with different failure semantics for the same job.
- `C020` [surplus] `dp/src/app/plan-runner.ts:60-71` `runRecpartPlan`/`runRecpartDryRun`/`startRecpartApply` are one-line wrappers that each pair a builder with `spawnRecpart`.
- `C021` [surplus] `rc/app/recab-runner.ts:14` passes `env: process.env`, which is spawn's default.
- `C022` [hidden] `rc/app/recab-runner.ts:14` relies on the inherited environment to carry recab's `RECAB_*` path overrides (`tools/recab/src/lib.rs:148-149`), which is how the prior untangle "verified" the TUI against a scratch tree. Nothing in the TUI states that.
- `C046` [hidden] The error-message idiom `error instanceof Error ? error.message : String(error)` is repeated at `dp/src/cli/run.ts:24`, `disk-plan-screen.tsx:236,335,364,400,444`, `rc/cli/run.ts:22` and `rc/app/recab-runner.ts:28`. install-docs has a helper (`tui/apps/live-tools/install-docs/src/app/errors.ts`) and the kit does it inside `useAsyncTask`.
- `C094` [hidden] `dp/src/cli/run.ts:4-28`, `rc/cli/run.ts:4-26` and `tui/apps/live-tools/install-docs/src/cli/run.ts:6-34` are near-identical CLI runners (help → exit 2 on error → TTY check → `waitUntilExit().catch` → exit 1). The kit offers `renderApp` but no runner, so a change to exit-code policy needs three edits.
- `C095` [hidden] `dp/src/cli/parse-args.ts:16-58` and `rc/cli/parse-args.ts:9-38` copy the same hand-rolled argv loop ("requires a value", `startsWith("-")`, "unknown argument").
- `C096` [hidden] `dp/src/app/bootstrap.tsx:10-16` and `rc/app/bootstrap.tsx:8-14` copy `createTuiApp({title}) + renderApp(..., {exitOnCtrlC: false})`. Each app also re-implements Ctrl-C quitting via `useHotkeys(["q","escape","C-c"])` (`disk-plan-screen.tsx:526`, `runtime-recab-screen.tsx:141`).

### F11. One generic component per job in the kit.

Solves 19 problems (11 hidden · 1 needless · 7 special).

The kit keeps one implementation per job:
- **Index state:** one hook (clamp or wrap, re-clamped when the count changes) under list, grid and nav.
- **Navigation list:** one `NavList` (sections, items, marker, and the empty label as a prop) replaces `SectionList` and `TreeNav`; recab, install-docs and disk-plan's page list all use it.
- **Grid:** one grid whose `SelectableGrid` and `useGridNavigation` share a column resolver and count gaps.
- **Styled text:** one styled-run type with one line renderer and one run word-wrapper. install-docs' wrapper moves into the kit, and the plain-text wrappers delegate to it, so install-docs' `CLAUDE.md` rule ("generic primitives live in the kit") holds.
- **Small helpers:** `truncateLines` for "first N lines + more"; `useScrollState` owns its maximum; blank lines render through the line renderer (no per-site `" "`); `KeyValue` pads with the kit width helper.

**Done when:** Each of those jobs has one implementation in `rg`; no hook is called after an early return in the kit.

**Depends on:** F5, F8, F9.

- `A140` [hidden] Clamped or wrapped index state is implemented separately in `core/src/runtime/ink/focus-manager.ts:11-46` (wraps), `core/src/hooks/use-list-navigation.ts:19-37` (clamps), `core/src/hooks/use-grid-navigation.ts:61-116`, `core/src/components/forms/select.tsx:21-29`, and `core/src/patterns/wizard-flow.tsx:51-63,137-157`. None of them uses the others.
- `A141` [hidden] `core/src/hooks/use-list-navigation.ts:11-17` and `core/src/hooks/use-grid-navigation.ts:26-31` have identical `safeCount` helpers. `core/src/runtime/ink/focus-manager.ts:12,39` inlines the same rule twice.
- `A142` [hidden] `core/src/components/navigation/section-list.tsx:18-23` and `core/src/components/navigation/tree-nav.tsx:43-48` have identical `marker()` helpers. `section-list.tsx:40-56` and `tree-nav.tsx:55-72` group items by section twice, so `SectionList` and `TreeNav` are two components for the same job (recab uses one, install-docs the other).
- `A144` [hidden] `core/src/components/navigation/selectable-grid.tsx:52` resolves columns from the viewport internally, while the caller's `useGridNavigation` needs the same count. So `disk-plan/presentation/disk-plan-screen.tsx:189` repeats `resolveGridColumns(diskColumnsSpec, viewport.columns, 1)` to keep navigation and rendering in step.
- `A145` [hidden] `core/src/primitives/layout/grid.tsx:93,101` (`columns = 2`, fallback `2`) and `core/src/components/navigation/selectable-grid.tsx:43,52` (`columns = 2`, fallback `1`) disagree on the fallback column count. `SelectableGrid` then passes the resolved number plus `viewportColumns` to `Grid` (`:59,62`), which resolves again.
- `A146` [hidden] `core/src/primitives/layout/grid.tsx:67-80` `groupGridRows` copies `core/src/utils/arrays.ts:11-22` `chunkArray`.
- `A150` [hidden] `core/src/hooks/use-scroll-state.ts:15-25`: `scrollBy` needs the caller to pass `maxScroll` on every call, and `scrollToBottom` defaults to `Number.MAX_SAFE_INTEGER`, so the stored offset is only valid if the render path clamps it later. Callers just have to know that.
- `A157` [special] `core/src/components/data/key-value.tsx:17` starts `maxKey` at 1 and uses `.padEnd` instead of the kit's `padRight`.
- `A158` [special] `core/src/components/navigation/tree-nav.tsx:81-84` hand-rolls `clampIndex` (`clampNumber(toNonNegativeInt(…))`) with its own empty guard.
- `A159` [hidden] There are three run types for coloured text: `RichTextRun` (`core/src/primitives/display/rich-text.tsx:6-14`), `StatusAtom` (`core/src/components/feedback/status-line.tsx:20-26`) and `SyntaxToken` (`core/src/primitives/display/syntax-token-line.tsx:8-11`), each with its own renderer (`RichTextLine`, `SegmentedStatusLine` at `status-line.tsx:70-97`, `SyntaxTokenLine`). install-docs adds a fourth local `SyntaxToken` (`install-docs/presentation/ink/blocks/code-block.tsx:18-21`) and converts between them.
- `A162` [special] `core/src/primitives/display/rich-text.tsx:45-50` returns before calling `useTuiTheme`/`useTuiColors`, so the hook order changes when `runs` goes from empty to non-empty (a rules-of-hooks violation that React reports at runtime).
- `A163` [special] `core/src/primitives/display/text.tsx:17-29`: an `if (typeof children === "string")` branch renders exactly the same element as the fallthrough.
- `A185` [special] `core/src/components/navigation/section-list.tsx:31` and `core/src/components/forms/select.tsx:19` each hard-code `"(no items)"`, while `core/src/patterns/wizard-flow.tsx:81,198` say `"No items available."`.
- `A187` [special] `core/src/components/navigation/selectable-grid.tsx:32-38` `widthForColumns` sizes items at `floor(100/columns)%`, ignoring `gapX`, so rows with gaps overflow.
- `A188` [special] `core/src/components/navigation/selectable-grid.tsx:6-9`: `RenderItemContext` isn't exported but appears in the exported `SelectableGridProps.renderItem` type.
- `B060` [needless] `ink/blocks/shared/rich-text-renderer.tsx:145-264` is a generic rich-run word-wrapper living in an app, while the kit owns `wrapText`/`prefixWrappedText` (`tui/kit/core/src/utils/strings.ts:120-147`) for plain text. CLAUDE.md rule 4 says generic primitives go in the kit.
- `B065` [hidden] `ink/blocks/shared/syntax-line.tsx:23-94` (`SyntaxLine`) re-implements the kit's `SyntaxTokenLine` (`tui/kit/core/src/primitives/display/syntax-token-line.tsx:101-120`), adding padding and background.
- `B173` [hidden] `install-docs/CLAUDE.md:32` ("Keep generic runtime/UI primitives in tui/kit/core") is contradicted by `ink/blocks/shared/rich-text-renderer.tsx`, `ink/blocks/shared/syntax-line.tsx`, `ink/primitives/rich-text-runs.ts`, `ink/primitives/admonition-frame.tsx` and the OSC 52 clipboard at `ink/screens/install-viewer.tsx:188-196`.
- `C045` [hidden] `dp/src/presentation/disk-plan-screen.tsx:75-81` `limitLines` and `rc/presentation/runtime-recab-screen.tsx:27-39` `compactOutput` are two copies of the same "first N lines + `... (k more lines)`" truncation, with different defaults (16 and 10).

### F12. install-docs lays each block out once: block → lines.

Solves 72 problems (36 hidden · 10 needless · 23 surplus · 3 special).

Each block type is one module that lays out a docs-content block at a given width as lines: styled runs plus anchors for links and actions. The viewer draws `lines.slice(scroll, scroll + rows)`, counts lines from the same array, and builds its action and link targets from the anchors, with each link at its own line. Dispatch is an exhaustive `switch` on docs-content's `DocsBlock['type']`. Gone: the plugin maps, the registry, renderer keys, nested-block sentinel items, `domain/render`, `adapters/`, re-declared docs-content types, pass-through wrapper components, and the unused `DocsRenderStyleContext` (so the domain no longer imports kit types). Highlighted fields are optional input (`highlightedLines ?? plain text`) rather than validated against a docs-content build step, and the clipboard copies the same `command` the highlight is made from. Only highlighted fields are parsed as colour markup. Table cells wrap onto more rows instead of being cut.

**Done when:** No block module has a separate measure function; adding a block type touches one module and the switch, and typecheck fails until both exist; `content-pane.tsx`'s negative margin, `extractBlockLinks` and `deriveActionableTargets` are gone; links inside QA answers highlight when selected; F4 snapshots scroll every page to its end without drift.

**Depends on:** F9, F11.

- `A064` [needless] `install-docs/domain/content/contracts.ts:9,27-28`: an app domain contract imports kit `TuiTheme`/`ColorRuntime`, so the kit's theme type shape leaks into docs domain types.
- `B001` [hidden] `ink/blocks/code-block.tsx:98-103` and `ink/blocks/code-block.tsx:170-178` compute the same inner width, line-number digits and gutter width separately for render and for measure. Nothing ties them together, so a change to one makes the scroll math wrong.
- `B002` [hidden] `ink/blocks/command-block.tsx:28-32` and `ink/blocks/command-block.tsx:74-79` wrap the description and output separately for render and for measure.
- `B003` [hidden] `ink/blocks/command-block.tsx:24-27` appends `  [COPY]` to the first command line when selected, and `ink/blocks/command-block.tsx:45-51` renders command lines unwrapped at full length. Measure (`command-block.tsx:75`) counts exactly one row per line, so a long line or the selection suffix wraps in Ink and the line count no longer matches what is drawn.
- `B004` [hidden] `ink/blocks/list-block.tsx:89-105` measures with the kit's plain-text `prefixWrappedText`, while render (`list-block.tsx:27-35`, via `ink/primitives/prefixed-rich-list-item.tsx:34-40`) wraps with the app's own `wrapRichTextRuns`. Two wrap algorithms are in use for one layout.
- `B005` [hidden] `ink/blocks/conversation-block.tsx:82-95` measures with the kit's `prefixWrappedText`, while render (`conversation-block.tsx:28-56`) uses `PrefixedRichListItem`/`wrapRichTextRuns`. Same two-algorithm split.
- `B006` [hidden] `src/rendering/plan/line-metrics.ts:29-31` measures the intro with the kit's `wrapBoundedText`, while `ink/document/intro-item.tsx:23-29` renders it with the app's `RichTextRenderer` word-wrapper. The measured and drawn line counts can differ.
- `B007` [hidden] `ink/blocks/list-block.tsx:23` and `list-block.tsx:87` build the ordered/bullet marker twice. `list-block.tsx:57` and `list-block.tsx:103` subtract the child indent `2` twice.
- `B008` [hidden] `ink/blocks/conversation-block.tsx:23` and `conversation-block.tsx:81` derive the role label twice. `conversation-block.tsx:50` and `conversation-block.tsx:92` each subtract the list indent `2`.
- `B009` [hidden] `ink/blocks/qa-block.tsx:36` and `qa-block.tsx:82` each compute the question width `safeWidth - 2`. `qa-block.tsx:46` and `qa-block.tsx:89` each hard-code the nested answer indent `2`. `qa-block.tsx:49-53` and `qa-block.tsx:85-86` each handle the empty answer. `qa-block.tsx:54` and `qa-block.tsx:93` each add the spacer row. Every QA layout rule is written twice.
- `B010` [hidden] `ink/blocks/qa-block.tsx:76` ignores `_indent` in measure and `qa-block.tsx:36` ignores `indent` for the question width. A nested QA silently overflows, and nothing marks that the two must agree.
- `B011` [hidden] `ink/blocks/note-block.tsx:69` and `note-block.tsx:127` repeat the narrow-width cutoff `< 4`, which `ink/primitives/admonition-frame.tsx:23` repeats a third time. `note-block.tsx:85` and `note-block.tsx:130` each derive the text width as `width - 4`, which must equal `admonition-frame.tsx:32-34` (two border columns plus two padding columns).
- `B012` [hidden] `ink/blocks/note-block.tsx:132` assumes the admonition frame adds exactly 3 rows (top, label, bottom). That number belongs to `ink/primitives/admonition-frame.tsx:43-70`.
- `B013` [hidden] `ink/blocks/table-block.tsx:46-51` and `table-block.tsx:120-127` each build the plain-text cell matrix and `computeColumnWidths`. Measure uses it only to detect an empty table, matching the early `return null` at `table-block.tsx:52-54`.
- `B014` [hidden] `ink/blocks/interactive-block.tsx:74` counts one row per step command, but `interactive-block.tsx:35-40` renders it unwrapped and padded through `SyntaxLine`. Long commands wrap in Ink and the count is wrong.
- `B015` [hidden] `ink/blocks/text-block.tsx:42-44` re-derives `contentWidth - indent` for measure, mirroring `text-block.tsx:16-17`.
- `B016` [hidden] `src/rendering/plan/line-metrics.ts:91-93` (one blank line between items), `ink/document/docs-content-view.tsx:33` (the spacer `<UiText>`) and `ink/screens/install-viewer.tsx:181-183` (the cursor `+1`) encode the same inter-item gap three times.
- `B017` [hidden] `src/rendering/plan/line-metrics.ts:45-46`, `line-metrics.ts:74-75` and `ink/document/renderer-registry.tsx:74-77` each normalise `indent` with the same finite/non-negative guard.
- `B018` [needless] `src/rendering/plan/line-metrics.ts:4` imports `DEFAULT_INSTALL_BLOCK_PLUGINS` from `presentation/ink/blocks/plugins`, so the "rendering" layer depends on React/Ink presentation modules. The import goes against the advertised layer order.
- `B019` [hidden] Adding or changing a block type touches `ink/blocks/<type>-block.tsx` (render and measure), `ink/blocks/plugins.tsx:13-23`, `src/rendering/plan/line-metrics.ts:47-50`, `ink/document/renderer-registry.tsx:67-72` and `ink/screens/install-viewer.tsx:80-143` (link extraction). The folder layout hides that these files change together.
- `B020` [hidden] `ink/screens/install-viewer.tsx:80-143` (`extractBlockLinks`) walks every block type's inline fields by hand, mirroring which fields each block plugin renders. Its `default: return []` (`install-viewer.tsx:140-141`) silently drops links from any new block type.
- `B021` [needless] `src/rendering/pipeline/viewport.ts:27-49` (`resolveInitialDocSelection`) is navigation logic that lives in the rendering pipeline. Its only caller is `ink/hooks/use-install-navigation.ts:5`.
- `B022` [needless] `src/domain/render/types.ts:1-32` sits in `domain/` but holds render-plan types consumed only by `rendering/` and `presentation/`.
- `B023` [needless] `src/domain/content/contracts.ts:9` and `contracts.ts:26-29` make the "domain" layer import `@levitate/tui-kit` types (`ColorRuntime`, `TuiTheme`) for `DocsRenderStyleContext`.
- `B024` [hidden] `ink/screens/install-viewer.tsx:145-186` (`deriveActionableTargets`) re-walks the plan with `measureDocItemLines` to rebuild line positions that `src/rendering/plan/line-metrics.ts:79-96` (`countRenderPlanLines`) already computes. Two cursors must stay in step.
- `B026` [needless] `ink/components/content-pane.tsx:18-38` renders every item of the document every frame and clips it with a negative `marginTop` (`content-pane.tsx:28`). Correct scrolling depends on the measured line totals matching Ink's layout exactly.
- `B027` [surplus] `ink/document/renderer-registry.tsx:94-101` (`createInstallDocsRendererRegistry(overrides)`): `overrides` is never passed. `src/app/bootstrap.tsx:33` builds the only registry, which is then threaded through `InstallViewerScreen` → `InstallContentPane` → `DocsContentView` → `renderDocItemWithRegistry` as a prop.
- `B028` [needless] `ink/document/renderer-registry.tsx:67` reads block plugins from the module-global `DEFAULT_INSTALL_BLOCK_PLUGINS` while node plugins come from the injected registry. The extension point covers only the 3 node kinds, not blocks.
- `B029` [surplus] `ink/document/renderer-registry.tsx:17-29` and `renderer-registry.tsx:49-92` form a typed plugin map with a redundant `kind` field per entry, for exactly three fixed node kinds (`intro`, `section`, `block`).
- `B030` [special] `ink/document/renderer-registry.tsx:40-45`, `renderer-registry.tsx:68-72` and `src/rendering/plan/line-metrics.ts:47-50` throw "missing plugin" at runtime for maps whose types (`DocNodePluginMap`, `BlockPluginMap`) are already total. That is three hand-written copies of the same remediation text.
- `B031` [surplus] `ink/blocks/contracts.ts:43-45` (`defineBlockPlugins`) is an identity function.
- `B032` [surplus] `ink/blocks/contracts.ts:6-9` (`BlockHotkeyHint`) and `contracts.ts:36` (`hotkeysHelp`) are declared and never implemented or read.
- `B033` [surplus] `ink/blocks/contracts.ts:25` (`rendererKey`) is set by every plugin (`text-block.tsx:33`, `code-block.tsx:165`, `command-block.tsx:63`, `table-block.tsx:110`, `list-block.tsx:73`, `conversation-block.tsx:67`, `interactive-block.tsx:58`, `qa-block.tsx:64`, `note-block.tsx:116`) and never read.
- `B034` [hidden] `ink/blocks/shared/renderer-key.ts:5-7` formats keys as `docs:block:<type>`, while `docs/content/src/render-contract.ts:44-46` defines `defaultDocsBlockRendererKey` as `block.<type>`. These are two "default renderer key" functions with different formats. `docs/content/src/types.ts:47-52` says renderers should honour `block.rendererKey`, and the app dispatches on `block.type` only.
- `B035` [hidden] `ink/blocks/contracts.ts:4` and `ink/blocks/shared/renderer-key.ts:3` each re-declare `DocsBlockType`, which `docs/content/src/render-contract.ts:3` already exports. `ink/blocks/contracts.ts:39-41` (`BlockPluginMap`) re-implements `DocsBlockRendererMap` from `render-contract.ts:31-33`.
- `B036` [hidden] `ink/document/renderer-registry.tsx:103-114` (`nestedBlockNode`) fabricates a `DocBlockItem` with sentinel values (`sectionIndex: -1`, `sectionTitle: "(nested)"`, `blockIndex: -1`) and a non-unique key `nested-block-${type}`. `renderer-registry.tsx:123-129` passes `selectedItemKey: undefined`, so `renderer-registry.tsx:84-86` never matches. Links inside QA answers are made actionable by `ink/screens/install-viewer.tsx:132-134` but are never highlighted when selected.
- `B037` [hidden] `ink/blocks/types.ts:12` (`RenderNestedBlock`, `indent` required) re-declares `ink/blocks/contracts.ts:13` (`renderBlock`, `indent` optional). `ink/blocks/qa-block.tsx:71-73` wraps one in the other.
- `B038` [surplus] `ink/blocks/types.ts:4-10` (`BlockComponentProps`) and `ink/blocks/contracts.ts:11-16` (`BlockRenderContext`) carry the same fields. Each plugin's `render` (e.g. `text-block.tsx:34-41`) only copies context fields into props.
- `B039` [surplus] `src/domain/content/contracts.ts:11` re-exports `DocsPageMeta`, `DocsProduct`, `DocsScope` and `NavSection` from `@levitate/docs-content` unchanged.
- `B040` [surplus] `src/domain/content/contracts.ts:31-32` (`DocsContentLike = DocsContent`, `RichTextLike = RichText`) are pure aliases. `RichTextLike` is never used.
- `B041` [hidden] `src/domain/content/contracts.ts:13-17` (`DocsSource`) hand-copies the shape of `docsNav`/`contentBySlug`/`metaBySlug` exported by `docs/content/src/generated` (`docs/content/src/index.ts:17`).
- `B042` [surplus] `src/adapters/docs-content/mapper.ts:4-6` (`createMappedDocsSource`) only calls `loadDocsSource()` and maps nothing.
- `B043` [surplus] `src/adapters/docs-content/source.ts:4-10` packs three imports into an object. `metaBySlug` (`source.ts:8`) is never read anywhere.
- `B044` [surplus] `src/domain/render/types.ts:30-32` (`DocRenderPlan = { items }`) wraps one array. Callers immediately unwrap `.items` (`src/rendering/pipeline/viewport.ts:69`).
- `B045` [surplus] `src/domain/render/types.ts:14` and `types.ts:21-24` (`sectionIndex`, `sectionTitle`, `sectionLevel`, `blockIndex`) are written by `src/rendering/plan/build-doc-plan.ts:33-44` and never read.
- `B046` [surplus] `ink/primitives/rich-paragraph.tsx:14-30` (`RichParagraph`) passes every prop straight to `RichTextRenderer`, renaming `intent` to `defaultIntent`.
- `B047` [surplus] `ink/primitives/command-line-series.tsx:14-36` adds only the first-line/continuation prefix choice over `CommandLineRow`.
- `B048` [hidden] `ink/document/intro-item.tsx:13-31` and `ink/blocks/text-block.tsx:10-29` are the same component (Box with padding, `RichParagraph`, width minus indent) except for the intent.
- `B049` [needless] `ink/components/content-pane.tsx:6-41` and `ink/document/docs-content-view.tsx:15-38` form a two-level wrapper chain over `renderDocItemWithRegistry`, each re-receiving the same `renderers`/`selectedItemKey`/`selectedLinkHref` props.
- `B050` [hidden] `ink/blocks/shared/content-utils.ts:9-33` validates `highlightedLines`/`highlightedCommandLines` at render time, and `content-utils.ts:5` points at `bun run build` in docs/content. These fields are optional in `docs/content/src/types.ts:68` and `:128`, so the app depends on a docs-content build step that the type does not declare.
- `B051` [hidden] `ink/blocks/shared/content-utils.ts:9-20` and `content-utils.ts:22-33` are line-for-line copies for code and command blocks.
- `B052` [hidden] `ink/screens/install-viewer.tsx:59-61` copies the raw `block.command` to the clipboard, while display and measure use `highlightedCommandLines` (`ink/blocks/command-block.tsx:23`, `:75`). These are two sources for one command, and nothing checks that they agree.
- `B053` [hidden] `src/rendering/plan/line-metrics.ts:6-17` (`inlineNodeToPlain`), `ink/blocks/shared/rich-text-renderer.tsx:63-74` (`plainTextForInlineNode`), `tui/kit/core/src/primitives/display/link.tsx:24-46` (`linkRuns`) and `link.tsx:70` (`UiLink`) each encode "show link as `text (href)` when href differs". If one changes, measured widths stop matching drawn widths.
- `B054` [hidden] `src/rendering/plan/line-metrics.ts:19-27` (`inlineContentToPlainText`) duplicates `ink/blocks/shared/rich-text-renderer.tsx:273-281` under the same name.
- `B055` [hidden] `ink/blocks/shared/rich-text-renderer.tsx:28-37` (`sameRunStyle`) and `rich-text-renderer.tsx:39-49` (`cloneRun`) enumerate every field of the kit's `RichTextRun` (`tui/kit/core/src/primitives/display/rich-text.tsx:6-14`). A new run field in the kit is silently dropped here.
- `B056` [hidden] `ink/blocks/shared/rich-text-renderer.tsx:39-49` (`cloneRun`, explicit fields) and `ink/primitives/rich-text-runs.ts:7-9` (`cloneRuns`, spread) clone runs in two different ways.
- `B058` [needless] `ink/blocks/shared/rich-text-renderer.tsx:299-308` (`wrapRichTextPlainLines`) takes a `defaultIntent` that cannot affect plain-text output. Callers still pass `"text"`/`"dimText"` (`note-block.tsx:131`, `interactive-block.tsx:71`, `qa-block.tsx:82`).
- `B059` [surplus] `ink/blocks/shared/rich-text-renderer.tsx:310-316` (`richTextRuns`) is exported and never imported.
- `B063` [special] `ink/primitives/rich-table-row.tsx:28-29` wraps each cell and then keeps only the first wrapped line, so long cells are silently truncated. `ink/blocks/table-block.tsx:131` measures `2 + rows.length` to match, and neither says so.
- `B067` [surplus] `ink/blocks/shared/syntax-line.tsx:17-21` (`syntaxTokenColors`) is never used.
- `B069` [hidden] `ink/blocks/interactive-block.tsx:35-40` feeds raw `step.command` (plain text per `docs/content/src/types.ts:111`) through `SyntaxLine` → `parseSyntaxTokenLine`. Any literal `[[` in a command is parsed as colour markup.
- `B082` [hidden] `ink/blocks/table-block.tsx:34` caps column width at the magic number `32`, which is unrelated to `theme.layout.sidebarWidth` `32` (`theme/index.ts:75`) but reads the same.
- `B107` [surplus] `ink/screens/install-viewer.tsx:276-279`: the `useMemo` depends on `docsViewport.visibleItems`, a new array every render (`src/rendering/pipeline/viewport.ts:68-69`), so it recomputes every render.
- `B108` [hidden] `ink/screens/install-viewer.tsx:170-178` gives every link target the whole block's `startLine`/`endLine`. `install-viewer.tsx:345-359` (`focusActionable`) therefore scrolls to the block, not the link. `selectedLinkHref` matching by href (`rich-text-renderer.tsx:86-88`) highlights every link with that href in the item.
- `B115` [needless] `src/rendering/pipeline/viewport.ts:78-79`: `visibleItems` holds all items, not the visible ones, and `totalItems` holds total lines. `viewport.ts:74-75` `startItem`/`endItem` are line numbers. Callers (`install-viewer.tsx:488`) must know the names are wrong.
- `B116` [surplus] `src/rendering/pipeline/viewport.ts:60` (`_slug`) and `viewport.ts:64` (`_styleContext`) are unused parameters. `DocsRenderStyleContext` (`src/domain/content/contracts.ts:26-29`) exists only for the latter.
- `B117` [surplus] `ink/document/section-heading-item.tsx:8` and `:14` (`indent`) are never passed.
- `B118` [surplus] `ink/primitives/command-line-row.tsx:9` and `:27` (`rowIndex`, consumed by `void rowIndex`), plus `ink/primitives/command-line-series.tsx:8` (`startRowIndex`, used only for React keys; passed by `interactive-block.tsx:38`).
- `B119` [surplus] `ink/primitives/command-line-row.tsx:11-12` (`fallbackIntent`, `backgroundIntent`) are never passed by any caller.
- `B122` [surplus] `ink/primitives/index.ts:1-7` is a barrel nothing imports (it appears in the zero fan-in list, `.artifacts/untangle-ui/map/fanin.txt`).
- `B123` [surplus] `ink/primitives/rich-text-runs.ts:3-5` (`runsLength`) and `:7-9` (`cloneRuns`) are exported but only used inside the file.
- `B124` [special] `ink/blocks/qa-block.tsx:7-8` imports from `./shared/rich-text-renderer` twice.

### F13. Apps get distro identity from their launcher, and install-docs fails fast.

Solves 22 problems (8 hidden · 7 surplus · 7 special).

Distro facts (title, default page, recommended disk mode) are declared in the variant (F18) and passed as argv by the launcher, e.g. `install-docs --page installation --title "AcornOS install guide"` or `disk-plan --mode mutable --title …`. The apps hold no distro tables. install-docs' profile system goes (`domain/distro/*`, `allowsMeta`, scopes): the nav shows what docs-content exports, as `CLAUDE.md:29` already says, under the title the launcher passes. Session setup has one path. `--page` is checked against the nav, and a wrong page exits 2 with the list of pages. Gone: the default-slug ladder, `missingPage`, the "unknown slug" note, the empty-nav frame and the `index` slug. In the viewer, the clipboard write goes through Ink's stdout and reports failure in the status bar, and the unused `onExit` goes.

**Done when:** `rg -i levitate tui/apps/live-tools/install-docs/src` finds only package names; an unknown `--page` exits 2; title and default page come from the variant. The builder side lands with F19.

**Depends on:** F10.

- `B085` [special] `src/app/session.ts:15-35` (`resolveDefaultSlug`) has three fallback levels (profile default, first nav item, first allowed slug). The final throw (`session.ts:30-32`) is unreachable because `session.ts:43-45` already threw on empty.
- `B086` [special] `src/app/session.ts:42` and `session.ts:47` each call `flattenDocsNav` through `resolveAllowedSlugs`/`resolveAllowedSlugSet`. `session.ts:48` (`buildInstallNavItems`) flattens a third time.
- `B087` [hidden] `src/domain/scope/allowed-slugs.ts:13-18` dedupes slugs, but `src/domain/navigation/nav-model.ts:44` (`filterNavItemsByAllowedSlugs`) keeps duplicates. `allowedSlugs` and `navItems` can disagree on the same source.
- `B088` [special] `src/domain/content/index.ts:4-14` (`missingPage` fallback) is unreachable, because `navItems` are already filtered to slugs with content (`allowed-slugs.ts:9`). It is also the kind of hidden fallback `install-docs/CLAUDE.md:31` forbids.
- `B089` [special] `ink/hooks/use-install-navigation.ts:26-30` builds an "unknown slug, showing …" startup note. It is unreachable because `src/app/session.ts:53` already throws on an unknown slug, and it contradicts `install-docs/CLAUDE.md:30`.
- `B090` [special] `ink/screens/install-viewer.tsx:224-249` renders a whole alternate "no docs pages" frame that `src/app/session.ts:49-51` makes unreachable. It duplicates the pane config at `install-viewer.tsx:498-522` and returns before the hooks at `install-viewer.tsx:276-330` (a conditional hook count).
- `B091` [surplus] `src/rendering/pipeline/viewport.ts:51-56` (`resolveInitialDocIndex`) is never used.
- `B092` [special] `src/domain/navigation/nav-model.ts:20` maps an empty href to the literal slug `"index"`, a page the nav does not have.
- `B094` [surplus] `src/domain/navigation/nav-model.ts:40-52`: `filterNavItemsByAllowedSlugs` and `buildInstallNavItems` are one-line wrappers.
- `B095` [surplus] `src/domain/distro/levitate-profile.ts:4-17` (`ALLOWED_PRODUCTS`, `ALLOWED_SCOPES`, `isAllowedProduct`, `isAllowedScope`) and `levitate-profile.ts:25-29` (`allowsMeta`) are never called. No code reads `allowedProducts`, `allowedScopes` or `allowsMeta` (`src/domain/distro/profile.ts:8-11`). The live sidebar therefore shows every docs section (Package Manager, Helpers, Architecture, Rec Tooling; `snapshots/inspect/120x36/001-installation-disk.txt:4-20`) under the title "Installation Docs".
- `B096` [surplus] `src/domain/distro/registry.ts:6-13`: the `acorn` profile (`allowsMeta: () => false`) cannot be selected. `src/cli/run.ts:20` never passes `distro`, and AcornOS ships a plain-text `less` frontend instead (`distro/variants/acorn/ring2/products.toml:128-129`, `distro/builder/src/pipeline/live_tools.rs:446-497`).
- `B097` [surplus] `src/app/bootstrap.tsx:13-14` and `bootstrap.tsx:18`: the `distro?: DistroId` option is never supplied, so the profile is always `"levitate"`. Neither argv, env nor the ISO build (`live_tools.rs:438-444` uses `distro_id` only in an error message) carries a distro.
- `B098` [surplus] `src/domain/distro/profile.ts:1-12`, `levitate-profile.ts` and `registry.ts` are three files for one profile whose only live fields are `defaultSlug` and `id` (used in error text, `src/app/session.ts:31`, `:44`, `:50`).
- `B099` [hidden] `src/domain/distro/levitate-profile.ts:21` (`"LevitateOS Field Manual"`) and `ink/screens/install-viewer.tsx:202` (default `title`) are copies. Neither is ever shown: `install-viewer.tsx:497-499` passes `title` to `SurfaceFrame` with `showHeader={false}` (`tui/kit/core/src/surfaces/frame/surface-frame.tsx:187-205` uses it only for the header).
- `B100` [hidden] Distro and app identity is hard-coded in separate places that must agree: `src/cli/help.ts:3` ("LevitateOS Docs TUI"), `src/app/bootstrap.tsx:30` ("levitate-install-docs"), `ink/screens/install-viewer.tsx:232` and `:501` ("Installation Docs", twice), `install-docs/package.json:2` (`@levitate/docs-tui`) and `package.json:7` (`levitate-install-docs`).
- `B101` [hidden] `src/domain/distro/levitate-profile.ts:24` (`defaultSlug: "installation"`) is repeated in `distro/builder/src/pipeline/live_tools.rs:609` (`docs-tui --slug installation`), `distro/builder/src/artifact/live_overlay.rs:368`, `README.md:52` (root) and `src/install-viewer.chrome.pty.test.ts:39`.
- `B106` [special] `ink/screens/install-viewer.tsx:188-196` (`copyToClipboardOsc52`) writes raw OSC 52 to `process.stdout` underneath Ink and swallows any exception (`install-viewer.tsx:193`). A generic terminal capability is implemented in the app rather than the kit.
- `B109` [surplus] `ink/screens/install-viewer.tsx:37` and `:204`, `:219`: `onExit` is never passed.
- `B172` [hidden] `install-docs/CLAUDE.md:31` ("No hidden fallbacks") is contradicted by `src/domain/content/index.ts:4-14`, `src/app/session.ts:24-34` and `ink/hooks/use-install-navigation.ts:26-30`.
- `B174` [hidden] `install-docs/CLAUDE.md:29` ("renders the full docs navigation tree") matches the code, but the domain profile (`src/domain/distro/levitate-profile.ts:4-5`, scopes `["install"]`) and the pane title "Installation Docs" (`ink/screens/install-viewer.tsx:232`, `:501`) both claim install-only scoping.
- `C003` [hidden] `dp/src/presentation/disk-plan-screen.tsx:814,832,816-819` says "`ab` is the default and recommended mode for LevitateOS" and argues for it with LevitateOS's LLM-assisted recipe workflow. AcornOS installs in `mutable`, so the app's advice follows one distro's policy while the tool (`recpart`) is shared.
- `C004` [hidden] `dp/src/presentation/disk-plan-screen.tsx:1154` hard-codes the frame title "LevitateOS Disk Plan". Distro identity is a copied string, not read from the distro being installed.

### F14. Facts owned by recpart and recab reach the TUIs only through `cargo xtask gen`.

Solves 27 problems (25 hidden · 1 surplus · 1 special).

The generator walks type references from each tool's root output types, so there's no hand-kept list. It writes one module per tool into the directory the consuming app's manifest names (`src/generated/<tool>.ts`, never mixed with hand-written code), and runs the shared formatter on its output so `gen --check` and `format:check` agree.

It also emits what the TUIs copy today:
- recpart's default install mode and per-mode summaries, from the clap doc comments.
- The meaning of dry-run `warnings`, documented as "apply refuses".
- Slot letters as one shared type, and field docs such as `rotational`.
- recab's subcommands with their about text and whether they need root.
- recab's `Status` fields with labels.
- Schema versions; recab status gains a `schema_version`.

recpart's handoff document carries the mount root and the next-step commands, so disk-plan stops spelling `recstrap`, `recfstab` and `recchroot`. One generated `parseToolResult(exitCode, stdout, stderr)` per tool states which stream carries JSON and tells not-JSON, wrong-schema and error payloads apart. Unused emitted constants go. This touches `tools/recpart` and `tools/recab` output (small, versioned additions).

**Done when:** `rg "'ab'|set-next|status --json|recstrap" tui/apps` finds only generated files; `cargo xtask gen --check` and format-check pass together; adding a nested recpart type needs no generator edit.

**Depends on:** F1, F2.

- `C005` [hidden] `dp/src/cli/parse-args.ts:18` defaults `mode` to `"ab"`, copying recpart's `default_value_t = InstallMode::Ab` (`tools/recpart/src/cli.rs:40`, REQ-MODE-001). It is repeated in the help text (`parse-args.ts:70` "mode defaults to 'ab'"), in `disk-plan-screen.tsx:814,832` ("default, recommended") and in `dp/README.md:29`. The generated contract carries `INSTALL_MODES` but not the default.
- `C006` [hidden] `dp/src/presentation/disk-plan-screen.tsx:808-853` draws exactly two hard-coded mode cards (`ab`, `mutable`) with their own descriptions, while `toggleMode` (`:464-471`) cycles the generated `INSTALL_MODES`. A third recpart mode would toggle into a state with no card.
- `C007` [hidden] The mode descriptions at `disk-plan-screen.tsx:834-836,849-851` restate recpart's layout semantics (`tools/recpart/README.md:19-21`) in a separate wording.
- `C008` [hidden] `dp/src/app/plan-runner.ts:55` (`json: exitCode === 0 ? stdout : stderr`) relies on recpart writing its error JSON to stderr and its result to stdout (`tools/recpart/src/main.rs:13`). The convention is noted only in a comment (`plan-runner.ts:16`). If recpart changes it, `parseRecpartJson` silently returns null and diagnostics fall back to raw text.
- `C009` [hidden] `dp/src/app/plan-runner.ts:85-86`: a failing `recpart list-disks --json` prints its `ErrorPayload` JSON on stderr, but `listDisks` throws `recpart list-disks failed: ${run.stderr}`, so the target-disk page (`disk-plan-screen.tsx:756`) shows a raw pretty-printed JSON blob. The plan, dry-run and apply failures go through `setFailureFromRun`/`ERROR_SCHEMA_VERSION` (`:209-221`). Two handlings of one recpart error contract.
- `C010` [hidden] `dp/src/presentation/disk-plan-screen.tsx:930` "Apply would refuse this disk" copies recpart's text-mode heading (`tools/recpart/src/cli.rs:148`), and `:91-94`/`:387` treat `warnings` as "refusals". The meaning (dry-run warnings = what apply refuses) is set in `tools/recpart/src/exec.rs:67-77` and is not part of the contract types (`ApplyResult.warnings: Array<string>`).
- `C013` [hidden] `dp/src/presentation/disk-plan-screen.tsx:788` labels every non-rotational disk "ssd", but the contract says `rotational` is "`false` for SSDs and virtual disks" (`dp/src/contract/recpart.ts:8-11`). The UI's meaning of the field drifts from the producer's.
- `C078` [hidden] `rc/presentation/runtime-recab-screen.tsx:113,121,124` build notices as `recab ${action.args[0]}`. That assumes `args[0]` is the subcommand name and hard-codes `recab` while the binary is `recabBin`.
- `C115` [hidden] `xtask/src/tasks/generate.rs:96,102` write into the `tui` submodule (`dp/src/contract/recpart.ts`, `rc/domain/contract.ts`) from the superproject. A contract change means a commit in `tools/recpart|recab`, in `xtask`, and in `tui`, with the generator holding the app paths.
- `C116` [hidden] `xtask/src/tasks/generate.rs:70-85` hand-lists every recpart type to emit, in dependency order. A new nested output type in `tools/recpart/src/types.rs` must also be added here, and the generator does not follow references.
- `C117` [hidden] `xtask/src/tasks/generate.rs:142,169-178` emit TS that oxfmt rejects (`dp/src/contract/recpart.ts` and `rc/domain/contract.ts` fail `format:check`). Formatting them makes `cargo xtask gen --check` (`check.rs:38`) report them as stale. The two checks contradict each other on the same files.
- `C118` [hidden] The generated contract locations differ per app: `dp/src/contract/recpart.ts` vs `rc/domain/contract.ts`. recab's sits inside `domain/`, which also holds hand-written code, so "generated, do not edit" and app code share a folder in one app and not the other.
- `C119` [surplus] `xtask/src/tasks/generate.rs:88` emits `HANDOFF_SCHEMA_VERSION` (`dp/src/contract/recpart.ts:65`). Nothing in disk-plan reads it: the handoff arrives inside the apply document and is never version-checked on its own.
- `C120` [hidden] `xtask/src/tasks/generate.rs:3` says the declarations go "into the TUI that reads that JSON" (singular). It writes to two TUIs.
- `C124` [hidden] `dp/README.md:24` "schema version must match expected value (`schema_version = 1`)". Apply and handoff are schema 2 (`dp/src/contract/recpart.ts:63,65`), and each document is checked against its own generated constant.
- `C133` [hidden] `docs/history/UNTANGLE.md:686` B014 (mode values re-declared) is closed by F21 via `INSTALL_MODES`, but the default and the two-card layout (`dp/src/cli/parse-args.ts:18,70`, `disk-plan-screen.tsx:808-853`) still hard-code `ab`/`mutable`.
- `R012` [special] `tui/apps/install/disk-plan/src/domain/recpart.ts:21-23` swallows the JSON parse error (`// not JSON`) and returns null, so callers can't tell "not JSON" from "wrong schema".

### F15. disk-plan as an explicit flow graph, with IO kept apart from pages.

Solves 40 problems (22 hidden · 3 needless · 9 surplus · 6 special).

The flow is one table of pages, each with `next(state)`, `back(state)` and `canLeave(state)`, branching on the apply outcome (result-handoff or failure-diagnostics). The welcome page and `--help` render the page list and the safety gates from that table. The apply gate is one predicate (plan ok, dry-run ok, no refusals, token typed) that both the page and `startApply` use, with no page-position arithmetic. IO lives in hooks over `runCommand` and `useAsyncTask`, one per recpart call; the dry-run runs once and both pages read the same result. Every executed command, `list-disks` and the real `apply` included, goes into a command log shown on the preview page, built from the argv that actually ran, and the plan page shows recpart's `sfdisk_script`. Each page is its own component. A disk choice is either `{listed: DiskInventory}` or `{unlisted: path}`. The token field is a text scope (F7) compared exactly to `CONFIRM_TOKEN`. A later success clears the stored failure, and page text uses one step-line format.

**Done when:** `disk-plan-screen.tsx` is replaced by a flow module, hooks and page components; F4's walk covers both apply outcomes and Back from a failure; Next on result-handoff doesn't land on failure-diagnostics; the README holds no page or key lists.

**Depends on:** F7, F10, F14.

- `C015` [hidden] `dp/src/presentation/disk-plan-screen.tsx:869` "Running recpart plan --json ..." and `dp/src/cli/parse-args.ts:72` "command runs: recpart plan --disk <disk> --mode <mode> --json" hand-write a command that `buildPlanCommand` (`plan-runner.ts:26-28`) owns. `dp/README.md:12-13` is a third copy.
- `C016` [hidden] `dp/src/presentation/disk-plan-screen.tsx:918-919,969,1043-1045` rebuild the displayed commands from `flow.disk`/`flow.mode` with `buildPlanCommand`/`buildApplyCommand`, separately from the runs that actually executed. `RecpartRunResult.command` (`plan-runner.ts:12`) carries the executed argv but is read only in an error string (`disk-plan-screen.tsx:132`).
- `C017` [hidden] `dp/src/presentation/disk-plan-screen.tsx:917` heading "Exact Command Sequence (No Hidden Commands)" lists the plan and dry-run invocations but not `recpart list-disks --json` (`plan-runner.ts:84`) or the real `apply --confirm DESTROY` (`plan-runner.ts:70`). The UI text contradicts what the app runs.
- `C023` [hidden] `dp/src/domain/flow.ts:3-14` `DISK_PLAN_PAGES` is copied by hand in the welcome page (`disk-plan-screen.tsx:721-731`, "Flow (10 pages)" plus ten literal lines) and in `dp/README.md:9-18`. Adding or reordering a page needs all three.
- `C024` [hidden] The flow is a branch (apply → result-handoff | failure-diagnostics; F → failure → back), but `flow.ts:41-53` models it as a linear list with `nextPage`/`prevPage`. The screen patches it with index jumps (`disk-plan-screen.tsx:491-500,505-511,643-644`). Side effects: Next on `result-handoff` lands on `failure-diagnostics` ("No failure captured."), and Back from an apply failure lands on `result-handoff` ("No successful apply result available.").
- `C025` [hidden] `dp/src/presentation/disk-plan-screen.tsx:409-411,514` derive the apply lock-out from page positions (`applyStarted && flow.pageIndex - 1 <= confirmIndex`). Reordering `DISK_PLAN_PAGES` changes safety behaviour without any type error.
- `C026` [hidden] `dp/src/domain/guards.ts:12-32` maps page → context flag, and the screen maps flag → state predicate (`disk-plan-screen.tsx:476-483` using `isPlanReady` etc. at `:83-98`). `startApply` (`:420-426`) re-checks the same plan/dry-run/preflight predicates on its own. One rule ("may we go past page X") is split across three places.
- `C027` [needless] `dp/src/domain/guards.ts:3-10` `AdvanceContext` is a shallow interface. Every new gate changes the type, the `if` chain in `canAdvancePage` and the object literal in the screen together.
- `C028` [surplus] `dp/src/domain/guards.ts:25-27` the `destructive-confirm` → `confirmUnlocked` guard never lets anything advance, because `onNext` returns on that page anyway (`disk-plan-screen.tsx:487-490`).
- `C029` [special] `dp/src/presentation/disk-plan-screen.tsx:487-501` `onNext` has an `if` per page (`destructive-confirm`, `apply-progress`) on top of the guard table.
- `C030` [needless] `dp/src/domain/flow.ts:1`, `dp/src/app/bootstrap.tsx:2` and `disk-plan-screen.tsx:17` import `DiskMode` from `cli/parse-args`, so domain and presentation depend on the CLI module for a type that is just the generated `InstallMode` (`parse-args.ts:3`).
- `C031` [surplus] `dp/src/cli/parse-args.ts:3` `DiskMode = InstallMode` is an alias with no meaning of its own.
- `C032` [surplus] `dp/src/domain/flow.ts:55-60` `setMode` is a one-field spread with one caller (`disk-plan-screen.tsx:466`).
- `C033` [surplus] `dp/src/domain/flow.ts:24-35` `clampPageIndex` handles non-finite and fractional input that never occurs (indices are integer ±1 or `indexOf` results) and re-implements the kit's `clampIndex` (`tui/kit/core/src/utils/clamp.ts:35-41`).
- `C034` [surplus] `dp/src/domain/flow.ts:38` `?? "welcome"` fallback is unreachable after clamping.
- `C035` [hidden] `dp/README.md:4-18` "Implemented thin shell pages" and `dp/src/cli/parse-args.ts:62` "thin shell" describe a design that the 1176-line screen no longer is.
- `C036` [needless] `dp/src/presentation/disk-plan-screen.tsx:151-1176` mixes in one function: all flow state (`:153-175`), disk listing IO (`:227-275`), three recpart-running effects (`:314-406`), the destructive apply launcher (`:413-450`), navigation rules (`:474-518`), eleven hotkey bindings (`:526-645`), the footer (`:647-695`) and ten page renderers (`:711-1150`). Changing any one page's rendering means editing the file that owns the destructive-apply safety logic.
- `C037` [hidden] `dp/src/presentation/disk-plan-screen.tsx:314-341`, `:343-370` and `:372-406` are three copies of one effect shape: guard on page, set running, run, `runState`, `setFailureFromRun`, a `.catch` with the error-message idiom, and a cancel flag. The kit already has `useAsyncTask` with cancellation (`tui/kit/core/src/hooks/use-async-task.ts:13-75`), unused here.
- `C038` [hidden] `dp/src/presentation/disk-plan-screen.tsx:343-370` (command-preview) and `:372-406` (preflight-checks) run the identical `recpart apply --dry-run` and both show its `warnings` (`:926-936` and `:971-975`). They gate differently, though: `commandPreviewReady` ignores warnings, `preflightPassing` requires none. Two pages over one result, each with its own reading of it.
- `C039` [hidden] `dp/src/presentation/disk-plan-screen.tsx:59` (`AsyncState.failure.raw`) and `:211` restate `RecpartRunResult`'s fields (`plan-runner.ts:11-18`), and `dp/src/presentation/page-text.ts:13-18` restates `FailureDiagnostic` (`disk-plan-screen.tsx:63-68`) structurally. Three hand-kept shapes for one record.
- `C040` [surplus] `dp/src/presentation/disk-plan-screen.tsx:59,211-213` type `json` as `string | null` and test `run.json ? … : null`, but `RecpartRunResult.json` is always a string.
- `C041` [surplus] `dp/src/presentation/disk-plan-screen.tsx:83-89` `isPlanReady`/`isDryRunReady` only wrap `kind === "success"`.
- `C042` [hidden] Step-line formats are written three ways: `[phase] command` (`disk-plan-screen.tsx:923`), `[pending] phase :: command` (`:1053`) and `progressStepLine`'s `[PLANNED|OK|FAIL] phase :: command` (`page-text.ts:8-11`). "Not yet run" is spelled both `pending` and `PLANNED`.
- `C043` [hidden] `dp/src/presentation/page-text.ts:19-33` returns lines whose first entry is the title by convention, and `disk-plan-screen.tsx:1141-1146` splits `lines[0]` off as a red heading. Callers must know the position rule.
- `C044` [surplus] `dp/src/presentation/page-text.ts:3-34` holds three single-caller string helpers (`confirmStatusLine`, `progressStepLine`, `failureDiagnosticLines`) split from the screen. The split has no second caller and they change only with the screen; it is left over from when unit tests imported them.
- `C047` [hidden] `flow.disk` has three writers: the listing effect (`disk-plan-screen.tsx:257-259,266-270`), the grid-sync effect (`:282-291`) and `selectDiskAtIndex` (`:197-207`, called on Enter at `:624-626`). The comment at `:223-224` explains the ordering, but nothing enforces it.
- `C048` [special] `dp/src/domain/recpart.ts:47-62` `unlistedDisk` fabricates a `DiskInventory` with `size_bytes: 0` and `"unknown"` strings for an explicit `--disk` recpart didn't list (`disk-plan-screen.tsx:248-251`). This is B126, kept.
- `C049` [hidden] The screen recognises that stand-in by the `size_bytes > 0` sentinel (`disk-plan-screen.tsx:781` "(not listed by recpart)", `:1018`). `unlistedDisk`'s zero and the screen's check must change together.
- `C051` [special] `dp/src/presentation/disk-plan-screen.tsx:225-237` uses `disksListedRef` as a run-once latch on an effect that depends on `[setDiskIndex]`, plus `initialDiskRef` to freeze the prop. This works around the effect's dependency list instead of following from it.
- `C053` [hidden] `dp/src/presentation/disk-plan-screen.tsx:799-801` shows "Missing --disk value." whenever no disk is selected, but `--disk` is optional (`parse-args.ts:71`). With an empty or failed listing, the message names the wrong cause.
- `C058` [special] `dp/src/presentation/disk-plan-screen.tsx:308-311` accepts only `[a-zA-Z]`, upper-cases, and truncates to the token length (B128, partly kept). This silently assumes the token is upper-case letters.
- `C063` [special] `dp/src/presentation/disk-plan-screen.tsx:387-394` turns dry-run warnings into a synthetic `FailureDiagnostic` with a hard-coded remediation ("Resolve what recpart reports, then re-enter this page."). `:218-219` has a second hard-coded fallback remediation.
- `C064` [hidden] `dp/src/presentation/disk-plan-screen.tsx:167,209-221`: `failure` holds only the last failure and is never cleared when a later run succeeds, so `F` and the footer `fail` segment keep showing a fixed problem.
- `C065` [surplus] `dp/src/presentation/disk-plan-screen.tsx:414,417`: `startApply` re-checks `page !== "destructive-confirm"` (its only caller, `useInput`, already returned for other pages) and checks both `applyRunRef.current` and `applyStarted` for the same fact.
- `C066` [special] `dp/src/presentation/disk-plan-screen.tsx:1096` "Waiting to start apply..." is the `idle` branch of a page reachable only after apply starts.
- `C125` [hidden] `dp/README.md:12-13` describe plan-preview as "runs `recpart plan --json`" and command-preview as "displays exact emitted step commands". Preflight also re-runs the dry-run, and `list-disks` runs on mount, so neither is listed as a command the app runs.
- `C126` [hidden] `tools/recpart/docs/frontend-handoff/README.md:20` says the plan flow renders "partition summary + script preview". disk-plan never shows `sfdisk_script` (`disk-plan-screen.tsx:867-887`).
- `C129` [hidden] `dp/src/presentation/disk-plan-screen.tsx:472-473` comment "there are no unguarded page jumps", while `:493,496-497,509,644` and `:433` set `pageIndex` directly (the F jump, the apply-result branch and the startApply jump) outside `canAdvancePage`.
- `C130` [hidden] `dp/src/presentation/disk-plan-screen.tsx:734-737` welcome "Safety Controls" claims ("Preflight must pass before destructive apply") restate the gates in `startApply`/guards as prose that does not follow them if they change.
- `C134` [hidden] `docs/history/UNTANGLE.md:689` B018 (command lines rendered by hand) is closed by F21, but `disk-plan-screen.tsx:869`, `parse-args.ts:72` and `dp/README.md:12-13` still spell recpart command lines by hand.

### F16. recab TUI: results stay visible, and status is a separate panel.

Solves 7 problems (2 hidden · 1 needless · 1 surplus · 3 special).

Running an action appends to a result log: the command (from the generated subcommand list), its exit code and its output. The status panel refreshes separately and never overwrites the last result, and status refresh is an ordinary action. Parse and exit-code handling live in one place (the generated parser). The runner doesn't import the UI action table, and effects depend only on inputs that change.

**Done when:** After `commit`, the screen shows commit's result next to the refreshed status; there's no `action.id ===` special case and no literal `status-error` segment.

**Depends on:** F10, F14.

- `C079` [hidden] `rc/presentation/runtime-recab-screen.tsx:115-122` and `:83`: after a successful mutating action, `refreshStatus` overwrites `lastResult` and `notice`. "Last Command" then shows `status --json` instead of the `commit`/`set-next`/`rollback` that just ran, so the result of the mutation is never shown.
- `C080` [special] `rc/presentation/runtime-recab-screen.tsx:108-111` `if (action.id === "status-refresh")` routes one table entry around the generic action path.
- `C081` [hidden] `rc/app/recab-runner.ts:57-63` returns `parseError: null` on non-zero exit, and the screen re-checks `exitCode` (`runtime-recab-screen.tsx:85-90`) before `parseError` (`:91-96`). The `?? "status JSON parse failed"` fallback at `:92` is unreachable. The branching is split between runner and screen.
- `C082` [needless] `rc/app/recab-runner.ts:3` imports `STATUS_ARGS` from `flows` (the UI action table), so the IO layer depends on the presentation-level action list.
- `C083` [special] `rc/presentation/runtime-recab-screen.tsx:137-139` effect depends on `[recabBin]`, a prop that never changes after mount.
- `C084` [surplus] `rc/domain/recab.ts:26-28` `formatSlot` wraps `?? "(none)"`.
- `R013` [special] `tui/apps/runtime/recab/presentation/runtime-recab-screen.tsx:191` pushes the literal text "status-error" as the segment body, a one-off segment for one state.

### F17. Guest scripts are files, with one placeholder scheme that is checked.

Solves 4 problems (3 hidden · 1 needless).

Every script the builder writes into the guest (entrypoint, launcher, profile scripts, autologin, live-net-setup, volatile-log, …) lives as a real file under `distro/builder/guest/`, pulled in with `include_str!`, and uses only `@NAME@` placeholders. One `render_guest_script(template, &[(name, value)])` fails the build if a placeholder is unknown or left unreplaced. It lives in a neutral module that both pipeline and artifact code use, so neither reaches into the other. `cargo xtask check` runs `sh -n` (and shellcheck when it's installed) on the files, plus a syntax check of the embedded awk programs, which catches the live-net-setup bug.

**Done when:** No `format!` builds a shell script; `rg '\{\{' distro/builder/src` finds no shell text; live-net-setup's awk parses under gawk and busybox awk.

**Depends on:** nothing.

- `D038` [hidden] `live_tools.rs:461-491` and `:513-575` build scripts with `format!` and `{{`/`}}` escaping, while `:585-620` uses a plain string with `@PLACEHOLDER@` substitution through `fill_guest_script`. That's two templating schemes in one function, and every edit has to know which escaping applies. None of these scripts can be run or shellchecked without building a rootfs.
- `D039` [hidden] `live_overlay.rs:512-513`: the `live-net-setup` awk programs sit in a raw string `r#"…"#` but use escapes meant for a normal string (`\\.`, `\"dev\"`). The shell therefore gets `/^10\\.0\\.2\\.15\\//` and `$i==\"dev\"`, and gawk rejects both (verified: "syntax error" and "backslash not last character on line"). `stable_ok` never increments, the unit loops 45 × `sleep 1` and exits 1, and it is ordered `Before=sshd.service`.
- `D040` [needless] `live_tools.rs:621` (pipeline) calls `crate::artifact::live_overlay::fill_guest_script` just to substitute placeholders. Meanwhile `live_overlay.rs:703-717` (artifact layer) imports the install-UX constant `crate::live_ux::INSTALL_ENTRYPOINT`. Each module reaches into the other.
- `D041` [hidden] `live_overlay.rs:703-717` `fill_guest_script` runs all three substitutions on every script. A misspelt placeholder in any caller survives into the guest as literal text, and nothing checks that all placeholders were replaced.

### F18. The install experience is declared once, in the variant's products.toml.

Solves 13 problems (5 hidden · 4 needless · 4 surplus).

`products.toml` holds one `install_ux` block per variant. It lists the consoles that launch the UX (and their autologin), the interactive shell, the docs frontend with its title and default page, and the split helper. The frontend is either `tui_app = "install-docs"` or `text = "install-guide.txt"`, resolved by the variant loader relative to the variant directory. The contract validates the block once, in `variant.rs`, with `deny_unknown_fields` on every action struct; `config.rs` only maps it. The builder stops reading `install_experience` from `scenarios.toml`. Gone: `AutomatedSsh`, `issue_message`, `ux_docs_frontend` under automated mode, and the install-experience marker file (tests read the variant). Builder code imports contract types from the contract crate, not through `live_tools`. Another session is editing `distro/contract` and `products.rs`, so land this in small commits and commit only these hunks.

**Done when:** No build input comes from `scenarios.toml`; each field has one validator; an unknown key in any runtime action fails variant loading.

**Depends on:** nothing.

- `D047` [surplus] `schema.rs:16-29` `InstallExperience::AutomatedSsh`, `live_tools.rs:566-575` (its entrypoint), `config.rs:161-163`, `schema.rs:262` `automated_ssh_actions` and `variant.rs:1115-1121,1697` `runtime_profiles_automated_ssh`: neither variant selects `automated_ssh` (`levitate/scenarios/scenarios.toml:79`, `acorn/scenarios/scenarios.toml:72` are both `"ux"`) and neither declares `runtime_profiles_automated_ssh`. The whole branch is speculative.
- `D048` [hidden] The UX/automated split happens twice. Profile groups pick actions (`config.rs:158-164`, `runtime_profiles_ux` → the split helper), but `install_mode_payload` sits in `live_tools_common` (`acorn/ring2/products.toml:125-129`, `levitate/ring2/products.toml:186-189`) and branches on `install_experience` in code (`live_tools.rs:433,512,583`). The docs payload is "common" yet only written for UX, while the split helper is "ux".
- `D049` [needless] `config.rs:110` and `products.rs:133,478`: the builder takes `install_experience` from `contract.scenarios.live_tools` (the test-scenario file `scenarios.toml`) and uses it to shape the shipped product. Build output depends on test-scenario config, while the payload it selects lives in `products.toml`.
- `D050` [needless] `live_tools.rs:42,105`: `add_required_tools` takes `install_experience` only to pass it on to the one action that uses it, so every other action's signature carries it as well.
- `D051` [needless] `schema.rs:251-254` `InstallModePayload` requires `ux_docs_frontend` even under `AutomatedSsh`, where `live_tools.rs:433` ignores it.
- `D052` [hidden] Each field is validated twice. `variant.rs:1233-1237` (`normalize_absolute_string`) and `config.rs:264-280` both check that `interactive_shell` is non-empty, absolute and has no `..`. `variant.rs` `normalize_relative_string` and `config.rs:191-212` both check `destination`. `ux_docs_guide` is checked only in `variant.rs:1241`, and `live_tools.rs:447` trusts it. Tighten one copy and the other doesn't follow.
- `D053` [needless] `schema.rs:227-232` `InstallDocsFrontend::PlainText { guide }` is a repo-relative path resolved at build time (`live_tools.rs:447`), so the contract type depends on the repo checkout layout.
- `D054` [hidden] `variant.rs:1743-1765` `VariantRing2RuntimeAction` lacks `#[serde(deny_unknown_fields)]`, unlike its sibling structs (`variant.rs:1548…1739`). A misspelt `ux_docs_guide` on a `bun_bundle` action, or any unknown key on another action, is silently dropped.
- `D055` [surplus] `schema.rs:191`, `variant.rs:971,1774`, `config.rs:366-368` and `overlay.rs:39-41`: `issue_message` is configurable but neither variant sets it. OpenRC ignores it entirely (`config.rs:369-385` drops it and `overlay.rs:58` always passes the banner).
- `D056` [hidden] `issue_message` (`overlay.rs:39-41`) can replace `/etc/issue`, but `testing/install-tests/src/qemu/serial/mod.rs:71` waits for `LIVE_LOGIN_HINT`, which only appears in the default banner. Setting the config option breaks live-tools readiness detection.
- `D096` [surplus] `live_tools.rs:419-425` + `live_ux.rs:4-5`: the install-experience marker file exists only so `testing/install-tests/src/scenarios/mod.rs:426-438` can compare it with `scenarios.toml`, the same source the builder wrote it from. The check can only catch a stale ISO.
- `D120` [hidden] `schema.rs:228-230` says `PlainText` is "shown in a pager", but the launcher also falls back to `cat` (`live_tools.rs:481-482`) and then execs a shell (`:488`).
- `R004` [surplus] `distro/builder/src/pipeline/live_tools.rs:8`: `pub(crate) use distro_contract::{InstallDocsFrontend, InstallExperience}` re-exports contract types so `distro/builder/src/pipeline/products.rs:13` can import `InstallExperience` from `live_tools`; a pass-through.

### F19. One owner for the chain from login to install screen.

Solves 43 problems (29 hidden · 2 needless · 6 surplus · 6 special).

A builder module, `install_ux`, absorbs `live_ux.rs`, the install parts of `live_tools.rs` and the UX parts of `live_overlay.rs` (the two files that change together most, 13 shared commits). It turns the F18 block into the guest files. On both init systems, the getty on each UX console autologins straight into one launcher, `install-ux`. There is no profile.d hook, no `LEVITATE_INSTALL_*` env protocol and no `LAUNCHED` guard, and boot-injection keys are no longer exported into every login shell.

The launcher runs the split pane with exec arrays built from the variant: `install-split --left <shell> -l --right install-docs --page …`, with the shell from `interactive_shell` and the docs command and argv from the frontend. When the split pane exits for any reason, the launcher `exec`s the login shell, so a failure never ends the session or loops through getty.

Both docs frontends share one argv contract (`--page`) and one lifetime: they exit, and the split pane shows "(exited) Enter to restart". The text frontend sets its own pager flags and wraps to the pane width, and `install-guide.txt` refers to the split pane's footer rather than restating keys.

Command names and paths are constants in this module and distro-neutral (`install-ux`, `install-split`, `install-docs`, `/usr/share/install-ux/`). User-facing text uses `os_name`. The serial UX is a console listed in the variant, turned on for a boot by one named boot-injection key that xtask sets from the same constant (the dead `SERIAL_VERBOSE` goes). The tmux `levitate-install-docs-split`, the `docs-tui` alias, the TMUX guard and the stale migration-doc entry go.

**Done when:** One module writes every install-UX guest file; `rg LEVITATE_INSTALL_ distro xtask testing` finds only the boot-injection key constant; on both distros tty1 autologins into the install screen, and the serial console does when the key is set; killing the split pane leaves a working root shell.

**Depends on:** F17, F18.

- `B130` [hidden] `distro/builder/src/pipeline/live_tools.rs:612` exports `LEVITATE_DOCS_PLAIN=1` for the serial console. Only the AcornOS plain-text launcher reads it (`live_tools.rs:478`). The Bun app ignores it, so on LevitateOS the serial console still gets the full-colour TUI, and nothing indicates the variable is a no-op there.
- `B131` [hidden] `distro/builder/src/pipeline/live_tools.rs:400` (`exec bun "$APP" "$@"`) and `live_tools.rs:502` (`docs-tui` → `levitate-install-docs "$@"`) forward argv into `src/cli/parse-args.ts`. The plain-text launcher (`live_tools.rs:461-488`) silently ignores the same `--slug installation` argv. One command name now has two argv contracts.
- `B170` [hidden] `install-docs/README.md:12` and `install-docs/bin/levitate-install-docs-split:1-52` describe a tmux split launcher named `levitate-install-docs-split`. The shipped split pane is the Rust binary of the same name (`distro/builder/install-split-pane/Cargo.toml:6-7`, `distro/variants/levitate/ring2/products.toml:195-196`). This is the same name with two implementations, and the tmux one hard-codes the dev entry `bun src/main.ts` (`bin/levitate-install-docs-split:24`, `:45`).
- `D001` [hidden] `builder/src/pipeline/live_tools.rs:605-617` (profile hook `30-install-ux.sh`) and `builder/src/artifact/live_overlay.rs:107-111` (OpenRC `serial-autologin`) both decide "launch the UX on the serial console when `LEVITATE_INSTALL_SERIAL_UX=1`", each with its own condition, and nothing ties the two together.
- `D002` [hidden] `builder/src/artifact/live_overlay.rs:442-446` is a third copy of the serial launch block (systemd `serial-autologin`), identical to `:107-111`, with nothing linking them.
- `D003` [surplus] `builder/src/artifact/live_overlay.rs:412-453`: the systemd `serial-autologin` script is written but nothing runs it. The serial getty drop-in at `:455` uses `agetty --autologin root` with no `-l`/`--login-program`, and `grep serial-autologin` finds no other user on the systemd side. Its UX launch, `LEVITATE_INSTALL_SERIAL_VERBOSE` printk handling, `LANG=C` and ready marker are all dead.
- `D004` [hidden] On LevitateOS the serial UX actually goes through the profile hook's ttyS0 branch (`live_tools.rs:607-613`). That branch sets `LEVITATE_INSTALL_RIGHT_CMD="docs-tui --slug installation"` and `LEVITATE_DOCS_PLAIN=1`. On AcornOS the serial UX goes through `live_overlay.rs:107-111`, which sets neither variable. So the same flag gives each distro a different serial UX (different right-pane command, different env), and the code doesn't show the difference.
- `D005` [surplus] `builder/src/pipeline/live_tools.rs:607-613`: on AcornOS this ttyS0 branch can't be reached. With `LEVITATE_INSTALL_SERIAL_UX=1`, the OpenRC autologin execs the entrypoint first and sets `LEVITATE_INSTALL_UX_LAUNCHED=1`, which makes the hook return at `:592`.
- `D006` [special] `builder/src/pipeline/live_tools.rs:605-606`: the hook hard-codes `/dev/tty1` as the only VT that launches the UX. On AcornOS, tty1 has no autologin (`live_overlay.rs:156` plain `getty 38400 tty1`), so the UX starts only after a manual root login. LevitateOS autologins tty1 (`live_overlay.rs:405-410`). That distro difference comes from init-specific code paths, not from config.
- `D007` [hidden] `xtask/src/tasks/scenarios.rs:157-163` decides from CLI mode (`!ssh && !no_shell && no window`) that "the install UX runs on" serial, and injects the literal `LEVITATE_INSTALL_SERIAL_UX=1`. The guest-side readers (`live_tools.rs:607`, `live_overlay.rs:107`, `live_overlay.rs:442`) repeat the literal, and there is no shared constant (`live_ux.rs` has none for it).
- `D009` [surplus] `xtask/src/tasks/scenarios.rs:160-162` injects `LEVITATE_INSTALL_SERIAL_VERBOSE=1` for `--verbose`. Its only reader is the dead systemd serial-autologin (`live_overlay.rs:426`), and the OpenRC script never reads it, so `--verbose` has no effect on either distro.
- `D010` [hidden] `builder/src/pipeline/live_tools.rs:620` and `builder/src/artifact/live_overlay.rs:110,445` `exec` the entrypoint in place of the login shell, and the entrypoint `exec`s the helper (`live_tools.rs:555`). If the helper exits or fails (missing bun, raw-mode error, a write to an exited pane), the login session ends and getty/inittab respawns autologin, which runs the UX again. No fallback shell exists, and the respawn loop is only implied by how the pieces fit together.
- `D011` [hidden] `builder/src/pipeline/live_tools.rs:585-620` relies on profile.d's lexical order. `00-serial-test.sh` (`live_overlay.rs:691-698`, recqemu `guest/serial-test.sh:10-15`) runs first on ttyS0: it turns echo off, empties PS1 and prints the shell-ready marker. Then `25-shell-color.sh` rewrites TERM. Only after that does `30-install-ux.sh` replace the shell with the UX. So the serial readiness marker announces a shell that never exists, and the numeric file names are the only thing that shows the order.
- `D012` [hidden] `LEVITATE_INSTALL_UX_LAUNCHED` is set at `live_tools.rs:619`, `live_overlay.rs:109` and `live_overlay.rs:444`, and read at `live_tools.rs:592`. It's a string literal at every site, with no constant and no doc.
- `D013` [hidden] The profile hook writes `LEVITATE_INSTALL_LEFT_CMD` at `live_tools.rs:597-604`, and `install-split-pane/src/main.rs:104` reads it. The writer and reader are in different crates, and both repeat the variable name as a literal.
- `D014` [hidden] `LEVITATE_INSTALL_RIGHT_CMD` is written at `live_tools.rs:608-611` and read at `install-split-pane/src/main.rs:114`. Its value, `docs-tui --slug installation`, is a shell string that has to be a valid command line for whichever docs frontend the variant ships. Under PlainText the launcher ignores `--slug`, and nothing says so.
- `D015` [surplus] `LEVITATE_DOCS_PLAIN`: set at `live_tools.rs:612` (reachable only on LevitateOS, see above) and read only at `live_tools.rs:478`, the AcornOS plain-text launcher. The LevitateOS Bun docs app never reads it (no `process.env` use of it in `tui/apps/live-tools/install-docs/src`), so where it's set nothing reads it, and where it's read nothing sets it.
- `D016` [hidden] `builder/src/live_ux.rs:19` gives only `SMOKE_ENV` a constant. The other six env vars of the same protocol (`…_UX_LAUNCHED`, `…_LEFT_CMD`, `…_RIGHT_CMD`, `…_SERIAL_UX`, `…_SERIAL_VERBOSE`, `LEVITATE_DOCS_PLAIN`) are literals scattered across builder, overlay, split-pane and xtask.
- `D017` [hidden] `builder/src/pipeline/live_tools.rs:594-604`: the boot-injection import and the `LEVITATE_INSTALL_LEFT_CMD` export run before the tty check at `:605`. Every interactive login on any tty, SSH included, therefore gets every boot-injection key exported (e.g. `SSH_AUTHORIZED_KEY`), plus `LEFT_CMD`, `TTY`, `line`, `key` and `value` leaked into its environment. A file that should only start the UX changes every login shell.
- `D018` [needless] `LEVITATE_INSTALL_LEFT_CMD` and `LEVITATE_INSTALL_RIGHT_CMD` (`main.rs:536-537`, `main.rs:572`) are untyped shell strings concatenated into `/bin/sh -lc "exec …"`. Profile-hook callers have to know the split pane's quoting rules and that it prefixes `exec`.
- `D019` [hidden] Five places pick the interactive shell independently: `interactive_shell` in `products.toml` (`acorn:127` `/bin/sh`, `levitate:188` `/bin/bash`); the profile hook's own `/bin/bash`-else-`/bin/sh` probe (`live_tools.rs:597-603`), which ignores `interactive_shell`; `main.rs:143-159` `resolved_shell_path` (`$SHELL`, then bash, then sh); the OpenRC autologin `exec /bin/sh -l` (`live_overlay.rs:115`, `overlay.rs:283`); and the systemd autologin `exec /bin/bash -il` (`live_overlay.rs:448`). They agree only by accident. On AcornOS, if bash is present, the left pane runs bash although the variant declares `/bin/sh`.
- `D020` [surplus] `interactive_shell` (`schema.rs:252`, `variant.rs:1233`, `config.rs:261-280`) is only the fallback inside `${SHELL:-…}` at `live_tools.rs:488,559,572`. A login always sets `SHELL`, so the configured value almost never takes effect.
- `D021` [hidden] `install-split-pane/src/main.rs:535-540`: real UX launches always pass `LEFT_CMD` (`live_tools.rs:597-604`), so `Config.shell` and `resolved_shell_path` matter only for `--smoke` (run over SSH without the profile hook) and for a log line. The smoke test exercises a different left-pane command than real users get.
- `D022` [needless] `install-split-pane/src/main.rs:542-545`: every pane is wrapped in `/bin/sh -lc`, which sources `/etc/profile` once more as a non-interactive login. The left pane then runs `/bin/bash -il`, which sources it a third time. The split pane's wrapper is tied to how the profile.d scripts guard themselves (`*i*`, `LEVITATE_INSTALL_UX_LAUNCHED`).
- `D023` [hidden] `levitate-install-docs-split` is spelled out independently in `install-split-pane/Cargo.toml:7`, `main.rs:76-77`, `recipes/install-split-pane.rhai:14,22,26,35,45,57,79`, `recipe/tools.rs:195-196`, `variants/levitate/ring2/products.toml:195-196`, `variants/acorn/ring2/products.toml:135,137` and `live_ux.rs:16`, plus a tmux script of the same name in `tui/apps/live-tools/install-docs/bin/levitate-install-docs-split`. Only `live_ux.rs` is shared, by the entrypoint and the tests.
- `D024` [hidden] `variants/*/ring2/products.toml` `destination = "usr/local/bin/levitate-install-docs-split"` (levitate:196, acorn:137) has to equal the `/usr/local/bin/{split}` the entrypoint hard-codes at `live_tools.rs:518-519`. Change the TOML destination and the entrypoint silently falls back to `command -v`, or to docs only.
- `D025` [hidden] `live_ux.rs:12` defines `INSTALL_DOCS`, yet the command name `levitate-install-docs` is retyped as a literal at `live_tools.rs:480` and `:526`, `main.rs:124`, `:163` and `:573`, and in the tui `package.json:7` bin name.
- `D026` [hidden] The Bun bundle path `usr/local/share/levitate/docs-tui/levitate-install-docs.js` is built at `live_tools.rs:319,326` and retyped inside the launcher script at `:383`. The bun path `usr/local/bin/bun` is likewise copied at `:361` and `:400`. None of them are in `live_ux.rs`.
- `D027` [hidden] The `docs-tui` alias path `usr/local/bin/docs-tui` (`live_tools.rs:435`) is a literal, and its users (`live_tools.rs:609`, `live_overlay.rs:368` live-help text) depend on that name with no shared constant.
- `D030` [special] `live_ux.rs:5,7,12,14`, `live_tools.rs:319,383` and `live_tools.rs:480` ship LevitateOS-branded paths and commands on AcornOS: `/usr/lib/levitate/install-experience`, `levitate-install-entrypoint`, `levitate-install-docs`, `/usr/local/share/levitate/…`, `LEVITATE_*` env, and the message "Run 'levitate-install-docs' to reopen it".
- `D036` [hidden] `live_tools.rs:654` exports `LESS=-FRSX` from the colour profile. The value flows through the split-pane host into the AcornOS plain-text docs pager (`live_tools.rs:478-479`): `-F` quits at once when the guide fits and `-S` chops long lines. A colour-defaults file changes how the install guide behaves, and the result depends on whether busybox `less` honours `LESS`.
- `D042` [special] `live_tools.rs:517-531` `choose_install_helper` tries an absolute path, then `command -v` for the split helper, then falls back to `levitate-install-docs` alone. The last rung gives a docs-only UX that the install test (`testing/install-tests/src/scenarios/mod.rs:954`) rejects as a failure.
- `D044` [special] `live_tools.rs:558-559` falls back to a login shell when no helper exists. `main.rs:573` (right-pane fallback) and `live_tools.rs:488` (plain launcher after `less`) each `exec` their own login shell too, so there are three separately written "drop to shell" fallbacks.
- `D045` [hidden] `live_tools.rs:488`: the plain-text docs launcher ends with `exec "${SHELL:-…}" -l`, so `levitate-install-docs` acts as a pane host rather than a docs command. If the user runs it from the shell, as its own message at `:480` suggests, every reopen nests another login shell. The Bun launcher (`:400`) just exits, which leaves an "(exited)" pane. The two frontends behind the same command name have different lifetimes.
- `D046` [special] `live_tools.rs:543` prints `[{distro}]` using `distro_id` (e.g. `acorn`) where every other user-facing string uses `os_name`.
- `D058` [hidden] `live_overlay.rs:107,442` (overlay layer) guard on `[ -x /usr/local/bin/levitate-install-entrypoint ]`, a file written into the rootfs layer by `live_tools.rs:511-582`. The overlay script depends on another module having written that rootfs file.
- `D065` [special] `live_tools.rs:591` `[ -n "${TMUX:-}" ] && return 0` is a special case left over from the tmux-based split launcher. Nothing on the live ISO ships tmux.
- `D114` [hidden] `live_overlay.rs:419-420` says the systemd serial autologin handles `LEVITATE_INSTALL_SERIAL_UX`, but nothing invokes that script (see `:455`).
- `D115` [hidden] `live_overlay.rs:93` ("Autologin for serial console testing") and `:163-164` ("Serial console with autologin for test harness"): the same script is also the interactive serial install-UX launcher.
- `D118` [hidden] `live_tools.rs:501` calls the `docs-tui` alias "Canonical docs-tui command alias for split-pane right-side launchers", but the split pane's default right command is `levitate-install-docs` (`main.rs:124`). `docs-tui` is only used by the LevitateOS serial branch (`live_tools.rs:609`) and the live-help text.
- `D121` [hidden] `variants/acorn/ring2/install-guide.txt:4-6` ("Left pane: root shell. Right pane: this guide. Ctrl-g switches panes … q closes it") assumes the split pane. The same text also shows up full-screen (entrypoint fallback `live_tools.rs:526-528`, or reopened from the shell), where it's wrong, and after `q` the "closed" pane becomes a shell (`live_tools.rs:488`).
- `D122` [hidden] `variants/acorn/ring2/install-guide.txt:35-50`: lines are up to 52 columns wide, but the right pane on an 80-column console has 38 columns (`main.rs:31-32` 50% minus borders at `:716`). With `LESS=-S` (`live_tools.rs:654`) the install commands get cut off. The guide's wrap width depends on the pane split, and nothing records that.
- `D123` [surplus] `distro/builder/docs/03_MIGRATION_STAGELESS.md:690-703` lists owner files that no longer exist (`testing/install-tests/src/stages/mod.rs`, `xtask/src/tasks/testing/scenarios.rs`) and markers (`stage02-entrypoint-helper=*`) for an already-ticked item.

### F20. The program that owns the terminal sets TERM, and nothing forces colour.

Solves 6 problems (5 hidden · 1 special).

The split pane is the terminal for its panes. It sets `TERM` to what its vt100 emulator implements and leaves `COLORTERM`, `FORCE_COLOR` and `NO_COLOR` as the user had them; ratatui downsamples to the real console when it draws. Launchers, the shell colour profile and `xtask tui run` don't touch these variables. xtask's capture host sets `TERM` once, as the terminal it emulates. With F6 in place, the kit produces correct colour for whatever level chalk detects.

**Done when:** `rg 'FORCE_COLOR|COLORTERM|NO_COLOR' distro xtask` finds no writes; `NO_COLOR=1` on the console gives a colourless docs pane; the docs pane is coloured on a plain `linux` console.

**Depends on:** F6, F3, F19.

- `X003` [hidden] "Force TERM to xterm-256color if empty/dumb/vt100/vt102/linux, COLORTERM=truecolor, FORCE_COLOR=3, clear NO_COLOR" is written five times: `distro/builder/install-split-pane/src/main.rs:550-567` (`apply_common_env`), `xtask/src/util/pty_capture.rs:114-131` (`apply_terminal_env`, same TERM list), `distro/builder/src/pipeline/live_tools.rs:~388-398` (Bun docs launcher shell), `live_tools.rs:~465-475` (plain-text docs launcher shell), `xtask/src/tasks/tui.rs:20` (`env_remove("NO_COLOR").env("FORCE_COLOR","3")`). They already differ (split pane sets `NO_COLOR=""`, xtask removes it; tui.rs doesn't touch TERM/COLORTERM).
- `X004` [special] The same code overrides `TERM=linux` (the real tty1 console) to `xterm-256color` + truecolor for the docs app, so the app is told it has a terminal it doesn't have; the split pane then re-renders the inner screen to the real console. The capability the app should render for is decided by three layers, none of which is the terminal.
- `A016` [hidden] `xtask/src/tasks/tui.rs:20` forces `FORCE_COLOR=3` and removes `NO_COLOR` before running install-docs. This hides the kit's ansi256 colour bug (see the colour runtime section) in development runs. The xtask has to know the kit's env contract.
- `B128` [hidden] The app reads no environment variables itself (`src/cli/run.ts:6` reads only argv). Colour depends on `TERM`/`COLORTERM`/`FORCE_COLOR`/`NO_COLOR` as interpreted by `tui/kit/core/src/theme/colors.ts:230-265`. That contract appears only in the launchers that force it: `distro/builder/src/pipeline/live_tools.rs:389-398`, `live_tools.rs:466-475`, `distro/builder/install-split-pane/src/main.rs:550-568`, `xtask/src/util/pty_capture.rs:114-131` and `xtask/src/tasks/tui.rs:318`. That is five copies with three different NO_COLOR treatments: unset if non-empty, set to `""`, and `env_remove`.
- `D033` [hidden] The TERM normalisation rule (`""|dumb|vt100|vt102|linux` → `xterm-256color`) is copied four times: `live_tools.rs:389-393` (Bun launcher), `live_tools.rs:466-470` (plain launcher), `live_tools.rs:643-650` (25-shell-color, ttyS*/ttyN only) and `main.rs:551-561` (`apply_common_env`). Nothing says they must agree.
- `D034` [hidden] NO_COLOR gets three incompatible meanings. `main.rs:567` sets it to `""` (still set); the launchers at `live_tools.rs:396-398,473-475` `unset` it; and `25-shell-color.sh` at `live_tools.rs:641` treats only `NO_COLOR=1` as opt-out. The split pane and the scripts each override the user's choice in their own way.

### F21. Each live-ISO file has one writer, and init and distro differences come from config.

Solves 21 problems (12 hidden · 1 needless · 3 surplus · 5 special).

One `InitSystem` interface (OpenRC, systemd) writes getty and autologin, the ready marker, `/etc/issue` and `/etc/shadow` from the same inputs. The rootfs and the overlay no longer both write `serial-autologin`, `inittab`, `/etc/issue` or `/etc/shadow`. The overlay's seed `shadow` is deleted, and `lastchg` takes the systemd value everywhere. tty1 has one getty drop-in. The shell UX defaults (prompt, PAGER/LESS, history) and `live-help` are written once by the overlay for both distros, branded with `os_name`, and the banner reads "<os> Live Tools". Knobs that never vary go (`masked_units`, `write_serial_test_profile`, `enforce_utf8_locale_profile`, `default_issue`), as do the crate's public re-exports of overlay internals. machine-id is left uninitialised so each boot gets its own, and the tmpfs `/var/log` is declared once. The module docs list what each writer produces.

**Done when:** Each guest path above has exactly one writer in `rg`; AcornOS has live-help; the overlay config struct has no field that is constant across callers.

**Depends on:** F17, F19.

- `D031` [special] `live_overlay.rs:360-378` (live-help text "LevitateOS Live Quick Help") and `:535` ("Levitate live network setup") hard-code the brand in the systemd overlay, though `os_name` is a config parameter. "systemd" is treated as meaning "LevitateOS".
- `D035` [hidden] `live_tools.rs:652-655` (`25-shell-color.sh`) and `live_overlay.rs:386-389` (`20-live-shell-ux.sh`, systemd only) both export `PAGER` and `LESS`. Both also set PS1: `live_overlay.rs:391-393` sets it if empty, then `live_tools.rs:685` overwrites it for bash, so the systemd one is dead for bash.
- `D037` [needless] `live_tools.rs:426-431`: `install_shell_color_profile` (generic shell colours, PS1, bash history and completion) is written by `install_mode_payload`, so shell defaults change with the install-UX action and only exist on live-tools ISOs.
- `D101` [hidden] OpenRC `serial-autologin` is written twice. `pipeline/overlay.rs:278-295` puts a rootfs version with no UX launch and no boot-injection import into the live-boot rootfs, which the live-tools product inherits (`extends`). `artifact/live_overlay.rs:92-120` puts an overlay version with the UX launch on top of it. Only the overlay copy runs, but someone fixing the rootfs one can't tell.
- `D102` [hidden] OpenRC `/etc/inittab` is written twice with the same getty lines: `pipeline/overlay.rs:294-322` (rootfs) and `artifact/live_overlay.rs:148-200` (overlay). The `serial-autologin` path and the baud/TERM arguments have to match in both.
- `D103` [hidden] `/etc/issue` is written by three generators: `overlay.rs:328-337` ("Boot Live"), `live_overlay.rs:123-131` (OpenRC default) and `live_overlay.rs:622-630` (systemd default). All three must carry `LIVE_LOGIN_HINT` for the test at `qemu/serial/mod.rs:71`.
- `D104` [surplus] `live_overlay.rs:123-127` and `:622-626`: the `default_issue` fallbacks are dead because the only caller always passes `Some(...)` (`overlay.rs:39-41,58`).
- `D105` [special] `overlay.rs:355-358` + `bin/workflows/parse.rs:146,158`: the banner format `"{os} {label} Live"` combined with labels "Live Boot"/"Live Tools" yields "AcornOS Live Tools Live".
- `D106` [hidden] `/etc/shadow` has three writers. `variants/acorn/ring2/overlays/live/etc/shadow` (seed, with `sshd`/`chrony` entries and `lastchg=19000`) is copied first at `live_overlay.rs:72-84`, then overwritten unconditionally at `:138-142`. `overlay.rs:339-350` writes a third copy into the rootfs. The seed file is dead, and the `sshd`/`chrony` entries are lost.
- `D107` [hidden] `live_overlay.rs:138` (OpenRC, root `lastchg=0`) vs `:658-663` (systemd, `lastchg=20000`). The systemd comment explains that 0 forces a password change at first login, yet the OpenRC copy (and `overlay.rs:342`) keeps 0. Only one of the copies got the fix.
- `D108` [hidden] `live_overlay.rs:405-410` (`getty@tty1.service.d/autologin.conf`) vs `:455-463` (`getty@.service.d/zz-autologin.conf`). systemd merges template and instance drop-ins sorted by file name, so `zz-` wins for tty1: tty1 likely runs agetty with `--keep-baud … vt100` and the `autologin.conf` drop-in is dead (inferred from systemd drop-in ordering, not booted). `25-shell-color.sh` (`live_tools.rs:643-649`) then silently repairs `TERM=vt100` on `/dev/tty[0-9]*`.
- `D109` [hidden] `live_overlay.rs:102-105` (OpenRC, marker to stdout, `/dev/console` and `/dev/kmsg`) vs `:423` (systemd, stdout only) vs `overlay.rs:281-283` (rootfs OpenRC, all three): three copies of "announce shell ready", now different.
- `D110` [surplus] `live_overlay.rs:45,47,49,52` + `overlay.rs:42-45`: `masked_units` (always `&[]`), `write_serial_test_profile` (always `true`), `machine_id` (always `Some`) and `enforce_utf8_locale_profile` (always `false`) are config knobs that never vary. The locale branch at `live_overlay.rs:636-646` is dead.
- `D111` [hidden] `live_overlay.rs:88` (OpenRC: serial test profile always written) vs `:632-634` (systemd: behind a flag that's always true). The same requirement is expressed two ways.
- `D112` [special] `live_overlay.rs:360-403`: `live-help` and `20-live-shell-ux.sh` exist only on the systemd overlay, so AcornOS has no live-help. The distro difference comes from the init-system code path.
- `D116` [hidden] `live_overlay.rs:50-51` describes `enforce_utf8_locale_profile` as a "strict UTF-8 locale profile", but the branch (`:636-645`) writes `LANG=C`.
- `D117` [hidden] `live_overlay.rs:330-336`: the systemd overlay doc lists four contents and leaves out live-help, the shell UX profile, the network-setup and shutdown units, the serial-autologin script, the locale profile, tmpfiles and machine-id.
- `R002` [hidden] The tmpfs `/var/log` mount is declared twice in the OpenRC overlay, with the option string copied: `distro/builder/src/artifact/live_overlay.rs:207-213` (fstab) and `:218-245` (`local.d/00-volatile-log.start`).
- `R005` [surplus] `distro/builder/src/lib.rs:76-79` publicly re-exports `create_openrc_live_overlay`, `create_systemd_live_overlay`, `InittabVariant`, `LiveOverlayConfig`, `SystemdLiveOverlayConfig`; the only users are in-crate (`distro/builder/src/pipeline/overlay.rs:8-11`, `distro/builder/src/pipeline/config.rs:14`).
- `R006` [special] `distro/builder/src/artifact/live_overlay.rs:658-660`: root `lastchg` is set to 20000 to dodge the forced password change, in the systemd copy only.
- `R010` [special] `distro/builder/src/pipeline/overlay.rs:13`, `:44`: every live ISO gets the fixed machine-id `0123456789abcdef0123456789abcdef`.

### F22. ISO payload is built by recipes and one generic TUI-app action.

Solves 18 problems (13 hidden · 1 needless · 1 surplus · 3 special).

Bundling a TUI app is one runtime action, `tui_app { app = "<name>" }`, that reads the manifest (F2). A recipe builds the app bundle from the root workspace with the frozen lockfile. Bun comes from a recipe pinned to the version in the root `package.json` `packageManager`, not from the host's PATH, and the ISO build runs no network `bun install`. Whether disk-plan ships on an ISO becomes one line in the variant; until it does, the READMEs say it ships nowhere.

Rust guest tools build through one recipe: `install-split-pane.rhai` folds into `install-tools.rhai`. `recipe/tools.rs` has one package map (which fixes the missing `recpart`) and one target list, and verifies with `--version` instead of grepping help text. The split-pane manifest inherits workspace settings like other members.

Smaller cleanups: one `write_executable`, one build-and-copy function, and `process::which` instead of a local copy. Each action names its payload layer (rootfs or overlay) instead of taking it from argument order. apk packages come through recipes, with no host `resolv.conf` left in the image.

**Done when:** The live-tools build succeeds with `recpart` declared; no host binary is copied into the ISO; the builder has no `tui/apps/...` path; there is one Rust guest-tool recipe.

**Depends on:** F2, F18.

- `B132` [hidden] `distro/builder/src/pipeline/live_tools.rs:319` and `:326` (bundle dir and file path) are repeated as a string literal at `live_tools.rs:383`. `live_tools.rs:361` (`usr/local/bin/bun`) is repeated at `live_tools.rs:400`.
- `B135` [hidden] `distro/builder/src/pipeline/live_tools.rs:295` checks `node_modules/@levitate/tui-kit/package.json` to decide whether to run `bun install`, while `xtask/src/tasks/check.rs:406` checks for `node_modules/`. These are two different "deps present" rules for one app.
- `B136` [hidden] `distro/builder/src/pipeline/live_tools.rs:287` and `:366` copy whatever `bun` is on the builder's PATH into the ISO. The ISO runtime version is set by the host and pinned nowhere (root `package.json` says `bun@1.2.17`, the baseline ran bun 1.4.2, and `.github/workflows/bun-ci.yml` uses `latest`).
- `C001` [hidden] `dp/README.md:3` ("Canonical install disk-planning UI owner"), `tools/recpart/README.md:12-14` and `tools/recpart/REQUIREMENTS.md:116-117` name disk-plan as the installer UI, but no ISO ships it (`distro/builder/src/pipeline/live_tools.rs:279` ships only install-docs). The guidance users actually get is the docs' raw `recpart` and `recstrap`/`recfstab` steps (`docs/content/src/content/05-rec-tooling/10-recpart.ts`, `01-getting-started/04-installation-base.ts:66`). Two owners of the install flow, and the layout hides which one users see.
- `C002` [hidden] `tui/README.md:3` says the apps are "for the LevitateOS live ISO and installer". disk-plan and recab are on neither live ISO and on no installed system, so nothing ties them to a product that would make them change.
- `D029` [hidden] `recipes/install-split-pane.rhai:24-28` verifies the built binary by grepping `--help` output for `"levitate-install-docs-split --smoke"`, which ties the recipe to the wording of `main.rs:74-81` `print_help`. Rewording the help text breaks the build.
- `D057` [hidden] `products.rs:473-480` passes `live_overlay_dir` as the `tool_payload_dir` parameter (`live_tools.rs:40`). As a result, `ToolPayloadWorkspaceBinary` writes to the overlay while `RootfsWorkspaceBinary` (split helper) and `install_mode_payload` write to the rootfs. Which layer a file lands in comes from argument order, not from the name.
- `D059` [hidden] `recipe/tools.rs:12-16` (`prepare_tool_source`) and `recipe/tools.rs:190-199` (`build_tool_program`) each keep their own package map. c015d4d added `recpart` to the second but not the first, so `build_tool_program("recpart")` reaches `prepare_tool_source`, which bails "no live-tool source producer declared for recpart". Both variants' `live_tools_common` declare `recpart` (`levitate/ring2/products.toml:168-170`, `acorn/ring2/products.toml:100-102`), so the live-tools build should fail at that step (not run, read from code).
- `D060` [hidden] The supported Rust targets are listed three times: `recipe/tools.rs:204-210`, `recipes/install-split-pane.rhai:17-19` and `recipes/install-tools.rhai:23-25`, plus the per-variant `target =` in `products.toml`.
- `D061` [hidden] `recipes/install-split-pane.rhai` and `recipes/install-tools.rhai:30-130` are near-identical recipes (acquire/build/install/cleanup, musl linker flags, GNU host check) that differ only in name and `_verify`. A change to the build flags has to be made in both.
- `D062` [needless] `recipe/tools.rs:225` always passes the `BIN` define, but `install-split-pane.rhai` never reads it, since only `install-tools.rhai:10` does. The general function carries a parameter that only one recipe understands.
- `D063` [hidden] `recipe/tools.rs:165-167` bails on `workspace = true` dependencies. `install-split-pane/Cargo.toml:10-15` has to spell versions and `edition` out rather than inherit from `[workspace.package]` like other members, and nothing in that manifest says why.
- `D067` [special] `live_tools.rs:287-292,361-378`: the host's own `bun` from `PATH` is copied into the guest as `/usr/local/bin/bun`. That's a host toolchain binary of unknown version and libc in the ISO, bypassing recipes, which by `AGENTS.md` own acquisition/build.
- `D068` [special] `live_tools.rs:296-317` runs `bun install` (network) during the ISO build when `node_modules` is missing. The build therefore depends on the developer's checkout state.
- `D069` [hidden] `live_tools.rs:192-215` (`copy_executable`), `:691-703` (`write_executable`), `:373-378` (bun chmod inline), `live_overlay.rs:681-687` (`write_executable`) and `overlay.rs:289-295` (inline chmod) are five copies of "write and chmod 0755".
- `D070` [hidden] `live_tools.rs:136-164` and `:166-190` are near-identical functions (build via `build_tool_program`, then copy) that differ only in destination, with identical error-context strings.
- `D072` [special] `live_tools.rs:222-231,242`: `ensure_apk_packages` copies the host's `/etc/resolv.conf` into the shipped rootfs and never removes it, then runs `apk add --no-cache --no-check-certificate` over the network inside `unshare` + `chroot`. The ISO contents depend on host DNS and an unverified mirror at build time.
- `R003` [surplus] `distro/builder/src/pipeline/live_tools.rs:267-272` (`find_executable_in_path`) re-implements `distro/builder/src/process.rs:286` (`which`).

### F23. The split pane is a robust terminal host.

Solves 16 problems (4 hidden · 4 surplus · 8 special).

One layout function gives both the drawn pane areas and the PTY sizes, with the footer excluded. The focused pane's cursor is drawn. A failed write or resize marks that pane as exited instead of ending the host, an exited pane shows "(exited) Enter restarts", and the host exits only when the user quits.

Every key crossterm reports is forwarded as its xterm sequence, F-keys included. The host keys (focus switch, quit) are one table that the footer renders and that `install-guide.txt` and live-help point to instead of restating, and the switch key isn't one `less` needs. The screen redraws only on PTY output, input or resize.

Dead state goes: `logs`, the unread scrollback, the duplicate Left/Right enums and the two percent constants. argv is parsed with clap without exiting inside the parser. Both pane commands are validated the same way, and errors name the command that was given.

**Done when:** F4's composed-screen render at 80×24 shows both panes at the right heights with the shell cursor; killing the right child leaves the host running; `main.rs` has one Left/Right enum.

**Depends on:** F19.

- `D032` [hidden] `live_overlay.rs:370-372` (live-help), `variants/acorn/ring2/install-guide.txt:5-6` ("Ctrl-g switches panes") and `main.rs:587-590` (footer) each restate the key bindings that only `main.rs:307-314` implements.
- `D073` [hidden] `main.rs:259-262` (startup) and `:323-325` (resize) size the PTYs from the full terminal `Rect(0,0,cols,rows)`, while `draw` at `:578-583` lays the panes out in `root[0]`, one row shorter because of the footer. Each child's PTY is one row taller than its visible area, and `render_styled_rows` (`:625`) drops the top row. The two `pane_layout` calls must stay in step, and nothing shows it.
- `D074` [surplus] `main.rs:366,371-386,406,410,413-415`: `App.logs` is filled and trimmed but never drawn or read, so it's dead state.
- `D075` [surplus] `main.rs:339-359` `FocusPane` and `:467-471` `PaneId` are two identical Left/Right enums, mapped by hand at `:584-585` and `:419-422`.
- `D076` [surplus] `main.rs:33,443`: `PTY_SCROLLBACK = 10_000` is allocated for each pane, but no key or code ever scrolls, so the scrollback can't be seen.
- `D077` [surplus] `main.rs:31-32`: `LEFT_PANE_PERCENT` and `RIGHT_PANE_PERCENT` are two constants for one fixed 50/50 split that nothing changes.
- `D078` [special] `main.rs:253`: the cursor is hidden once, and the vt100 cursor position (`screen().cursor_position()`/`hide_cursor()`) is never rendered, so the focused shell pane shows no cursor.
- `D079` [special] `main.rs:317,321` (`write_to_focused(...)?`) and `:325` (`resize(...)?`): one failed write to an exited child's PTY, or a failed resize, ends the whole UI with an error. Combined with the `exec` chain, that kills the login session.
- `D080` [special] `main.rs:404-411`: when a pane's child exits, the only reaction is an "(exited)" title. With both panes dead the app keeps running until Ctrl-q, and nothing restarts a pane.
- `D081` [special] `main.rs:511-514`: any read error other than EINTR is reported as `Closed`, so the error is swallowed. `:220-223` and `:332-335` discard kill/wait results with `let _ =`.
- `D082` [special] `main.rs:799`: function keys and any other unmapped `KeyCode` are silently dropped (`_ => return None`), so the docs TUI never receives them.
- `D083` [special] `main.rs:307-314`: Ctrl-q and Ctrl-g are always taken by the host and can't reach a child (Ctrl-G is `less`'s file-info key).
- `D084` [special] `main.rs:300` `POLL_INTERVAL` 16 ms: the loop redraws every iteration whether or not anything changed.
- `D085` [special] `main.rs:62-64`: `Args::parse` calls `process::exit(0)` from inside the parser.
- `D086` [hidden] `main.rs:116-121`: if `LEVITATE_INSTALL_RIGHT_CMD`'s first token isn't on PATH, the setting is silently ignored and the default docs command used, while `resolve_left_command` (`:103-111`) runs whatever it's given unchecked. The two pane configs have opposite validation, and a misconfiguration never surfaces.
- `D088` [hidden] `main.rs:163,573` hard-code the default docs command's name in user-facing error text, separately from `:124`.

### F24. Install tests exercise the real launch, not a smoke stand-in.

Solves 16 problems (10 hidden · 1 needless · 5 special).

The entrypoint has no test branches (`--probe`, `LEVITATE_INSTALL_ENTRYPOINT_SMOKE`), and the split pane has no `--smoke`. The install-tests scenario boots one live-tools ISO with the serial-UX key set and checks the serial console's composed screen: the docs pane shows the default page's title, and the shell pane answers a typed command after a focus switch. The default scenario, without the key, keeps a plain serial shell. xtask injects the key only for products that have an install UX. Guest-facing strings (login hint, ready marker) move into `distro-contract`, so the test crate reads them there instead of depending on `distro-builder`. Readiness patterns and comments match what runs.

**Done when:** `testing/install-tests` has no smoke markers and doesn't depend on `distro-builder`; a broken docs bundle or a broken shell fails the scenario. This needs a bootable QEMU setup, which this machine didn't have at audit time.

**Depends on:** F19, F23.

- `D008` [special] `xtask/src/tasks/scenarios.rs:157-159` injects `LEVITATE_INSTALL_SERIAL_UX=1` for the `live-boot` product too. That ISO has no install entrypoint, so the flag only works because each reader guards with `[ -x @INSTALL_ENTRYPOINT@ ]`.
- `D028` [hidden] `live_ux.rs:21` defines `SPLIT_SMOKE_OK = "split-smoke:ok"`, but the producer at `main.rs:226` prints its own literal `"split-smoke:ok left=…"`. The constant's comment points at the binary, but the binary can't import it.
- `D043` [special] `live_tools.rs:533-541` (`--probe`) and `:548-554` (`LEVITATE_INSTALL_ENTRYPOINT_SMOKE=1` → `--smoke`) are test-only branches compiled into the shipped entrypoint, which matches helper names by shell `case` patterns.
- `D087` [special] `main.rs:243`: `truncate_for_error` slices `&s[s.len()-MAX..]` by byte offset on a `from_utf8_lossy` string. If that offset lands inside a multi-byte character (box drawing, colour prompts), it panics, turning a smoke failure into a panic.
- `D089` [hidden] `main.rs:187-205`: the left-pane smoke check sends `printf '__LEVITATE_SPLIT_LEFT_OK__\n'` and passes as soon as the marker shows up in the PTY output. The tty line discipline echoes the typed input, marker included, before any shell runs, so the check passes even if the left shell never starts. It can't catch a broken shell.
- `D090` [hidden] `main.rs:177-182`: the right-pane smoke check only runs `command -v levitate-install-docs`, the same existence check `resolve_right_command` (`:118`) already made. It never runs the docs command, so a missing Bun bundle, a broken `bun` or a missing `/usr/local/share/levitate/install-docs.txt` all pass.
- `D091` [hidden] `main.rs:43-45` vs `:247-286`: `--smoke` never touches `run_ui`. It doesn't exercise raw mode, alternate screen, layout, rendering, key forwarding or resize, which is everything that can break in the real UX.
- `D092` [special] `main.rs:35,178,199`: smoke relies on an 8 s timeout, a `sleep 0.2` inside the right probe and a 100 ms `recv_timeout` poll.
- `D093` [hidden] `testing/install-tests/src/scenarios/mod.rs:944-976` runs `LEVITATE_INSTALL_ENTRYPOINT_SMOKE=1 /usr/local/bin/levitate-install-entrypoint` over SSH. The profile hook, the tty1/ttyS0 selection, `LEFT_CMD`/`RIGHT_CMD`, autologin and the serial UX path are never exercised, so the test checks the smoke stand-in rather than the real launch.
- `D094` [hidden] `testing/install-tests/src/scenarios/mod.rs:975` reports "install UX split-pane smoke verified (shell-left + docs-right)", although the smoke verifies neither a running shell nor rendered docs.
- `D095` [hidden] `testing/install-tests/src/scenarios/mod.rs:446-449` only checks `test -r 30-install-ux.sh`. The hook's content and whether it launches anything aren't tested.
- `D097` [hidden] `testing/install-tests/src/scenarios/mod.rs:479-480` says "the serial console may run the install UX rather than a shell", but the same scenario then runs `console.exec` on serial (`:402` → `verify_live_overlay_behavior`, `:1201-1216`), which needs a shell. The comment contradicts the code, and the scenario would fail if serial UX actually ran.
- `D098` [special] `testing/install-tests/src/qemu/serial/mod.rs:62-80` (`wait_for_live_tools_with_context`) adds `serial_ready_patterns` plus `LIVE_LOGIN_HINT` as readiness patterns for a "serial runs the UX" case the harness never triggers, because it never injects `LEVITATE_INSTALL_SERIAL_UX`.
- `D099` [needless] `testing/install-tests/src/scenarios/mod.rs:28-31` and `qemu/serial/mod.rs:71`: the test crate depends on the whole `distro-builder` crate (`testing/install-tests/Cargo.toml:18`) to read the string constants in `live_ux.rs`. The UX's guest-facing interface lives inside the builder's internals.
- `D100` [hidden] `testing/install-tests/src/scenarios/mod.rs:954` accepts both `SPLIT_HELPER` and `*/SPLIT_HELPER`, mirroring the two `choose_install_helper` rungs at `live_tools.rs:518-528`. Change the ladder and the test has to change too.
- `D119` [hidden] `schema.rs:357-359` documents `serial_ready_patterns` as the readiness lines when "the serial console runs the install UX", but the harness never runs serial UX (see the install-test entries above).

### Leave as is

Left during the refactor (F14): these need recab's and recpart's own output to change (recab's CLI moved into its library and a `schema_version` on `status --json`; recpart's handoff document carrying the mount root, next-step commands and a slot type). Another session was rewriting recab (trial supervision, durable state) and editing recpart's policy and tests while this ran, so changing their public output here would have collided. The TUIs keep these copies until those tools settle; the generator side (dependency walk, per-manifest output, formatting, default mode, mode help, one output reader) is done.

- `C011` [hidden] `dp/src/presentation/disk-plan-screen.tsx:139-145` `nextInstallSteps` spells the `recstrap <t>`, `recfstab <t> >> <t>/etc/fstab` and `recchroot <t>` syntax plus "set a root password and install the bootloader". These are a copy of the docs' manual sequence (`docs/content/src/content/05-rec-tooling/02-installation-tools-overview.ts:83`, `06-recfstab.ts:55`), and the generated `TOOL_HELP` exists for exactly these CLIs.
- `C012` [hidden] `dp/src/presentation/disk-plan-screen.tsx:1118-1122` renders `mode_context.install_target_slot`/`inactive_slot_hint` as free strings (`dp/src/contract/recpart.ts:51` `string | null`). They are the same `"A"`/`"B"` letters as recab's generated `Slot` (`rc/domain/contract.ts:3`), with no shared type (`tools/recpart/src/handoff.rs:24-28`).
- `C014` [hidden] The apply command (`plan-runner.ts:35`) never passes `--mount-root`, so it depends on recpart's default `/mnt` (`tools/recpart/src/cli.rs:53`). The handoff target that `nextInstallSteps` uses is derived from that default. This is B019, kept by the prior untangle and still implicit.
- `C073` [hidden] `rc/flows/index.ts:10,36,43,50` spell recab's CLI (`status --json`, `set-next <SLOT>`, `commit`, `rollback`) as free strings, separate from clap's `Command` (`tools/recab/src/main.rs:16-35`). The prior untangle listed this as left over (`docs/history/UNTANGLE.md:1062`), and it is still true.
- `C074` [hidden] `rc/flows/index.ts:27,35,43,51` action descriptions restate recab's clap doc comments (`tools/recab/src/main.rs:17,24,30,33`) and already disagree. Rollback is "Set next/default target back to known-good slot" in the TUI and "Configure next boot to previously known-good slot" in recab.
- `C075` [hidden] `rc/presentation/runtime-recab-screen.tsx:41-49` `STATUS_ROWS` restates `Status`'s fields and recab's text labels (`tools/recab/src/main.rs:74-98`) with different empty spellings (`(none)` via `formatSlot` vs recab's `unknown`/`none`). B023 was kept as "the TUI's own presentation", but adding a `Status` field still requires editing this table by hand.
- `C076` [hidden] `rc/presentation/runtime-recab-screen.tsx:205` "Mutating actions require root." and `rc/flows/index.ts:14` `section: "Read" | "Mutating"` restate recab's privilege rule (`ErrorClass::Permission`, `tools/recab/src/lib.rs:52`).
- `C077` [hidden] `rc/domain/recab.ts:17-24` `parseRecabStatusJson` accepts any object whose `active_slot` is a slot and casts it to `Status`. recab's status JSON has no `schema_version` (`tools/recab/src/lib.rs:138-146`), unlike every recpart document, so a shape change is undetectable. The comment's "only its shape is confirmed" overstates a one-field check.
- `C132` [hidden] `docs/history/UNTANGLE.md:1031` and `:1062` leave "the recab TUI spells CLI arguments outside the contract and restates `Status` fields" open. That is still the case at `rc/flows/index.ts:10,36,43,50` and `rc/presentation/runtime-recab-screen.tsx:41-49`.
- `C137` [hidden] `docs/history/UNTANGLE.md:1037` (F27) makes "the installer TUI own its next-steps text on the handoff page" (`disk-plan-screen.tsx:139-145`). The TUI ships nowhere, so the guidance users get is the docs' copy, which F27 assigned a different scope.

`displayVendor` hiding `ATA`/`NVME`/`SCSI` is a real requirement: those strings name the transport, not the vendor, and showing them is noise. It is one local function (prior untangle B127, kept).

- `C050` [special] `dp/src/presentation/disk-plan-screen.tsx:100-107` `displayVendor` hides the vendors `ATA`/`NVME`/`SCSI` (B127, kept).

History records the names as they were at the time, and no live doc uses the old ones. Editing the history would falsify it.

- `C136` [hidden] `docs/history/UNTANGLE.md:1031` "`cargo xtask contracts` writes …" and "`just tui-install-disk-plan` builds and puts this checkout's recpart on PATH". The command is now `cargo xtask gen` (`xtask/src/tasks/generate.rs:1`) and the build/PATH step lives in `xtask/src/tasks/tui.rs:24-42`. The history is accurate as history, but neither name exists now. (Checked: no live doc still uses the old names.)

The split-pane crate stays in `distro/builder`. After F19 and F23 its only interface is argv, and F4 checks the width contract by rendering at the pane's real size. Moving a Rust guest binary into the TypeScript `tui` repo, or into a new `tools/` submodule, costs a repo split and recipe changes for coupling that will no longer exist. The parallel tmux implementation this entry also mentions is deleted under F19 (B170).

- `D064` [hidden] `install-split-pane/` is a guest runtime program but lives under `distro/builder/`, the build tool, and is a root workspace member (`Cargo.toml:6`). The other half of the install UX (`tui/apps/live-tools/install-docs`) lives in `tui/`, and its own tmux `bin/levitate-install-docs-split` (`package.json:11`) is a second, parallel implementation of the same helper.

AcornOS's non-merged `/usr` is a real layout difference, and reading the rootfs as it is is the right way to find it. A config knob would just restate the rootfs.

- `D071` [special] `live_tools.rs:120-128` adds `bin` to the destination list only when the rootfs `/bin` is a real directory, a special case for non-merged-/usr roots (AcornOS).

Generic live-ISO behaviour outside the install UI: QEMU-slirp network setup, the live init's mount layout, chrony's log directory, the Alpine cdrom unmount, serial-getty's targets and OpenRC's per-service runlevels. No install-UI fix touches them; they belong to a separate live-overlay untangle. D113 does cost real hardware a 45-second wait before sshd, which is worth raising on its own.

- `D113` [special] `live_overlay.rs:493-525` (`live-net-setup`): a 45-iteration `sleep 1` retry loop with hard-coded slirp addresses `10.0.2.15`/`10.0.2.2` ships in every LevitateOS live ISO, including on real hardware.
- `R001` [hidden] `distro/builder/src/artifact/live_overlay.rs:569` and `:579` hardcode `/live-overlay /rootfs /run/live-media` and `/dev/loop1 /dev/loop0` from the live init's mount layout; `recinit::live::LIVE_MEDIA_DIR` (`tools/recinit/src/live.rs:56`) exists but is not used.
- `R007` [special] `distro/builder/src/artifact/live_overlay.rs:239`: `mkdir -p /var/log/chrony` for one service inside the generic volatile-log script.
- `R008` [special] `distro/builder/src/artifact/live_overlay.rs:585-587`: `/media/cdrom` unmount (an Alpine mount point) in the systemd shutdown cleanup.
- `R009` [special] `distro/builder/src/artifact/live_overlay.rs:468-475`: `serial-getty@ttyS0` is enabled from both `basic.target.wants` and `getty.target.wants`, with no comment on why it needs the earlier target.
- `R011` [special] `distro/builder/src/pipeline/overlay.rs:209-253`: OpenRC service wiring has one hand-written branch per service name (`sshd`, `networking`, `dhcpcd`), each with its own runlevel; any other name bails.

## Progress

Commits land on the checked-out branches (superproject `master`, submodules `codex/streamline-development`); AGENTS.md forbids creating branches, so each fix is one commit per repo it touches, submodules first.

**Baseline rerun (2026-11-06, before F1).** Same results as `## Baseline` once this machine had what the old one had: a root `bun install`, `docs/content` `bun run build` (its `src/generated` was missing, so install-docs failed typecheck), and a `bun install` inside recab (which recreates the duplicate React). Typecheck passes in all four packages; lint fails in kit and install-docs, format-check fails in all four; `inspect:check` fails on `001-installation-disk.txt`; `test:e2e` 0/3 (`unrecognized subcommand 'docs'`); `cargo check -p install-split-pane` passes. Renders: disk-plan identical, recab identical (crash), docs identical except `021-faq`, `022-how-we-compare`, `024-variants` and the manifest, which changed with docs/content `b22d24b` (Rocky → Fedora wording). Outputs in `.artifacts/untangle-ui/rerun/`.

- [x] F1 — One TUI workspace: one React, one lockfile, one toolchain, one package shape. tui `74d5e49`; superproject commit "One TUI workspace…" (root workspaces incl. recab, root `bun.lock` un-ignored and committed, root `.oxlintrc.json`/`.oxfmtrc.json` with tabs (the style of kit, install-docs and docs/content; disk-plan and recab reformatted), turbo keeps only dev/build, xtask runs one `bun install --frozen-lockfile`).
  - **Bug fixed:** recab TUI crashed on launch (duplicate React); it renders now.
  - recab's runner moved to `Bun.spawn` early (its `node:child_process` typings broke without `@types/node`); same failure semantics. The PTY test's unused `expect` import was dropped so lint passes.
  - Known until F14: the generated contracts are now formatted, so `cargo xtask gen --check` reports them stale.
- [x] F2 — One app manifest. contract `c239381` (`distro_contract::tui_apps`, reads root workspaces and each app's `package.json`), builder `ae1609b`, tui `93d9bfb`, superproject commit "One app manifest…". Apps are `install-docs`, `disk-plan` and `recab-tui` (recab's TUI is renamed so its command can't clash with the `recab` tool); xtask's `TuiApp` enum, per-app arms and `TS_PACKAGES` are gone, and `cargo xtask tui|inspect tui <name>` works for every app, building the app's declared tools onto PATH.
  - Fixed early: B135 — the builder now runs the same `bun install --frozen-lockfile` at the root instead of probing `node_modules/@levitate/tui-kit`.
  - Deferred: the live ISO still names the command `levitate-install-docs` (F19 renames guest commands); C097's per-app title/scope strings now all use the one name but are still literals until F10's `runApp({name})`; disk-plan's inspect args still carry `--disk /dev/sda` (now in its manifest) until F3's fixture recpart.
- [x] F3 — One xtask TUI driver. tui `806668c`, contract `4b06682`, superproject commit "One xtask TUI driver…". `inspect_docs.rs` and `inspect_tui.rs` are replaced by `xtask/src/tasks/tui/inspect.rs`; `cargo xtask inspect <app> [--input KEYS] [--all-pages] [-- args]` replaces `inspect docs` and `inspect tui`. `util/pty_capture.rs` is now a `PtySession` that writes keys to the PTY master one at a time and waits for the screen to go quiet (no fixed timer, no `/dev/tty`, no `script(1)` banner stripping, no `001-` numbering). Pages come from `install-docs --pages`; disk-plan's preset no longer injects `/dev/sda` (the app lists disks itself) and a shim refuses `recpart apply --confirm` in headless runs. One `cargo_target_dir` helper; xtask no longer checks for a TTY before `tui` (the app does).
  - **Bug fixed:** `--input` keys never reached the app; `cargo xtask inspect disk-plan --input jj` now lands on mode-selection.
  - Verified: all 35 docs pages render byte-identical to the baseline rerun.
  - Deferred to F20: `pty_capture` still forces TERM/COLORTERM/FORCE_COLOR like the split pane (X008's copied env), and `tui` still forces `FORCE_COLOR=3` (A016).
- [x] F4 — Checks that fail on real bugs. builder `a717466` (`install-split-pane` exposes its screen layout as a library), contract `6362aed` (manifest `inspect.mask`, `inspect.walk`), tui `5c45ef5`, superproject commit "Checks that fail on real bugs…". `cargo xtask check tui` runs typecheck/lint/format-check for every TUI package, renders every app at 120x36, renders install-docs at the docs-pane size of an 80x25 and a 128x48 console (38x22, 62x45, from the split pane's own layout), compares `snapshots/<cols>x<rows>/<page>.txt` (masks from the manifest, `--bless` to accept, diff printed), and runs each app's declared key walk. The bun PTY test, `verify-inspect-snapshots.sh` and the `inspect:check`/`test:e2e` scripts are deleted.
  - **Bug fixed early (D073):** the split pane sized each child's PTY from the full terminal while drawing it one row shorter; both now come from one layout.
  - Verified: the walk reaches disk-plan's preflight page; editing a snapshot fails the check with a diff; recab's old crash text and "Terminal too small" are failure markers.
  - Expected failures until F9: both docs-pane renders ("Terminal too small: got 38x22, need 88x24").
  - Not done here: disk-plan's walk drives the host's real `recpart` read-only (list/plan/dry-run; the shim refuses `apply --confirm`), so it stops at preflight on a host whose disk is mounted and can't cover the apply outcomes; a recorded recpart fixture belongs with F15. recab-tui renders against the host's real `recab` (exit 21 off an A/B system) rather than a fixture.
- [x] F5 — Delete the kit surface no app uses. tui `02ee744`. Kit: 84 files / 5,511 lines → 38 / ~3,960. `src/index.ts` lists exactly the 42 names the apps import; everything unreachable from it was deleted, then unused exports were un-exported and dead declarations removed until lint was clean. `useAsyncTask` was deleted too (nothing used it; F10 writes the version disk-plan needs, which also fixes A180/A181).
  - **Bug fixed (by deletion):** `AsyncWizardFlow` ignored `defaultIndex` (A152).
  - Verified: `cargo xtask check tui` passes except the two expected docs-pane renders; snapshots unchanged; the README Quick Start typechecks and renders.
- [x] F6 — One colour pipeline from theme intent to Ink. tui `55010ff`. A `ThemeColor` is `#rrggbb` or `"inherit"`; `resolveColor`/`resolveIntentColor` return hex or undefined; `detectColorRuntime(env)` only reads `NO_COLOR` (chalk's supports-color decides the level and ignores `NO_COLOR`). `createTheme` merges colours by spread. Docs theme names shared colours once; note colours are `NOTE_COLORS`; inline code uses the new `code` intent; disk-plan's `"red"`/`"cyan"`/… are now `error`/`accent`/`warning`/`success` intents.
  - **Bugs fixed:** colours vanished on 256-colour terminals without `COLORTERM`/`FORCE_COLOR` (installation-disk page: 20 → 229 colour codes; verified with `script`); `NO_COLOR=1` still printed hex note/syntax colours (now 0 colour codes); command rows always black; `withBackgroundIntent` dropped its background; list markers and conversation role labels rendered uncoloured; conditional hook calls in `intent-color.ts`, `syntax-line.tsx`, `table-block.tsx`.
  - Behaviour change on purpose: command rows take the theme's `commandBarBackground` (`inherit` in the docs theme) instead of black.
  - Still open, in their own fixes: `rich-text.tsx` returns before its hooks (A162, F11); launchers still force `FORCE_COLOR=3` (F20), now harmless.
- [x] F7 — Keymap as data, one dispatcher, focus scopes. tui `0d4f785`. Kit `runtime/keymap.tsx`: `KeyboardProvider` (one `useInput`, mounted by `renderApp`), `useKeymap(keymap, handlers, {active, onText})`, `keyName`, `keymapHelp`, `keyHint`; `keymapStatusSegments` in status-line. Keymaps: install-docs `presentation/ink/keymap.ts`, disk-plan `presentation/keymap.ts`, recab-tui `RECAB_KEYMAP` in `flows`. `useFocusPlane` is state only; disk-plan's pane focus is still its own `"nav"|"content"` state (A062's second half moves with F15's rewrite).
  - **Bugs fixed:** Ctrl-C also fired install-docs' `c`/`y` copy; `G` also fired `g`; disk-plan's confirm-token typing reached navigation/quit bindings and Enter ran two handlers; disk-plan footer showed wrong keys in nav focus; quit during disk-plan's apply was silently ignored (now says why) and recab-tui quit mid-mutation (now refused with a notice).
  - Behaviour changes on purpose: the DESTROY token is compared exactly as typed (no upper-casing, any printable character), C058; the docs status bar shows the focused pane's keys first (snapshots re-blessed); install-docs' copy note no longer repeats the action counter.
  - Also: recab-tui's manifest refuses `recab commit|rollback|set-next` in headless runs (found while checking that `c` no longer quits: it ran the host's real `recab commit`).
- [x] F8 — No app vocabulary in the kit. tui `528ce43`. Renames: sidebarSectionText→navHeading, sidebarItemText→navItem, sidebarItemActive*→selection/selectionBackground, sidebarBorder→navBorder, sidebarBackground→navBackground, linkText→link, linkActiveText→linkActive, sectionHeading→heading, sectionSubheading→subheading, cardBorder→contentBorder (pane frames) / panelBorder (blocks), cardBackground→panelBackground, layout sidebarWidth→navWidth; commandBarBackground/commandPrompt left the kit (`COMMAND_COLORS` in the docs theme). Markup parser → `install-docs/.../blocks/shared/syntax-markup.ts`; `linkRuns` → `rich-text-renderer.tsx`; `CommandLineRow`/`SyntaxLine` take theme colours instead of unused intent props (B118/B119 done early; `syntaxTokenColors` B067 deleted early).
  - Verified: snapshots and the 256-colour escape histogram of the installation-disk page are identical to F7.
  - Left for F9: `ThemeLayout.headerHeight/footerHeight` and the strings helpers' `minimum = 20` default (A063) belong to the frame rework.
- [x] F9 — One frame and width model; one app shell. tui `0119ef3`. Kit `surfaces/shell.tsx` (`useShellLayout`, `AppShell`), `chrome/glyphs.ts` (`glyphs`, `inkBorderStyle`, `BorderStyle`), `utils/strings.ts` on `string-width`. Deleted: `surfaces/frame/*`, `chrome/{components,renderer,grid,types,glyph-sets}.ts`, `runtime/terminal/screen-size.ts`, `utils/arrays.ts`, install-docs `chrome-architecture.md`. `ThemeLayout` is `{navWidth, minColumns}`; `createTheme` spreads. Kit now 26 files / ~2,020 lines (from 84 / 5,511).
  - **Bug fixed:** install-docs in the install screen's docs pane showed only "Terminal too small" (verified at 38x22 and 62x45 by `cargo xtask check tui`); it now opens on the page, Tab shows the page list. Snapshot `snapshots/38x22/installation.txt` added.
  - Also: disk-plan's disk cards used `"cyan"`/`"gray"` and Ink's own `single` box (missed by F6).
  - Unchanged: 120x36 snapshots identical; F9's empty-nav frame in the viewer (B090) was deleted early because it was the only other `SurfaceFrame` user and is unreachable.
  - Moved to F11: A184 (blank lines rendered as `" "` per site) needs the one line renderer F11 builds.
- [x] F10 — The kit owns the app lifecycle. tui `c53f1d6`. Kit `app/run-app.tsx` (`runApp`, `UsageError`, `errorMessage`, flag spec → `--help`), `app/command.ts` (`startCommand`, `runCommand`), `useAppName`. Each app's `src/main.tsx` is one `runApp` call; `cli/` directories, bootstraps and install-docs' `errors.ts` are gone; disk-plan's `DiskMode` alias became `InstallMode` (C030/C031 done early) and its recpart wrappers became `runRecpart`/`startRecpart` over the kit runner.
  - **Bugs fixed:** a render error exited 0 (verified in a PTY for an error in the first frame and in a later one: now exit 1 with Ink's error screen and the stack on stderr). Cause found on the way: when the first frame throws, Ink unmounts before `waitUntilExit` is attached and that promise never settles, so the kit records the crash in a boundary and sets the status on `exit`. `--slug -x` was rejected as a missing value (B084).
  - Behaviour changes on purpose: recab-tui has no `--recab-bin` (tools come from PATH, as xtask sets it); every app's usage errors print `<name>: <message> (see --help)` and exit 2; the docs footer reads `[install-docs]` (snapshots re-blessed).
  - Deferred to F15: `useAsyncTask` (A180/A181) is written with disk-plan's hooks, its only user.
- [x] F11 — One generic component per job. tui `866af40`. Kit: `components/navigation/nav-list.tsx` (`NavList`, `NavListMode`), `components/navigation/selectable-grid.tsx` (grid folded in), `utils/runs.ts` (`RichTextRun`, `appendRun`, `wrapRuns`, `wrapText`, `prefixWrappedText`, `truncateLines`). Deleted: `tree-nav.tsx`, `section-list.tsx`, `primitives/layout/grid.tsx`, the docs app's word-wrap internals and `StatusAtom`.
  - **Bug fixed:** `RichTextLine` called hooks after an early return (A162; React reports it when a line goes from empty to non-empty).
  - Behaviour change on purpose: recab-tui's action list now uses the same nav list as the docs sidebar (section markers, highlighted selection) instead of a plain `>`/`::` text block.
  - Verified: snapshots unchanged; 256-colour code counts unchanged (installation-disk 229).
  - Moved to F12: A150 (`useScrollState` taking its maximum once) needs the docs line model, where the maximum is known before the offset.
- [x] F12 — install-docs lays each block out once. tui `dd7b090`. New `src/layout/page.ts` (`layoutPage`, `Line`, `Anchor`) and `src/layout/syntax-markup.ts`; the viewer draws lines from it. Deleted `rendering/`, `domain/render/`, `presentation/ink/{blocks,document,primitives}/`, `components/content-pane.tsx`; install-docs is 1,436 lines (from 4.3k at the audit). Kit `useScrollState(maxScroll)` (A150, moved here from F11).
  - **Bugs fixed:** scroll drift from measure/render disagreement (long command lines, the `[COPY]` suffix, interactive steps); QA-answer links never highlighted; link targets scrolled to the block, not the link's line; all links sharing an href highlighted together; table cells cut to one line; interactive step commands parsed as colour markup.
  - Verified: `G` lands on each page's last line (installation-disk, faq, helpers-overview, cli-reference, variants); snapshot changes limited to tables (cells wrap; the divider spans the columns only).
  - Behaviour changes on purpose: a selected command shows `▸` instead of `$` and no longer grows a `[COPY]` suffix (the status bar says Enter copies), so selecting never moves the layout.
  - Found, not fixed here (logged under Problems as N001): docs-content's highlighted lines end a `\`-continued line with `\[[/]]`, which the markup grammar reads as an escaped `[[`, so `[[/]]` shows in code blocks (also before this fix).
- [x] F13 — Apps get distro identity from their launcher; install-docs fails fast. tui `c934d87`. install-docs: `src/docs.ts` replaces `adapters/`, `domain/` and `app/`; `--title` (default "Docs") and `--slug` (default first page) come from the launcher; unknown `--slug` exits 2. disk-plan: the mode page's "default" follows the `--mode` it was launched with; the LevitateOS-specific advice is gone.
  - Behaviour changes on purpose: without `--slug` the first nav page opens (was the profile's "installation"); the page list is titled by `--title` ("Installation Docs" was hard-coded); clipboard copies go through Ink's stdout and the note says they were sent via OSC 52 (success can't be detected).
  - Until F19: the live ISO still launches `docs-tui --slug installation` without `--title`, so it shows "Docs".
  - The mode cards themselves (C006/C007, still `cyan`/`gray` borders) are rewritten in F15.
- [x] F14 — Rust facts reach the TUIs only through `cargo xtask gen` (generator side). tools/recpart `a68e10c` (doc comments on `InstallMode` variants and `ApplyResult.warnings`, schemas regenerated), tui `677543c`, superproject commit "Facts owned by recpart…". The generator walks ts-rs dependencies from each root type, writes `src/generated/<tool>.ts` into every app whose manifest lists the tool, formats through the repo's oxfmt (so `gen --check` and `format:check` agree, verified), emits `DEFAULT_INSTALL_MODE`, `INSTALL_MODE_HELP`, `SLOT_HELP` and `readRecpartOutput`, and no longer emits the unused `HANDOFF_SCHEMA_VERSION`.
  - **Bug fixed:** a failing `recpart list-disks` showed its error document as a raw JSON blob; now its `observed (remediation)`.
  - Bonus: recpart's own `plan --help` now describes each `--mode`.
  - Left as is (see there): C011, C012, C014, C073–C077, C132, C137 need recab/recpart output changes while another session was rewriting those tools.
- [x] F15 — disk-plan as an explicit flow graph. tui `471a6d3`, contract `44362e5` (manifest `inspect.path`, named `inspect.walks` with env), superproject commit "disk-plan as an explicit flow graph…". disk-plan: `src/flow.ts`, `src/app/plan-runner.ts`, `src/presentation/{use-recpart.ts,pages.tsx,disk-plan-screen.tsx}`, `fixtures/recpart` + recorded JSON. Kit: `useAsyncTask` (A180/A181, deferred from F10). xtask: walks take env; a key that changes nothing on screen counts as settled.
  - **Bugs fixed:** Next on result-handoff landed on failure-diagnostics; Back from an apply failure landed on result-handoff; a stored failure outlived a successful re-run; the "exact command sequence" omitted list-disks and the real apply; non-rotational disks were labelled "ssd"; the empty-list message blamed `--disk`. Driver bug found by the walk: a keypress with no screen change made `inspect` wait until timeout.
  - Verified: `cargo xtask check tui` walks disk-plan through both apply outcomes (success → handoff, Next stays; failure → diagnostics, Back → apply-progress) against the fixture.
  - Unchanged on purpose: the next-step commands on the handoff page are still this app's text (C011, left as is under F14); generic vendor names are still hidden (C050).
- [x] F16 — recab-tui: results stay visible, status is a separate panel. tui `61445c5`.
  - **Bugs fixed:** the status refresh after a successful mutation overwrote its result (C079); the footer printed the literal "status-error" (R013).
  - Verified headlessly: pressing `c` logs "Commit active slot: recab commit (exit 97 …refused…)" while the status panel keeps its own error.
  - Note: another session edits recab and recab-tui's status rows (`d1f0b39`, the `committed`/`rollback` rows); `STATUS_ROWS` was left as they have it.
- [ ] F17 — Guest scripts are files, with one placeholder scheme that is checked.
- [ ] F18 — The install experience is declared once, in the variant's products.toml.
- [ ] F19 — One owner for the chain from login to install screen.
- [ ] F20 — The program that owns the terminal sets TERM, and nothing forces colour.
- [ ] F21 — Each live-ISO file has one writer, and init and distro differences come from config.
- [ ] F22 — ISO payload is built by recipes and one generic TUI-app action.
- [ ] F23 — The split pane is a robust terminal host.
- [ ] F24 — Install tests exercise the real launch, not a smoke stand-in.
