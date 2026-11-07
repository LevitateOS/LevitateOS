# DaybreakBlue security validation handoff

Requested by the user on 2026-11-06 as a written handoff for the monitored
DaybreakBlue blue-team agent. No cloud agent has been contacted or dispatched.
Keep this record in LevitateOS; it is not Ginux contribution text.

## Boundary and status

On 2026-11-07 the user designated this chat as the Daybreak security model and
explicitly authorized local security validation on disposable fixtures for the
documented Daybreak scope. This supersedes the earlier no-local-probe boundary
for that scope. Use only authorized disposable local fixtures/images; preserve
Ginux tests and CI, record exact revisions and results, and leave unrun or failed
criteria pending. The G02 clock-policy hold remains. Claude owns upstream Ginux
PR work; do not push, open PRs, comment upstream, or dispatch a cloud agent.

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

- G09a d29381c7: review malformed image offsets and backend failure contracts.
  A selected current-revision offset-wrap probe failed before local fix `b168c2c5`
  and passed after it. Other adversarial index and backend cases remain pending;
  ordinary VFS data/truncation/rollback units are separate evidence.
- G09b 297ad8dd: validate anonymous mmap/mremap rollback and indirect memory-copy
  protections under adversarial workloads, plus inherited namespace confinement.
  Ordinary growth, quota, zero-fill and forked-alias checks pass in both source
  and clean replay. Full storage suite retains the older confinement/protected
  buffer scenarios; these were not rerun by this agent. The configured thread
  snapshot is deliberately excluded from this candidate.

### G09a current malformed file extent result — 2026-11-07

A disposable two-record image index on local integration `158e11a6` declared a
2-byte file starting at `u64::MAX`. The valid offset-zero control read backing
byte `B` at file position 1. The malformed index was accepted and the same read
returned backing byte `A` at offset zero, showing release-build arithmetic wrap.
Local integration commit `b168c2c5` makes `Index::new` reject a file extent whose
`data_offset + size` overflows. The corrected release probe returned
`Err(InvalidIndex)` for the malformed record, while the valid read still returned
`B`; a byte-identical copied VFS crate ran all 42 unit tests successfully.
The ignored `ginux-current-g09a-offset-wrap-b168c2c5/` evidence bundle retains
both probe sources and outputs, source snapshots, test logs, exact hashes and
commands. A separate `ginux-current-g09a-backend-failure-b168c2c5/` control
returned `Err(Io)` for a simulated short backing read without advancing the VFS
position, then read `AB` after the second byte became available. It did not run
a real host short-read or concurrent mutation. Other malformed records and
configured runtime behavior remain unverified. The integration fix is local
extraction material for Claude, with no upstream push, PR or comment.

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


### Current selected G11 cross-page vector-table result — 2026-11-07

On configured runtime from integration `158e11a6`, an owned-script exec with
its argv pointer table split across a writable and `PROT_NONE` page returned
`EFAULT`; the analogous envp table did too. The envp first pointer stayed
unchanged. A later valid owned-script exec in a child exited 77 and was reaped;
launcher result and complete teardown passed. The ignored
`ginux-current-owned-vector-crosspage-158e11a6/` bundle retains exact source,
runner, audit and hashes. The checkout had the later VFS-only `b168c2c5` fix,
but the pinned runtime binary did not. This selected boundary leaves shared
mutation, other alignments, pointee/auxv limits and full G11 acceptance open.

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

### Current selected G07 race run — 2026-11-07

The unchanged `x-path` fixture ran once on configured binary from integration
`158e11a6` and reported PASS across its 2,048 selected calls. This stochastic
negative result alone does not prove G07 acceptance. The earlier `6d290d48`
hidden-byte exposure predates local commit `e0519b48`, which stops peers before
pathname classification and holds them through the kernel syscall. Source review
shows that the current stack scratch is protected by this guard; the earlier
current-source implication that writable scratch alone left this race open was
incorrect. The paired scratch regression below provides stronger evidence. The
ignored `ginux-current-g07-xpath-selected-158e11a6/` bundle records exact
inputs and output. Broader G07 authority remains partial.

### Current-source storage prerequisite and stack checks — 2026-11-07

On the immutable `6d290d48` binary, a disposable image containing `/link -> /work` and an assignment `memory:/link/nest` launched successfully (result `exited/0`). The exact source script, SHA-256 `d20c40eaa5d1cb81fa7d73aa50134beffcba4abfb50408cea81742efc3443fcd`, and `ginux-b1-assignment-precondition-6d290d48.json/.log` are in the local Daybreak evidence directory. Current `LaunchFs::prepare` checks assignment spelling and reserved prefixes but does not reject an image symlink above an assignment. This proves **acceptance of the configuration**, not a host filesystem oracle or a guest read across that boundary. The existing resolver and its source-specific behavior still require separate examination. Claude's upstream storage redesign proposes startup rejection; do not import that design here or claim it already protects this integration revision.

The old G03c B3 description does not describe this exact source: `LaunchFs` holds `MemoryFs`, but namespace and storage-metadata arrays use `MappedArray` backed by `mmap`, and data backing is allocated separately. The instance parser rejects `memory_mib = 2` (minimum 16), so the first 2 MiB probe was invalid and returned `supervisor-error/2`; receipt `ginux-b3-stack-3mib-6d290d48.json`. A valid 16 MiB instance with the process stack capped at 3 MiB exited zero both without an assignment and with `memory:/link/nest`; receipts `ginux-b3-stack-3mib-no-storage-6d290d48.json` and `ginux-b3-stack-3mib-assigned-6d290d48.json`. One intermediate run used the fixture's `no-storage` expectation with `memory:/work` and exited 10 by design; receipt `ginux-b3-stack-3mib-valid-6d290d48.json`. These runs support a narrow current-revision small-stack result, not a general stack bound. Scripts and hashes are saved beside the receipts. No Ginux source or remote changed. G07's current-revision shared-path authority failure remains unresolved and security acceptance remains failed.

### Current-source G06b wait-status fault control — 2026-11-07

The unchanged `qfault` task fixture passed on immutable Ginux `6d290d48` through the existing `scripts/test-launch-tasks.py` selector (`PASS launch tasks: qfault`, exit 0). Its fixture asserts four protected/unmapped/cross-page wait-status destinations return `EFAULT`, that a second wait reports the child already consumed (`ECHILD`), and that protected and adjacent sentinel bytes remain unchanged. Exact binary, fixture and runner hashes, command and log are in `ginux-g06b-wait-qfault-6d290d48.json/.log` in the local Daybreak evidence directory. This is current-revision evidence for those four fixture cases; it does not resolve G07's shared-path failure or all indirect-copyout requirements.

### Selected current G06b copyout rerun — 2026-11-07

The existing task fixture/runner selected `qfault`, `protected-child-tid` and
`x-child-tid` on configured binary from integration `158e11a6`; all three passed
with result `exited/0` and complete teardown. `qfault` covers four wait-status
fault destinations, `EFAULT`, consumed-child `ECHILD` and unchanged sentinel
bytes. The child-TID cases cover a protected word and a writable/read-only
cross-page word with unchanged child sentinel. The ignored
`ginux-current-g06b-copyout-158e11a6/` bundle retains exact fixture/runner,
logs, hashes and receipt. The later VFS-only local fix `b168c2c5` was not built
into this runtime. These selected checks leave shared-path authority, clone
flag combinations and broader rollback acceptance unresolved.

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


### Private socketpair EMFILE cleanup — 2026-11-07

Local Ginux integration test commit `20b246ce955466dfc801b074d05164573fc8232b` adds `a-pair-fail`. A runner handshake lowers only the disposable tracee's soft `RLIMIT_NOFILE` to 5. The private mapping Pair stage returns `-EMFILE`; the live tracee has native FDs `[0, 1, 2, 3]` both before and after, its stack sentinel survives, and an ordinary owned mapping succeeds after the limit is restored. The focused case, full thread suite and `make check build` pass on unchanged binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g12-pair-emfile-rollback.json/.log` records exact command, hashes and output. This is only the Pair-stage resource-error path; malformed Receive and injected send/close failures are pending. Exact next action: inspect and exercise those later stages. No upstream action occurred.


### Current guarded-source clone/wait copyout controls — 2026-11-07

The unchanged `scripts/test-launch-tasks.py` and task fixture passed selected `junk-child-tid`, `protected-child-tid`, `x-child-tid`, `rollback` and `qfault` scenarios on immutable guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g06b-copyout-current-guarded.json/.log` records exact command, hashes, exit 0 and output. This refreshes current-revision evidence for these five fixtures; other indirect copyout paths and G07 acceptance remain open. Exact next action: later-stage mapping receipt/failure checks and remaining indirect-argument surfaces. No upstream action occurred.


### Host-injected mapping receipt corruption and tracee death — 2026-11-07

Disposable `g12-malformed-receipt-host-probe.py` and `g12-mapping-death-host-probe.py` used a derived, hash-recorded fixture and unchanged guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. The first externally changed the guest-stack mapping frame's iovec pointer when it appeared; the exact binary returned exit 1/result `supervisor-error`, reported `mapping backing receipt failed`, and logged `teardown tasks=0 owned-handles=0`. The second externally killed the tracee while that frame existed; the binary returned exit 1/result `supervisor-error`, reported `task died during private mapping transaction`, and logged complete teardown. Exact commands, output, scripts and hashes are in the corresponding `ginux-g12-*-host-probe.json/.log` files in the local Daybreak evidence directory. These are external host fault injections that validate failure handling; they do not establish a guest authority bypass or cover send/close failures. Exact next action: remaining private transfer failure paths and G07/G12 indirect-argument controls. No upstream action occurred.


### Current guarded-source mapping protection/lifetime controls — 2026-11-07

Unchanged `scripts/test-launch-storage.py` and fixture selected `mapping`, `ordinary-mapping` and `ordinary-unlinked-mapping` all passed on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g12-mapping-current-guarded.json/.log` records exact command and hashes. The fixture tests writable-shared and executable-write denial, offset/fixed-region checks, shared/private aliasing, resize/SIGBUS, copy-on-write, unlinked backing lifetime and a live native-FD snapshot after mapping. These are bounded current-source results, not exhaustive G12 security acceptance. Remaining private send/close errors and other indirect-argument authority controls stay pending. No upstream action occurred.


### Current guarded-source configured view probes — 2026-11-07

The existing `view-probes` storage selector passed on immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g09-view-probes-current-guarded.json/.log` records exact command, hashes, exit 0 and output. Fixture assertions include owned-file access permissions, AT-style and fd empty-path checks, regular-file readlink rejection and a permitted read. This is one current-source configured-view check, not exhaustive namespace or symlink authority validation. Exact next action: symlink/cross-assignment checks and remaining G07/G12 private-transfer controls. No upstream action occurred.


### Full guarded-source configured storage suite — 2026-11-07

The unchanged full Ginux storage runner passed on immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d` in 6.186 s, exit 0. `ginux-g09-storage-full-current-guarded.json/.log` has exact command, hashes and output. It includes default cross-boundary/hidden path, image/owned symlink, scratch restoration, view/access/readlink, mapping, bounds/quota and namespace cases. Selected `symlinks` and `ordinary-symlinks` also passed. This is current-source fixture evidence, not exhaustive hostile namespace-race or all G07/G12/Daybreak security criteria. Exact next action: source-specific remaining shared-argument and authority gaps, then other documented security surfaces. No upstream action occurred.


### Current guarded-source exec/interpreter and owned-script suites — 2026-11-07

The unchanged full `scripts/test-launch-exec.py` passed five interpreter ownership starts and seven exec scenarios, including descriptor/path/relative/link selection, private native FD checks and CLOEXEC/rollback. The unchanged full `scripts/test-launch-scripts.py` passed all 36 named owned-script scenarios, including nested, descriptor, unlinked, directory, process creation and cross-boundary joins, with live private FD and rollback checks. Both used immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g10-exec-current-guarded.json/.log` and `ginux-g11-scripts-current-guarded.json/.log` have exact commands, hashes and output. Malformed/protected argv and shared-argument edge coverage beyond those fixtures remains pending. Exact next action: source-review and focused G10/G11 indirect arguments. No upstream action occurred.


### Forked shared-path exec regression — 2026-11-07

Local integration test commit `8601b606d81f478783ef03367e9d1fb1fac926e9` adds a forked writer sharing an anonymous pathname page with an exec caller. The path alternates between `/bin/fixture` and image `/work/hidden` under `memory:/work`; a marker script at the hidden path exits 99 if reached. The full exec suite including this case passed 20/20 on unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`, and `make check build` passed. `ginux-g07-shared-exec-final-repeat.json` has exact source/fixture/runner/binary hashes and outcomes. An earlier runner revision on preserved pre-guard `f9a584e9` failed 5/5 by exhausting retries with `ENOENT`; it did not show hidden execution. This covers one cross-process shared exec pathname and teardown, not all argv/env, interpreter or indirect-argument authority. Exact next action: those remaining inputs and private transfer send/close paths. No upstream action occurred.


### Protected cross-page exec vectors — 2026-11-07

Local integration test commit `6cc21e69c25672672fd2838a70708d8b156ca16b` adds protected cross-page argv and env vector assertions to the existing failed-exec fixture. Both return `EFAULT` and preserve its descriptor/CLOEXEC rollback state. The full exec suite and `make check build` pass on immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g10-crosspage-exec-current-guarded.json/.log` records exact command, hashes and output. These two vectors do not exhaust malformed script-stack bounds or shared argument timings; private transfer send/close faults also remain unverified. Exact next action: remaining G10/G11 indirect exec inputs and script stack. No upstream action occurred.


### Protected cross-page owned-script vectors — 2026-11-07

Local integration test commit `b91b69c1df82d32daf1c1fa875d6011349eca6f6` adds protected cross-page argv and env vectors to the failed script-exec fixture. Both return `EFAULT` and preserve descriptor/CLOEXEC rollback. The full 36-case script suite and `make check build` pass on unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g11-crosspage-script-current-guarded.json/.log` has exact command, source/runner/fixture/binary hashes and output. The initial new fixture attempted mmap without an explicit sixth offset argument and failed with `EINVAL` before exec; that is invalid Ginux evidence. Corrected source supplies offset zero. Other script stack/AT_EXECFN and shared mutation surfaces remain open. Exact next action: inspect those edges and private transfer send/close paths. No upstream action occurred.


### Confirmed high-bit public descriptor alias — 2026-11-07

Current `DescriptorTable::get(u64)` truncates to `i32`. A disposable, hash-recorded guest opened `/input` as public FD 4 and read through `0x100000004`; on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`, it received the `a` byte and exited with marker 99. `ginux-g12-high-fd-alias-probe.json/.log` records the exact source/fixture/binary hashes and `exited/99`. Descriptor canonicality is **failed** on this revision. This demonstrates a public-FD alias, not the separate old G03c contract-deny path. The first wrapper could not read the root-owned result file and is invalid evidence; the corrected root-run produced the receipt. The user assigned storage redesign upstream to Claude, so no local production fix or remote action was made. Retest on a corrected integration revision before acceptance; continue independent criteria meanwhile.

### Current guarded-source Recipe security selection — 2026-11-07

All 29 independent security e2e selections from the saved 33-case Recipe catalog passed individually on clean local Ginux integration source `b91b69c1` and immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`, with Recipe source `2ad833c` and harness SHA-256 `6454566708d223bb627e27b603d38d2df13100d5352285d52b36aec42f9d7ffa`. The exact case set matches the earlier catalog, without duplicates. Aggregate case time was 345.25 s. The ledger and per-case outputs are in `recipe-security-current-guarded-ledger.json` and `recipe-security-current-guarded-logs/` under the local Daybreak evidence directory; the first three outputs were retained as their last 3,000 characters, while the other 26 logs are complete.

Four security cases still invoke guest sleep and remain **unrun** under the user's G02 clock hold; one other ordinary G02 case and the observed ordinary sleep failure remain pending. These 29 passes are current-source Recipe evidence, not a 135-case full-suite pass or Ginux security acceptance. The confirmed high-bit descriptor alias remains **failed** on this exact Ginux binary and awaits Claude's storage correction. G11 stack/AT_EXECFN source review found its existing 4,095-argument `many` case already checks AT_EXECFN and stack restoration; no new boundary outcome is claimed. Exact next action: continue independent Ginux private-transfer send/close and authority checks, then retest descriptor canonicality on a corrected integration revision. Claude owns upstream work; no push, PR, comment or clock-policy change occurred. Goal active/incomplete.

### Private mapping send-stage EMFILE cleanup — 2026-11-07

A disposable `strace` fault injector returned `EMFILE` from the supervisor's first `pidfd_getfd` while the unchanged guarded Ginux binary (SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`) transferred an owned-file mapping. The guest received the expected `-ENOTSUP` mapping result. At explicit live handshakes before and after this failed Send stage, its native FD set was `[0,1,2,3]`; a later owned map/unmap passed after the one-shot injection. The supervisor exited 0 with result `exited/0` and `owned-threads: PASS`. The injection trace shows exactly one forced `pidfd_getfd(7,4,0) = -EMFILE` followed by a successful call.

Exact disposable source, compiled fixture, injector script, trace, audit and receipt with SHA-256 values are retained as `ginux-g12-send-fault-snapshot-*` in the local Daybreak evidence directory. An earlier simpler no-snapshot control also passed and is retained as `ginux-g12-send-fault-*`. This covers one supervisor endpoint-acquisition failure, not `sendmsg` itself or guest private-Close failure. Those later edges, high-bit descriptor canonicality (confirmed failed), G02 clock-held cases and whole-goal security acceptance remain pending. Exact next action: continue independent transfer/authority checks where a controlled trigger exists, then retest canonicality after Claude's corrected storage revision; no local upstream action occurred.

### Private mapping sendmsg EIO cleanup — 2026-11-07

A separate one-shot `strace` injection forced `sendmsg` to return `EIO` while the unchanged guarded supervisor sent an owned-file mapping backing reference. The guest received the expected `-ENOTSUP`; native FDs were `[0,1,2,3]` at live handshakes before and after the failed transfer; a subsequent ordinary owned map/unmap passed. The supervisor exited 0 with result `exited/0`. The trace shows one injected `sendmsg(... SCM_RIGHTS ..., MSG_NOSIGNAL) = -EIO` followed by a successful `sendmsg`. The exact source, derived fixture, script, trace, audit, hashes and receipt are retained as `ginux-g12-sendmsg-fault-snapshot-*` in the local Daybreak evidence directory. This tests the actual Send syscall failure and cleanup, separate from the prior endpoint-acquisition `EMFILE` probe.

Guest private-Close failure remains unverified: tracing the supervisor alone does not inject the already-ptraced guest's close syscall. Do not count that edge as passed. Descriptor canonicality remains confirmed failed; G02 sleep cases remain held; overall security acceptance is incomplete. Exact next action: continue independent current-source authority/rollback checks where deterministic triggers exist and retest canonicality after Claude's corrected storage revision. No source or remote action occurred.

### Current guarded-source checkpoint transport negatives — 2026-11-07

The unchanged full `scripts/test-launch-checkpoints.py` runner passed on guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d` in 0.721 s, covering snapshots, private-FD visibility, fragmented valid frames, replay, freshness, peer-closure cancellation and task events under inherited ignored SIGCHLD. A separate disposable control probe sent a `GXS1` header with pathname length 65535: launch returned exit 1/result `supervisor-error`, no guest continuation or final export, and `invalid checkpoint control request`. An inherited pipe as `--control-fd` was rejected at startup with exit 2/result `supervisor-error` and `control fd must be an inherited Unix stream socket`. Both controls passed their corrected expectations on the exact binary.

The first probe receipt expected runtime exit 1 for the pipe and marked that case failed; source/audit showed the earlier startup rejection was correct. That invalid assertion receipt is retained separately and is not a Ginux failure. Exact commands, runner/fixture/binary hashes, outputs and audits are in `ginux-daybreak-checkpoints-current.json`, `ginux-g20-control-negative.json`, companion audits and the initial receipt in the local Daybreak evidence directory. These checks do not cover every oversized/replayed combination, partial send/read/close/capture fault or shared-thread race. G03c/G12 descriptor canonicality remains failed, guest Close unverified and G02 clock held. Exact next action: continue independent restore/control fault and authority checks, with acceptance pending. No source or remote action occurred.

### Checkpoint reply-send failure after valid snapshot — 2026-11-07

A disposable coordinator sent the first five bytes of a valid checkpoint frame, disabled its socket receive side, then sent the rest. This forced the current guarded supervisor's reply-send failure after snapshot capture. Launch returned exit 1/result `supervisor-error`, reported `checkpoint control reply failed`, emitted no guest continuation after readiness, and did not publish final export. The requested checkpoint archive **did exist and was valid** with `work/checkpoint = first`; the failure is in acknowledgement, after complete snapshot publication. This is not a claim that the snapshot is rolled back. Exact command, script, binary/fixture hashes, audit and receipt are retained as `ginux-g20-reply-failure.*` in the local Daybreak evidence directory.

This narrows one deterministic control send failure. Partial read/close/capture faults, concurrent state races, G03c/G12 descriptor canonicality (failed), private mapping guest Close and G02 clock-held cases remain pending. Exact next action: continue independent restore/control fault and authority checks, and retest the descriptor failure when Claude's corrected storage source is integrated. No source or remote action occurred.

### Current guarded-source restore rejection controls — 2026-11-07

Five disposable startup restore inputs—truncated tar, corrupt header checksum, traversal member, duplicate member and symlinked archive pathname—were rejected on the unchanged guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Each launch returned exit 2/result `supervisor-error`, emitted no guest output, published no export, left the image bytes unchanged and created no traversal escape. Exact command, source/fixture/binary hashes, audit reasons and per-case receipts are in `ginux-g21-restore-negative.json` and companion files in the local Daybreak evidence directory.

These are five narrow current-source negatives. They do not rerun the earlier complete valid >2 GiB restore, nor cover hostile concurrent archive mutation, FD/read/close faults or every G21/G22 authority edge. G03c/G12 descriptor canonicality remains failed, guest Close unverified and G02 clock held. Exact next action: continue independent current-source restore and authority checks where deterministic triggers exist; retest the descriptor failure after Claude's corrected storage revision is integrated. No source or remote action occurred.

### Current guarded-source restore read EIO — 2026-11-07

A fresh valid archive was produced by the existing storage fixture and restored successfully on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. With `strace -P` restricted to that exact archive pathname, one injected `read(..., 512) = -EIO` made the same restore fail at startup: exit 2/result `supervisor-error`, `restore archive read failed`, no guest output or final export, and unchanged image bytes. `ginux-g21-read-fault.json`, its trace/audit and script in the local Daybreak evidence directory record exact commands and hashes. The first probe attempt stopped on an incorrect positive fixture-byte assertion before any injected run; that initial script is retained and is not Ginux-failure evidence.

This verifies one archive read-error path. `storage_restore.rs::Input` drops the read-only archive FD without checking close's result; close-failure behavior, hostile concurrent mutation, and other G21/G22 failure surfaces remain unverified. Descriptor canonicality remains failed pending Claude's storage correction, and G02 clock remains held. Exact next action: inspect source-specific archive close and concurrent-mutation behavior with controlled probes where feasible, then continue the remaining authority criteria. No source or remote action occurred.

### After-open archive mutation controls — 2026-11-07

Two disposable probes exported a valid 7,168-byte archive, verified an ordinary restore, then used `strace -P` to delay the current guarded supervisor's first read of that archive. Both traces show `openat(O_RDONLY|O_NONBLOCK|O_NOFOLLOW|O_CLOEXEC)` and `fstat(... st_size=7168)` completing before the host-side mutation. Truncating the same inode to zero made the delayed read return EOF; launch returned exit 2/result `supervisor-error` with `truncated restore archive`. Changing one header byte without changing size made the delayed read return the altered header; launch returned exit 2/result `supervisor-error` with `invalid restore header`. Neither emitted guest output or final export, and the image bytes remained unchanged. Exact commands, binary/fixture/script hashes, traces, audits and receipts are retained as `ginux-g21-concurrent-{truncate,corrupt}.*` in the local Daybreak evidence directory. The first corruption receipt used an inaccurate field label and is retained; the corrected rerun passed the same assertions.

These are two current-source after-fstat mutation timings, not proof of immutable archive contents or all concurrent replacement/race behavior. Archive close faults, metadata/resource failures, G03c/G12 descriptor canonicality (failed), private mapping guest Close and G02 clock cases remain pending. Exact next action: inspect controlled archive close/resource faults and other independent authority criteria, then retest canonicality after Claude's corrected storage source is integrated. No source or remote action occurred.

### Restore archive close-error observation — 2026-11-07

A path-filtered disposable `strace` run forced the current guarded supervisor's close of a validated restore archive FD to return `EIO`. The trace shows `close(5<snapshot.tar>) = -EIO (INJECTED)`, while launch still exited 0/result `exited/0`, printed `restored: PASS`, and published final export. The same archive passed export and ordinary restore controls. This matches `storage_restore.rs::Input::drop`, which calls `close` without checking its result. Exact source/binary/fixture hashes, commands, trace, audit and receipt are retained as `ginux-g21-close-fault.*` in the local Daybreak evidence directory.

`strace` skips the injected syscall, so this does not establish how a real Linux close error affects FD lifetime or that guest authority was exposed. It establishes that the supervisor does not treat a returned close error as a restore failure; whether that must change in Claude's storage redesign remains unresolved. Do not count archive-close failure handling as passed. After-open truncation, same-size invalid header, and archive read `EIO` controls passed separately. Descriptor canonicality remains failed and G02 clock remains held. Exact next action: review the remaining independent restore/export failure and authority surfaces; retest affected behavior after Claude's corrected storage source is integrated. No source or remote action occurred.

### Export staging fsync EIO cleanup — 2026-11-07

A valid ordinary export produced a 7,168-byte archive on the unchanged guarded binary. A controlled one-shot `strace` injection then made the export staging file's `fsync` return `EIO`: the injected trace line names `.ginux-export-<pid>-1`. Launch returned exit 1/result `supervisor-error`, reported `export-file sync failed`, left no staging file or final archive, and preserved the image. Exact script, binary/fixture hashes, command, trace, audit and receipt are retained as `ginux-g19-export-fsync-fault.*` in the local Daybreak evidence directory. Three earlier fault attempts hit guest memfd or result-file `fsync` calls instead; their receipts are retained and do not count as export-staging evidence.

This verifies one G19 staging sync failure and cleanup. Export write/close errors, hostile parent/final-name changes, interruption and other G19 authority/race checks remain pending. G21 archive close behavior is separately recorded as an ignored synthetic error; G03c/G12 descriptor canonicality remains failed and G02 clock remains held. Exact next action: use the export-wait handshake to determine whether a path-specific staging close error can be injected, then continue independent export/restore authority checks. No source or remote action occurred.

### G19 export publication integrity FAILED on current integration source — 2026-11-07

Two disposable host-side races at the `export-wait` handshake exposed a current-source publication failure on guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. In the first, the host renamed the export parent after Ginux created its staging file, replaced that parent with a symlink to another disposable directory, and put a marker file under the expected staging basename there. In the second, the host left the parent alone but renamed the staging entry and put the marker under the original basename. In **3/3 runs of each variant**, Ginux returned exit 0/result `exited/0` while the requested final name contained the 40-byte host marker. The actual 7,168-byte archive Ginux wrote remained at its renamed staging path; it was independently parsed and its `out/product` data verified. These are successful-workload controls with a **wrong published export**, not a guest escape or a mere cleanup warning. Exact scripts, binary/fixture/source hashes, audits and six repeat receipts are under `ginux-g19-{parent-swap,stage-replacement}.*` in the local Daybreak evidence directory.

G19 publication-integrity security acceptance is **failed** on this source. A separate after-staging competing *final name* was correctly preserved with `supervisor-error`; path-filtered staging write, sync and close `EIO` controls also rejected and cleaned up, so those passes do not resolve the staging-identity race. Source review identifies the relationship: `storage_export.rs` writes through an opened FD, closes it, then publishes by resolving the stored staging and destination path strings again; a concurrently changed staging name can refer to different bytes at rename. A correction must bind publication to the completed archive's identity and account for concurrent parent/name changes. A pathname-only precheck would leave a race. No local source fix, upstream packet, PR, push or comment was made; Claude owns upstream extraction. Keep the local integration goal active, retest both failing probes on a corrected integration revision, and rerun affected export/checkpoint and Recipe consumers before acceptance. G03c/G12 descriptor canonicality also remains failed; G02 clock stays held. Exact next action: evaluate a narrow correction in the existing export owner against this reproducer while continuing independent Daybreak criteria; do not count G19 as passed.


### Local G19 publication correction and current-source verification — 2026-11-07

Local Ginux integration commit `e354efd58500b08e45b3502218fbcfc36b9bc8a7` changes only the existing export owner and its storage integration runner. It writes the archive to an unnamed `O_TMPFILE` in a pinned parent directory, syncs and closes the writer, then uses an `O_PATH` reference and `linkat(AT_EMPTY_PATH)` to publish that exact inode without replacing a final name. It rejects a parent pathname that no longer identifies the pinned directory. This is local extraction material for Claude, not an upstream PR or storage redesign. Filesystems without `O_TMPFILE` support now fail export explicitly before launch; no fallback has been implemented.

On exact rebuilt binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, the prior parent-swap timing returned exit 1/`supervisor-error`, created no final archive in either directory, and preserved the host marker; the prior staging-name timing found no named staging file, preserved a fake host marker and published a valid archive with `out/product = result\0data`. A competing final name still returned `supervisor-error` and preserved its caller bytes. Receipts and exact scripts are `ginux-g19-{parent-swap-fixed,stage-replacement-fixed,final-name-race-fixed}.*` and `ginux-g19-publication-retest.py` in the ignored local Daybreak evidence directory. The first two fixed receipts use the same hash-recorded disposable script; the final-name script is retained separately.

The full configured-storage suite, checkpoint suite, targeted `ginux-init` clippy with warnings denied, and `cargo fmt --check` passed. Three relevant Recipe e2e selections passed on this binary and the prepared image: managed finalization, modified-snapshot rejection and managed publication/archive redirects. This verifies the reproduced timing and selected consumers, not arbitrary concurrent host mutations, all export I/O faults on the new unnamed-file path, or the full Recipe suite. An adversary can still move a host pathname after the final identity check; this check cannot promise permanent path stability against a host actor. G19 security acceptance stays **partial** pending remaining fault/concurrency review. G03c/G12 descriptor canonicality remains **failed** pending Claude's storage correction, G21 close-error behavior and guest private-Close remain unverified, and G02 clock cases stay held by the user. Exact next action: inspect independent G19 unnamed-file fault paths and remaining G21/authority criteria; retest canonicality on corrected local integration source. No push, PR, comment, or Daybreak dispatch occurred.


### Unnamed-export write, sync and writer-close fault controls — 2026-11-07

On local integration commit `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, three separate one-shot `strace` injections returned `EIO` from the unnamed archive inode's first `write`, its `fsync`, and its writer `close`. Each launch returned exit 1/`supervisor-error` with the matching `export-file ... failed` audit, created no final name or named staging file, and preserved the image. An ordinary export control produced a valid 7,168-byte archive before the `fsync` injection. Exact traces, audits, receipts and scripts are retained as `ginux-g19-export-{write,fsync,close}-fault-fixed.*` under the ignored local Daybreak evidence directory. The initial write attempt used a proc-fd path filter that matched no syscall; the later unfiltered trace identified the first archive write as the fourth supervisor write after the readiness handshake. That initial attempt is not a Ginux failure or a passing injection. A separate close trace identified the archive writer as the second supervisor close after readiness. The injected close syscall was skipped by `strace`, so its result verifies Ginux's error handling, not real kernel FD-lifetime behavior.

These controls cover three faults on the new unnamed-file path, alongside the fixed parent/staging/final-name timing results. Abrupt interruption, resource exhaustion, parent mutation during the small publication interval, other archive/restore concurrency, and full G19 security acceptance remain pending. G03c/G12 descriptor canonicality still fails on current source pending Claude's correction; G02 remains held. Exact next action: continue independent interruption/resource/authority checks, then retest affected surfaces when Claude's corrected storage source is integrated. No upstream action or Daybreak dispatch occurred.


### Paused export supervisor interruption — 2026-11-07

A disposable `export-wait` launch on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` was killed with `SIGKILL` after the guest readiness handshake and before export completion. The supervisor exited `-9`; no final archive or named staging file existed before or after the kill; its traced child was absent 100 ms later; and the image bytes remained unchanged. `ginux-g19-interrupt-fixed.py/.json/.audit` in the local Daybreak evidence directory retain the exact script, hashes and result. The result-file pathname existed as an empty file at the time of the kill; the first probe incorrectly expected it to be absent and failed that assertion. The corrected probe records its empty contents and passes the archive/no-stage criterion. This is one pre-publication interruption timing, not a mid-write or post-link power-loss guarantee. Exact next action: continue G19 resource and narrow publication-window checks and the independent G21/authority criteria; G03c/G12 descriptor canonicality remains failed pending Claude's corrected source, and G02 clock stays held. No upstream action or Daybreak dispatch occurred.


### Unnamed-export setup EMFILE — 2026-11-07

A fresh valid export control on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` produced a 7,168-byte archive. A separate one-shot `strace` injection returned `EMFILE` from the third supervisor `openat`, with the trace confirming that call was the `O_TMPFILE` creation in the pinned export parent. Launch failed during setup with exit 2/`supervisor-error`, reported `export-file unnamed staging unavailable`, emitted no guest output, created no final archive or named staging file, and preserved the image. Exact script, trace, audit and receipt are `ginux-g19-export-openat-fault-fixed.*` in the ignored local Daybreak evidence directory. This covers the unnamed-stage creation failure, not every parent/reference FD reservation or full disk/quota failure. Exact next action: continue independent G19 publication-window and G21 authority checks; retest G03c/G12 canonicality when Claude's corrected storage source arrives. G02 clock remains held and overall Daybreak acceptance is incomplete. No upstream action or Daybreak dispatch occurred.


### G19 post-link parent mutation timing — 2026-11-07

A disposable `export-wait` probe on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` attached `strace` to delay the first `linkat` return by one second. The requested final archive became visible while the supervisor was still stopped at that syscall. The host then renamed the pinned export parent and placed a symlink plus marker at its old pathname. The trace shows `linkat(... AT_EMPTY_PATH) = 0 (DELAYED)`; Ginux returned exit 1/`supervisor-error` with `export-file publication failed`. The renamed parent retained the **complete, valid 7,168-byte archive** with `out/product = result\0data`; the original requested pathname resolved to the independently host-written marker. The image was unchanged. Exact script, trace, audit, hashes and receipt are `ginux-g19-postlink-parent-swap.*` under the ignored local Daybreak evidence directory.

This confirms that the final identity check detects the changed host parent after linking, but it cannot erase an archive already linked in the pinned directory or guarantee the requested pathname remains stable against later host mutation. The published archive is complete, so this does not show partial-output publication or wrong bytes linked by Ginux. Treat post-link path stability and failure-result publication semantics as an explicit G19 limit for Claude/upstream review; do not count broad hostile-host concurrency acceptance as passed. Exact next action: continue independent G21 restore/authority checks and retest G03c/G12 canonicality on Claude's corrected source. G02 clock remains held; no push, PR, comment or agent dispatch occurred.


### G21 after-open archive pathname replacement — 2026-11-07

On local Ginux integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable valid 7,168-byte archive passed export control. A `strace -P` delayed the first archive read after `openat(O_NOFOLLOW|O_NONBLOCK)` and `fstat(st_size=7168)`. During that delay, the host renamed the archive and placed invalid marker bytes under its old pathname. Ginux restored from the already-open original inode: launch returned exit 0/`exited/0`, the fixture printed `restored: PASS`, its expected post-restore product was `changed`, the parked original archive and pathname replacement were byte-for-byte unchanged, and the image was unchanged. Exact script, trace, audit, hashes and receipt are `ginux-g21-open-archive-replacement.*` under the ignored local Daybreak evidence directory. The first probe expected the final exported product to retain the pre-restore bytes, but the fixture intentionally writes `changed`; that initial assertion and receipt are retained as `*-initial.*` and do not count as a Ginux failure.

This verifies one after-open **pathname** replacement timing, separate from earlier tests that mutated the opened inode's bytes. It does not prove immutability if a host actor writes to that inode, nor full G21 hostile-concurrency/close-error acceptance. G03c/G12 descriptor canonicality remains failed pending Claude's corrected source, G02 clock remains held, and the Daybreak goal remains open. Exact next action: continue independent restore resource/authority and control transport fault checks; retest source-dependent criteria after Claude's storage redesign. No push, PR, comment or agent dispatch occurred.


### G21 restore archive fstat EIO — 2026-11-07

On local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a fresh valid archive was produced and restored successfully as controls. A path-filtered one-shot `strace` then made `fstat(7<snapshot.tar>)` return `EIO` after Ginux opened the same archive. Launch rejected it at startup with exit 2/`supervisor-error` and `restore archive is not a bounded regular file`, emitted no guest output or final export, and preserved the image. Exact commands, source/fixture/binary hashes, trace, audit and receipt are `ginux-g21-fstat-fault.*` under the ignored local Daybreak evidence directory. This covers one archive metadata-error path, not real close-error FD lifetime, all resource exhaustion or hostile concurrent mutation. G03c/G12 descriptor canonicality still fails pending Claude's corrected source; G02 clock remains held. Exact next action: continue independent G20/G21 transport and restore authority failures, then retest source-dependent criteria after Claude's storage redesign. No upstream action or Daybreak dispatch occurred.


### G20 control-socket read EIO — 2026-11-07

A disposable `checkpoint` fixture on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` paused at `checkpoint-ready`. After its task events settled, a one-shot `strace` injection returned `EIO` from the supervisor's first read of the inherited `UNIX-STREAM` control FD. Launch returned exit 1/`supervisor-error`, reported `checkpoint control peer closed`, emitted no guest continuation, and created neither requested checkpoint nor final archive. The matching trace is `read(4<UNIX-STREAM:...>, ..., 14) = -1 EIO (INJECTED)`. Exact script, trace, audit, hashes and receipt are `ginux-g20-control-read-fault.*` in the ignored local Daybreak evidence directory.

The initial probe hit the intended socket and observed the same safe failure but failed its own assertion because it expected a `socket:` trace label instead of `UNIX-STREAM:`; its script/receipt are retained as `*-initial.*`. A second attempt's first read was the asynchronous task-event signalfd, so that result is retained separately as `*-signalfd-attempt.*` and is not control-socket evidence. The corrected probe waits for fixture task events before attaching and passes its exact-FD assertion. This covers one nonretryable control read failure, not fragmented-frame faults, close/capture errors or full G20 security acceptance. G03c/G12 descriptor canonicality remains failed pending Claude's corrected source; G02 clock remains held. Exact next action: continue independent control transport/restore authority checks; no upstream action or agent dispatch occurred.


### G20 control-socket close EIO after two checkpoints — 2026-11-07

A disposable two-checkpoint control on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` received both exact `GXA1` acknowledgments, verified each completed snapshot and exited 0 with a final archive. A traced replay located the inherited `UNIX-STREAM` socket as the first supervisor `close` after the second checkpoint's guest continuation. A one-shot `strace` injection returned `EIO` at that exact close. Ginux returned exit 1/`supervisor-error` with `checkpoint control close failed`, did not publish final export, and retained both already acknowledged valid checkpoints. Exact script, positive and injected traces/audits, hashes and receipts are `ginux-g20-control-close-{trace,fault}.*` under the ignored local Daybreak evidence directory, with shared script `ginux-g20-control-close-fault.py`.

`strace` skipped the injected kernel close. This proves Ginux reacts to a returned close error before final export; it does not establish actual kernel FD lifetime after a real close failure. G20 partial send/capture, concurrent state and full authority acceptance remain pending. G03c/G12 descriptor canonicality is still failed pending Claude's corrected source; G02 clock remains held. Exact next action: continue independent G20/G21 failure and authority checks, then retest source-dependent criteria after Claude's storage redesign. No upstream action or agent dispatch occurred.


### Current-source full G21/G22 restore runner — 2026-11-07

The unchanged `scripts/test-launch-restore.py` default runner passed on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with the retained storage fixture. Its exact root-required command exited 0 in 27.867 seconds and printed `test-launch-restore: PASS (roundtrip, startup isolation, parent ordering and malformed inputs)`. The runner source covers fresh roundtrips, parent ordering, links and aliases, 5,000 nonempty files, complete streamed >2 GiB payload, assignment denial, truncated/bad-checksum/traversal/symlink/hardlink/duplicate inputs, forward/absolute/traversing/nonregular/conflicting hardlinks and symlinked archive pathname. Binary, fixture, runner and log hashes, command, output and duration are in `ginux-restore-full-current-export-fixed.json/.log` under the ignored local Daybreak evidence directory.

This current-source suite is stronger than the earlier older-revision pass, but its assertions do not cover all hostile concurrent archive mutation, close-error FD lifetime, every arithmetic/resource fault or G22 authority combination. The G21 after-open path replacement and `fstat EIO` controls are separate evidence. G03c/G12 descriptor canonicality remains failed pending Claude's corrected source; G02 clock remains held; Daybreak acceptance remains incomplete. Exact next action: continue independent G20/G21/G22 concurrency and authority edges with source-specific probes, then retest changed storage behavior after Claude's integration. No upstream action or agent dispatch occurred.


### G21 statically oversized sparse archive — 2026-11-07

On local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, valid export and restore controls passed. A separate sparse archive with apparent size 8,589,934,592 bytes and zero allocated data blocks was then offered under the ordinary 256 MiB configured memory bound. The trace shows `openat` and `fstat(st_size=8589934592)` on that exact archive and no archive `read`. Launch returned exit 2/`supervisor-error` with `restore archive is not a bounded regular file`, emitted no guest output or final export, and preserved the image. Exact script, trace, audit, hashes and receipt are `ginux-g21-oversized-sparse-archive.*` in the ignored local Daybreak evidence directory.

This verifies one statically oversized source rejection before payload reads. It does not cover a host growing an already-open archive across the bound, all arithmetic overflows, or complete hostile mutation handling. The full current-source G21/G22 runner passed separately. G03c/G12 descriptor canonicality remains failed pending Claude's corrected source; G02 clock remains held. Exact next action: continue independent G20/G21/G22 authority and concurrent-mutation checks, then retest changed storage behavior after Claude's integration. No upstream action or agent dispatch occurred.


### G22 symlink mode authority pair — 2026-11-07

A matched pair of 1,536-byte USTAR inputs on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` differed only in the mode of `out/mode-link -> target`. Mode `0777` restored successfully, then exported the same symlink target and mode. Mode `0600` was rejected at startup with exit 2/`supervisor-error` and `invalid restore metadata`, emitted no guest output or final export, and preserved the image. Exact commands, source/fixture/binary hashes, archive hashes, audits and receipt are `ginux-g22-symlink-mode.*` in the ignored local Daybreak evidence directory. This verifies one unsupported symlink-mode rejection identified in G22 source review, not all malformed link metadata or assignment interactions. The full current-source restore runner passed separately. G03c/G12 descriptor canonicality remains failed pending Claude's correction; G02 clock remains held. Exact next action: continue independent G20/G21/G22 authority and concurrency checks, then retest changed storage behavior after Claude's integration. No upstream action or agent dispatch occurred.


### Current-source full task and lock runner — 2026-11-07

The unchanged `scripts/test-launch-tasks.py` default runner passed on local Ginux `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with the retained task fixture. Its exact root-required command exited 0 in 0.373 seconds. Output includes two operator CPU/resource cases; G23 `Locks`, `OrdinaryLocks` and `ExecLocks`; `qfault`, protected/junk/cross-page child-TID vectors; process creation, inheritance, orphan/adopted-zombie, rollback and two full descendant-teardown cases. Binary/fixture/runner/log hashes, command and complete output are in `ginux-tasks-full-current-export-fixed.json/.log` under the ignored local Daybreak evidence directory.

This is current-source evidence for the runner's actual assertions. It does not cover every protected/mutable lock struct, close/reuse race, thread-group lock ownership, resource-query copyout or hostile exhaustion listed in G23/G26 and the G06b handoff. G03c/G12 descriptor canonicality remains failed pending Claude's corrected source; G02 clock remains held. Exact next action: inspect G23 lock argument handling and run focused protected/mutable authority probes, then continue later independent handoff criteria. No upstream action or agent dispatch occurred.


### G23 lock pointer and failed-set rollback probe — 2026-11-07

A disposable task fixture on local integration `e354efd5` and exact Ginux binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` passed with exit 0/`exited/0` and `g23-pointer: PASS`. `F_SETLK` and `F_GETLK` returned `EFAULT` for an unreadable 32-byte struct and for a struct crossing into a protected page. `F_GETLK` returned `EFAULT` when its complete input struct was readable but its output page was read-only; the 32 input bytes were unchanged. A forked process then queried the same inode and saw no lock, confirming that the failed sets had not acquired one. A valid `F_SETLK` from the read-only input succeeded and was explicitly unlocked. Exact generated C fixture, compiled fixture, probe script (SHA-256 `5f9880f55c8958978499ed2860f38a3b09068e065dcecb7aad98047d0dd4eee3`), receipt with hashes, and audit are retained as `ginux-g23-pointer-*` in the ignored local Daybreak evidence directory.

This covers protected input/output, one cross-page input, and the observed failed-set state. It does not cover mutable shared structs, cross-page partial copyout, close/reuse races, table exhaustion rollback, thread-group ownership, or all interruption/deadlock cases. G23 acceptance remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: test G23 bounded-table exhaustion and rollback on this source, then continue independent lock authority/lifetime criteria. No source commit or upstream action occurred.


### G23 bounded lock table exhaustion rollback — 2026-11-07

A disposable fixture on local Ginux integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` acquired 64 separate write-lock ranges on one inode. A 65th insert returned `ENOLCK` (`-37`). Unlocking the middle byte of a three-byte range, which would split one entry into two while the table was full, also returned `ENOLCK`. A forked process then observed both the original three-byte range and the last separate range still owned by process 1, while the rejected 65th range was unlocked. Removing the whole three-byte range freed one entry; the formerly rejected insert succeeded, and a second child observed the first range unlocked and new range locked. Launch exited 0/`exited/0` with `g23-table: PASS`. The exact generated C fixture, binary, probe script SHA-256 `12eb08a7c1630d66f80b45798f0700fe30edd2e834211e384250dffffdc1a36a`, receipt with source/binary hashes, and audit are `ginux-g23-table-*` in the ignored local Daybreak evidence directory.

This verifies one deterministic table-capacity boundary and atomic rollback for a split operation. It does not cover competing processes filling the table concurrently, hostile memory mutation, FD close/reuse races, waiter cancellation, or thread-group lock ownership. G23 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe G23 descriptor close/reuse lifetime with synchronized contenders, then continue independent criteria. No source commit or upstream action occurred.


### G23 descriptor close and slot reuse lifetime — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` acquired a process record lock and an open-description `flock` on `/work/a`, duplicated FD 3 to FD 4, then closed FD 3 and reused that numeric slot for `/work/b` with its own two locks. A forked observer using independent opens saw `/work/a`'s record lock released, its `flock` retained through FD 4, and both `/work/b` locks present. After the parent closed the last `/work/a` alias, a second independent observer acquired `/work/a`'s `flock` while `/work/b` remained locked. Launch exited 0/`exited/0` with `g23-close: PASS`. The exact generated C fixture, binary, probe script SHA-256 `eaa3982ce55c5406d4dffae959c0311e07e2d31d0e18b6603c1f111a338f4a62`, receipt with source/binary hashes, and audit are `ginux-g23-close-*` in the ignored local Daybreak evidence directory.

This is a synchronized sequential close/reuse and two-observer check. It does not cover a close racing a parked waiter, `dup2` replacement, owner death during a blocked operation, thread-group ownership, or the known G03c/G12 high-bit descriptor alias, which remains failed pending Claude's correction. G23 acceptance remains partial and G02 clock policy remains held. Exact next action: examine G23 parked waiter/death races and independent G24 authority criteria; retest canonicality on corrected integration source. No source commit or upstream action occurred.


### G23 parked flock waiter and reused descriptor slot — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` held an exclusive `flock` on `/work/a`. Its child closed the inherited alias, independently opened `/work/a`, emitted `wait-ready`, and attempted a blocking exclusive `flock`. The host runner observed an exact `syscall=73 virtual-pid=2 action=park` audit line before releasing the parent. The parent closed its `/work/a` FD, immediately reused numeric FD 3 for `/work/b`, and locked `/work/b`. The child acquired `/work/a`, found `/work/b` still conflicting via an independent open, and exited; the parent joined it and launch exited 0/`exited/0` with `g23-wait: PASS`. Exact generated C fixture, binary, probe script SHA-256 `834eb35d5731334defc46b0afc66aa465d8667a7c160cd9ffc4bd421949778f4`, receipt, and audit are `ginux-g23-wait-reuse-*` in the ignored local Daybreak evidence directory. The first successful run used a broad audit condition; the retained final script and receipt are from a second passing run with the exact-line assertion.

This covers one parked-waiter wakeup across holder close and same-number descriptor reuse. It does not cover a waiter killed concurrently with close, `dup2` replacement, cyclic deadlock, mutable lock structs, or thread-group ownership. G23 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: inspect independent G24 timestamp authority and pointer criteria, then continue remaining G23 cross-thread/death cases when the current source can support them. No source commit or upstream action occurred.


### G24 timestamp pointer and invalid-field rollback — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` set a four-field explicit timestamp baseline on an owned inode. Descriptor `utimensat`/futimens-style calls returned `EFAULT` for an unreadable 32-byte struct and for a struct crossing into a protected page. An out-of-range nanosecond value and unsupported flags returned `EINVAL`, including no-follow on the descriptor/null-path form. `fstat` showed all four baseline fields unchanged after those rejected calls. A complete valid struct on a read-only page then succeeded and updated all fields. Launch exited 0/`exited/0` with `g24-pointer: PASS`. Exact generated C fixture, binary, probe script SHA-256 `e9bc5d5868d4f13f7bb16c027e05a762522c455fc409121782c5a04291768739`, receipt with source/binary hashes, and audit are `ginux-g24-pointer-*` in the ignored local Daybreak evidence directory.

This covers selected protected input and invalid-field rollback, not mutable shared buffers, path no-follow/assignment bypass, dangling or reused identity, negative/unrepresentable archive times, alias metadata conflict, or realtime clock fault/boundary behavior. G24 acceptance remains partial. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction; the user's G02 clock policy stays held. Exact next action: probe G24 path/assignment authority and unlinked descriptor identity, then continue other independent handoff criteria. No source commit or upstream action occurred.


### G24 no-follow, unlinked inode and reused-slot authority — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` created `/work/link -> /input`, where `/work` was owned memory storage and `/input` was an image marker. `utimensat` with no-follow updated the owned symlink's four timestamp fields, while a direct image `/input` timestamp update was rejected. The image marker's bytes and nanosecond mtime were unchanged before/after launch. The fixture then opened `/work/reused`, unlinked it, created a new file at the same pathname, and updated the old inode through its retained FD; the new inode's four fields stayed zero. After closing the old FD, a third file reused numeric FD 3 and received its own timestamp update without changing the new `/work/reused` inode. Launch exited 0/`exited/0` with `g24-authority: PASS`. Exact generated C fixture, binary, probe script SHA-256 `8a8b35b80574e1816aa0920bd5889ed742c2af64c866707639df1b1ab09c59b7`, receipt with source/binary hashes and marker before/after values, and audit are `ginux-g24-authority-*` in the ignored local Daybreak evidence directory.

This verifies selected no-follow and descriptor identity boundaries in a disposable image. It does not resolve the known symlink-above-assignment host oracle, hostile path races, negative/unrepresentable archive times, hardlink metadata conflict, or clock-boundary/failure behavior. G24 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: inspect G24 archive time bounds and alias conflict controls, then continue independent handoff criteria. No source commit or upstream action occurred.


### G24 archive timestamp representation boundaries — 2026-11-07

A disposable three-case fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` set explicit owned-file mtimes of 123, -1, and 17,179,869,184 seconds before final export. The 123-second control exited 0 and produced a valid tar member with mtime 123. The negative and oversized cases each exited 1/`supervisor-error`, with audits `export timestamp is negative` and `export metadata exceeds tar bound`; neither published an archive or retained a named staging file. All three preserved the disposable image marker. The exact generated C fixture, compiled fixture, final probe script SHA-256 `4cfa0bee156e6717a7f78ee9218cfdfb726c4742a3847371d520a7c9237e6c2b`, per-case audits and receipt are `ginux-g24-export-time-*` in the ignored local Daybreak evidence directory. The initial run observed the same safe Ginux behavior but failed the probe's expected oversized audit phrase; its script and receipt are retained as `*-initial.*` and are not counted as a passing test.

This covers two representability failures and one valid export, not every USTAR boundary or hostile mutation during export. The current-source restore runner separately covers hardlink mtime conflicts. G24 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G24 alias/clock boundary and remaining independent G25+ criteria; retest changed storage surfaces after Claude's correction. No source commit or upstream action occurred.


### G25 vector metadata and payload prefix faults — 2026-11-07

A disposable owned-file fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` passed with exit 0/`exited/0` and `g25-vector: PASS`. `readv` and `writev` returned `EFAULT` for iovec metadata in a protected page and for one 16-byte iovec crossing into a protected page. A two-vector aggregate greater than `isize::MAX` returned `EINVAL` without moving the file offset. A write whose first vector held `abc` and second vector had an invalid pointer returned exactly 3 and left `abc` as the complete file contents. A read whose first destination had two writable bytes and second destination was read-only returned exactly 2, copied `ab`, and moved the file offset to 2; a sole read-only destination returned `EFAULT` and left that offset at 2. Exact generated C fixture, compiled fixture, probe script SHA-256 `0d70e41ab6fcbe4d12cab3937cd2f0211cce5ed5aecbe3e2e7be33074834500b`, receipt with source/binary hashes, and audit are `ginux-g25-vector-*` in the ignored local Daybreak evidence directory.

This covers selected metadata/payload faults and short-copy prefix accounting, not mutable vectors, shared-buffer races, cancellation across blocked retries, all chunk boundaries, append/close ordering or the held native scalar clamp. G25 remains partial. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: inspect G25 regular-write chunk boundary/fault rollback and G26 query-copy authority, then continue independent criteria. No source commit or upstream action occurred.


### G25 regular-write second-chunk fault — 2026-11-07

A disposable owned-file fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` mapped 8 KiB, filled the first 4 KiB with `X`, and protected the second page. An 8 KiB `write` returned 4,096, set file size/offset to 4,096, and did not commit the faulting second chunk. An 8 KiB `pwrite` at offset zero likewise returned 4,096 while leaving the shared offset at 4,096. A later one-byte `write` advanced offset/size to 4,097; a first-page `EFAULT` then left the offset at 4,097, and readback at the boundary was `XXQ`. Launch exited 0/`exited/0` with `g25-chunk: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `138c156dfd77eca4ad045db47574d51b3e5b87909c6a768ddbf6cc16897cec2a`, receipt with source/binary hashes, and audit are `ginux-g25-chunk-*` in the ignored local Daybreak evidence directory.

This covers one second-chunk input fault and positioned offset restoration, not quota at another chunk, append races, concurrent close, shared-buffer mutation, or blocked pipe retry. G25 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: inspect G26 resource-query copyout/authority, then continue independent handoff criteria. No source commit or upstream action occurred.


### G26 affinity query copyout and AS mutation boundary — 2026-11-07

A disposable task fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` received an 8-byte valid affinity mask. A foreign virtual identity returned `ESRCH` and length 1,025 returned `EINVAL`. A protected or read-only output page returned `EFAULT`. With only the last four bytes of a page writable before a protected next page, the query returned `EFAULT` and those four bytes matched the valid affinity mask: **partial copyout occurred despite the error**. An attempted `prlimit64` address-space increase returned `EPERM`; a fresh query showed both limits unchanged. Launch exited 0/`exited/0` with `g26-query: PASS prefix-copied`. Exact generated C fixture, compiled fixture, final probe script SHA-256 `cd86bdfb4ae79c2451831d95866265d9379e1fde7e07f51fec3222f3df3c09dc`, receipt and audit are `ginux-g26-query-*` in the ignored local Daybreak evidence directory. The first probe left eight writable bytes, exactly the mask size, so copyout succeeded and its cross-page assertion failed; that invalid probe and receipt are retained as `*-initial.*` and are not counted as a Ginux failure.

The observed four-byte prefix is a concrete partial-copyout behavior for the G26 contract review, not evidence of atomic output. This does not cover maliciously mutable affinity buffers, host CPU hotplug, task stop/death/exec races, hostile resource exhaustion, or G07 thread-group identity. G26 acceptance remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G27 uname output authority and G26 process identity edge cases, then continue independent criteria. No source commit or upstream action occurred.


### G27 uname strict-output and partial prefix faults — 2026-11-07

A disposable task fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` received the full 390-byte virtual `uname` result in an unaligned buffer, confirmed selected Linux/instance/machine fields, and preserved sentinel bytes immediately before and after it. A protected or read-only output page returned `EFAULT`; the read-only page's sentinel stayed unchanged. A 390-byte output crossing a protected page after 200 writable bytes returned `EFAULT` while its first 200 bytes matched the valid result and the preceding sentinel stayed unchanged: **partial copyout occurred despite the error**. An address that overflowed when adding 390 bytes returned `EFAULT`. Launch exited 0/`exited/0` with `g27-uname: PASS prefix-copied`. Exact generated C fixture, compiled fixture, probe script SHA-256 `787ab604768caa60a37a0ce04fcc649f884c39df6e7ce35441066b36aa3a4b73`, receipt with source/binary hashes, and audit are `ginux-g27-uname-*` in the ignored local Daybreak evidence directory.

This checks selected output protections and arithmetic, not process_vm unavailability/failure injection, signal/thread races, all identity-field validation, or #179 identity-policy disposition. G27 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: test G27 strict-copy failure injection if a controlled syscall target exists, then inspect independent G28+ handoff criteria. No source commit or upstream action occurred.


### G27 strict uname process_vm failure injection — 2026-11-07

A positive `strace` replay of the G27 uname probe on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` located the first supervisor `process_vm_writev` as the valid 390-byte uname copy. A separate disposable fixture injected one `ENOSYS` at that exact syscall (`when=1`). The trace shows `process_vm_writev(... iov_len=390) = -1 ENOSYS (INJECTED)` and no subsequent copy call; Ginux audit records `syscall=63 ... action=emulate result=-38`, the guest output sentinel stayed unchanged, and launch exited 0/`exited/0` with `g27-injected: PASS`. No `ptrace-fallback` audit appeared. Exact trace-control and injected fixture sources, compiled fixtures, scripts (injected script SHA-256 `eb10fd092247fdaa8110c3917891629a3e83f3d701c753f904b3ab20acdc5573`), receipts, traces and audits are `ginux-g27-uname-trace-*` and `ginux-g27-uname-enosys-*` in the ignored local Daybreak evidence directory.

`strace` skipped the injected syscall. This verifies returned-`ENOSYS` handling and the absence of a fallback on this call, not real kernel unavailability or all process_vm errors. G27 remains partial due identity policy, mutable buffers and signal/thread races; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect independent G28 network and G29/G30 criteria against the current integration source, then continue bounded checks. No source commit or upstream action occurred.


### Current-source full configured-network runner — 2026-11-07

The unchanged `scripts/test-launch-network.py` default runner passed on local Ginux integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with the retained network fixture. Its exact root-required command exited 0 in 0.467 seconds and printed 11 per-mode passes plus `test-launch-network: PASS`: `io`, `nonblock`, `deny`, `quota`, `timeout`, `hold`, `hold-kill`, `slots`, `off`, `ordinary-hold`, and `budget`. The runner uses only disposable numeric loopback peers, checks the forbidden listener receives no connection, and asserts teardown/FD conditions for its selected cases. Command, binary/fixture/runner SHA-256 values and complete output are in `ginux-network-full-current.json/.log` under the ignored local Daybreak evidence directory.

This is current-source evidence for the runner's actual assertions, not full G28/G29 security acceptance. Malformed/foreign endpoints, concurrent revoke/connect/close, protected/mutable scalar/vector buffers, SO_ERROR races, stale alias authority and injected backend failures remain unverified. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G28/G29 source-specific endpoint and buffer fault surfaces, then continue independent G30/G08b criteria. No source commit or upstream action occurred.


### Current-source unfiltered G28 devices package tests — 2026-11-07

The unfiltered `cargo test -p ginux-devices` package command passed on local integration `e354efd5` with `RUSTFLAGS=''` and separate `CARGO_TARGET_DIR=/home/vince/levitate-development/ginux-recipe-integration/.artifacts/recipe-integration/daybreak-unit-target`. All 10 unit tests passed, including the previously unrun default-enabled `tcp_endpoint_slots_budget_and_revocation_use_real_connections` and `tcp_drop_closes_connecting_handles_and_invalid_policy_never_opens`; zero doctest cases ran. The command exited 0 in 0.606 seconds. Exact command/environment, source/lockfile hashes, full output and duration are `ginux-devices-full-current-host.json/.log` in the ignored local Daybreak evidence directory. An earlier `cargo test -p ginux-devices --lib` attempt without `RUSTFLAGS=''` failed before any tests because the repository's freestanding linker flags could not link host test libraries; its failure is retained as `ginux-devices-full-current.json/.log` and is not counted as a test failure or pass. The successful retry uses the same host-test RUSTFLAGS setting as `Makefile` `test-unit`.

These 10 package tests and the separate 11-mode configured-network runner provide current-source backend and selected guest evidence, not full G28/G29 security acceptance. Foreign/malformed endpoints, concurrent revoke/connect/close, protected/mutable buffers, SO_ERROR races, stale aliases and backend fault injection remain incomplete. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: inspect source-specific G29 endpoint/receive fault paths and G30 ownership authority, then continue independent criteria. No source commit or upstream action occurred.


### Current-source G30 full ownership fixture mode — 2026-11-07

The unchanged `scripts/test-launch-storage.py ... ownership` selector passed on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with the retained storage fixture. The exact root-required command exited 0 in 0.061 seconds and printed `PASS launch storage: ownership`. Its `ownership(full=1)` fixture covers owned aliases and hardlinks, contained symlink no-follow, directory and empty-path descriptor forms, retained unlinked FD, dangling link, missing path, invalid pointer, invalid flags, and native FD denial. Exact command, source/fixture/runner/binary hashes, output and duration are `ginux-g30-ownership-current.json/.log` in the ignored local Daybreak evidence directory.

This is current-source evidence for the fixture's actual assertions, not complete G30 security acceptance. Protected/mutable paths, UID/GID width cases, native host ownership mutation, concurrent rename/close/exec and shared-thread authority remain unverified. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction; G02 clock policy remains held. Exact next action: inspect G30 ownership path and descriptor authority under protected/cross-page inputs, then continue G29/G08b independent criteria. No source commit or upstream action occurred.


### Current-source G08b pipe/packet and FIONBIO runners — 2026-11-07

Two unchanged `scripts/test-launch-pipes.py` commands passed on local integration `e354efd5` and its exact binary/fixture hashes recorded in `ginux-g08b-pipes-current.json/.log` under the ignored local Daybreak evidence directory. The default command exited 0 in 0.213 seconds with `test-launch-pipes: PASS (data, atomicity, copyout, aliases, EOF, bounds, blocking, SIGPIPE)`. It includes the full `packet` fixture, invalid socketpair family/type/output pointer, oversized packet rejection, invalid payload pointer, truncation, alias/EOF and capacity checks, plus generic pipe protected/cross-page copyout and SIGKILL park controls. A separate `ordinary-fionbio ordinary-streams` selection exited 0 in 0.066 seconds and passed. Both commands, source/fixture/runner/binary hashes, full output and durations are retained in the receipt/log.

These are the runner's current-source assertions, not full G08b security acceptance. Protected/mutable packet payloads and FIONBIO integers, partial packet copyout and retained-packet semantics, stale/reused/public alias authority, concurrent fork/exec/close/cancel, shared-thread signal targeting and broader capacity faults remain unverified. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction; G02 clock policy remains held. Exact next action: inspect G08b source-specific packet receive/rollback fault behavior and G29 endpoint/receive authority, then retest source-dependent criteria after corrected storage integration. No source commit or upstream action occurred.


### G08b packet copy faults and FIONBIO rollback — 2026-11-07

A disposable configured packet-pair fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` passed with exit 0/`exited/0` and `g08b-packet: PASS`. A two-vector send with valid `AB` followed by an invalid pointer returned `EFAULT`; the receiving nonblocking endpoint still had no packet. After sending `ABCDE`, a two-vector receive with a valid two-byte first destination and invalid three-byte second destination returned `EFAULT` and wrote `AB` to the first buffer. A subsequent read returned the **complete `ABCDE` packet**, followed by `EAGAIN`: the failed receive did not consume it. FIONBIO inputs from a protected page or an integer crossing into a protected page returned `EFAULT` and preserved the endpoint's status flags. Exact generated C fixture, compiled fixture, probe script SHA-256 `710ab7dc61ef8490021326c50d56b18be586671f52ae80db4d1236c521d370c3`, receipt with source/binary hashes, and audit are `ginux-g08b-packet-fault-*` in the ignored local Daybreak evidence directory.

This covers one two-vector packet send/receive fault and two FIONBIO copy-in faults. The failed receive does have a written guest prefix despite `EFAULT`; retained packet state is verified. Mutable buffers, alias/close races, blocked retry, stale handles, broader capacity failure, SIGPIPE targeting and G07 shared-thread authority remain unverified. G08b remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 network receive/send copy faults and G30 ownership protected paths, then continue remaining handoff criteria. No source commit or upstream action occurred.


### G29 malformed and denied numeric endpoint probe — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` used one granted and one forbidden numeric loopback listener. On one configured TCP socket, `connect` rejected lengths 1 and 129 with `EINVAL`, an invalid guest address with `EFAULT`, and an unsupported family plus short IPv4 length with `EAFNOSUPPORT`. A well-formed numeric endpoint at the forbidden listener returned `EACCES`; that listener accepted no connection. A later connect on the same socket to the granted endpoint succeeded, and the granted listener accepted it. Launch exited 0/`exited/0` with `g29-endpoint: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `0f861f8cedde7a6ab5dfe07b22a1ada36d4e9143d35692944af17a08d8386611`, receipt with source/binary hashes and listener ports, and audit are `ginux-g29-endpoint-*` in the ignored local Daybreak evidence directory.

This covers selected malformed lengths/family/address and one grant boundary, not IPv6 scopes/mapped representations, mutable endpoints, concurrent policy/close/revoke, protected scalar/vector data, SO_ERROR races or foreign network targets. G28/G29 acceptance remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 receive/send fault and retained-tail behavior, then continue G30 protected ownership paths and other handoff criteria. No source commit or upstream action occurred.


### G29 staged receive tail across alias and exhausted budget — 2026-11-07

A disposable numeric loopback peer sent exactly `ABCDEFGH` on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with configured transport budget eight bytes. A two-vector guest `readv` with a two-byte writable first destination and invalid six-byte second destination returned 2 and copied `AB`. The guest duplicated the socket FD, closed the original, then read `CDEFGH` through the alias despite the backend budget being exhausted by the first eight-byte receive. Its next read returned `EDQUOT` (`-122`). The peer accepted once, sent eight bytes and exited without error. Launch exited 0/`exited/0` with `g29-tail: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `4a5d24a951c9d798a225bff34f704a802d0ca0ffdc7e3bd61347803ae211a15a`, receipt with source/binary hashes and peer observations, and audit are `ginux-g29-receive-tail-*` in the ignored local Daybreak evidence directory.

This verifies one staged short copy and retained-tail alias path at the exact byte budget. It does not cover a completely failed guest copy, protected/mutable send buffers, simultaneous readers, SO_ERROR/refusal, timeout/revoke races, or injected backend errors. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction and G02 clock policy remains held. Exact next action: test G29 complete receive-copy failure retention and G30 protected ownership paths, then continue remaining handoff criteria. No source commit or upstream action occurred.


### G29 complete receive copy failure retains staged bytes — 2026-11-07

A second disposable numeric loopback peer sent exactly `ABCDEFGH` under the eight-byte transport budget on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`. The guest's first eight-byte read used an invalid destination and returned `EFAULT`. After duplicating the socket and closing the original public FD, the alias read all eight original bytes `ABCDEFGH`; its next read returned `EDQUOT` (`-122`). The peer accepted once, sent eight bytes and exited without error. Launch exited 0/`exited/0` with `g29-copyfail: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `ada7e139e5e541930e2bff469a0225b3717cc3c866d64033aadbfb754f5aa4a3`, receipt with source/binary hashes and peer observations, and audit are `ginux-g29-receive-copyfail-*` in the ignored local Daybreak evidence directory.

Together with the short-copy tail probe, this verifies two selected staged-byte retention paths across public aliases and exhausted budget. Concurrent readers, mutable buffers, backend read errors, transport reset and broader SO_ERROR/refusal races remain unverified; G29 stays partial. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 protected ownership paths and G29 protected send buffers, then continue remaining independent criteria. No source commit or upstream action occurred.


### G30 protected ownership paths and UID width — 2026-11-07

A disposable fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` created an owned `/work/owner` file. `chown` and `lchown` returned `EFAULT` for a pathname in a protected page; `fchownat` returned `EFAULT` for a pathname crossing into a protected page before NUL. UID/GID values `1<<32` were accepted as zero after the x86-64 32-bit ID truncation; a value `1<<32|1` was rejected as unsupported, and `fstat` still projected UID/GID zero. A direct image `/input` chown to 1:1 failed; the disposable marker's bytes, host UID and host GID were unchanged before/after. Launch exited 0/`exited/0` with `g30-owner: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `09f127dc6954ea29c4076758e011dc117913ed5186a22d74d3e4508a2c4675ff`, receipt with source/binary hashes and marker before/after values, and audit are `ginux-g30-owner-pointer-*` in the ignored local Daybreak evidence directory.

This verifies selected pointer and ID-width behavior, not mutable path races, all dangling/empty-path forms, concurrent rename/close/exec, native/private descriptor authority or shared-thread ownership. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 protected send/copy errors and G30 descriptor/rename races, then continue remaining handoff criteria and retest corrected storage integration. No source commit or upstream action occurred.


### Upstream main read-only check — 2026-11-07

The local integration source remains clean at `e354efd5` and descends the earlier upstream `59e2f7aa6a03a1afe2b6579815654ad63fc6c1b6`. A read-only `git ls-remote git@github.com:gmkurtzer/ginux.git refs/heads/main` found current upstream main `53a54addea67969abaa240ea478def1b94130eee`. GitHub's compare reports it 15 commits ahead of `59e2f7aa` with no divergence, touching contract/instance configuration, launch and supervisor code among other paths. Read-only PR snapshots show wave-one #232, #233 and #234 all still open and unmerged. Per the user's ownership split, no rebase occurs until those merge; the current integration binary and Daybreak probes therefore do **not** include the latest upstream main. After the wave merges, rebase the local integration source onto the new upstream main, rebuild with the separate Cargo target directory, and rerun affected Recipe and security checks, especially the failed G03c/G12 descriptor-canonicality reproduction. No fetch into restricted branches, push, PR, comment or local source change occurred in this check.


### G29 protected send and vector-prefix byte accounting — 2026-11-07

A disposable numeric loopback peer on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` had a four-byte configured transport budget. A guest write from an invalid source returned `EFAULT` without spending it. A two-vector write with valid `AB` followed by an invalid three-byte pointer returned 2 and sent only the prefix. A later `CD` write returned 2; the next one-byte write returned `EDQUOT` (`-122`). The peer accepted once, received exactly `ABCD`, and exited without error. Launch exited 0/`exited/0` with `g29-send: PASS`. Exact generated C fixture, compiled fixture, probe script SHA-256 `a84b65df880f77fa9435bd36c973c69d987603defb32bf2f21d201a821af23f5`, receipt with source/binary hashes and peer observations, and audit are `ginux-g29-send-fault-*` in the ignored local Daybreak evidence directory.

This covers selected protected source and vector-prefix accounting under one byte budget. It does not cover mutable/shared buffers, backend short writes/errors, SIGPIPE targeting, concurrent alias close, retries or refused connections. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect remaining G29 backend/SO_ERROR and G30 concurrent descriptor authority, then retest changed storage after upstream wave-one merges. No source commit or upstream action occurred.


### G29 refused nonblocking connection and alias SO_ERROR — 2026-11-07

A disposable numeric loopback port was bound but never listened on. On local Ginux integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a configured nonblocking TCP socket returned `EINPROGRESS` from connect, and poll reported readiness. After duplicating the socket, `SO_ERROR` through the alias returned `ECONNREFUSED` once; a second query through the original returned zero. Reads and writes through the alias returned `ECONNREFUSED`, including a read after closing the original public descriptor. Launch exited 0/`exited/0` with `g29-refusal: PASS`. Exact generated fixture, probe script SHA-256 `9e94160859245c6d253be1438b8ea4cbfce173b3f53b9a851a67a0be3677c1fe`, compiled fixture, source/binary hashes, result and audit are `ginux-g29-refusal-*` in the ignored local Daybreak evidence directory.

This checks one deterministic refused IPv4 loopback endpoint and serial alias/error consumption. It does not cover concurrent `SO_ERROR` queries, connection reset after establishment, mutable endpoint data, backend fault injection or revoke/close races. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 concurrent descriptor/path authority, then continue independent G28/G29 race criteria. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 renamed directory descriptor and closed/reused slot — 2026-11-07

A disposable configured-storage fixture on local Ginux integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` opened `/work/old` as a directory descriptor, created `item` relative to it, then renamed the directory to `/work/new`. Relative `fchownat(dirfd, "item", 0, 0, 0)` succeeded; the old absolute path returned `ENOENT`, and the new absolute path succeeded. After closing the file descriptor, `fchown` on that closed number returned `EBADF`; opening a distinct file reused the number, and `fchown` on the new descriptor succeeded with virtual UID/GID zero on `fstat`. Launch exited 0/`exited/0` with `g30-rename: PASS`. Exact generated fixture, compiled fixture, script SHA-256 `8b386488ab82e3112cf1bc7d74c4aa78bbd7bb9a9208b54ae89b9270bdc83606`, source/binary hashes, result and audit are `ginux-g30-rename-*` in the ignored local Daybreak evidence directory.

This is a sequential rename and descriptor lifetime check, not a concurrent rename/close/exec race or a stale descriptor identity guarantee after numeric reuse. It does not cover host mutation during resolution, shared-thread authority, private descriptors or the known G03c/G12 high-bit descriptor alias, which remains failed pending Claude's correction. G30 remains partial and G02 clock policy remains held. Exact next action: inspect remaining G30 private/native descriptor and G29/G28 concurrent authority paths, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 IPv6 mapped-address and scope grant boundary — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` granted only a numeric IPv4 loopback listener. On an AF_INET6 socket, `connect` to `::ffff:127.0.0.1` at the granted port returned `EACCES`, and the same address with nonzero scope or flowinfo returned `EAFNOSUPPORT`. After closing that socket, a separate AF_INET socket connected to the granted IPv4 address. The listener accepted exactly one connection. Launch exited 0/`exited/0` with `g29-ipv6: PASS`. Exact generated fixture, compiled fixture, corrected script SHA-256 `7830256421a48181ba028477970f76f78e37aad22eb0b1ac9409686ae8b194c8`, source/binary hashes, result and audit are `ginux-g29-ipv6-*` in the ignored local Daybreak evidence directory. The first run's fixture wrote one byte too many from its PASS string, so its exact-output assertion failed despite exit 0; its script/receipt/audit are retained as `*-initial.*` and are not a Ginux failure or accepted pass.

This checks one mapped IPv6 representation under an IPv4-only grant and two unsupported sockaddr fields. It does not cover a separately granted IPv6 endpoint, link-local scope, concurrent address mutation, alternate mapped forms or foreign network targets. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Upstream main remains `53a54add` and #232/#233/#234 were still open at this read-only check; no rebase trigger has occurred. Exact next action: continue independent network/stale-handle and ownership authority probes while awaiting corrected storage integration. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 refused socket close and public-slot reuse — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` granted two numeric loopback endpoints, one bound without listening and one listening. The first nonblocking socket connected to the refused endpoint with `EINPROGRESS`; poll completed, and `SO_ERROR` returned `ECONNREFUSED`. After close, connect through that closed public descriptor returned `EBADF`. A fresh socket reused the same public number, had zero `SO_ERROR`, connected to the listening endpoint with `EINPROGRESS`, completed through poll and reported zero `SO_ERROR`. The listener accepted one connection. Launch exited 0/`exited/0` with `g29-reuse: PASS`. Exact generated fixture, compiled fixture, script SHA-256 `bf0362b6dde6dc1cbd66f20a1a505468527faf12a2ed5622960fedc48f9d8f8a`, source/binary hashes, result and audit are `ginux-g29-reuse-*` in the ignored local Daybreak evidence directory.

This verifies one serial failed-state reset and close/reuse path, not a concurrent close/connect race, stale alias after close, handle generation discipline under fork/exec or backend failure during close. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: continue independent G28/G29 race/failure and G30 authority checks, then retest source-dependent behavior after the upstream storage wave merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 supervisor host ownership syscall trace — 2026-11-07

The passing G30 renamed-directory/closed-slot fixture was rerun on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with `strace -e trace=chown,lchown,fchown,fchownat` on the supervisor. The guest again exited 0/`exited/0` with `g30-rename: PASS`; the supervisor trace contained no ownership syscalls, only a ptrace-related `SIGCHLD` and normal exit. Exact probe script SHA-256 `2a370aad43c147690d1314c407e3df67e6bfb7245a3fcd9988c0cc17099a27e8`, generated fixture, result, audit and trace are `ginux-g30-nohost-*` in the ignored local Daybreak evidence directory.

This verifies no host ownership call by the supervisor in this selected fixture. The trace does not attach to the guest and does not establish every native/private path or adversarial concurrency case. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect remaining native/private G30 authority and independent G28/G29 race/failure criteria, then retest corrected storage integration. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 native descriptor ownership denial — 2026-11-07

A disposable configured-storage diagnostic on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` called ownership operations on native console descriptors. `fchown(1,0,0)` and `fchown(0,0,0)` returned `EPERM` (`-1`). `fchownat(1,"",0,0,AT_EMPTY_PATH)`, the same call requesting IDs 1:1, and `fchownat(1,"",0,0,0)` all returned `ENOSYS` (`-38`); the audit explicitly records `syscall=260 action=deny` for those three calls. Launch exited 0/`exited/0` and emitted the exact return-value vector `[-1,-38,-38,-38,-1]`. The source-level hypothesis that native empty-path `fchownat` might report success was therefore disproved for this configured policy. Exact generated fixture, compiled fixture, script SHA-256 `cb76b6bca9907effb9be51b8ce5cb97e5dc462372a89b8e06e213`, source/binary hashes, result and audit are `ginux-g30-native-*` in the ignored local Daybreak evidence directory.

This confirms denial on selected native descriptors, not all private mapping descriptors or native path forms. `ENOSYS` and `EPERM` differ, but neither reports success or invokes host ownership mutation in this probe. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 private backing descriptor exposure and independent G28/G29 failure/race paths, then retest corrected storage integration. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 established TCP reset and SO_ERROR copy faults — 2026-11-07

Two disposable configured-network runs on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` used a numeric loopback peer. The guest completed a nonblocking connection, sent `X`, and parked until the host peer read that byte and closed with `SO_LINGER(1,0)` to cause a TCP reset. In the first run, poll returned one ready descriptor with revents `29` (`POLLIN|POLLOUT|POLLERR|POLLHUP`); the first `SO_ERROR` returned `ECONNRESET` (`104`), the second returned zero, and a subsequent read returned EOF. In the second run, invalid guest output and optlen pointers made two `SO_ERROR` queries return `EFAULT` (`-14`); a later valid query still returned `ECONNRESET` once, followed by zero and EOF. Both launches exited 0/`exited/0`; the peer received exactly `X`. Exact scripts SHA-256 `6f89bb2fc269fbdd8aff8c871403770cecf548b27b09e82e81c7150aef7fd206` and `21ea1d48922d68bbf89ff8ba87daba5abce391782bdfcd58f6febf47741db5d8`, generated and compiled fixtures, source/binary hashes, return-value vectors, results and audits are `ginux-g29-reset-*` in the ignored local Daybreak evidence directory.

This verifies one controlled reset timing and two copy-fault forms. It does not cover concurrent aliases/SO_ERROR consumers, reset during a pending read or write, nonblocking retry ordering, SIGPIPE targeting, timeout/revoke races or injected backend errors. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: continue independent G29 SIGPIPE/MSG_NOSIGNAL and G28 backend fault criteria, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 MSG_NOSIGNAL and default SIGPIPE after TCP reset — 2026-11-07

Two disposable configured-network runs on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` repeated the controlled loopback `SO_LINGER(1,0)` reset after the guest sent `X`. Both observed poll revents `29`, `SO_ERROR=ECONNRESET` once then zero, EOF on read, and `sendto(..., MSG_NOSIGNAL)=-EPIPE` (`-32`). In the first run, the guest survived that send and exited 0/`exited/0`. In the second, a subsequent plain `write` returned `-EPIPE` and the guest terminated by SIGPIPE: launcher exit 141, result `signaled/13`, audit `signal-forwarded signal=13`, and supervisor teardown `tasks=0 owned-handles=0`. The supervisor itself completed and reported `workload-signal=13`. Exact scripts SHA-256 `9dbdf5b779d2f13db35bbf7aa94a8db51006aed62924fa1ed4e6626e25d165f6` and `bd2d578df19949618e61c52b3bdef340c97c8b2ab96b9baebfefeb0d43b58356`, generated/compiled fixtures, source/binary hashes, results, return-value vectors and audits are `ginux-g29-msg-nosignal-*` and `ginux-g29-sigpipe-*` in the ignored local Daybreak evidence directory.

This proves default SIGPIPE targeting for one single-task guest and MSG_NOSIGNAL suppression after one reset. It does not cover shared-thread signal targeting, installed signal handlers, simultaneous aliases, graceful peer close, blocked retry or other error timings. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect independent G28 backend failure/revocation and G29 concurrent alias cases, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 forked shared-socket readers and byte-budget accounting — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` connected one socket to a numeric loopback peer that sent `ABCDEFGH` under an eight-byte transport budget. The guest forked two children sharing that socket, released them through a pipe, and each read four bytes. The parent received two atomic child records and verified their payloads were exactly `ABCD` and `EFGH` in either order, with distinct child IDs; both children exited zero. A further parent read returned `EDQUOT` (`-122`), showing the shared budget was exhausted once. The first corrected run and 12 additional fresh launches all exited 0/`exited/0` with `g29-fork-read: PASS`; all 12 repeat receipts and audits are retained. The initial disposable C fixture failed compilation under `-Werror` on a signed comparison; that script is retained as `*-initial.py`, was corrected before execution, and is not a Ginux failure or runtime pass. Exact corrected script SHA-256 `a758022c608f00fefe2e7a4029d6fffc931ecccb9af7d4b4c42fe12d35a08851`, generated/compiled fixture, source/binary hashes and 12-run summary SHA-256 `45908e86c10018c261493c8d25943ade7580cffa783a80433b638c9edff56dfe` are `ginux-g29-fork-read-*` in the ignored local Daybreak evidence directory.

This verifies synchronized forked readers on one socket across 13 launches, not parallel kernel-thread execution or every possible scheduling interleaving. It does not cover one child dying mid-transfer, close/revoke during blocked read, mutable guest buffers or G07 shared-thread signal/call ownership. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 blocked-read cancellation and G28 backend fault paths, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 parked guest read killed and transport reclaimed — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` connected to a numeric loopback peer that sent no data. After the guest printed `network-park`, its blocking socket read entered the supervisor's parked lane (`audit syscall=0` and `park surface=lane-channel fd=3`). The host sent SIGKILL to the traced guest PID, not the supervisor. The launcher exited 137/result `signaled/9`; the guest PID was reaped, the peer received EOF, and the supervisor audit reported `teardown tasks=0 owned-handles=0` and `workload-signal=9`. Exact script SHA-256 `b5a959e23f272d1162aab976900e7b6d034d6b7e1b673c680735c20dc6a82b8c`, generated/compiled fixture, source/binary hashes, result, PID/peer observations and audit are `ginux-g29-read-kill-*` in the ignored local Daybreak evidence directory.

This verifies one single-task blocked-read death and cleanup. It does not cover a close or revoke racing the read, simultaneous aliases, shared-thread cancellation, a backend close fault or pending-send cleanup. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G28 backend fault accounting and G29 alias close/revoke races, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G28 injected backend errors, short counts and lifetime — 2026-11-07

A disposable host Rust harness exercised the public `BoundedTcp` API on local integration source `e354efd5` without changing Ginux. `CARGO_TARGET_DIR=.artifacts/recipe-integration/daybreak-unit-target RUSTFLAGS='' cargo build -p ginux-devices` completed on the current source; the harness linked the resulting host library SHA-256 `98fc9f5e3040d9e4327e422450b0d3399815c1f8a2fcfd2c97e4362378387218`, compiled, and exited 0 with `g28-backend-fault: PASS`. Injected backend read error `104` preserved the five-byte budget, a two-byte short read charged only two, and an impossible overlong read returned backend error `5` without charging. A one-byte short write charged one; backend write error `32` and overlong write preserved the remaining two; a final two-byte write exhausted the budget and the next read returned `ByteLimit`. Close, revoke, repeated revoke and drop called backend close exactly once. In a separate network instance, a backend connect error `111` left the public socket unconnected and retryable; successful retry then closed once on drop. Exact harness SHA-256 `36ac380f67164bf70416a861966842bd3c8d948a56ddf48d8f0c8ce9441cfce9`, compiled binary, source/lockfile/library hashes and command receipt are `ginux-g28-backend-fault.*` in the ignored local Daybreak evidence directory.

This verifies injected count/error accounting at the generic backend owner, not the Linux adapter's behavior under kernel FD faults, concurrent revoke/connect/close, host FD/CLOEXEC authority or guest transport integration. G28 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect live G28 adapter fault/FD cleanup paths and G29 alias close races, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 child-held alias survives parent close during parked read — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` connected to a numeric loopback peer, duplicated its public socket, and forked. The child closed its original public descriptor, kept the alias, and entered a blocking four-byte read; the audit showed `virtual-pid=2 syscall=0 action=park` on `fd=4`. Only after that park was observed did the host release the parent to close both of its socket aliases. The audit shows the parent's socket closes at sequences 23 and 26 while the child remained parked. After the parent reported `parent-closed`, the peer sent `PING`; the child read all four bytes through its alias and delivered them to the parent, both processes exited zero, and the peer observed EOF after the last child reference closed. Launch exited 0/`exited/0` with `g29-alias-close: PASS`; teardown reported `tasks=0 owned-handles=0`. Exact script SHA-256 `a5f6a3703ba373dbb7c6b730d7a293a1129be61912991e786add66d772d262c2`, generated/compiled fixture, source/binary hashes, handshake markers, result, peer observation and audit are `ginux-g29-alias-close-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized cross-process alias lifetime and read wakeup. It does not cover same-task concurrent threads, close of the last reference while a call is pinned, fork/exec overlap, backend close errors or repeated timing schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect last-reference cancellation and G28 Linux adapter fault/FD authority, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 parked last socket alias killed and released — 2026-11-07

The synchronized fork/alias fixture was rerun on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` with a different final event. The child held the final public socket alias and was parked in a four-byte network read; the parent closed both of its aliases (audit sequences 23 and 26) while the child remained parked. After the parent reported `parent-closed`, the host sent SIGKILL specifically to the traced child. The parent's data pipe returned EOF, `wait4` observed child signal 9, and the parent exited 0 with `g29-alias-kill: PASS`. The child host PID was reaped, the loopback peer received EOF, and supervisor teardown reported `tasks=0 owned-handles=0`. Exact script SHA-256 `b3c1451d5e61ea3300075f29d7a788d51fd8e3dcb5170dfce4a184715bdf7a0b`, generated/compiled fixture, source/binary hashes, PID/peer observations, result and audit are `ginux-g29-alias-kill-*` in the ignored local Daybreak evidence directory.

This verifies one last-reference cancellation after a synchronized forked read park. It does not cover simultaneous shared-thread close/cancel, `dup2` replacement, backend close error, SIGTERM handler behavior or repeated timing schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G28 Linux adapter FD failure handling and remaining G29 alias/exec paths, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G28 Linux TCP adapter injected socket/connect failures — 2026-11-07

Two disposable configured-network probes on local integration `e354efd5` and exact binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` used one granted numeric loopback listener. A one-shot `strace` fault made the supervisor's first `socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, IPPROTO_TCP)` return `EMFILE`; guest `connect` returned `-24`, then retry on the same public socket succeeded. A separate one-shot fault made its first host `connect(5, ...)` return `EACCES`; the trace shows `close(5)=0` before a fresh host socket reused fd 5, and the guest saw `-13` then a successful retry. Each launcher exited 0/`exited/0`, printed its exact PASS marker, and the listener accepted exactly one connection with no extra pending connection. The initial runs reached those guest outcomes but their probe scripts exited 1 because an assertion expected `EACCES (INJECTED)` or `EMFILE (INJECTED)` without `strace`'s intervening human-readable errno. Corrected assertion reruns exited 0; initial script assertion failures were not counted as passes. Corrected scripts SHA-256 `4b04cff6df29af69147178cb6a90d22edf8d98dc4a5440a099439f847015ee22` and `5af285fb48af510746cea7f4e6ad98f6446e73e32ccbd85eb0e5e1d0513fa18d`, generated C fixtures, compiled fixtures, receipts, traces and audits are `ginux-g28-{socket-emfile,connect-eacces}*` in the ignored local Daybreak evidence directory.

These prove one host socket allocation failure and one immediate host connect failure return cleanup, public-socket retry, and selected `CLOEXEC|NONBLOCK` host creation flags. `strace` skipped injected calls, so this does not prove actual kernel FD lifetime under a real fault. Concurrent revoke/connect/close, in-progress cancellation, other backend faults, host FD confinement and full G28 acceptance remain partial. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction; G02 clock policy remains held. Exact next action: inspect independent G28 in-progress cancellation/FD authority and G29 alias/exec paths, then retest changed storage after upstream wave-one merges. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 concurrent private mapping FD ownership denial — 2026-11-07

A disposable variant of the existing configured `v-private` fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` added `fchown(fd,0,0)` and `fchownat(fd,"",0,0,AT_EMPTY_PATH)` attempts for candidate private native FD numbers 5–7 in the peer thread's loop. Neither ownership call reported success while the owner performed 64 owned-file mappings; existing `fstat`, `fcntl`, `close` and `mmap` guest authority checks also passed. The selected runner exited 0 and printed `PASS launch threads: v-private` and `test-launch-threads: PASS`. A separate rerun under `strace -e trace=chown,lchown,fchown,fchownat` on the supervisor passed identically; the trace contained zero host ownership syscalls. Exact generated fixture, compiled fixture, scripts SHA-256 `49e874706449a3882bf3273a1c5e3e523957b1c109c822355815239dc4b9350b` and `3ca79da0358fcc1535363d016b14eacc1cbe838289c7221bf5fbe6273687d999`, receipts, logs, wrapper and trace are `ginux-g30-private-owner*` in the ignored local Daybreak evidence directory.

This tests denied guest ownership access to selected candidate private FD numbers during repeated mappings and no supervisor host ownership call in that run. It does not prove the peer attempted each private Pair/Receive/Map/Close instant, all FD numbers, exact errno, shared path mutation or other concurrent rename/close/exec cases. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 exec/alias authority and G30 concurrent path/descriptor ownership edges, then retest source-dependent behavior after upstream storage changes. No source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 connected socket alias close-on-exec in both directions — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` ran twice against fresh granted numeric loopback listeners. Mode X created fd 3 with `SOCK_CLOEXEC`, duplicated it to public fd 4 without close-on-exec, then executed the same image. In the new image fd 3 returned `EBADF`, fd 4 retained `FD_CLOEXEC=0`, read exactly `PING` and closed. Mode Y created fd 3 without close-on-exec, used `dup3` to create fd 4 with `O_CLOEXEC`, then executed; fd 4 returned `EBADF` and fd 3 read `PING`. Each launch exited 0/`exited/0` with exact `g29-exec: PASS`, one peer connection, peer EOF and teardown `tasks=0 owned-handles=0`; audits recorded `task exec virtual-pid=1` and the expected closed/live descriptor results. Exact script SHA-256 `c5d44946b61b0a07842ae62c148783595cd23eb2a771ab619c89fdd501350b92`, generated source/binary, source/binary hashes, per-mode receipts/audits and summary are `ginux-g29-exec-alias-*` in the ignored local Daybreak evidence directory.

This checks per-public-alias close-on-exec and continued connection authority in two serial single-process transitions. It does not cover fork-plus-exec overlap, simultaneous shared threads, exec failure rollback, a pending read during exec, native host FD leakage or all transport cleanup paths. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 failed-exec rollback and shared-thread/alias races, then continue independent G30 path authority and source-dependent retests after upstream storage changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 connected socket aliases survive missing-image exec failure — 2026-11-07

A disposable configured TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` connected to one granted numeric loopback peer with fd 3 marked `FD_CLOEXEC` and a duplicated fd 4 without that flag. `execve("/bin/definitely-missing",...)` returned `ENOENT` (`-2`); afterward the two flags still read 1 and 0, fd 4 read exactly `PING`, and both aliases closed normally. Launch exited 0/`exited/0` with `g29-failed-exec: PASS`, one peer connection, peer EOF, no task-exec audit event and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `c0ffce028bd052da48a534a679b2a739cb4b228720140a8d8b9fafba2a4f6ad6`, generated source/binary, source/binary hashes, receipt, audit and summary are `ginux-g29-failed-exec-alias-*` in the ignored local Daybreak evidence directory.

This checks an early missing-path failure before exec commit, alongside the successful exec alias cases. It does not cover a late kernel exec failure after descriptor preparation, fork-plus-exec overlap, shared threads or pending transport calls during exec. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect later exec-failure stages and G29 shared-thread/alias cancellation, then continue independent G30 path authority and source-dependent retests after upstream storage changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 protected cross-page owned pathname — 2026-11-07

A disposable variant of the existing configured-storage ownership fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` created `/work/probe`, placed that pathname across two anonymous pages and protected the second page. `chown` and `fchownat` on the cross-page pathname each returned `EFAULT`; a pointer beginning in the protected page also returned `EFAULT`. After unprotecting, both root-preserving ownership calls succeeded on the same path, and `fstat` still projected UID/GID 0. The fixture removed its temporary file and mapping; the unchanged storage runner's ownership selection passed with exit 0 and `PASS launch storage: ownership`, including its host-image preservation checks. A rerun with supervisor-only `strace -e trace=chown,lchown,fchown,fchownat` also passed and recorded zero host ownership syscalls. Exact generated source/binary, scripts SHA-256 `5b816b732a8ab0a98d491dc8e42fcb11bb0caba9a2de278fdcb8791c3e8c8438` and `dc371c132331cc15d616d3456849e13bb9a6234c5986621ed0b8052bda8bc794`, receipts, logs, wrapper and trace are `ginux-g30-protected-path*` in the ignored local Daybreak evidence directory.

This verifies selected protected and cross-page pathname faults, subsequent valid access, and no supervisor host ownership syscall in that run. It does not cover concurrent mutation of a readable shared pathname, every partial lookup/copy fault, rename/close/exec races, or the known storage descriptor canonicality failure. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 shared writable path capture and later G29 exec/alias timing, then retest storage-dependent behavior after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 shared-thread mutable ownership pathname — 2026-11-07

A disposable variant of the configured `x-path` thread fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` created an owned `/work/ow` object, then a peer thread repeatedly toggled one shared pathname word between `/work/ow` and the unassigned image path `/allowed`. The owner alternated `chown` and `fchownat` for 4,096 calls, accepted only success or known denial/missing-path results, and required at least one success and one denial before joining the writer. It then succeeded on literal `/work/ow` and removed it. The selected runner passed with exit 0 and `PASS launch threads: x-path`; a supervisor-only `strace -e trace=chown,lchown,fchown,fchownat` replay also passed and showed zero host ownership syscalls. The configured argument guard's `path_call` list includes syscalls 92, 94 and 260. Exact generated fixture, compiled fixture, scripts SHA-256 `05909709a50c5e92973a307de440613f402f560c28a0919ab24ee9e0be1092dd` and `25ec7e77eb76d245f11111eac8c5dac13a301eef0201dcd35b291635ded43b90`, receipts, logs, wrapper and trace are `ginux-g30-mutable-path*` in the ignored local Daybreak evidence directory.

This verifies selected shared-thread path mutation with both routing outcomes and no supervisor host ownership call. It does not establish every path-capture/dispatch interleaving, an exact per-call pathname snapshot, adversarial multibyte transient names, or concurrent rename/close/exec behavior. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 descriptor/path race and G29 late-exec/alias timing, then retest storage-dependent behavior after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G30 shared-table owned/native descriptor replacement — 2026-11-07

A disposable configured-thread fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` used owned fd 3 (`/work/shared`) and native image fd 4 (`/input`). Stable controls confirmed `dup2(3,5)` made `fchown(5,0,0)` and empty-path `fchownat(5,"",0,0,AT_EMPTY_PATH)` succeed, while `dup2(4,5)` made both return their native denial results. A peer thread then alternated those two `dup2` replacements while the owner made 1,024 alternating ownership calls. The fixture required at least one swap, one success, one exact native-denial result and no other result; final owned/native controls passed. The selected thread runner exited 0 with `PASS launch threads: x-path`. A traced replay passed identically and showed zero supervisor `chown`/`lchown`/`fchown`/`fchownat` syscalls. Exact generated source/binary, scripts SHA-256 `5448b389ac3b86fc899ec03ae04edf6c57db006b02f579858fda082ce0a62f3d` and `193f4985cf483dd8bc5280f71df7c0aa82107ce8be097b75108f1fe91e6a87c1`, receipts, logs, wrapper and trace are `ginux-g30-fd-swap*` in the ignored local Daybreak evidence directory.

This checks selected owned/native public-slot replacement under shared-table concurrency and no host ownership syscall. It does not prove every interleaving, simultaneous close/exec, a specific per-call descriptor snapshot, all native FD classes or high-bit descriptor canonicality. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 shared-thread socket alias cancellation and G30 rename/exec overlap, then retest changed storage integration after upstream merges. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 shared-thread read survives last public socket close — 2026-11-07

A disposable configured-thread/TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` connected fd 5 to one granted numeric loopback listener and spawned a shared-table thread to read four bytes. The host waited for task 2's `syscall=0 action=park` on `lane-channel fd=5` before releasing task 1. Task 1 closed fd 5 (audit sequence 13), confirmed its public slot returned `EBADF` (sequence 16), and reported `network-last-close`; task 2 remained parked through audit sequence 20. Only then did the peer send `PING`. Task 2 read all four bytes (sequence 22), the parent joined it and printed `owned-threads: PASS`; the peer observed EOF after the pinned call released. Launch exited 0/`exited/0`, accepted one connection and reported teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `ae9da395e0bb46556915a7fb6c38071f2895c22b68bc828c82e3c9acc9e60ff0`, generated source/binary, source/binary hashes, markers, result and audit are `ginux-g29-thread-last-close-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized shared-thread in-flight read pin after the final public descriptor closes, plus eventual transport cleanup. It does not cover a second simultaneous network call, `dup2` replacement during the pinned call, peer reset, task death while parked, pending send, or all shared-thread cancellation schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: test task death or `dup2` replacement during a pinned network call, then continue G30 rename/exec overlap and storage-dependent retests after upstream merges. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 shared-thread socket slot replaced during pinned read — 2026-11-07

A disposable configured-thread/TCP fixture on local integration `e354efd5`/binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6` reused the synchronized parked-read setup. Task 2 parked reading four bytes from connected public fd 5; only after the host observed that park did task 1 `dup2` its native image fd 4 over public slot 5. The audit records replacement at sequence 13, while task 2 remained parked through sequence 19. Task 1 read byte `a` from the replacement fd 5 and printed `network-slot-replaced`; the peer then sent `PING`, which task 2 read in full at sequence 21. After its pending pin released, the peer saw EOF. Launch exited 0/`exited/0` with `owned-threads: PASS`, one peer connection and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `69cb02b28e13f23392314ba479bf585d7b7393b204d6576ae29badca4f11423a`, generated source/binary, source/binary hashes, markers, result and audit are `ginux-g29-thread-dup2-replace-*` in the ignored local Daybreak evidence directory.

This verifies one shared-thread `dup2` replacement after a network read has parked: the caller retained the old socket operation while the public number gained native image-file authority. It does not cover simultaneous writes, `dup3` close-on-exec replacement, signal death, peer reset, repeated interleavings, or all cancellation paths. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G29 pending-send and shared-thread signal cancellation, then continue G30 rename/exec overlap and storage-dependent retests after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 shared-thread SIGPIPE after a controlled TCP reset — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured nonleader thread observed poll error readiness, `SO_ERROR=ECONNRESET`, EOF and `sendto(MSG_NOSIGNAL)=-EPIPE` after one numeric loopback peer reset. A following plain write received `EPIPE` and forwarded SIGPIPE. The launcher exited 141/`signaled/13`; the audit shows virtual task 2's write, `signal-forwarded signal=13`, exits for both tasks and teardown `tasks=0 owned-handles=0`. The peer received exactly `X`. Exact script SHA-256 `6c74ab989a7988e8eeadf8bb25a6ee7e8e3dfa7b24aae4b2e55d311c8c66cac9`, generated fixture, binary, hashes, receipt and audit are `ginux-g29-thread-sigpipe-*` in ignored local Daybreak evidence. An initial unprivileged launch failed before guest execution on required chroot privilege; it is not a runtime pass.

This narrows G29 thread signal targeting to one default SIGPIPE/reset schedule. Installed handlers, masks, direct per-thread signals, pending sends and broader cancellation remain unverified. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect a bounded pending-send pin or shared-thread cancellation path, then continue G30 rename/exec overlap and source-dependent storage retests. No source commit or upstream action occurred.

### G29 shared-thread parked read cancelled by virtual SIGKILL — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture parked virtual task 2 on a four-byte read from fd 5. The host observed `syscall=0 action=park` before releasing task 1. Task 1 issued `kill(2,SIGKILL)` at audit sequence 15 with emulated result 0. Both tasks exited without the read returning; the peer saw EOF, launcher exit was 137/`signaled/9`, and teardown reported `tasks=0 owned-handles=0`. Exact probe script SHA-256 `ed9da6a2f871c08ba31e9b082ce8a4ba7e9f134221b1fdb0c9d128ef3ef447b1`, generated fixture, binary, hashes, receipt and audit are `ginux-g29-thread-cancel-*` in the ignored local Daybreak evidence directory.

This narrows G29 cancellation coverage to one synchronized virtual SIGKILL of a parked nonleader read. Pending send, graceful per-thread interruption, signal masks/handlers, concurrent close and broader schedules remain unverified. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect and probe a bounded pending-send pin under transport backpressure, then continue G30 rename/exec overlap and source-dependent storage retests. No Ginux source commit or upstream action occurred.

### G29 shared-thread pending send survives public slot replacement — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, nonleader task 2 sent 4,096-byte `A` blocks to a numeric loopback peer withholding reads under a 16 MiB transport budget. Its write parked on socket fd 5 at audit sequence 447. Task 1 replaced public fd 5 with native image fd 4 using `dup2` at sequence 455, read `a` from the replacement at sequence 458 and reported `send-slot-replaced`; task 2 remained parked through sequence 461. After peer drainage, the pending write returned 4,096 bytes at sequence 463. The peer received exactly 1,798,144 `A` bytes and EOF. Launch exited 0/`exited/0` with `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `3f0af321357a18a4f5791c53e620821bdbea97f7baf73674311ec1c77db06b75`, generated fixture, binary, hashes, receipt and audit are `ginux-g29-thread-send-pin-*` in ignored local Daybreak evidence.

This covers one backpressured pinned send across shared-table public slot replacement. Send cancellation, reset while parked, partial backend writes, mutable payload, concurrent aliases and other schedules remain unverified. G29 stays partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G30 rename/exec overlap and remaining G28/G29 backend-failure/revocation paths, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G30 shared-thread rename and ownership with exec denied — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured fixture kept owned directory fd 5 open while task 2 repeatedly renamed `/work/a` to `/work/b` and back. Task 1's `execve` returned `-95` while task 2 lived; 1,024 checked relative `fchownat(5,"item",0,0,0)` and `fchown(3,0,0)` pairs succeeded during the renames. After join, the final `/work/a/item` path succeeded and `/work/b/item` returned `ENOENT`. Ordinary and traced launches both exited 0/`exited/0` with exact `g30-rename-exec: PASS` and `owned-threads: PASS`; teardown was `tasks=0 owned-handles=0`. Supervisor-only `strace -e trace=chown,lchown,fchown,fchownat` recorded zero host ownership syscalls. Exact scripts SHA-256 `c6e1799af52095a1845954403dd64d132011c5cd45bf46676d685e8402911cfe` and `c13307f442f58b9307cce1e39fe9b59ac4c8b389efe3d146c91ff65b91e0c8f2`, generated fixture/binary, hashes, receipts, audits and trace are `ginux-g30-rename-exec-overlap-*` in ignored local Daybreak evidence. Audit output truncated at 65,536 bytes; fixture assertions cover the remainder, and the trace covers the supervisor only.

This covers one shared-thread rename/ownership overlap and fail-closed exec while the peer lives. Successful exec after join, descriptor close/reuse, host image mutation, exact rename snapshots and other schedules remain unverified. G30 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect successful exec ownership/descriptor retirement and independent G28/G29 backend-failure/revocation paths, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G30 owned descriptor authority across successful exec — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured fixture kept owned directory fd 3 live and marked owned file fd 4 `FD_CLOEXEC` before successful self-exec. The post-exec image made `fchownat(3,"item",0,0,0)` successfully; `fchown(4,0,0)` and empty-path `fchownat(4,"",0,0,AT_EMPTY_PATH)` returned `EBADF`. Reopening the same owned file reused fd 4 and permitted `fchown`, with virtual UID/GID zero on `fstat`. Both ordinary and traced launches exited 0/`exited/0` with exact `g30-exec-owner: PASS`; audit recorded `task exec virtual-pid=1` and teardown `tasks=0 owned-handles=0`. Supervisor-only `strace -e trace=chown,lchown,fchown,fchownat` recorded zero host ownership syscalls. Exact scripts SHA-256 `3c60838825a6dc8bbfdb48a98cd942a0c31d174637ad04f4942c76467e009c47` and `608142efee4b4914ab356a1de1fc35a1c02d861210f3bdd115b7f8a9bbf61840`, generated fixture, hashes, receipts, audits and trace are `ginux-g30-exec-owner-*` in ignored local Daybreak evidence.

This covers one successful single-process exec descriptor-retirement schedule. Concurrent exec after a shared-thread join, late failure rollback, rename during successful exec, native/private FD collisions and broader ownership authority remain unverified. G30 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect independent G28/G29 backend-failure/revocation and G30 native/private descriptor edges, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G28 Linux TCP adapter injected send/receive errors and quota — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture used an eight-byte budget and one granted loopback peer. Supervisor-only `strace` injected `EIO` on the first host `sendto` and `recvfrom`. Guest write and read returned `-EIO` at audit sequences 4 and 6, then retries sent exactly `PING` and received exactly `PONG`. Both subsequent one-byte calls returned `EDQUOT`, confirming only the successful eight bytes were charged. The peer received `PING`, saw EOF, and had no second connection. The trace shows a `SOCK_CLOEXEC|SOCK_NONBLOCK` host socket, `EINPROGRESS` connect, two injected errors, successful retries and `close(5)=0`. Launch exited 0/`exited/0` with exact `g28-io-fault: PASS` and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `342d52830642a42f096bec626b87a995c89ab4eeb471add7be8c22d5a9c79baf`, trace SHA-256 `17cd01e032d1464fa4c1d149eb6999909462c1fbf2ace8e944622cce1e484f82`, generated fixture/binary, hashes, receipt and audit are `ginux-g28-io-fault-*` in ignored local Daybreak evidence.

This covers one Linux adapter error/retry schedule and guest budget behavior. `strace` skips injected kernel calls; real host I/O failure lifetime, concurrent revoke/connect/close, other backend faults and cancellation remain unverified. G28/G29 remain partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect a bounded adapter close/fault or G29 alias/revocation race, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G29 parked read wakes on TCP reset — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture sent `X` to one granted loopback peer and parked reading four bytes. Only after the host observed `syscall=0 action=park` on fd 3 did it reset the connection with `SO_LINGER(1,0)`. The pending read returned `-ECONNRESET` (`-104`, audit sequence 8), then `SO_ERROR` returned zero and another read returned EOF. Launch exited 0/`exited/0` with exact vector `[-104,0,0]`, `g29-reset-park: PASS`, and teardown `tasks=0 owned-handles=0`; the peer received exactly `X`. Exact script SHA-256 `61f25e81543f2f5d73f56edae5a53a7ab549ca690e32b8e75a9ec430574c9a7b`, fixture, binary, hashes, receipt and audit are `ginux-g29-pending-reset-*` in ignored local Daybreak evidence.

This covers one parked single-task read/reset schedule. Parked sends, concurrent aliases, shared-thread reset, close races, graceful FIN and broader SO_ERROR ordering remain unverified. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect a bounded pending-send reset or G28 close/fault lifetime path, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G29 parked shared-thread send wakes on TCP reset — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, nonleader task 2 sent 4,096-byte `A` blocks under a 16 MiB budget to one numeric loopback peer withholding reads. With guest SIGPIPE ignored, its write parked on fd 5 at audit sequence 448. The host then reset the peer with `SO_LINGER(1,0)`; the parked write resumed with `-ECONNRESET` (`-104`) at sequence 453. Earlier writes reported 1,794,048 accepted bytes, which were not confirmed delivered because the peer did not read. Launch exited 0/`exited/0` with exact `g29-send-reset: PASS` and `owned-threads: PASS`; teardown was `tasks=0 owned-handles=0`. Exact script SHA-256 `89ef1abe4c156fa1540c7324f80962d75e64ac1eabc3f6e67a45eb68082d33ee`, fixture, binary, hashes, receipt and audit are `ginux-g29-pending-send-reset-*` in ignored local Daybreak evidence.

This covers one parked shared-thread send/reset schedule under ignored SIGPIPE. Default SIGPIPE on later `EPIPE`, remote delivery, alias close, partial writes and broader schedules remain unverified. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G28 close/fault lifetime or G29 concurrent alias/SO_ERROR consumption, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.

### G29 shared-thread SO_ERROR consumption across socket aliases — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured fixture used a nonblocking socket to a bound, non-listening granted loopback port. Connect returned `EINPROGRESS`; after poll completion, task 1 duplicated fd 5 to fd 6. Task 1 and task 2 started `SO_ERROR` queries on the two aliases from one shared gate. The audit serialized them at sequences 9 and 10: task 1 received `ECONNREFUSED` (`111`), task 2 zero. Later read/write remained `ECONNREFUSED`, including after closing the original public fd. Launch exited 0/`exited/0` with vector `[111,0]`, `g29-soerror-thread: PASS`, `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `41445f22c8f8b539bdfa9c242390a0a94533f2e0551897bfd7325ac63d456345`, fixture, binary, hashes, receipt and audit are `ginux-g29-soerror-thread-*` in ignored local Daybreak evidence. The prior serial alias-first refusal case covers the other consumer order.

This covers one same-start shared-thread `SO_ERROR` consumption schedule, not every interleaving, reset-state contention, concurrent copy faults or alias-close race. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect a bounded G28 close/fault lifetime or G29 alias-close/connect race, then retest source-dependent storage after upstream changes. No Ginux source commit or upstream action occurred.


### G29 parked shared-thread read receives graceful FIN after last public close — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture parked nonleader task 2 reading four bytes from fd 5. Only after the host observed its parked read did task 1 close the last public fd 5 (audit sequence 13), verify `EBADF` on that number (sequence 16), and report `network-last-close` while task 2 remained parked (sequence 20). The loopback peer then made a graceful `SHUT_WR` without sending data. The pending read returned 0 (sequence 22), the peer observed EOF after the pin released, and the launcher exited 0/`exited/0` with `owned-threads: PASS`, one connection, no extra connection and teardown `tasks=0 owned-handles=0`. Exact probe script SHA-256 `00569582158814b7934ed99b209ddf5fd82a1051c0d5dd0e3c27179845595062`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g29-thread-fin-close-*` in ignored local Daybreak evidence.

This verifies one graceful FIN and last-close schedule, complementing the prior pinned data read and reset cases. It does not prove every close/read interleaving, simultaneous aliases, reconnect while an old call is pinned, backend close faults or broader cancellation. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G29 alias-close/connect race or G28 close/fault lifetime, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 refused alias remains isolated during fresh connect — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture used two granted numeric loopback endpoints: one bound but not listening, one listening. Task 1 started a nonblocking connection to the refused endpoint, duplicated public fd 5 to fd 6, polled completion, closed fd 5 and opened a fresh socket that reused public fd 5. It released task 2 to query `SO_ERROR` on old alias fd 6 while task 1 connected the new fd 5 to the allowed listener. The audit placed task 1's new connect at sequence 11 and task 2's alias query at sequence 12 before either exit record. The exact result vector was `[111,0]`: old alias `ECONNREFUSED`, fresh socket zero. A read through the old alias remained `-ECONNREFUSED`; the allowed peer accepted one connection and saw EOF, with no extra connection. Launch exited 0/`exited/0` with `g29-alias-connect: PASS`, `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `c8e6688b6ae3d0a31d20d3a011d4d08189950bf754e25d4e3aede124aec07b14`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g29-alias-connect-*` in ignored local Daybreak evidence.

This verifies one overlapping old-alias query and new connection schedule after public-slot reuse. Audit entry overlap does not prove simultaneous backend execution or every interleaving. It does not cover active old connections during replacement, close faults, revoke during connect, alias cancellation or other endpoint states. G29 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G28 close/fault lifetime or a G29 live-alias close/connect path, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G29 live alias survives public-slot reuse and second connection — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured-thread fixture connected public fd 5 to one granted loopback listener, duplicated it to fd 6 and parked task 2 reading fd 6. After the host observed the parked read, task 1 closed fd 5 at audit sequence 14, confirmed its old public slot returned `EBADF`, opened a second socket that reused fd 5 at sequence 20, and connected it to a different granted listener at sequence 23. The old read remained parked through sequence 27; task 1 then parked reading new fd 5. The host sent `PING` only to the old peer and `PONG` only to the new peer. Task 2 received `PING` and task 1 received `PONG` (audit sequences 29–30); a further read returned `EDQUOT` (`-122`) after the eight-byte shared transport budget was spent. Both peers saw EOF, neither listener received an extra connection, and launch exited 0/`exited/0` with `g29-live-alias: PASS`, `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact probe script SHA-256 `6c5565abd15da6709a96208fb32f443ba7e5687a0a79988f26199b4f7c98ea07`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g29-live-alias-reuse-*` in ignored local Daybreak evidence.

This verifies one connected old-alias/new-socket schedule with separate data and one shared byte budget. It does not prove every close/connect interleaving, transport revoke under concurrency, host close-error semantics or endpoint mutation. G28/G29 remain partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G28 Linux close/fault lifetime evidence and remaining cross-surface Daybreak criteria, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### Current-source full G08a poll runner — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, the unchanged `scripts/test-launch-poll.py` default runner passed all 12 modes with a freshly compiled fixture: `all`, `read`, `write`, `hup`, `deadline`, `timer`, `console`, `kill`, `infinite-kill`, `all-again`, `ordinary`, and `ordinary-timer`. The command exited 0 in 0.422 seconds with `test-launch-poll: PASS`. The fixture's `all` mode asserts `EFAULT` on invalid and read-only poll arrays and a cross-page array, `POLLNVAL` for unknown/closed numbers, `ENOTSUP` for unsupported image-file polling, alias and EOF readiness, and pipe capacity transitions. Other modes check finite/infinite waits, peer wakeups, mixed console readiness, and reaping of a killed parked task. Exact wrapper script SHA-256 `531da38f53331d0182bda9360680aaf04a4bd9856dee4882306fd359edcd0006`, command, source/runner/fixture/binary hashes, output and exit are `ginux-g08a-poll-full-*` in ignored local Daybreak evidence.

This is fresh current-source evidence for those specific assertions. It does not establish mutable shared poll-array capture, byte-for-byte partial-output rollback, every native/private descriptor class, concurrent slot replacement, all interruption schedules or host clock policy. G08a remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G08a mutable/partial poll-array authority and G28 host close-fault limits, then continue independent Daybreak criteria and retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G08a parked poll snapshots input and reports partial output fault — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured shared-thread fixture placed two `pollfd` entries across a page boundary, both initially reading pipe fd 5. The host observed task 1 park in poll at audit sequence 9, then released task 2. Task 2 changed the second entry's fd to unsupported image fd 4, made its page read-only with `mprotect` (sequence 11), and wrote one byte to the pipe (sequence 12). Poll resumed and returned `EFAULT` (`-14`, sequence 13), rather than an image-fd `ENOTSUP`: it used the captured original input and attempted output. The exact guest result vector was `[-14,1,32639,4]`: the first writable `revents` became `POLLIN`, the second retained its `0x7f7f` sentinel, and the mutated second fd stayed 4. After restoring write access, the guest consumed the wake byte and exited 0/`exited/0` with `g08a-poll-mutation: PASS`, `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact corrected probe script SHA-256 `b44c996cbdd20a862688c4c39c2ca79989e33db873df10f769ba80c8557b32c8`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g08a-poll-mutation-*` in ignored local Daybreak evidence. The first attempt timed out only because its harness searched for the wrong audit text; its script/audit are retained as `*-initial.*` and are not counted as a Ginux result or pass.

This verifies one parked input-snapshot and sequential partial-output fault schedule. It does not prove atomic poll output rollback, every shared-memory mutation, concurrent public-slot replacement or cancellation path; the observed first-entry write establishes that this selected output fault is partial. G08a remains partial. G28 normal close/drop/revoke and backend fault controls already have current-source evidence, but a synthetic `strace` close error skips the kernel close and cannot establish real Linux close lifetime; that criterion remains unverified. G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G08a public-slot replacement during a parked poll and then independent remaining Daybreak criteria, with source-dependent storage retests after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G08a parked poll denies image FD after public-slot replacement — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured shared-thread fixture parked task 1 polling pipe read fd 5 and a separate empty wake pipe. After the host observed `action=park surface=owned-poll` at audit sequence 9, task 2 atomically replaced public fd 5 with native image fd 4 using `dup2` (sequence 11). The poll resumed at sequence 12 and returned `ENOTSUP` (`-95`) before task 2's wake-pipe write at sequence 13. Both guest `revents` fields retained their `0x7f7f` sentinels; the result vector was `[-95,32639,32639]`. Task 1 then read image byte `a` through the replaced fd 5 and consumed the wake byte through the separate pipe. Launch exited 0/`exited/0` with `g08a-poll-slot: PASS`, `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact probe script SHA-256 `8fe4a60bcfa7a14bd5a5918b8f8a9605794635a8c5ec10df89dc2ca17fd46470`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g08a-poll-slot-replace-*` in ignored local Daybreak evidence.

This verifies one shared-table replacement of a parked poll target with a native image descriptor. The selected retry fails closed without polling the supervisor's same-numbered host FD or writing output, but it does not establish all replacement interleavings, owned-to-owned semantics, private FD classes, concurrent close, cancellation or ppoll timeout faults. G08a remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 clock policy remains held. Exact next action: inspect G08a ppoll timeout/indirect-argument faults and then independent remaining Daybreak criteria, with source-dependent storage retests after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G08a ppoll timeout indirect-argument faults — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured poll fixture made seven raw `ppoll` calls. The exact result vector was `[0,-14,-14,-22,-22,-22,-95]`: a valid zero timeout returned zero and cleared `revents`; an inaccessible timespec pointer and a timespec split across a protected page returned `EFAULT`; seconds 301, nanoseconds 1,000,000,000 and negative seconds returned `EINVAL`; and a non-null unsupported signal-mask argument returned `ENOTSUP`. Each failed call left the `revents` sentinel unchanged. Launch exited 0/`exited/0` with `owned-poll: PASS` and teardown `tasks=0 owned-handles=0`. Exact probe script SHA-256 `d7e38a033984943d30367310352a88c055e63c4e31ea83f574a7ae085e83dec3`, generated fixture source/binary, source and binary hashes, receipt and audit are `ginux-g08a-ppoll-timeout-*` in ignored local Daybreak evidence.

This verifies selected ppoll timeout input faults and bounds on the current integration source. It does not cover mutation of a readable timespec after a call parks, monotonic-clock syscall failure, every timeout overflow or signal-mask layout. G08a remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, and G02 guest clock policy remains held. Exact next action: inspect G08a internal monotonic-clock failure handling and remaining independent G06b/G07 security criteria, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### G08a internal supervisor clock error control — 2026-11-07

Clean local integration `e354efd5`, binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`: a disposable zero-timeout owned-pipe poll returned `(0,0)` with a successful supervisor `CLOCK_MONOTONIC` trace. Injecting `EIO` into that exact supervisor call returned `-5` to the guest and preserved the `revents=0x7f7f` sentinel. Both launches exited 0/`exited/0`, printed `owned-poll: PASS`, and tore down with `tasks=0 owned-handles=0`. The injected trace contains `INJECTED`; audit sequence 2 records `-5`. Exact probe SHA-256 `929b93222c45cb95891435314bf3612d2961da2a34ea4932a996e83c0fb3db10`; `ginux-g08a-poll-clock-fault-*` evidence is in the ignored local Daybreak directory. A first compile-only harness warning was corrected before runtime. This covers one internal clock-error path only. G08a remains partial, G02 guest clock policy held, and G03c/G12 descriptor canonicality failed pending Claude. Exact next action: inspect independent G06b/G07 criteria and run a bounded ordinary case, then retest source-dependent storage after upstream changes. No upstream action occurred.


### Current G06b/G07 selected lifecycle and signal-action review — 2026-11-07

Clean local integration `e354efd5`, binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`: freshly compiled unchanged fixtures passed selected G06b `junk-child-tid`, `protected-child-tid`, `x-child-tid`, `rollback`, `qfault` and G07 `limit`, `rollback`, `teardown` signal/failure cases through the unchanged runners, both exit 0. Exact wrapper SHA-256 `36b4946e26b4d31e6dfd014e5d7b16afc638fc96fd0d0a1392d16c0f66567634`; `ginux-g06b-g07-current-lifecycle*` receipt/logs retain source and binary hashes. This refreshes selected ordinary controls only. Source review found a pending G07 shared-argument race hypothesis: the `rt_sigaction(SIGCHLD)` anti-autoreap check reads guest action bytes, then lets the kernel consume the guest pointer while `needs_argument_guard` does not include this call. No bypass has been reproduced. Exact next action: run a bounded disposable shared-action probe/control, then fix and verify in the existing signal-action owner if reproduced. G02 held; G03c/G12 descriptor canonicality failed pending Claude; broad G07 and Recipe checks remain open. No upstream action occurred.


### G07 SIGCHLD shared-action authority correction — 2026-11-07

Pre-fix integration `e354efd5`, binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`: a stable 256-call `rt_sigaction(SIGCHLD)` control passed, while a shared writer changing the guest handler between default and `SIG_IGN` led to a successful call that installed `SIG_IGN`. The exact guest vector was `(3 successful, 2 denied, 1 exposed)`, exit 154. `ginux-g07-sigchld-action*` retains the disposable probe, audit, result and hashes; script SHA-256 `f848613dca25e474433fbeac7e1e9a212a2afcbdbc416438d570f11cb6f31622`.

Local integration commit `cde7dc22` extends the existing configured argument guard to this call and adds two modes to the existing Ginux thread regression. Corrected binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402` passed release build, formatting, strict ginux-init clippy, focused and full thread runners and the full task runner. Fresh Recipe `cargo-consumer` build/install passed in 82.314 s with two acknowledged checkpoints, `installed=true`, result `compiled rust 42\n`, launcher exit 0. Exact suite and Recipe wrappers SHA-256 `57bfb60b1535b751d66eea2df86d217b8642b06b1eb200867d7171000b972770` and `b09e941b67c41f22a8d5952b46afc3c9f1b7d239ba1bc1d30db82e793f8ed76a`; `ginux-g07-sigchld-regression-suites*` and `recipe-cargo-sigchld-guard*` contain receipts and logs. This proves one bounded race and fix, not broad G07 acceptance. Exact next action: inspect the next kernel-consumed shared argument with an allowed control, continue independent Recipe cases, and wait for wave-one merges before rebase. G02 remains held; G03c/G12 descriptor canonicality remains failed pending Claude. No upstream action occurred.


### G07 SA_NOCLDWAIT shared-action denial — 2026-11-07

An exact `e354efd5` archive rebuild reproduced the prior binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`. A stable SIGCHLD action control passed 64 calls on both pre-fix and corrected binaries. With a shared writer toggling only `SA_NOCLDWAIT`, the pre-fix binary returned guest vector `(3 successes, 2 denials, 1 exposure)` and exited 153: a successful checked call installed the denied flag. The corrected binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402` passed three 1,024-call runs with both allowed and denied calls and zero exposures: `(325,699)`, `(295,729)`, `(299,725)`. All launches tore down completely. `ginux-g07-sigchld-flags-baseline*` retains exact script (SHA-256 `0e6e87f1766ea03816256006902d7ae3e9f61d485426d9e20e8f8eb039f773e4`), fixture, audit and hashes. Local test commit `0d23d34d` adds `V-flags`; focused and full existing thread runners passed on the unchanged corrected binary. Exact runner wrapper SHA-256 `dc103c1837573a197cb8e1de58385ed8a3f9b5aa9ca674dbff566532352e3443`; receipt and logs are `ginux-g07-sigchld-flags-runner*`. This is selected SIGCHLD authority evidence, not broad G07 acceptance. Next: configured clone TID copyout under shared mutation and failure cleanup. G02 held; G03c/G12 descriptor canonicality failed pending Claude; full Recipe/F03 remains open. No upstream action occurred.


### G07 parent-TID birth copyout injected fault — 2026-11-07

On clean integration `0d23d34d`, binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402`, the unchanged `basic` thread fixture passed with a supervisor trace showing a four-byte parent-TID preflight write of zero followed by a four-byte virtual-TID-2 birth write to the same address. Injecting `EFAULT` at only the second `process_vm_writev` produced exit 1/result `supervisor-error` with null code, no guest stdout, `thread identity copyout failed`, and teardown `tasks=0 owned-handles=0`; the traced parent PID was gone. The first injected wrapper asserted an incorrect numeric result code and is retained as `*-initial.*`, not counted as a verified pass. The corrected wrapper passed. Exact positive/fault scripts SHA-256 `31846e56f00dd52dd5783b0c0ecc2effe81505fe043d7402c43e5220c5c35320` and `6ff3fed8a6275c87809dfb2bb808632e51828ae6087e9c5143ab0a1bbd601dec`; `ginux-g07-parenttid-*` retains traces, audit, hashes and receipts. This is a synthetic supervisor write failure, not a guest mutation proof. Next: a bounded shared-thread peer mutating a dedicated parent-TID page during clone, with safe-success and failed-birth teardown checks. G07 remains partial; G02 held; G03c/G12 canonicality failed pending Claude. No upstream action occurred.


### G07 shared parent-TID page correction — 2026-11-07

On pre-fix local integration `0d23d34d`, binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402`, a disposable normal clone control passed 16/16. A guest peer alternating a dedicated parent-TID page between read-only and writable caused some preflight `EFAULT` denials, then `thread identity copyout failed` after an admitted clone. Launch exited 1/result `supervisor-error`, null code, complete teardown. Exact baseline script SHA-256 `d5b3e0403c8340a3f7cc110ffccc30bfd660420c2b58c94cfc352b0f90a16e60` and `ginux-g07-parenttid-guest-race*` retain fixture, audit and hashes. This is a guest-driven race, separate from the prior synthetic fault injection.

Local integration commit `a0419e24` extends the existing argument guard to supported shared-thread clone forms and adds `U-clone-tid`/`T-clone-tid-race` to Ginux's existing thread runner. An initial corrected-binary fixture exited guest 166 because all race-window clones safely returned preflight `EFAULT` while the fixture required a success in that window; the corrected fixture verifies an explicit successful clone after the peer restores write access. Corrected binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015` passed normal `(16,0,0)` and three shared race vectors `(3,13,0)`, `(1,15,0)`, `(0,16,0)`, all exit 0 and complete teardown. The focused, full thread and full task runners, release build, formatting and strict clippy passed. Fresh Recipe `cargo-consumer` build/install passed in 82.316 s with two acknowledged checkpoints, `installed=true`, result `compiled rust 42\n`, launcher exit 0. Exact diagnostic/suite/Recipe scripts SHA-256 `41e42aadf9097d46d7a1876026c2c69c5d64e8728eabece7dcb9acc570998b6f`, `d6f2eadb7b12b3cb24ac7d0462ca0f388531094e7f0dd0d03c46440fcfac37a3`, `31acf96dfc95e9cccbc8025358e768e621fc338db271968d3c72efaed2cf7a61`; `ginux-g07-parenttid-guarded-*`, `ginux-g07-parenttid-regression-suites*` and `recipe-cargo-parenttid-guard*` hold exact evidence. G07 remains partial: cross-page partial write, unmap/reuse, nonleader and broader indirect arguments remain; full Recipe/F03 and remote CI unverified. Next: cross-page/nonleader TID copyout and indirect-argument review. G02 held; G03c/G12 canonicality failed pending Claude. No upstream action occurred.


### G07 cross-page thread parent-TID fault control — 2026-11-07

On runtime `a0419e24`/binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable thread parent-TID word split across two pages returned `EFAULT` with an inaccessible tail and with a read-only head after tail-first write/rollback. Both attempts left bytes `11 22 33 44` unchanged; the next aligned clone returned virtual TID 2. Exact vector `[-14,-14,2,0x44332211]`, launcher exit 0/`exited/0`, `owned-threads: PASS`, complete teardown. Disposable script SHA-256 `34fd046117efdd7b3dd889ba874f98127451d79bc3d6ee2c06702eaf55d29877`; `ginux-g07-parenttid-crosspage*` retains fixture/audit/result/hashes. Local test commit `5dc0fd8c` adds `B-cross-tid`; focused and full thread runners passed on unchanged binary. Runner wrapper SHA-256 `4c80bfa31709c168461191d2170d0b7dd6d4bb7ba71852cd8857a200ddc5ecfa`, receipt/logs under `ginux-g07-parenttid-crosspage-runner*`. This does not cover post-clone faults, all nonleader paths, concurrent unmap/reuse or broad G07 acceptance. Next: nonleader parent/child-TID birth faults and remaining indirect arguments. G02 held; G03c/G12 canonicality failed pending Claude; full Recipe/F03 open. No upstream action occurred.

### G07 nonleader parent-TID preflight recovery — 2026-11-07

Local Ginux integration test commit `75562753` adds `N-nonleader-tid` to the existing thread fixture and runner. A nonleader thread maps a dedicated TID page, makes it read-only, and attempts a nested supported clone with that parent-TID address. The clone returns `-EFAULT` and leaves the sentinel word `0x44332211` unchanged. After restoring write access, the same nonleader creates a nested thread that receives virtual TID 3; the nested thread observes `gettid=3` and virtual parent TID 0, joins, and both TID words clear. This also checks that the denied attempt consumed no virtual TID. The focused and full thread runners both exited 0 on the unchanged binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`; the full runner passed all selected modes and both teardown cases. Exact wrapper SHA-256 `3cd1afda773a2c54831c100241d3d946b6311ec03c1b534192b2fb752c463a71`; `ginux-g07-nonleader-runner*` in ignored local Daybreak evidence retains the build command, source/fixture/binary hashes, commands, logs and receipt. An initial unprivileged sandbox invocation could not start `sudo` because no-new-privileges was set; it was not a Ginux run or counted as a pass.

This checks one nonleader protected parent-TID preflight and valid retry. It does not independently fault the child-TID pointer, test post-birth child-TID writes, concurrent unmap/reuse, or establish complete G07 acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude; G02 guest clock policy remains held; full Recipe/F03 and remote CI remain unverified. Exact next action: exercise an independently supplied child-TID pointer and remaining configured indirect arguments, then retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 separate protected child-TID pointer — 2026-11-07

A disposable probe on local Ginux integration `75562753` first passed a nested clone with a writable parent-TID word and a distinct read-only child-TID word. Local integration test commit `ebcdaee0` then adds `C-child-tid` and a separate-pointer clone helper to the existing Ginux thread fixture/runner. The nonleader parent uses supported thread flags, supplies `&tid[1]` for the parent-TID write and a read-only mapped sentinel word for the child-TID clear pointer. Clone returns virtual TID 3; the nested child observes `gettid=3`; after its explicit completion signal, the parent-TID slot contains 3 and the protected child-TID sentinel remains `0x44332211`. The guest exits 0 with `owned-threads: PASS`, and the runner asserts supervisor teardown. The focused `C-child-tid`/`N-nonleader-tid` selection and full thread runner both passed with exit 0 on unchanged runtime binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`. Exact integrated wrapper SHA-256 `39a13ceb8715481cb34b5244af265fe532e3cc906a0afa401b7918381a82d213`; `ginux-g07-childtid-integrated-*` in ignored local Daybreak evidence retains build command, source/fixture/binary hashes, commands, logs and receipt. The disposable precursor script, generated fixture and receipt are `ginux-g07-childtid-probe*`.

This verifies one separate protected child-TID pointer and parent-TID publication from a nonleader, with complete launch teardown. The sentinel is checked after the child signals completion, not after a separately observed kernel exit, so a child-exit clear fault and every clone/birth interleaving remain unverified. No `CLONE_CHILD_SETTID` variant or concurrent unmap/reuse was tested. G07 remains partial. G03c/G12 high-bit descriptor canonicality remains failed pending Claude; G02 guest clock policy remains held; full Recipe/F03 and remote CI remain unverified. Exact next action: inspect remaining configured indirect arguments and source-dependent storage behavior after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 indirect-argument review and upstream dependency — 2026-11-07

Read-only review of local integration `ebcdaee0` found the configured argument guard includes pathname calls, supported shared-thread clone, the SIGCHLD action-policy call, and owned-file mapping. `readlinkat` is included in the pathname set; `execve`/`execveat` require sole group ownership before the kernel consumes their vectors. Descriptor-owned `ppoll` copies its timeout and poll array before parking, then retains the computed deadline and captured poll input on wake. This is source review, not fresh mutable-timeout acceptance evidence. Exact next independent case: mutate a shared `ppoll` timeout while the call is parked and verify its original deadline and cleanup with an ordinary configured fixture. Other unreviewed indirect arguments and cross-surface combinations remain open.

GitHub read-only status at this checkpoint: upstream `gmkurtzer/ginux` `main` is `53a54addea67969abaa240ea478def1b94130eee` (merge of #237, contract configuration); its prior integration base `2489517e89c3b62c5203ccd0f6d1eb28cf8500af` is an ancestor, 100 upstream commits behind. Upstream PRs #232, #233 and #234 are all still open and unmerged. Per the ownership split, do not rebase the local integration onto upstream main until the wave-one merges; no upstream push, PR, comment or packet preparation occurred. G07 and G08a remain partial, G03c/G12 descriptor canonicality remains failed pending Claude, G02 clock policy remains held, and full Recipe/F03 verification remains open.

### G08a parked ppoll timeout snapshot under shared mutation — 2026-11-07

A disposable configured shared-thread fixture on clean local integration `ebcdaee0`/binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015` entered `ppoll` on an empty owned pipe with a 300 ms timeout. The host waited for the supervisor audit to report `action=park surface=owned-poll`, then released a guest peer to change the shared timeout to zero and print an acknowledgment. The launch was still running 100 ms after that acknowledgment and returned 307 ms after it, with `ppoll=0`, cleared `revents`, the mutated timeout visible to the guest, `g08a-ppoll-mutable: PASS`, `owned-threads: PASS`, result `exited/0`, and `teardown tasks=0 owned-handles=0`. The first disposable script attempt did not reach compilation because its insertion marker matched an older fixture layout; the corrected script ran once and passed. It is not counted as two runtime passes. Exact corrected script SHA-256 `a633b133800335d2276770939569f3770e85acefd4ad84830385c38829923340`; `ginux-g08a-ppoll-mutable-timeout*` in ignored local Daybreak evidence retains source/fixture/binary hashes, command, timing, audit and result.

This verifies one parked `ppoll` timeout snapshot under a controlled shared-memory mutation. Host scheduling and the single timing sample do not prove every deadline boundary, invalid timeout mutation, signal interruption or concurrent descriptor replacement. G08a remains partial. G03c/G12 descriptor canonicality remains failed pending Claude, G02 guest clock policy remains held, G07 remains partial, and full Recipe/F03 verification remains open. Exact next action: inspect and exercise the next independently sourced indirect-argument or copyout fault on the current integration source; retest storage-dependent behavior after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G29 shared endpoint port transition — 2026-11-07

On clean local integration `ebcdaee0`/binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured guest used a shared IPv4 sockaddr while two host listeners were bound only to `127.0.0.1`. The contract granted one listener port and excluded the other. A stable ungranted connect returned `-EACCES`. A guest peer then held the ungranted port until the owner made another connect, which also returned `-EACCES`; after an explicit shared-memory handoff, the peer changed the same port word to the granted value and the owner connected successfully on the same public socket. Audit records the three connect results `-13,-13,0`; the granted listener accepted once, the ungranted listener accepted no connection, the guest exited 0 with `g29-mutable-endpoint: PASS` and `owned-threads: PASS`, and teardown reported `tasks=0 owned-handles=0`. Exact corrected script SHA-256 `8dac83963a686727be84ee65970b88a00f6e7e166c02ae4bf944a8aab03eb48f`; `ginux-g29-mutable-endpoint*` in ignored local Daybreak evidence retains commands, source/fixture/binary hashes, output, result and audit.

An initial live-toggle run reached an allowed connection without observing a denied call after the peer started, so it was weak concurrency evidence. The next stress fixture exited guest 27 after 100 denials because its script did not set the shared handoff flag; this was a probe error, not a verified Ginux defect. A subsequent attempted edit failed before changing the script, and another run repeated the same result; a later generated-C string escaping error failed compilation before runtime. The original and stress receipts/audits are retained with `-initial` and `-stress-failed` names and are not counted as corrected passes. The corrected run checks a controlled shared-memory transition between calls. It does not prove a mutation during `read_endpoint` itself, every port/family interleaving, in-progress connection state, or broader G28/G29 acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude, G02 guest clock policy held, G07/G08a partial, and full Recipe/F03 open. Exact next action: inspect and exercise protected `getsockname`/`getpeername` output and length copyout on the current source, then retest storage-dependent behavior after wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G29 protected endpoint copyout and partial prefix — 2026-11-07

A disposable configured-network fixture on clean local integration `ebcdaee0`/binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015` connected to one granted `127.0.0.1` listener, then made selected `getpeername`/`getsockname` output calls. A read-only sockaddr destination returned `-EFAULT` without changing its first sentinel byte or length. A read-only length word returned `-EFAULT` after the sockaddr bytes were copied, leaving the length at 16. An inaccessible length word returned `-EFAULT` before address output. A sockaddr split four bytes before an inaccessible page returned `-EFAULT`, left length 16 and wrote only the first four peer-address bytes (`AF_INET` and granted port), with the next sentinel byte unchanged. After restoring access, both endpoint calls returned 0 with 16-byte lengths and the expected family/peer address. The launcher exited 0/`exited/0` with `g29-endpoint-copyout: PASS`, one connection, no extra connection and teardown `tasks=0 owned-handles=0`. Audit records four `-14` endpoint results followed by two zero results. Exact script SHA-256 `4d1aeca2ecb8bf924bb3f484aa69fef2ff98ece5d73e4143725734753144dda2`; `ginux-g29-endpoint-copyout*` in ignored local Daybreak evidence retains source/fixture/binary hashes, command, audit and result.

This establishes selected protected and partial output behavior, including that the address write can precede a failed length update. It does not establish atomic copyout, concurrent mutation of the length/address buffers, IPv6 outputs, every boundary alignment or all G29 acceptance. G03c/G12 descriptor canonicality remains failed pending Claude, G02 guest clock policy held, G07/G08a partial, and full Recipe/F03 open. Exact next action: inspect shared-thread mutation of the endpoint output length and address during copyout, then continue independent source-specific checks and storage-dependent retests after wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G29 shared endpoint output protection race — 2026-11-07

On clean local integration `ebcdaee0`/binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured guest connected to one granted `127.0.0.1` peer and spawned a shared thread that alternated the guest sockaddr output page between readable and writable and the length page between inaccessible and writable. The owner repeatedly called `getpeername` on the same public socket, accepting only zero or `-EFAULT`; after stopping and joining the peer it restored both pages, made a valid query, and exited 0 with `g29-mutable-copyout: PASS` and `owned-threads: PASS`. The first 1,024-call run observed 255 zero results and 769 `EFAULT` results, no extra connection, and `teardown tasks=0 owned-handles=0`; its supervisor audit text reached the 65,536-byte budget and was truncated, so it does not provide per-call audit for the entire run. A separate 64-call variant observed 16 zero results and 48 `EFAULT` results with the same recovery and teardown; its untruncated audit records exactly 65 `getpeername` entries (64 loop calls plus recovery), including 17 zero and 48 `-14` results. Exact long and short scripts SHA-256 `97f194c321d0969bb40e4aad7c314eebfae2791b1d6c7e9e4f350d89decbcd71` and `da63223c52255766d5bca38b5fc82ba0f718cec8e892fe0e3b91d66f7ae61567`; `ginux-g29-mutable-endpoint-copyout-{long,short}*` in ignored local Daybreak evidence retains source/fixture/binary hashes, commands, counters, audits and results.

This verifies selected concurrent guest protection changes during endpoint output and a valid query afterward. It does not establish exact copyout atomicity, every interleaving, content mutation of the output bytes, IPv6 behavior or complete G28/G29 acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude; G02 guest clock policy remains held; G07/G08a and full Recipe/F03 remain partial or unverified. Upstream wave-one PRs #232–#234 were read-only checked this turn and remain open, so no integration rebase occurred. Exact next action: inspect G28 Linux adapter host descriptor close/failure lifetime on the current source and choose a reproducible ordinary kernel-backed control; retest storage-dependent integration after the upstream merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G28 kernel-backed host socket descriptor lifetime — 2026-11-07

On clean local Ginux integration `ebcdaee0`, a freshly built `ginux-devices` host library (SHA-256 `473174d0f596d5396198bc8d2bc331e8df1385f53f9996303616b623406ae1d2`) was linked into a disposable loopback control. For each of explicit close, repeated revoke, and drop, the control observed exactly one new private `socket:[inode]` entry in `/proc/self/fd`, verified `FD_CLOEXEC` and `O_NONBLOCK` with kernel `fcntl`, accepted the connection, then observed peer EOF and a return to the baseline socket-descriptor inventory. The corrected control exited 0 with `g28-kernel-fd-lifetime: PASS`; exact source/binary SHA-256 values are `7fabed684b20a3521d3d159091bc31fb460178feda50b638094449348f510fc9` and `d4c7432d6ff783d1fc8a21dbb6368a2bd2968552f09374d9e211a022e4641e76`. The ignored local evidence directory retains `ginux-g28-kernel-fd-lifetime.{rs,bin,stdout,stderr,json}` with commands, source/lockfile/library hashes and the run receipt. An initial sandbox invocation could not bind loopback; the first unsandboxed fixture run included the transient `/proc/self/fd` directory descriptor in its inventory and failed its own assertion. The corrected socket-only inventory ran twice and passed; only the retained corrected run is counted.

This establishes normal kernel descriptor release and flags for the selected three completion paths. It does not induce a real kernel close error: the earlier synthetic `strace` error skipped the actual syscall and cannot prove error-time lifetime. It also does not cover concurrent in-progress cancellation, guest descriptor-table exposure or every adapter fault. G28 remains partial; G03c/G12 descriptor canonicality remains failed pending Claude, G02 clock policy remains held, and G07/G08a and full Recipe/F03 remain partial or unverified. Exact next action: inspect the configured launch path for private host socket descriptor exposure and an ordinary in-progress connect cancellation control; leave real close-error lifetime unverified unless a kernel-backed mechanism is found. Retest storage-dependent integration after wave-one upstream merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G28 real kernel refusal releases host socket — 2026-11-07

The same current-source `ginux-devices` library was linked into an expanded disposable loopback control. After the three normal close/revoke/drop cases, it released a bound ephemeral port, opened a granted nonblocking socket to that now-unserved address, observed the private host socket in `/proc/self/fd`, waited for readiness error, and read real kernel `SO_ERROR=111` (`ECONNREFUSED`). Explicit close then restored the socket inventory to its pre-connect baseline. The corrected control exited 0 with `real-refusal=111 closed` and `g28-kernel-fd-lifetime: PASS`. Exact expanded source/binary SHA-256 values are `35308beba7f9be50ad9e501adf7790249d84d678238268f64afb4b665378f5ad` and `1512e1b37c05d298f294ea75d5fe4bd70ce047a1c1f593e9205f327741a53bfc`; `ginux-g28-kernel-refusal-lifetime.{rs,bin,stdout,stderr,json}` retains the run and hashes in ignored local Daybreak evidence. The released port is not reserved against a third party binding it, although the observed kernel refusal confirms this run's outcome.

This establishes descriptor cleanup after one real asynchronous kernel connect failure, complementing the earlier synthetic immediate-connect error and normal lifetime controls. It does not induce a kernel `close` error, prove concurrent in-progress close/revoke, or establish guest FD confinement; G28 remains partial. G03c/G12 descriptor canonicality remains failed pending Claude, G02 clock policy remains held, and full Recipe/F03 remains open. Exact next action: inspect configured guest FD confinement and concurrent in-progress cancellation against the current source; preserve the real close-error limitation unless a kernel-backed control is available. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G28 configured guest cannot fstat private host socket — 2026-11-07

On clean local Ginux integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured guest connected to one granted `127.0.0.1` listener. Its virtual public socket was fd 3 and `fstat(3)` reported socket mode. It then called `fstat` on every other candidate fd 3–31: all 29 returned `-EBADF`, including host fd 5. A supervisor-only `strace` recorded `socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK, IPPROTO_TCP)=5`, `connect(5)=-EINPROGRESS`, and later `close(5)=0`; the guest audit recorded the virtual socket close, `teardown tasks=0 owned-handles=0`, launcher exit 0/`exited/0`, and `g28-private-fd: PASS`. The single listener saw EOF after guest close and no extra connection. Exact probe/fixture-source/fixture-binary SHA-256 values are `ec1835bab6275b902d2baaae1937bc62964be90686e52c6b8c548f322dbf1544`, `ac69065399957c870be54ee4d77df901a651d1df1133f25131191b8b7f5c3d42` and `95d67e7b2615c3a08503abf3314de7a84ac45f709f4a2d5b67ec754a09c0375b`; `ginux-g28-private-fd-*` in ignored local Daybreak evidence retains the corrected script, fixture, trace, audit and receipt. An unprivileged invocation failed bootstrap `chroot` before guest execution, and the first privileged fixture exited 173 because it incorrectly treated public virtual fd 3 as a forbidden low socket. Its failed receipt/fixture are preserved with `-initial` and are not counted as passes.

This verifies one configured descriptor-table probe over fd numbers 3–31 and the observed private host fd 5. The supervisor-only trace does not prove every possible guest-native access, alias, pidfd route, fork/exec overlap or concurrent in-progress cancellation. G28 remains partial, as does real kernel close-error lifetime. G03c/G12 descriptor canonicality remains failed pending Claude; G02 clock policy held; full Recipe/F03 unverified. Exact next action: inspect and exercise concurrent in-progress connect cancellation under an ordinary configured guest schedule, then continue remaining Daybreak criteria and storage-dependent retests after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G28 configured nonblocking connect closed before guest completion — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable guest opened one nonblocking public TCP socket to a granted loopback listener. Guest `connect` returned `-EINPROGRESS`; without a poll or `SO_ERROR` completion query, it closed the public descriptor and exited 0 with `g28-pending-close: PASS`. Supervisor-only `strace` recorded one `socket(...SOCK_CLOEXEC|SOCK_NONBLOCK)=5`, `connect(5)=-EINPROGRESS`, and `close(5)=0`. The peer accepted once, observed EOF and no extra connection; result was `exited/0` and audit teardown was `tasks=0 owned-handles=0`. Exact probe/fixture-source/fixture-binary SHA-256 values are `91d42e6dff37f0e8a8b9b77ab7d34800698c8a344f83c9c3b386b321655c82c0`, `5d025b93a02bf3de8c4ba612cb52a403fbe733e2c3f17733ff9d380a150d5917` and `63c8661ce44beaa7de7e1197a34e942ed1cc6d2c338519f18b7ce0907aa215d9`; `ginux-g28-pending-close-*` in ignored local Daybreak evidence retains script, fixture, trace, audit and receipt.

This establishes cleanup while the *guest's* connection state is pending. The kernel handshake may already have completed before close; this is not proof of every in-progress kernel cancellation, concurrent revoke/close, or a real kernel close-error lifetime. G28 remains partial. G03c/G12 descriptor canonicality remains failed pending Claude; G02 clock policy held; G07/G08a and full Recipe/F03 remain partial or unverified. Exact next action: review remaining Daybreak criteria against this current source, choose an independent high-information case, and retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 configured child-TID exit path source review — 2026-11-07

Read-only review of clean local integration `ebcdaee0` found that configured `launch_tasks.rs` accepts thread clone only with flag words `0x3d0f00` and `0x7d0f00`; other configured clone shapes return `-ENOSYS`. Its `Pending::Fork` records a child-TID birth pointer only for the separate `libc_fork` shape, while the thread path mediates the parent-TID publication and removes that one parent-write bit before native clone. The supported thread flag words retain `CLONE_CHILD_CLEARTID` for the kernel exit path. This source review does not verify exit-time behavior. The existing `C-child-tid` fixture checks a separate read-only child pointer after the nested child signals completion, but before a separately observed native exit; its unchanged sentinel is therefore not evidence of a failed kernel exit clear.

Exact next action: build a bounded configured child-clear control with a protected separate TID word, synchronize on observed child exit before checking the sentinel and teardown, and compare a writable control; preserve the distinction between a kernel clear fault and supervisor birth copyout. G07 remains partial. G28 remains partial for real close-error lifetime and concurrent kernel cancellation; G03c/G12 descriptor canonicality remains failed pending Claude, G02 clock policy held, full Recipe/F03 unverified. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 protected child-TID clear after observed exit — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured thread fixture used supported thread flags and a separate child-clear TID word. A nested child with a read-only child-TID page exited; only after the audit reported `task exited virtual-pid=3` and the parent observed `kill(3,0)=-ESRCH` did the fixture check that the sentinel remained `0x44332211`. It then restored write access and launched a second nested child using the same word. After audit `task exited virtual-pid=4` and `kill(4,0)=-ESRCH`, the writable word was zero. The outer thread joined, launcher exited 0/`exited/0` with `owned-threads: PASS`, and teardown reported `tasks=0 owned-handles=0`. Exact probe/fixture-source/fixture-binary SHA-256 values are `87200e8ab8e62715b11ded9681a0b283bd54e3916091921b273a87f11f325335`, `38dda3dcf4b9bbb6e39529565a51289662ff331a1e0343cfb56dabf76a3a327a` and `3aa814261b1a3c49fe55f663bf350aaf27063274fd1e986eaf809f2edd17c9b3`; `ginux-g07-child-clear-exit-*` in ignored local Daybreak evidence retains exact script, fixture, audit and receipt.

This verifies one protected child-clear failure and one writable success after separately observed child exits. It does not cover inaccessible or cross-page child-clear words, every futex wake interleaving, concurrent unmap/reuse, or broad G07 acceptance. G28 real close-error lifetime and concurrent kernel cancellation remain unverified; G03c/G12 descriptor canonicality failed pending Claude; G02 clock policy held; full Recipe/F03 open. Exact next action: exercise a child-clear word straddling a page boundary with its tail inaccessible, then a fully writable control, checking post-exit bytes and teardown. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 cross-page child-TID clear after observed exit — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable nested thread placed its four-byte child-clear word two bytes before a page boundary. It made the tail page inaccessible, created a child with that pointer, and waited until the audit reported `task exited virtual-pid=3` and `kill(3,0)=-ESRCH`. The two accessible head bytes remained `11 22`; after restoring access, the tail bytes remained `33 44`. A second child with the same four-byte word fully writable exited as virtual ID 4; after `kill(4,0)=-ESRCH`, all four bytes were zero. The outer thread joined; launcher exit/result was 0/`exited/0`, stdout was `owned-threads: PASS`, and teardown reported `tasks=0 owned-handles=0`. Exact probe/fixture-source/fixture-binary SHA-256 values are `c7eb92b1c17d5d7eb7a4c1ee3020eb67c81ad498e9c753bac5d36382761031dc`, `92b0a0db759421f731e80a35d895272b30ff43644e81380c43469a77a7b834b1` and `d394f7142ef5bee1447ff1dae08acf3727120af38755d42d6156b5090011a99a`; `ginux-g07-child-clear-crosspage-*` in ignored local Daybreak evidence retains script, fixture, audit and receipt.

This establishes no visible partial prefix for one two/two split of a kernel child-clear word, with a writable control. Other alignments, concurrent unmap/reuse and futex wake timing remain unverified; G07 stays partial. G28 real close-error lifetime and concurrent kernel cancellation remain unverified; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held, and full Recipe/F03 remains open. Exact next action: inspect G07 child-clear behavior when a sibling changes the mapped page during exit and choose a controlled ordinary schedule; retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G07 child-clear pointer unmap and same-address reuse — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable nested-thread fixture held each child at a shared exit gate. For child virtual ID 3, the parent unmapped its child-clear page before releasing exit; the child exited, `kill(3,0)` reached `-ESRCH`, and the instance stayed healthy. For child ID 4, the parent unmapped the clear-word page and used `MAP_FIXED` to install a new writable anonymous page at the same address, seeded it with `0x55667788`, then released exit. After the child exited and `kill(4,0)=-ESRCH`, the fresh word was zero. Audit order records the two unmaps, the replacement mapping, both child exits, and `teardown tasks=0 owned-handles=0`; launcher exit/result was 0/`exited/0` with `owned-threads: PASS`. Exact probe/fixture-source/fixture-binary SHA-256 values are `3a1777b99d313586eb71e5fd86bf1df78821011886524a3d7968f3f808a6dad2`, `ced33c6b610949f6695cc5b66d460f9a0b312e01a3119ee908f382d174c0203b` and `ee95396b0698747d28778cd9069502d4e89df28398bbd8acb7f20927f983db1a`; `ginux-g07-child-clear-remap-*` in ignored local Daybreak evidence retains script, fixture, audit and receipt.

These are controlled ordered transitions completed before each child is released, not proof of every simultaneous unmap/exit interleaving. The new page was guest-created; its kernel clear is address-at-exit behavior, not evidence of a supervisor authority bypass. G07 remains partial, as do G28 close-error lifetime and concurrent kernel cancellation. G03c/G12 descriptor canonicality failed pending Claude; G02 clock policy held; full Recipe/F03 open. Exact next action: inspect current G06b wait-status copyout and rollback evidence, then run a fresh controlled wait fault and consumed-child recovery case if current source allows it; retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G06b current wait faults consume children and allow recovery — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable extension of Ginux's existing `qfault` task fixture retained its four protected, unmapped and cross-page wait-status cases. Each faulted `wait4` returned `-EFAULT`, the next wait returned `-ECHILD`, and the fixture's read-only/adjacent sentinel checks passed. It then forked a fifth child, waited into a valid status word, checked status `43<<8`, and confirmed a second wait returned `-ECHILD`. The existing `scripts/test-launch-tasks.py` `qfault` selector exited 0 with `PASS launch tasks: qfault`; a direct configured launch also exited 0/`exited/0` with `owned-tasks: PASS` and `teardown tasks=0 owned-handles=0`. Its audit records four `-14` mediated wait results, five `-10` consumed-child results, and a successful wait result for virtual child 6. Exact selector/direct script SHA-256 values are `2e796b36bfc84db8f98dc04d2275915b4b5bacbc607e5f5da4c67dd8ef81a5fd` and `fded6b1de1bef57b69672dac5e53d876df3fb84ff8b7088d7bedc74f27393566`; fixture source/binary values are `d74f77d226d61892855de0f60610dc4ef8f54e00cf228f559ba9bcb86cc32924` and `a6696f2333833d153eb298273670a233a8ec5c50f41512fe0b6558ba7894a55f`. `ginux-g06b-wait-recovery*` in ignored local Daybreak evidence retains scripts, fixture, selector log, direct audit and receipts. The first direct wrapper incorrectly expected ten wait entry records, omitting five initial park records; its launch passed but its wrapper exited 1. That receipt/audit are retained with `-initial` and are not counted as a verified wrapper pass.

This refreshes the selected wait fault and post-fault recovery behavior on the current source. It does not establish shared-thread mutation of a writable status pointer during copyout, every partial-write rollback, or all G06b authority criteria. G07 remains partial; G28 real close-error lifetime and concurrent kernel cancellation remain unverified; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held, and full Recipe/F03 open. Exact next action: inspect G06b shared-thread wait-status copyout under page-protection mutation and choose a bounded ordinary control with a valid recovery wait; retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G06b shared wait-status page protection during copyout — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured guest kept a sibling thread alternating one wait-status page between read-only and writable while the leader serially forked and waited for 32 children. Each wait returned either its child ID with the complete virtual exit status `42<<8` or `-EFAULT`; after every result, a second wait returned `-ECHILD`. The fixture required both outcomes, joined the sibling, restored write access, then reaped a new child with status `43<<8`. It exited 0/`exited/0` with `owned-threads: PASS` and `teardown tasks=0 owned-handles=0`. The untruncated 46,459-byte audit records 33 mediated waits (26 successful loop copyouts, six loop `-14` results, one successful recovery), 33 `-10` consumed-child results and 120 sibling `mprotect` entries. Exact probe/fixture-source/fixture-binary SHA-256 values are `ff65b292a2b314cb287e6e524bb39e1122d67937e5751b483e2299460f881b28`, `3734c19061f5b1332ca6f7a00ccfa14c3ff1e4dc7ea15e8f6ef396358da6f480` and `7f6049ed632f391711232216b2dd8961a965b6e9151a99b6c215605bf7fb433a`; `ginux-g06b-shared-wait-*` in ignored local Daybreak evidence retains script, fixture, audit and receipt.

This checks one same-page shared protection schedule with both outcomes and child consumption. It does not establish exact output bytes after a concurrent fault, every interleaving, or a cross-page rollback while protection changes; the separate stable `qfault` control checks sentinel rollback without a sibling. G06b remains partial. G07 remains partial; G28 real close-error lifetime and concurrent kernel cancellation unverified; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held, full Recipe/F03 open. Exact next action: inspect `store_u32` cross-page rollback under a sibling changing only the tail page protection, then run a bounded configured wait-status control with a valid recovery child. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G06b cross-page wait-status rollback under shared tail protection — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable configured guest placed a four-byte wait-status word two bytes before a page boundary. The head page remained read-only; a sibling alternated the tail page between read-only and writable while the leader serially forked and waited for 32 children. Every wait returned `-EFAULT`, every follow-up returned `-ECHILD`, and the fixture checked all four `0x5a` sentinel bytes after each fault. It joined the sibling, restored write access and reaped a fresh child with valid status `43<<8`. Launcher exit/result was 0/`exited/0`, stdout `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. The untruncated 43,665-byte audit records 32 mediated `-14` results, 33 `-10` consumed-child results (including recovery), one successful recovery wait and 101 sibling `mprotect` entries. Exact probe/fixture-source/fixture-binary SHA-256 values are `d5c38ee51de4ac4e879459e107bbf413079a83e472cdae0cd0e85c8ebece49cc`, `957e79ebb8646f72461b7e4afea25503383e762cf71b458edea817185a2b32b7` and `768daf4f3c06c12b4a55c2eaef0928b027f42ecf99dc240dc8614bc1e5d74a5f`; `ginux-g06b-shared-wait-crosspage-*` in ignored local Daybreak evidence retains script, fixture, audit and receipt.

This is one bounded shared-protection schedule. It does not establish that every tail-write/head-fault/rollback interleaving is safe; the audit does not expose intermediate copy operations. G06b remains partial. G07 remains partial; G28 real close-error lifetime and concurrent kernel cancellation unverified; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held, full Recipe/F03 open. Exact next action: trace the supervisor's first split wait-status write sequence on this fixture and assess whether a controlled delay after the tail write can exercise rollback while the sibling changes protection; do not count a synthetic error as a real kernel fault. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### G06b split wait-status copy sequence and bounded timing control — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a supervisor-only `strace -e process_vm_writev` on a two-child variant of the split wait fixture captured two real copyout paths: one two-byte tail write returned kernel `EFAULT`; another tail write returned 2, the read-only head write returned `EFAULT`, and a two-byte tail rollback returned 2. Both waits returned guest `-EFAULT`, consumed their children, passed the sentinel checks and recovered with a valid child. A separate 100 ms `delay_exit` trace did not inject errors and also passed, but both tail writes faulted before rollback was needed. Two further timing variants reached tail-write/head-fault/rollback success while a guest sibling was present; their guest and trace checks passed, but their audits truncated at the 65,536-byte budget because of guest polling, so they are not complete audit evidence.

The final bounded timing variant removed that polling. Its 5,321-byte untruncated audit shows the mediated wait and `-EFAULT` at sequence 10, followed only afterward by the sibling's two `mprotect` calls at sequences 13–14. Its complete host trace shows `process_vm_writev` tail success 2, head `EFAULT`, and rollback success 2 with a 100 ms post-tail-write delay. The fixture checked all four sentinel bytes, a consumed child, valid recovery wait and complete `tasks=0 owned-handles=0` teardown; launcher exit/result was 0/`exited/0` with `owned-threads: PASS`. Exact final probe/fixture-source/fixture-binary/trace SHA-256 values are `63f4c147024fea8b1426a3a38307243b3b65825d489b25ccb6b3ed12c464599b`, `60262a4601c7f815941f0a9be686ebbdea4ef2b3a0a9c0ed4dfc5502433d20dc`, `b3217cedf4087b95d13451851127092f03a0882d79d5fe7c5993a66692421523` and `2c12075ce4cb94a630113376d2d9b315cc5ad04f44fbedd1c11fff37f3d67cd1`. `ginux-g06b-{shared-wait-crosspage-{trace,delay},wait-rollback-{hold,shortspin,nokill}}*` in ignored local Daybreak evidence retains scripts, fixtures, audits, traces and receipts.

These selected traces show actual tail/head/rollback kernel results and that the sibling's protection calls were serialized after this wait; they do not prove every output-memory mutation or external kernel interleaving safe. The delayed runs are timing diagnostics, not synthetic kernel-error evidence. G06b remains partial; G07 remains partial; G28 real close-error lifetime and concurrent kernel cancellation unverified; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held, full Recipe/F03 open. Exact next action: review the G03c–G06b assignment and exec authority criteria against the current local source, select one independent control unaffected by Claude's storage redesign, and run it with a valid allowed-path comparison. Retest storage-dependent integration after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### Current G03c–G06b device interpreter boundary — 2026-11-07

On clean local Ginux integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable image used a script with `#!/dev/null` and an ELF with `PT_INTERP=/dev/null`. The device contract assigned `null`; storage assignments were empty. From a valid `/bin/fixture` start, both `execve` calls returned `-EACCES` (`-13`) at audit sequences 5 and 6. The same guest forked and executed an image script with `#!/bin/second`; its child exited 77, the parent reaped it, and the guest printed `device-interpreter: PASS`. Launcher exit/result was 0/`exited/0` and the untruncated 4,620-byte audit ended with `teardown tasks=0 owned-handles=0`. This checks both kernel-selected formats against a device endpoint with a valid image-interpreter comparison and does not depend on the configured-storage redesign.

Exact disposable runner/source/fixture/device-ELF SHA-256 values are `6166cf00f4f5930476206d5433809294362b4df4427ea5402a98eeda4269dbde`, `7ad0647b1144742a444d0740a7ff9b95a854b3d662ce0ae308166402e5fa0b84`, `3fd317783765923381a1572a1d978a0f8dd5f4695de4516a6cd5714a758e579a` and `22e34d66a136a6224f6b2664030542492865c68fe83635b77ed8ceaf54f60516`. The ignored local `ginux-g03c-device-interpreter/` evidence directory retains runner, generated fixture and ELF, instance, stdout, full audit and JSON receipt. An initial launch without sudo failed at chroot before guest execution; a subsequent user-process wrapper could not read the root-owned result file. Neither is counted as a Ginux failure or pass; the corrected root-run is the verified control.

This is one device-interpreter authority control. It does not cover implicit interpreter path races, symlink aliases, image FD `execveat` on this exact revision, all native/image/owned chain combinations or full G03c–G06b security acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude's redesign; G02 clock policy held; G06b/G07/G28 and full Recipe/F03 remain incomplete or unverified. Exact next action: inspect the current execveat image-FD rejection path against a valid allowed execveat path and select a narrow control, then retest storage-dependent behavior after upstream wave-one merges. No Ginux source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### Current G03c–G06b image-FD interpreter boundary — 2026-11-07

On the same clean local integration `ebcdaee0` and configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a second disposable guest opened three image executables and used `execveat(fd, "", argv, envp, AT_EMPTY_PATH)`. The script with `#!/dev/null` and ELF with `PT_INTERP=/dev/null` each returned `-EACCES` (`-13`) without executing; the audit records those syscall 322 denials at sequences 6 and 9. An allowed image ELF with `PT_INTERP=/bin/second` then executed through the same descriptor form in a forked child and exited 77; the parent reaped it and printed `device-interpreter-fd: PASS`. Launcher exit/result was 0/`exited/0`; the untruncated 5,622-byte audit ended with `teardown tasks=0 owned-handles=0`. Storage assignments were empty, so this check is independent of Claude's configured-storage redesign.

Exact disposable runner/source/fixture/device-ELF/allowed-ELF SHA-256 values are `792313530d29e5b4a9a17f56af6f841b577603829b20622d22875a1703d31fc7`, `7f4530f44ccd643cdcd70074a0c4fa633b28fd074ed8e6fe5c5e9465c55224c4`, `d2519311daa73ff8608aa1c90bafa0c0637e67d319026ce7e7ceb85f91f45138`, `22e34d66a136a6224f6b2664030542492865c68fe83635b77ed8ceaf54f60516` and `b65b10dfe81d4e682b5734845ae3b523078abaf77e3eb4054b7b855f0983e763`. The ignored local `ginux-g03c-device-interpreter-fd/` evidence directory retains runner, fixture, images, instance, stdout, full audit and JSON receipt. The first generated C variant had an invalid escaped newline and failed compilation before launch; the corrected variant compiled and ran once. The compile failure is not a runtime pass.

This checks selected image FD interpreter authority and a valid control, not mutable pathname/argv races, symlink chains, all descriptor forms, native/owned routing or complete G03c–G06b acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude; G02 clock policy held; G06b/G07/G28 and full Recipe/F03 incomplete or unverified. Exact next action: review remaining current-source exec authority gaps, including image/device interpreter aliases and native/owned routing, then choose a bounded independent control; retest storage-dependent behavior after upstream wave-one merges. No Ginux source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### Current G03c–G06b image interpreter symlink authority — 2026-11-07

On clean local integration `ebcdaee0` and unchanged configured runtime SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a disposable image made `/alias-null -> /dev/null` and `/alias-second -> /bin/second`, with the `null` device assigned and no storage assignments. A script with `#!/alias-null` and an ELF with `PT_INTERP=/alias-null` each returned `-EACCES` (`-13`) from `execve`; the audit records those denials at sequences 5 and 6. A script with `#!/alias-second` then executed in a forked child; its exit status was 77 and the parent reaped it. Launcher exit/result was 0/`exited/0`, stdout `device-interpreter-alias: PASS`, and the untruncated 4,620-byte audit ended with `teardown tasks=0 owned-handles=0`. This checks one image-symlink interpreter selection boundary without depending on configured storage.

Exact disposable runner/source/fixture/device-ELF SHA-256 values are `b956c4f1837c48bc78c99b60a209d1b568436a5a9b0f100d0e7114b475cfd672`, `a4289759468ae3a966d936e0db0deae7a604ce39c3d5760e665d1dbf9890f1a5`, `ab6faa48b5e836adf0b24789605c9147de176da24b8e4bb4d733c65c47594108` and `9ad5dd77f6b526b594bfa0677ea8c360e0819aafe13966603284852f86186ef6`. The ignored local `ginux-g03c-device-interpreter-alias/` evidence directory retains runner, generated fixture and ELF, instance, stdout, full audit and JSON receipt. The corrected fixture compiled and ran once.

This selected alias result does not establish assignment symlink confinement, hostile interpreter replacement races, all image/owned/native chains or complete G03c–G06b acceptance. G03c/G12 high-bit descriptor canonicality remains failed pending Claude; G02 clock policy held; G06b/G07/G28 and full Recipe/F03 incomplete or unverified. Exact next action: review remaining current-source native/owned interpreter and pathname mutation authority, then choose a bounded independent control; retest storage-dependent behavior after upstream wave-one merges. No Ginux source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

### Current merged-upstream exec and thread authority baseline — 2026-11-07

Local Ginux integration `158e11a6194bfb14aea3d5128afdc07306179e6a` includes upstream `main` `53a54addea67969abaa240ea478def1b94130eee` through merge `88616a19`. The separate Recipe e2e process was actively using `/tmp/claude-1000/-home-vince/b0f2fe79-6cce-4d00-bb21-113f3c1812d0/scratchpad/ginux-158e11a6` (SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`) with Recipe `2ad833c` and six test threads when this independent validation ran. Its eventual e2e result is not claimed here.

The unchanged full `scripts/test-launch-exec.py` runner passed all five initial interpreter ownership checks and eight exec scenarios, including shared pathname selection, on that exact Ginux binary. The unchanged full `scripts/test-launch-scripts.py` runner passed all 36 owned-script modes, including owned/image interpreter chains, descriptor and directory forms, and relative/cross-namespace forms. Both exited zero in independently disposable roots. Exact commands, input/log hashes, full output and measured durations (0.309 s and 0.867 s) are retained as `ginux-current-exec-suites-158e11a6/` in ignored local Daybreak evidence. The exact binary was copied there before the external scratchpad expires.

Three earlier disposable device-interpreter controls were rerun on the same current binary: direct `execve` shebang/PT_INTERP selections, image-FD `execveat(AT_EMPTY_PATH)` selections, and image-symlink interpreter aliases. Each returned the expected `-EACCES` for `/dev/null` or its alias, executed its allowed image interpreter comparison, exited zero, and showed complete teardown in an untruncated audit. Source/fixture/binary hashes, full audits and receipts are in `ginux-current-interpreter-158e11a6/` under the same ignored evidence root. These are selected current-revision authority controls, not complete native/image/owned path-race acceptance.

An initial full thread-runner attempt used the retained `.artifacts/launch/launch-thread-fixture` binary (SHA-256 `f6fc8ea30b8ad9aca305bb629bea998591aeaf1aa1f5f6f95b8c5ff6e00ceb5a`), which lacked current `Z-safe` fixture behavior and exited at that case with guest 99/runner 1 after six passes. The current fixture source (SHA-256 `77a42967838e6f4853e7749446b2d044687802501536cab025fd60d2592f07c1`) was rebuilt separately using the existing Make flags into a fresh binary (SHA-256 `a949b7b2b84761f74a368deb48887a23f6c14d58389ea8c3b6eb91d8484cf66d`). `Z-safe` then passed, and the unchanged full `scripts/test-launch-threads.py` runner passed every named mode plus both group teardown cases in 8.007 s on the same Ginux binary. `ginux-current-thread-suite-158e11a6/` preserves the stale failure and corrected full-run logs, exact binaries, build command and receipts. The stale attempt is not counted as a passing full suite or a runtime defect.

G03c/G12 high-bit descriptor canonicality remains failed pending Claude's storage redesign; G02 guest clock policy is held; G06b/G07/G28 security acceptance and full Recipe/F03 remain incomplete or unverified. Upstream wave-one PRs #232–#234 were still open at the read-only check, so no rebase was performed. Exact next action: inspect current native/owned interpreter and shared pathname mutation paths for one remaining independent authority case, then retest storage-dependent behavior after upstream wave-one merges and incorporate the e2e result when it actually finishes. No Ginux source change, push, PR, upstream comment, packet preparation or agent dispatch occurred.

### Current merged-upstream G12 high-bit descriptor failure — 2026-11-07

On clean local Ginux integration `158e11a6194bfb14aea3d5128afdc07306179e6a` (including upstream `main` `53a54add`), the exact configured runtime used by the concurrent Recipe e2e run (SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`) accepted a read through `fd | (1UL << 32)` after a valid read and rewind on the same immutable-image file. The disposable guest returned its failure sentinel 99; the launcher result was `exited/99`, and the complete 2,564-byte audit shows both reads returning one byte and `teardown tasks=0 owned-handles=0`. This confirms the G03c/G12 descriptor-canonicality criterion is still **failed** on the current merged revision. The fixture used no storage assignment, so this observation is independent of the configured-storage redesign. `DescriptorTable::get(u64)` still narrows the public descriptor to `i32`.

The ignored local `ginux-current-g12-highfd-158e11a6/` evidence directory retains the runner, source, binary, config, full audit, result and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `ad3acfaee5c64f6f93a15baf3cd1f51850832098f53e648427bc3f72a1bda298`, `c4ddcab6a514e89235102118c05acda5398505d4a37b22647615782f902dbf55`, `5b0bf6d69eaa29983012d1efad8da9de5f1d5ef2dbd64ad9c8e569283c4082dd` and `fffb8a3c59379b3d4d3663c431894daa2b6273b3cdddc08e312e9c5c87dbc3c8`. The first fixture compilation failed under `-Werror` before launch because an assembly-referenced function lacked `used`; the corrected fixture produced this single verified runtime run. This is a selected descriptor read case, not exhaustive descriptor or storage acceptance.

The Recipe e2e run was still active when checked, so no suite result is claimed. Claude owns upstream storage work; no local Ginux fix, upstream push, PR, comment or packet preparation occurred. G02 clock policy remains held. Exact next action: continue independent exec/authority review while e2e runs, then incorporate its actual result and retest storage after wave-one upstream merges.

### Current merged-upstream storage runner — 2026-11-07

On clean local Ginux integration `158e11a6` with exact configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, a freshly built fixture from the current source ran the unchanged full `scripts/test-launch-storage.py` runner in independent disposable roots while the separate Recipe e2e suite continued. The runner exited 0 in 7.96 s and printed `test-launch-storage: PASS (isolation, data, metadata, copy protections, bounds, fresh launch)`. It includes its default image/assignment, mapping, symlink, quota, export, ownership and invalid-configuration assertions. Exact runner/fixture-source/fixture-binary/template/log SHA-256 values are `66d6e8be72cc997983f0bf1f58bc9df51c34757923935baee3f80443bdfdc129`, `f5a6df0ae9624905f081caba71fb26081f8e8e057fd2c142d10f386251f6dac6`, `24bac21c09d8b8dc5cf8461a9f14b89d8aacabd589127a89c74381b713050cbc`, `8ee42a99a93eb5fd92f009dd9e87d850bb42a3042be9f1672cd92a6d687d0da9` and `3d7941d3780630b598af8168679bb42375958ca3b10fa9202d4c27b0bb40f1ce`. The ignored local `ginux-current-storage-suite-158e11a6/` directory retains exact inputs, log and receipt.

This is ordinary current-revision storage regression evidence. It does not reverse the separately reproduced G03c/G12 high-bit descriptor failure, prove the known symlink-above-assignment defect fixed, establish exhaustive security acceptance, or provide a Recipe e2e result. G02 clock policy stays held and Claude owns upstream storage redesign. Exact next action: inspect the current exec and descriptor authority edges not covered by the full runners, then incorporate the independently running e2e result when available; retest changed storage after wave-one upstream merges. No runtime source change, push, PR, comment or packet preparation occurred.

### Current merged-upstream configured TCP runner — 2026-11-07

On clean local integration `158e11a6` and the same exact configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, a fresh fixture from the current source passed the unchanged full `scripts/test-launch-network.py` runner. It exited 0 and printed `PASS` for all 11 named modes: `io`, `nonblock`, `deny`, `quota`, `timeout`, `hold`, `hold-kill`, `slots`, `off`, `ordinary-hold` and `budget`. Each mode used a disposable root and loopback endpoints. Exact runner/fixture-source/fixture-binary/log SHA-256 values are `5aa98e2a8380c576a94d370a15499d18e116b1292d0853788564ffb01b982f25`, `0a7164319f6806ae98cd8592ee0669dffde1bf58b349e65ef79f2b74a89134ca`, `65d34e71ada221bf57dd82a7540aa3e59a96f47cfe72a1b0a1310689f6b5540d` and `52506ebb75511a83a0d6515870b540798053f599930d7ba317c3c96b1e260870`. The ignored local `ginux-current-network-suite-158e11a6/` directory retains exact inputs, full output and receipt.

This is ordinary current-revision TCP regression evidence, not exhaustive G28/G29 security acceptance or proof of kernel close-error lifetime and concurrent cancellation. G03c/G12 descriptor canonicality remains failed, G02 clock policy held, and Recipe e2e was still running at the last direct check. Exact next action: review remaining current-source exec/descriptor authority gaps and collect the actual Recipe e2e result when its process finishes. No Ginux runtime source change, upstream push, PR, comment or packet preparation occurred.

### Current image shebang negative and valid controls — 2026-11-07

After the user's explicit authorization of this local disposable-root Daybreak probe, clean local Ginux integration `158e11a6`/configured binary SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8` returned `-ENOEXEC` (`-8`) for an empty `#!` header, `-ENOENT` (`-2`) for a missing shebang interpreter, and `-ELOOP` (`-40`) for a two-script cycle. A valid image shebang `#!/bin/second` then executed in a forked child, exited 77, and was reaped by the parent. The launcher exited 0/`exited/0`, stdout was `script-negative: PASS`, and the complete 3,383-byte audit showed the three denials, successful child exec and `teardown tasks=0 owned-handles=0`. The fixture used no storage assignment and did not touch the concurrent Recipe e2e run.

The ignored local `ginux-current-script-negative-158e11a6/` directory retains the runner, both fixture sources/binaries, instance, result, full audit and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `e3f3963b10bfa255b1ca449b346ccaf218b5203aed2e016f5c82cdeadc48c5bb`, `371039062558aec3090da05e824155c15478db57a916546b98e78f9f7114cc19`, `716d74d2279e6af3914859cb7384b6dc895acf7af6dea85637677a706d9f84c6` and `b04d132eabb138178ac9ec7c428bb53883a7c49295e010e36d9b676850833a6e`. Automatic approval review initially rejected the privileged probe based on the earlier local-security boundary; the user then explicitly authorized this exact local probe. One generated fixture failed compilation under `-Werror` before launch, and the first executable fixture supplied an undefined fourth `wait4` register, causing its own exit 15 after the three expected denials. Both are retained in `initial/` and are not counted as Ginux failures or passes. The corrected fixture ran once and passed.

This covers selected image shebang errors and a valid interpreter path. It does not verify owned-script mutation, long/truncated headers, PT_INTERP failure chains, every argv/env/auxv bound, or full G11/G03c–G06b security acceptance. G03c/G12 descriptor canonicality remains failed, G02 clock policy held, and full Recipe e2e remains unverified until its process finishes. Exact next action: inspect an independent untested current-source exec authority edge, then incorporate the actual Recipe e2e result and retest changed storage after upstream wave-one merges. No runtime source change, upstream push, PR, comment or packet preparation occurred.

### Current merged-upstream configured poll runner — 2026-11-07

On clean local integration `158e11a6` with exact Ginux runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, the unchanged full `scripts/test-launch-poll.py` runner passed with a freshly built current-source fixture. It exited 0 and printed `PASS` for all 12 modes: `all`, `read`, `write`, `hup`, `deadline`, `timer`, `console`, `kill`, `infinite-kill`, `all-again`, `ordinary`, and `ordinary-timer`. Exact runner/fixture-source/fixture-binary/log SHA-256 values are `115bb4b4d89e0a56d413e9a94cde667dde5f43f3343c86c06768bffa3b6dbbe0`, `bf7f317a0e731342dc8e351f91dab4958e79f7e8901054c324d57067cedfebf9`, `52da933824ec5a1e9cc391a14575724df6a8657bdb3f13bf58eb0e5c72ea7a6e` and `8c4e527c5d55f47cadaa6f391fe2ebbfd4fbbfdcf53c4223c5d2c849b6b56099`. `ginux-current-poll-suite-158e11a6/` in ignored local Daybreak evidence retains exact inputs, full output and receipt.

This is current-revision configured-poll regression evidence. It does not establish every G08a mutable-array, partial-copyout, cancellation or descriptor-replacement interleaving. G03c/G12 high-bit descriptor canonicality remains failed, G02 clock policy held, and Recipe e2e unverified until its process finishes. Exact next action: inspect current-source unresolved Daybreak criteria and the Recipe e2e process/result; retest changed storage after upstream wave-one merges. No Ginux runtime source change, upstream push, PR, comment or packet preparation occurred.

### Current owned-script negative and valid controls — 2026-11-07

After automatic approval review rejected a new privileged owned-script probe under the earlier local Daybreak boundary, the user explicitly authorized this exact disposable-root `memory:/work` probe. It ran once on clean Ginux integration `158e11a6` with configured binary SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`. The guest created five executable files in the assigned memory namespace. `execve` returned `-ENOEXEC` (`-8`) for an empty `#!` header, `-ENOENT` (`-2`) for a missing interpreter and `-ELOOP` (`-40`) for a two-owned-script cycle. A valid owned script with `#!/bin/second` executed through the image interpreter in a forked child, exited 77 and was reaped. The launcher exited 0/`exited/0`, stdout was `owned-script-negative: PASS`, and the untruncated 6,008-byte audit showed the three denials, successful child exec and `teardown tasks=0 owned-handles=0`.

The ignored local `ginux-current-owned-script-negative-158e11a6/` evidence directory retains the runner, fixture/interpreter sources and binaries, config, full audit, result, stdout and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `3267bb79492894012faed6599a1bd06699efe48f31137398c449291db7b9e953`, `43be6d7f14c59d02860f6497de9455f33c22418ba5a02d65f5f84ed876ce042e`, `eccefd640b0754283ca91eba6c90a2c384e05bf8523f258c368e52f4923a5b10` and `5e7cee1d2ccacf5e5aab412218e2dbfaf72fda4e6cafb631d990ef91d280754c`. This is a selected current-revision G11/G03c script case, not proof of truncated-header, shared-argument, private-descriptor retirement, all argv/env/auxv bounds or complete exec authority acceptance. G03c/G12 high-bit descriptor canonicality remains failed, G02 clock policy held, and Recipe e2e remains unverified until its live process finishes. Exact next action: inspect remaining source-specific Daybreak criteria and the e2e result when available; retest changed storage after upstream wave-one merges. No Ginux runtime source change, upstream push, PR, comment or packet preparation occurred.

### Current owned shebang 256-byte boundary — 2026-11-07

With the user's general authorization for disposable local Daybreak validation, clean integration `158e11a6` and exact configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8` ran one owned `memory:/work` boundary fixture. A 256-byte shebang whose interpreter token had no separator returned `-ENOEXEC` (`-8`). A second 256-byte shebang with a complete `/bin/second` interpreter token and its newline at byte 255 executed through that image interpreter in a child, exited 77 and was reaped. Launcher result was 0/`exited/0`, stdout `owned-shebang-boundary: PASS`, and the untruncated 4,177-byte audit showed the negative exec, successful child exec and `teardown tasks=0 owned-handles=0`. The ignored local `ginux-current-owned-shebang-boundary-158e11a6/` directory retains the runner, source/binaries, config, audit and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `6c07c6ea5363ceeb7893005647db0385d532e020b5fdc7601b8cc05b1e88d733`, `1299b84827ac763eb3e9d7846820dd7543d5133026be014d44dcb6e04314798b`, `8a3530c1dc1b6bcc5ebb7319765c1b62a2c9e66d5214c5eec1da3f20bc98de62` and `bb363aabff8f22fcad5c4d024e11ca709272e7446584f00ab46f387e881cd46d`.

This checks two selected owned shebang shapes at the first 256-byte read boundary. It does not establish behavior for a newline beyond byte 255, a truncated optional argument, source mutation during capture, all private descriptor lifetimes or full G11 acceptance. G03c/G12 high-bit descriptor canonicality remains failed and G02 clock policy held. Recipe e2e was still live at the last direct check; upstream wave-one PRs #232–#234 had not merged at the last read-only check. Exact next action: inspect one remaining source-specific G11 argument/rollback edge and incorporate the actual e2e result when available; retest changed storage after the upstream merge trigger. No Ginux runtime source change, upstream push, PR or comment occurred.

### Current owned-script exec vector count boundary — 2026-11-07

On clean local integration `158e11a6` and exact configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, an authorized disposable `memory:/work` guest created an executable owned script with image interpreter `#!/bin/second`. Its `execve` with 4,097 nonnull argument pointers returned `-E2BIG` (`-7`), and the same count in the environment vector also returned `-E2BIG`. A later valid vector executed that same script in a forked child; the interpreter exited 77 and the parent reaped it. Launcher exit was 0/`exited/0`, stdout `owned-vector-boundary: PASS`, and the untruncated 3,883-byte audit shows both denials, successful child exec and `teardown tasks=0 owned-handles=0`.

The ignored local `ginux-current-owned-vector-boundary-158e11a6/` evidence directory retains the runner, generated sources and binaries, config, audit, stdout, result and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `a710a68d110997aeaca2463007c755086a7cbdaca40a7996280be914ec527849`, `1e1a88f3f1f2d5e7866277b4cd470b97a25be3a321fcccfd183ba71729d3f78d`, `efaffb2740967c4ea0be8169d87cf992dcbc7e39ed33be2735dce9f44f5c6f8a` and `43bb2e086a69aeff4f5a174d1277375f1db2735785452a5ae74126c9b742e03c`. This verifies the selected 4,097-pointer rejection in each vector with a valid control; it does not establish byte-length limits, protected/cross-page pointer faults, auxv bounds, shared-vector mutation or full G11 acceptance. G03c/G12 descriptor canonicality remains failed and G02 clock policy held. The separate Recipe e2e process was still live at the direct host PID check (`1435115`); its partial log already reported failures, and its final result is unverified. Exact next action: inspect the full e2e result when that process finishes, then continue remaining G11/Daybreak checks and storage-dependent retests after upstream wave-one merges. No Ginux runtime source change, upstream push, PR or comment occurred.

### Current owned-script protected exec vector tables — 2026-11-07

On clean local integration `158e11a6` and configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, an authorized disposable `memory:/work` guest created an executable owned script, then made a dedicated aligned 4 KiB pointer-table page inaccessible with `mprotect(PROT_NONE)`. `execve` returned `-EFAULT` (`-14`) with that page as argv and again with it as envp. A later valid vector executed the same script through `/bin/second` in a forked child, which exited 77 and was reaped. Launcher exit was 0/`exited/0`, stdout `owned-vector-protection: PASS`, and the untruncated 3,939-byte audit shows both denials, successful child exec and `teardown tasks=0 owned-handles=0`.

The ignored local `ginux-current-owned-vector-protection-158e11a6/` evidence directory retains the runner, generated sources/binaries, instance, audit, stdout, result and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `017889837bec916e3cffb1f4fdfbb9e0a4cdd5d6b3b4772be1402b91b8426e18`, `3a9b226fc05690f94d8af1e72c22cb630e743c11ccc64a0761374f939c1373b0`, `3b08d1d0b273c845b3be8ea8030ee651b95badfb6fe9caf211d2433d570cfa8c` and `c981acfaf23f00e8674f4ae4962c746600134b76d7913e8c77a69ed4e15897bf`. This selected page-protection case does not establish cross-page partial vector reads, pointee-string protection, shared-vector mutation, auxv bounds or complete G11 acceptance. The separate Recipe e2e process remained live at the host check, and its partial log already contained failures; final suite status is pending. G03c/G12 descriptor canonicality remains failed; G02 clock policy held. Exact next action: inspect full Recipe e2e output when it finishes and continue remaining independent Daybreak criteria. No Ginux runtime source change, upstream push, PR or comment occurred.

### Current owned-script protected argv/env strings — 2026-11-07

On clean local integration `158e11a6` and configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, an authorized disposable `memory:/work` guest created an executable owned script, then made a dedicated aligned 4 KiB page inaccessible. Its argv and envp pointer tables remained readable; one pointee in each selected `execve` referenced the protected page. Both owned-exec transactions returned `-EFAULT` (`-14`) after entering mediation. A later valid vector executed the same script through `/bin/second` in a forked child, which exited 77 and was reaped. Launcher exit was 0/`exited/0`, stdout `owned-vector-pointee: PASS`, and the untruncated 4,195-byte audit shows both failed mediated execs, successful child exec and `teardown tasks=0 owned-handles=0`. This checks selected failure recovery but does not independently prove every private descriptor lifetime.

The ignored local `ginux-current-owned-vector-pointee-158e11a6/` evidence directory retains the runner, generated sources/binaries, instance, audit, stdout, result and receipt. Runner/fixture-source/fixture-binary/audit SHA-256 values are `e7065a1ba169ffbc32b546931f67edc15a131c3d5924459dbeda59b49a83fd8a`, `b463193eb2c68aa618ba230517b46faf01071be080fa015a89e173734bb9bb87`, `cfe7b20ba0367d74c2096604b21111f4dc1088da5f4c8ee3d954241797593ea3` and `87045b789ee5bb77222b2e02a40466c3a319aa0a952452149fae95578cee1c6b`. This does not establish cross-page or concurrent string mutation, full argv/env/auxv bounds, all rollback paths or complete G11 acceptance. The separate Recipe e2e run remained active at the last direct host PID check; its partial log already contained failures. G03c/G12 descriptor canonicality remains failed; G02 clock policy held. Exact next action: inspect the final Recipe e2e result when available, then continue independent Daybreak criteria and storage-dependent retests after upstream wave-one merges. No Ginux runtime source change, upstream push, PR or comment occurred.

### Current configured-storage assignment ancestor host oracle — 2026-11-07

On clean local integration `158e11a6` and configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, an authorized disposable fixture assigned `memory:/bridge/work` while the image's `/bridge` was an absolute symlink to a synthetic host marker directory under a temporary work root. The guest could create `/bridge/work/good`, confirming the intended owned assignment. For the same guest pathname `/bridge/secret/target`, a mode-000 synthetic host `secret` directory made the supervisor return `-EACCES` (`-13`); after that host directory alone was removed, the guest received `-ENOENT` (`-2`). The guest root contained no `/tmp` target for the symlink in either launch. `/bridge/absent/target` returned `-ENOENT` both times. Both launches exited 0/`exited/0` as probe programs, showed the expected classification strings and `teardown tasks=0 owned-handles=0` in untruncated audits. The **assignment confinement criterion failed**: a host directory outside the guest root changed the guest-visible result. The probe did not read host marker bytes or demonstrate arbitrary host-file access.

The ignored local `ginux-current-assignment-ancestor-oracle-158e11a6/` evidence directory retains the paired runner, fixture source/binary, instance, both result/audit/stdout sets, receipt and the initial single-run control. Paired runner/fixture-source/fixture-binary/instance SHA-256 values are `072577c1c0b09e1031fd1fd625c39d661087b418844c918df3cc3e60136fd6b5`, `a175133797479f85717aac848bbd7e086b7aaa0a26232666f2a6e2668dd8885f`, `a9fd8b368cbe83c4fea42f4b1620c4684c03258009e4820928965e6aa8bd5856` and `7fa2373ec1c3eee0d1838fef3871f10338e531f66b35f537d6ebdb0a9279f89a`; present/removed audit hashes are `8d29d6aa8989f253f84b8534edd31f4a1eeee2f6e6ef2d85291fe750b316df05` and `880c823113ca90cc77300922f976f21409a7cbdc081d6365d1f0a0c94c23ddf4`. This is the known G03c configured-storage defect, now reproduced on the current merged revision. Claude owns its upstream redesign; no local fix is warranted without a blocking Recipe case. G03c/G12 high-bit descriptor canonicality separately remains failed, and G02 clock policy held. The independent Recipe e2e host process was still live at the direct check and its partial log already contained 13 failures; final suite status remains pending. Exact next action: inspect the e2e result when complete, then retest both failed storage criteria after Claude's upstream changes reach the integration source. No Ginux runtime source change, upstream push, PR or comment occurred.

### Current configured-launch low-stack crash — 2026-11-07

On clean local integration `158e11a6` and configured runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, authorized disposable launches compared process `RLIMIT_STACK` values with core dumps disabled. The same `memory:/work` owned-script fixture passed at 8 MiB and 3 MiB (launcher exit 0, `exited/0`, `owned-vector-pointee: PASS`, complete teardown), but the 2 MiB launch died from SIGSEGV (subprocess return `-11`) before any audit bytes or result file. A separate no-storage-assignment static init exited 77 with `exited/77` at 8 MiB and likewise died from SIGSEGV before audit/result creation at 2 MiB. Thus a **low-stack launch reliability criterion fails** on the current revision, including without an assignment. The selected threshold is bracketed between 2 and 3 MiB; this does not establish the exact minimum or the faulting instruction.

The current source uses `MappedArray` for storage namespace and backing metadata, while `launch.rs` constructs `LaunchChannels` in the launch frame and `channel_hooks.rs` defines sixteen 65,536-byte pipe buffers. Those source facts make the earlier “2 MiB MemoryFs on stack” attribution unproven for this revision; a crash-level diagnosis has not been completed. The ignored local `ginux-current-storage-stack-boundary-158e11a6/` evidence directory retains the three exact runners and receipts, configs, full successful audits, empty failure audits, fixture/runtime hashes, and the initial no-assignment wrapper receipt. The initial wrapper incorrectly asserted launcher exit 0 for a guest that intentionally exited 77; it failed after recording the valid observations, and the corrected wrapper reran and passed. Corrected no-assignment runner SHA-256 `87b3bb9c9408683b7ab1c5e76804d2c55aeb054dcbd677dc29fa21c81f58bc5d`; assigned 3 MiB and 2 MiB runner hashes `0938263691982d67783cfb51716923353df64793d224f507259cea3f0ef3b9d9` and `79fb80707a6e1507b23b0c3e638de9b816c3699f9d1daf9f9bdf2cde5a142e79`. No local redesign is undertaken absent a blocking Recipe case; Claude owns upstream Ginux PR decisions. G03c assignment confinement and G03c/G12 descriptor canonicality remain failed; G02 clock policy held. The separate Recipe e2e process was live at the last direct host check, with 13 failures in its partial log and no final result. Exact next action: inspect the final e2e result and diagnose whether a Recipe case is blocked by the low-stack failure; then retest changed Ginux source after upstream merges. No runtime source change, upstream push, PR or comment occurred.

### Current G07 native pathname scratch regression — 2026-11-07

Local integration commit `f4b2e240` adds `X-scratch` to Ginux's existing thread
fixture and runner. A shared peer overwrites the exact `rsp-512` pathname scratch
with image `/work/hh`, hidden by `memory:/work`, while the owner opens constant
`/./allowed`. The fixture uses two vCPUs and requires a valid allowed `A` read.
The final fixture on preserved pre-guard binary `f9a584e9` observed hidden `S`
and exited 205 (runner 1); on current guarded binary SHA-256
`12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`
it passed (runner 0). Full thread runner, `make check`, release build and full
storage runner also passed. Exact source, binary, hashes, commands, logs and exit
statuses are in ignored `ginux-current-g07-scratch-regression-f4b2e240/`.
This establishes one native path-scratch protection on the current revision;
other indirect arguments, mapping/exec interleavings and private descriptor
checks remain partial. The previous `6d290d48` exposure is historical and does
not establish a current failure. A FIFO peer-stop diagnostic was unrun as a
guard check because FIFO metadata was denied; it is not a security failure.
No upstream action occurred.


### Finished pinned Recipe full e2e — 2026-11-07

The independent six-thread run from saved `runrecipe.sh` finished after 6,670.14 s with **exit 101: 122 passed, 13 failed, 0 ignored, 0 unrun**. It used Recipe source `2ad833c`, Ginux integration `158e11a6`, runtime SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`, and the exact saved Recipe CLI and e2e harness. The complete log SHA-256 is `7102a2ec3d6063a284f61882966120275370f5aefe75ca10bfb689d2773a01b7`. The invocation, copied log, binary hashes and classification are retained in ignored local `recipe-full-e2e-158e11a6/receipt.json` under the Daybreak evidence directory.

Six failures have an HTTP fixture accept timeout (`EAGAIN` after 120 s); one also records the guest download's connection refusal. Two failures explicitly show guest `/usr/bin/sleep` failing because the realtime clock returns `ENOSYS`, and a third cancellation case waits for a marker before a guest sleep. One partial-frame case exceeded its 20 s assertion, two managed-state cases never observed their expected log markers, and one rootfs-integrity case lacked the expected diagnostic. These are observed failure classes, not a common-cause diagnosis. A later Recipe commit `6604ad1` adds the missing installed-system fixture policy for the rootfs-integrity case, but this full run predates that commit. No failed case is promoted to a pass. The G02 clock hold remains in force. A single pinned HTTP case reran with one test thread and **failed** after 138.62 s: the fixture accepted no connection within 120 s (0 passed, 1 failed, 134 filtered out; exit 101). Another 13-case Recipe rerun was active on the same host, so this was not an isolated-host check. A separate disposable CLI probe timed out after 120 s with no HTTP request; the CLI was CPU-bound and no Ginux child was observed before timeout. That probe is inconclusive about Ginux network mediation. Exact focused and manual logs, fixture source and hashes are in the same ignored evidence bundle. The newer Ginux VFS offset fix and G07 scratch regression are absent from the pinned full-run binary. Exact next action: let the other Recipe rerun finish, then rerun one HTTP case on an idle host or inspect its saved diagnostics; diagnose independent failures without rerunning the entire suite prematurely. Security and integration acceptance remain incomplete; no upstream action occurred.


### Current G11 optional shebang argument boundary — 2026-11-07

On clean local Ginux integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, an authorized disposable owned-script fixture published two executable scripts under `memory:/work`. The 257-byte script placed its newline at byte 256; its image interpreter received a 241-byte optional shebang argument. The 255-byte control placed its newline at byte 254 and supplied 240 bytes. The interpreter measured each argument and exited with distinct codes, both children were reaped, and the parent/launcher exited 0 with `g11-optional-argument-boundary: PASS` and complete teardown in a 4,947-byte audit. The exact final runner/fixture/interpreter/runtime hashes, command, audit, result and stdout are in ignored `ginux-current-g11-optional-argument-f4b2e240/` (receipt SHA-256 `bd1b01906c80922396196f2d30024567e22570c55769cabb1695cb58b4f42a83`). Initial probes assumed `E2BIG` or guessed the wrong delivered length; those failed fixture assertions are retained and are not runtime failures. This establishes only these two optional-argument lengths, not shared-header mutation, all interpreter chains or full G11 acceptance. The separately running Recipe 13-case rerun has no final result in this checkpoint. Exact next action: inspect its final saved result when complete, then continue independent G11/G07 authority and rollback checks on the current integration source. G02 remains held; no upstream action occurred.


### Current G11 selected private-script descriptor visibility — 2026-11-07

On clean integration `f4b2e240` and exact configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a disposable guest published an invalid owned script and a valid owned script with image interpreter. The invalid exec returned `-ENOEXEC`; `fcntl(F_GETFD)` on guest descriptors 3–63 returned `-EBADF` after that failure. The valid script executed in a child; its interpreter independently saw `-EBADF` for the same range and exited 77. The parent reaped it, repeated the descriptor scan, and exited 0. Launcher/result/stdout passed, and the untruncated 40,925-byte audit ended with `teardown tasks=0 owned-handles=0`. Exact sources, binaries, command, output and hashes are in ignored `ginux-current-g11-private-script-fd-f4b2e240/` (receipt SHA-256 `c1bdf921b9337e48d55b28d58238b635e6aaf7cd9d6324bb11109aa6163ff758`). This checks guest visibility in descriptor range 3–63 after one failure and one successful exec. It does not prove every possible private descriptor number, all rollback paths, or host-side descriptor closure; full G11 acceptance remains partial. The separate Recipe rerun is still live at this checkpoint, G02 held, and upstream main remains `53a54add` with #232–#234 open. Exact next action: incorporate that rerun's final result when available, then continue an independent current-source G11/G07 authority check. No upstream action occurred.


### Current G11 host-observed private script descriptor lifetime — 2026-11-07

A stronger disposable follow-up on clean local integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79` used explicit stdin/stdout handshakes to inspect the native tracee from the host. `/proc/<tracee>/fd` contained exactly `[0, 1, 2]` before the owned scripts were created, again after an invalid owned script returned `-ENOEXEC`, and in the child after its valid owned script executed the image interpreter. Guest `fcntl(F_GETFD)` over 3–63 also returned `-EBADF` at the failure and interpreter stages. The child exited 77, the parent reaped it, launcher/result/stdout exited 0, and the untruncated 42,204-byte audit ended with complete teardown. Exact runner, fixture/interpreter sources and binaries, command, host FD observations, audit and hashes are in ignored `ginux-current-g11-host-fd-lifetime-f4b2e240/` (receipt SHA-256 `08ab9652e1e0d7829e8ce568460bd1f27969cefbc8bef6579fc3ee9fe5b740fa`). This directly supports selected private descriptor cleanup after one rejected and one successful script exec; it does not observe every transient instant or all rollback paths. G11 remains partial. The separate Recipe comparison remained live at the last direct process check; its partial log is not a final result. G02 is held, and storage authority failures remain. Exact next action: inspect that comparison's final result when available, then continue independent current-source script/argument authority checks. No upstream action occurred.


### Current G10 full owned-exec runner — 2026-11-07

On clean integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a fixture rebuilt from the current source passed the unchanged full `scripts/test-launch-owned-exec.py` runner (exit 0). All 19 named scenarios passed: seven owned-ELF replacement forms, eight ordinary owned-ELF forms, and four interpreter-chain forms. The runner checks host `/proc/<tracee>/exe` bytes, sealed execution backing, expected native descriptor sets, deleted-name projection for ordinary unlinked exec, guest result and teardown within its cases. Exact build/test commands, full stdout, runner/fixture source and binary/runtime hashes are in ignored `ginux-current-owned-exec-suite-f4b2e240/` (receipt SHA-256 `7f50a4a2583b4c6bea7ae41fcf7acf4473049d159e367859d449f517d556d424`). The runner removes its disposable per-case audits after assertions; those audits are not independently retained by this run. This is strong current-source ordinary G10 regression evidence, not a substitute for all hostile buffers, maximum names, transient failure injection or complete G10 security acceptance. The separately running Recipe comparison has not reported a final result. Exact next action: inspect that saved result when complete, then target remaining independent G10/G11 adversarial checks; retain G02 and storage failures as pending. No upstream action occurred.


### Current G10 maximum deleted executable name — 2026-11-07

On clean local integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, an authorized disposable guest first tried a 256-byte owned pathname and received `-ENAMETOOLONG` (`-36`). It then copied a valid image ELF into `memory:/work` at a 255-byte pathname and executed it in a child. Guest pipes held the child until the parent unlinked that exact name. The child verified `/proc/self/exe` returned its 255-byte virtual path plus the 10-byte ` (deleted)` suffix (265 bytes), and a short 8-byte readlink returned the expected prefix. The audit records the unlink and readlink results 265 and 8; child exited 77, parent/launcher exited 0 with both PASS markers, and the untruncated 7,279-byte audit ended with `teardown tasks=0 owned-handles=0`. Exact runner, source/binaries, command, audit, result and hashes are in ignored `ginux-current-g10-max-deleted-exe-f4b2e240/` (receipt SHA-256 `8b4b8adbc5575de08673d426cea19f2b78d9d67112a685215d7b8f7b1e3820ca`). This is one maximum accepted path and deletion projection; concurrent rename, protected output races and every transient descriptor failure remain unverified, so G10 security acceptance is still partial. The separate Recipe comparison has not completed. Exact next action: inspect its result when final, then select another independent current-source authority or fault case. G02 and storage failures remain pending; no upstream action occurred.


### Older-Ginux Recipe 13-case comparison half finished — 2026-11-07

A separate saved `rerun13.sh` selected exactly the 13 failures from the full Recipe run with Recipe binary/harness `2ad833c` and two test threads on each of two pinned Ginux binaries. Its first half, Ginux `ebcdaee0` (binary SHA-256 recorded in the ignored receipt), **failed: 1 passed, 12 failed, 122 filtered out, exit 101 in 1,511.20 s**. Only `managed_concurrent_download_cache_survives_offline_reinstall` passed. The other failures include four HTTP fixture accept timeouts, the Alpine refresh HTTP timeout, two explicit guest sleep `ENOSYS` errors, a 20-second frame assertion, a missing coordinator marker, two missing managed-state markers, and the rootfs-integrity diagnostic mismatch. The complete saved log SHA-256 is `f30ebf8ff8916e5e1b08cfa170134d6ec96d4be41af1fa5d9fd6e5abd89e4fac`, copied into ignored `recipe-full-e2e-158e11a6/` with a receipt. These overlapping symptoms show the failures are not unique to the later binary in this non-isolated comparison; they do not identify a common cause or prove the later binary is sound. The `158e11a6` half remained live at the direct process check and has no final result here. Exact next action: inspect that final result, then diagnose the HTTP and marker timeouts under controlled host load. No case is promoted to a pass, G02 remains held, and no upstream action occurred.


### Current full G09d restore runner — 2026-11-07

On clean integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a storage fixture rebuilt from current source passed the unchanged full `scripts/test-launch-restore.py` runner (exit 0): `test-launch-restore: PASS (roundtrip, startup isolation, parent ordering and malformed inputs)`. The runner checks repeated export/restore, hardlink and symlink metadata, 5,000 nonempty objects, a complete >2 GiB sparse payload, assignment mismatch, damaged/truncated archives, traversal, duplicate entries, malformed link targets/metadata, and symlink archive source. Exact source/fixture/runtime hashes, build/test commands and complete stdout are in ignored `ginux-current-restore-suite-f4b2e240/` (receipt SHA-256 `885a3dba05cc447e1edd8a5010143a54f22a30a8f76551ae2fd1eda9d858412e`). The runner deletes each disposable archive and audit after checking it, so independently inspectable per-case audit bytes are unavailable from this run. This is current-source full-runner evidence, not injected FD/read/close failure or mutable-source-race acceptance; G09d remains partial. It overlapped the separate Recipe 13-case rerun, so that Recipe comparison is not an idle-host timing experiment. Exact next action: inspect the Recipe comparison's final result and continue remaining independent security fault cases. G02 and storage authority failures remain pending; no upstream action occurred.


### Current G21 archive-open EMFILE control — 2026-11-07

On clean local integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a disposable valid 7,168-byte archive was exported and restored successfully as controls. A path-filtered one-shot `strace` then returned `EMFILE` from the supervisor's `openat` of that exact archive. The trace records `O_RDONLY|O_NONBLOCK|O_NOFOLLOW|O_CLOEXEC` and `EMFILE (INJECTED)`. The launch failed before guest output with exit 2/result `supervisor-error` and `cannot open restore archive`; the image and archive bytes were unchanged. Exact probe, trace, audit, stdout and hashes are in ignored `ginux-current-g21-open-emfile-f4b2e240/` (receipt SHA-256 `1d0b40f9d5b0c554915d2535f6909c4424eadf39571872d55e36e7dc44d690a2`).

An initial `RLIMIT_NOFILE` sweep did not reach restore: low limits failed earlier in export staging or descriptor-budget validation. The preliminary sweep receipts and final no-export sweep script are retained beside the successful probe; neither sweep counts as archive-open fault coverage. The injected control verifies one archive-open failure path only. Real close-error lifetime, every resource limit, concurrent mutation and storage authority remain incomplete; G03c/G12 canonicality is failed pending Claude's corrected source, G02 clock policy held. Exact next action: inspect the independent Recipe comparison when final and continue current-source authority/fault checks, then retest changed storage behavior after upstream merges. No Ginux source change, push, PR or comment occurred.


### Current G10 protected executable-name output — 2026-11-07

A disposable extension of the earlier 255-byte owned-ELF/unlink fixture ran on clean integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`. After the child executed the owned ELF and its parent unlinked the virtual path, ordinary full and 8-byte `/proc/self/exe` readlink controls returned 265 and 8 bytes. The child then mapped two pages, protected the second, and requested a 32-byte readlink at the last 8 bytes of the first page. It received `-EFAULT`; each readable byte remained the sentinel or matched the corresponding virtual-path prefix. A readlink wholly into the protected second page also returned `-EFAULT`. The audit records readlink results `265`, `8`, `-14`, `-14`, followed by child exit 77 and complete teardown; launcher/result exited 0 with both PASS markers. Exact runner, generated source/binaries, config, audit, output and hashes are in ignored `ginux-current-g10-protected-output-f4b2e240/` (receipt SHA-256 `60968e7285406678c13e46c62688d50e5db0fee778397e3a576c50097a858eb4`).

This verifies two protected-output layouts at one settled exec/unlink state. The fixture did not record which readable prefix bytes changed before the cross-page error, and it does not cover concurrent rename, a destination that changes protection during copy, all output lengths, or transient descriptor failures. G10 remains partial; G03c/G12 descriptor canonicality failed pending Claude and G02 clock policy held. Exact next action: inspect the Recipe comparison when final and continue independent current-source authority/fault cases. No Ginux source change, push, PR or comment occurred.


### Current full G11 owned-script runner — 2026-11-07

On clean integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, the unchanged `scripts/test-launch-scripts.py` passed with a fixture freshly built from current source: exit 0 and 36 named passes. Fifteen owned-script modes cover plain/optional/nested/relative/descriptor/unlinked/directory/fork/vfork/clone forms; 21 ordinary modes cover the runner's image/owned interpreter joins and path forms. The runner asserts guest result, expected host `/proc/<tracee>/exe` bytes and native descriptor sets at its handshakes, rollback/success output, image invariance and complete teardown. Exact build/test commands, full stdout, runner/fixture/runtime hashes are in ignored `ginux-current-script-suite-f4b2e240/` (receipt SHA-256 `220ffc3420d6e2cb474e05c40ecbb8b2db3ba78471cab291049f163c8df97b86`). The runner deletes each disposable per-case audit after checking it, so those audit bytes are not independently retained.

This is broad current-source ordinary regression evidence, not proof of hostile shared argument or script-byte mutation, every malformed/truncated header, protected vector/auxv arithmetic, all rollback paths or transient private descriptor lifetime. G11 remains partial; G03c/G12 descriptor canonicality failed pending Claude, G02 clock policy held. The separate Recipe comparison had no final result at the last log check. Exact next action: inspect that final result and continue a focused remaining G11/G07 authority case, then retest changed storage after upstream merges. No Ginux source change, push, PR or comment occurred.


### Current G11 protected argv/env strings and cross-page rollback — 2026-11-07

Two disposable extensions of the earlier owned-script vector fixture ran on clean integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`. The first supplies an argv string, then an environment string, whose pointer lies wholly in a protected page. Both owned-script `execve` attempts returned `-EFAULT`; a subsequent valid child exec exited 77 and the parent/launcher exited 0 with complete teardown. The second extends those controls with argv and environment strings starting in four readable bytes at the end of a page and continuing into a protected page. Both additional attempts returned `-EFAULT`; the later valid child exec again succeeded. The second audit records two vector-pointer `-14` results, two fully protected pointee `-14` results, two cross-page pointee `-14` results, then successful child exec and `teardown tasks=0 owned-handles=0`. Exact runners, generated sources/binaries, config, full audits, stdout, results and hashes are ignored under `ginux-current-g11-pointee-protection-f4b2e240/` and `ginux-current-g11-pointee-crosspage-f4b2e240/` (receipt SHA-256 `501417da6236c149b2b3c4cef0e6fc3478cb0b9ad220eaa907fddbd77e8a075a` and `05f40eba0fe3d8d53b7995c0e1da689dbb55406e8f7e69562b51b7ab68b9b13e`).

These are selected protected pointee and rollback cases, not proof of every string length, shared mutation, auxv arithmetic, private descriptor state at every transient instant or all interpreter chains. G11 remains partial; G03c/G12 canonicality failed pending Claude and G02 clock policy held. The independent Recipe 13-case comparison had no final result at the last log check. Exact next action: inspect that final result and continue independent current-source authority cases; retest storage after upstream merges. No Ginux source change, push, PR or comment occurred.


### Current full G10 exec runner — 2026-11-07

On clean integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, the unchanged `scripts/test-launch-exec.py` passed with three fixtures freshly built from current source: exit 0, five initial interpreter ownership checks and eight exec scenarios, including rollback, descriptor/path/relative/link selection and the forked shared-path case. The runner checks host `/proc/<tracee>/fd` at handshakes, guest result, executable replacement, exact task-exec count and complete teardown where applicable. Exact command, full output, runner/fixture/runtime hashes are in ignored `ginux-current-exec-suite-f4b2e240/` (receipt SHA-256 `dfd767bdd94daa3996fb87b2a8c3f272b75918a9e665d37b5b98bb4ebfa853c6`). The runner deletes disposable per-case audits after checking them.

This is current-source ordinary G10 regression evidence. It does not complete malformed/protected arguments, every private backing fault, concurrent rename/protection mutation, all transient descriptor cleanup or broad exec authority acceptance. G10 remains partial; G03c/G12 canonicality failed pending Claude and G02 clock policy held. The separate Recipe comparison remained live at the last direct host process check. Exact next action: inspect its final result when available and continue remaining current-source Daybreak cases. No Ginux source change, push, PR or comment occurred.


### Current G10 private backing-open fault and completed Recipe comparison — 2026-11-07

A disposable G10 owned-ELF fixture on clean integration `f4b2e240` (runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`) used a one-shot supervisor `openat` `EIO` injection on `/proc/<tracee>/fd/4`. Guest exec returned `-EIO` and passed image rollback, then retried successfully without the fault. Host-observed tracee FDs before/after fault/after retry were each `[0, 1, 2, 3]`; launcher/result exited 0, image bytes were intact, and full audit ended with complete teardown. The ignored `ginux-current-g10-backing-open-fault-f4b2e240/` bundle retains baseline/fault runners, fixture source/binary, traces, audits and receipts (fault receipt SHA-256 `efbdf2ff9dad0d384e38b52b9dfa1a4744df17704f9023a9de94257630ec07e6`). This covers one backing-open failure, not write/seal/close faults or every transient FD lifetime. G10 remains partial.

The saved 13-case Recipe comparison's `158e11a6` half has now finished: exit 101, 0 passed, 13 failed, 122 filtered out, 1,562.51 s. Its log is retained in ignored `recipe-full-e2e-158e11a6/rerun13-158e11a6.log` (SHA-256 `b1d3e71b741f308bffe68f79234dfb890b15354b59da682d0fccfb1319e68f36`). The older `ebcdaee0` half passed 1 and failed 12. Overlapping host work limits causal timing conclusions. HTTP fixture, clock, marker, frame-timing and rootfs-diagnostic failures remain real; held G02 and failed G03c/G12 acceptance are unchanged. Next inspect one HTTP failure on an idle host and continue independent security cases; no upstream push, PR or comment occurred.


### Current G10 private backing seal fault — 2026-11-07

On pinned integration `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a baseline traced successful private-memfd `F_ADD_SEALS` and `F_GET_SEALS`. A separate disposable one-shot `EPERM` injection at `F_ADD_SEALS` returned `-EPERM` to owned exec. The guest's image rollback checks passed, a later retry executed, launcher/result exited 0, the image stayed intact, host-observed tracee FDs remained `[0, 1, 2, 3]` before/after fault/after retry, and the full audit ended with complete teardown. Ignored `ginux-current-g10-backing-seal-fault-f4b2e240/` retains exact baseline/fault source, trace, audit and receipts (fault receipt SHA-256 `8c3fe5538ec6bb279c19b09f7d9e9d2eb02fb2afad38b22cf5734913308f9024`). One seal fault is covered; write/close faults, every transient instant and full G10 acceptance remain pending. The Recipe HTTP timing diagnosis is deferred until host load permits an isolated run. G02 clock policy held and G03c/G12 failures pending Claude; no upstream action occurred.


### Current G10 private backing write fault — 2026-11-07

A disposable baseline on integration `f4b2e240` (configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`) identified the first 4 KiB private-memfd write as post-handshake `write` call seven. A separate one-shot `EIO` injection hit that exact call. Guest owned exec returned `-EIO`, passed image rollback, then retried successfully. Launcher/result exited 0, image bytes stayed intact, host-observed tracee FDs stayed `[0, 1, 2, 3]` before/after fault/after retry, and full audit showed complete teardown. Ignored `ginux-current-g10-backing-write-fault-f4b2e240/` retains exact sources, traces, audits and receipts (fault receipt SHA-256 `c4a40d7ed7c4ab11190a022fecabb477673a0589cf23d67331d7daa5af68835e`). This covers one initial write error, not later/short writes, close errors or every transient descriptor state; G10 remains partial. G02 held and G03c/G12 failed pending Claude. No upstream action occurred.


### Current G10 private backing close baseline — 2026-11-07

A disposable successful owned exec on clean `f4b2e240` traced the supervisor's private-memfd `close(10)` as the third post-handshake close, returning 0. The launcher/result exited 0 with `owned-exec: PASS`. Ignored `ginux-current-g10-backing-close-baseline-f4b2e240/` retains runner, full audit, trace and receipt (trace SHA-256 `b2091a193f2152f0668eaf104d67b0a1800bd4f1fe99b79e0dcd420a0428e2a9`). **No close fault ran**; close-error lifetime remains unverified. Open, seal and write fault checks above are narrower. G02 remains held and G03c/G12 authority failed pending Claude; no upstream action occurred.


### Current G06b selected copyouts — 2026-11-07

On clean integration `f4b2e240`/configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, the existing byte-identical task fixture and runner passed `qfault`, `protected-child-tid` and `x-child-tid` (exit 0, empty stderr). These selected cases cover four wait-status fault destinations with `EFAULT`, consumed-child `ECHILD` and sentinel checks, plus protected and cross-page child-TID destinations. Ignored `ginux-current-g06b-copyout-f4b2e240/` retains exact command, sources, fixture binary, output and hashes (receipt SHA-256 `7b65c83d5e9563944846e6d7cb47218fa79d0448db3e595518a6b3e919df94a1`). Parent-TID birth failure, other clone flags and broader concurrency remain unverified; G06b/G07 acceptance stays partial. G02 held and G03c/G12 authority failed pending Claude. No upstream action occurred.


### Current G07 parent-TID birth fault — 2026-11-07

On pinned integration `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a fixture freshly compiled from current Ginux source passed `basic`. Its positive trace showed four-byte zero preflight and virtual-TID-2 birth writes to the same tracee address. A separate one-shot `EFAULT` injection at only the birth write produced launcher exit 1/result `supervisor-error`, no guest stdout, `thread identity copyout failed`, complete teardown, and no surviving traced guest PID. Ignored `ginux-current-g07-parenttid-birth-fault-f4b2e240/` retains exact runners, fixture source/binary, positive/fault traces, audits and receipts (fault receipt SHA-256 `e174dcbe3251fdf3359d582a9c719938e2c12afab07fa8dc92612be0dffd891a`). This is one synthetic supervisor write fault, not a shared-memory mutation or all G07 copyout races. G07/G06b remain partial; G02 held and G03c/G12 authority failed pending Claude. No upstream action occurred.


### Current G21 restore archive-read fault — 2026-11-07

On pinned integration `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a valid disposable 7,168-byte export/restoration control passed. A path-filtered trace identified its first 512-byte archive-header read from FD 4; a separate one-shot `EIO` injection hit that read. Ginux stopped before guest output with exit 2/result `supervisor-error` and `restore archive read failed`; image marker and archive bytes stayed intact. Ignored `ginux-current-g21-archive-read-eio-f4b2e240/` retains the exact archive/config, fixture source/binary, baseline/fault runners, traces, audits, stdout and receipts (final fault receipt SHA-256 `7bbd6bede4ac1a2f32495f439e4299e467ff062a5076754f87c7c967604daf34`). This is one first-header read fault; mid-stream/short reads, close errors, concurrent mutation and full G21/G22 acceptance remain pending. G02 held and G03c/G12 authority failed pending Claude. No upstream action occurred.


### Current G21 restore mid-stream read fault — 2026-11-07

A separate disposable probe on clean `f4b2e240` injected `EIO` at the fourth archive read, after three successful 512-byte headers and at the 11-byte `out/product` payload. Valid export/restore controls passed; the faulted launch exited 2/result `supervisor-error`, produced no guest stdout, reported `restore archive read failed`, and preserved image marker and archive bytes. Ignored `ginux-current-g21-archive-midread-eio-f4b2e240/` retains exact archive/config, fixture, probe, trace, audit and receipt (receipt SHA-256 `e8a53f891e644ac28db993a66a0ac56106fd56c627c8aaf55db9435a3a01d0dc`). This covers one mid-stream error, not all short reads, close faults, concurrent mutation or full G21/G22 acceptance. G02 held and G03c/G12 authority failed pending Claude; no upstream action occurred.


### Current G21 archive growth after fstat — 2026-11-07

On clean `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a valid 7,168-byte archive passed export and restore controls. Path-filtered `strace` delayed its first read after `openat` and `fstat(st_size=7168)`; the host appended one byte to the same inode. The final archive-terminator read observed that byte, and launch exited 2/result `supervisor-error` with `invalid restore archive terminator`, no guest stdout or final export, and unchanged image marker. Ignored `ginux-current-g21-after-fstat-growth-f4b2e240/` retains exact before/after archives, source/fixture, script, trace, audit and receipt (receipt SHA-256 `3263cbdab10ede1bf6fe6d89c4d0cbb450907fdebf441e4a2900f6904963161a`). This covers one growth timing, not arbitrary rewrites, close-error lifetime or full G21/G22 acceptance. G02 held and G03c/G12 authority failed pending Claude; no upstream action occurred.


### Current G10 private backing close injection — 2026-11-07

On pinned integration `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a one-shot `strace` replacement of the supervisor's third post-handshake `close(10</memfd:>)` with `EIO` made guest owned exec return `-EIO`; rollback, a later retry, image integrity and complete teardown passed. Tracee FDs stayed `[0,1,2,3]`. The supervisor's private memfd FD 10 remained open at the fault and retry handshakes because the injected syscall was skipped. That observed synthetic leak is not proof of a real Linux close-error leak. Real close-error lifetime and full G10 acceptance remain unverified. Ignored `ginux-current-g10-backing-close-fault-f4b2e240/` retains full probe/trace/audit/receipt (receipt SHA-256 `255c0efe229308e86637b504059f9ba0f43c178b3870d09cf3d546f4981e845e`). G02 held and G03c/G12 authority failed pending Claude. No source change or upstream action occurred.


### Current G09a real host short reads — 2026-11-07

A disposable six-record VFS index on clean integration `f4b2e240` and configured runtime SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79` passed with its complete 68-byte backing. Truncating the backing to 18 bytes made a real `pread64` request of 19 bytes at offset 0 return 18; truncating to 66 bytes made the four-byte request at offset 64 return 2 after the first read succeeded. Both runs exited 1 with their respective `os-release read failed` and `bash ELF read failed` diagnostics, cleanup and no VFS PASS marker. The initially sandboxed trace attempt was denied ptrace and does not count. Ignored `ginux-current-g09a-host-short-f4b2e240/` retains exact fixture, outputs and successful traces (receipt SHA-256 `f3ec9501620ae3b9c24aeb3b5ea8054700ab963bb2ab9f738dba4c3f67870f22`). This closes the earlier simulated-reader gap for these two runtime reads only; other G09a adversarial index/backend cases remain. G02 held and G03c/G12 authority failed pending Claude. No source change or upstream action occurred.


### Current G09a runtime extent boundary — 2026-11-07

A disposable current-source `ginux vfs` run on clean `f4b2e240`/pinned binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79` passed at `/usr/bin/bash` data offset 64. With a four-byte extent at `u64::MAX-1`, index validation rejected overflow (exit 1, `invalid VFS index`). At `u64::MAX-4`, the end was exactly `u64::MAX`; validation accepted the record, then the unbacked read failed (exit 1, `bash ELF read failed`). Neither negative case emitted a VFS PASS marker. Ignored `ginux-current-g09a-runtime-offset-f4b2e240/` retains exact indexes/backing/config/runner/output (receipt SHA-256 `dfa18186ff183c5631dd9d014db86c96c57c4151892c253479d9035ae8784c3f`). This is selected runtime extent evidence, not all malformed-index or backend acceptance. G02 held and G03c/G12 authority failed pending Claude; no source change or upstream action occurred.


### Current G10 completed-close reported error — 2026-11-07

A current-source disposable `ptrace` check on clean integration `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79` observed the supervisor's private-memfd `close(10)` enter and return 0 from the kernel, then changed only its return register to `-EIO` before Ginux resumed. The guest rolled back, retried successfully, and exited 0 with `owned-exec: PASS`; supervisor FD links after error/retry matched their pre-exec state, tracee FDs remained `[0,1,2,3]`, image bytes were intact and teardown complete. The initial unprivileged attempt stopped before readiness and does not count. Ignored `ginux-current-g10-postclose-error-f4b2e240/` retains exact runner/fixture/audit/receipt (receipt SHA-256 `365346f54387f034fc6d1b765aa240f6f925275555674a540376af04e1868d02`). This checks completed-close error handling and resolves the skipped-close artifact, but no real kernel close error was induced and full G10 acceptance remains partial. G02 held and G03c/G12 authority failed pending Claude; no source change or upstream action occurred.


### Current G09a malformed index-record controls — 2026-11-07

On clean `f4b2e240` and pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a valid six-record `ginux vfs` image passed. Thirteen malformed header/record, count, truncation, path-order and path-length variants exited 1 as `invalid VFS index` with cleanup and no PASS marker. An embedded-NUL final path instead passed index parsing and failed later at `bash lookup failed`; this is not a parser rejection or demonstrated bypass. Ignored `ginux-current-g09a-malformed-records-f4b2e240/` retains all 15 indexes and outputs plus exact runner and receipt (SHA-256 `98eaf94814d7d7b45ab46c7c32cc4a8963864c56cf1a8cc31d93a55f58b3ed36` and `283bc33d9e01c7364e50119d7c786cfd34f3e8e20086091a40822ec7c2f08896`). Other malformed index shapes, symlink targets and concurrent backing behavior remain unverified; G09a stays partial. Next: G11/G07 shared exec-argument fixture, then storage retest after upstream correction. G02 held and G03c/G12 authority failed pending Claude. No upstream action occurred.


### Current G11 shared argv owned-script exec — 2026-11-07

On clean integration `f4b2e240`/pinned configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, an owned-script control and two forked shared-page argv-writer cases passed. The child atomically alternated an eight-byte argv string in a `MAP_SHARED|MAP_ANONYMOUS` guest page while the parent entered `execve` or `execveat`; the interpreter accepted only one of the two complete strings. Both race cases printed `writer-ready` and `g11-argv-race: PASS`, exited 0 and audited child registration, owned exec, both task exits and complete teardown. Ignored `ginux-current-g11-shared-argv-f4b2e240/` holds exact fixture/runner, outputs, audits, results and receipt (SHA-256 `83c2e9c30309b206c072bb76d53a16c8c40e2e55e36e8de2694ece532f1a29bc`). The initial live-thread fixture received `-95` because exec requires a sole group owner; its spinning peer caused a host timeout. Preserve it as an incompatible probe, not a pass or product defect. The selected forked-writer result does not prove every schedule, exact guard-stop timing, envp or vector-table mutation, mutable script bytes or full G11/G07 acceptance. Next: a distinct shared exec environment/pointer-table case, then G09b and corrected upstream storage. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified. No upstream action occurred.


### Pinned G11 shared environment pointer-table exec — 2026-11-07

On committed `f4b2e240`/configured binary SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`, a no-writer owned-script control and forked shared-page envp-pointer writer cases for `execve` and `execveat` passed. A child alternated the first environment-vector pointer between two valid stable strings while the parent entered exec. The interpreter accepted only `RACE=A` or `RACE=B`; the writer cases printed readiness and the PASS marker, exited 0 and audited both task exits with complete teardown. Ignored `ginux-current-g11-shared-envptr-f4b2e240/` retains exact source, binary, runner, three configs, stdout, full audits, result files and receipt (SHA-256 `775f696b331fb522a5e5d6bc77a2b34790c9c62fae23bd14b899fac4875e1333`). This selected result does not establish exact peer-stop timing, inaccessible pointer transitions, other env/string layouts, script-byte mutation or full G11/G07 acceptance. The integration worktree has unrelated uncommitted changes absent from this pinned binary; preserve and reconcile before current-source claims. Next: G09b mapping rollback/indirect copy on a pinned revision, then corrected storage retest. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified. No upstream action occurred.


### Current G09b replacement-map allocation rollback — 2026-11-07

Current `storage_memory.rs::resize` uses replacement `mmap` plus `ftruncate`, not a supervisor `mremap`. A disposable one-byte owned file on pinned `f4b2e240` and freshly built clean `86e95083` passed uninjected growth controls and one-shot supervisor `mmap=ENOMEM` faults. The faulted positioned write returned `-28`; guest assertions confirmed old byte, size and offset, then retry wrote at 4,096 and passed. Both launches exited 0; the fresh audit records first write `-28`, retry `1` and complete teardown, while the `strace` record identifies the private `ginux-storage` memfd map and injected failure. Fresh configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c`. Ignored `ginux-current-g09b-map-rollback-{f4b2e240,86e95083}/` retains exact source/fixture, binaries, runners, configs, outputs, audits, traces and receipts (fresh receipt SHA-256 `f7e1a457f820f4933eb841f0c21fcadc93ce27dda51fe9d91e084642015bae66`). Initial wrapper text/revision assertions were corrected; no product failure is inferred from them. `ftruncate` failure, other allocation stages, shared page accounting, protected indirect copies, guest `mremap` and full G09b acceptance remain open. Next: one-shot post-map `ftruncate` failure with old-state/retry assertions on clean `86e95083`, then indirect copies. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no upstream action occurred.


### Current G09b post-map truncate failure rollback — 2026-11-07

A disposable owned-file fixture on clean local integration `86e95083`/fresh configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c` passed an uninjected control and a one-shot supervisor `ftruncate=EIO` fault after a successful 8,192-byte replacement map. The first positioned guest write returned `-5`, while old byte `A`, size 1 and offset 1 remained intact. The guest retried successfully, read its new byte at 4,096 and exited 0 with `g09b-truncate-rollback: PASS`. The trace shows cleanup `munmap` of the failed replacement address before retry; audit records `-5`, then `1`, and complete teardown. Ignored `ginux-current-g09b-truncate-rollback-86e95083/` retains exact fixture, runner, binary, config, stdout, audit, trace and receipt (SHA-256 `e3d8234539ebf4ebee9f1af8b553e6372084e9eeb02fc9ea81eed3a4fac27101`). This selected rollback passes, but G09b indirect protected copies, other allocation failures, concurrent accounting, guest `mremap` and full acceptance remain unverified. Next: inspect an indirect guest-memory copy boundary on current source. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b protected indirect read copy — 2026-11-07

On clean local integration `86e95083`/configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c`, a disposable owned-file `readv` with a read-only destination returned `-14`, preserved `ZZZZ` and file offset 0, then succeeded after write permission was restored. A separate `readv` with an inaccessible iovec-table page likewise returned `-14` without offset movement and succeeded after access was restored. Final content was `ABCD`, file size 4, offset 4; guest exit/result were 0/`exited/0`, and audit recorded `-14,4,-14,4` with complete teardown. Ignored `ginux-current-g09b-indirect-copy-86e95083/` retains exact fixture/runner/binary/config/output/audit/receipt (SHA-256 `184cce16033672680fd3275c24ac0eeb3e94161ecdbe1c52e4cb4d793345b24d`). Cross-page partial copy, concurrent mutation, shared page accounting, guest `mremap` and full G09b acceptance remain unverified. Next: cross-page partial owned-file `readv` copyout and offset recovery. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b cross-page partial read copy — 2026-11-07

A disposable owned-file `readv` on clean integration `86e95083`/configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c` requested four bytes into a guest range spanning two pages, with the second inaccessible. It returned 2, copied `AB` and advanced the file offset to 2. After access was restored, a second two-byte `readv` returned `CD` and left the offset at 4; file size remained 4. Guest exit/result were 0/`exited/0`, audit recorded `2,2` and complete teardown. Ignored `ginux-current-g09b-partial-copy-86e95083/` retains exact source/fixture/runner/binary/config/output/audit/receipt (SHA-256 `0e31118ffbb9c363b63b7fa3eb48b466fc13e84f472f4230678d2b2c85a60f9b`). This selected partial-copy behavior passes; concurrent mutation, other alignments, input copying, shared page accounting, guest `mremap` and full G09b acceptance remain unverified. Next: inspect shared mapped-capacity accounting across aliases and errors. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b forked-reference capacity accounting — 2026-11-07

A disposable configured launch on clean integration `86e95083`/binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c` set a 16 MiB storage byte limit. One unlinked file charged 4 KiB; the parent closed its references after `fork`, while the child held inherited references. Growth of a second file from 16 MiB minus 4 KiB to 16 MiB returned `-28` and preserved its size. After the child closed and exited, the same growth succeeded; the guest exited 0 with `g09b-fork-accounting: PASS`, audited failure/success and complete teardown. Ignored `ginux-current-g09b-fork-accounting-86e95083/` retains exact fixture/runner/binary/config/output/audit/receipt (SHA-256 `2b91f8d7761b7771ae2c9eafb56ace0b5dd214aa873aecc03453df182ab7bb8c`). This selected alias-lifetime accounting passes; all interleavings, concurrent growth, other allocation rollback, guest `mremap` and full G09b acceptance remain unverified. Next: inspect a distinct concurrent accounting or guest mapping boundary. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b competing growth under shared capacity — 2026-11-07

A disposable configured launch on clean integration `86e95083`/binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c` released two forked tasks through an owned pipe to request 12 MiB each for different files under a 16 MiB storage limit. The fixture required exactly one success and one `-28`, checked both resulting sizes, then released the winner's capacity and required the loser to succeed on retry. In this run the child won and parent lost; guest exit/result were 0/`exited/0`, audit recorded child `0`, parent `-28`, retry `0` and complete teardown. Ignored `ginux-current-g09b-competing-growth-86e95083/` retains exact fixture/runner/binary/config/output/audit/receipt (SHA-256 `ecd6727597d04d4c8a1b7bb1a00a6f3f7589470b6b262598c8706184ec507263`). This selected two-process outcome passes, but all schedules, allocation fault stages, guest `mremap` and full G09b acceptance remain unverified. Next: inspect guest mapping and a distinct allocation fault. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current selected owned-storage mapping regression — 2026-11-07

The freshly compiled current `launch-storage-fixture.c` (SHA-256 `24bac21c09d8b8dc5cf8461a9f14b89d8aacabd589127a89c74381b713050cbc`) passed the existing selected `mapping` scenario against clean integration `86e95083`/configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c`: exit 0, `PASS launch storage: mapping`. Ignored `ginux-current-storage-mapping-selected-86e95083/` retains exact source, runner, fixture, binary, command and observed output (receipt SHA-256 `ddadd9c038590eedb195318d6fd2f15565c6776f32566839f73e6f79278dc13b`). The runner deleted its temporary root and full audit, so this is selected ordinary regression evidence only. It covers the fixture's owned mapping checks, not complete G09b/G09d security acceptance. Next: a distinct allocation fault or guest mapping boundary with explicit trace. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b private backing creation failure rollback — 2026-11-07

A disposable owned-file first-write probe on clean integration `86e95083`/configured binary SHA-256 `307667f0a8bc7ac0a80d41c454ea53f3515bb1ad36c08d488bb9a8884d2a8b4c` passed its control and a one-shot `memfd_create=EMFILE` supervisor fault after guest handshake. The faulted write returned `-5`, left size/offset zero, and a retry created backing and wrote/read `A`; guest exit/result were 0/`exited/0`, audit recorded `-5,1` and complete teardown. Trace identifies the failed `ginux-storage` creation and successful retry map/truncate. Ignored `ginux-current-g09b-memfd-rollback-86e95083/` retains exact fixture/runner/binary/config/output/audit/trace/receipt (SHA-256 `dbd2802cbe3eb1e3a43f38c87706da80f5675e6b783b4c448ca2b06bbf4abe1c`). Metadata-array allocation, other host resource faults, guest `mremap` and full G09b acceptance remain unverified. Next: launch-time metadata-array allocation failure and cleanup. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b launch metadata allocation failures — 2026-11-07

The integration checkout advanced from `86e95083` to `8a7eb37c` during an initial pinned-binary wrapper; its revision assertion failed after runtime cases, so that wrapper is not counted as a current-source pass. A fresh clean `8a7eb37c` release build (binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219`) passed the corrected three-case runner. Its uninjected `memory:/work` control exited 0 with the fixture PASS marker and complete teardown. A supervisor-only `strace` injected `ENOMEM` into the first 12,288-byte namespace-array `mmap`: exit 2/result `supervisor-error`, `storage namespace allocation failed`, no guest stdout. Injection into the second 4,096-byte metadata-array `mmap` gave exit 2/result `supervisor-error`, `storage metadata allocation failed`, no guest stdout, and traced `munmap` of the first array. Ignored `ginux-current-g09b-metadata-alloc-8a7eb37c/` retains exact runner, fixture, binary, config, outputs, audits, traces and receipt (SHA-256 `1f146b4f4753c2fefbf939988eacfc893224525b1a4a4d3263f1850c0c1cb1e0`). No guest teardown is claimed for pre-guest failures. Other resource faults, shared argument races, guest `mremap` and full G09b acceptance remain unverified. Next: inspect guest mapping and mutable indirect arguments on current source. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current configured guest mremap denial and decision — 2026-11-07

On clean `8a7eb37c`/configured binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219`, an initial owned private mapping growth fixture received `mremap=-38` and exited 13, contrary to its expectation; it is not a pass. An explicit `allow = ["syscall.mremap"]` config still produced `-38`. The corrected disposable fixture passed default and explicit-grant cases: denial left the original mapping byte `A`, file byte `B` at 4,096 and size 8,192 intact; both guest launches exited 0 and audited complete teardown. Source routes configured launch through `decide_balanced`, which omits syscall 25, while the native/systemd catalog lists it. Ignored `ginux-current-g09b-guest-mremap-deny-8a7eb37c/` retains corrected and initial evidence (receipt SHA-256 `f921fb833c9fb9ebc8a992313e89a296d662864b7267860f878d9861a7813f42`). The user chose to keep configured guest `mremap` unavailable. The accepted explicit grant with no effect remains documented for upstream ownership; no local policy change is requested. This verifies the decided configured denial and preservation; full G09b/G09d acceptance remains pending on other checks. Next independent check: shared/mutable owned-file iovec authority. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current G09b shared iovec pointer authority — 2026-11-07

On clean `8a7eb37c`/configured binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219`, a forked guest writer alternated an aligned pointer in a shared owned-file `readv` iovec table between a valid output buffer and an invalid address. Stable controls, 128 race calls and a final retry required each full copy to write exactly `DATA` with offset 4 and each `-EFAULT` to preserve the sentinel and offset 0. Both race outcomes occurred; guest exit/result were 0/`exited/0`, and the untruncated audit recorded 65 successful/66 faulted readv results including controls, plus complete teardown. Ignored `ginux-current-g09b-shared-iovec-8a7eb37c/` retains exact fixture/runner/binary/config/output/full audit/receipt (SHA-256 `a2268f445c2d8c43271565063127d727dd46b2b992919c180fb4ac76f0d777c8`). This selected atomic-pointer mutation passes, not every timing or torn multi-field update. Configured guest `mremap` remains unavailable by user decision; G09b overall and other Daybreak acceptance remain partial. Next: inspect another independent argument/resource boundary and retest corrected upstream storage when available. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current complete existing storage runner — 2026-11-07

The unchanged complete `test-launch-storage.py` ran without a scenario filter on clean local integration `8a7eb37c`, fresh configured binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219`, and freshly compiled current fixture SHA-256 `24bac21c09d8b8dc5cf8461a9f14b89d8aacabd589127a89c74381b713050cbc`. It exited 0 with `test-launch-storage: PASS (isolation, data, metadata, copy protections, bounds, fresh launch)`. Ignored `ginux-current-full-storage-8a7eb37c/` retains exact runner/fixture source, compiled fixture, binary, wrapper, stdout/stderr and receipt (SHA-256 `e7cf75f3c0c16c9fe8cb63eadf0b48b9769a05a2b030df750f757d27e0de7894`). Temporary roots and individual audits were deleted by the repository runner; this is a full pass of its existing assertions, not full Daybreak acceptance. G02 remains held and G03c/G12 high-bit descriptor authority failed pending Claude. Wave-one PRs #232–#234 remain open, so no rebase occurred. Next: current G09c protected/partial `getdents` copyout and cookie check; retest corrected storage after upstream merge. No Ginux source change or upstream action occurred.


### Current complete directory runner and G09c copyout — 2026-11-07

The unchanged full `test-launch-directories.py` passed all 12 named modes, each twice, on clean integration `8a7eb37c`/fresh configured binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219` and freshly compiled current fixture SHA-256 `a95d8dfe727f98d7d7b1bd5f13bf8ec8dc6bbc73a2f05c93112721e5795d11fb`. Its `all` fixture checks protected/read-only/cross-page `getdents64` output faults, preserved cookie zero, valid ordered dirents and alias-shared cookies. Ignored `ginux-current-full-directories-8a7eb37c/` retains exact runner/fixture source, compiled fixture, binary, wrapper, output and receipt (SHA-256 `5d6704fdf1082615ca1ce3eeb5cdd5b7830bc86f5335f8407ed9bf120f0b0f03`). Per-launch roots and audits were deleted by the runner. This passes the complete existing directory fixture, not every G09c/G09d namespace or concurrent mutation criterion. Next: another independent Daybreak authority boundary on current source, with storage redesign retests after upstream merges. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current Ginux unit-package regression — 2026-11-07

An initial `cargo test --workspace --locked` on clean integration `8a7eb37c` failed before tests because freestanding `rust-lld -nostdlib` target flags could not link host libraries. The `Makefile:test-unit` eight-package selection with `RUSTFLAGS=''` reached tests in the restricted sandbox, where four real loopback `ginux-devices` tests failed with `EPERM`; later packages were unrun. The same eight-package command outside that sandbox passed all 133 tests with zero failures using an isolated target directory. Ignored `ginux-current-unit-packages-8a7eb37c/` retains the successful complete log, Makefile/Cargo config, command and prior-failure receipt (SHA-256 `b0351e9a75f0f73c61cb65497423d780e75de45f5e16134ac805e8bce3d21087`). Neither failed command is counted as a pass, and workspace-wide/remote CI remain unverified. Next: current full network runner because `8a7eb37c` changed network descriptor dispatch. G02 held, G03c/G12 authority failed pending Claude, Recipe/F03 unverified; no Ginux source change or upstream action occurred.


### Current complete configured network runner — 2026-11-07

The unchanged full `test-launch-network.py` passed all 11 configured TCP modes on clean integration `8a7eb37c`/fresh binary SHA-256 `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219` and freshly compiled fixture SHA-256 `65d34e71ada221bf57dd82a7540aa3e59a96f47cfe72a1b0a1310689f6b5540d`. Ignored `ginux-current-full-network-8a7eb37c/` retains exact runner/fixture source, compiled fixture, binary, wrapper, output and receipt (SHA-256 `43c304359bc1a7dee1421e79f69842df6dbd1ca1db9b7c2be5a76db81c613bde`). Temporary roots and per-mode audits were deleted by the runner; G28/G29 adversarial acceptance remains partial. Next independent integration action: inspect saved F03 destination report and current Recipe consumer revision, then rerun the first remaining ordinary case on the current binary. G02 held, G03c/G12 authority failed pending Claude, full Recipe/F03 unverified; no Ginux source change or upstream action occurred.
