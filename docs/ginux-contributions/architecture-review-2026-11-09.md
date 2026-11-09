# Ginux downstream architecture review — 2026-11-09

Independent review of our Recipe-required Ginux extensions, with the redesign
the evidence justified. Local only: nothing was pushed, no PR was edited and
upstream was not contacted. This is development evidence, not security or
release acceptance.

## Sources

| Item | Revision |
| --- | --- |
| Upstream `gmkurtzer/ginux:main` (fetched at start) | `55b719db9c52889543fbb177fab4cf2b2f90955e` |
| Our maintained main, `origin/main` and local `main` | `ef1b538f940d8ae99b5e798c19cada9976381d86` |
| Baseline launcher built from it | SHA-256 `a7cc61f2…65df4` (matches the upstream-integration record) |
| Redesign commit | `1b7257be66ff3d5688b8e17424fa4cb670592f43` |
| Launcher built from it | SHA-256 `4c2b2ca254d7237f26eac006528bda341eb1d074c5445f9de4a0b15a731c998d` |
| Recipe | `9f16113807dad92b2b2edf21d4b0d78765ee505c`, CLI/worker `9f47819f…7056f`, image `ffe04d86…05c15` |

The commit sits on the detached HEAD of `tools/ginux`. Local `main` is checked
out, clean, in `/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11`
and was not moved. No upstream-owned behavior was changed; `main.rs` gains one
`mod` line and the redesign calls upstream's own `fatal_default_signal` and
`tracee_killed`.

Evidence: `.artifacts/ginux-architecture-review-2026-11-09/` (probe sources,
both launchers, every log named below, `SHA256SUMS`).

## Findings

### Confirmed, observed on the baseline launcher

| # | Finding | Evidence | Affected workload |
| --- | --- | --- | --- |
| 1 | A signal arriving while the supervisor replaces a syscall with an injected transaction stops the whole instance (`signal interrupted exec descriptor setup`). The README recorded this as deliberate fail-closed behavior for mappings; the trigger is any SIGCHLD. | `launch_tasks.rs` signal-stop branch; probe `m` 3/3; `map-signal` and `exec-signal` fail deterministically. | Any process that maps a storage file or execs while a child exits. |
| 2 | A task parked in an owned read, poll, epoll wait or `wait4` never receives a handled signal: it stays in its entry stop and the handler never runs. Documented as "unfinished", but the actual outcome is an indefinite hang. | No path in `wake` looked at pending signals; probes `s`/`r`/`p` time out 6/6; five `park-*` scenarios time out. | SIGCHLD self-pipe loops: libuv (CMake, Node), Python asyncio, GLib child watches. |
| 3 | Emulated `nanosleep` returns EINTR with no signal: the argument guard's `PTRACE_INTERRUPT` ends a poll quantum, and so does an ignored SIGCHLD. | `clock_hooks.rs` mapped every restart code to EINTR; probe `n` 20/20 sleeps fail; `sleep-peer`, `sleep-ignored` fail. | Any sleeper beside a peer making path calls, i.e. every parallel build. |
| 4 | A child the kernel created before its parent died has no owner. A fork child stays stopped in `unknown_stops` forever; the 33rd stops the instance, and that failure then hangs in teardown because its stop was consumed but never recorded. | Probe `k` at 2 and 4 vCPUs: 12/12 hang; live inspection showed the supervisor blocked in teardown's `wait4` with one stopped unrecorded tracee; `birth-orphan` fails deterministically. | A process killed while it forks. |
| 5 | A thread born into an exiting group stops the instance (`event from unregistered task`). | Probe `t`: 3 of 12 runs of 300 rounds at 2 and 4 vCPUs. Not reproducible on demand. | A multi-threaded program whose `exit_group` races thread creation. |

Findings 1–3 share one cause: nothing owned what a signal or a supervisor stop
means for a syscall the supervisor is holding. Findings 4–5 share another:
rollback of a creation was written three times and none covered a native child.

### Narrowed or rejected

- **Task lifecycle (hypothesis 1): confirmed for creation, not beyond.** The
  process-table extensions (`cancel_fork`, `cancel_thread`, `adopt_orphans`,
  `peek_wait`) are consistent and unit-tested. Wait faults and nonleader exec
  showed no defect. A second native reap of an adopted zombie arriving after
  its virtual reap would have been fatal by source reading; probe `a` never
  hit it in 4,500 baseline rounds. It is now harmless by the rule for unregistered
  terminal statuses, but it was not observed.
- **Injected transactions (hypothesis 2): narrowed.** Mapping, exec and path
  rewriting duplicate instruction checks and replay arithmetic, but the only
  ownership gap they share is signal interruption. Peer suspension is owned by
  the argument guard, temporary descriptors and scratch by each transaction,
  and failure reporting is explicit per site. A shared transaction framework
  would move code without fixing a defect, so none was built.
- **Descriptor and object lifetimes (hypothesis 3): rejected as a redesign.**
  Reference counts, generation keys, parked-call pins, final close and native
  alias retirement are consistent; no reachable lifetime defect was found. Raw
  handle numbers never reach a backend except through the one translation in
  `descriptors.rs`. Numeric ranges remain a structural weakness, but typed
  identities would have to span the storage backend, and the prepared storage
  redesign (`9802ef03`, not integrated: main still carries `fs_hooks.rs`)
  keeps numeric `FD_BASE` handles. That change belongs with its owner.
- **Networking, checkpoint/publication, identity views:** no redesign. These
  were surveyed from source by a subagent and not independently re-derived or
  exercised here beyond the existing suites. Recorded gaps: no SO_RCVTIMEO or
  close-while-parked test; a host that renames the export parent after a
  successful link leaves the published name behind.
- **Not changed:** parked retries still write a fresh audit entry each tick,
  which can exhaust the 64 KiB audit budget under a 1 ms poll tick.

## Redesign

**Held calls (`task_signals.rs`, new).** Invariant: while the supervisor holds
a task's syscall, an interruption has one owner and a defined outcome.

- Injected transactions are atomic to signals. The tracee's mask is blocked
  with `PTRACE_SETSIGMASK` exactly while supervisor-chosen syscalls replace
  its own, and settled at the only two places a stop can end: `resume` and
  `park`. The kernel delivers the signal at the original call's boundary. The
  fatal check stays as an assertion.
- A parked object wait is handed back to the kernel when a queued signal would
  run a handler or end the task. The call skips its entry and is replayed from
  its own instruction, as upstream's native loop does for `wait4`. At the
  signal-delivery stop the supervisor completes the call with a restart
  result and the kernel applies the handler's real flags. Ignored signals
  leave the call parked; ordering gates are never interrupted.
- A sleep continues across interrupted quanta and ends only at a handler's
  delivery stop.

**Creation (`launch_tasks.rs`).** Invariant: every native task the kernel
attaches is either published to one virtual task or held unclaimed, and an
unclaimed newborn is discharged exactly once.

- `cancel_creation` is the only rollback, for a failed clone, a cwd copy
  failure and a dead parent.
- A newborn is claimable only by a live task with an open reservation whose
  process or thread group matches the kernel's own parent/tgid record.
  Otherwise it is killed without ever running.
- A terminal status from a host the table does not hold is consumed. A
  consumed stop is killed before any failure, so teardown cannot wait on it.
- `birth` binds the native identity only after the birth stop.

## Checks and results

All on launcher `4c2b2ca2…` unless marked baseline.

| Check | Result |
| --- | --- |
| `make check` (fmt, strict clippy, name/terminology) | pass |
| `make test-unit` | 148 passed, 0 failed |
| `make test-contract test-ci-changes test-bench-compare` | pass |
| All 20 existing targets of `check.yml`'s launch-integration job | all exit 0 (`logs/summary.txt`) |
| `make test-launch-signals` (14 scenarios) | pass; 15/15 repeats after the runner fix below (3 full runs, 12 of the three stepped controls) |
| Same scenarios on the baseline | 10 of 14 fail as expected; `park-ignored` and `sleep-handled` pass (behavior preserved); the two bounded races passed at 120 rounds and are not a reliable baseline discriminator |
| Lifecycle probes, 300 rounds, 2 and 4 vCPUs | `k` 6/6, `t` 12/12, `a` 3/3 pass (baseline: `k` 0/12, `t` 9/12) |
| Six real Recipe consumers (SQLite, Cargo GNU/musl, Git HTTP/HTTPS, RPM, APK) | all exit 0 in 11–15 s each, comparable to the integration record |

Deterministic controls: the `park-*` scenarios release the child only after
the audit shows the parent parked; `map-signal`, `exec-signal` and
`birth-orphan` stop the supervisor, verify the tracee's state in `/proc`, and
act there. `birth-orphan` discards an attempt the supervisor already
published and repeats.

Failures during this work, kept in the logs: two `exec-signal` runs failed
because the first runner stopped the supervisor before it had processed the
handshake write; the runner now steps past it (`logs/signals-repeat.txt`).

## Unverified

- The thread-creation race (finding 5) has no deterministic control; its
  window is too short to step. It rests on the probe and the invariant.
- A signal already queued when a task parks is caught through the reap
  counter, not through a forced schedule.
- A signal sent to a parked task from outside the instance is not noticed
  until its next wake. Upstream's native loop has the same boundary.
- The six consumers pass but were not shown to reach the new paths.
- No CMake, libuv or `make -j` workload was run. `pselect`, `ppoll` with a
  mask and `tgkill` stay refused, so jobserver-style tools remain blocked for
  other reasons.
- No hosted CI, no full Recipe catalog, no product build, no security review.

## Changed documented behavior

The README and `docs/configurable-contract.md` said signal interruption of
emulated blocking calls was unfinished and that a signal during a mapping
stops the instance. Both statements are replaced. No refusal was relaxed:
temporary signal masks, kill's signal set and SIGCHLD ignore/NOCLDWAIT are
unchanged.

## Pull requests

No published PR (#207, #232–#234, #236, #238, #241–#248) contains the changed
code, so none needs revision. The unpublished waves do:

- Wave 3 (descriptors and task ownership, G06b/G07/G08a) must carry the
  creation reservation and the park rule with their introducing owners.
- Wave 4 (G10 owned ELF, G12 mappings) must carry the signal hold with the
  first injected transaction.
- Wave 7 (clock/readiness contract) must carry the sleep rule and the
  corrected documentation.
- `test-launch-signals` belongs with whichever wave first makes it pass.
