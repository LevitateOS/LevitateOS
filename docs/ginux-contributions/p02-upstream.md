# Upstream refresh — P02

Checked 2026-11-06. Upstream main is `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
Git fetch refreshed cached refs only; integration HEAD remains `7cf2638a`.
[Exact remote snapshots](p02-upstream.json) preserve metadata and policy comments.

## Completed and existing contributions

- #153 and #154 are merged. Their restored tests remain required; their historical
  discussions were reread without editing any remote text.
- #216, #217 and #218 are all merged, at `2a82f61e`, `6fab4895` and `64a3e3f7`.
  Do not propose those changes again. The latter two merged after the integration's
  last upstream reconciliation.
- #221 is merged: `d0d0a5a1` adds Linux-like wait4 copyout failure handling and new
  process-native regressions. It must be preserved when preparing new candidates.
- Current fork has only five remote branches: main and the bounded-console and
  three VFS contribution branches. Their exact heads are in the snapshot; older
  prototype branch refs are no longer advertised remotely. Existing local refs,
  worktrees, stashes and reachable history remain available for P03.

## Open PR overlap

- [#207](https://github.com/gmkurtzer/ginux/pull/207), our bounded console polling,
  remains open at `e7f5f552`; its review explicitly requests holding that head
  pending the #179 policy decision. G01 must not become a duplicate PR or modify it.
- [#224](https://github.com/gmkurtzer/ginux/pull/224) fixes clone tid copyout faults.
  Preserve its ownership; assess overlap with G06/G07, not a new copy of this fix.
- [#226](https://github.com/gmkurtzer/ginux/pull/226) fixes tracee killing and setup
  teardown when ptrace requests are denied. Assess failure-cleanup overlap with G06.
- The open-PR list returned three entries, below the default page capacity; none
  were omitted by that listing. Their descriptions and current comments were read.

## Policy still unresolved

- [#179](https://github.com/gmkurtzer/ginux/issues/179#issuecomment-6661708444): hold
  process-local syscall expansion and virtual identity choices. G01/G02/G14/G27 and
  relevant G05/G08 surfaces require an explicit policy disposition before delivery.
  Existing local experiments can be audited, but are not evidence of upstream consent.
- [#178](https://github.com/gmkurtzer/ginux/issues/178#issuecomment-6661706860) and
  [#163](https://github.com/gmkurtzer/ginux/issues/163#issuecomment-6658892629):
  read/write/getrandom clamping remains a policy choice. G25 needs a separate
  disposition for mediated image/stdio transfer changes and owned-file I/O. The
  comment specifically asks to keep writev independently reviewable.
- [#219](https://github.com/gmkurtzer/ginux/issues/219#issuecomment-6672163838):
  sysinfo exposure/emulation is held for a policy decision. Do not silently fold
  it into G27's uname/configured identity proposal.
- [#222](https://github.com/gmkurtzer/ginux/issues/222#issuecomment-6677739555):
  release overflow checks are undecided. Check our arithmetic directly; do not
  silently change the global release profile or claim upstream decided this issue.
- #223/#225 describe the already-open #224/#226 fixes; no comments were present.
  New networking/storage/resource policy must also be assessed against actual
  documentation; no existing approval was inferred from silence or local success.

## Current required CI

`static-and-unit`: macOS15 Intel, rustfmt/clippy and x86-64 Linux target, then
`make check` and `make test-unit`.

`launch-integration`: Ubuntu24.04, build main/launch/process/gate1 fixtures;
`sudo make test-launch`, `sudo make test-native-supervision`,
`make test-process-native`, `sudo make test-gate1-setup`.

All of these must survive extracted changes. Future added fixtures belong with
their feature. This task only read workflow commands; no new build/CI run is claimed.

P02 complete. Next: P03's semantic accounting of each local/fork-only change.
