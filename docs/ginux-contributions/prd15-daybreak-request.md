# PRD15 configured runtime and paired Recipe verification

PRD15/19 remains open. Please verify this exact candidate in the authorized
Daybreak context and return source-bound receipts, failures and unverified cases.
This request grants no security execution permission to the product-build chat.
No host clock changes, publication or upstream communication is requested.

## Exact candidate

- Ginux: `/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11`, clean
  `3cd1e2de49cfc16c0d22804849a82a5d24cf22d8`, six local commits over maintained
  `28f4368f6fd70d0e485386892a08d6734b5988dd` (including upstream Gate 11).
- Launcher:
  `/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd15/bin/ginux-opath-1`,
  SHA-256 `fb87792d5b4363ca669d20935ce6f3d3f6d08fb97ee0cdc74e60bc324c200e6e`.
  The clean exact-commit release rebuild matches these bytes.
- Recipe: `/home/vince/Projects/LevitateOS/tools/recipe`, clean
  `e28a60ab05393325a95dee14e2a4a7361e598aa5`; CLI and worker both
  `.artifacts/ginux-contribution-preparation/prd15/bin/recipe-subset-1`, SHA-256
  `5ca87251f17567c757499b097e710908f55a0bdcbb3ee98da70f5f918bdee8b3`.
- Acquisition image:
  `.artifacts/ginux-contribution-preparation/prd14/ginux-acquisition-image/rootfs.tar`,
  SHA-256 `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`;
  root identity `oci:recipe-acquisition@sha256:ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`.

Paths without an absolute prefix are relative to `/home/vince/Projects/LevitateOS`.
Candidate identities, commands, successful functional controls and retained
failures are under `.artifacts/ginux-contribution-preparation/prd15/` and in the
[product checkpoint](prd15-product-contract.md). Functional receipts are not
security acceptance. Builder/product acceptance and maintained-source integration
of these corrections remain separate pending work. Changed revisions or binaries
need an explicit affected-source reconciliation and verification.

## Required affected verification

1. **Owned FD names (S09-A/C).** Exact decimal `/proc/self/fd/N` is a bounded
   public name, not Linux's low-32-bit FD argument. Verify ownership through
   aliases, fork/exec, rename/unlink/replacement and slot reuse; truncation and
   copy faults; non-filesystem and private/inherited descriptors; foreign/proc
   paths and explicit deny precedence. No host path or proc-open authority.
2. **Own stack projection (S09-A/B/E/G).** Only the registered stopped task's
   canonical `[stack]` metadata may appear. Check scan/line bounds, task identity
   and concurrent lifecycle, unavailable/invalid metadata, immutable anonymous
   snapshots, copy failures, alias/fork/exec/close ownership, quotas and cleanup.
   No other mappings, host paths, mutable namespace/export entries, reopen,
   mapping or metadata mutation. Broader proc paths remain denied.
3. **Metadata O_PATH references (S09-A/C/E).** Check the immutable descriptor
   kind through dup/fcntl/dup2/fork/exec, CLOEXEC and all current data/mapping/exec
   owners. Metadata queries and FD-name lookup have positive controls; data I/O,
   enumeration, reopen, status/mode mutation, fchdir and FD/relative execution
   must remain denied. A metadata dirfd cannot authorize relative data open or
   namespace mutation. Check supported flags versus unsupported O_NOFOLLOW and
   mutating combinations, symlink confinement, failed copies, closed/unlinked/
   reused references, existing filesystem/public budgets and lower limits.
   Contract denials must precede allocation/effects. Native policy is unchanged.
4. **Counters and epoll (S09-B/C/G).** Existing channel-object/public/TCP limits
   remain in force; the new watch table is bounded to 32. Check object identity,
   weak registration lifetime, alias and FD/object reuse, fork/shared tables,
   parked pins, cancellation/death/revoke, close and error cleanup. Verify
   scalar-counter overflow/backpressure/semaphore/nonblocking semantics,
   successful-transfer-only notifications (including undrained EPOLLET and
   aliases), copy-fault preservation and no delivery consumption on failed
   copyout. Verify edge/level/oneshot/rearm/fairness, packet/pipe/TCP ownership,
   actual TCP RDHUP, unsupported files/nesting, maxevents bounds and timeouts.
   General temporary signal masks/restart semantics are excluded: pwait admits
   only null or the equal current 8-byte mask; interrupted admitted futex
   ERESTART_RESTARTBLOCK becomes EINTR; syscall 219 stays ungranted.
5. **Paired Recipe endpoint selection (S09-C/H).** On clean e28a60ab, verify
   deterministic bounded selection of more than eight resolved addresses,
   explicit numeric pins, host coverage and failure when it cannot fit. Only
   the same selected set may appear in immutable hosts, effective grants and
   authority receipts. Preserve graph-wide caller narrowing, independent
   dependency authority, TLS verification, no workload DNS and public limits;
   malformed/mismatched/unselected endpoint cases must not gain authority.
6. **Joined-source and consumer preservation.** Reconcile the affected
   [S09-A–H criteria](prd09-security-acceptance.md) against this joined source,
   retaining outstanding T-01/T-02/T-03, shared-task and persistence criteria
   rather than promoting old selected negatives to acceptance. Check that new
   public references do not defeat private transfer/close ownership, stable
   argument/copy protection, checkpoint/export/error ordering, Recipe scratch
   leases or rejection of non-successful launcher completion. The agreed
   export link commit point still applies. Report every remaining criterion
   explicitly; an unaffected historical pass needs an exact-source rationale.

Existing functional reproduction owners include `make build-launch-fd-fixture
build-launch-poll-fixture`, `make test-launch-fds test-launch-poll`, and the
task's `hoisted-local.rhai` plus `distro/builder/recipes/tui-binary.rhai`.
`run-opath.py` selects the exact launcher/CLI/worker/image and hashes them before
and after each invocation. Recipe operations require whole-coordinator bootstrap
privileges. Do not wrap only the launcher or weaken locks, linker threading,
metadata checks, package verification, isolation or resource limits.

Return exact source/tree/binary/image identities, commands and environment,
artifacts/traces and hashes, positive and denied controls, repetitions or schedule
coverage, concrete findings and every failed/unrun criterion. A written handoff,
old receipt or selected stochastic negative cannot close acceptance.
