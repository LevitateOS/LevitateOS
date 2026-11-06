# G06a — process-table orphan adoption

Current retained local commit `4a62d81e3d81ede3b987d679333c39b62212c8bf`, sole
parent `ff926d0c228b71d55d0f2f0909dd86ec62197c8a`. The current-main refresh below
owns its fresh evidence and pending checks; earlier results remain historical.

Earlier standalone local commit `b2347523d460b8cbeee2284fd1089509bf9f920c`, sole parent
upstream `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
[Retained source patch](patches/G06a.patch). One file,+118 lines (23 implementation,
95 regression tests). No dependency on G03–G05 or the configured task runtime.

The existing process table leaves children attached to a parent after its final
thread exits. Adopt them into a live virtual init group at that point, on both
normal and signaled retirement. Preserve the parent while another thread runs;
handle a still-live init thread even when init's leader has exited. Leave children
unadopted if no live init group exists. Existing typed ChildStatus/WaitTarget API,
credentials, process groups, sessions and task generation semantics remain intact.

## Review and evidence

The entire final diff and ProcessTable fork/clone/exit/signaled/exec_group/wait/
shutdown callers were reviewed in a separate self-review pass. Independent reviewer
coverage is not claimed. The old ccd62ca1 donor only adopted at a leader's normal
exit; that would adopt too early with live threads and miss fatal-signal exits.
The retained fix uses current thread-group identity, checks final running-member
retirement and requires a live init group before changing only parent fields.

Three added cases fail on exact upstream (12 existing tests pass/3 new tests fail):
normal and signaled parent exits with child/thread views and later waits; a fork
from a parent thread that stays attached until its final group member exits;
and dead-init versus live-init-thread reaping. All15 process tests pass after
the correction. All existing tests are retained.

Both the candidate and a fresh main export with only this patch applied pass68
repository unit tests plus:

- `make check test-unit`
- `make build build-launch-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch test-native-supervision test-gate1-setup`
- `make test-process-native` outside the ptrace-restricted execution sandbox

Clean replay matches all150 tracked source paths; strict patch/whitespace checks
pass. No supervisor/core/runtime code or CI targets changed. Complete logs, exact
hashes, explicit runtime exit receipts and short draft are retained under
`.artifacts/ginux-contribution-preparation/G06a/`.

R01–R04 pass locally as a standalone candidate on the pinned upstream main.
macOS Actions/full Rocky VM checks are unrun. This is a process-table correction,
not proof of new configured child/exec functionality; the larger G06 runtime is
still pending. No real Ginux branch/index/worktree/remote was changed or published.

## Current-main refresh — 2026-11-06

The patch applies directly to observed main `8a475e8b`, with no other proposal
in its parent range. It still changes one process-table file: 23 implementation
lines and 95 regression-test lines. Upstream's new `set_process_group_for` method
and `parent_establishes_child_job_control_group` test remain intact and pass.

The complete final diff, current ProcessTable fork/clone/exit/signaled/exec/wait/
shutdown paths, job-control additions and native retirement callers were reread
in a separate self-review. The helper runs only after a successful normal or
signaled retirement, checks for another running group member and a live init
group, and changes only parent fields. Credentials, groups, sessions, generation
and wait/status APIs are unchanged. No new blocking source change was identified;
this is not independent reviewer or security acceptance.

All three adoption regressions fail on exact new upstream production code with
only those tests added. Candidate and independent clean-main-plus-patch replay
both pass:

- `make check test-unit build build-launch-fixture build-process-fixtures build-gate1-fixture`, including all 76 current repository unit tests and 16 process-package tests.
- Eight selected ordinary configured-launch/valid-wait controls per export.

All 187 tracked paths match the retained commit in source and clean exports.
Both exact binaries hash
`be2588cc6639c57dc19c996972d5052d6d26c6b1c0e1df0bca349821bfe03f34`.
Commands, failure logs, receipts, source exports, manifest, earlier patch and new
concise draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G06a/`.
Build targets are isolated; the local snapshot is unreferenced and no real
branch, shared index, checkout or remote text was changed.

All current upstream tests and CI wiring are preserved. Full default Linux
runtime jobs include authority/protected/fault scenarios and remain unrun under
the written security boundary; macOS/remote CI also remains unrun. R03 retains
those checks. The ordinary results do not add configured G07 thread support or
resolve its shared-argument hold, and do not authorize publication.

## PR231 latest-observed-base refresh — 2026-11-06

Head `4a62d81e3d81ede3b987d679333c39b62212c8bf`, tree
`bfd06357784e4f432ef291858a68b82e642e446a`, is directly on observed upstream
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a`. PR231 does not change the process
table; candidate process-file bytes match previous head01f22e06. Scope remains
one file,+118 (23 implementation/95 test lines). A separate self-review reread
the full diff, current fork/clone/normal and signaled retirement/exec/wait/shutdown
paths, job-control additions and native retirement callers. Only parent fields
change after final group retirement with live init; current typed/status APIs
and all upstream coverage remain. Independent review is not claimed.

The exactff926d0c production code with only the same regressions added yields
13 existing process passes and3 adoption failures. Source and strict clean replay
pass make check/test-unit/build and launch/process/setup fixture builds:86 units,
including16 process tests and new upstream contract/domain coverage. Eight ordinary
configured-launch/valid wait-status controls pass per export. All196 tracked
paths match the final head. Both binaries hash
`5aaaeaa085e0776a5aae04ee3f2089226b08dd30b8547288bdd7e8c059aaf375`.
Exact logs/receipts/draft/manifest/patch are in
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G06a/`.

Full default runtime/security suites, macOS and remote CI remain pending. Library
thread-group tests do not establish G07 configured thread support, contract
enforcement or security acceptance. No real checkout/index, branch, remote text
or publication changed.
