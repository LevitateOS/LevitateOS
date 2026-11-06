# G06a — process-table orphan adoption

Standalone local commit `b2347523d460b8cbeee2284fd1089509bf9f920c`, sole parent
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
