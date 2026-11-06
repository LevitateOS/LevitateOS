# G08b — local packet pairs, FIONBIO and nested stream capacity

Head `bc34cf66d8472b786ba10455352a91c6a7081bb9`, tree `f3318e93f85f0841e448a8b91f52275eeb7d14ed`, directly on G08a
`fe27c693bf2629166ee82da2efefb62b298bb567`. [Patch](patches/G08b.patch):5 files,+320/-53.
Sources75acd2be,3b27b370,e954010d. This finishes retaining the useful original G08
channel source gap found during F01; it does not complete actual compiler/thread/
security/policy/consumer acceptance or create a standalone-ready PR.

Existing LaunchChannels buffers now optionally store bounded length-prefixed
packet messages, with a paired endpoint pointing at its opposite receive owner.
Each pair consumes2 buffers; max message4096 bytes, short receive discards the
remaining message, zero-length messages retain their boundary. Public aliases/
fork share existing object references, and final endpoint close releases its
read and opposite write direction. No names, listening, host socket, descriptor
transfer or local shutdown is added. MSG_DONTWAIT/NOSIGNAL stay bounded to their
supported sends/receives; native core/bootstrap poll and guest clocks are untouched.

FIONBIO copies a complete integer before mutating shared open status, preserves
APPEND/access bits and per-public-descriptor CLOEXEC, and updates all aliases of
that endpoint. Capacity increases8→16 channel buffers; reference indices derive
from32 channel handle slots so assigned devices follow rather than overlap them.
The current per-process G08a table needs no G07 DescriptorCall/thread ancestry.
G25 vector support and G29 local/network syscall/reference composition stay F02
joins; no unrelated owners are imported solely to match old donor structure.

Separate full-diff self-review read all5 changed files and existing reservation,
copyout rollback, alias/fork/close/reference, buffer lifecycle/readiness, scheduler
retry, strict copy, metadata flags and existing CI/runner owners. The complete
corrected diff was reviewed after authoring. This is self-review, not independent
or adversarial review.

- **Medium, reproduced/corrected — metadata validation regression:** the initial
  donor port accepted only0x1000 for descriptor stat flags, dropping G08a's current
  valid fstatat/statx masks. Restored the exact existing validation owner and added
  valid0x1100 fstatat/0x5000 statx calls to ordinary and full fixtures. The preserved
  intermediate a4a7111e fails that ordinary check at163; corrected head passes.
- **Dependency correction:** packet/FIONBIO changes use channel/public FD owners,
  not inherent shared-thread or guest-clock behavior. Donor's G01 bootstrap-poll
  prefix was excluded; current launch tick/readiness remains G08a's owner.
- **Coverage retained:** every original/default protected-copy, unsupported socket,
  full-buffer, alias/capacity, SIGPIPE and proc/cancellation case stays enabled.
  Default runner includes packet mode. Explicit ordinary modes name their actual
  coverage; they do not call protected pointers or proc/FD authority probes.
  Existing Make/CI pipe and poll fixture targets already cover these files.

Source and strict clean base-plus-patch replay pass make check test-unit build
build-launch-pipe-fixture build-launch-poll-fixture:82 units,170 matched tracked
paths, identical Ginux and fixtures. Three ordinary channel modes in each copy
cover message boundaries/truncation, duplex and zero messages, socket metadata,
readable+HUP, aliases, repeated normal reclamation, real blocking fork peers,
FIONBIO shared status/CLOEXEC and a blocked reader wakeup, accepted metadata flags,
and12 simultaneously used pipes with child/grandchild streams and final EOF.
Five existing ordinary poll/pipe/alias/timer controls also pass in both copies.

Same final valid fixtures on exact G08a fail packet creation103, FIONBIO152 and
stream allocation170 at the old8-buffer capacity. The initial fixture compiler
caught one misleading-indentation line; an initial child helper also duplicated
the parent's PASS output. Both fixture issues were corrected before freezing.
Initial and superseded-head logs remain clearly labeled. These are functional
checks, not actual nested compiler/application closure.

Remaining: actual compiler streams with current thread/filesystem/vector owners
at F02/F03; G07 shared-path/thread hold; G25 byte-stream vector merge; G29 send/
recv demultiplexing must preserve local packet sockets and place network refs
after the enlarged channel+device range; later Stat links/times must follow their
owners. Full/default security modes, Linux/macOS remote CI, native readiness/
clock policy and fresh current-main independence remain pending. General signal
interruption/restart remains unfinished as documented by G08a.

[DaybreakBlue](daybreakblue-handoff.md) owns protected/mutable packet/IOCTL buffers,
partial copies, invalid flags/families/oversized messages, allocation rollback,
private/native FD authority, stale/reused handles, hostile capacity/race/cancel/
thread/SIGPIPE probes. Default tests are preserved and those probes remain unrun.
No cloud dispatch, publication or maintainer contact occurred.

Patch/draft/helpers/results/failures/hashes live in
.artifacts/ginux-contribution-preparation/G08b/. Real Ginux branch/ref/checkouts/
indexes and remote human text remain intact. F04 is the user inspection stop.
