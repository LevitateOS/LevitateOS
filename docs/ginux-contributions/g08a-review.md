# G08a — minimal per-process public readiness prerequisite

Head `fe27c693bf2629166ee82da2efefb62b298bb567`, tree `73f3e8adbe874a711c1e6777cb5e39cce94532d2`, on G06b `8c783e717b839b0e511820e89d29840cae0f3c0c`.
[Patch](patches/G08a.patch):12 files,+434/-10. Extracted from210a8cca while tracing
G29's actual readiness dependency. This is partial G08 preparation, not a new PR
or completion of socket-pair/thread/readiness policy acceptance.

Existing descriptor/backends own all queries. A bounded128-byte poll layout holds
up to16 entries and snapshots once across parked retries. Pipes supply live
buffer/peer status, assigned files/null immediate readiness, console aliases only
the inherited0–2 OFDs. Image FDs are unsupported rather than applied to matching
host numbers. Error/HUP bits ignore the requested mask; output writes only revents.
Timeout0–300000 ms and negative infinite waits use checked internal monotonic
arithmetic; finite/console waits use the donor's1 ms nonbusy sleep tick. Other
owned waits wake on actual object/death progress. General nonfatal signal
interruption/restart is explicitly unfinished.

The current G06b table is per-process, so poll state belongs there without G07's
shared-FD/thread transaction architecture. Fork initializes fresh call state;
finish/death clears it. Typed death accounting remains the owner if a stopped
caller disappears during retry/resume. No guest clock/core-native rule, socket
pair, FIONBIO, G28/TCP or unrelated filesystem/mapping behavior is imported.
G07 shared-table pinning/call ownership and G20 event-wait composition remain F02
joins. #179/#207 policy is unchanged and still holds publication.

Separate full-diff self-review read all12 changed files and the current public
FD alias/fork/close/finish owners, inherited console assumptions, backend readiness,
strict copies, timer/death/wake paths, complete fixture/default cases and CI.
This is self-review, not independent or adversarial review.

- **Dependency correction:** G29 needs readiness, not every G07/thread or G08
  socket-pair ancestor. This coherent prerequisite is retained separately on
  the existing G06b owner; no public stack is proposed.
- **Snapshot correction:** parked queries retain the initial descriptor/event
  layout instead of rereading mutable guest arrays on every retry. Race/fault
  validation remains with DaybreakBlue; no adversarial reproducer ran here.
- **Arithmetic correction:** internal nanosecond conversion/addition is checked,
  without changing global release overflow policy or granting guest clocks.
- **Coverage preserved:** donor full/default invalid-pointer/authority/kill cases
  remain in the new poll target; existing tests/targets are untouched. Essential
  fixture build/run wiring is included in Linux CI. Local mode selection reports
  only the ordinary scenarios actually run.

Source and strict clean base-plus-patch replay pass make check test-unit build
build-launch-poll-fixture:82 units,170 tracked paths matched, identical Ginux/
fixture hashes. Five ordinary modes in both copies cover live pipe/memory/null/
console-alias mixed queries, preserved input fields, duplicate writer lifetime,
buffered data with HUP, EOF, blocked read/write peer wakeups and60 ms timers.
Real fork/wait/death paths are exercised by those peer cases. Exact G06b fails
first ordinary pipe readiness and exits71. No protected arrays, foreign FD probes,
proc/FD inspection or hostile races ran in these selected checks.

[DaybreakBlue](daybreakblue-handoff.md) owns protected/mutable arrays, partial
copyout, unknown/native authority, layout/FD reuse and shared-thread races,
cancellation/interruption/rollback and failure/clock boundary probes. Full/default
Linux runtime, macOS/remote CI, independent review, thread/event-owner joins,
#179/#207 disposition and current-main independence remain pending. G08's original
socket-pair/FIONBIO/nested compiler streams and G29 network/HTTP/TLS/Git acceptance
remain open. No cloud dispatch occurred.

Patch/draft/commands/receipts/hashes/helpers live in
.artifacts/ginux-contribution-preparation/G08a/. Real Ginux branches/indexes/refs/
checkouts and remote human text remain intact. Nothing published; F04 remains the
user inspection boundary.
