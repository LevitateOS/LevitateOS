# PRD13 — CI and portability

Status: in progress; required final-source hosted/macOS results and source delivery
are absent. Do not close PRD13 from local Linux results or older PR badges.

The user authorized reading existing GitHub/Actions metadata and standard Rust
toolchain/crate downloads. No push, hosted start/rerun, PR, comment or release is
authorized. The user separately approved making Recipe release publication
explicitly opt-in: pushes and ordinary manual CI dispatches validate only.
After reviewing the exact proposed Ginux CI-only push/run, the user chose
**keep local; wait for maintained source**. Do not push or dispatch either
candidate, and do not ask again for the same action without changed direction.

Initial source: Recipe `8db564c526c1baa302dbdea0f27e247c91f7116c`;
Ginux `f7f0d14cc5abc1874b755e6990e4bd87bfacf050`. Local corrections are being
validated before recording final candidate revisions. PRD01's maintained source
and PRD05's required upstream merges remain open. All six required upstream PRs
(232, 233, 234, 236, 238, 243) are open against main `53a54addea67969abaa240ea478def1b94130eee`.

## Findings and current evidence

- Recipe strict Clippy exposed nested formatting, redundant collection/borrowing,
  a single-element test loop, and an oversized protocol enum. Corrections retain
  test assertions and box the request enum payload transparently to serde; the
  worker wire format/version is unchanged. Current stable strict Clippy passes
  for the library, binaries and all three integration targets. No Recipe unit
  tests were run or added.
- Recipe declares Rust 1.93 in its package manifest, matching its existing CI
  contract. Standalone unlocked resolution still needs independent stable/MSRV
  builds; the workspace lock must remain untouched.
- Ginux `make check` passes. The sandbox denied socket binding in four unit
  cases; the complete unmodified `make test-unit` passes outside that sandbox.
  This environment correction preserves all upstream tests.
- Ginux CI omitted its existing image-I/O boundary fixture. The local workflow
  now includes its build/runtime targets and a validation-only manual trigger.
- Existing six PR heads have successful macOS static/unit and Linux receipts,
  retained under `.artifacts/ginux-contribution-preparation/prd13/hosted/` with
  API metadata alongside. Those runs use older, smaller runtime definitions;
  they establish neither current local source nor complete runtime coverage.
- Actual local Recipe/Ginux heads have no hosted runs. Shared Recipe CI builds
  and lints but does not run its managed acceptance catalogs; this gap remains
  under active correction.

Artifacts and raw failures: `.artifacts/ginux-contribution-preparation/prd13/`.
The whole-workspace fmt attempt also encountered unrelated in-progress builder
formatting; package/standalone formatting is the relevant Recipe check.

## Verified local candidate (2026-11-08)

Ginux candidate `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f` adds only the
three workflow lines to the already tested `f7f0d14c` production source.
Launcher SHA-256 remains
`0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3`.
`make check`, all eight `make test-unit` packages, all required CI fixture/build
commands, all 19 root runtime targets, and the ordinary-user native process
runner pass. One initial `X-scratch` thread schedule exceeded its ten-second
harness deadline while compiler work was active. Both its focused rerun and the
complete thread target subsequently pass with the original deadline/assertions;
the initial failure remains in `ginux-test-launch-threads.log`, and confirmation
is in `ginux-test-launch-threads-confirm.log`. No Ginux assertion was weakened.

Recipe production correction `bd3d851e6c3bf666adc246762d8e3ef2b954bf8c` passes:
standalone current stable 1.99.1 unlocked build, strict standalone Clippy, fmt,
standalone Rust 1.93.0 independent unlocked check and locked build, workspace
locked build, and standalone `cargo package` including its verification build.
The independent fresh resolutions produce the same lock SHA-256
`48c776bdeb1cf63378e7132fd1dfbc1a819db09a95a23fa3f7ecb1b676e85f65`.
The disposable exports' complete production/test bytes were checked against
`bd3d851` before execution. Current stable full integration compilation passes;
the 152-case runtime run is still in progress and contributes no full-suite claim
until all cases and final integrity checks succeed.

The Ginux complete hosted candidate patch is retained as
`ginux-hosted-candidate.patch` under the artifact directory. Publishing it for CI
would expose the entire local integration stack against upstream main, not just
the three workflow lines. No source adoption or upstream publication is implied.

Existing hosted PR runs actually checked out GitHub's synthetic merge commits:

| PR | Run | Actual checkout |
| --- | --- | --- |
| 232 | 42309743251 | `42be013f0b7dbd429d5992cca190ee8a0799cb8b` |
| 233 | 42321453838 | `434bb0c928b6595728fd554289cd94c45b4f3338` |
| 234 | 42309751643 | `b67c644df05686e80a62ae5084e08b8d691a48fb` |
| 236 | 42361535214 | `5fe9596b684ec333dbb387e31b75519ca0f6f78b` |
| 238 | 42326977073 | `4746ae6cedcb07517b40777331cfaa4dd6e41e83` |
| 243 | 42364174394 | `bb72c51c78d8299105ad1d13f82923a74dda4193` |

The API head identities, executed checkout SHAs, job steps and runner labels are
separately recorded in `hosted-summary.json` and the twelve full raw job logs.
They must not be represented as hosted execution of current local candidates.

## Resume checkpoint

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, its required reference,
applicable AGENTS and this checkpoint after context shifts. Root instructions
take precedence over skill process requirements: no new ceremony or branches.
Next: execute disposable stable/MSRV unlocked builds; validate current Ginux
fixtures/runtime; implement and run meaningful pinned managed CI, then request
only the remote actions still necessary for exact-source acceptance. Never
publish or claim hosted success without permission and actual receipts.
