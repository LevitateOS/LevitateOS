# PRD13 — CI and portability

**Open: final source delivery and final-source hosted Linux acceptance remain pending.**
The user indefinitely deferred all macOS work. macOS CI/portability is excluded
from release acceptance; existing upstream CI definitions remain preserved.

The user authorized read-only GitHub/Actions inspection and standard Rust/crate
fetches, approved explicitly opt-in Recipe publication, and then chose **keep
local; wait for maintained source** for the exact proposed Ginux CI push/run.
No push, dispatch, PR, upstream comment or release occurred. Do not repeat that
permission request without changed direction. PRD01 source adoption and PRD05's
actual upstream merges remain separate open dependencies.

## Current candidate verification (2026-11-09)

The maintained-own-repository decision and baseline delivery were resolved under
PRD14, including upstream Gate 11. Subsequent corrections remain local; no new
push/hosted-job permission is inferred. The active [PRD15/19 checkpoint](prd15-product-contract.md)
owns current source/combination scheduling. Older results below are historical.

Recipe `176395aeaa5bb8999c1a6289d18e53c8012ecdf9` passes independent disposable
stable 1.99.1 unlocked binary build/fmt/strict all-target Clippy and MSRV 1.93.0
unlocked check/build. Both also pass binary builds against the actual tracked
lock `48c776bd…`. Independently resolved `a4eda69a…` differs only in itoa
1.0.18→1.0.19; that lock is compatibility evidence, **not adopted**. Workspace
locks remain unchanged. Logs/commands/diff are in `recipe-current-checks.json`,
`recipe-tracked-lock-checks.json` and `recipe-lock-resolution.diff`.

Ginux b81f30b5 static checks and 147 unit tests passed. Its full real-XFS runtime
batch was **13/20 passed**, not a complete pass. The native finding is corrected
in **e4d95de10520aa8e91bc2ec69e64da004443e2da**, frozen launcher SHA-256
`21a54e60bd54a4dd2b07957cf2e139a268eb050cb73929e0b8666b31f60c2333`.
A userspace-SIGCHLD control fails on old fb87792d and passes on the correction;
full native controls pass on XFS. The existing pending-exit state now selects
PTRACE_SYSCALL versus PTRACE_CONT; no grants or native policy are widened.
Static checks, all 147 units (zero ignored) and all **20 existing Linux runtime
targets** pass on the corrected source using an empty **8 GiB tmpfs**, with
original deadlines/assertions. The tmpfs was unmounted and removed.

| Recorded failure | Current disposition |
| --- | --- |
| Native SIGCHLD “unexpected native syscall-exit stop” | Fixed with retained old-fail/new-pass regression and all native controls. Final-source security verification remains separate. |
| Storage namespace, 20 seconds | Diagnostic captures completed workload exit zero, zero tasks/handles, then host XFS fsync beyond the harness limit. Old binary passes the same case on tmpfs. Corrected full target passes on tmpfs and quiet XFS (5.32 s total), with original 20 s per-invocation deadline. |
| Restore big payload, 120 seconds | Full target passes on tmpfs with all >2 GiB byte/hash assertions. Quiet full XFS target also passes (30.41 s total) with original 120 s per-invocation deadline; no independent fsync trace for the original timeout. |
| Threads z-race, 10 seconds | Full target passes on tmpfs and quiet XFS with original per-invocation deadline. |
| Directories all-1, 20 seconds | Full target passes on tmpfs and quiet XFS with original per-invocation deadline. |
| Poll all, 10 seconds | Full poll/event target passes on tmpfs and quiet XFS with original deadlines. |
| Owned execution success, 10 seconds | Full target passes on tmpfs and quiet XFS with original per-invocation deadline. |

The exact failed-fixture process observation finds no surviving process after
teardown (`timeout-process-observation-v2.json`; v1 accidentally included its
observer shell and is explicitly corrected). This does not turn timed-out
cases into passes or establish all security lifetime criteria. All six complete quiet-XFS retests now pass with their original deadlines and
assertions, after task-owned heavy I/O finished (`ginux-quiet-xfs-retests.json`).
Initial failures remain retained as load-sensitive harness timing; only namespace
has a direct fsync trace. No timed-out run is counted as passed.

Exact source archive, commands, logs and receipts live at
`/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/prd15/current-ci/`.
The older fb87792d/467140e9 Recipe catalog finished **152/152 passed**, with no
ignored/filtered cases. The fresh optimized 21a54e60/e7073fc1 combination is now
executing under `prd15/release-pipeline-progress.json`: library/image controls,
16 documented commands and all six consumers pass; new products, complete e2e
and guest acceptance remain in progress. Their results are not mixed with older
bytes. Release builds/harnesses use exact 176395a, tracked lock and Rust 1.99.0.
No Recipe unit tests, hosted Linux success, macOS work or security closure is
claimed. Hosted Linux remains pending authorization on the final reviewable source.

## Historical local sources and outcomes

| Component | Source or identity | Outcome |
| --- | --- | --- |
| Recipe production correction | `bd3d851e6c3bf666adc246762d8e3ef2b954bf8c` | Complete single-combination 152/152 runtime run passes |
| Recipe CI/docs candidate | `d4801b112326c3793efb0b902aade27f03ca014e` | Clean local commit; production/test/runner bytes equal the successful run; final package verifies |
| Ginux CI candidate | `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f` | Static, complete unit, build/fixtures and all required Linux CI runtime checks pass |
| Ginux production owner | `f7f0d14cc5abc1874b755e6990e4bd87bfacf050` | Only three workflow lines added by `a498e4f6`; executable unchanged |
| Ordinary compiler/acquisition image | `c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e` | Exact retained eight-RPM image, 1,276,850,176 bytes |
| CLI and matching worker | `b8cbbfa5acfce37226022e8c47d35be75a1af66fac9b6e8a35f0a134856e0a63` | Same executable for both roles throughout all 152 cases |
| Launcher | `0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3` | Unchanged throughout the run |

Evidence root: `.artifacts/ginux-contribution-preparation/prd13/`.
The final manifest verifies all **17,856 regular files** (18 recorded symlinks,
zero special files); manifest SHA-256 is
`9e12028126222982dd52fdbeccb8044605bfe04f963a4784fe643f63f04e4b07`.
`runtime-tmpfs/full/combination.json`, `identity.json`, per-case receipts/logs,
`results.json` and `integrity.json` record all 150 e2e, one public-library and one
image-producer case. Every case passes; none is ignored, interrupted or omitted.
Final integrity reports no changed source/artifact and no owned process/scratch
leaks. All 20,716 evidence entries were persisted with byte/metadata/link
verification and the bounded tmpfs was unmounted; evidence paths are unchanged. The public-library case uses the deliberate spoofed worker environment.
The full run includes all nine product-dependent cases with the actual builder
binary from `cc403cf1e477c74e8860fa477d4d9efba5844d3c` and frozen producer recipes
from `4fec6bae95bd4c355ce71ec58410f08e1852d531`; it does not use the mutable
current builder recipes. No Recipe unit tests or unit doctests were run/added.

`final-source-reconciliation.json` compares the executed source with the final
committed export: all production/test/runner bytes match. Only bootstrap README
text changed after successful runtime integrity verification. All 152 passes
come from the same binaries/image/harness run; no earlier-revision passes are
combined into this result. This remains provisional local evidence, not acceptance
of an unchosen maintained delivery source.

## Recipe build and dependency evidence

Current stable is Rust **1.99.1**; declared MSRV is **1.93**, checked with 1.93.0.
Independent disposable exports outside the workspace each started without a lock:

- Stable `cargo build --verbose`: passes with fresh registry resolution.
- MSRV `cargo check --verbose`: passes with independent fresh resolution.
- Both generated lock SHA-256 values are
  `48c776bdeb1cf63378e7132fd1dfbc1a819db09a95a23fa3f7ecb1b676e85f65`.
- Stable standalone fmt and strict Clippy, including all three integration
  targets: pass. Workspace locked build and MSRV locked check pass; standalone
  MSRV locked build also passes. The workspace lock was not deleted/replaced.
- Standalone package verification passes again on final `d4801b1` source:
  `cargo +stable package --locked`, 145 packaged files. Crate SHA-256:
  `997f0d4aefb671037f165385e6d54ed7bbf6c70e27b52891a8846eed27ccf373`.

Logs, both generated locks, toolchain identities, package receipts, source
archives and exact retained binaries are alongside the runtime receipts. The
initial whole-workspace fmt attempt encountered unrelated in-progress builder
formatting; package/standalone fmt passes and those unrelated edits were preserved.

## Findings and disposition

| Finding | Disposition and verification |
| --- | --- |
| Strict Clippy failures | Fixed nested formatting, redundant collection/borrowing and the single-element integration-test loop, preserving all assertions. Boxed the private worker request variant; serde encoding and wire version are unchanged. Strict all-integration Clippy and complete actual CLI/worker execution pass. |
| Manifest omitted the existing MSRV contract | Added `rust-version = "1.93"`; independent unlocked stable/MSRV resolution and actual builds/checks pass. |
| Shared Recipe CI never ran managed regressions | Added a required disposable Linux managed job, built CLI/matching worker and all three integration harnesses, exact source/image input checks, integration Clippy, per-case bounded execution, cleanup checks, and uploaded receipts. The locally executed full-products entry point passes all 152 cases. Hosted execution remains unrun. |
| Ginux CI omitted existing image-I/O boundary checks | Added the existing fixture build/runtime targets and a manual validation trigger. Image-I/O runtime checks pass without changing their assertions. |
| Recipe push/manual CI implicitly published releases | User approved explicit opt-in. Publication now requires manual `publish_release: true` and successful shared/managed CI. Default is false; version choice reaches its producer. Shared workflow definitions are pinned to inspected superproject `eea341c64a065d8465727919a72bc592878ea70b`. YAML/input/dependency checks pass; no remote publication was attempted. |
| Four Ginux unit socket binds denied in sandbox | Environment failure retained in `ginux-unit.log`. Complete unmodified unit target passes outside that restriction, including all socket cases. No tests removed or bypassed. |
| Initial threaded `X-scratch` case exceeded ten-second harness deadline during compiler load | Initial failure preserved. Focused case and complete thread target both pass with original deadline/assertions; exact owned-path process inspection finds no survivors. No source behavior defect demonstrated. This is bounded CI evidence, not exhaustive security race acceptance. |
| Bootstrap instructions mislabeled the current image | Retained `c089…` provenance contains eight RPMs; current manifest contains twelve, adding four ISO tools. Instructions now select actual producer provenance digest. This run still uses the recorded eight-RPM image; no new image's runtime acceptance is implied. |

The standalone CI job runs **143 cases**: 141 e2e plus library/image. Its explicit
nine exclusions are the `kernel_acquisition`, `kernel_recipes`, `preseed_recipes`
and `target_metadata` product modules. They require actual superproject producer
inputs; they are not counted as standalone passes. `--full-products` excludes
nothing, and all nine are included in the successful local 152-case result.
See Recipe [CI contract](../../tools/recipe/docs/ci.md) and its tracked runner.

Ginux passes `make check`, all eight complete `make test-unit` packages
(**138 tests, zero ignored**), every build/fixture target in the selected Linux
workflow (plus image-I/O), all 19 root launch/storage/checkpoint/restore/device/
pipe/FD/task/thread/directory/exec/native/Gate1/poll/owned-exec/script/network/
image-I/O targets, and ordinary-user `make test-process-native`.
`ginux-runtime-results.json` preserves exact commands' source/exit/timing; the
initial thread failure and confirmation are separate records. Native, clock and
configured authority policy were not widened for CI.

## Hosted receipts and remaining acceptance

The six inspected upstream PRs were open against main `53a54addea67969abaa240ea478def1b94130eee`
at metadata collection. Their older Linux jobs succeeded on synthetic merges:

| PR | Successful run | Actual checkout |
| --- | --- | --- |
| 232 | [42309743251](https://github.com/gmkurtzer/ginux/actions/runs/42309743251) | `42be013f0b7dbd429d5992cca190ee8a0799cb8b` |
| 233 | [42321453838](https://github.com/gmkurtzer/ginux/actions/runs/42321453838) | `434bb0c928b6595728fd554289cd94c45b4f3338` |
| 234 | [42309751643](https://github.com/gmkurtzer/ginux/actions/runs/42309751643) | `b67c644df05686e80a62ae5084e08b8d691a48fb` |
| 236 | [42361535214](https://github.com/gmkurtzer/ginux/actions/runs/42361535214) | `5fe9596b684ec333dbb387e31b75519ca0f6f78b` |
| 238 | [42326977073](https://github.com/gmkurtzer/ginux/actions/runs/42326977073) | `4746ae6cedcb07517b40777331cfaa4dd6e41e83` |
| 243 | [42364174394](https://github.com/gmkurtzer/ginux/actions/runs/42364174394) | `bb72c51c78d8299105ad1d13f82923a74dda4193` |

`hosted-summary.json` and raw API/job logs distinguish API head identity, actual
checkout, executed steps and runner labels. These runs used smaller runtime
workflows; they are not evidence for either local candidate or its complete CI.
Previously fetched historical logs remain preserved; no further macOS work is
required or pending.

Remaining requirements are explicit: agree/obtain maintained source under PRD01,
integrate required actual upstream merges under PRD05, provision exact maintained
Ginux source and produced image URL/SHA inputs, revalidate affected consumers on
that delivered combination, and obtain authorized final-source hosted Linux
results. A twelve-RPM image or changed source needs its own affected verification;
current green receipts cannot fill that gap. PRD09 security acceptance remains
separate; this task ran only the authorized tracked CI/runtime catalogs.

## Reproduction and resume

Use separate new exports outside the workspace for stable/MSRV unlocked checks;
never delete the workspace lock. Follow Recipe `docs/ci.md` for integration
compilation and runner arguments. For this full-products receipt, the exact
builder environment, binaries, source pins and every command are in
`runtime-tmpfs/full/combination.json` and per-case `receipt.json`. Ginux's selected
`.github/workflows/check.yml`, `ginux-build.log` and runtime/unit logs give its
complete commands, with root privileges confined to the tracked launch runners.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, required references,
applicable AGENTS and this checkpoint after context shifts. Root instructions
supersede skill process requirements. Resume from the current candidate section
and linked PRD15 checkpoint: finish the corrected optimized source combination,
then obtain the required exact-source hosted Linux approval.
Keep PRD13 unchecked. macOS is indefinitely deferred.
