# G20 — private checkpoint control and task-event waiting

Head `7afa833d1babcc5dd62ef326f81c52835f9aa349`, tree `1d010ec50a9d95edab1ac96d888c8e6592420f90`, on G19
`a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e`. [Patch](patches/G20.patch):11 files,+563/-36.
Per-process source and selected ordinary verification are retained. Security,
thread-sharing, full CI and current-main independence remain pending.

`--control-fd` accepts one inherited Unix stream socket. The existing donor
protocol uses bounded GXS1 frames, strictly increasing u64 sequences, a u16 path
length1–512 and non-NUL path bytes. GXA1 acknowledges only after snapshot write,
sync, close and no-overwrite publication. Partial reads/replies persist in the
single event-loop owner; invalid requests, capture failure and peer loss stop
launch. The workload's existing bootstrap closes inherited descriptors above
stdio. Runtime FD-authority validation is deliberately unrun here.

Separate whole-diff self-review covered current argv/config/completion callers,
bootstrap descriptor cleanup, task birth/wait/death/teardown, control buffering,
readiness and archive lifetime. This is self-review, not independent review.

- **Repeated capture ownership corrected:** the G19 PID-only staging name would
  collide with an already prepared final export. Checked PID/serial names now
  isolate each capture. Capture owns its staging locally, while final export
  retains its existing global owner. Both reuse one write/sync/close/publication
  implementation and the same filesystem archive visitor.
- **Cleanup retained:** common launch completion closes control idempotently,
  including config rejection. Task teardown closes control and signal descriptors
  independently before export and always tears down storage. Error completion
  cancels pending final output through the existing owner.
- **Event ownership preserved:** after the initial tracee exec stop, the supervisor
  blocks SIGCHLD, restores its default disposition and creates private signalfd.
  wait4(WNOHANG) alone consumes statuses; poll waits for signal/control readiness
  without a timer tick. The blocked signal bridges the check/wait interval.
  Existing typed child birth, wait and cleanup logic is unchanged. G08's finite
  guest-readiness timer is not imported and must join only with its own capability.
- **Scope:** core/process policy, native supervisor, public descriptors, tracee
  transfer/mapping/storage backing, VFS and its shared path walker are unchanged.
  G07 thread/shared-argument checks and G13/G15/G18/G21/G22 joins stay assigned to
  their existing owners. No guest socket channels or consumer protocol is added.
- **Coverage wiring:** the introducing packet adds its Make target and Linux CI
  invocation using the existing storage fixture build. Static-and-unit still runs
  make check and make test-unit. Full negative/private-FD defaults are preserved.

Exact G19 rejects the valid inherited-control invocation with usage exit2.
Source and strict clean prerequisite-plus-patch replay pass make check test-unit
build and storage/directory/script/owned-exec fixture builds:92 units,34 VFS,
181 matching tracked paths. Ginux and all four fixture binaries match. Ordinary
runtime checks pass in both copies: two valid fragmented captures, byte/content
isolation after mutation, child-produced data, final export in the same directory,
normal peer cancellation and10000 task syscalls with idle control off/on and
inherited ignored SIGCHLD. Seven final-export/config/flush cases, two valid mapping,
six ELF,14 script and growth/bounds/quota controls pass too. Timing output is a
smoke observation, not a performance guarantee or race proof.

Malformed/replayed/oversized frames, FD visibility and private authority, hostile
paths, injected failures, inherited signal-mask/race behavior and thread sharing
remain with [DaybreakBlue](daybreakblue-handoff.md), with no cloud dispatch.
Independent review, full Linux suite, macOS/remote CI and fresh-main standalone
validation remain unrun. The G19 prerequisite is local; do not publish this stack.
Patch, draft, commands, logs, hashes and helpers are retained under
.artifacts/ginux-contribution-preparation/G20/. No real Ginux branch, index,
checkout/ref, remote text or maintainer message changed. F04 remains the review stop.
