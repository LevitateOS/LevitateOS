# Ginux contribution task list

Updated: 2026-11-06. Status: preparation active under the user’s sequential goal authorization;
extraction, new reviews and publication below are pending. This request authorizes this task list, not
opening PRs. The user will inspect the prepared changes before publication.

This is the remaining contribution queue for the completed local Ginux→Recipe
implementation. [Local validation](recipe-local-validation.md) owns the previous
runtime evidence; [the completed Recipe plan](recipe-simplification-implementation-plan.md)
owns T01–T27. Neither is reopened by this document. Keep this downstream planning
record in this superproject, outside Ginux contributions.

## Outcome and boundaries

Prepare the smallest useful, general Ginux contributions, review and fix each,
validate the actual proposed revision, and let the user inspect the whole package
before opening anything. Every eventual PR targets upstream main and works
without any other unmerged PR. Keep upstream tests and CI intact. Use existing
behavior owners; do not add frameworks or broad Linux compatibility promises.

The earlier **13 areas are not the complete PR inventory**. They describe recent
changes layered onto older local foundations. This plan covers those foundations
as well: 31 original change packets plus the confirmed standalone G05a subpacket,
and preparation, review and delivery tasks. A packet is a review/work unit, not a promised PR. Upstream reconciliation
may eliminate, split or combine packets. The final PR count remains unknown.

Standalone publication can require several waves: a prerequisite must merge
before its dependents can be extracted and validated against new main. Preparing
all local material first does not make dependent PRs simultaneously standalone.
Do not duplicate a prerequisite across PRs or hide a stack by setting every base
to main. Do not turn all packets into one large integration PR.

Preparation uses disposable local source exports and retained patches. No pushes,
new PRs, maintainer messages, existing remote-body edits or branch creation/switching
are authorized; existing checkouts remain preserved. Preserve
human-authored text verbatim. The Recipe zero-users/no-compatibility decision
does not authorize breaking Ginux's upstream interfaces or adopting downstream
policy there. Retain useful upstream defaults and resolve public contract changes
against upstream's own decisions.

## Security validation handoff

The user's 2026-11-06 continuation adds a boundary: record security work that
looks like hacking for DaybreakBlue, the monitored blue-team cloud agent, rather
than performing it here. Ordinary implementation, source review and functional
checks continue locally. [DaybreakBlue handoff](ginux-contributions/daybreakblue-handoff.md)
records adversarial validation separately; unperformed checks remain pending and
must not be described as passing. This does not remove tests or weaken upstream
coverage. No cloud dispatch or outgoing message is implied by this written handoff.

## Evidence used to order the work

- Inspected local Ginux: `7cf2638adaf2fd3b783be636fe2a05b2d60458c5`, clean checkout
  `/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux`, branch
  `codex/ginux-recipe-local-integration`.
- Last recorded upstream reconciliation base: `2a82f61e`, joined by `8b5cf5dd`.
  **This is a historical baseline, not a fresh remote-status check.**
- Local non-merge history beyond that base has 66 commits; the net diff spans
  57 files, +15,332/-456 lines. This scale is why the 13-row table alone is
  insufficient. P03 below must reconcile semantics, not blindly replay commits.
- Existing contribution record: [PRs #216–#218](ginux-pr-preparation-20261105.md).
  It also records #207 and issue #179 policy questions. Their current status and
  comments must be reread at P02; old holds or old CI results are not current facts.
- The integrated checks are historical supporting evidence. They do not prove
  that any extracted packet builds or works independently on current main.

## How to use the order

Execute P01–P05 first. All G tasks require P05. A listed G dependency means its
capability must be available for extraction/testing; current upstream may already
satisfy it. These are conservative capability dependencies from the local code,
not claims that whole commits must be cherry-picked. P04 must prove or remove
edges by tracing actual symbols, configurations and fixtures before extraction.

A G task can be reviewed against a local dependency composition while waiting,
but its standalone acceptance stays pending until all prerequisites are upstream.
The table is topologically ordered. Independent rows can run concurrently when
their writers and test resources do not overlap. A policy decision holds only
its affected packet and dependents, not unrelated fixes.

## Preparation tasks

- [x] **P01 — Preserve and inventory exact inputs.** Record upstream/fork URLs,
  current branches, local and remote refs, dirty changes and existing worktrees;
  preserve original stashes and unrelated work. Record the integration commit and
  baseline above plus any later changes. Use explicit Ginux work-tree paths because
  its shared `core.worktree` setting can otherwise select the wrong checkout.
- [x] **P02 — Refresh upstream and read discussion.** Read current main, open and
  merged PRs, relevant issue comments and required CI configuration. Recheck #153,
  #154, #207, #216–#218 and #179 as historical entry points, then follow current
  discussion. Record URLs, exact main SHA and policy outcomes. No outgoing messages.
- [x] **P03 — Account for every local change.** Compare every local/fork-only
  commit and changed path against current main and the working integration.
  Classify as already upstream/equivalent, superseded prototype, existing PR,
  candidate, or intentionally downstream. Include older remote branches so unique
  work is not lost, but do not revive obsolete demo-only implementations. The
  historical invalid-command fix `4632b5b2` was reported covered by #165; verify
  that and exclude duplicate delivery. Merge `8b5cf5dd` is reconciliation history,
  not its own feature. Assign every remaining hunk, fixture, doc and build change.
- [x] **P04 — Verify dependency and policy map.** Trace task/FD/storage/namespace
  ownership, configuration schemas, archive formats, syscall dispatch and fixtures.
  Verify the edges below against P02's main. Identify capability prerequisites,
  not accidental commit ancestry. Separate pure fixes from feature additions and
  unresolved policy choices. Explain any inseparable grouping. Record public API
  impact and Ginux's upstream compatibility expectations.
- [x] **P05 — Freeze extraction packets and execution lanes.** Give each retained
  packet one observable problem, scope, source hunks, base, dependency list and
  verification commands. Reuse suitable worktrees; obtain the user authorization
  required by repository rules before creating/switching branches. All eventual
  contribution branches use `codex/` and current upstream main as their base.
  Do not mutate existing PR branches. Confirm file ownership before parallel work.

## Change packets in dependency order

Rows describe the complete queue; the checkpoint and package manifests record
completed extractions and reviews. Existing integrated implementation/tests alone
do not establish a reviewed candidate. Source references locate implementation, not a prescribed
cherry-pick sequence. Include each packet's meaningful tests, build wiring and
small generic documentation in that packet; never defer essential coverage to G31.

| ID | Packet and source references | Capability prerequisites after P05 | Required observable evidence |
| --- | --- | --- | --- |
| G01 | Assigned-console bounded polling (`3dd05f42`, `f8235d56`) | Current configured launch; reconcile #207/#179 policy first | Assigned descriptors wait/wake within bounds; denied/unassigned descriptors stay denied; interruption and timeout work without busy looping. |
| G02 | Restricted global clock reads and interruptible sleeps (`9ec95527`) | Approved upstream clock/sleep contract | Only the chosen realtime/monotonic clock IDs, deadline expiry and signal interruption; document native global-clock visibility honestly, and deny fd/PID CPU-clock encodings. |
| G05a | Shared offsets for duplicated VFS descriptors (`b500f859`, relevant semantics from `7bc0ee04`) | Current upstream VFS only | The direct duplicate-read regression fails on upstream and integration, passes old review fix; extract current-main correction, test aliases, independent opens, reused slots and CLOEXEC. No configured-storage prerequisite. |
| G03 | Configured writable storage and file metadata (`222d29ff`, `10091e4e`) | Current configured launch/VFS and approved storage assignment contract | Explicit mounts only; correct read/write/seek/stat/truncate and quota failure; unassigned paths cannot fall through to host writes. |
| G04 | Explicit virtual null devices (`e3458063`) | G03c configured namespace/copy owner and device assignment contract | Assigned null semantics, denied absent devices, correct path metadata and teardown. |
| G05 | Owned pipes and unified public descriptor ownership (`7f072f2c`, `3b00c1c8`, fixture correction `f1c6419e`) | G03, G04 | Correct alias/refcount/dup/close/CLOEXEC behavior, EOF/backpressure, invalid-pointer rollback and FD exhaustion. Reassess whether pipes and descriptor core can split coherently. |
| G06 | Configured child/exec lifecycle and death accounting (`ccd62ca1`, `8f768700`, `9279e71b`, `d693ae63`, `07c949db`) | G05; reconcile existing process PRs | Fork/exec/wait/signal/exit, failed exec retains FDs, CLOEXEC commits only on success, adopted children reaped once, repeated races leave no surviving tasks. Extract portable standalone fixes separately when main supports them. |
| G07 | Thread ownership and per-task transactions (`928dc9a1`, `e59e2e61`) | G06 | Shared FD lifetime, independent syscall transactions, clone failure rollback, nonleader/leader exit and supported thread exec constraints. |
| G08 | Owned readiness and local sequenced-packet channels (`75acd2be`, `210a8cca`, `3b27b370`, `e954010d`) | G01, G02, G05, G06, G07 | Real peers, nonblocking transitions, short I/O, readiness/deadlines, half-close and bounded capacity; nested compiler streams and cancellation. Split readiness/channels if independently usable. |
| G09 | Growable owned files, directories and basic path operations (`06d24ae2`, `32e2c85f`, `9ba6a1d4`, `d9490772`, `8752f708`, `37ca30cb`, `adc79f0a`) | G03, G05, G06 | Sparse growth, shared quota, directory cursors/mutations, relative paths and access probes; failed operations preserve data, identity and confinement. |
| G10 | Execution of owned ELF files (`c5ae1c5d`, `32ba41e9`) | G06, G09 | Valid ELF executes from owned backing, malformed input fails, executable identity is virtual, pins release at death and no host-path reopening occurs. |
| G11 | Owned scripts and bounded interpreter chains (`73593745`, `a08aff7a`, `cda76faa`) | G10 | Script FD lifetime, image/owned interpreter selection, recursion bounds, malformed/shebang failure and exec cleanup. |
| G12 | Owned file mappings (`868862bc`, `680d5081`) | G07, G09, G10 | Mapping offsets/permissions/aliasing, resize and private-copy behavior, nonleader mappings and cleanup; backing remains supervisor-owned. |
| G13 | Owned symlink traversal and mutation (`909399b4`, `aa8becf2`, `8b14578c`, relevant docs `d7f628cc`) | G09, G11; cross-check #218 overlap | Relative/absolute/dangling/loop cases; link-before-parent resolution, correct nofollow/final-entry mutation, failure preserves namespace. Allocate script docs back to G11. |
| G14 | Owned current directories (`6a45d8f8`) | G06, G09, G13 | chdir/fchdir/getcwd, relative paths across fork/exec and directory mutations; no host cwd leakage. |
| G15 | Shared hardlink inode identity (`9e2a39e8`) | G05, G09, G13 | Aliases share bytes/metadata and open-file lifetime; unlink/relink, quota charging and final close remain correct. |
| G16 | Unified image/owned traversal (`1da0ce13`, `93a1a56d`) | G04, G13, G14 | Cross-boundary links/parents/exec, assignment precedence and confinement; hidden `/dev` paths retain absent semantics. |
| G18 | Configurable bounded namespace capacity (`38fa14fd`) | G09 | Thousands of actual nonempty objects; checked allocation arithmetic, metadata/data accounting, configured exhaustion and shared-byte charging. |
| G17 | Private backing descriptors, public FD budgets and positioned writes (`53a2c9cc`, `d5357101`) | G05, G09, G12, G18 | Private reservations do not consume or raise public limits; lower operator limits work, pwrite respects offset/alias semantics and all resources close on failure. |
| G19 | Flush and quiesced export (`53dec6b3`, `48b26f8d`, relevant static fixes `c541a67f`) | G06, G07, G09 | fsync/fdatasync reach owned backing, live writers settle before successful export, failed workload/export cannot publish partial output. Allocate static fixes to their owning packets. |
| G20 | Private checkpoint control and event-driven waiting (`2b306eff`, `990fbb7a`) | G06, G07, G19 | Checkpoint acknowledgment follows consistent capture; opaque bounded control, malformed frames, dead peers, cancellation and simultaneous task events behave correctly. |
| G21 | Validated startup restore (`1d9e00e0`) | G09, G19 | Restore occurs before workload execution; reject malformed/traversing/oversized snapshots and occupied destinations without corrupting previous state. |
| G22 | Bounded archives with hardlink identity (`e3374593`) | G13, G15, G18, G21 | Export/restore preserves aliases and symbolic links; >2 GiB payload and 5,000 nonempty files; reject unsafe paths, conflicting entries, invalid forward aliases and inconsistent link metadata. |
| G23 | Owned file locks (`26784843`) | G06, G07, G15, G17 | Real simultaneous SQLite/Cargo clients contend correctly, locks release under close/exit, ownership follows defined descriptor/process semantics; no disabled-lock workaround. |
| G24 | Inode timestamps and archive mtime (`d21d26c8`) | G15, G22 | Explicit timestamps survive alias access/checkpoint/restart, invalid/no-follow cases are correct; document seconds-only persistence. |
| G25 | Vectored and chunked regular I/O (`88fa4749`, `7d556b4e`, `74835089`) | G05, G09, G17 | readv/writev bounds, short I/O, invalid buffers, offsets and partial effects; real APK/RPM and >1 MiB reads/writes complete without assuming pipes are regular files. |
| G26 | Explicit inherited process resource limits (`0c571cbb`) | G06, G07; upstream resource contract | AS exhaustion, affinity enforcement, fork/exec inheritance and invalid requests; state per-process virtual AS and allowed CPU subset accurately, not aggregate RSS/CPU time. |
| G27 | Configured virtual platform identity (`9cf265ba`) | Current identity view and agreed configuration contract | Raw ABI and real RPM tool report configured virtual identity; invalid/oversized values fail and no host uname leaks. |
| G28 | Bounded TCP device backend (`8ff83d64`, `539c1c77`, `9a70427b`) | Device assignment and approved network policy | Explicit endpoints, nonblocking connect/read/write, endpoint metadata, connection/byte budgets, revocation and cleanup. Backend-only proposal must be useful/testable on main. |
| G29 | Configured TCP syscall/readiness integration (`66de2880`) | G05, G06, G07, G08, G28 | Default-denied real connections; authorized HTTP/TLS/Git, refused endpoints, readiness, byte/connection exhaustion, cancellation/revocation; no host-network fallback. |
| G30 | Virtual root ownership on owned objects (`7cf2638a`) | G09, G13, G15, G25 | Actual cpio extraction with uid/gid zero or unchanged IDs; invalid IDs, absent objects and nofollow fail correctly; no claim of arbitrary Linux ownership. |
| G31 | Checkpoint/restore CI coverage (`5e20a65f`) | G20, G21 and fixtures included in those proposals | Actual runner builds and executes checkpoint/restore checks; static-and-unit still runs all existing modules. Prefer folding this small wiring into the feature that introduces the fixtures. |

[P04's source-derived dependency record](ginux-contributions/p04-dependencies.md)
explains removed false dependencies, required splitting inside G17/G25/G26/G27,
policy holds and the current-main standalone G05a correction. Source fixture
registration and generic documentation stay with each implementing change.

The exact upstream prerequisite for each packet is a **capability**, not all local
changes listed before it. G27 and G28 can start immediately after P05 when their
upstream contracts are settled. G01/G02/G28 may require maintainer policy resolution;
record the question and user-reviewed draft locally rather than sending it.

## Review and validation required for every eventual PR

Apply R01–R04 to **each actual extracted candidate**, including any additional
split from the table. Repeat affected checks after fixes. A combined integration
pass never substitutes for the candidate's own review.

- [ ] **R01 — Full source and contract review.** Read the entire proposed diff,
  relevant existing callers and failure paths. Check assignment authority,
  host/domain separation, bounds/overflow, reference and task lifetime, descriptor
  aliasing, cancellation, cleanup, unsupported operations, public configuration,
  docs and CI. Record concrete findings with severity and file/line evidence.
  Use a separate review pass after authoring; label independent reviewer coverage
  truthfully and never claim it from self-review. Fix all blocking findings.
- [ ] **R02 — Demonstrate the regression or capability.** Use a minimal Ginux-native
  reproducer. For bug fixes, show the regression fails on the exact upstream base
  and passes with the candidate. For new features, show the supported behavior
  and explicit failure boundaries without importing Recipe orchestration or policy.
  Preserve existing upstream unit tests; add useful focused regression coverage.
- [ ] **R03 — Validate the exact standalone revision.** Record base/head SHAs,
  commands, platform and output. Run current `make check test-unit`, required build
  targets and the actual upstream CI commands applicable to the change. Run
  affected real launch/native fixture suites with required privileges. Derive
  commands from that candidate's Makefile/workflow; do not assume every integrated
  test target already exists on main. Review the local macOS/Linux workflow split;
  Linux-local success is not a macOS Actions pass. All required checks must be
  green for the eventual PR head, or explicitly pending until remote CI exists.
- [ ] **R04 — Inspect independence and prepare the review package.** From a clean
  tree at current main, apply only the candidate and reproduce its checks. Verify
  commit range contains no other pending contribution, workspace policy, generated
  artifacts or unrelated edits. Inspect the final full diff after fixes. Prepare
  concise title/body with concrete behavior, why, exact testing and limits; include
  the patch, findings/fix record and reproduction commands. If dependencies are
  still local, label this candidate blocked for standalone delivery, not ready.

## Cross-candidate review and user checkpoint

- [ ] **F01 — Account for the complete queue.** Depends on P03/P04 and every
  retained G packet's local review. Every local hunk and old-fork-only change has
  a disposition; all 13 original areas below are covered. List real candidate
  count, replaced/upstream items, unresolved policy and prerequisite holds.
- [ ] **F02 — Review interactions.** Depends on R01–R04 for currently standalone
  candidates and local compositions for dependent packets. Apply ready candidates
  together in a disposable source tree; inspect shared files/contracts and run
  relevant combined task/FD/storage/network/checkpoint scenarios. Resolve order-
  dependent behavior and conflicts without silently expanding individual PRs.
- [ ] **F03 — Verify the consumer closure.** Depends on the final local composition.
  Run the current complete applicable managed Recipe suite on recorded binaries,
  plus affected actual consumer paths; rerun any fixes at their final revisions.
  The old 120/132 initial run plus 26 focused passes is not a new whole-suite pass.
  This validates the contribution set remains usable; keep this evidence local.
  Full ISO/VM/interactive PTY/kernel-build work is outside this queue unless a
  concrete affected behavior requires it and its scope is agreed.
- [ ] **F04 — Present everything for the user's review.** Depends on F01–F03.
  Present candidate patches, short draft bodies, exact base/head, review results,
  tests, remaining limitations and the publication dependency graph. Distinguish
  ready-on-main proposals from prepared proposals waiting on an upstream merge.
  Stop for the user to inspect and explicitly authorize the reviewed publication
  wave. Approval of this task list is not approval of unpublished future diffs.

## Publication only after the user's review

- [ ] **U01 — Refresh before delivery.** Depends on F04 approval and each selected
  candidate's upstream prerequisites being merged. Reread current main, discussion
  and existing PRs. Drop new duplicates, extract/rebase to current main, and repeat
  affected R01–R04 checks. A material change to approved scope returns to the user.
- [ ] **U02 — Open the approved standalone wave.** Depends on U01. Push only the
  reviewed candidate heads and open only approved PRs against main. Attach every
  created PR to this chat. Preserve human text in existing PRs/comments; never
  replace bodies from stale copies. No merge or extra maintainer message is implied.
- [ ] **U03 — Review the published diffs and make CI green.** Depends on U02.
  Verify GitHub's actual diff/head/base matches the approved local package, then
  inspect every required job, including platform-specific jobs. Fix causes, retain
  unit/static/integration coverage and rerun affected checks. Review each resulting
  head again. Pending, skipped, failed or unrun required checks are not green.
- [ ] **U04 — Reconcile the next wave after prerequisites merge.** Depends on the
  actual upstream merges, not merely green checks. Refresh the dependency map,
  repeat U01 and the user review boundary for changed/new candidates, then U02/U03.
  Continue until every retained candidate is submitted and reviewed or has an
  explicit recorded disposition. Upstream acceptance/merge remains the maintainer's
  decision; never mark those external steps complete on our own.

## Parallel lanes

This is a proposed work allocation, not authorization to start extra agents now.
Use separate candidate worktrees after branch/worktree authorization. Within the
integration tree, one physical writer owns each shared file; no concurrent hunk
editing of `launch_tasks.rs`, `fs_hooks.rs`, `descriptors.rs`, VFS, shared domain
configuration, Makefile or workflow files.

| Lane | Ready-work sequence | Coordination |
| --- | --- | --- |
| Runtime/FD | G01/G02 → G05 → G06 → G07 → G08 → G17 → G25/G26 | Take G03/G04 contracts from filesystem/device owners; release shared dispatch/task files explicitly. |
| Filesystem/storage | G03 → G09 → G10/G11/G12 → G13 → G14/G15 → G16/G18 → G19/G20/G21 → G22/G23/G24/G30 | Start only after listed table prerequisites; own archive/VFS changes and coordinate FD joins. |
| Devices/identity/network | G04, G27, G28 → G29 | Backend/policy work can proceed while runtime primitives are prepared; one owner applies shared config/dispatch joins. |
| Review/integration/docs | P01–P05, then R01–R04 as packets finish, G31, F01–F04, approved U tasks | Read-only review can overlap writing; final review waits for a stable SHA. Serialize expensive full builds/runtime runs to avoid resource contention. |

These lanes optimize preparation time; upstream merge dependencies still govern
publication. Do not keep reviewers idle waiting for every packet to finish.

## Coverage of the original 13 areas

| Original candidate area | Tasks |
| --- | --- |
| Owned cwd / hardlinks | G14, G15 |
| Image/owned traversal | G16 |
| Scalable storage / archive identity | G17, G18, G22 |
| SQLite/Cargo locks | G23 |
| Timestamps | G24 |
| TCP / readiness | G28, G29 (G08 prerequisite) |
| Private/public FD budgets | G17 |
| Vectored / bounded I/O | G25 |
| Process bounds | G26 |
| Virtual identity | G27 |
| Duplicate task-death accounting | G06 |
| Virtual root ownership | G30 |
| Checkpoint/restore CI | G31 |

## Checkpoint

User authorization: “make a goal for yourself to do one task at a time until finish”.
Run preparation serially through F04; stop before publication for user inspection.
The parallel-lane section above is available scheduling context, not the active
execution mode. No extra agents are started.

P01 complete. [Exact inventory](ginux-contributions/p01-inventory.json) records
all 12 registered Ginux worktrees, cached local/remote refs, statuses, tracked-diff
hashes and both stashes. The integration remains clean at `7cf2638a`. The old
combination worktree's staged VFS edits and all unrelated files are preserved.
No branch was created/switched and no source was modified.

Recipe has advanced separately from the previous `022e91c` validation baseline to
`4056374758ba2de8517b2faf3438c64380611992`; its untracked audit/artifact outputs
and the superproject's existing pointer difference are preserved. F03 must use
explicitly recorded current revisions rather than silently reusing old results.

P02 complete. [Upstream refresh and policy record](ginux-contributions/p02-upstream.md)
pins current main `3f2fe7df`, confirms #216–#218 merged and #221 newly merged,
and records the holds on #207/#179 and transfer clamping. Open #224/#226 overlap
with process cleanup/copyout work. No existing PR was changed. Git fetch changed
cached refs only; integration source and HEAD remain intact.

P03 complete as inventory/classification. [Reconciliation](ginux-contributions/p03-reconciliation.md)
and [catalog](ginux-contributions/p03-catalog.json) account for66 integration
commits,49 extra-worktree commits,59 changed paths and261 assigned diff hunks.
Full review is still pending for each extracted candidate.

A direct VFS regression confirms duplicated reads incorrectly keep separate
offsets on both upstream3f2fe7df and integration7cf2638a, and passes on old fix
b500f859. Preserve this as a standalone G05 subpacket. The other review fixes have
semantic dispositions, including fixture assertions to retain. Older prototype
and dirty/stashed work is preserved and mapped to its current owners.

The upstream merge preview identifies conflicts in supervisor.rs and VFSlib.rs;
new #221 helpers/fixtures and #217/#218 changes must be preserved. No source/index/
branch/remote was modified by the preview or regression probes.

P04 complete. [Dependency and extraction boundaries](ginux-contributions/p04-dependencies.md)
correct G17/G18 ordering, separate private checkpoint control from workload
channels, remove restore's false checkpoint prerequisite, and identify G05a as
independent on current main. G02's clocks are native global clocks, not a virtual
timer implementation. Relevant policy restrictions remain explicit.

P05 complete. The [frozen package plan](ginux-contributions/p05-package-plan.json)
records the original32 rows plus extracted subpackets, exact source commits, dependencies, acceptance and R01–R04
requirements. It is an execution manifest, not proof of extraction or completed
review. Use disposable /tmp source snapshots; no branch creation/switch is needed.

G01's existing held PR and G02's local policy delta have explicit
[review/hold dispositions](ginux-contributions/policy-held-packets.md). G02 still
needs standalone interrupted-sleep/runtime acceptance; it is not marked passed.

G05a is fully prepared and locally reviewed/validated as a standalone candidate
`bb2c82bf`, with current upstream3f2fe7df as its sole parent. [Review/evidence](ginux-contributions/g05a-review.md):
three baseline failures become passes,15 VFS/68 total unit tests pass, and all
required Linux launch/native/process/setup commands pass in both the candidate
and an independent clean main-plus-patch source export. Draft/patch/logs live under
`.artifacts/ginux-contribution-preparation/G05a/`. macOS remote CI is pending.
No Ginux branch/index/worktree/remote was changed; only local commit objects and
source exports were created.

G03a is a second independent candidate, extracted from the existing G03 donor:
correct the mkdirat syscall for an absent configured workdir. [Review/evidence](ginux-contributions/g03a-review.md)
records exact upstream failure, candidate0b563cf8,65 unit tests and all required
Linux runtime checks passing in both candidate and clean-applied source. Its sole
parent is upstream3f2fe7df; patch/draft/logs are retained with the package. The
larger G03 feature is still incomplete.

G03b's VFS foundation is extracted, corrected and locally validated at785bfb9d
on the explicit G05a prerequisite. [Review/evidence](ginux-contributions/g03b-review.md)
records four failing original-donor cases,24 passing current VFS tests,77 total
unit tests and both complete Linux runtime replays. Its patch is retained in
this superproject's docs alongside G05a/G03a. It is **not standalone-ready** until
G05a merges upstream; the local composition must not be published as a stack.

G03c's runtime adapter is locally reviewed/validated atb36e0752 on composition
603391c6 (G05a/G03b plus G03a). [Review/evidence](ginux-contributions/g03c-review.md)
records four initial-adapter path failures, their corrections, scratch restoration,
metadata flags,77 unit tests and all applicable Linux runtime checks passing twice.
The source patch is retained downstream. Minimal image lookup/native path rewriting
moves forward from G16 because assignment isolation requires it; later G16 joins
remain pending. G03's three extracted parts are locally prepared, with explicit
upstream prerequisite waits. No publication or existing-PR modification occurred.

G04 is locally prepared at5036b7e8 on the explicit G03c prerequisite.
[Review/evidence](ginux-contributions/g04-review.md) records the original donor's
hidden-device alias failure, corrected empty-path descriptor routing,77 unit tests,
all applicable Linux runtime suites in both candidate/clean sources, and final
null-I/O boundary checks. The full patch is retained downstream. G04's actual
runtime dependency is shared configured namespace/copy code, now explicit; it is
not standalone-ready before G03c and its prerequisites merge.

G05 is locally prepared at8c1a579a on G04, with pipes/public descriptor ownership
kept coherent to avoid a temporary public backend-handle ABI. [Review/evidence](ginux-contributions/g05-review.md)
records cross-owner aliases, failed-open reservation rollback, translated openat
register restoration, pipe copy/atomicity/EOF/SIGPIPE/parked termination,77 unit
tests and all applicable Linux suites passing in candidate and clean replay.
The source patch is retained. Held G01/G02 code and future task/network joins
are excluded; core policy remains unchanged. Standalone delivery waits for G04.

G06 upstream refresh confirms #224 clone-TID and #226 tracee teardown remain
open/unchanged with no discussion comments. Their main base is still3f2fe7df;
the open-PR list also retains held #207. Raw read-only snapshots are retained
with G06a. Do not duplicate those native-process contributions.

G06a is an independent process-table correction atb2347523, sole parent3f2fe7df.
[Review/evidence](ginux-contributions/g06a-review.md) records three baseline failures
and68 passing full unit tests plus all required Linux checks in candidate and
clean replay. It reuses current typed wait/status APIs and adopts only after the
parent's last running thread, including signal exit and live-init-thread cases.
No broader configured child/exec capability is claimed by this standalone fix.

G06b is locally prepared at `8c783e71` on explicit composition `5cb0370f`
(G05 plus G06a). [Review/evidence](ginux-contributions/g06b-review.md) records
strict child-TID/wait-status copyout, fault-still-reaps semantics, image-interpreter
ownership checks, native exec/CLOEXEC transactions and adopted-zombie handling.
82 full unit tests and all applicable Linux suites pass in candidate and clean
patch replay, plus 20 task/exec repetitions. All 168 tracked paths match. Current
upstream native helpers/tests are unchanged; #224/#226 are not duplicated. G06's
local preparation is complete through its two subpackets; standalone delivery
still waits for actual prerequisite merges. The source patch/draft/logs/hashes
are retained downstream; nothing was published.

G07 is partially extracted and retained as an explicitly unready snapshot.
[Review/evidence](ginux-contributions/g07-review.md) records83 passing unit tests
and11 selected ordinary thread scenarios, with qfault/x-path security checks
excluded. A previously recorded shared-path race is unresolved and handed to
DaybreakBlue in the downstream security record. G07 is not complete or ready;
G08 remains held by G01/G02 policy and G07. No security acceptance is inferred
from functional results, and no DaybreakBlue dispatch has occurred.

G09a separates VFS byte storage atd29381c7 on G03b, preserving the inline
default and existing offsets/whiteouts. [Review/evidence](ginux-contributions/g09a-review.md)
records80 passing unit tests, source/clean build checks, four selected native
smoke scenarios and the fixed O_TRUNC capacity failure. Full runtime CI remains
pending; it is not standalone-ready before its VFS prerequisites merge.

G09b adds anonymous runtime growth at297ad8dd on compositionccd42e31
(G06b/G09a). [Review/evidence](ginux-contributions/g09b-review.md) records85 units
and growth/bounds/quota runtime scenarios passing in both source and strict
clean replay, with169 tracked paths matching. The feature fails on its exact
prerequisite at the old64 KiB ceiling. Full CI/security evidence remains pending.
G09a/G09b are review subpackets intended together for one useful growth proposal;
no abstraction-only PR or stack is implied. Memfd identity moves to G12, its
actual consumer. G09 directories/relative paths/access remain incomplete.

G09c's VFS directory owner is retained at94fbfc2d on G09a.
[Review/evidence](ginux-contributions/g09c-review.md) records84 units/31 VFS tests,
checked source/clean builds,150-path strict replay and the corrected dot-entry
mutation failure. Directory runtime integration, full CI/security and actual
upstream independence remain pending. Pair this review subpacket with its
runtime adapter before proposing a useful contribution.

G09d's directory/cwd runtime is retained at85b53fe1 on G09b/G09c composition
300fe207. [Review/evidence](ginux-contributions/g09d-review.md) records89 units,
source/clean builds,171-path strict replay, ordinary directory/cwd scenarios and
storage growth/bounds/quota. G09b fails the new ABI fixtures; the final join passes.
Access/readlink and security probes remain unrun for DaybreakBlue, including the
view-probes run rejected by automatic approval review. G09c/G09d must travel
together with their enum/runtime errno join; the intermediate composition must
not be published. G09 source/selected functional preparation is retained across
four review subpackets, with explicit security/full CI/upstream holds. Memfd
identity moves to G12; unsupported /dev/fd reopening tests move to their later
executable/descriptor owner. No PR or policy text entered Ginux.

G10 source and selected ordinary validation are retained at `14866b1d` on
G09d85b53fe1. [Review/evidence](ginux-contributions/g10-review.md) records91 units,
174-path strict replay, six ordinary owned-ELF scenarios with an 83,288-byte compiled
fixture and image-cwd/storage controls passing in source/clean exports. It fixes
pinned-whiteout reuse and preserves interpreter authority through the existing
metadata reader. The exact prerequisite fails the focused valid-exec scenario.
Readlink/private-FD/seal/malformed-input and shared-argument security acceptance
remain with DaybreakBlue; G10 is explicitly security-held and not publication
ready. Full CI/current-main independence remain pending. Internal routing text
was also removed from the one local agent-authored public draft that contained
it; no remote text was edited.

Current goal-turn classification: **progress**. The full goal remains active;
security-held G07, later packets, policy acceptance, interactions and Recipe
closure remain incomplete. F04 remains the user inspection stop.

G11 source and ordinary validation are retained at `0798d991` on
G1014866b1d. [Review/evidence](ginux-contributions/g11-review.md) records92
units,177-path strict replay and14 ordinary script scenarios, plus ELF/cwd/storage
controls passing in source/clean exports. Current metadata/component/FD/task owners
replace donor lexical parsing, obsolete offsets and thread-group assumptions.
Full/default security fixtures and CI wiring are retained; low-level stack/private
FD/chain authority verification remains unrun with DaybreakBlue. The exact G10
binary fails the focused valid-script case. G11 inherits the security hold and is
not standalone/publication-ready. No Ginux branches, checkouts/indexes or remotes changed.

Exact next task: **G12 — owned mappings**. Inspect32e2c85f (memfd backing deferred
from G09),868862bc/680d5081 before extraction on G110798d991. Trace actual
mapping/backing/task dependencies; preserve growable-storage quotas, VFS aliasing,
exec pins, new script cleanup and current namespace resolution. G07's unresolved
shared-argument/thread ownership security hold remains: do not treat nonleader
mapping acceptance as satisfied or import the unready G07 snapshot as validated.
Prepare independent source/ordinary mapping behavior where possible and explicitly
retain held joins/verification. Write low-level map/injection/protection/confinement
security work down for DaybreakBlue before doing it. KeepG01/G02/G08 policy holds,
all downstream ownership boundaries and the F04 publication stop intact.

Active skill: `hdae-development`, discovered at
`/home/vince/.codex/worktrees/ginux-pr-combination-check/ginux/.agents/skills/hdae-development/SKILL.md`.
Reload it and its required references after a context shift; then read applicable
instructions and this checkpoint. Direct user scope and the superproject's
proportional-process instructions prevail; this plan adds no automated framework.
