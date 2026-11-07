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

## G24 — inode timestamps and archive mtime (in progress)

Porting d21d26c8 directly on reviewed G22/G15 metadata/archive ownership, without
G23 locks, G20 control, network stat encoders or held public clock policy. Ordinary
checks use explicit trusted times, UTIME_OMIT, owned alias/descriptor identity,
valid no-follow updates to a contained owned symlink, and whole-second mtime
restart while atime/nanoseconds are deliberately not persisted.
DaybreakBlue must validate protected/mutable time structs, invalid flags/fields,
no-follow/assignment bypass or races, dangling/unlinked/reused identity authority,
negative/unrepresentable archive times, alias metadata conflicts, clock-boundary
and failure-injection behavior. Do not run those adversarial probes here.
UTIME_NOW reads native supervisor realtime; ordinary NOW/implicit-time calls now have functional coverage. #179 policy
and clock-boundary/failure validation remain pending.
Retain full/default invalid-pointer/NOW/metadata checks; no cloud dispatch occurs.

G24 initial retained revision before ordinary NOW coverage: 2dbbcb02731350c6b12a4225829f5d012845f85c on G22 7b2adc4f75acca26f2b356fbf9a47a9644c39bc8.
100 routine units and selected ordinary explicit/OMIT, alias/descriptor/stat,
contained symlink metadata and whole-second archive restart plus existing
large/scale/storage/exec controls pass in source/clean. A valid futimens null-path
adapter failure was reproduced and corrected in the shared resolver. Native NOW,
invalid/protected/mutable inputs, authority/races, unrepresentable archive times
and conflicting alias timestamps remain unrun. No cloud dispatch or security
acceptance; [review/evidence](g24-review.md).

G24 exact retained revision: b33fbd24ace566d8db20c8e2d7a295680084ac7b on G22 7b2adc4f75acca26f2b356fbf9a47a9644c39bc8.
100 routine units and selected ordinary explicit/OMIT, alias/descriptor/stat,
contained symlink metadata and whole-second archive restart plus existing
large/scale/storage/exec controls pass in source/clean. A valid futimens null-path
adapter failure was reproduced and corrected in the shared resolver. Ordinary
NOW/null-times checks also pass. Clock-boundary/failure and invalid/protected/
mutable inputs, authority/races, unrepresentable archive times
and conflicting alias timestamps remain unrun. No cloud dispatch or security
acceptance; [review/evidence](g24-review.md).

## G25 — vectors and chunked regular I/O (partial)

Exact retained ordinary head `9afd60cca140881288498fc21058b404814c08e4` on G17a `94baebbec5e887ae659718c3f6162d08ca267b99`.
89 routine units and selected ordinary source/clean vectors, complete2 MiB+37
payload, offset/alias, quota-prefix and nonblocking pipe atomicity/EOF checks pass.
Native scalar clamp74835089 remains policy-held and absent; actual APK/RPM is
still F02/F03 consumer work, not established by these small fixtures.

DaybreakBlue must validate protected/mutable iovec metadata and payloads, faults
inside and between vector/chunk boundaries, short-copy/error prefix accounting,
aggregate/address overflow, read-only destinations, FD/inode authority and reused
slots, cancellation and blocked retry with changing layouts, shared-buffer/thread
races, append/offset/close ordering and failure rollback. Preserve existing full
pointer/copy tests and required CI; no adversarial probes were run here and no
cloud agent was dispatched. G07's shared-argument hold remains. Source self-review
is not security acceptance; [review/evidence](g25-review.md).

## G26 — inherited AS/CPU bounds and task queries (partial)

Exact G26a `1d0824e6634aca994a38a9f9531b90344c787ac7` on recorded main; G26b `2ed520c4437a8459c70dc79208d40b2923390939` on
G06b/G26a composition `687eb942fb9d0628558d1d25094f5d53a9e3094f`. Ordinary source/clean AS bounds,
lower operator limit, exact eligible CPU subsets and fork/exec inheritance pass.
No host-pressure stress: over-limit requests reserve virtual space only and
successful1 MiB mappings touch two pages. Existing default coverage is preserved.

DaybreakBlue must validate resource/affinity mutation and bypass, unsupported or
foreign identities/CPU IDs, oversized/invalid/protected/mutable query buffers,
partial copyout, AS arithmetic/limits, hostile memory/process exhaustion,
bootstrap syscall failures, CPU hotplug/reconfiguration, stop/death/exec races
and thread-group target selection. Known F02 join: a nonleader requesting its
own TGID must resolve the registered leader host, not always the caller. Current
G26b is process-only (PID==TGID); do not imply G07 acceptance. No adversarial
probes or cloud dispatch occurred. Public resource contract, full CI and combined
consumer evidence remain pending; [review/evidence](g26-review.md).

## G27 — configured uname identity

Exact head `1eb3e189525670e1056e89ef4bc49ab7f0d481dd` solely on recorded main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
65 routine units, full ordinary390-byte utsname ABI/padding/max-valid ID and
actual RPM4.19.1.1 documented Lua identity queries pass in source/clean. The
existing ordinary valid-buffer native wait_status case also passes. New uname
copyout is strict process_vm; existing legacy helper fallback remains unchanged.
No fallback injection or protected output probe ran here.

DaybreakBlue must validate invalid/oversized/NUL identity fields and short output,
protected/mutable/partially mapped destinations, address arithmetic and prefix
fault effects, process_vm unavailability/failure, signal/thread races and correct
owner joins. The trusted ordinary RPM baseline on exact main signals11 after
uname is denied; candidate succeeds. Treat that as observed functional evidence,
not an exploitability conclusion; no hostile input or crash exploitation ran.
No cloud dispatch occurred. #179 identity policy, full CI and G06/F02 task-loop
join remain pending; [review/evidence](g27-review.md).

## G28 — supervisor TCP backend (partial)

Exact `0a3cf5f5ab7e77080eb9491cb6bffd5ba642cb68` on recorded main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
Static/build and67 selected routine units pass in source/clean, including real
IPv4/IPv6 metadata, nonblocking I/O, budget/half-close and normal close/drop/revoke.
Only trusted controlled local peers were contacted; no scans or foreign endpoints.

Default-enabled but unrun donor tests:
`tcp_endpoint_slots_budget_and_revocation_use_real_connections` and
`tcp_drop_closes_connecting_handles_and_invalid_policy_never_opens`.
Local unit commands use explicit filters solely for these two new tests;
unfiltered make test-unit/new Linux CI remains pending, not green.

DaybreakBlue must validate denied/foreign/malformed endpoints and family/scope
representations, policy/assignment bypass, revoked/stale/reused handles and public
alias authority, concurrent revoke/close/connect and failed/in-progress cleanup,
backend error/short-count failures, byte-accounting boundaries and adversarial
network/race/failure probes. Check async SO_ERROR consumption when G29 integrates
it. Host descriptor/CLOEXEC authority and guest confinement need that join too.
No cloud dispatch occurred; upstream network policy/full CI and final consumer
acceptance remain pending; [review/evidence](g28-review.md).

## G08a readiness prerequisite and G29 source findings

G08a `fe27c693bf2629166ee82da2efefb62b298bb567` on G06b `8c783e717b839b0e511820e89d29840cae0f3c0c` passes82 units and
five ordinary source/clean poll modes (live/mixed/aliases, read/write/HUP peers,
EOF and timer). Poll input is now snapshotted across retries with checked internal
time arithmetic. Default protected-array/authority/kill cases remain enabled but
unrun; no proc/FD inspection or hostile probes were executed locally.
DaybreakBlue must validate protected/mutable poll buffers and partial output,
unknown/native/closed/reused FD authority, thread/call snapshots, interruption/
cancellation/death races, rollback and clock/bounds failures. #179/#207 and full
runtime/remote CI remain pending; no cloud dispatch.

## G29 — configured TCP integration (partial)

Head `26fbfef2ecc4d3192ff605b23c9a4bb02e781171` on G08a/G28 composition `2ba7c8942e1530048904db5a63c5b9de2e1c0298`.
84 selected routine units, ordinary source/clean TCP/poll modes and actual curl
IPv4/IPv6 HTTP and trusted TLS pass. Actual Git remains blocked before transport
by absent getcwd until G14/F02/F03. No negative TLS/foreign endpoint or hostile
buffer probes ran. [Review/evidence](g29-review.md).

Initial source findings are corrected: receive data is staged until copied,
failed connection state survives SO_ERROR consumption, canonical vector copies
are reused and deadlines persist across retries. These fixes have source review
and ordinary functional evidence, not adversarial verification.

DaybreakBlue must validate partial/protected/mutable scalar/vector receive and
send buffers, retained tails across aliases and exhausted budget, zero/EOF and
copy-error ordering, address/aggregate overflow, malformed/denied/foreign numeric
endpoints/families/scope/mapped representations, SO_ERROR refusal/reset/poll/retry
races and consumable versus persistent state, SIGPIPE/MSG_NOSIGNAL and thread
signal targeting, private-FD/CLOEXEC/assignment authority, stale/reused/closed
handles, concurrent fork/exec/close/cancel/revoke, shutdown/error/timeout edges,
quota/connection exhaustion and injected backend failures. Preserve full default
fixture modes deny/quota/hold/hold-kill/slots/off and the3 new policy/security
units; all remain enabled and locally unrun. G07 shared-path/thread hold still
applies. No cloud dispatch or security acceptance occurred; policy/full CI/fresh
upstream independence and final consumer closure remain pending.

## G30 — storage virtual-root ownership

Head `2186548d5af38b6ab06379a38b9f8fc7a08636bc` directly on G15 `8ab24d4b47adeb9897623627060846070d41219d`.
98 units and ordinary source/clean ownership/storage checks plus actual trusted
GNU cpio2.15 valid archive extraction pass. Actual cpio fails ownership syscalls
on exact G15. A reproduced empty-path dispatch failure is corrected. No ownership
suppression flags, external downloads, invalid archive or hostile buffer probes.
[Review/evidence](g30-review.md).

Validate protected/mutable ownership paths, UID/GID width/invalid IDs, flags and
missing/dangling/empty-path behavior; native/private/closed/reused descriptors,
assignment and inode authority; shared-thread/rename/close/exec races and error
ordering; verify no host ownership mutation or fallback. The full new ownership
mode retains donor negative/pointer/authority checks but is locally unrun. G07
shared-path/security, full CI and combined consumers remain pending. No cloud
dispatch or security acceptance occurred.

## G08b — configured packet pairs/FIONBIO/capacity

Head `bc34cf66d8472b786ba10455352a91c6a7081bb9` on G08a `fe27c693bf2629166ee82da2efefb62b298bb567`.
82 units and ordinary source/clean packet messages/truncation/zero/alias/EOF/peer,
FIONBIO/metadata,12 nested-process streams and poll controls pass. Current stat
flag handling was restored after a donor-port regression reproduced at163.
No native bootstrap poll or guest clock rules were added. Actual compiler/
thread/vector/network joins and full CI/policy remain pending. [Review](g08b-review.md).

Validate protected/mutable packet receive/send and FIONBIO integers, cross-page
short copies and prefix/atomicity/retained-packet behavior; malformed or oversized
messages, unknown flags/family/protocol and unsupported operations; allocation/
copyout rollback, reused/stale/closed/native/private descriptors and assignment
separation; concurrent aliases/close/fork/exec/cancel, blocked retries, capacity
failure, MSG_NOSIGNAL/SIGPIPE and shared-thread signal/call ownership. Retain full
default source tests. These adversarial/proc/authority/fault probes remain unrun;
no cloud dispatch or security acceptance. G07 shared-path hold remains.

## F02 — combined per-process snapshot (security unverified)

Combined `dfebda7c29b628fc41f50635b07e8cb48d6f883a` excludes G07 threads/shared arguments, native
clock/poll policy and the native read clamp. Ordinary source/clean build,102
selected units, channels/storage/cwd/locks/TCP/control/restore and actual
curl/Git/cpio checks pass. These results do not validate hostile cross-owner
inputs, private-FD/mapping transactions, shared argument races, control/task
cancellation faults or rollback/authority/confinement under adversarial input.
Carry all earlier packet handoffs into the combined review, including packet/
TCP dispatch, dynamic references, archive canonical metadata and typed resource
queries. Three default-enabled TCP policy/security units and full security
fixtures remain locally unrun. G07 remains unready; no cloud dispatch occurred.

G12 corrected `8bb50aaa50e80428ff417654d35affd468255775` preserves full FD security cases while replacing
its obsolete read-only mapping denial assertion with valid data/unmap checks.
Only ordinary mapping scenarios ran; the full protected/private-FD fixture is
still for monitored review. No security acceptance or green full CI is claimed.

## F03 — current Recipe integration catalog

Fresh snapshot `bde746221988094d4a0b80bb46808beab447e3dc` plus hashed tracked dirty overlays is built
separately; current e2e catalog has135 cases. F03 state/catalog lists exact names.
Only selected ordinary operations run locally. Preserve/review the full
worker_entry spoof/bootstrap, protected setup/source/asset authority, archive
traversal/link redirect, modified snapshot, redirect/network grant, protected
autofix and fault/cancellation cases under monitored security execution. They
remain enabled and unrun locally; no cloud agent was dispatched. The ordinary
first managed case passing is not complete suite/security acceptance. G07
thread/shared arguments and all earlier Ginux handoffs remain unresolved.

### Destination resume: G07 blocks ordinary Recipe consumers

On 2026-11-06 the complete paused F03 batch log was reread. All four failed
ordinary cases stop at Recipe's output-reader thread startup with ENOSYS:
`dependencies::cli_transitive_dependencies_compile_tools_through_the_shared_lifecycle`,
`managed_inputs::managed_bash_child_writes_domain_files_and_keeps_logs_out_of_json`,
`managed_inputs::managed_owned_elf_build_executes_and_validates_its_product`, and
`managed_storage::managed_storage_preserves_source_modes_links_and_restart_identity`.
The last case stops at touch, before its timestamp/restart assertions. Do not
treat these receipts as independent compiler, executable or archive defects.

Recipe's existing `src/process.rs:270` creates output-reader threads. Combined
F02 `crates/ginux-init/src/launch_tasks.rs:845` deliberately rejects thread clone
flags with -38. Required capability remains G07, whose retained unready revision
and unresolved shared-path finding are recorded above. Before importing thread
support, correct and validate its kernel-argument authority in the monitored
environment, then join the existing FD/task/path/cwd/mapping/resource owners in
F02 and rerun actual consumer cases on exact new binaries. No local thread
workaround, security probe or dispatch has occurred.

Migration paths: combined source is
`.artifacts/development-migration/2026-11-06/f02-source/`; paused Recipe source is
`/home/vince/levitate-development/f03-recipe-source`; preserved G07 evidence is
under the recovery directory's `recovery/evidence/ginux-contribution-preparation/G07/`.
The destination harness and commands are in `f03-destination-bin/`; the old
harness embeds source-machine paths. This path update does not authorize cloud
dispatch or constitute security acceptance.

The destination-built Bash scenario has now been repeated once on the migrated
host and fails identically at reader-thread creation (harness exit 101, 86.45s).
This rules out relying on migration alone to clear the observed consumer failure.
The remaining original failures are not rerun unchanged; keep them pending on
the G07 capability. Ordinary non-thread consumers continue independently.
Fresh receipt/log: the new F03 evidence directory's
`next-ordinary-receipts.json` and
`managed_bash_child_writes_domain_files_and_keeps_logs_out_of_json.log`.

Valid tar extraction reveals another G07 consumer on the destination: the two
ordinary contained-symlink/hardlink cases panic at thread creation. The existing
Recipe progress owner calls `enable_steady_tick` at
`src/helpers/internal/progress.rs:20`, reached from extraction at
`src/helpers/build/extract.rs:425`. Git and download progress use the same owner.
Logs retain clone3/clone ENOSYS and worker exit 101; these are ordinary functional
failures, not archive security validation. The zip case was interrupted during
administrative cleanup of the owned batch and is not a pass. Preserve these
checks for exact-revision reruns after G07; do not disable progress to substitute
for thread support. Other ordinary cases continue, with all security acceptance
and dispatch still pending.

Read-only upstream refresh now pins main
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`, 52 commits beyond the old extraction
base, including #228/#229 native runtime/domain/network work. The old combined
snapshot and G07 finding remain exact historical inputs, not proof about the
new source. Reconcile new native owners and typed network fields before choosing
the final combined revision for monitored validation. No new security probe,
cloud dispatch or security acceptance occurred during this refresh.


## Refreshed G03c security revision — 2026-11-06

Current local G03c is `e9c0a4d494cb9c2c2080289aa198dda42dd65b08`, based on
`6c35957d17c54dde9d98e5a91e49d37465d13a6d` (refreshed G03b/G03a on observed main
`8a475e8b`). [Current review](g03c-review.md) records 85 units and eleven ordinary
controls in source/clean replay; it does not establish security acceptance.
Revalidate assignment/path/descriptor authority and native canonical rewriting,
protected/partial/overlapping buffer and scratch restoration, and indirect/shared
argument capture on this exact revision or an explicitly recorded successor.
The complete preserved `test-launch-storage`, `test-launch`, native/process and
setup suites remain pending where they contain these probes. No such probes
were run in this refresh and no DaybreakBlue agent was dispatched. Earlier
source-machine security results are historical and must not satisfy this hold.


## Refreshed G04 security revision — 2026-11-06

Current local G04 is `09fdb1b0f2caae3432846966815dcb7444a3c0e2` on refreshed G03c
`e9c0a4d494cb9c2c2080289aa198dda42dd65b08`. [Current review](g04-review.md) records
85 units and fourteen ordinary controls in source/clean replay. The earlier
G03c–G06b authority handoff applies to this revision: validate hidden image-device
aliases, immutable virtual `/dev`, descriptor namespace/copyout authority,
protected/partial buffers and shared canonical-path scratch independently.
Complete default fixtures stay present and enabled; their security probes were
not executed during this refresh. Keep acceptance pending until monitored results
exist on the final revision. No DaybreakBlue agent has been dispatched.


## Refreshed G05 security revision — 2026-11-06

Current local G05 is `51b2c751af4a68d78261e7b63e731672e0b13240` on refreshed G04
`09fdb1b0f2caae3432846966815dcb7444a3c0e2`. [Current review](g05-review.md) records
85 units and twenty-three ordinary controls in source/clean replay, including
normal SIGPIPE and owned parked-fixture termination/reaping. The existing
G03c–G06b security handoff applies: validate public/private backend authority,
protected/partial pipe-pair copyout rollback, atomic input-copy faults and read
partial effects, translated arguments and canonical scratch independently.
Complete default fixtures and CI remain present; their fault/authority probes
were not run here. Shared-argument/thread safety and G07 acceptance remain pending.
No DaybreakBlue agent has been dispatched and no security pass is claimed.


## Refreshed G06b security revision — 2026-11-06

Current G06b is `7240bb217e16750f55f57a490d444964e8bf537a`, based on refreshed G05/G06a
composition `7e0061b500e32e0735d34a375fcb668968f09ee6`. [Current review](g06b-review.md)
records 90 units and forty-six ordinary controls in source/clean replay. The old
packet's duplicate memmove was dropped in favor of the unchanged upstream owner.
Revalidate protected/unmapped/cross-page child-TID/wait stores and rollback,
wait-fault consumption, implicit interpreter ownership, image exec FD authority,
private native CLOEXEC closure, shared/indirect kernel arguments and the preserved
full default security fixtures on this revision or a recorded successor.
Those probes remain unrun; ordinary successful exec and virtual CLOEXEC checks
are not private-kernel-FD or authority acceptance. G07's old unready snapshot and
shared-path finding remain unresolved; it is not refreshed or imported here.
No DaybreakBlue agent has been dispatched and no security pass is claimed.


## Refreshed G09a security revision — 2026-11-06

Current VFS subpacket G09a is `648ec32a34ece2f1cd7d37b10271e4166eb14683` on refreshed
G03b `5fb12ab4a56efd365d7d451d181f6e2a41c9d31d`. [Current review](g09a-review.md) records
89 units and eight ordinary controls in source/clean replay, plus the exact-base
truncation failure. A new ordinary unit covers multi-chunk data and trusted
reader-error cleanup; it does not validate malformed image offsets or adversarial
backend/range behavior. Those existing security checks remain pending alongside
full default suites. Runtime growth remains G09b; no thread/security acceptance
or DaybreakBlue dispatch is implied.


## Refreshed G09b security revision — 2026-11-06

Current runtime growth subpacket G09b is `a2a70d1c7811886d2fca20d228a94e7dc6b31a61` on
refreshed G06b/G09a composition `ff29e8574f1382355bd4b925bf2aaf634885c305`.
[Current review](g09b-review.md) records 94 units and forty-nine ordinary controls
in source/clean replay, with exact-prerequisite failure beyond 64 KiB. Validate
anonymous allocation/remap rollback, hostile range/copy behavior, page accounting
and indirect/shared argument authority independently. Ordinary quota exhaustion,
zero-fill, alias lifetime and successful teardown do not satisfy those security
checks. Default protected/authority fixtures remain enabled and unrun. G07 stays
unready; no cloud agent was dispatched and no security pass is claimed.


## Refreshed G09c/G09d security revisions — 2026-11-06

Current VFS directory owner G09c is `1e34ef3641c9a2d93f449818692692ac2b561ebc`
on refreshed G09a; paired configured runtime G09d is
`ab47378f657b8d13d72077009107d223ba04d3a3` on refreshed G09b/G09c composition
`eed013c73443a24390b1f5f6638dedca170c855f`. [G09c review](g09c-review.md) and
[G09d review](g09d-review.md) record fresh ordinary evidence only. Revalidate
protected/partial getdents copyout and cookie preservation, private/public FD
translation, namespace/assignment authority, canonical scratch restoration,
per-process proc-root/cwd interpretation, access/readlink view-probes and shared
argument authority on these exact revisions or a recorded successor. The earlier
automatic approval rejection of view-probes remains respected; no such run was
retried or routed around. Default tests/CI remain enabled and unrun where they
contain these checks. G07 is still unready and excluded. Keep security acceptance
pending; no DaybreakBlue agent was dispatched.


## Refreshed G10 security revision — 2026-11-06

G10 is now `38f7ee402611f9023b4173eff5dc5d64b7d68375` on refreshed G09d
`ab47378f657b8d13d72077009107d223ba04d3a3`, still based on observed main8a475e8b.
[Current review](g10-review.md) records100 units and59 ordinary controls per
source/clean. Shared/indirect kernel-argument capture remains unresolved and
blocking for publication, including injected exec setup. Malformed ELF/interpreter
authority, private descriptor/seal visibility and rollback, protected-memory and
virtual executable-name/readlink runtime checks remain unrun. The full native
exec fixture also has unconditional readlink/protected-pointer probes; local
inherited ordinary checks use the exact earlier G09d image program instead,
without altering that contributed fixture or CI. Reconcile upstream PR#231's
new contract/audit owner before assigning acceptance to a final revision. No
DaybreakBlue agent was dispatched and no security pass is claimed.


## Upstream PR231 configured-contract join — source finding, 2026-11-06

Observed upstream `ff926d0c228b71d55d0f2f0909dd86ec62197c8a` adds contract
resolution and native supervisor enforcement. Source review finds configured
launch remains separate (`launch.rs:1028` dispatch), while the retained G10 task
loop handles task/cwd/exec operations before `launch_tasks.rs:600` fixed dispatch.
Do not assume the new EffectiveContract or bounded audit covers these paths.
Ordinary integration must use upstream's catalog/profile/rule and AuditBudget
owners before effects; include new supported operations in that same catalog
rather than bypassing it. Monitor validation of deny precedence, semantic-family
selection, side-effect ordering, bounded/disabled audit and task/owned hooks on
the final recorded revision. No runtime reproduction or adversarial probe was
executed here, and no DaybreakBlue agent was dispatched. Keep acceptance pending.
The separate G07 shared-kernel-argument finding remains unresolved.


## G03c resolved-contract revision — 2026-11-07

Current G03c is `3801fd45bd6280a5b320f2d464ada0bd04539326`, based on refreshed
G03b/G03a composition85ee433a rooted at upstreamff926d0c. [Review](g03c-review.md)
records96 units and15 ordinary controls per source/clean, plus generated catalog
docs. It integrates upstream rule selection before configured filesystem claims
and scratch rewriting, catalog lstat6/fchmodat268 with native denial preserved,
and existing64KiB audit/disabled-mode metadata. Pure contract units and ordinary
data/logging checks are not monitored enforcement/security acceptance.

The new default Dpolicy fixture cases check raw syscall.open and semantic
filesystem.path.write denial without creating a file; they are compiled and
registered but unrun here. Validate those and broader raw/semantic precedence,
pre-effect ordering, translated/private descriptors, inherited/native file views,
metadata/copy protection, scratch restoration, audit metadata/bounds/disabled
behavior and shared/indirect argument authority on this exact revision or a
recorded successor. All earlier G03c authority/protected/fault probes remain
required. Recheck these joins as device/FD/task packets are refreshed, especially
early task entry in G06b. No adversarial runtime reproduction or cloud agent was
run. G07 remains excluded and unready; keep acceptance pending.


## G04 latest-base resolved-contract revision — 2026-11-07

Current G04 is `8880ddc8fd7b3918707a5a832de47e86f49c9fda` on G03c3801fd45 rooted atff926d0c.
[Review](g04-review.md) records96 units and18 ordinary controls per source/clean.
Device effects are under the configured resolved-contract guard; faccessat269
uses upstream's catalog and existing read-family owner with native denial intact.
Revalidate raw/semantic denial before device opens/descriptor effects, namespace
and assignment authority, hidden image aliases, strict protected/partial metadata
copies, private/public descriptor separation, scratch restoration and shared/
indirect arguments on this head or a recorded successor. Existing full fixtures
and CI remain enabled; their adversarial probes and enforcement cases are unrun.
Ordinary valid null access/metadata is not security acceptance. G07 stays unready;
no DaybreakBlue agent was dispatched. Keep monitored acceptance pending.


## G05 latest-base descriptor/contract revision — 2026-11-07

Current G05 is `2718b860f04bc644c001d7a9cea906a74ddd386a` on G048880ddc8 rooted atff926d0c.
[Review](g05-review.md) records96 units and27 ordinary controls per source/clean.
Resolved policy precedes public slot/alias/pipe effects and path scratch. A pure
lookup on the existing descriptor owner retains regular-file semantic families
through public aliases. The ordinary write-full failure revealed missing pipe22
catalog membership; the corrected catalog retains native Deny38 and the unchanged
lifecycle fixture now passes. This is functional evidence, not security acceptance.

Revalidate raw/semantic denial precedence before reservation/dup retirement/open/
pipe allocation/copies, owned alias classification, private/backend authority,
strict protected/partial pipe and metadata copies, restoration ordering, audit
bounds/disabled mode and shared/indirect arguments on this revision or successor.
Default fixtures and CI remain intact and unrun where adversarial. Earlier G05
security requirements still apply; G06b early task hooks need the same contract
ordering. G07 remains unready. No DaybreakBlue agent was dispatched; acceptance
stays pending.


## Local configured-thread consumer source — 2026-11-07

The ownership split allows ordinary local Recipe integration while Claude owns
upstream work. Codex local source
`/home/vince/levitate-development/ginux-recipe-integration` now has `91e244c1`
(per-task descriptor transactions) and `78f96df1` (bounded libc threads, shared
descriptor/cwd/executable groups and blocked-IO pins), on preserved F02dfebda7c.
Ordinary lifecycle, process/exec and Recipe Bash/tar consumers pass. These are
functional results only. The shared-path/indirect-argument capture finding above
is still unresolved; public/private descriptor authority, copyout faults and
concurrent transaction/teardown attacks have not been rerun. Known storage B1–B3
remain unchanged. Pin any later local source before monitored verification.
This note neither dispatches DaybreakBlue nor accepts those pending criteria.


The local integration now descends from upstream59e2f7aa. Latest functional
source062fb5ef adds bounded task-creation diagnostics and32 task slots for the
measured ordinary Cargo process tree; shared-argument/authority/copyout security
criteria above remain pending. Read the local integration state.json for any
later source before monitored validation. No security agent was dispatched.


### Local musl and Recipe fixed PATH join — 2026-11-07

Local Ginuxe9a2f8a0 accepts musl's exact pthread clone flags0x7d0f00 through
the existing group owner; real static Rust compilation, execution and joining
pass ordinary checks. Existing shared-argument/copyout/authority holds remain.
Recipe input preparation also needs its canonical, generated PATH distinguished
from a host source directory with the same spelling (/usr/bin/cpio imports).
The proposed narrow exemption matches the entire fixed PATH assignment; other
paths retain prefix checking. Review host-path enforcement on its final recorded
source before monitored acceptance. No adversarial probe or dispatch occurred.


### Bounded pipe progress source — 2026-11-07

Local Ginux6a19a70a now retains blocking write/vector progress in DescriptorCall
through full-buffer waits; partial publication wakes readers, admitted mode and
vector layout stay fixed, and descriptor pins retain the existing task lifetime.
A196733-byte ordinary write/writev and real RPM pipeline pass. Review protected
and partial-copy handling, peer/endpoint flag changes, multiple concurrent writers,
close/termination/reuse and indirect-buffer authority on this final source before
monitored acceptance. Existing tests remain enabled; those adversarial probes
were not run here. Recipe0c4f0d4 is the final fixed-PATH guard change referenced
above. No security dispatch occurred; required acceptance stays pending.


### Configured creation-mask source — 2026-11-07

Local Ginux436c4ae5 adds process-group umask state for owned creation. Ordinary
file/directory modes, thread sharing, fork isolation and exec retention pass,
as does actual GCC/SQLite mode0755. Review concurrent creation/mask changes,
policy admission/audit and existing mode/authority criteria on this source before
monitored acceptance. Earlier early-task/resolved-contract and shared-argument
holds still apply. No adversarial enforcement probe or security dispatch occurred.


### Final local source for monitored work — 2026-11-07

Pin Ginux436c4ae5843e4d50f4833158d351523f35669909 (binary96fd7dfa43cda4f051cb7b819cf00a104e2b4cceb91d2ab065e7d19e544e77db),
Recipe runtime0c4f0d4e9d1f84ab7f638611e895dbf799500e7a and test source
2ad833cc93f0e209fe205adb4186a822a13caaea. final-local-review.json reconciles
100 independent passes, one clock-policy failure and34 unrun e2e cases. The33
monitored cases and mixed library/image targets remain enabled/unrun; the two
G02-dependent criteria are separately held by explicit user choice. Native
clock policy was not changed. Current preseed fixtures follow all nine additional
Alpine APK facts and preserve the original hostile quoted-path case for monitored
work. Earlier shared-argument, contract/authority, storage B1–B3, protected copy,
concurrency/teardown and negative transport requirements remain pending. No agent
was contacted or dispatched, and ordinary results are not security acceptance.

### Local security-role check — 2026-11-07

The user authorized this Codex session to perform local low-level security work.
The first four isolated Recipe e2e cases passed against Ginux
`436c4ae5843e4d50f4833158d351523f35669909` and Recipe test source
`2ad833cc93f0e209fe205adb4186a822a13caaea`: private worker bootstrap
rejection, public worker environment spoofing rejection, unsafe ZIP member
rejection, and existing ZIP symlink-component rejection. Exact commands, binary
hashes, exit codes and logs are in the local integration artifact
`daybreak-local-security-2026-11-07/summary.json`. This is local evidence, not
a monitored-cloud result. The prior `final-local-review.json` remains a historical
100/1/34 ledger; the four new passes make 29 of its 33 security e2e cases still
unrun. One other G02-dependent case remains unrun and the observed G02 case
remains failed under the user's explicit clock-policy hold. Known B1–B3 and
other Ginux security checks remain unresolved. No Ginux source change, upstream
push, PR, maintainer contact or agent dispatch occurred.

### Local Daybreak security continuation — 2026-11-07

The user explicitly authorized this session to perform the documented local
low-level security work. This supersedes this handoff's earlier local-probe
boundary for this session; it does not authorize upstream publication, remote
messages, another agent, the held G02 clock policy, or a redesign of storage.
This is local evidence, not a monitored-cloud result.

Recipe test source is `2ad833cc93f0e209fe205adb4186a822a13caaea`;
runtime source is `0c4f0d4e9d1f84ab7f638611e895dbf799500e7a`.
The 29 independently selected security e2e cases with exact exit-zero,
one-test/one-pass receipts ran against Ginux `436c4ae5843e4d50f4833158d351523f35669909`
(binary SHA-256 `96fd7dfa43cda4f051cb7b819cf00a104e2b4cceb91d2ab065e7d19e544e77db`).
The receipt directory is
`/home/vince/levitate-development/ginux-recipe-integration/.artifacts/recipe-integration/daybreak-local-security-2026-11-07/`.
The historical 135-case ledger therefore has 100 earlier ordinary passes,
29 additional security passes, one observed G02 failure, and five unrun cases;
this arithmetic combines receipts on the old pinned Ginux revision and is not
a fresh complete-suite result on the current revision. The five unrun cases
comprise four security cases that directly invoke guest `/usr/bin/sleep` and one
other G02-dependent state case. The user's clock-policy hold remains in force;
none of these cases passed or were substituted. The mixed Recipe library
backend spoofing and image producer negative controls also passed separately
with their own receipts.

Ginux's unchanged full device, pipe, task, FD, thread, poll, network, basic
launch, directory, owned-exec, exec, checkpoint, restore, native supervision,
and process-native suites passed in local runs. The restore suite includes
malformed archives and the complete >2 GiB payload. The full storage suite
**failed** at `view-probes`: an owned `/work/probe` opens and passes `fstat`, but
`faccessat2(fd, "", R_OK, AT_EMPTY_PATH)` returns `ENOENT` instead of zero.
A disposable minimal trace confirmed the same result on the current Ginux
revision `c7b4b73ae141cf6bea44995898f9d8c84fe6e282`; the diagnostic's
root-owned result-file read failed, but its guest stdout and audit were saved
in `view-probe-debug.log`. The separate `symlinks`, `mapping`, `ownership`, and
`timestamps` storage modes passed. No storage redesign or acceptance claim
follows from those passes.

The unchanged full script suite initially failed two `/dev/fd/<directory>/../../`
namespace-escape cases on `436c4ae5`, returning `EINVAL` where the existing
fixture requires `EOPNOTSUPP`. The smallest local integration correction maps
only the above-root resolution error at that descriptor-reopen boundary;
commit `c7b4b73ae141cf6bea44995898f9d8c84fe6e282` records it for Claude to
extract if appropriate. The complete unchanged script suite then passed on
that revision (binary SHA-256
`e064480ac41f78724b661ce893e9ebb737f36c01aaf2d5029e1f1c9c4daa6fa4`),
as did `make check build`, all existing Ginux unit tests, native supervision,
and process-native including its repetitions. The current source remains clean.

Source audit found that this local integration HEAD descends from upstream
`59e2f7aa` and does **not** contain the separate resolved configured-contract
commit `37ddb461` (`git merge-base --is-ancestor` exits 1). Thus the G03c
contract-descriptor truncation finding in the upstream packet must not be
reported as a current local-runtime reproduction. The local source still
routes configured filesystem claims through the descriptor table and launch
filesystem. Its current storage uses mapped namespace arrays and `MemoryStorage`;
the earlier packet's stack-overflow finding also needs a revision-specific
recheck before being asserted here. The image symlink/assignment concern and
all untested shared-argument, policy, copyout, race, teardown, transport, and
source-review requirements remain pending. Do not treat passing broad fixture
families as proof of every handoff criterion.

Exact next action: investigate the current `view-probes` result without changing
Claude's storage redesign; enumerate the remaining handoff criteria against
current-source fixtures and run focused, controlled checks where the owner and
revision are clear. Recheck Recipe on any later integration source before a
current-revision claim. Keep G02-dependent cases held and acceptance open.

### Current-revision focused reruns — 2026-11-07

After the local descriptor-reopen correction, two exact Recipe e2e cases passed
on Ginux `c7b4b73a`: owned executable permission rejection and generated
owned/image interpreter chains. Their `*-c7b4b73a.json` receipts and complete
logs are in the evidence directory above. The unchanged storage runner exposes
24 selectable modes. Each of the 23 modes other than `view-probes` passed
independently on `c7b4b73a`, including bounds, quota, mapping, symlinks,
namespace scale, ownership, positioned/vector/large I/O, timestamps, flush,
export and failure paths. Every mode has its own command, exit code and log
receipt (`ginux-storage-<mode>-c7b4b73a.json`). The `view-probes` failure
persists; default-only path/assignment and no-storage controls remain unrun
because the full runner stops at that failure. These selected passes cannot
make the full storage suite pass.

### Default-only storage selection and fixture correction — 2026-11-07

A disposable selection copy of the existing storage runner isolated its
19 default-only scenario labels without changing the storage fixture body or
Ginux runtime. Eighteen initially passed; `missing-owned` instead entered the
earlier generic mapping branch and exited 135 while awaiting mapping input.
The source had two `argv[1][0] == 'm'` branches, making the intended
`missing-owned` branch unreachable. Local integration commit
`b4c6955116bcbfd186f2e9a03430b7a5815bca3b` changes only the earlier
branch to an exact `mapping` comparison. The pre-correction fixture binary is
preserved in `bin/launch-storage-fixture-pre-dispatch`; the corrected fixture
was rebuilt with the existing Make target. Independent `mapping` and
`missing-owned` selections both pass afterward, with exact receipts and logs
under the evidence directory. The other 18 default-only selection receipts
remain from the preceding fixture revision; the one-line dispatch correction
does not alter their branch bodies. The full unchanged storage runner was
rerun with the corrected fixture and still fails first at `view-probes`
(guest exit 149, runner exit 1). This is a fixture selection fix, not
storage security acceptance or a runtime fix. No remote change was made.

Current local Ginux integration HEAD is `b4c69551`; the immutable runtime
binary is still from `c7b4b73a`, since the later commit changes only fixture
source. Source ancestry checks show neither canonical G03c `3801fd45` nor
the later resolved-contract variant `37ddb461` is an ancestor of this HEAD.
Current `path_hooks::Rewrite::prepare` still writes the chosen native pathname
to tracee stack memory before the kernel consumes it, so the documented
shared-VM thread mutation issue remains source-visible and requires a
revision-specific correction and adversarial verification. No new runtime
thread-race probe or acceptance is claimed here.

### Empty-path descriptor routing correction — 2026-11-07

A temporary diagnostic build proved that Ginux's filesystem hook was never
entered for the failing owned-file `faccessat2(fd, "", R_OK, AT_EMPTY_PATH)`.
The earlier device hook handled empty paths for `fstatat` and `statx`, but not
`faccessat2`; it called the generic path resolver, which returned `ENOENT`
before the owned-file descriptor hook could claim the call. The diagnostic
source was restored exactly after the run. Local integration commit
`6d290d484f7c47d979e847f6daa8049781977860` extends the device hook's
empty-path descriptor routing to `faccessat2` and handles live device
`faccessat2` descriptors there. The existing device fixture now asserts
allowed read access and denied execute access through `AT_EMPTY_PATH`.
The separate fixture dispatch correction remains `b4c69551`.

The immutable final runtime binary is
`/home/vince/levitate-development/ginux-recipe-integration/.artifacts/recipe-integration/bin/ginux-6d290d48`
(SHA-256 `c40cd3a75c5f479f1e761d53d02e3f12c4b748efc0f4b13eeefa2850cd8d7324`).
`make check build`, the existing full Ginux unit suite, and the unchanged full
storage, device and FD launch suites pass on the corrected source/binary.
The full storage suite now reaches all of its path, namespace, mapping,
copy-protection, quota, export, and invalid-assignment controls; its historical
`view-probes` failure is fixed on this revision, not erased from prior receipts.
Three focused Recipe e2e cases also pass on this binary: absolute host-path
write rejection, owned executable permission rejection, and generated
owned/image interpreter chains. Exact commands, exit-zero receipts, hashes
and logs are in the local evidence directory above. The other 27 security
Recipe passes remain on the older `436c4ae5` binary; no current-revision
135-case ledger pass is claimed.

The broader Daybreak requirements still need revision-specific source review
and focused adversarial work, especially shared writable syscall arguments,
thread/descriptors/teardown races, contract admission and audit, assignment
symlink authority, protected copyout, negative transport and restore inputs.
The G02 clock-dependent five cases remain held/unrun or failed as previously
recorded. Claude's upstream storage redesign remains separate. No remote
push, PR, upstream comment or agent dispatch occurred.

### Current-revision Recipe security selection complete — 2026-11-07

`security-ledger-6d290d48.json` reconciles all 33 previously unrun Recipe
security e2e names against the immutable Ginux `6d290d48` binary: **29 pass**
with individual exit-zero, one-test/one-pass logs and matching binary hashes;
**four remain unrun** because they directly invoke guest sleep under the
user's G02 hold. Two of those 29 had passed on this revision before the batch;
the other 27 were rerun individually. No case failed or timed out in this
current-revision security selection. The original 100 ordinary Recipe passes
remain historical evidence on `436c4ae5`; one G02 case failed there and one
other G02-dependent case remains unrun. The current-revision security selection
therefore does not imply a complete 135-case suite pass. Broader Ginux
security criteria in this handoff remain open as listed above.

### Current-source G07 controlled security checks — 2026-11-07

The original G07 `x-path` fixture was reconstructed from the retained
`G07/unready-source.patch` into disposable local fixture/runner copies, with
only controlled image files. On immutable Ginux `6d290d48` (SHA-256
`c40cd3a75c5f479f1e761d53d02e3f12c4b748efc0f4b13eeefa2850cd8d7324`),
the scenario returned guest exit 99. A diagnostic-only fixture variant made
thread/join error exit 100 and hidden-byte exposure exit 99; it again returned
99. The running writer alternated `/allowed` (image byte `A`) and `/work/hh`
(image byte `S`, hidden by `memory:/work`). The reader saw `S`. This is a
current-revision reproduction of the shared writable pathname authority gap,
not just the historical source concern. The exact fixture/runner sources,
commands, binary and source hashes, and complete failing logs are under the
local evidence directory as `g07-xpath-*` and
`ginux-g07-xpath-distinguished-6d290d48.json`. The Ginux source tree was not
changed for this probe.

The recovered G07 `qfault` scenario passed on the same binary: unmapped,
read-only and cross-page clone child-TID copyout was rejected with `EFAULT`,
the probed bytes remained unchanged, and a valid thread joined afterward.
The recovered `dup-race` scenario also passed: concurrent open/read/close and
public descriptor alias operations completed within their expected results.
Exact separate receipts are `ginux-g07-qfault-6d290d48.json` and
`ginux-g07-dup-race-6d290d48.json`. Each run used its own recorded disposable
fixture hash; they do not make the shared-path criterion pass. Teardown,
indirect kernel arguments, SIGCHLD disposition and broader concurrency
criteria remain separately unverified on this source.

The source still gives the kernel a pointer into writable tracee memory for
native pathname operations. `path_hooks::Rewrite` also uses tracee stack
scratch, which threads sharing the VM can write. An ordinary copy to another
writable tracee address would not close this race. Security acceptance of G07
is **failed** pending a design that stabilizes every kernel-consumed pathname
and indirect argument across shared-VM threads, followed by the controlled
race and regression suites on the corrected revision. Do not replace the
existing image/native filesystem behavior with blanket denial: Recipe's
threaded compiler consumers use it. Keep the correction local to the
integration source if needed for Recipe; Claude owns upstream extraction and
publication. No push, PR, upstream comment or agent dispatch occurred.

### Current-source storage prerequisite and stack checks — 2026-11-07

On the immutable `6d290d48` binary, a disposable image containing `/link -> /work` and an assignment `memory:/link/nest` launched successfully (result `exited/0`). The exact source script, SHA-256 `d20c40eaa5d1cb81fa7d73aa50134beffcba4abfb50408cea81742efc3443fcd`, and `ginux-b1-assignment-precondition-6d290d48.json/.log` are in the local Daybreak evidence directory. Current `LaunchFs::prepare` checks assignment spelling and reserved prefixes but does not reject an image symlink above an assignment. This proves **acceptance of the configuration**, not a host filesystem oracle or a guest read across that boundary. The existing resolver and its source-specific behavior still require separate examination. Claude's upstream storage redesign proposes startup rejection; do not import that design here or claim it already protects this integration revision.

The old G03c B3 description does not describe this exact source: `LaunchFs` holds `MemoryFs`, but namespace and storage-metadata arrays use `MappedArray` backed by `mmap`, and data backing is allocated separately. The instance parser rejects `memory_mib = 2` (minimum 16), so the first 2 MiB probe was invalid and returned `supervisor-error/2`; receipt `ginux-b3-stack-3mib-6d290d48.json`. A valid 16 MiB instance with the process stack capped at 3 MiB exited zero both without an assignment and with `memory:/link/nest`; receipts `ginux-b3-stack-3mib-no-storage-6d290d48.json` and `ginux-b3-stack-3mib-assigned-6d290d48.json`. One intermediate run used the fixture's `no-storage` expectation with `memory:/work` and exited 10 by design; receipt `ginux-b3-stack-3mib-valid-6d290d48.json`. These runs support a narrow current-revision small-stack result, not a general stack bound. Scripts and hashes are saved beside the receipts. No Ginux source or remote changed. G07's current-revision shared-path authority failure remains unresolved and security acceptance remains failed.

### Current-source G06b wait-status fault control — 2026-11-07

The unchanged `qfault` task fixture passed on immutable Ginux `6d290d48` through the existing `scripts/test-launch-tasks.py` selector (`PASS launch tasks: qfault`, exit 0). Its fixture asserts four protected/unmapped/cross-page wait-status destinations return `EFAULT`, that a second wait reports the child already consumed (`ECHILD`), and that protected and adjacent sentinel bytes remain unchanged. Exact binary, fixture and runner hashes, command and log are in `ginux-g06b-wait-qfault-6d290d48.json/.log` in the local Daybreak evidence directory. This is current-revision evidence for those four fixture cases; it does not resolve G07's shared-path failure or all indirect-copyout requirements.

### G07 inherited SIGCHLD and teardown controls — 2026-11-07

On `6d290d48`, the existing `clone` task fixture passed with Ginux alone inheriting ignored `SIGCHLD` (`ginux-g07-clone-sigchld-childonly-6d290d48.json`; corrected runner source and hash saved beside it). A recovered true-thread fixture then passed `basic` and both `teardown` modes (external SIGKILL and supervisor-failure SIGSTOP): the runner asserted the expected launch result and that all tracee TIDs disappeared. Exact receipts are `ginux-g07-basic-sigchld-childonly-6d290d48.json` and `ginux-g07-teardown-sigchld-childonly-6d290d48.json`; the disposable runner source and SHA-256 `0a81e33e4c47a46a6d41583d9063f6dad911ad08b40fef6240b34b23df624553` are saved beside them.

An earlier wrapper set ignored `SIGCHLD` in the Python runner itself. Its `basic` receipt looked successful, but the runner could not reliably collect child exit codes; its `teardown` receipt has a traceback even though the wrapper reported exit zero. The earlier `ginux-g07-clone-ignored-sigchld-6d290d48.json` and both `*-ignored-sigchld-6d290d48` thread receipts are **invalid as acceptance evidence** and retained for diagnosis. The corrected runner changes disposition only in each launched Ginux child, so Python retains reliable `waitpid` behavior. These passing controls do not repair or override the reproduced G07 shared-path authority bypass; G07 acceptance remains failed.

### Current-source restore rejection and FIFO input — 2026-11-07

The unchanged full `scripts/test-launch-restore.py` suite passed on the immutable `6d290d48` Ginux binary and the corrected storage fixture: `test-launch-restore: PASS (roundtrip, startup isolation, parent ordering and malformed inputs)`, exit 0, in 26.164 s. The exact runner, fixture and binary hashes, command and complete output are in `ginux-restore-full-6d290d48.json/.log` in the local Daybreak evidence directory. This suite includes the complete valid >2 GiB payload and 5000-object controls, plus rejection of assignment mismatch, truncated/checksum-damaged archives, traversal, symlink/hardlink directory entries, duplicates, malformed hardlink targets/metadata and a symlink archive source. It does not inject FD/read/close failures or race a mutable source.

A separate disposable FIFO source test (`daybreak-restore-fifo.py`, saved beside its receipt) returned launch exit 2 with `supervisor-error`, no guest stdout and `restore archive is not a bounded regular file` in 0.066 s; no timeout or blocking open occurred. Receipt: `ginux-restore-fifo-6d290d48.json/.log`. Current `storage_restore.rs` opens with `O_NONBLOCK|O_NOFOLLOW` and checks `fstat` for a bounded regular file before parsing. These observations cover the tested FIFO and archive inputs only; hostile replacement, concurrent mutation, arithmetic/failure injection and every G21/G22 authority edge remain pending. G07's path race remains a failing criterion.

Source review of `6d290d48` also confirms that configured dispatch can classify a pathname read from guest memory and `path_hooks::Rewrite::prepare` later calls `native_path`, which reads the guest path again before putting a translated name in guest stack scratch. The original and scratch memory are shared with guest threads. A single snapshot of the original path would close only the first interval; the kernel-consumed scratch must be protected from concurrent guest mutation or native path calls must be performed without a guest-writable pointer. The current TRACEME attachment does not support `PTRACE_INTERRUPT` for safe peer stopping (per [Linux `ptrace(2)`](https://man7.org/linux/man-pages/man2/ptrace.2.html)); no speculative partial fix was committed. Exact next dependent action remains a complete stable-argument design and controlled race regression, preserving Recipe's threaded native/image filesystem behavior.

### Seized configured-launch prerequisite — 2026-11-07

Local integration commit `f9a584e9e15f227c1c4d5f118360da12019bc4d2` switches only configured launch from `PTRACE_TRACEME` to a private-pipe-gated `PTRACE_SEIZE` before guest execution. Auto-attached child birth stops use the seized event form; real group stops still fail explicitly, and the initial exec-exit stop no longer produces a phantom sequence-zero audit record. The native lane retains its prior tracing. A disposable privileged prototype first confirmed the [Linux `ptrace(2)`](https://man7.org/linux/man-pages/man2/ptrace.2.html) seize/exec-stop behavior. The first integration smoke failed because `pipe2` writes two 32-bit FDs and the initial patch used a 64-bit array; the corrected ABI and that failure are retained in the local history.

The final source built in a dedicated Cargo target directory with strict ginux-init clippy passing. The unchanged **full** configured launch, task, and thread suites pass on the commit's immutable binary SHA-256 `0dca4cc81193b3869e7a9377fadda4678479086518bccec214496510bf77ed0b`. Exact commands, hashes and logs are linked by `ginux-seize-bootstrap-f9a584e9.json` in the local Daybreak evidence directory. The ordinary G07 `x-path` adversarial rerun on that same binary still returns guest99 and runner1 (`ginux-g07-xpath-seize-bootstrap-f9a584e9.json/.log`): seized attachment alone does not prevent shared writable path mutation. No peer-interruption scheduler or stable-argument correction has been implemented yet. G07 acceptance remains **failed**, and Recipe consumer verification still targets the prior `6d290d48` binary until a complete correction is built and rerun. No remote push, PR, upstream comment or Claude packet edit occurred.


### Local configured path hold — 2026-11-07

The current local integration source is clean commit `e0519b483dac254ec2062e3201f511ea9c4b6d31` (parent `f9a584e9`). It stops running configured tasks before supervisor path classification and holds them through the native syscall's kernel use of its guest-stack rewrite. It also holds parked tasks from waking, defers pending clone/fork until their guaranteed ptrace stops, retains the virtual child reservation across an internal `-513` fork restart, and resumes synthetic interrupt stops. No upstream branch, PR or remote was changed.

The regression added to Ginux's existing thread fixture and runner selects `x-path`: a writer changes the pathname between image `/allowed` and storage-hidden `/work/hh`; the reader must never observe the hidden file's `S` byte. The new selector on preserved `bin/ginux-f9a584e9` (SHA-256 `0dca4cc81193b3869e7a9377fadda4678479086518bccec214496510bf77ed0b`) failed with guest 99 and runner 1; receipt `ginux-g07-source-regression-baseline-f9a584e9.json`. The exact final binary is `bin/ginux-path-guard-final-candidate`, SHA-256 `3807a25284d442c8e456e8293fbb2b536acc3caf8bbaa3b1827632e631b3d0ab`. The unchanged full launch, task, and thread runners (now including `x-path`) passed on it; receipt `ginux-g07-source-regression-final-suites.json` and per-suite logs. `make check build` with `CARGO_TARGET_DIR=.artifacts/recipe-integration/daybreak-target` passed; receipt `ginux-path-guard-final-check-build.json`.

The exact ordinary Recipe Cargo consumer on that final binary and fresh build/state directories passed with exit 0 in 81.708 s (`recipe-cargo-path-guard-final.json/.log`), exercising actual threaded Rust build and install hooks. Earlier global-guard candidates failed this consumer at the first hook after about 86 seconds: `recipe-cargo-global-guard.json` and `recipe-cargo-guard-diagnostic.json` record `path peer interrupt timed out`, and the diagnostic identified owner virtual task 2 waiting on peer 1's pending clone. A natural-stop variant also failed (`recipe-cargo-guard-all-creation.json`); the final implementation defers that in-kernel clone until the path hold ends, and the same consumer then passed (`recipe-cargo-guard-deferred-creation.json`). All receipts remain under `.artifacts/recipe-integration/daybreak-local-security-2026-11-07/` in the local Ginux integration source. Interrupted, failed or unrun checks were not counted as passes.

This is a narrow G07 path-confinement result, **not** complete G07 security acceptance. Indirect guest arguments outside pathname syscalls, shared mapping and exec transitions beyond the controlled fixture, protected copyout, policy/authority controls, concurrency teardown, negative transport and the remaining handoff criteria still require source-specific checks. The full current-source Recipe suite and remote CI remain unverified; the G02 guest clock/sleep hold remains in force. Exact next action: inspect and test the remaining indirect-argument surfaces on `e0519b48` in dependency order, then rerun affected Recipe consumers. Claude decides upstream extraction; no push, PR, comment or packet preparation occurred.


### Multistage exec hold and next indirect-argument owner — 2026-11-07

The current local integration HEAD is clean `1c2b1333c6a03fcd8f190dfb8921ce2583981be9`, on `e0519b48`. The earlier path-guard commit released at the first exec setup syscall exit; this one-line follow-up retains the hold while `Task.exec` exists, through kernel exec completion or failed-exec cleanup. The exact binary is `bin/ginux-exec-hold-experiment`, SHA-256 `15bb49d12b34b8d8a0d57126121e7aca70fdc424d7cfa3ef50ac2af530b7bca2`. `ginux-exec-hold-suites.json` links passing full launch, task, thread and exec suite logs. `recipe-cargo-exec-hold.json/.log` records a fresh ordinary Cargo Recipe build/install pass in 81.924 s on this binary. A controlled live-peer exec call returns the existing `-95` policy and both binaries terminate cleanly (`ginux-g07-live-peer-exec-corrected-results.json`); initial disposable fixture runs timed out because the writer loop was not stopped after the expected denial, and those first runs are not Ginux-failure evidence.

Next security dependency: `MappingCall::new` in `crates/ginux-init/src/file_mapping.rs` writes a 256-byte socket/message frame into guest stack scratch before replaying native `socketpair`, `recvmsg`, `mmap` and private-FD closes. The path guard does not cover syscall 9 or hold across this mapping state machine. Shared tasks can mutate those kernel-consumed bytes or see transient descriptors while the supervisor proceeds. Source review establishes a pending security surface, not a demonstrated exploit on this exact source. Verify a complete hold or another stable-argument design across every mapping stage, failure cleanup, fixed-region and nonleader cases, then run ordinary mapping/Recipe regressions. Keep G07 acceptance and all other unverified handoff criteria pending; G02 clock policy remains held. Claude owns upstream extraction, and no push, PR or upstream comment occurred.


### Owned mapping scratch hold — 2026-11-07

Current local integration HEAD `ad364bd76b8d54b58c90a6a19f3199b35c025eee` extends the configured argument guard to owned-file syscall 9. The same admission predicate is used for initial syscall stops and descriptor-snapshot retries. Other running tasks are held before `MappingCall::new` writes its 256-byte guest-stack socket/message frame and remain held through pair creation, backing transfer, received-descriptor mapping, private closes and scratch restoration. A new `y-mapping` positive scenario in Ginux's existing thread fixture runs an owned mapping while a peer spins. The final binary is `bin/ginux-mapping-arg-hold-retry-candidate`, SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d` (same hash after rustfmt).

`ginux-mapping-arg-hold-retry-suites.json` links passing unchanged full storage, thread and exec suites, including the new concurrent mapping case. `make check build` passed after one formatting-only correction. `recipe-cargo-mapping-arg-hold-retry.json/.log` records a fresh ordinary Cargo Recipe build/install pass in 83.841 s. The first `make check build` attempt stopped at rustfmt's requested line break and is not a pass; the rerun passed. A pre-retry-predicate candidate also passed full storage and positive `y-mapping`, but it did not guard parked mapping retries and is superseded.

Adversarial mapping acceptance remains **pending**: the guest-scratch race is now covered by the focused regression below, while temporary private descriptor observation/use during each stage, nonleader/leader-exit, hostile fixed-map, failure-injection and malformed-receipt controls remain incomplete. The exact next action is to construct bounded disposable probes for private-FD authority and these remaining surfaces on the current integration source, preserving the existing positive mapping controls and recording all outcomes. The other G07/G12, policy, copyout, transport and restore criteria remain open; G02 clock/sleep is held. Claude owns upstream extraction. No remote push, PR or comment occurred.


### Reproducible shared mapping scratch probe — 2026-11-07

Local Ginux integration test commit `167f57b57a0e5da852099d359c214cf477415aa1` adds `k-scratch` to the existing launch-thread fixture and runner. Its peer watches only the disposable workload's guest-stack mapping message frame and alters the iovec pointer if it becomes visible while the peer runs. `ginux-g12-integrated-scratch-repeat.json` records exact runner, fixture and both binary hashes: the pre-guard `1c2b1333` binary failed **20/20** with `mapping backing receipt failed`, while the guarded `ad364bd7` binary passed **20/20**. The first-failure log and earlier disposable probe sources/receipts remain in the Daybreak evidence directory. The full Ginux thread suite including `k-scratch` and `make check build` pass after this test-only commit; the runtime binary is unchanged from `ad364bd7`. The current local integration HEAD is clean `167f57b5`. This is evidence for this specific shared-scratch mutation, not proof of temporary FD authority, nonleader or failure-path safety. Exact next action: test private descriptor observation/use during mapping stages with controlled disposable peers, then remaining G07/G12 checks. No upstream action occurred.


### Private mapping descriptor guest-authority probe — 2026-11-07

The local Ginux integration test commit `c3a3f7b515b16d782f875f05329961f204a90a47` adds `v-private` to the existing thread fixture and runner. While the owner performs 64 owned-file mappings, a peer loops over candidate private native FD numbers 5–7, requiring `fstat`, `fcntl(F_GETFD)`, `close` and file `mmap` to return `EBADF`. The focused case and full thread suite passed on the unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`; `make check build` passed. `ginux-g12-private-fd-guest-probe.json/.log` records the exact command, duration, hashes and full output. Descriptor source review shows `DescriptorTable::get` gates the probed guest calls and `MappingCall` uses native FDs only in its private replay while the argument guard holds peers. This checks guest-visible authority during concurrent mappings but does not force attempts at each Pair/Receive/Map/Close stage. Nonleader/leader-exit, fixed-map and failure-path checks remain pending. Exact next action: nonleader mapping and post-leader-exit controls. No upstream action occurred.


### Nonleader and surviving-thread mapping controls — 2026-11-07

Local integration test commit `4075d9d599f7934b51bcc33ef1eb855743e6f145` adds `n-mapping` (nonleader owned-file map/unmap) and `j-leader-map` (same operation after the runner observes the group leader as a zombie). Both focused cases and the full thread suite pass on the guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`; `make check build` passed. `ginux-g12-nonleader-leader-exit.json/.log` records command, hashes, duration and full output. The first edit failed to compile and the ensuing stale-fixture guest99 is invalid evidence; the corrected rebuild passed. These cases establish ordinary mapping and cleanup in those two thread lifecycles, not fixed-map interference, injected failure rollback or malformed receipt safety. Exact next action: fixed-region denial and preservation. No upstream action occurred.


### Fixed-region owned-mapping denial — 2026-11-07

Local integration test commit `fdeae41bd0b1c38edbf52af69cfcb18c516c9102` adds `q-fixed` to the existing thread fixture and runner. `MAP_FIXED` over the page containing the executing syscall and over its stack scratch both return `-ENOTSUP`; the stack sentinel survives and a following ordinary owned mapping works. The focused case, full thread suite and `make check build` pass on immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. The exact source, fixture, runner and binary hashes and full output are in `ginux-g12-fixed-region.json/.log`. Failure rollback, malformed receipt and other hostile layouts remain unverified. Exact next action: failure-path controls. No upstream action occurred.


### Failed owned-mapping rollback — 2026-11-07

Local Ginux integration test commit `836061e13cf72ee05ca45c57b546de034c8e7855` adds `w-rollback`. A page-aligned high offset reaches the private owned-file mapping transaction and returns a kernel error; the runner confirms `mediate surface=owned-file-mapping`. With the guest alive at two handshakes, `/proc/<tracee>/fd` is `[0, 1, 2, 3]` before and after the failed mapping. The stack sentinel and a subsequent ordinary map/unmap pass. The focused case, full thread suite and `make check build` pass on immutable binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g12-failed-map-rollback.json/.log` has exact command, source/runner/fixture/binary hashes, duration and output. This is one kernel-map-error cleanup path, not injected socket/receipt/close failure or malformed-receipt evidence. Exact next action: malformed receipt and private-stage failure controls. No upstream action occurred.
