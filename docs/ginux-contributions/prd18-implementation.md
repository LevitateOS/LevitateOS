# PRD18 — declared NEEDS implemented and locally verified

The recorded release decision includes the complete proposal. Implementation is
complete locally, with both later user decisions applied: complete outputs at
installation/installed reuse, typed subsets at intermediate phases; explicit
caller restrictions across the dependency graph. No PRD18 semantic decision or
recorded regression remains unresolved. This does not close PRD01 maintained
runtime delivery, PRD05 upstream/storage integration, PRD09 security acceptance or
PRD19 production/boot/install/A/B acceptance. Nothing was pushed or published.

## Supported contract

- Every executable API-2 recipe, including an extending child, owns literal
  `const NEEDS`. Read the unoptimized source AST without evaluating setup.
  Inputs use `file`, `directory`, `file?`, `directory?`; outputs use exact file/
  directory names. Quoted Rhai map keys support names such as `"package"` and
  `"source.git"`. Unknown fields, computed declarations, duplicate/unknown/
  missing/wrong-kind bindings and root symlinks fail before launch.
- Recipes own endpoints and memory/storage/time/network-byte limits. Defaults
  remain 2048 MiB, 16,384 storage objects, 3600 seconds and 8 GiB network bytes;
  an omitted network grants none. Storage bytes remain bounded by memory and
  metadata accounting. Public callers supply bindings and optional narrowing,
  rather than the old effective grant fields.
- Restrictions must first be valid lowerings/subsets for the named recipe.
  Across its transitive dependencies, numeric ceilings take the stricter bound
  and endpoints intersect each node's own declaration. No parent grant is lent;
  limits remain per instance, not an aggregate graph budget. Force flags affect
  the named recipe; target/jobs/defines remain orchestration/facts.
- Preflight the complete effective DAG, source closure and input bindings before
  its first domain. Dependencies execute as separate Ginux instances with
  separate state/files identities. Bind inputs with `DEP::NAME`. Published
  products become immutable consumer inputs under
  `RUNTIME.paths.dependencies/DEP/OUTPUT`; a `tools` directory contributes to
  the private tools prefix. Reuse the existing input snapshot and tree merge
  owners, preserving cross-product hardlinks and rootfs symlink bytes. Reject
  alias/product namespace overlap. An overriding child excludes superseded base
  dependencies. Manual checks, remove and cleanup act on the named recipe's
  retained files without running dependency installation/readiness; full graph
  declaration/source/input preflight still applies.
- Derive `[userspace.contract]` from a fixed build base plus effective network
  authority. Pin catalog **2** and both complete operation-table digests:
  empty connect `121cb495da506cad611b9d81dfb38eb15ff23e9c965d8d758f3a1c5076f04f79`,
  connect `4106370023c93de07ccc93b48b8883dc2e09186b1fa0ed1835e6d97ff7ccf314`.
  Listening is never granted; direct MMIO/DMA/raw packets are denied. Owned
  local-packet send/receive numbers do not lend TCP endpoint authority. Require
  `configured-network-audit=1` and `configured-denial-audit=1`; validate each
  actual rendered config, not only a declaration-only approximation. Contract
  identity participates in retention. **Transport 9** rejects the old
  coordinator/worker grant model before setup; API 2 remains the sole interface.
- `validate` checks graph/bindings/source closure and derived contracts;
  `inspect` resolves main/dependency authority; recipe/invocation `explain`
  traces actual rules and saved execution/caller restrictions; `diff` compares
  authority, including dependency grants/bindings, rather than hook text. These
  commands do not evaluate setup or start a workload. Metadata compilation does
  not resolve DNS/open the image; actual launch resolves/pins/validates endpoints.
- Save exact declaration/config, execution, pins and digest in private
  `authority.json`. Explanations survive later source edits. Ginux's finite
  endpoint-use and contract-denial summaries survive syscall trace truncation.
  Missing/truncated observations remain unknown, never an implicit allow or an
  unused-grant claim. Do not fabricate recipe phase correlation. Warn about
  unused effective endpoints only with complete observation.
- Validate every present intermediate output before checkpoint/publication;
  require the complete set at install/installed reuse. Check owned paths, names,
  kinds and ancestors. Validate the archive privately before final retention and
  publication. Preserve state serialization's existing owner and post-cleanup
  verification context. Coordinator completion metadata lives at
  `ctx.runtime.contract` / `ctx.runtime.network_usage`, preserving top-level
  recipe facts. Repair cannot change NEEDS/API/dependencies/base selection,
  including multiline declarations.

All production recipes, builder/variant/kernel/xtask/library/CLI callers,
examples and documentation are migrated directly. Variant TOML selects recipes
and binds paths; it no longer grants network/resources or duplicates preseed
output kinds. Builder consumers read `NEEDS.outputs`. Kernels have a normal
LevitateOS default; Acorn is selected explicitly by its builder. Source kernel
builds require image-provided tools: the historical DNF fallback has no declared
mirror grants and remains unsupported/open under PRD06. Kernel rebuilds and
general package solving are excluded release claims; no grant was invented.

## Exact sources

Local commits:

- Recipe `4ee73d7ae7973567e437f40a33c236ab98ebded1`.
- Builder `cc403cf1e477c74e8860fa477d4d9efba5844d3c`.
- Contract `1da4f83a7ef6508c6cfe77298df623e18e6b7780`.
- Ginux `/home/vince/levitate-development/ginux-prd18-audit`, clean at
  `e9b7ce0b40136f4a30892a072e28e65746a37e3b`: diagnostic commits `45ccd299`,
  `7990f1df`, plus the unchanged authorized PRD08 patch `4aa51c1d` cherry-picked
  as `e9b7ce0b` (range-diff equality). Frozen Daybreak/export/PRD08 donors are
  unchanged. Native policy remains separate; authorized configured clocks and
  zero-filled sysinfo/large image I/O are preserved. This composition does not
  close PRD05 or select maintained fork delivery.

Evidence: `.artifacts/ginux-contribution-preparation/prd18/`.
Final CLI/worker: `final-target/x86_64-unknown-linux-gnu/release/recipe`, SHA-256
`470e3bb2d8b8e0ce990f9d683724649073ada744377198d15d77a08d142c37f6`.
Ginux SHA-256:
`bd0c09928c886245fb9a531b4b2d69e39995e804e9f6f232dd25b390ef41d118`.
Image: `oci:recipe-compiler@sha256:c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e`,
from `.artifacts/ginux-env/prepared-image/rootfs.tar`; launch hashes actual bytes.
`final-source.json` records source/config hashes, commits and complete local
patches. These are local reproducible inputs, not a remotely obtainable release.

The original CLI was `72a96558…`, the revised CLI `771ba607…`, and the final CLI
`470e3bb2…`. Both older binaries stayed hash-checked unchanged while their runs
were active. Final runtime source differs from the 33-case candidate only by
exposing existing execution data in explanations; final controls verify those
fields. Later test-only corrections preserve the product checks below.

## Evidence and concrete finding dispositions

- `existing-regressions.log`: **124 pass, 13 fail**, terminal original 137-case
  run. It is retained as red evidence. Every failure has a corrected passing
  disposition below; it is not relabeled a green full-suite run.
- `accepted-runtime.log`: **30 pass, 3 fail** on revised sources. All ten other
  new NEEDS cases pass, including typed phase subsets/install/reuse, direct
  dependencies and immutable products, source-edit-independent explanation,
  inheritance/override, authority diff, namespace collision, unused grants and
  transport rejection. Its three failed fixtures are replaced by passing
  partial-frame/lifetime/transitive controls below. **All 11 NEEDS cases have
  passing receipts**, across this run and the final graph case.
- `numeric-audit-runtime.log`: **1/1 pass** on final CLI: real allowed TCP, own
  stricter dependency limits, graph-wide caps/intersections, immutable products,
  explicit `--no-network` on all three nodes, actual retained Ginux socket
  denials and explanations containing the exact saved execution. Latest E2E
  executable SHA-256:
  `a96e1a278c7da7ee653a592e01f23ce4e732f3b0f5f956e6fa19e67874503167`.
- `supported-lifetime-runtime.log`: **1/1 pass** on final CLI, active scratch
  leases, live coordinator before kill, all tracked tasks gone within 5 seconds,
  subsequent abandoned-scratch reclamation and unrelated sentinel preservation.
- `metadata-correction-runtime.log`: **1/1 pass** on final CLI, real extraction,
  checkpoints/reuse, publication/reimport and EROFS deployment. Preserves modes,
  uid/gid, hardlinks, rootfs links and capabilities; altered privileged payload
  metadata still fails. `target-correction-runtime.log`: selected-target/image
  separation passes. `final-oracles-runtime.log`: partial-frame control passes
  with the same 20-second post-frame bound; its graph reader failure is retained.
- `final-contract-runtime.log`: **1 pass, 2 fail**. Static non-evaluating commands
  and saved/resolved explanation fields pass on final CLI. Its lifetime fixture
  and numeric-field assertion failures are explicitly replaced by the two final
  passing controls, not counted as functional passes themselves.
- `final-production-declarations.json`: **13/13** actual production/example
  declarations compile through final CLI and Ginux. Caller compilation and both
  migrated variant loaders pass (`accepted-callers-check.log`,
  `variant-{levitate,acorn}-status.*`). These are not product scenario passes.
- Exact Ginux `make check`, eleven-mode real TCP checks (including audit-cap,
  disabled audit and denial after trace exhaustion), and image I/O/query checks
  pass: `ginux-joined-{check,network-runtime,image-runtime}.log`.

All 13 original failures have concrete closure:

| Original case(s) | Correction and passing receipt |
| --- | --- |
| Context fresh paths/named outputs | Reconstruct both declared outputs on installed reuse; `accepted-runtime.log`. |
| Three kernel readiness/remove/cleanup cases | Do not require dependency installation for manual operations; all three pass in `accepted-runtime.log`, preserving release/module/config checks and immutable inputs. |
| Protected-check repair | Preserve the readiness-check diagnostic while extending declaration protection; real provider rejection and multiline authority cases pass in `accepted-runtime.log`. |
| Finalization and launcher failure after worker success | Fault wrappers delegate administrative validation, injecting only actual launch completion/status faults; both pass in `accepted-runtime.log`. |
| Partial launcher frame | Delegate validation; measure from actual partial-frame emission rather than image preparation. Same 20s bound rejects 60s pipe holders; passing `final-oracles-runtime.log`. |
| Coordinator death | Avoid unnecessary transfer scratch for dependency-free calls; allow image bootstrap separately from the unchanged 5s reaping bound. Repeated supported 1s sleeps hold the worker; `supported-lifetime-runtime.log`. |
| Two publication cases | Assert the earlier NEEDS/private-archive validation owner, preserving host sentinel, prior products, path/link rejection and no failed publication; both pass in `accepted-runtime.log`. |
| Selected target missing product | Declare the target input so the original image-versus-target check runs; `target-correction-runtime.log`. |
| Target metadata | Quote reserved Rhai key `"package"`, retaining the input ABI; full `metadata-correction-runtime.log` passes. |

Additional implementation findings are closed: use Read for the catalog clock
selector; retain contract denials beyond the trace cap; preserve shared hardlinks
through the existing snapshot owner; exclude overridden base dependencies; ignore
dependency hook-only authority diffs; preserve top-level recipe facts. Their real
NEEDS/Ginux checks pass. Fixture failures were also corrected explicitly: ignore
log lock files when reading invocation directories; compare actual numeric syscall
41 fields; do not use 600s sleep requests above Ginux's existing 300s bound. No
public limit or native policy was widened. The empty exact-selector run, initial
sandbox startup failure, diagnostic red runs and deliberately canceled build are
retained; none counts as a pass. No Recipe unit tests were added or run.

## Reproduction and release boundary

From this checkout, rebuild the recorded Recipe source without replacing other
running candidates:

```sh
cargo build --offline --release --target x86_64-unknown-linux-gnu \
  --target-dir .artifacts/ginux-contribution-preparation/prd18/final-target \
  -p levitate-recipe --bin recipe
cargo test --offline -p levitate-recipe --test e2e --no-run
cargo check --offline -p distro-builder -p distro-contract -p levitate-xtask
```

Run the real CLI/Ginux cases with the same launcher/worker/image combination:

```sh
sudo -n env \
  GINUX_BIN=/home/vince/levitate-development/ginux-prd18-audit/target/x86_64-unknown-linux-gnu/release/ginux \
  GINUX_ROOTFS=/home/vince/Projects/LevitateOS/.artifacts/ginux-env/prepared-image/rootfs.tar \
  GINUX_ROOT=oci:recipe-compiler@sha256:c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e \
  RECIPE_TEST_BIN=/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd18/final-target/x86_64-unknown-linux-gnu/release/recipe \
  RECIPE_WORKER_BIN=/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd18/final-target/x86_64-unknown-linux-gnu/release/recipe \
  target/debug/deps/e2e-4004a17346684ed4 needs:: --test-threads=2
```

Select the exact lifetime/metadata names above with `--exact --nocapture` for
those controls. These require the host chroot/ptrace/mount environment. Inspect
actual case counts and terminal statuses; manifests identify historical runs.
Preserve meaningful oracles when refreshing after upstream/storage changes.

Local implementation is complete. The next release action is the already-open
PRD01 source decision and PRD05 required integration joins, followed by their
exact-source revalidation. No source-fork/publication choice, final-source
security acceptance, kernel rebuild, real full ISO or VM acceptance is implied.
Upstream Ginux takes precedence.

Active skill: `hdae-development`, `.agents/skills/hdae-development/SKILL.md`.
Reload it fully, applicable instructions, its reference and this checkpoint after
context shift. Repository instructions override process bureaucracy. The optional
extra graph bookkeeping rejected under `overengineering-interrupt` was not added.
The root queue and interface contain earlier unrelated edits; their combined
working changes remain unstaged. The focused record, design, variants and source
pins are committed separately, preserving unrelated root staging. Queue/Daybreak
staged binary diff SHA-256 remains
`f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.
