# PRD04 F-01: backing-close correction verified; commit-point decision open

The frozen candidate `5449e2a184bb1147cf618b140a29196b74206b13` and its binaries
and failed Daybreak receipts remain unchanged. This corrective source is local,
not a standalone current-main contribution or accepted security closure. The
user authorizes local work and prohibits upstream PR publication; upstream
Ginux takes precedence when its foundation changes land.

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

**Open contract decision:** after successful link publication, an injected close
error on either inode/parent reference reports supervisor-error but leaves the
valid final archive. The follow-up preserved this existing behavior. It differs
from the corrected pre-publication backing-close failure. The user has been
asked whether publication is the commit point (late cleanup error remains
reported and callers must reject non-success), or these errors also require
absence of the final archive and further publication/protocol work. Do not
silently weaken the earlier no-accepted-export criterion or mark PRD04 complete
before resolving this boundary. The original F-01 symptom is verified corrected;
overall export acceptance and PRD04 remain open for the decision.

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
result/audit hashes and per-control findings. PRD04 F-01 and PRD07's export wave
remain open until the authorized results establish their disposition. This
focused request does not reopen unrelated items 1–4 or assert all PRD09 schedules.
