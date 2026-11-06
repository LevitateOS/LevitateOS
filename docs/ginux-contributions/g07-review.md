# G07 — thread extraction remains incomplete

Base `8c783e717b839b0e511820e89d29840cae0f3c0c`; unready local snapshot `5fa497c18ee306b1e4da36ffedde1d51b0e73263`, tree `2131675dbd8fd637f91fc08a1e16c9af07ff0ce4`.
This is a preserved work-in-progress, **not a reviewed or publication-ready
candidate**. The retained patch and manifest are under
`.artifacts/ginux-contribution-preparation/G07/`. No real branch/index changed.

## Ordinary work and evidence

Adapted the two historical thread donors onto G06b, preserving current typed
process APIs, strict word-copy owner, per-task pathname scratch and exec failure
restoration. Shared descriptor groups own their table once; each task owns its
syscall transaction and blocked-object pin. Existing partial work fixes a child
thread retiring before its creator receives clone's exit, and dup replacing an
unfinished native-open reservation. Historical failures and corrected repeats
remain under the disposable G07 directory.

Fresh `make check test-unit build build-launch-thread-fixture` passes (83 unit
tests). Explicitly selected ordinary runtime scenarios pass: basic identity,
pinned pipe lifetime, limits, concurrent native descriptor operations, fork from
a thread, leader exiting first, failed-clone rollback, short-lived thread reuse,
dup/open contention, signal teardown and supervisor-error teardown. Scenario
selection was added to the existing script; its default still retains all tests.
These checks do not establish security acceptance or complete R01–R04.

## Blocking work and next action

A previously recorded shared-path race remains unresolved. See the written
[DaybreakBlue handoff](daybreakblue-handoff.md). Do not rerun exploit-like probes
here or mark them green from historical evidence. DaybreakBlue must investigate
and correct stable kernel-argument authority, then return exact-revision security
results. qfault/x-path remain in source and are deliberately unrun this turn.
No full-suite or clean replay success is claimed. Preserve this snapshot while
continuing independent authorized work; G08 and G07-dependent joins stay held.

No publication, PR edits, maintainer contact or cloud dispatch occurred.
