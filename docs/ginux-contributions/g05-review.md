# G05 — public descriptor ownership and anonymous pipes

Local feature commit `8c1a579aafb873be289b49259b961ee2cb6532eb`.
Explicit local prerequisite G04 `5036b7e8a61c4cf1e18abad8746208d03d268ea5`
(with its G03/G05a prerequisites), underlying upstream3f2fe7df.
[Retained source patch](patches/G05.patch). Seventeen files,+1297/-76.

Keep pipe allocation and the common public descriptor ABI together. Splitting
would first expose high pipe backend handles and then immediately replace that
interface. The runtime table is tightly coupled to backend dispatch/translation,
so an unused separate generic library is not introduced. All owners reuse the
same descriptor/alias lifetime; later task/network implementations are excluded.

## Review and retained corrections

A separate full-diff self-review covered slot reservation/rollback, native and
owned handles, dup2 retirement, last-alias close, descriptor versus shared flags,
copy faults, pipe ring bounds/atomicity, termination, syscall argument restoration,
configuration, failure cleanup, documentation and CI. No independent reviewer
coverage is claimed.

- Public slots0–255 respect the captured lower operator soft limit. Reserve
  before any open/create/truncate and free failed reservations. Private backend
  integers are rejected as public authority. Native returned FDs are bounded.
- Aliases retain one backend open object, preserving shared offsets/status and
  only closing its last reference. CLOEXEC belongs to each public slot. Dup2/dup3
  retire old targets correctly across native files, memory files, null and pipes;
  native FD reuse cannot change a different public slot's meaning.
- Descriptor translation and canonical native path rewriting compose in the
  existing loop. Path scratch/translated arguments restore first; the descriptor
  owner then restores the original public arguments. A real openat check verifies
  original dirfd, pathname, CLOEXEC flags and mode registers. Native audit and
  argument validation use the translated operation when dup retirement executes
  a native close. Existing G03 scratch/metadata regressions remain intact.
-260 real EACCES native opens do not leak reserved slots. Public/backend
  exhaustion leaves existing data intact. A runtime resource-width probe already
  passes on the initial extraction because upstream's unchanged core rejects
  resource IDs above15: no fictitious defect/fix is claimed. Lower-limit8 tests
  retain the operator limit and fail further public allocation with EMFILE.
- Pipe pair allocation rolls back both backend handles and public slots on bad,
  read-only or partial copyout. Atomic writes<=4096 bytes publish no partial input
  on a copy fault; reads consume only successfully copied bytes. Ring capacity,
  nonblocking empty/full transitions, EOF after the last writer, shared endpoint
  flags, alias lifetime, SIGPIPE and eight-pipe exhaustion are checked.
- Blocking pipe operations remain at the syscall-entry stop in this single-task
  runtime. The corrected f1c6419e observer reads a separate audit description;
  it cannot move stderr's writer offset. SIGKILL terminates/reaps parked reads
  and writes. Peer scheduling and signal-driven restart are not claimed here.
- Held bootstrap polling/clock code is omitted. Core syscall policy is byte-for-
  byte unchanged. Supervisor changes only derive Clone/Copy for the register value
  type; #221 copyout/fallback helpers and tests remain unchanged. Existing unit
  modules, native checks and earlier path/device regressions are preserved.

The same public-FD/pipe fixtures fail on prerequisite G04 (workload exits10/20)
and pass on this candidate. Those failures demonstrate the added capabilities,
not a claim that the prerequisite promised them.

## Validation and delivery boundary

Both the candidate and a fresh G04 export with only this patch applied pass77
repository unit tests and:

- `make check test-unit`
- `make build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-launch-pipe-fixture build-launch-fd-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch test-launch-storage test-launch-devices test-launch-pipes test-launch-fds test-native-supervision test-gate1-setup`
- `make test-process-native` outside the ptrace-restricted execution sandbox

Clean patch replay matches all162 tracked paths with strict whitespace checks.
Every original unit module/required job remains; existing fixture assertions change
only their expected public handle range/limit, retaining data/path/copy checks.
Exact hashes, explicit runtime exit receipts, capability failures, full logs and
short draft are under `.artifacts/ginux-contribution-preparation/G05/`.

R01–R03 and clean composition proof pass locally. **Standalone delivery waits for
G04 and its prerequisites upstream**, then refresh/extract/revalidate against main;
no public stack. macOS Actions/full Rocky VM checks are unrun. Bounds are256 public
FDs,8 pipes of64KiB and4096-byte I/O/atomic writes. Children/threads, descriptor
polling, owned mappings, directory FDs and exec-time CLOEXEC are later packets.
No real Git branch/index/worktree/remote was changed.
