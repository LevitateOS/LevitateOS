# PRD07 G06 — process-library prerequisites (local)

This is a standalone library extraction directly against current main, not the
configured child/exec implementation. It supplies two required APIs from donor
`7240bb21` without bringing in its older task loop or aggregate orphan tests.
Publication remains prohibited; upstream's foundation takes precedence.

## Exact source and complete diff

Clean worktree `/home/vince/levitate-development/ginux-prd07-process-api` at
`4690d87f425599ee4b5e7c10d0578784cd8c4242`, tree
`fc66ad0514fca60fa942da1026d90950793bb46f`, directly on current main
`53a54addea67969abaa240ea478def1b94130eee`.
[Complete one-file +98/-1 patch](patches/PRD07-process-api.patch), SHA-256
`8f526200b447144d3afdcbacff25cfa8e5e96af709a49be3f34d1df024ea96e0`.
Launcher SHA-256:
`39236fd577ff8c025d883427701e150cd64b64f763aa7a6736200ba1d7d94f52`.
No core, catalog, configured launch, native policy or public limit changes.

The complete donor library delta consists of `cancel_fork`, `peek_wait`, the
`wait` wrapper and two named tests. All are retained, with comments clarifying
the API boundaries. Other G06 donor implementation/fixtures remain required
with the task/exec owner; this extraction does not dispose of them as delivered.

## API contract and failure review

`cancel_fork(parent, child)` releases a supervisor's reserved birth **only before
the caller publishes it to execution**. It validates live parent/child identity,
matching parent process, a running process leader with no exec epoch, and no
thread/descendant state. Invalid ownership/state returns an error before clearing
anything. Cancellation frees the occupied slot but does not rewind PID identity
allocation. The API does not track whether the caller made a still-running child
runnable: respecting the prepublication boundary is a supervisor obligation,
which the later runtime join must verify.

`peek_wait` borrows immutably and returns the same typed exited/signaled outcome
and child selection as the existing `wait`, including group/thread-parent
selection and current errors. It never consumes the outcome. `wait` calls it,
then clears precisely that child's slot. Supported wait options/outcomes remain
unchanged; no stopped/continued-state support is added. The focused tests retain
cancellation rollback/invalid-owner/exec/thread/descendant checks, plus repeated
inspection and one-time consumption of a signaled child from a parent thread.

A direct API driver fails to compile against exact main with both methods
missing, then compiles/runs against the candidate. The base library/driver/log
and hashes are preserved. A direct `cargo test` diagnostic failed to link because
it inherited this repository's freestanding rust-lld flags; the repository's
existing `make test-unit` clears those flags and passes. This was an invocation
error, not a source defect or suppressed test.

The first three-way extraction conflicted at the test insertion point with
older aggregate orphan tests in the donor ancestry. Those are not silently
reintroduced. The exact two API tests were extracted into current main; all
current-main tests stay. The separate #234 join below retains its complete six
more precise orphan tests instead. Conflict snapshots remain diagnostic evidence.

## #234 compatibility and local integration

The library has no compile prerequisite on #234, and neither changes the other's
runtime functions. They both insert tests before the same existing test, so
there is a textual conflict. Resolved locally by retaining all six #234 cases
and both API cases, preserving every existing test body exactly. A machine
receipt records that comparison.

Clean compatibility worktree
`/home/vince/levitate-development/ginux-prd07-process-api-orphans` at
`20700e102ed76a6c352fec2cd6048e0d6de5007a`, tree
`7a9b5bea3890617eccaed68fc33fde97ba89d7d9`, is based on the earlier exact
current-main #234 composition `69de214f`. Its launcher SHA-256 is
`a9c8332ef992cc9276f6d829d12dacd323223b3b876b0e6ab50c792009c440fa`.
Static checks, 97 units and the entire original Linux CI pass, including both
native orphan scenarios. This is prospective compatibility, not an actual merge.

The further local storage/device/FD composition is clean at
`/home/vince/levitate-development/ginux-prd07-tasks`, revision
`1ad8af6ed0f2e76f6b0e465dff46ab26f9e69b51`, tree
`405105eba2cc8c8316a13a46675ce5296a5e6474`. Commits `5c3aafec` (#234) and
`1ad8af6e` (APIs) apply over G05 `997ffae2`. Launcher SHA-256:
`68f9ec37d95097833e30accbd5ee8ad55bbb1ae8e33e684a34df6b3d158d11b1`.
Static checks, all 149 units and the complete thirteen-command G05 CI pass,
including the six orphan tests, two native orphan scenarios, full storage/device
and FD/pipe suites. Exact delta and receipts are `tasks-foundation.diff`,
`tasks-foundation-source.json` and `tasks-foundation/` under PRD07 artifacts.
This remains a local prospective composition. Configured children remain disabled until
the task/exec join includes modern state ownership and guards. The required
runtime cut must preserve typed-storage directory/cwd identity per process,
per-task syscall transactions, descriptor reference accounting and birth rollback.

`f9a584e9` seizes tracees behind a private startup gate so peers can be interrupted
without guest-visible signals. `e0519b48` holds them across native path kernel
use, with `1c2b1333` retaining the hold through exec setup. Fork-only support is
not an exemption: processes can share anonymous memory containing path/output
buffers. These guards belong with the first concurrent runtime, along with the
existing death/unknown-birth-stop handling. Later owned mappings (`ad364bd7`),
SIGCHLD action (`cde7dc22`) and shared TID-page birth (`a0419e24`) keep their guards
when those operations become reachable. Preserve thread-directed SIGPIPE,
bootstrap mask clearing and #234's orphan suite. The obsolete G07 snapshot is
not the final owner. None of this runtime work is claimed complete by the API
candidate or its green checks.

## Reproduction and evidence

On each pinned worktree:

```sh
git rev-parse HEAD
git status --porcelain
make check
make test-unit
make build build-launch-fixture build-process-fixtures build-gate1-fixture
sudo make test-launch
sudo make test-native-supervision
make test-process-native
sudo make test-gate1-setup
```

Standalone current-main source passes 91 units (all original 89 plus the two
retained API cases), static checks and all six Linux CI command groups.
The #234 composition passes 97 units and the same complete CI, retaining the
six additional orphan cases and the two native orphan scenarios.

Evidence under `.artifacts/ginux-contribution-preparation/prd07/`:
`process-api-source.json`; `process-api/receipts.json` and
`process-api-orphans/receipts.json`; static logs; `process-api-control/driver.rs`,
red/green build/run logs, donor patch, conflict snapshots and
`orphan-test-retention.json`. No publication, configured-child runtime or
final-source Daybreak acceptance is inferred.

## PR draft — do not submit

Title: **feat(process): expose birth cancellation and wait inspection**

Supervisors need to release an unpublished process reservation on bootstrap
failure and inspect a typed child outcome before committing wait copyout.
Add `cancel_fork` and non-consuming `peek_wait`; keep existing `wait` behavior
by consuming the selected outcome after inspection. Both original focused
lifecycle tests are retained.

The full single-file change passes static checks, all 91 units and original
Linux CI directly against current main. A separate local composition retains
all six #234 orphan tests and passes 97 units plus both native orphan scenarios.
No configured runtime or policy change is included. Publication is prohibited;
recheck upstream's foundation and actual prerequisite merges before proposing it.
