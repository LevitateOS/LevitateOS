# DaybreakBlue security validation handoff

Requested by the user on 2026-11-06. Intended recipient: DaybreakBlue, the
monitored cloud blue-team agent specializing in low-level security. This is a
written handoff only; no agent has been contacted or dispatched. Keep this record
in LevitateOS; it is not Ginux contribution text.

## Boundary and status

Continue ordinary implementation, source review, compilation, unit checks and
functional runtime scenarios locally. Do not execute exploit-like security
validation here. Preserve upstream tests. Select ordinary functional scenarios
instead of automatically rerunning suites containing adversarial probes. Record
excluded checks as pending; do not substitute historical results for fresh
security acceptance. Earlier validation evidence remains historical evidence
performed before this instruction, not a new DaybreakBlue result.

## Pending validation

- G06b `8c783e717b839b0e511820e89d29840cae0f3c0c`: independently validate protected,
  unmapped and cross-page clone child-TID/wait-status writes, rollback after a
  partial write, and wait faults still consuming a child. Relevant owners:
  `tracee_io.rs`, `launch_tasks.rs`; fixture sources and historical reproductions
  are retained with G06b. Prior local results predate this handoff; no new run.
- G03c–G06b: independently validate assignment confinement, native/image/owned
  interpreter authority and resistance to path/descriptor authority bypass.
  Include kernel-selected shebang/PT_INTERP targets and execveat image FDs.
  Relevant owners: `fs_hooks.rs`, `launch.rs`, `launch_tasks.rs`, `descriptors.rs`.
  Preserve useful allowed-path controls alongside denial probes. No new run.
- G07 (in preparation): validate clone TID copyout protections, public/private FD
  authority across shared tables, concurrent transaction isolation and teardown
  races. Pin the final candidate revision before running; source review and
  ordinary thread-lifecycle checks do not satisfy this adversarial acceptance.

Run only against authorized disposable local fixtures/images in the monitored
blue-team environment. Return exact revision, commands, observations, exit
statuses and any unverified checks. Do not publish PRs or contact maintainers.

## G07 blocking source finding

An unfinished disposable G07 export already existed when this continuation
started: `/tmp/ginux-contribution-preparation/G07/source`. Its prior
`path-race-initial.log` records a failing `x-path` scenario (fixture exit99).
The tracee's shared pathname can change between owner selection and native
pathname capture; a concurrent thread can therefore reach an image path hidden
by the configured memory assignment. This old reproduction was read, not rerun
under the new instruction. No G07 candidate is approved or publication-ready.

DaybreakBlue should investigate and correct stable pathname/metadata capture
across threads and other shared writable mappings, then validate confinement
against the exact corrected revision. Stack scratch alone is shared writable
tracee memory and is not a complete solution. Include SIGCHLD disposition
validation and kernel-consumed indirect arguments in the review. Do not broaden
public Linux support or weaken assignment authority to make tests green.
The existing `qfault` clone-TID and `x-path` security scenarios remain in source
but are excluded from this agent's new functional-only runs. Existing review
artifacts and logs remain available; no new exploitation or security probing
has been performed here. A written handoff is not evidence of dispatch or fix.

## G09 security validation pending

- G09a d29381c7: review malformed image offsets and backend failure contracts;
  checked copy-up extent arithmetic is implemented, but adversarial index probes
  have not been run here. Ordinary VFS data/truncation/rollback units pass.
- G09b 297ad8dd: validate anonymous mmap/mremap rollback and indirect memory-copy
  protections under adversarial workloads, plus inherited namespace confinement.
  Ordinary growth, quota, zero-fill and forked-alias checks pass in both source
  and clean replay. Full storage suite retains the older confinement/protected
  buffer scenarios; these were not rerun by this agent. The configured thread
  snapshot is deliberately excluded from this candidate.

## G09d pending security scenarios and automatic-review boundary

G09d is retained at85b53fe15a64041f451ec7e7aa1e77d08cb2c18a in the disposable source export at
`/tmp/ginux-contribution-preparation/G09d/source`; use that exact retained
revision or record any refresh before security validation. Full directory/default storage suites have
not been run here. Their protected/unmapped/cross-page getdents buffers, private
FD authority, cross-assignment directory traversal and rename confinement cases
remain for DaybreakBlue. Preserve the existing default scenarios and tests.

Automatic approval review rejected a proposed privileged run containing
`view-probes`: access/readlink checks were classified as authority-related
security validation conflicting with the user's handoff boundary. That command
did not run. Those scenarios remain pending for DaybreakBlue and must not be
retried indirectly here. A safer selection excluding view-probes was approved:
ordinary directory/cwd plus growth/bounds/quota scenarios pass; this provides no
substitute security evidence. No cloud dispatch or request for extra privilege
is implied. Review new kernel-owned cwd/proc-link handling and its namespace
checks alongside G07's unresolved shared-argument capture problem.

## G10 low-level executable backing and security validation

Upcoming extraction uses existing donors c5ae1c5d/32ba41e9 on reviewed G09d85b53fe1.
The implementation stages owned program bytes in a private sealed memfd and uses
supervisor-chosen ptrace syscall transactions to execute it. Those mechanisms
can resemble code injection; this record explicitly assigns their low-level
security review and adversarial validation to DaybreakBlue. No G10 runtime has
been executed yet. Ordinary source adaptation/build and functional execution of
controlled compiled fixtures remain the local implementation scope.

Validate private descriptor authority, sealing and transient descriptor cleanup;
pinned-file lifetime and writer exclusion; malformed ELF and implicit interpreter
selection; failure rollback and successful-exec-only CLOEXEC retirement; virtual
executable-name projection and resistance to host-path/fd leakage. Readlink and
proc executable authority checks remain for DaybreakBlue, consistent with the
prior automatic-review rejection. Do not use an unmonitored workaround to run
those scenarios here. Pin the final retained candidate before validation; keep
unperformed checks and G07's shared-argument race explicitly unresolved. This
written request does not dispatch an agent, publish code or contact maintainers.

### G10 source findings and remaining checks

The retained source is being finalized from the G09d base. Source-only review
found that the old donor checked owned ELF magic/machine but did not join its
PT_INTERP metadata to the existing image interpreter authority check. The new
source shares that metadata reader and namespace check; no hostile-interpreter
runtime validation was performed here. Confirm this correction independently.

Also validate transient kernel-consumed exec arguments against the shared-memory
capture problem already recorded for G07, including the empty pathname used by
injected memfd/execveat transactions. Excluding thread support does not prove
shared writable argument mappings are safe across other permitted processes.
The functional candidate remains security-held until stable argument authority
and associated adversarial checks are resolved. Do not infer safety from its
successful ordinary execution scenarios.

Executable-name source now prefers the VFS name for owned programs even when
the tracee root is '/', and reserves room for a deleted-name suffix. The existing
allowed-ELF interpreter fixture needs its main executable name '/allowed-elf',
not a blanket '/bin/' prefix; this source assertion was corrected without running
its readlink validation here. Review exact-name, maximum-length, deletion and
protected-buffer behavior in the monitored environment. Sealed executable copies
are additional memory beyond the documented writable-file backing bound; no
aggregate RAM quota is claimed. No DaybreakBlue dispatch has occurred.

### G10 exact retained revision

Validate candidate14866b1de07bc8474557396c09f74275e8fb6bf5 (baseG09d85b53fe1).
[Source review and ordinary evidence](g10-review.md) records91 passing units and
six controlled ELF execution scenarios plus image-cwd/storage controls in both
source and clean replay. It is security-held; ordinary success is not adversarial
acceptance. Include older-kernel memfd/seal fallback behavior in the pending
low-level checks. Full/default runtime probes were preserved and not run here.


## G11 interpreter chains and new exec stack validation

Extraction starts from G10 candidate14866b1d and donors73593745/a08aff7a/cda76faa.
Script support rewrites only the newly committed exec stack, restores virtual
script names/AT_EXECFN and retires the private script memfd before the first
interpreter syscall. This resembles argument/code injection: low-level security
review and adversarial verification belong to DaybreakBlue. Locally, perform
source adaptation/builds and ordinary controlled script execution only.

Validate bounded owned/image interpreter selection, malformed/truncated shebangs,
recursive chains, argv/env/auxv bounds and arithmetic, hostile/protected buffers,
private descriptor closure and failure rollback, descriptor-based script lifetime,
no host/native FD importing, and permissions/namespace traversal for /dev/fd
reopening. Include shared argument capture inherited from G07/G10. Preserve the
existing full fixtures/default CI; excluded security checks remain unperformed,
not passed. Pin the final G11 retained revision before monitored validation.
No dispatch or external message is authorized by this written handoff.


### G11 exact retained revision

Validate candidate0798d991ab2d652e54682750acb4fdd9f1e6d9ac on G1014866b1de07bc8474557396c09f74275e8fb6bf5. The
[review](g11-review.md) records92 passing units and14 selected ordinary script
scenarios in source and strict clean replay, plus ELF/cwd/storage controls.
Current source shares the shebang reader and component resolver, supplies fresh
VFS reopen positions and cleans intermediate pins through exec/death ownership.
These are source/functional results only. Low-level stack/AT_EXECFN, private
script-FD retirement, malformed/cyclic chains, authority and shared-argument
security verification remain unrun. Default full fixtures remain available;
no cloud agent was dispatched. Do not infer security acceptance from this record.


## G12 private mapping transfer and backing lifetime

Source preparation starts on G11 candidate0798d991 with donors32e2c85f,
868862bc and680d5081. This mechanism uses ptrace syscall replay, temporary
socketpair/SCM_RIGHTS descriptors and pidfd_getfd to obtain a read-only mapping
reference to supervisor-owned memfd storage. These low-level mechanisms resemble
injection/FD extraction: assign their security review and adversarial verification
to DaybreakBlue. Ordinary source/build and controlled valid mapping functionality
may be prepared locally; do not execute attack, protection-bypass, proc/private-FD
inspection or hostile mapping/scratch scenarios here.

Review shared argument/scratch mutation, kernel FD table authority and collisions,
nonleader/leader-exit handling, interruption/death/rollback, fixed mappings near
replay/scratch regions, mprotect/mremap permissions, writable-shared denial,
unlinked mapping lifetime, backing resize/quota arithmetic and private-FD cleanup.
G07's unresolved thread/shared-argument hold remains; no thread mapping acceptance
is implied by the per-process functional candidate. Preserve all default tests,
record exact retained source, and keep unperformed security acceptance pending.
This is a written handoff only; no cloud dispatch or maintainer message.


### G12 exact per-process revision and held thread join

Validate candidate83072ec3ddab3e4712ce8db9ba232a10342557de on G110798d991ab2d652e54682750acb4fdd9f1e6d9ac. The
[review](g12-review.md) records92 units and two selected valid ordinary mapping
scenarios, plus storage/ELF/script controls, in source and clean replay. Default
protection/SIGBUS/fixed-region/private-FD cases remain unrun. Nonleader fixture
hunks from680d5081 are retained as held-thread-join.patch in the G12 artifact
directory; no G07 thread capability or acceptance was imported. Review backing
resize/unlinked lifetime and the absence of aggregate mapping/COW accounting.
No monitored cloud dispatch has occurred; security and thread joins remain pending.


## G13 owned symlinks and traversal security

Prepare ordinary symlink creation, resolution and namespace mutation on G11
0798d991, using donors909399b4/aa8becf2/8b14578c and the current upstream-derived
component walker. Archive/restore hunks remain assigned to their later owners.

DaybreakBlue must validate assignment/host confinement, cross-image/owned and
hidden-device link paths, malformed/oversized targets, adversarial link chains,
namespace races, protected/partial readlink buffers and exec interpreter aliases.
Do not run security-looking traversal/readlink authority probes here, consistent
with the earlier automatic-review rejection. Ordinary VFS API units are local
functional checks; runtime adversarial loops and malformed paths remain unrun.
Preserve default fixtures, record exclusions honestly and pin the final candidate.
No cloud dispatch, maintainer contact or publication is implied.


### G13 exact retained revision

Validate candidate5c3ae8e5068b5da21d2c37f9545c0f7b10eba147 on G110798d991ab2d652e54682750acb4fdd9f1e6d9ac.
[Review/evidence](g13-review.md) records95 routine units, ordinary link file I/O,
script/interpreter aliases and ELF/storage/directory controls in source/clean
replay. Routine VFS API units include bounded cycle/error behavior; runtime
confinement/loop/malformed path and readlink/protected-buffer authority checks
remain unrun. Current shared component walker is unchanged, and whiteout/mkdir
capacity regressions are corrected. Review the assigned-storage traversal gate,
final-entry mutations and kernel-consumed arguments independently. No monitored
agent has been dispatched, and security acceptance remains pending.


## G14 current-directory authority and lifecycle validation

Prepare owned cwd on G13 candidate5c3ae8e from donor6a45d8f8, using the current
per-process Task owner rather than the held G07 thread groups. Ordinary cwd,
relative file operations, fork/exec and directory rename/removal can be checked
locally. Assign low-level host cwd/path leakage, protected getcwd buffers,
namespace/descriptor races, implicit relative interpreter authority and teardown
under interrupted transitions to DaybreakBlue. Thread-shared cwd remains a held
G07 join; preserve donor fixture hunks without claiming they ran. Keep G13
component and final-entry semantics and G10's shared-argument security hold.
No cloud dispatch, publication or maintainer message is implied.


### G14 exact per-process revision

Validate candidateec6f9c480b7de187350672478500448e59a54ea3 on G135c3ae8e5068b5da21d2c37f9545c0f7b10eba147.
[Review/evidence](g14-review.md) records95 routine units and ordinary cwd/fork/
exec/rename/removal plus capacity rollback/reclamation, in source/clean replay.
Protected getcwd, host-path leakage, raced namespace/FD/argument state and native
relative-interpreter authority remain unrun. Thread-sharing hunks remain in the
G14 held-thread-join.patch artifact for G07. Review the transient cwd selector,
success-only native cwd commit, failed-fork rollback and deferred shared-thread
ownership independently. No monitored agent was dispatched; security remains pending.


## G15 hardlink authority and alias lifetime

Prepare donor9e2a39e8 on G13 candidate5c3ae8e; no G14 cwd capability is required
by the hardlink implementation. Ordinary shared bytes/metadata/counts, quota,
FD/reopen/executable reference lifetime and unlink/rename are local functionality.
DaybreakBlue owns adversarial namespace/FD races, assignment/host authority,
proc executable/readlink identity projection and protected metadata copyout.

Source review must preserve the distinction between an inode and the name through
which a file/exec reference was obtained. Removing a hardlink must not permit
recycling that entry while its FD or executable still uses its name. Keep shared
positions and independent reopen offsets; maintain G13's exact-entry whiteouts.
Runtime security-looking probes remain unrun and are not replaced by ordinary
library/API checks. Pin the final candidate before monitored validation. This
written handoff dispatches no agent and authorizes no publication or messages.


### G15 exact retained revision

Candidate 8ab24d4b47adeb9897623627060846070d41219d on G13 5c3ae8e5068b5da21d2c37f9545c0f7b10eba147 has98 passing routine units and
selected ordinary alias/quota/ELF/script/storage checks in source and clean replay.
[Review/evidence](g15-review.md) records name-versus-data identity, final-close
reclamation and ordinary donor regression comparisons. Adversarial namespace,
FD/private backing, protected copy, shared arguments and low-level executable-name
validation remain pending. No cloud agent was dispatched and no security
acceptance is claimed. G14 cwd/G12 mappings still need the later combined review.


## G16 — cross-image/owned traversal and interpreter routing (in progress)

Candidate is being ported on G14ec6f9c48, excluding G15/G12/G07 ancestry.
Ordinary local work covers image reads through owned links, image directory-FD
relative operations, virtual cwd parent traversal and valid script interpreter
aliases. The existing component walker and private syscall scratch owner remain.
DaybreakBlue must independently validate adversarial cross-boundary links and
namespace mutation, hidden device/proc/sys paths, directory/FD authority and
replacement races, protected/overlapping syscall scratch, implicit interpreter
selection, sealed script-header canonicalization and argv/AT_EXECFN restoration.
Do not infer confinement or race safety from these ordinary checks. Retain full
security fixtures and CI; they are not run here. No cloud dispatch is authorized
or implied. Exact revision will be appended after source/ordinary preparation.


### G16 exact retained revision

Candidate 4682b0b0535d5a2f16b8ed55634ad7b3a7b26dd7 on G14 ec6f9c480b7de187350672478500448e59a54ea3 has95 routine units, three new ordinary
path/exec modes and five new valid script-interpreter modes passing in source and
clean replay, plus existing controls. [Review/evidence](g16-review.md) records the
canonical-token rejection added through source review; its adversarial validation
and the previously listed boundary/FD/race/stack checks remain pending. VFS walker
and native lookup are preserved. Full cross modes remain present and unrun here.
No monitored agent was dispatched; security acceptance is still pending.


## G18 — configurable namespace allocation/accounting (in progress)

Local port starts on G09d85b53fe1, excluding unrelated exec/link/cwd/mapping/thread
features. Ordinary local checks will create and read5000 nonempty files, exercise
configured exhaustion, retain an unlinked open file and reclaim its slot on close.
DaybreakBlue should validate adversarial configuration sizes, checked arithmetic,
mapped-array initialization/alignment/lifetime, allocation failure and cleanup,
metadata/data accounting under hostile state and future alias/pin interactions.
Do not run malformed/bypass/protected-memory probes here. Public inline VFS defaults
are preserved; namespace/backing capacities must match in the runtime adapter.
No agent dispatch or outgoing message is implied; exact revision remains pending.


### G18 exact retained revision

Candidate f58ba480942e91ad7a0a43158a9c57496acc202a on G09d 85b53fe15a64041f451ec7e7aa1e77d08cb2c18a has91 routine units and5000 nonempty
files with individual readback, configured exhaustion and unlinked-FD final-close
reclamation passing in source/clean exports. [Review/evidence](g18-review.md) records
checked metadata charging and explicit byte quota. Malicious configuration sizes,
allocation failure/cleanup, unsafe pointer/alignment/initialization and future
alias/cwd/executable/map interactions remain pending for security validation.
Routine API/error units do not constitute that adversarial acceptance. No cloud
agent was dispatched or security probes performed.


## G17 — positioned I/O and private descriptor budgets (in progress)

G17a ports pwrite onto G09d without namespace/mapping ancestry. Ordinary local
checks cover duplicate/shared offsets, sparse writes, negative offset rejection,
append behavior and byte-quota failure. Protected/partially readable buffers and
raced FD/offset ownership remain for DaybreakBlue. G17b will join actual memfd
backing with G18 metadata and retain the captured public descriptor bound.
Private-FD authority, exhaustion/failure injection, bootstrap leakage and mutable
syscall races must be validated independently; do not execute those probes here.
This written handoff dispatches no agent; exact revisions will be appended later.

G17a exact retained revision: 94baebbec5e887ae659718c3f6162d08ca267b99 on 85b53fe15a64041f451ec7e7aa1e77d08cb2c18a.89 routine units and
ordinary positioned/alias/sparse/append/quota checks pass in source/clean replay.
Protected-copy/race acceptance remains pending; no cloud agent dispatched.

G17b exact retained revision: 6fcdd6183703768a2f4ed1cc48380eaa50da547c on f41ed1b093044b10d45fcab771687dfb089ed3a5 (G12+G18).
94 routine units and selected ordinary low-limit/20-file/5000-file/mapping/exec
checks pass in source and clean replay. Independently validate private FD leakage,
inherited FD overhead beyond the fixed128 allowance, bootstrap/transient state,
protected copy, failure injection and races. No cloud dispatch or security
acceptance is claimed. Full FD width/authority and mapping inspection modes remain.


## G19 — flush and quiesced export (in progress)

Local port uses G12 memfd/per-process ownership and does not import held G07
thread groups. Ordinary verification will cover fsync/fdatasync, complete regular
file/directory archives, a child writer settled before final capture, failed
workload cancellation and an existing destination. Final publication now uses a
private same-directory staging file and RENAME_NOREPLACE after write/sync/close.
DaybreakBlue must validate malicious archive/path components, hostile destination
parents or staging-name replacement, final-name races, abrupt interruption,
write/sync/close/cleanup failure injection, inherited FD authority and concurrent
writer/shared-argument behavior. Do not execute those security probes here.
Full thread and symlink/hardlink/archive joins remain pending. Ordinary results
cannot establish those guarantees; no cloud agent has been dispatched.

G19 exact retained revision: a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e on G12 83072ec3ddab3e4712ce8db9ba232a10342557de.
92 routine units and ordinary flush/export/config-cleanup/child-settlement plus
mapping/exec/storage controls pass in source and clean replay. Early config
rejection staging leakage was reproduced and fixed at common completion. Validate
hostile parent/staging/final-name changes, abrupt interruption, I/O/close/cleanup
failure injection and shared-thread/argument races independently. A complete
archive is published only after close, but no adversarial authority acceptance is
inferred. No cloud agent dispatched; [review/evidence](g19-review.md).


## G20 — private control and task-event wait (in progress)

Porting on G19a36e7644 without guest polling/channels, restore or held threads.
Ordinary local checks will use valid full/fragmented control frames, two complete
snapshots, final export in the same directory, normal peer closure/cancellation,
and task status wakeups (including ignored host SIGCHLD). Distinct internal staging
identities isolate concurrent final-export and checkpoint owners.
DaybreakBlue must validate hostile/oversized/replayed frames, control FD authority
and bootstrap leakage, readiness/lost-stop races, pending mutation/mapping/thread
state, inherited signal masks and fault-injected send/read/close/capture failures.
These are not run here. Keep the donor full negative/private-FD checks in CI;
ordinary selections are not security acceptance or monitored cloud dispatch.

G20 exact retained revision: 7afa833d1babcc5dd62ef326f81c52835f9aa349 on G19 a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e.
92 routine units and selected ordinary capture/fragmentation/final-export/peer-
closure/task-event plus existing storage/exec controls pass in source/clean replay.
No negative-frame, private-FD inspection, race or injected-failure run occurred.
Validate bounded framing/authority, bootstrap leakage, readiness/lost-stop races,
inherited signal masks and shared mutation independently. No cloud dispatch;
[review/evidence](g20-review.md).

## G21 — startup restore (in progress)

Porting1d9e00e0 directly on G19a36e7644, without G20 control or held threads.
Ordinary checks cover valid final-export reconstruction, long names, permissions,
empty/binary/multi-chunk data, repeated fresh starts, child-before-parent ordering
and unchanged source archives. Full/default rejection fixtures remain available.
DaybreakBlue must validate malformed/truncated/oversized archives, traversal and
assignment authority, duplicate/colliding entries, unsupported links, hostile
source replacement, FIFO/device inputs, quota/FD/read/close failures and concurrent
mutation. The source open adds O_NONBLOCK so a non-regular input cannot block
before fstat; this is a source correction, not experimentally validated security.
Do not run those adversarial probes here. Namespace/link/scale joins remain G22
and F02 work. No cloud dispatch or security acceptance is implied.

G21 exact retained revision: 33bf08a746bd64306e0375f8d419af4744d31064 on G19 a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e.
92 routine units and selected ordinary restore/restart/data/mode/parent-order plus
export/mapping/exec/storage controls pass in source and clean replay. Default
malformed/traversal/duplicate/authority fixtures remain unrun. Independently
validate archive and assignment bounds, hostile input/replacement, resource and
close/read failures and shared-mutation behavior. No cloud dispatch or security
acceptance; [review/evidence](g21-review.md).

## G22 — bounded archives with link identity (in progress)

Porting e3374593 on a disposable G13/G15/G18/G17b/G21 composition, preserving
G19 atomic final publication and G21 startup cleanup without G20 or held threads.
Ordinary local verification will use legitimate linked files/symlinks, long
names/targets, an unlinked original with a surviving alias, 5000 nonempty files
and a complete valid2 GiB+1 payload within an explicit4 GiB memory budget.
These positive archive/consumer checks do not validate hostile input handling.
DaybreakBlue must validate unsafe paths and assignment authority, forward or
conflicting aliases, nonregular parents/targets, duplicate/conflicting metadata,
malformed/oversized headers and payloads, hostile source changes, arithmetic
bounds, FD/read/write/close failure injection and concurrent mutation. Full
negative defaults are retained but not executed here. No cloud dispatch occurs.

G22 exact retained revision: 7b2adc4f75acca26f2b356fbf9a47a9644c39bc8 on composition 9caccb3dd0b9d9866726c186cde04c4962152f4c.
100 routine units and selected ordinary link/alias/long-target,5000-file and
complete2 GiB+1 restore/export with full streaming digest pass in source/clean.
Unsupported symlink modes now reject rather than silently becoming0777, based on
source review; that adversarial rejection remains unrun. Validate archive/alias/
assignment authority, malformed and oversized input, conflicting metadata,
hostile source changes, failure injection and races independently. No security
acceptance or cloud dispatch; [review/evidence](g22-review.md).

## G23 — owned file locks (in progress)

Porting26784843 directly on reviewed G15 inode/description ownership, without
held G07 thread groups, public readiness policy or unrelated archive capabilities.
Ordinary checks use trusted lock structs and synchronized real child processes
for conflicts, query results, split ranges, blocking wakeups and process versus
open-description lifetime. Full invalid-pointer/default fixtures remain available.
DaybreakBlue must validate protected/mutable lock structs, raced close/reuse and
owner changes, descriptor/inode authority, table exhaustion rollback, interruption
and wait/death races, thread group ownership and deadlock/cancellation behavior.
Do not run adversarial FD/lock/copy/race probes here. Actual SQLite/Cargo consumer
closure requires their other filesystem capabilities and remains explicitly
separate from minimal lock-fixture evidence. No cloud dispatch occurs.

G23 exact retained per-process revision: 14c3332121d0f8375649d6af5ddb18fb16633e22 on G15 8ab24d4b47adeb9897623627060846070d41219d.
98 routine units and selected ordinary real record/flock contenders, audit-
confirmed blocking/wakeup, supported SIGKILL cancellation, exec/CLOEXEC/final-close
and existing controls pass in source/clean. Full pointer/authority/race/capacity
and thread acceptance remain unrun; actual SQLite/Cargo closure is still F02/F03
work. No cloud dispatch or security acceptance; [review/evidence](g23-review.md).
