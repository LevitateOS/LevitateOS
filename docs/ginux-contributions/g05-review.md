# G05 — public descriptor ownership and anonymous pipes

Current retained local commit `51b2c751af4a68d78261e7b63e731672e0b13240`, on refreshed G04
`09fdb1b0f2caae3432846966815dcb7444a3c0e2`, rooted at observed main `8a475e8b`.
The current-main refresh below owns fresh evidence and pending checks; earlier
results remain historical.

Earlier local feature commit `8c1a579aafb873be289b49259b961ee2cb6532eb`.
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

## Current-main refresh — 2026-11-06

The retained scope remains seventeen files, +1297/-76. Upstream changed the
`main.rs` module context and moved the native dispatch under a systemd/native
selection. Only the descriptors declaration and unexpected configured-Park error
arm are placed in the current native match; the systemd branch and task owners
remain intact. Other sixteen files apply unchanged. Added/removed source lines
match the previous corrected extraction apart from indentation in that arm.

A separate full-diff self-review reread descriptor reservation/get/dup/close/
finish, channel allocation/ring/lifetime, backend dispatch, bootstrap limits,
path/argument restoration and current native completion/failure callers. Aliases
retain one open object; last-alias close retires the backend. Pending opens free
failed reservations and public allocation precedes mutations. Path scratch is
restored before public argument restoration. Pipe data changes only after the
existing copy result; no native pipe or descriptor authority is added. Core
contract bytes are unchanged. No new blocking source correction was identified;
independent reviewer or security acceptance is not claimed.

Candidate and independent clean G04-plus-only-G05 replay both pass:

- `make check test-unit build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-launch-pipe-fixture build-launch-fd-fixture build-process-fixtures build-gate1-fixture`, including all 85 current unit tests and 24 VFS tests.
- Twenty-three ordinary controls per export: eight launch/valid-wait, three
  storage, three null-device, six public-FD/pipe and three pipe lifecycle modes.
  They check shared offsets/CLOEXEC across owners, native FD reuse and original
  read-argument restoration, public exhaustion without truncation, lower
  operator limit 8, 64 KiB byte-checked pipe transfer, atomic nonblocking capacity,
  shared endpoint flags, EOF after the final writer, eight-pipe capacity/reuse,
  mixed assignments and repeated fresh launches. SIGPIPE terminates normally;
  both parked-operation fixtures terminate and are reaped after SIGKILL directed
  only to their owned pidfds.

On the exact recorded G04 binary, identical valid pipe and public-FD fixtures
exit 10 (pipe creation missing) and 11 (common public allocation missing).
These demonstrate new capabilities, not regressions in a prerequisite that never
promised them. Separate ordinary storage/device fixtures use the new documented
public numbers; existing complete source fixtures retain their data/path/copy
assertions and are adjusted only for this public ABI.

All 199 tracked paths match the candidate in source and clean exports. Both
binaries hash `78e05c5aa1a7e684e28828cb47756dc5d552573392da86d5a061ad09541079ce`.
Exact source, commands, receipts, logs, old/new patches, manifest and concise
draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G05/`.
Build targets are isolated and the snapshot commit is local/unreferenced. No real
Ginux branch/index/checkout or remote text changed.

Full default FD/pipe/storage/device/launch/native/setup suites retain protected,
fault, private-backend authority and scratch cases. Those probes remain unrun
under the downstream DaybreakBlue boundary; ordinary lifecycle success does not
satisfy their acceptance. No agent was dispatched. macOS/remote CI is unrun.
R03/R04 retain full required validation and actual G04/prerequisite upstream
merges before standalone extraction/revalidation. No stacked publication or
combined F02/F03 closure is implied; child/thread integration remains later work.
