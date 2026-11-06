# DaybreakBlue security validation handoff

Requested by the user on 2026-11-06. Intended recipient: DaybreakBlue, the
monitored cloud blue-team agent specializing in low-level security. This is a
written handoff only; no agent has been contacted or dispatched. Keep this record
in LevitateOS; it is not Ginux contribution text.

## Boundary and status

Continue ordinary implementation, source review, compilation, unit checks and
functional runtime scenarios locally. Do not execute exploit-like security
validation here. Preserve upstream tests. Select ordinary functional scenarios
instead of automatically rerunning suites containing adversarial probes. Record
excluded checks as pending; do not substitute historical results for fresh
security acceptance. Earlier validation evidence remains historical evidence
performed before this instruction, not a new DaybreakBlue result.

## Pending validation

- G06b `8c783e717b839b0e511820e89d29840cae0f3c0c`: independently validate protected,
  unmapped and cross-page clone child-TID/wait-status writes, rollback after a
  partial write, and wait faults still consuming a child. Relevant owners:
  `tracee_io.rs`, `launch_tasks.rs`; fixture sources and historical reproductions
  are retained with G06b. Prior local results predate this handoff; no new run.
- G03c–G06b: independently validate assignment confinement, native/image/owned
  interpreter authority and resistance to path/descriptor authority bypass.
  Include kernel-selected shebang/PT_INTERP targets and execveat image FDs.
  Relevant owners: `fs_hooks.rs`, `launch.rs`, `launch_tasks.rs`, `descriptors.rs`.
  Preserve useful allowed-path controls alongside denial probes. No new run.
- G07 (in preparation): validate clone TID copyout protections, public/private FD
  authority across shared tables, concurrent transaction isolation and teardown
  races. Pin the final candidate revision before running; source review and
  ordinary thread-lifecycle checks do not satisfy this adversarial acceptance.

Run only against authorized disposable local fixtures/images in the monitored
blue-team environment. Return exact revision, commands, observations, exit
statuses and any unverified checks. Do not publish PRs or contact maintainers.

## G07 blocking source finding

An unfinished disposable G07 export already existed when this continuation
started: `/tmp/ginux-contribution-preparation/G07/source`. Its prior
`path-race-initial.log` records a failing `x-path` scenario (fixture exit99).
The tracee's shared pathname can change between owner selection and native
pathname capture; a concurrent thread can therefore reach an image path hidden
by the configured memory assignment. This old reproduction was read, not rerun
under the new instruction. No G07 candidate is approved or publication-ready.

DaybreakBlue should investigate and correct stable pathname/metadata capture
across threads and other shared writable mappings, then validate confinement
against the exact corrected revision. Stack scratch alone is shared writable
tracee memory and is not a complete solution. Include SIGCHLD disposition
validation and kernel-consumed indirect arguments in the review. Do not broaden
public Linux support or weaken assignment authority to make tests green.
The existing `qfault` clone-TID and `x-path` security scenarios remain in source
but are excluded from this agent's new functional-only runs. Existing review
artifacts and logs remain available; no new exploitation or security probing
has been performed here. A written handoff is not evidence of dispatch or fix.

## G09 security validation pending

- G09a d29381c7: review malformed image offsets and backend failure contracts;
  checked copy-up extent arithmetic is implemented, but adversarial index probes
  have not been run here. Ordinary VFS data/truncation/rollback units pass.
- G09b 297ad8dd: validate anonymous mmap/mremap rollback and indirect memory-copy
  protections under adversarial workloads, plus inherited namespace confinement.
  Ordinary growth, quota, zero-fill and forked-alias checks pass in both source
  and clean replay. Full storage suite retains the older confinement/protected
  buffer scenarios; these were not rerun by this agent. The configured thread
  snapshot is deliberately excluded from this candidate.
