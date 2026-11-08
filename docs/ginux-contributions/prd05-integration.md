# PRD05 — waiting for actual upstream merges

**Incomplete; integration has not begun.** Read-only GitHub refresh on
2026-11-08 at 07:10:27 UTC finds upstream main still at
[`53a54addea67969abaa240ea478def1b94130eee`](https://github.com/gmkurtzer/ginux/commit/53a54addea67969abaa240ea478def1b94130eee).
All six tracked prerequisites report `state=open`, `merged=false` and
`merged_at=null`. Required merged storage source does not exist on that main.
The user explicitly requires waiting in this situation, without substituting
local rehearsals or unrelated checks.

## Exact upstream evidence

| Required change | Current head | Merge state |
| --- | --- | --- |
| [#232 shared VFS open descriptions](https://github.com/gmkurtzer/ginux/pull/232) | `5e79a17286f793b3e7c9301b7653d43abe3a75eb` | Open, unmerged |
| [#233 bootstrap mkdirat](https://github.com/gmkurtzer/ginux/pull/233) | `f751cd6f04f7d95487b0fba5e1c01b6540ab43a7` | Open, unmerged |
| [#234 orphan adoption](https://github.com/gmkurtzer/ginux/pull/234) | `78a534c1dcff70207900013381ff857b5687a66e` | Open, unmerged |
| [#236 inherited-directory confinement](https://github.com/gmkurtzer/ginux/pull/236) | `1f43d7fa1d2ea30975629356492e640e095b4d4e` | Open, unmerged |
| [#238 signal-driven IO refusal](https://github.com/gmkurtzer/ginux/pull/238) | `efaeeca84829e0c232709da8fcf91d5248952e32` | Open, unmerged |
| [#243 contract enforcement/audit](https://github.com/gmkurtzer/ginux/pull/243) | `067d992f29d11f7b79e78753258721fc94a2f2c3` | Open, unmerged |

Every PR still carries the maintainer's 2026-11-07 foundation-review response;
no additional changes are requested. Non-null API `merge_commit_sha` values do
not establish a merge while the PRs remain open with `merged=false`.
None of the six published heads is an ancestor of the observed main. Ancestry
alone would not exclude a squash/equivalent implementation; here main is also
unchanged from the fully accounted source. Its tree contains neither
`crates/ginux-init/src/launch_paths.rs` nor `launch_storage.rs`.

The [PRD03 feature](prd03-storage-review.md) at `9802ef03` and the separate
zero-fill/configured-catalog contributions remain local. Their [dependency
chain](prd07-delivery-waves.md) is not fulfilled by prospective merge trees.
#234 is needed for the eventual process integration, separately from the storage
compile chain. No required PR is treated as merged because it is mergeable,
reviewed or locally green.

Machine receipts are in
`.artifacts/ginux-contribution-preparation/prd05/upstream-merge-receipt.json`
(GitHub main/PR metadata, heads, merge flags/timestamps and maintainer comments)
and `local-state.json` (local pins, cleanliness, ancestry/path checks and staged
state hash). GitHub was accessed through the connected read-only tools; no
upstream push, comment or PR change was made.

## Actual replacement disposition in this task

**No local fix, implementation hunk or test is replaced or retired.** There is
no newly merged upstream behavior owner to adopt. A final semantic replacement
map and integrated revision therefore remain **not performed**, rather than
being inferred from the older candidates.

The [PRD06 per-hunk/test accounting](prd06-reconciliation.md) and
[test retention record](prd06-test-retention.md) remain the preservation inputs,
not an accepted PRD05 replacement map. In particular:

| Owner boundary after a real merge | Present disposition |
| --- | --- |
| Shared offsets/append/open descriptions and #232 tests | Retain local behavior and all distinct tests; no actual upstream replacement yet. |
| Bootstrap syscall, inherited directories and signal-driven IO (#233/#236/#238) | Retain existing corrections and their regressions; no actual upstream replacement yet. |
| Resolved contract, deny precedence, operation classification and audit (#243/PRD02) | Retain configured catalog/enforcement and later owner-aware read/readv corrections; no replacement by the simpler register-only draft. |
| Orphan accounting (#234/T-03) | Retain all six incoming exact cases, both native scenarios and later aggregate tests; prospective local composition is not an upstream merge. |
| Typed `VFS`/`launch_paths`/`launch_storage` versus local `fs_hooks`/`storage_memory` | Actual owner migration pending. Preserve later descriptor/task/exec/map/archive corrections until each has an explicit verified destination. |
| T-01 mmap offset, T-02 full 9,000-byte copy-up/rollback, R-02 scratch leases | Required final-join work remains open; an old checkbox or local draft does not close it. |
| PRD04 backing/export corrections and PRD08 configured contracts/clocks | Preserve their separate exact-source fixes/evidence. They require reconciliation on the eventual combined revision; PRD04 export acceptance remains open. |

Current clean local integration remains
`/home/vince/levitate-development/ginux-recipe-integration` at
`5300f5ef9a8281f4f10f5519d5eb761654e04829`; clean Recipe remains
`b41443880fe585a27bc799d80382c9ff5b07047b`. The clean checked-out `tools/ginux`
remains `2eac10bf8c9e7a7e72e1ae94990aa2425b2f3ba0`. None was changed or
advertised as the merged storage integration. The PRD07 local rehearsals remain
separate, source-bound preparation evidence.

## Resume condition and remaining acceptance

Wait for actual required upstream merges or the owner's replacement foundation.
Then inspect actual merged commit/tree contents and behavior, including squash
or redesigned equivalents, before choosing local replacements. Upstream Ginux
takes precedence; the earlier draft's APIs/architecture are not fixed decisions.

Only then construct the coherent local integration, record each removed hunk/test
and its exact replacement (or explicit retained correction), migrate storage
contracts as required and run affected Ginux storage/FD/thread/path/exec/map/
export/checkpoint/restore checks plus Recipe consumers on that exact revision.
Preserve later consumer corrections and distinct tests. Final security work
continues only through its authorized context. PRD02–PRD04 dependencies still
apply; this absence-of-merges result does not waive them.

No Ginux or Recipe build/runtime checks were run for PRD05: there is no resulting
merged-source candidate to validate. No unrelated extraction, source integration,
submodule update or upstream publication was substituted. PRD05 stays unchecked.
No additional user decision is needed to identify this blocker.

Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; full reload after a context shift is
required. Current task: PRD05, blocked on actual upstream source. Exact next
action: refresh required merge state after upstream changes, then inspect the
actual source before resuming semantic integration. No recurring poll or
external dispatch is scheduled.
