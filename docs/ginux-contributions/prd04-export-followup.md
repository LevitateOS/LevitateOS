# PRD04 F-01: correction verified and publication boundary agreed

The frozen candidate `5449e2a184bb1147cf618b140a29196b74206b13` and its binaries
and failed Daybreak receipts remain unchanged. This corrective source is local,
not a standalone current-main contribution or accepted security closure. The
user authorizes local work and prohibits upstream PR publication; upstream
Ginux takes precedence when its foundation changes land.

## Current request reconciled — 2026-11-09

The requested new revision already exists as **2e88c9ad**, directly over frozen
**5449e2a1**, and the requested focused Daybreak retest already passed on its
exact **576ee03c…** launcher. Fresh validation rechecked all 313 retained evidence
hashes and rebuilt the same launcher from the same clean tree. Frozen HEAD,
tree, tracked status and 95bc570e… launcher are unchanged before/after. The
complete three-file +54/-11 correction below is the actual implementation;
no no-op revision is needed merely to change a commit hash.

`make check`, `make test-unit`, build/fixture, all eight ordinary storage/export
modes, ordinary checkpoint and restore pass again. Exact commands, exits,
source/tree/binary/fixture hashes and the retained launcher are in
`.artifacts/ginux-contribution-preparation/prd04/export-renewal-2026-11-09/validation.json`.
`acceptance-reconciliation.json` and ACCEPTANCE-SHA256SUMS bind this fresh check
to the existing focused return. The old validation workflow label requesting a
renewal remains historical; its actual test results are unchanged. The duplicate
renewal question is withdrawn: no additional return is required for unchanged
source/bytes and the already-agreed contract. This does not represent old tests
as new execution or claim security probes ran in this general context.

**The user's open-until-retest-passes condition is satisfied by the actual
verified focused return. Scoped PRD04 closure stands.** Final-source PRD09,
real close-time ownership and wider schedules/delivery remain open. Preserve
both exact candidates and every original failed/passing receipt.

Active skill: hdae-development at
`.agents/skills/hdae-development/SKILL.md`; reload it, required reference,
applicable instructions and this checkpoint after context recovery. Exact next
action: retain this scoped correction/verification in the downstream source and
reconcile any actual new finding or changed final-source owner through PRD09.
No push, upstream PR/comment, tag, release or new source-delivery choice occurred.

## Returned focused Daybreak verification

The user returned the [focused receipts](/home/vince/levitate-development/ginux-prd04-export/.artifacts/prd04/daybreak-f01-2026-11-08/receipts.md)
on the exact source/tree/binary listed below. All 313 files in its SHA256SUMS
verify; manifest SHA-256:
`02a095dea2a5dc4b940579069d0acb1b0d010cbf4645ea404bd9d16f3c8564af`.
The initial verification command used the worktree instead of the evidence
directory as cwd and could not find its relative paths; the corrected cwd
verification passes. Both logs are retained under PRD07 artifacts.

Frozen source still publishes a valid 2,560-byte archive on the injected final
backing-close failure. Corrected source returns structured supervisor-error,
settles tasks/references and leaves **no final archive**. Workload-time final
unlinked close likewise prevents guest continuation and export; a synchronized
checkpoint request during the delayed fault receives no acknowledgment and
produces no snapshot. Dual backing/reference-close injection preserves the
primary backing error. Ordinary exports, child quiescence, times, two acknowledged
checkpoints and complete restore/alias/5,000-file/>2 GiB payload controls pass.

Write/fsync/writer-close faults leave no final archive. Prelink parent mutation
does not publish. Postlink pathname replacement reports failure while preserving
the pinned archive and the host replacement marker; no unsafe host unlink is
inferred as an acceptable rollback. All these are selected exact-source results;
synthetic skipped closes do not establish real Linux error-time FD lifetime.

**Resolved user decision, 2026-11-08:** link publication is the commit point.
A late inode/parent reference-close error may leave the intact final archive but
must report supervisor-error; consumers must reject non-success. This preserves
the user's earlier reporting and cancellation-error decisions. Pre-publication
backing-close failure still requires absence, as the focused receipts prove.
No unsafe pathname rollback or silent error suppression is authorized. The
existing corrected source matches this decision; no further publication/protocol
rewrite is required for the recorded symptom. Scoped PRD04 failure closure is
complete; recomposition/final security acceptance remains [PRD09](prd09-security-acceptance.md),
and PRD07 delivery/publication remains separate.

## Exact candidate and ordinary validation

- Clean worktree: `/home/vince/levitate-development/ginux-prd04-export`.
- Revision: `2e88c9ad89c487d3fc7c29ba35efba3911587737`, directly over frozen
  `5449e2a184bb1147cf618b140a29196b74206b13`.
- Tree: `ab48409d6b8b86866d64f3f1eb7d3ba9d52563ca`.
- Launcher: `target/x86_64-unknown-linux-gnu/release/ginux`, SHA-256
  `576ee03c8d3a7db19a057306d8223841bc536cbd51b91087fe1e7c2e57f616ed`.
- [Complete three-file +54/-11 diff](patches/PRD04-export-publication.patch).

`make check`, `make test-unit`, `make build build-launch-storage-fixture` and
eight existing ordinary storage/export modes passed on this exact clean source:
`flush export export-wait failed-export child-export invalid-export-config
bad-export-config ordinary-times`. Existing ordinary checkpoint and restore
runners also passed. Per-command SHA/exit/log receipts and binary hash are in
`.artifacts/ginux-contribution-preparation/prd07/export-followup/`; reproducible
runner: `../run-export-followup.py`. No Daybreak fault injection was run here.

## Ownership and error ordering

`storage_export::stage` writes, pads and syncs the unnamed archive, closes its
writer and returns a `PendingExport` carrying the exact inode/parent references.
Final task teardown first settles children, descriptors, images, control/events
and known cleanup errors, stages bytes while backing is live, and always tears
down the filesystem. It publishes the staged inode only if backing release
succeeds. On release failure it cancels the unnamed archive and returns the
original error. A prior staging/settlement error still tears down backing and
cancels any reserved final export; the first failure stays authoritative.

The original pinned-inode/parent freshness checks, exclusive link publication
and reference-close error reporting are preserved. A pending archive is marked
must-use and each teardown match arm explicitly publishes/cancels it. Checkpoint
capture remains write/sync/writer-close followed by immediate publication while
the quiesced filesystem remains live. Its API and acknowledgment order do not
use the final-teardown deferral.

## Focused authorized Daybreak retest request (returned above)

Please verify this new revision and binary hash, using frozen `5449e2a1` as the
failed control. Preserve the old failed artifact and original six-item receipts.

1. Repeat item 5's final backing-close EIO with `--export-file`. Require exit 1,
   parseable `supervisor-error`, no panic, settled tasks/references and **absence
   of the requested final archive**. Keep exact injection/trace/hash receipts;
   a skipped synthetic close is not a real Linux close-lifetime proof.
2. Repeat workload-time unlinked final-close failure: no guest continuation,
   no accepted checkpoint/final export and structured completion. Distinguish
   the primary backing error from secondary archive-reference cleanup.
3. Recheck affected successful export/quiescence and ordinary checkpoint/restore
   controls; archive contents/metadata and result success must remain correct.
4. Recheck affected export write/sync/writer-close/reference-close, cancellation
   and pinned-inode/parent mutation controls from the existing authorized
   acceptance set. Report final-path presence separately from result status for
   every failure. Preserve existing contracts and identify any remaining finding;
   do not infer a pass from a structured error alone.

Return exact source/binary/runner/config/fixture identities, commands, exits,
result/audit hashes and per-control findings. The returned results above and approved commit point establish the scoped
PRD04 F-01 disposition. PRD07 delivery and final-source PRD09 remain open. This
focused request does not reopen unrelated items 1–4 or assert all PRD09 schedules.
