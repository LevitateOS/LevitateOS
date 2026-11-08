# G29 — configured TCP integration (partial)

Current release contract: [PRD08](prd08-public-contracts.md) records the user's
configured downstream approval and explicit limits, independently of upstream
holds. The candidate/evidence below remains historical; its security, current-
main delivery and final-source acceptance are not upgraded by that decision.

Head `26fbfef2ecc4d3192ff605b23c9a4bb02e781171`, tree `089f3da2f0ebf26996ce3635cfea1c1c90142dc9`, on local prerequisite
composition `2ba7c8942e1530048904db5a63c5b9de2e1c0298` of G08a `fe27c693` and G28 `0a3cf5f5`.
[Patch](patches/G29.patch):12 files,+1022/-13, adapted from66de2880 to current
per-process owners. This is local preparation, not a standalone-ready proposal
or a completed network/security/consumer acceptance.

The config owner adds at most8 numeric IPv4/IPv6 destinations, live connection
limit1–16 and a positive cumulative sent/received byte budget. Old configs and
normalization remain unchanged when fields are absent. Legacy mediated metadata
without grants still parses but configured launch requires the complete explicit
policy. No DNS or arbitrary native socket fallback is added. Core/native rules
stay unchanged. NetworkBackend/endpoint/budget contract still needs upstream review.

Public socket allocation reserves a slot before acquiring the owned object;
errors release it. The existing160-entry reference owner now includes16 network
objects after its144 existing objects. Dup/fork share a socket and its private
transport; final close/exec/death use existing releases. Network syscalls on
non-sockets return ENOTSOCK. Poll uses the same registered owner and external1 ms
scheduler tick, with one stored timeout per parked call. No G07 shared-table,
G02 guest clock, G26 resource, archive or native scalar-clamp ancestry is imported.
G25's canonical IoVectors helper is reused verbatim without regular-file/pwrite
prerequisites. Deduplicate that same helper when the owners join at F01/F02.

Separate whole-diff self-review read all12 changed files, public reservation/
close/alias/fork/finish/death owners, G08a retries and timer loop, strict vector
copies, G28 API/error/byte accounting, full default fixtures, docs and CI. This
is self-review, not an independent or adversarial assessment.

Findings and corrections:

- **High, corrected in source — receive copyout/data loss:** donor preflight
  copyin/writeback is removed. Each socket retains at most4096 received bytes,
  charges actual transport once, advances only by the copied prefix and keeps
  the tail for aliases until delivered/closed. Staged data supplies POLLIN even
  after transport budget exhaustion; zero/EOF does not touch destination bytes.
  Protected/partial/mutable destination verification is explicitly unrun.
- **High, corrected in source — consumed connection error:** completion now
  checks transport readiness and stores persistent failure separately from the
  consumable SO_ERROR value. Poll, connect retry and getsockopt share this state;
  consuming the error cannot make the failed connection connected. Established
  transport errors still use native SO_ERROR. Refusal/error races remain unrun.
- **Medium, corrected — vector divergence and signal handling:** canonical16-
  vector checked aggregate/copy rules replace separate parsing. MSG_NOSIGNAL
  protects the supervisor; ordinary EPIPE writes signal the guest unless the
  guest send requests suppression. Shared-thread signal targeting and adversarial
  copy/prefix verification remain F02/DaybreakBlue joins.
- **Medium, corrected — timeout retries:** ppoll snapshots its initial timeout
  and poll layout, rather than rereading relative deadlines on every tick.
  Per-socket timeouts similarly freeze on first park. Arithmetic is checked.
  Null ppoll signal masks only, millisecond rounding/no timespec mutation and
  unfinished general signal restart/interruption are documented.
- **Coverage preserved:** every old unit and target remains. Donor default
  deny/quota/FD-inspection/kill/slot/off scenarios remain enabled. Optional mode
  selection accurately names only checks actually run. New ordinary-hold and
  budget modes verify normal peer EOF and exactly8 transferred bytes without
  exercising a denied operation. The valid receive-vector fixture respects short
  stream reads. Essential new fixture build/run wiring stays in this packet.

Source and strict clean base-plus-patch replay pass make check build plus both
network/poll fixture builds. All173 tracked paths match, Ginux and fixture hashes
match. Both copies pass84 selected routine units (all82 preexisting composition
units plus2 ordinary G28 TCP units). Three new policy/security tests remain
default-enabled but unrun: tcp_grants_are_numeric_bounded_and_complete and the
2 G28 donor tests. Unfiltered make test-unit/full Linux CI/macOS/remote CI is
pending, not green. Default Linux total would be87.

Five ordinary TCP modes in each copy cover blocking and nonblocking connect,
SO_ERROR success, poll/ppoll peer wakeup, NODELAY, socket names, valid readv/writev,
zero/short reads, alias/fork lifetime, half-close/EOF,50 ms receive timeout, normal
controlled close and exact byte-budget use. Five existing ordinary poll controls
also pass. Actual trusted host curl fetches the exact payload on controlled
IPv4 HTTP, IPv6 HTTP and TLS with an explicit private test CA in source/clean.
Minimal host binaries/DSOs and their hashes are recorded in consumer-provenance;
no external network or negative TLS probes ran. TLS verification is not disabled.

Actual Git ls-remote was attempted: it exits128 before transport at unsupported
getcwd (syscall79). Its earlier harness attempts exceeded the8-argument config
bound or omitted explicitly assigned null; corrected harness evidence is retained.
Do not substitute curl or historical integrated Git logs for this failed actual
Git acceptance. G14 and remaining combined consumer capabilities must join and
Git/Recipe must rerun at F02/F03. No cwd bypass or unrelated filesystem import was
added. Exact prerequisite rejects these newly introduced valid grant config
fields with unknown-field/exit2 before workload; final candidate accepts them.

Remaining joins: G07 shared FD/call/thread owners and the unresolved shared-path
security hold; G20 event wait with a finite tick while network/timers wait;
G24 socket Stat nlink/time fields; G25 helper deduplication; actual HTTP/TLS/Git
acquisition and complete Recipe consumer closure on fresh combined revisions.
No live sockets are exported/restored. Private descriptor/CLOEXEC/grant authority,
refused endpoints, protected buffers, error/race/cancel/revoke/exhaustion and fault
injection remain [DaybreakBlue](daybreakblue-handoff.md) work. Full default network
security modes and unfiltered CI remain unrun. No cloud dispatch occurred.

Patch/draft/helpers/commands/results/failures/hashes are retained under
.artifacts/ginux-contribution-preparation/G29/. No branches, refs, checkouts,
real indexes, remote text or publication were changed. F04 is the user inspection
stop before any push, PR or maintainer message.
