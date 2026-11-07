# Ginux contribution task list

Updated: 2026-11-07. Status: independent local verification complete; required clock/security acceptance pending.
The authoritative ownership split below gives Claude all upstream PR work. Codex's earlier
packet/publication queue remains historical; its current work is F03 and the local fixes Recipe
needs. No Codex push, PR, upstream comment or security-agent dispatch is authorized.

This is the remaining contribution queue for the completed local Ginux→Recipe
implementation. [Local validation](recipe-local-validation.md) owns the previous
runtime evidence; [the completed Recipe plan](recipe-simplification-implementation-plan.md)
owns T01–T27. Neither is reopened by this document. Keep this downstream planning
record in this superproject, outside Ginux contributions.

## Migration-only publication exception

The user's 2026-11-06 request to commit and push everything for a computer migration
authorizes origin backups of current development state and recovery artifacts.
It does not resume the paused implementation goal, approve any Ginux PR, change
readiness claims or authorize upstream maintainer messages. See
[the migration recovery record](development-migration/2026-11-06/README.md).
Earlier no-push instructions continue to govern contribution delivery after this
specific migration backup.

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
| G22 | Bounded archives with hardlink identity (`e3374593`) | G13, G15, G18, G17b (memfd scale), G21 | Export/restore preserves aliases and symbolic links; >2 GiB payload and 5,000 nonempty files; reject unsafe paths, conflicting entries, invalid forward aliases and inconsistent link metadata. |
| G23 | Owned file locks (`26784843`) | G06, G15; G07 thread join and consumer capabilities at F02/F03 | Real simultaneous SQLite/Cargo clients contend correctly, locks release under close/exit, ownership follows defined descriptor/process semantics; no disabled-lock workaround. |
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

- [x] **F01 — Account for the complete queue.** Depends on P03/P04 and every
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

G12's per-process mapping portion is retained at `83072ec3` on
G110798d991. [Review/evidence](ginux-contributions/g12-review.md) records92
units,178-path strict replay, two valid mapping scenarios and storage/ELF/script
controls passing in source/clean exports; exact G11 fails the mapping fixture.
Memfd identity from32e2c85f now travels with its real mapping consumer. G12 remains
partial: G07 thread ownership/nonleader fixture join, security/full CI and actual
upstream independence remain pending. The thread-only fixture patch is preserved
separately, and low-level mapping checks are written down for DaybreakBlue.

G13 source/ordinary preparation is retained at `5c3ae8e5` directly on
G110798d991, without unrelated G12 ancestry. [Review/evidence](ginux-contributions/g13-review.md)
records95 units,177-path strict replay, ordinary link I/O,16 script modes and
ELF/storage/directory controls passing in source/clean exports. The shared walker
is unchanged; reproduced whiteout and mkdir-capacity regressions are corrected.
The exact G11 binary rejects the link fixture. Archive/restore joins remain with
G19/G21/G22; runtime readlink/confinement/adversarial checks remain with DaybreakBlue.
G13 inherits security/full CI/upstream prerequisite holds and is not publication-ready.

G14 per-process cwd source/ordinary preparation is retained at `ec6f9c48` on
G135c3ae8e5. [Review/evidence](ginux-contributions/g14-review.md) records95
units,177-path strict replay and ordinary chdir/fchdir/getcwd, fork/exec and directory
identity, plus32 failed fork reservations and four one-slot child reclamations.
Current link/script/ELF/storage/directory controls pass in source/clean exports;
exact G13 fails the owned-cwd fixture. Thread-shared cwd stays with G07's held
join, and security/full CI/current-main independence remain pending. No publication.

G15 source/ordinary preparation is retained at `8ab24d4b` directly on
G135c3ae8e5. [Review/evidence](ginux-contributions/g15-review.md) records98
units,177-path strict replay, shared hardlink metadata/bytes, alias executable
identity and final-descriptor quota reclamation, plus script/ELF/storage/directory
controls passing in source/clean exports. Donor executable-name and initial file-
count regressions are reproduced and corrected. Exact G13 rejects the valid-link
fixture. G19/G22 export joins and G14/G12 interactions remain explicitly assigned;
security/full CI/upstream prerequisites remain pending. Nothing was published.

G16 source/ordinary preparation is retained at `4682b0b0` on G14ec6f9c48.
[Review/evidence](ginux-contributions/g16-review.md) records95 units,177-path strict
replay, three new ordinary cross-path/FD/cwd/exec modes and five script-interpreter
alias modes, plus existing controls in source/clean exports. The current walker,
image lookup and native rewrite owners are reused; canonical script targets with
shebang separators now fail explicitly. Exact G14 rejects the new valid-cross
fixture. G15/G12/G07 joins and security/full CI/current-main independence remain
pending. No Ginux checkout, branch/index, remote or human text changed.

G18 source/ordinary preparation is retained at `f58ba480` on G09d85b53fe1.
[Review/evidence](ginux-contributions/g18-review.md) records91 units,171-path strict
replay and5000 actual nonempty files with readback, namespace exhaustion and last-
close reclamation, plus storage/directory controls in source/clean exports. Default
capacity32 is retained; explicit object/data limits charge rounded metadata before
data. The exact old runtime fails the same scale fixture at its32-slot bound.
Restore/archive and G10–G16 combined joins remain assigned to their owners/F02;
security/full CI/current-main independence remain pending. Nothing was published.

G17a pwrite is locally prepared at `94baebbe` on G09d85b53fe1.
[Review/evidence](ginux-contributions/g17a-review.md) records89 units,171-path strict
replay and ordinary positioned/alias/sparse/append/quota/directory checks passing
in source/clean copies. Exact G09d rejects pwrite. This part has no G18/mapping
prerequisite. G17 remains partial; security/full CI/upstream holds remain.

G17b is locally prepared at `6fcdd618` on composition `f41ed1b0`
(G12/G18). [Review/evidence](ginux-contributions/g17b-review.md) records94 units,
178-path strict replay, public/native limit8 with20 private-backed files, rejected
insufficient hard capacity and5000 nonempty files launched under soft64, plus
mapping/script/ELF/storage controls in source/clean exports. Exact prerequisite
fails both new limit/scale fixtures. The outdated mapping fixture assertion is
corrected in its prerequisite owner and retained for F01/F02. G17a/G17b ordinary
preparation is retained; inherited security/full CI/upstream holds remain pending.

G19 per-process source/ordinary preparation is retained at `a36e7644` on
G1283072ec3. [Review/evidence](ginux-contributions/g19-review.md) records92
units,179-path strict replay and ordinary flush, binary/long-path/mode export,
publication-after-completion, child settlement, failed workload and early/late
config cleanup in source/clean exports, plus mapping/ELF/script/storage controls.
The donor's direct-final-file output is replaced with private staging and no-
overwrite publication after sync/close. A reproduced early-rejection staging leak
is fixed at common completion. Exact G12 rejects flush. G07 thread/security,
G13/G15/G18/G21/G22 joins, full CI and actual upstream independence remain pending.

G20 per-process source/ordinary preparation is retained at `7afa833d` on
G19a36e7644. [Review/evidence](ginux-contributions/g20-review.md) records92
units,181-path strict replay, two valid fragmented captures plus final export,
normal peer cancellation and task events, plus export/mapping/exec/storage controls
passing in source/clean exports. Shared archive staging now has distinct capture
identities. Private signalfd/poll replaces blocking task wait while wait4 retains
status ownership. Essential G31 checkpoint CI wiring is folded into this packet;
full/default/remote runs, security, G07 threads and upstream independence remain
pending. Exact G19 rejects the new option. No publication occurred.

G21 source/ordinary preparation is retained at `33bf08a7` directly on
G19a36e7644, without a G20 dependency. [Review/evidence](ginux-contributions/g21-review.md)
records 92 units, 181-path strict replay and valid startup reconstruction, repeated
fresh launches, binary/empty/multi-chunk data, modes, long paths and child-before-
parent ordering, plus export/mapping/exec/storage controls, in source/clean copies.
The fixture now respects existing short reads; no G25 behavior is imported.
Essential G31 restore wiring is folded into its introducing packet. Exact G19
exports the archive but rejects restore. Security/full CI/upstream independence
and G18/G13/G15/G20 combined joins remain pending; no publication occurred.

G22 source/ordinary preparation is retained at `7b2adc4f` on reviewed
G13/G15/G18/G17b/G21 composition `9caccb3d`. [Review/evidence](ginux-contributions/g22-review.md)
records100 units,181-path strict replay and link/alias/long-target identity,
5000 nonempty files and a complete2 GiB+1 restore/export with every payload byte
compared by streaming digest, plus existing controls, passing in source/clean.
G17b supplies this memfd composition's required private FD budget; no G20/thread/
held policy code is imported. Source review fixed silent symlink-mode changes.
G19 atomic export and G21 startup cleanup remain. Existing restore CI wiring
covers the new fixtures. Security/full CI/upstream independence and remaining
F02 interactions are pending; nothing was published.

G23 per-process source/ordinary preparation is retained at `14c33321` directly
on G158ab24d4b. [Review/evidence](ginux-contributions/g23-review.md) records98
units,178-path strict replay and real record/flock contenders, audit-confirmed
blocking/wakeup, supported SIGKILL cancellation, exec/CLOEXEC/final-close lifetime
and existing process/storage/exec controls passing in source/clean. Code uses
existing G15 ownership without G17 or G07 machinery. The full packet remains
partial: G07 thread/security, actual simultaneous SQLite/Cargo with their combined
consumer prerequisites at F02/F03, full CI and upstream independence are pending.
Do not substitute these fixtures or historical integrated logs for that consumer
acceptance. Existing Make/CI wiring includes the new cases; no publication.

G24 explicit-time source/ordinary preparation is retained at `b33fbd24` on
G227b2adc4f. [Review/evidence](ginux-contributions/g24-review.md) records100
units,181-path strict replay, nanosecond/stat/statx/OMIT and alias/descriptor
updates, contained symlink metadata, whole-second archive restart and existing
large/scale/storage/exec controls passing in source/clean. A valid futimens
null-path adapter failure is fixed at the shared path owner. Atime/nanoseconds
are not persisted and IO does not update times implicitly. Ordinary NOW/null-times
calls also pass; native-realtime/#179 policy, security/full CI and upstream independence remain pending; no consent is inferred.
The absent network Stat field join is assigned to G29. G23's actual application/
thread closure remains pending for F02/F03; nothing was published.

G25 owned-vector/chunk source preparation is retained at `9afd60cc` directly
on G17a94baebbe. [Review/evidence](ginux-contributions/g25-review.md) records89
units,171-path strict replay and vectors/short transfers/alias offsets, complete
2 MiB+37 byte-checked payload, positioned writes, quota prefixes and pipe atomicity/
EOF plus existing controls passing in source/clean. Final large-read fixture
allocates its entire request. Native descriptor clamp74835089 is excluded and
policy-held under #178/#163. The full task remains partial: actual APK/RPM,
G07/security, G08 channels, full CI and upstream independence remain F02/F03 or
publication prerequisites; fixtures do not replace application acceptance.

G26 is split into source packets G26a `1d0824e6` on recorded main and G26b
`2ed520c4` on G06b/G26a composition `687eb942`.
[Review/evidence](ginux-contributions/g26-review.md) records65/82 units and strict
clean replay, configured/lower-operator AS limits, exact operator CPU subsets and
process fork/exec inheritance plus ordinary lifecycle controls passing. Kernel
limits are per-process AS/eligible CPUs, not aggregate RSS or CPU time. Existing
bootstrap/descriptor/CLOEXEC owners and full default coverage are preserved.
Full G26 remains partial: resource contract, G07 nonleader/TGID target join,
security/full CI and fresh upstream independence are pending; no publication.

G27 source/ordinary preparation is retained at `1eb3e189` directly on recorded
main, without G06/G26/storage prerequisites. [Review/evidence](ginux-contributions/g27-review.md)
records65 units,150-path strict replay, complete390-byte ABI/padding/max-valid ID,
actual RPM4.19.1.1 identity queries under two domain names and ordinary valid
native wait-status copy control passing in source/clean. IdentityView and the
existing launch/copy owners provide the behavior; core/native defaults and
sysinfo/#219 remain unchanged. Exact main lacks the ABI and its RPM query fails.
#179 identity policy, security/full CI, fresh upstream acceptance and G06/F02
runtime-owner join remain pending. No publication occurred.

G28 backend-only source preparation is retained at `0a3cf5f5` directly on
recorded main. [Review/evidence](ginux-contributions/g28-review.md) records150-path
strict replay, static/build and67 selected routine units passing in source/clean,
including real IPv4/IPv6 metadata, synchronized nonblocking I/O, half-close,
cumulative budget and close/drop/normal revoke. Linux-only device coverage now
has a Linux CI invocation with the correct unit flags. Two new donor policy/
security tests remain enabled but unrun; unfiltered make test-unit/full CI is
pending, not green. Network contract/security and fresh upstream review still
hold delivery. No G29 guest networking, remote mutation or publication occurred.

G29 tracing exposed its actual G08 readiness prerequisite. Minimal G08a
`fe27c693` on G06b is now source-prepared separately: [review/evidence](ginux-contributions/g08a-review.md)
records82 units,170-path strict replay and five ordinary poll/peer/alias/EOF/timer
modes passing in source/clean. Poll layouts are snapshotted and internal arithmetic
checked. No thread/socketpair/FIONBIO, guest clock or shared native poll policy is
imported. G08's remaining scope/security/#179/#207/full CI remain open.
G29 `26fbfef2` is retained on G08a/G28 composition `2ba7c894`.
[Review/evidence](ginux-contributions/g29-review.md) records173-path strict replay,
84 selected routine units and five ordinary TCP/five poll modes passing in
source/clean. Receive tails and failed connection state are now retained; shared
vector copies and frozen deadlines replace the donor's divergent handling.
Actual curl IPv4/IPv6 HTTP and verified local TLS pass. Actual Git fails before
transport at absent getcwd; preserve for G14/F02/F03, without a workaround or
false closure. Three default-enabled policy/security units/full security modes,
G07/G20/G24/G25 joins, network policy and full CI/upstream independence remain
pending. G29 is partial, not publication-ready. No publication occurred.

G30 `2186548d` is retained directly on G15.
[Review/evidence](ginux-contributions/g30-review.md) records98 units,177-path strict
replay and ordinary owner/path/FD/nofollow/empty-path/alias metadata plus storage
controls passing in source/clean. Actual GNU cpio2.15 extracts all8229 payload
bytes, hardlink identity and symlink metadata with UID/GID0; the same archive and
wrapper fail ownership on exact G15. Empty-path shared routing was reproduced
and corrected. No G25/G24/G14 ancestry or ownership-suppression flag is needed.
Security/full default runtime/remote CI/current-main independence and actual
combined package/Recipe closure remain pending. No publication occurred.

G31's one-line CI change is fully assigned to the introducing G20/G21 owners;
[review/evidence](ginux-contributions/g31-review.md) verifies exact workflow/Make/
runner/fixture bytes and wiring dry runs in both source/clean exports. Required
make test-unit/all7 packages remain, ordinary runtime receipts already pass on
those exact heads. No separate wiring-only patch/PR is needed. Full/default,
combined owner and remote CI evidence remains pending; nothing published.

F01's inventory/dispositions are accounted: [review/evidence](ginux-contributions/f01-review.md)
verifies37 patch/head hashes, all66 integration commits,49 old-fork dispositions,
2 reconciliation merges,6 preserved dirty/stash records and every original261
hunk. All13 original areas have source/disposition coverage. Source checks on37
prepared heads plus unready G07 preserve all65 original named tests, all7 unit
package commands/required CI, native fixture/caller bodies and exact native core
contract. G08b fixes the source gap, G31 is folded, and no old native helper/test
reversions or accidental native clock/poll/getcwd grants are imported.

This completes accounting only. G07 remains unready with DaybreakBlue; policy,
security/full CI/current-main independence, actual compiler/SQLite/Cargo/APK/RPM/
Git and complete current Recipe acceptance remain open.37 patches is not a PR
count, and no publication is authorized.

F02's37-patch per-process union is retained at `dfebda7c29b628fc41f50635b07e8cb48d6f883a`:
[review/evidence](ginux-contributions/f02-review.md). Source/clean replay matches189
paths and identical binary; make check/build,102 selected units and eleven ordinary
integration groups pass, including complete>2GiB/5000-file restore. Actual curl
IPv4/IPv6 HTTP/TLS, Git HTTP/TLS and cpio pass in both. Backend coexistence, packet
vectors and the corrected mixed task/resource selection pass. All65 original
unit names/required CI/native source remain. Three security/policy unit tests,
full default fixtures and remote CI remain unrun. This is partial F02, not green
full CI or full compiler/Recipe acceptance.

G12 fixture correction is assigned to `8bb50aaa50e80428ff417654d35affd468255775` with92 units/178-path strict
replay and ordinary mapping/storage passes; earlier candidate evidence remains
historical. G22's reviewed base already contained the archive alias/metadata join;
it is explicitly retained for future extraction rather than silently assumed.
Every original policy/security/current-main independence hold remains. No
publication occurred.

F02 also passes the actual SQLite two-process contention/commit/data-integrity
consumer in source/clean. F03 ordinary verification has started on a separate
fresh Recipe snapshot `bde746221988094d4a0b80bb46808beab447e3dc` plus hashed tracked dirty overlays;
[progress/evidence](ginux-contributions/f03-review.md) records135 current scenarios,
new immutable binaries and1/1 first managed case passing. The12-case ordinary
batch was interrupted at the user’s pause; inspect saved state/logs on resume. Full suite,
compiler/Cargo/APK/RPM and security acceptance remain open.

**Historical pause, superseded by explicit resume below.** The active F03 batch was stopped; partial results
are7 passed,4 failed and1 interrupted/unreported, plus the first ordinary
probe passed. These are not full-suite results. The C dependency/Bash paths expose
reader-thread creation ENOSYS while G07 is excluded; logs preserve all failures.

The saved resume action was: inspect F03 state/logs and the
remaining case; continue authorized ordinary consumer work without disguising
G07/security dependencies. The supplementary candidate-integrity refresh stopped
on a missing source_patch key in an older manifest row; it made no candidate
changes or proof claim. Preserve current source/candidate/Recipe snapshots and
concurrent live edits. G07 and all security/policy/full CI/current-main holds
remain. F04 inspection precedes any publication; no branches/refs/pushes/PRs/
maintainer messages are authorized.

### Destination resume — 2026-11-06

The user explicitly resumed the sequential goal through final review and
verification, stopping at F04 for inspection with no publication. F03 remains
active and partial. The complete saved log has now been read: all four failures
report command-output reader thread creation ENOSYS, consistent with excluded
G07. The previously interrupted state-restore case now passes 1/1 in 328.38s on
the destination-built harness, covering all four invocations. Thirteen fresh
destination cases have completed: ten pass and three fail. The Bash child
case fails with the same reader-thread ENOSYS; two valid tar cases also fail
because the existing progress spinner needs a thread. A valid zip case was
interrupted when that batch was stopped and remains unverified. The retained-link
case was also interrupted at the upstream-refresh stop below; neither is a pass.
Passing cases additionally cover custom readiness,
128 KiB file bytes, directory copy/move/cleanup, lifecycle event order, current
API initialization and 5,000-entry publication. No failure or interrupted case
has been counted as passing. See [F03 evidence](ginux-contributions/f03-review.md).
The additional retained-dependency-state case passes all three invocations,
including persisted discoveries from reused readiness checks. Changed source/
input identity and default application-state placement/reuse also pass.

Initial strict replay verified all 37 original retained patches against their recorded
parents/trees, including corrected G12. All 189 F02 tracked paths match the
recorded combined head. The paused Recipe snapshot matches all 153 current
tracked paths except its separately normalized standalone lock. The six restored
worktrees remain untouched; their index/status fingerprints are retained with
the new F03 evidence. That initial F03 continuation changed no Ginux candidate;
the subsequent current-main refreshes below supersede individual retained heads.

The revised ordinary queue is stopped. Exact selection, receipts and the reconciled
135-case status catalog are in
`.artifacts/ginux-contribution-preparation/F03/resume-2026-11-06/`.
**New prerequisite evidence:** upstream main is now
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`, 52 commits beyond the recorded base,
with substantial native runtime and domain/network changes from #228/#229.
The six directly based patches apply cleanly, but that does not prove their new
source/runtime acceptance. Exact source and context previews are retained in
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/`.
Earlier P02/P03 and candidate results remain historical records; current-main
reconciliation is required before final closure.

Exact next action: refresh R04's current-main prerequisite serially, continuing
with upstream PR#231 contract/audit reconciliation against newly observed
main `ff926d0c228b71d55d0f2f0909dd86ec62197c8a`, before G11 or any final
current-main claim. Preserve the completed8a475e8b-composition evidence. G07's preserved
unready snapshot still needs DaybreakBlue correction/validation; do not import it. Local dependent
preparation stays blocked for standalone delivery until its prerequisites
actually merge upstream.
G05a is refreshed at
`936d0eb7a9b46e58ade5219705c12d3d476dfae9`, solely on observed main `8a475e8b`:
three exact-base regressions fail, all 76 units/static/build and eight ordinary
controls pass in source and strict 187-path clean replay. The one-file VFS diff
is unchanged; both binaries match. Its retained patch/manifest now describe the
new head. [Review](ginux-contributions/g05a-review.md) keeps full default runtime
and macOS/remote CI pending. G03a is refreshed at
`c28dd1321f024ec6b5d2087fd489457f31f20e2c`, also solely on `8a475e8b`:
the exact base exits 2 for the missing workdir; source and strict clean replay
create it and exit 0. All 73 units/static/build and nine ordinary controls pass
in each export; the two-file +4/-1 scope and upstream CI coverage are preserved.
[Review](ginux-contributions/g03a-review.md) retains all unrun default/runtime/
macOS checks. G06a is refreshed at
`01f22e0615e54fdcf3b85f8cc3b3542fb88b1a27`, solely on `8a475e8b`:
all three exact-base adoption regressions fail; all 76 units/static/build and
eight ordinary controls pass in source/clean, preserving new upstream job-control
coverage. The one-file +118 scope is unchanged and binaries match.
[Review](ginux-contributions/g06a-review.md) keeps full default/runtime/macOS
checks pending. G03b is refreshed at
`5fb12ab4a56efd365d7d451d181f6e2a41c9d31d` on refreshed G05a:
all 85 units/static/build and eight ordinary controls pass in source/clean,
with all 187 tracked paths and both binaries matching. Its one-file +425/-6
delta is unchanged. [Review](ginux-contributions/g03b-review.md) keeps actual
G05a upstream merge, full default/runtime/security and macOS acceptance pending.
G03c is refreshed at `e9c0a4d494cb9c2c2080289aa198dda42dd65b08` on refreshed
G03b/G03a composition `6c35957d17c54dde9d98e5a91e49d37465d13a6d`:
all 85 units/static/build and eleven ordinary controls pass in source/clean,
all 190 tracked paths and both binaries match, and the exact prerequisite rejects
the identical valid storage fixture. The ten-file +1140/-28 delta preserves new
upstream native/domain/network additions. [Review](ginux-contributions/g03c-review.md)
and [security handoff](ginux-contributions/daybreakblue-handoff.md) retain full
required runtime/security/macOS CI and actual upstream-merge holds.
G04 is refreshed at `09fdb1b0f2caae3432846966815dcb7444a3c0e2` on G03c:
all 85 units/static/build and fourteen ordinary controls pass in source/clean,
all 194 tracked paths and both binaries match, and exact G03c rejects the
identical valid assigned-null fixture. Only the module/native audit-label context
needed adjustment for upstream changes; its twelve-file +601/-119 scope remains.
[Review](ginux-contributions/g04-review.md) keeps security/full CI and actual
upstream prerequisite merges pending. G05 is refreshed at
`51b2c751af4a68d78261e7b63e731672e0b13240` on G04:
all 85 units/static/build and twenty-three ordinary controls pass in source/clean,
all 199 tracked paths and both binaries match, and exact G04 fails the identical
pipe/public-FD capability fixtures at exits 10/11. Its seventeen-file +1297/-76
scope is preserved, with the native match context adapted to current upstream.
[Review](ginux-contributions/g05-review.md) retains security/full default CI,
macOS and upstream prerequisite merge holds. G06b is refreshed at
`7240bb217e16750f55f57a490d444964e8bf537a` on refreshed G05/G06a composition
`7e0061b500e32e0735d34a375fcb668968f09ee6`. The first build found duplicate memmove; the
packet now reuses the current upstream owner. All 90 units/static/build and
forty-six ordinary controls pass in source/clean, with all 205 tracked paths and
both binaries matching. Exact prerequisite fork/exec fixtures fail at exits
95/61 and pass on the final head. [Review](ginux-contributions/g06b-review.md)
keeps protected-copy/interpreter/FD/shared-argument security, full CI and actual
upstream prerequisites pending. G07 remains unready on its historical base; its
security correction and thread consumer closure are not established.
G09a is refreshed at `648ec32a34ece2f1cd7d37b10271e4166eb14683` on G03b:
all 89 units/static/build and eight ordinary controls pass in source/clean,
all 187 tracked paths and both binaries match, and the exact prerequisite fails
the truncating-open capacity regression. Production code is unchanged; a focused
9,000-byte/partial-copy cleanup unit is added. [Review](ginux-contributions/g09a-review.md)
keeps its G09b pairing, security/full CI and actual upstream prerequisites pending.
G09b is refreshed at `a2a70d1c7811886d2fca20d228a94e7dc6b31a61` on refreshed
G06b/G09a composition `ff29e8574f1382355bd4b925bf2aaf634885c305`: all 94 units/static/
build and forty-nine ordinary controls pass in source/clean; all 206 tracked paths
and both binaries match. The exact prerequisite fails the identical growth
fixture at workload exit 121 beyond 64 KiB. Its seven-file +281/-13 delta is
unchanged. [Review](ginux-contributions/g09b-review.md) keeps the useful G09a/G09b
pair, security/full CI and actual upstream prerequisite holds explicit. Initial
37-patch integrity results remain historical. G09c is refreshed at
`1e34ef3641c9a2d93f449818692692ac2b561ebc` on G09a: all93 units/static/build
and eight ordinary controls pass in source/clean, all187 paths and both binaries
match, and the one-file +491/-2 production/four-test delta is unchanged. Only
append context adapts around preserved G09a chunk-copy coverage.
[Review](ginux-contributions/g09c-review.md) keeps G09d pairing, its errno join,
security/full CI and actual upstream prerequisites pending. G09d is refreshed at
`ab47378f657b8d13d72077009107d223ba04d3a3` on G09b/G09c composition
`eed013c73443a24390b1f5f6638dedca170c855f`: all98 units/static/build,
49 ordinary controls and four directory/cwd launches pass per source/clean;
all208 paths and both binaries match. The identical fixture fails on exact
G09b at exits12/94. Its eleven-file +582/-53 delta is unchanged, including the
enum/runtime join. [Review](ginux-contributions/g09d-review.md) and the downstream
security handoff retain security/view-probes/full CI/actual-upstream holds. G10 is
refreshed at `38f7ee402611f9023b4173eff5dc5d64b7d68375` on G09d: all100 units/
static/build and59 ordinary controls pass per source/clean, all211 paths and both
binaries match, and exact G09d rejects the same valid owned ELF at exit128.
[Review](ginux-contributions/g10-review.md) records its unchanged fourteen-file
+1165/-131 delta and all security/shared-argument/full CI/upstream holds. The refreshed
`upstream-refresh-2026-11-06/current-inventory.json` records each currently retained
patch/head and exact-parent replay. Thirteen packets have source/ordinary preparation against the earlier observed
8a475e8b base; the other24 have not yet reached that base. Newly observed main
ff926d0c requires reconciliation of all retained packets before latest-main
acceptance;
integrity is not source/runtime acceptance. The fresh preserved-worktree report
also confirms all six restored HEAD, raw-index and staged/unstaged/conflict-status
fingerprints are unchanged. Reconcile the
remaining source owners before rebuilding the final F02/F03 revisions. Keep
reader/progress-thread-dependent consumers pending on G07. Do not import
the unready thread snapshot, rerun adversarial probes, dispatch DaybreakBlue,
weaken required checks or treat source integrity as full acceptance. G07,
security/policy, full CI and current-main independence remain open; F04 depends
on the required F02/F03 evidence.

Active skill: `hdae-development`, discovered at
`/home/vince/Projects/LevitateOS/.agents/skills/hdae-development/SKILL.md`.
Reload it and its required references after a context shift; then read applicable
instructions and this checkpoint. Direct user scope and the superproject's
proportional-process instructions prevail; this plan adds no automated framework.


### New upstream contract owner — 2026-11-06

A read-only GitHub recheck found upstream main
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a` ([PR#231](https://github.com/gmkurtzer/ginux/pull/231)),
eight commits after8a475e8b, with a new ginux-contract catalog, normalized domain
contract profiles, semantic-family enforcement and bounded audit evidence. Ginux
leads; Recipe and the proposed contributions must follow this owner. Do not keep
a downstream parallel contract or describe the8a475e8b refresh as latest-main
acceptance. The main checkout's Ginux pin is still
`2eac10bf8c9e7a7e72e1ae94990aa2425b2f3ba0`; it was inspected, not changed.
Exact next action: inspect the complete upstream delta and current configured
launch/contract callers in a separate source export, then reconcile candidates
serially in dependency order. Preserve all prior exact-revision results and
restored worktrees. No branch/ref rewrite or publication is authorized.

The exactff926d0c object/source and43-path delta are now retained separately in
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/`;
no real checkout or submodule pin changed. Initial source review confirms the
new EffectiveContract/AuditBudget join is in upstream native_main/supervise_native;
configured launch remains a separate supervisor. Trace its dispatch and Recipe
callers explicitly rather than assuming a clean replay inherits enforcement.
The complete contract library/tool and remaining delta review are unfinished;
`review-progress.json` records the exact read position. Latest-main baseline
compilation/unit/ordinary evidence is recorded separately from every candidate.

[Upstream contract review](ginux-contributions/upstream-contract-review.md) owns
this new prerequisite. The exactff926d0c baseline now passes83 units/static/build,
generated contract-document consistency and eight ordinary controls; this does
not validate a refreshed contribution or configured contract enforcement. Read
its complete remaining source/callers before choosing the minimal adjustment.


### PR231 source trace and serial refresh — 2026-11-06

The complete contract library/tool, configuration/Cargo/script changes and
Recipe renderer/preparation caller are now read. Ginux's existing versioned
catalog, profiles, EffectiveContract and AuditBudget lead the adjustment.
Recipe commit `d0fafc199640b6c9e71b9589c75378e2a7345b32` corrects its one
unversioned profile value to `gate5-balanced-v1`; library/binary compilation
passes, and exact upstream rejects the previous name while accepting this name.
Full rendered-config/consumer acceptance remains pending on the actual storage/
network/runtime contributions. Original paused inputs were preserved.

G05a is now `25d4f05a30a2b367e24aea9460149bd55baa3406`, directly onff926d0c:
three exact-base regressions fail, all86 units/static/build and eight ordinary
controls pass in source/clean, all196 tracked paths and both binaries match.
Its VFS source is unchanged. [Review](ginux-contributions/g05a-review.md) retains
full default/security/macOS/remote CI holds. The latest37-patch inventory is now
`upstream-contract-refresh-2026-11-06/current-inventory.json`; only G05a has
source/ordinary preparation onff926d0c, and36 retained packets remain.

Exact next action: refresh G03a, then G06a and G03b, serially onff926d0c. At
G03c, join configured launch effects to upstream contract/catalog/audit owners
before continuing dependent runtime packets, then inspect the task-loop join
at G06b. Source review identified the separate fixed-dispatch paths; security
checks are recorded downstream and remain pending, with no local exploit-like
reproduction or DaybreakBlue dispatch. G07, F02/F03 and final F04 remain open.

G03a is now `feafbad2a374e80c18943370e7efd1263554a245`, directly onff926d0c:
exact upstream exits2 without creating the missing workdir; source/clean create
it and exit0. All83 units/static/build and nine ordinary controls pass per
export, all196 paths and both binaries match, and the two-file +4/-1 scope is
unchanged. [Review](ginux-contributions/g03a-review.md) preserves all full default/
security/macOS/remote CI holds. Two retained packets now have source/ordinary
preparation onff926d0c;35 remain. Configured launch's existing64KiB byte audit
cap must be preserved through its own writer; do not add a parallel budget merely
because native supervision now has an8192-event budget.

Exact next action: refresh G06a onff926d0c, then G03b. At G03c, use upstream
contract/catalog owners before configured effects and extend the existing audit
writer as needed; review task-loop ordering at G06b. This supersedes earlier
read-position/next-action entries. Keep G07, security, F02/F03 and F04 pending.


### Latest-base G06a — 2026-11-06

G06a is now `4a62d81e3d81ede3b987d679333c39b62212c8bf`, directly onff926d0c.
The process source is unchanged from the previous corrected extraction. Three
exact-base adoption regressions fail; source/clean pass all86 units/static/build
and eight ordinary controls, with all196 tracked paths and both binaries matching.
[Review](ginux-contributions/g06a-review.md) retains full default/security/macOS/
remote CI and G07 holds. Three retained packets have latest-base source/ordinary
preparation;34 remain. Exact next action: refresh G03b on refreshed G05a, then
join configured contract/catalog before effects and retain the existing audit
owner at G03c. Later task-loop ordering belongs to G06b. F02/F03/F04 remain open.

G03b is now `095f89c5bfb2f8d07404f2304a5088f709ef7652` on refreshed G05a
`25d4f05a30a2b367e24aea9460149bd55baa3406`, rooted atff926d0c. Source/clean
pass all95 units/static/build and eight ordinary controls; all196 paths and both
binaries match, with unchanged one-file +425/-6 VFS source and all tests retained.
[Review](ginux-contributions/g03b-review.md) retains actual G05a merge and full
runtime/security/macOS/remote CI holds. Four retained packets have latest-base
source/ordinary preparation;33 remain. Exact next action: reconcile G03c on
refreshed G03b/G03a, tracing configured contract dispatch, new syscall catalog
membership and the existing bounded audit writer before implementing the join.
Keep actual upstream semantics and source owners; do not count a clean apply or
old passing runtime check as the missing policy integration. G07/F02/F03/F04 and
security acceptance remain pending.


G03c's unverified source preview now exists at
`upstream-contract-refresh-2026-11-06/G03c/source`, based on local G03b/G03a
composition `85ee433afd71c9985cb714246c343145c2cdfb01`, preview tree
`e72f1f95535a23ab4d03da5c7fb031a5111bb17a`. Its retained proposal remains the
previous reviewed head until the required join is implemented and verified.
The preview applies cleanly, but no G03c build/runtime acceptance is claimed.
Complete storage/path owners, configured supervisor/audit, fixtures/README/CI
and relevant catalog checks have been read. Source findings: policy must precede
filesystem effects and scratch rewrite; implemented lstat6/fchmodat268 lack
catalog entries; preserve existing native-denial/core-catalog coverage when
adding configured-only operations. Extend the existing64KiB audit writer through
its owner. Exact next action: implement that join, then static/unit/build and
ordinary source/clean verification; preserve all default/security checks and
keep their monitored acceptance pending. `G03c/state.json` records the source
read position. Do not dispatch DaybreakBlue or rerun adversarial probes.


### G03c resolved-contract checkpoint — 2026-11-07

The preview's missing join is implemented in final G03c
`3801fd45bd6280a5b320f2d464ada0bd04539326`, tree
`f85bd3524a30f7f794847516ffd3453d3f49423d`, on refreshed G03b/G03a composition
`85ee433afd71c9985cb714246c343145c2cdfb01` rooted atff926d0c.
[Review](ginux-contributions/g03c-review.md) owns the thirteen-file +1351/-39 scope:
upstream EffectiveContract now selects raw/semantic policy before configured
filesystem effects or scratch rewriting; native shares that existing owner;
lstat6/fchmodat268 are cataloged with native Deny38 preserved; configured audit
retains its64KiB cap and gains metadata/disabled mode. No parallel policy registry
or audit system was introduced. All prior tests and CI remain enabled.

Source and strict clean replay pass96 units/static/build (24 VFS/10 contract),
generated contract docs and15 ordinary controls per export. These include valid
owned metadata and complete audit records/disabled output/budget, alongside all
11 earlier ordinary controls. All199 tracked paths and both binaries match. The
exact prerequisite rejects the same storage fixture and reproduces ignored
audit-disabled mode with an exit-zero workload. A downstream audit receipt
serialization failure was corrected without changing assertions; initial evidence
is retained. The new default raw/semantic denial fixtures are compiled and
registered but unrun; [DaybreakBlue handoff](ginux-contributions/daybreakblue-handoff.md)
keeps runtime policy/security acceptance pending. No cloud dispatch occurred.

Five retained packets now have source/ordinary preparation rooted atff926d0c;
32 remain. This does not close full default/runtime-policy/security/macOS/remote
CI, actual upstream prerequisites or F02/F03/F04. Latest patch integrity/readiness
is in `upstream-contract-refresh-2026-11-06/current-inventory.json`. Original
worktrees, paused snapshots and human remote text remain preserved.

Exact next action: refresh G04 on G03c3801fd45 and keep device claims under the
resolved policy guard and existing audit owner. Then G05 must preserve that guard
through descriptor translation, and G06b must check policy before its early task/
exec/wait hooks. Reconcile each final packet's whole diff and ordinary/clean
verification serially. G07 and shared/indirect argument security remain unready.
Active skill remains hdae-development at the discovery path above; reload it,
required references and this latest checkpoint after any context shift.


### G04 latest-base checkpoint — 2026-11-07

G04 is now `8880ddc8fd7b3918707a5a832de47e86f49c9fda`, tree `2215efdef3eca9a27e90a1e72ed91b9fa53611f9`, on refreshed
G03c3801fd45 rooted atff926d0c. [Review](ginux-contributions/g04-review.md) owns
its fourteen-file +624/-125 scope. Device claims remain below resolved policy
admission; faccessat269 joins upstream's catalog/read-family owner with native
Deny38 and all earlier assertions preserved. Existing audit/copy owners remain.
Source/clean pass96 units/static/build, generated docs and18 ordinary controls
per export; all203 paths and both binaries match. Exact G03c rejects the identical
valid null assignment. All37 retained patch/head/parent/tree replays verify and
all six restored worktrees remain unchanged. Read-only GitHub still reports
upstreamff926d0c; Ginux leads and Recipe/contributions follow its contract.

Six latest-base preparations exist;31 retained patches remain. Full default/
runtime-policy/security/macOS/remote CI, actual upstream prerequisite merges and
G07/F02/F03/F04 remain pending. No branches/real checkouts/remote text/publication
changed. Current goal turn: progress. Exact next action: refresh G05 on G04
8880ddc8fd7b3918707a5a832de47e86f49c9fda, preserving contract admission before public descriptor
translation/effects; then review G06b before early task/exec/wait hooks. Security
work stays downstream for DaybreakBlue without dispatch. Active HDAE skill and
required context-reload procedure remain as recorded above.


### G05 latest-base checkpoint — 2026-11-07

G05 is now `2718b860f04bc644c001d7a9cea906a74ddd386a`, tree `72bfe4dec68a549daed6d6f180517fcad41748c3`, on latest
G048880ddc8 rooted atff926d0c. [Review](ginux-contributions/g05-review.md) owns
its nineteen-file +1331/-87 scope. Policy precedes descriptor translation and
slot/alias/pipe effects; the existing descriptor owner performs a pure regular-file
semantic lookup. The initial write-full lifecycle failed because pipe22 was absent
from upstream's new catalog. That implemented ABI is now cataloged with native
Deny38 preserved, and the unchanged check passes; initial failure evidence remains.
The inherited valid metadata range follows the public ABI without losing assertions.

Source/clean pass96 units/static/build, generated docs and27 ordinary controls
per export. All208 tracked paths and both binaries match. Exact G04 fails the
same valid pipe/public-FD fixtures at exits10/11. All37 retained patch/head/parent/
tree replays verify; all six restored worktrees remain unchanged. Seven latest-base
preparations exist;30 retained patches remain. Full default/runtime-policy/security/
macOS/remote CI, actual upstream prerequisite merges and G07/F02/F03/F04 remain
pending. Current goal turn: progress. No branch/ref/real checkout, remote text
or publication changed; no DaybreakBlue dispatch occurred.

Exact next action: compose latest G052718b860 with G06a4a62d81e, then refresh
G06b serially. Inspect and place contract admission before its early task/exec/
wait hooks, join actually implemented new syscalls to upstream's catalog and
retain the existing bounded/disabled audit owner. Do not import unready G07 or
substitute ordinary evidence for monitored security/full CI acceptance. Active
HDAE skill and context-reload procedure remain as recorded above.


### User pause — G06b source preview, 2026-11-07

The user paused the persistent goal. Stop implementation/verification until an
explicit resume. No build/runtime process remains active: build session64899
completed with exit0 before this checkpoint; no G06b runtime suite was started.
The goal status is paused, not complete. The preceding goal turn made progress.

A separate G06b preview exists at
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G06b/source`.
Its exact local prerequisite is `98531335b47de1363d30428a56b0fc7b7e38fce5`, tree
`5b2d2d65d6356d3d3c77a8a855bdd7c81f5726a3`, composing latest G052718b860 and
G06a4a62d81e. Composition/temp indexes/read positions are retained with the preview.
No final source commit/patch or clean export has been prepared. The retained
G06b proposal remains historical7240bb21; do not replace it with a readiness claim.

Preview edits move resolved-contract admission to task entry before early exec/
fork/wait/signal/descriptor hooks; blocked calls reenter the same admission point.
The task supervisor resolves one EffectiveContract through the existing owner and
uses the existing bounded/disabled audit writer. Implemented execveat322 now joins
upstream's catalog with native Deny38 retained. All prior assertions/fixtures/CI
remain. README/launch/storage paths and source whole-record assertions are adapted
to the moved task owner. Initial source-context/format failures are preserved.

The first source `make check test-unit build build-launch-fixture
build-launch-storage-fixture build-launch-device-fixture build-launch-pipe-fixture
build-launch-fd-fixture build-launch-task-fixture build-launch-exec-fixture
build-process-fixtures build-gate1-fixture` completed with exit0 and101 unit
tests. The contract-document producer succeeded; its final check, full-diff review,
ordinary runtime, baseline comparison, source hashes/clean replay and security/full
CI remain pending. This is partial source progress, not verified G06b acceptance.

On explicit resume: reload the complete active HDAE skill/references, applicable
instructions and this checkpoint, then inspect actual preview source/diff/fixtures.
Complete task/contract/catalog/audit review before running scoped ordinary controls;
its inherited audit receipt regex must recognize the existing virtual-pid field
while retaining all whole-record/budget assertions. Then prepare and verify the
independent clean replay. Preserve all six restored worktrees, prior patches and
paused Recipe inputs. No G07 import, adversarial probes, DaybreakBlue dispatch,
branches/remote mutations or publication is authorized. F02/F03/F04 remain open.


### Ownership split — 2026-11-07 (authoritative)

Two owners, two goals:

- **Codex keeps its goal: finish integrating Ginux into Recipe.** It continues the local
  Ginux→Recipe integration and consumer work (F03 and what Recipe needs from Ginux) on its own
  local integration source. It may change that local Ginux source wherever Recipe needs it.
- **Claude owns upstream Ginux pull requests**: choosing, extracting, reviewing, reworking,
  publishing and following them up on gmkurtzer/ginux. Where the two overlap, Claude's PR
  work takes precedence.

What this means for Codex:
- Stop preparing upstream packets: no refreshing G-packets onto upstream main, and no
  R01–R04/F04 publication work. The 2026-11-06/07 G03b–G05 refreshes and the G06b preview
  stay as history; Claude decides what becomes a PR.
- When the integration needs a new Ginux fix or behavior, commit it on the integration
  source with a clear message so Claude can extract it as a PR later.
- Do not push to the LevitateOS/ginux fork, open PRs or comment upstream. Do not touch the
  `fix/*` or `prep/*` branches, or the worktrees `/home/vince/Projects/ginux-review` and
  `/home/vince/Projects/ginux-storage`.
- Upstream PRs change upstream main. When one merges, the integration rebases onto it.

Current upstream state lives in [the PR inventory](ginux-contributions/pr-inventory.md).
Wave 1 is open as #232 (G05a), #233 (G03a) and #234 (G06a). Claude is redesigning G03b+G03c
as one configured-storage feature ([spec](ginux-contributions/storage-redesign-spec.md)).


### Local consumer integration resumed — 2026-11-07

The user's ownership split supersedes every earlier packet-refresh/publication
next action. The active goal is local Ginux→Recipe integration and consumer
verification; Claude owns upstream extraction, review, fixes and publication.
Wave1 #232–#234 were checked read-only and remain open/unmerged. Rebase the local
integration when they merge. No fix/* or prep/* branch/worktree was touched.

Codex's integration source is `/home/vince/levitate-development/ginux-recipe-integration`,
created separately from preserved F02dfebda7c. Local commit
`91e244c175ad73aaef2d5e574045282fe770df98` moves pending descriptor results,
restored arguments and poll snapshots/deadlines into each traced Task, preparing
shared-table thread support without importing the unready thread snapshot.
Static/build,102 routine Ginux units and selected ordinary pipe/vector, polling/
deadline, task/exec/lock checks pass. Three existing policy/security unit tests
remain enabled/unrun; no full CI/security/thread-consumer passage is claimed.

The previously interrupted ordinary Recipe retained-links case now passes1/1
in166.17s on this binary and the destination-built saved Recipe harness. Both
managed invocations restore/publish link bytes and targets without rebuilding.
Its exact receipt/logs are in the integration source's
`.artifacts/recipe-integration/`; no test process remains active. The source
state.json owns the exact local head/binary/verification. This is one saved
Recipe-snapshot case, not a final/current whole-suite pass.
Next: shared descriptor-group, executable and cwd lifetime, then bounded thread
creation using the new per-task transactions. Verify ordinary thread behavior
before rerunning failed Recipe output-reader/progress consumers.
Keep B1–B3 recorded; do not redesign storage or run adversarial probes here.
The requested #207 conflict-push explanation is appended to pr-inventory.md,
preserving Claude's current uncommitted edits; push nothing further to the fork.
Active HDAE skill/reload procedure remains as recorded above; no cloud dispatch.


### Local configured threads — 2026-11-07

Local integration commit `78f96df1` (on `91e244c1`) adds the libc thread clone
shape through the existing process owner. Threads share descriptor/cwd/executable
lifetimes; pending syscall results and blocked-IO pins remain per task. Fork
snapshot publication, concurrent child waits, leader-first exit and thread-slot
retirement retain explicit ownership. Exec remains limited to a single process
leader. This is local consumer work for Claude to extract, with no upstream push.

Static/build and103 routine Ginux units pass; the three preserved policy/security
units remain unrun. Eight ordinary thread modes plus signal/failure teardown pass,
as do selected process, pipe/vector, poll, cwd, owned ELF and script regressions.
The saved Recipe Bash/output-reader case now passes1/1 in83.76s on binary
`d76807581abb74a37c32c80fc3719f6a49c7bebf5d5783e0ce22b14ba71f5b13`.
Earlier failed receipts are retained. This is ordinary capability evidence, not
G07 shared-argument security acceptance or full/current Recipe-suite passage.

Two ordinary saved tar/progress cases are running serially; current Recipe CLI
and e2e harness are building in a separate target directory. Exact logs/receipts
and source head live in the integration source's `.artifacts/recipe-integration/`.
Next: inspect completed tar results, diagnose any ordinary failures, then finish
current-Recipe consumer verification. Known B1–B3 storage defects remain unchanged;
security work stays downstream and pending without dispatch. Preserve all six
historical restored worktrees. Active HDAE skill and reload procedure remain as
recorded above. The goal is active and incomplete.


### Fresh current Recipe consumer batch — 2026-11-07

Current Recipe `d0fafc199640b6c9e71b9589c75378e2a7345b32` is clean and newly
built with its e2e harness in `/home/vince/levitate-development/recipe-consumer-target`.
Only AGENTS.md and the versioned balanced-profile choice differ from the saved
snapshot (apart from its standalone lock). Full source hashes, workspace lock,
binary hashes and135 exact catalog names are recorded in the integration source's
`.artifacts/recipe-integration/current-recipe-provenance.json`.

On local Ginux `78f96df1`, the fresh serial batch passes8/8: real C transitive
dependency compile/execute/reuse (175.36s), Bash capture (83.05s), owned ELF
(87.61s), modes/links/timestamps plus restart identity (173.95s), tar symlink
(83.10s), tar hardlink (86.92s), executable zip (86.20s), and retained-link
publication without rebuilding (174.74s). Every exact selection ran one test and
completed with exit0. `current-ordinary-{state,receipts}.json` and per-case logs
preserve commands and results. Original failures/interrupted receipts remain
unchanged; this is a selected current-source batch, not a whole135-case pass.

The actual ordinary offline Cargo/cache consumer is now running: two clients
share a target directory, then a compiled Rust program starts a thread and
returns verified output. Its result remains pending until the whole command and
product checks finish. Remaining ordinary consumer/full-suite checks, security
acceptance and integration rebase remain open. Read-only GitHub still reports
#232–#234 open/unmerged on upstreamff926d0c. The current integration deliberately
retains its F02 base plus the two local fixes; it is not latest-main/contract
acceptance. All six restored worktrees match HEAD, raw index and NUL-delimited
status fingerprints. core.worktree remains in config.worktree. No push, PR,
upstream comment, forbidden branch/worktree edit or security dispatch occurred.

Next: inspect Cargo's receipt/logs, fix actual ordinary causes in existing local
owners, then continue remaining source-reviewed Recipe consumers. Active HDAE
skill/reload procedure remains as recorded above; goal active and incomplete.


### Upstream main moved — 2026-11-07

The user directs pulling Ginux's major main update when appropriate. No ordinary
test process remains active. Upstream main is now
`59e2f7aa6a03a1afe2b6579815654ad63fc6c1b6`,15 commits beyondff926d0c, including
instance terminology/config/crate renames. Fetch the exact object without changing
shared branch refs, inspect the delta, and update only Codex's local integration
source and its Recipe consumer. Do not refresh upstream G-packets, touch Claude's
fix/prep branches/worktrees, publish, or alter the worktree Git configuration.

Before this update the actual offline Cargo/cache fixture failed in88.75s: the
first rust-lld invocation threw std::system_error/EAGAIN during thread creation;
the second Cargo client later compiled successfully. Both clients are required
to succeed, so the fixture correctly failed; no compiled-product acceptance is
claimed. The saved supervisor audit reaches its unchanged64KiB budget before
the linker, so it does not identify the precise exhausted resource. Diagnose
that actual cause after the upstream owner reconciliation; do not suppress
linker threads or raise limits merely to turn the check green. Exact command,
fixture, stderr and receipt are retained under the integration source's
`.artifacts/recipe-integration/cargo-fixture/`. Eight prior fresh Recipe case
passes remain evidence only for78f96df1/d0fafc1, not the coming rebased source.
Active HDAE skill/reload requirement remains as above. Goal active/incomplete.


### Instance-main integration checkpoint — 2026-11-07

Codex's detached local integration is now rebased onto upstream main
`59e2f7aa6a03a1afe2b6579815654ad63fc6c1b6`. The original F02 source and old
local commits remain intact. Rebased commits are0c4eb9d9 (composition),12c176d4
(per-task descriptor transactions),4a8d50f7 (shared configured threads), then
`8981bf774af386bb51e5126d1d54cc0e00a5e449` (local instance adaptation).
Upstream native/systemd paths and its memmove correction are retained. Local
storage/network fields join the new instance owner with noncolliding bits36–40;
normalization and CLI overrides share that owner. All upstream fields, units and
CI remain. An initial conflict-parser truncation was caught before verification;
that local attempt was aborted, the complete files restored from recorded
commits, and the rebase rerun with corrected parsing. No remote history changed.

Ginux static/build (including terminology checks),121 routine units and the
selected ordinary thread/process/pipe/poll/cwd/ELF/script groups pass. Three
existing security units and full/default/platform CI remain unrun. The fresh
Recipe renderer change is committed as `f70b35d`: it emits `[instance]` and
uses InstanceConfig directly, with no old-schema adapter. Its new CLI/harness
builds and the real Bash/output-reader case passes1/1 in86.54s with the verified
prepared image. Exact current source/binary hashes and logs live in the local
integration source's `.artifacts/recipe-integration/instance-recipe-provenance.json`
and state.json. Earlier eight-case results remain historical for the old source.

The unchanged actual Cargo/cache fixture is now rechecking these rebased
revisions in `cargo-instance-fixture/`; its result remains pending. The earlier
linker EAGAIN remains a failure, not overridden by the second client's success.
Next: inspect that recheck; if it fails, identify the actual exhausted resource
using bounded diagnostics, then fix its local owner and rerun without disabling
checks. Remaining current ordinary consumers and security acceptance stay open.
No upstream packet refresh/publication, forbidden branch/worktree edits, fork
push, upstream comment or agent dispatch occurred. core.worktree remains in
config.worktree. Preserve unrelated inventory/UI/untangle edits. Active HDAE
skill and context-reload requirements remain as above. Goal active/incomplete.


### Cargo task-capacity cause fixed — 2026-11-07

Local Ginux `12c4ea80` adds one bounded resource diagnostic after audit truncation.
The unchanged Cargo fixture then confirms the cause: the16 configured task
slots fill during rust-lld's ordinary process/thread tree. Local fix
`062fb5ef53ab0db6ab14f995b4ec87d1d97ccf2b` keeps a fixed32-task bound, including
unreaped children. CPU/memory/descriptor limits and compiler/cache checks are
unchanged. Process/thread fixtures prove exact capacity, EAGAIN beyond it, full
reaping and slot reuse. Existing forbidden-clone assertions remain in the full
default case; an ordinary capacity selector avoids running those probes locally.
Static/build,121 routine units and the ordinary thread/process/resource/lock
checks pass on this final source. The three preserved security units remain unrun.

Actual offline concurrent Cargo now passes in85.39s on Recipef70b35d: both clients
succeed, logs prove one compilation and real cache/build-directory lock waiting,
and the compiled Rust executable creates a thread and returns `compiled rust 42`
inside Ginux. Its523736-byte executable is published with mode0755, two matching
checkpoints acknowledged, and both compiler logs checked. No capture, progress,
locking, validation or compiler-thread workaround was introduced. Earlier failed
receipts remain under cargo-fixture/, cargo-instance-fixture/ and
cargo-diagnostic-fixture/. The passing command/receipt is cargo-capacity32-fixture/.

The final-revision eight-case serial Recipe batch is running on
Ginux062fb5ef/Recipef70b35d; `.artifacts/recipe-integration/final-ordinary-state.json`
and final-consumer-provenance.json own the live progress and exact hashes.
Next: inspect that batch, diagnose failures honestly, then run remaining
source-reviewed ordinary consumers and actual APK/RPM paths. Current full-suite,
security/default/platform CI and required acceptance remain pending. No remote
changes or security dispatch occurred. Active HDAE/reload procedure remains as
recorded above. Goal active/incomplete; no current-case process was interrupted.


### Musl consumer and package-path checkpoint — 2026-11-07

The final062fb5ef/f70b35d eight-case ordinary batch finished8/8 with exit0
and exact one-case receipts. Results remain tied to that source. Workspace
release Recipe commands preserve the existing profile and reduce the unchanged
GNU Cargo fixture from85.39s to9.28s, producing the identical executable hash.

Actual musl compilation/linking succeeded, but its program could not start a
thread: musl uses clone flags0x7d0f00, while the local owner accepted only0x3d0f00.
Local Ginux commit `e9a2f8a07e8e34d2de9281e1b3de1f33d5a30377` admits that exact
additional native thread form. Static/build and nine ordinary thread modes plus
teardown pass. The unchanged real musl Cargo/cache consumer now passes9.05s: both
clients succeed, one compilation, cache/target locking, static ELF and an actual
started/joined Rust thread returning42. No linker-thread suppression is used.
Recipe documentation commit0d412c8 corrects GNU/musl compiler guidance; runtime
source remainsf70b35d. Original musl failure and passing receipts are retained.

The actual pinned Fedora RPM fixture failed before runtime launch: importing
/usr/bin/cpio makes the host-path guard mistake its own fixed image PATH for a
leak. A minimal Recipe input-owner correction is being built: use the same fixed
PATH producer and exempt only its complete canonical PATH assignment, retaining
prefix scanning on other values. No bypass/escape probe was run; this defensive
join needs downstream monitored review. The initial failed RPM receipt is kept.
APK runtime acquisition/extraction remains unrun. Exact evidence is under the
local integration source's `.artifacts/recipe-integration/`.

Next: finish the input-owner build, run the unchanged RPM pipeline on a new
receipt, fix actual ordinary causes if needed, then pinned APK and remaining
source-reviewed ordinary consumers serially. No upstream work/publication,
forbidden worktree/branch changes or security dispatch occurred. Full/security
acceptance remains pending; goal active/incomplete. Active skill:
hdae-development at `.agents/skills/hdae-development/SKILL.md`; reload the full
skill, required references and this checkpoint after a context shift.


### Actual RPM/APK consumers pass — 2026-11-07

Recipe commit `0c4f0d4e9d1f84ab7f638611e895dbf799500e7a` fixes imported
/usr/bin/cpio input preparation by recognizing only its own complete fixed PATH.
Other path values still undergo prefix scanning. Its ordinary temporary-root
regression and real imported-tool execution pass. A first check used a retained
nonexecutable binary copy and failed before launch; the corrected build selection
passed. Both logs remain. No host-path bypass probe ran.

The unchanged RPM pipeline then exposed real owned-pipe truncation: rpm2cpio
produces170444 bytes into an owned file, but only4096 through a blocking pipe.
Local Ginux commit `6a19a70a8c2ff8ad469b8127a2b8a1a5244fa714` retains task-owned
write/vector progress across bounded-buffer waits and wakes peer readers after
partial publication. The64KiB buffer and nonblocking/packet short transfers stay
bounded. A196733-byte forked write/writev regression fails on the previous binary
and passes here; all five ordinary pipe groups, polling and nine thread modes
plus teardown pass. Final static/build passes. No existing tests/CI were removed.

The original RPM pipeline now passes7.66s on this source and Recipe0c4f0d4:
expected169536-byte library digest, mode0755, UID/GID0 and both symlinks verified.
Actual APK managed HTTPS acquisition, pinned digest, native APK extraction and
static apk-tools3.0.9-r0 execution pass8.88s with three acknowledged checkpoints.
No host downloader/extractor substituted for managed execution. All earlier
failures and diagnostic file-based runs remain separately recorded. Exact source
and binary hashes live in `blocking-pipe-final-provenance.json`; Ginux binary
SHA256 is717e5c13f284f2edd9485ab71ca22c718850efeaa6decba34be0462ce141fad4.

Read-only upstream check still reports main59e2f7aa and #232/#233/#234 open/unmerged.
No further rebase is currently needed. The source-reviewed20-case ordinary
state/target/lifecycle/cleanup/hash queue is now running serially using release
commands and the existing e2e harness; remaining-ordinary-{state,receipts}.json
own actual status. Next: inspect its receipts, fix actual failures, then finish
remaining ordinary consumer coverage/source review. Security/full/default/platform
acceptance remains pending, and no DaybreakBlue dispatch is authorized. No pushes,
PRs, upstream comments or forbidden source/branch edits occurred. Goal active and
incomplete. Active HDAE skill/reload procedure remains as recorded above.


### Ordinary lifecycle queue complete — 2026-11-07

All20 selected state/target/lifecycle/cleanup/hash cases now pass on local
Ginux6a19a70a/Recipe0c4f0d4, with exact one-case receipts in
remaining-ordinary-{state,receipts}.json. Positive real Git clone, exact-commit
reuse and switching also pass. The first positive fixture incorrectly declared
is_built ready and skipped acquisition; its failed receipt is retained. Corrected
readiness causes actual clone/selection/publication, without a product change.

Final-source actual SQLite and GNU/musl compiler consumers are now running
serially. Their results remain pending. The complete135-case catalog is mapped
in ordinary-coverage-selection.json:20 just passed,82 further ordinary cases
selected/unrun,33 held for downstream monitored authority/fault/escape/spoofing
review. Original mixed cases stay intact; no partial test is counted as passing.
Library/image test targets also contain held spoofing/unsafe-path cases and are
unrun. Next: inspect actual consumer receipts, fix actual failures, then execute
the82 ordinary cases serially. Current security/full-suite acceptance remains
pending. Goal active/incomplete; no upstream action or security dispatch.
Active HDAE skill/reload procedure remains as recorded above.


### SQLite creation-mask fix and final consumer run — 2026-11-07

The20-case queue passed on6a19a70a. The actual SQLite client compiled, contended
correctly, committed sum33 and passed integrity checks, but its ELF had mode0645
instead of0755; host validation correctly failed. Configured umask95 returned
ENOSYS and binutils used that value as the prior mask. Local Ginux commit
`436c4ae5843e4d50f4833158d351523f35669909` tracks the creation mask in the
existing process group and applies it to owned file/directory creation. Default022,
thread sharing, fork copying and exec retention pass focused checks. Native lane
and catalog behavior are unchanged; this is not Claude's storage redesign.
The mask regression fails on6a19a70a and passes here. Static/build and affected
ordinary task/lock/resource/thread/pipe groups pass. An initial C indentation
warning was fixed without changing compiler flags or assertions.

All six actual consumers now pass on436c4ae5/Recipe0c4f0d4: SQLite7.84s (including
mode0755, two-client contention, sum33 and integrity); pinned Git8.92s; RPM9.43s;
APK9.87s; GNU Cargo9.80s; musl Cargo10.00s. Both Cargo clients succeed, compile
once, retain lock/cache checks, and execute a Rust program that starts/joins a
thread. No --threads=1 workaround. Previous failed receipts remain unchanged.
Exact evidence: umask-real-consumer-receipts.json and umask-consumer-provenance.json
under the local integration source's `.artifacts/recipe-integration/`. Ginux
binary SHA256 is96fd7dfa43cda4f051cb7b819cf00a104e2b4cceb91d2ab065e7d19e544e77db.

The final-source102-case ordinary e2e queue is now running serially; prior20/eight
results remain historical after this creation-mode change. umask-ordinary-
{selection,state,receipts}.json owns actual progress and33 unchanged monitored
review holds; other mixed library/image targets remain unrun. Never count an
unrun, failed or interrupted selection as passed. Next: inspect the running queue,
fix actual ordinary causes and reverify final source, then complete the local
source/consumer review. Security/full/default/platform acceptance remains pending.
No upstream packet/publication work, push/comment, forbidden worktree/branch edit
or security dispatch occurred. Goal active/incomplete. Active HDAE skill and
required context-reload procedure remain as recorded above.


### User retains G02 clock hold — 2026-11-07

The final-source102-case run stopped after51 passes at
`managed_cancellation::managed_deadline_allows_silent_work_beyond_frame_timeout_and_keeps_logs`.
The saved failure is `/usr/bin/sleep 3`: “cannot read realtime clock: Function
not implemented”, not a Recipe frame/deadline failure. All original receipts
remain. Upstream #179 still asks to hold implementation pending contract choice.
After seeing this evidence and the configured-only option, the user explicitly
chose **“Keep clock policy held; continue independent cases.”** No clock/sleep
implementation or native grant was added. This criterion stays failed/pending.

`managed_state::managed_state_serializes_different_inputs_sharing_a_destination`
also calls guest sleep3 and remains unrun for that same dependency. The other49
unrun ordinary cases are continuing serially on unchanged Ginux436c4ae5 and
Recipe0c4f0d4. clock-held-independent-{selection,state,receipts}.json owns this
continuation; it preserves both clock-dependent criteria and the33 monitored
security holds separately. The original102-case batch is not a pass.

All six actual consumers remain passed on this source. The six recovery
worktrees match recorded destination raw indexes, original staged/conflict
entries, HEAD/status and every recorded changed-file byte; the original source
index stat caches differ from the already-recorded destination caches, not from
new work here. All83 upstream named units remain present. Preservation receipts
are umask-preserved-worktrees.json and umask-unit-source-preservation.json.

Next: inspect the49-case independent run and fix actual in-scope ordinary causes,
then finish the local review with clock/security acceptance explicitly pending.
No push, PR, upstream comment, forbidden branch/worktree edit or security dispatch.
Goal active/incomplete. Active HDAE skill/reload procedure remains as recorded.


### Current Alpine asset closure fixture — 2026-11-07

The49-case independent continuation passed30 cases, including the payload above
2GiB,5,000 entries, modes/links/restart identity and imported-target validation.
It then failed Alpine preseed refresh because the existing fixture provided only
ISO/apk-tools while current builder206e875 requires nine additional pinned APKs.
The actual recipe/facts are authoritative; no package or producer check was removed.
The original failure remains in clock-held-independent-state.json and its log.

A local Recipe test-fixture correction (two uncommitted integration-test files on
0c4f0d4) reads the trusted pure Alpine facts, stages the entire asset closure, and
serves the package set on one ordinary HTTP origin within existing endpoint bounds.
ISO/apk-tools corruption assertions remain; refresh adds bad-extra-APK preservation,
and explicit rootfs inputs now include the extra APK directory. No production
source, limits or runtime binaries changed. The fixture uses small named data for
integrity/lifecycle checks; it does not claim those bytes are usable ISO/APK tools.
Actual pinned APK/RPM consumers remain separately verified.

The rebuilt harness compiles without warnings and focused Alpine refresh passes
1/1 in62.41s, including success and all three failure types with previous inputs
and publications preserved. Its19-case remainder (including that focused pass)
and four affected ordinary download checks are being verified. The currently
running batch has22 cases:18 not-yet-run independent cases plus those four reruns.
preseed-remaining-{selection,state,receipts}.json owns actual status. Original
harness/command bytes are retained in bin/ with preseed-original-harness-retention.json;
preseed-consumer-provenance.json records current test source and unchanged runtime.

Next: inspect this batch, fix actual ordinary causes if any, then commit the
verified two-file fixture correction and finish local review. G02 remains held
by the user's explicit choice; its two criteria,33 monitored e2e criteria, mixed
library/image targets and security/full-suite acceptance stay pending. No pushes,
PRs, upstream comments or security dispatch. Goal active/incomplete. Active HDAE
skill and required reload procedure remain as recorded above.


### Final independent local review — 2026-11-07

The final ledger reconciles all135 e2e names: **100 independent cases passed,
1 required clock-dependent case failed,34 cases unrun** (33 monitored-review
cases and one other G02-dependent case). The user explicitly retained the clock
hold. Every counted pass has exact one-test selection, exit0 and one-passed
summary; failed/interrupted/unrun records are never promoted. The original
clock and preseed failures remain, with the corrected current-asset preseed
refresh and all three affected ordinary preseed cases passing separately. All
four affected download cases reran successfully after their HTTP helper change.

Recipe fixture commit `2ad833cc93f0e209fe205adb4186a822a13caaea` is clean; only
tests/e2e/{downloads,preseed_recipes}.rs differ from runtime0c4f0d4. Old harness
bytes were retained before rebuilding. Final aggregation checks their hashes,
all original unchanged case modules, current binaries and source identities.
Actual SQLite/Git/RPM/APK/GNU/musl receipts remain passed on Ginux436c4ae5.
Static/build and focused ordinary Ginux ownership checks pass; all83 upstream
named units and existing CI remain. No full/default/security/platform-CI pass
is claimed. All six historical worktrees retain recorded destination index
bytes, original staged/conflict entries and changed-file contents. No test or
build process remains running.

Final source/provenance/receipt review is in [F03](ginux-contributions/f03-review.md)
and the local source's `.artifacts/recipe-integration/final-local-review.json`.
The final read-only upstream check still reports59e2f7aa and Wave1 #232–#234
open/unmerged; no further rebase is needed now. The config location check confirms
core.worktree exists only in config.worktree. No fork push, PR, upstream comment,
forbidden branch/worktree edit or security dispatch occurred. Unrelated inventory
and untangle edits remain preserved.

The goal remains **incomplete**: required G02 and monitored-security/default
acceptance are unresolved. The current turn made progress and finished all
authorized independent ordinary verification. Exact next action: use the
recorded local sources for human inspection; when upstream resolves the retained
clock policy or authorized monitored results arrive, rebase only Codex's local
integration as needed and rerun the affected pending criteria. Do not implement
G02, dispatch DaybreakBlue or resume upstream-packet/publication work under this
checkpoint. Active skill: hdae-development at
`.agents/skills/hdae-development/SKILL.md`; after a context shift reload the full
skill, required references, applicable instructions and this checkpoint.


### Acceptance blocked after revalidation — 2026-11-07

The same acceptance impasse has recurred for three consecutive goal turns,
including the final independent-work turn and two automatic continuations.
The previous/current continuation is no progress, not a verified live-process
wait. Current repository evidence still has Ginux436c4ae5 and Recipe2ad833c;
the ledger remains100 passes, one held clock failure and34 unrun e2e cases,
with six actual consumers passing. No independent implementation/check remains.

Fresh read-only upstream evidence still reports main59e2f7aa and unchanged
#179 comments holding policy implementation. No authorized monitored results
have arrived. The user explicitly retained G02 and has not authorized security
dispatch. Further required work therefore needs user direction or an actual
external change. The goal is blocked, not complete; all criteria remain open
as previously recorded. No source, remote, forbidden worktree or test changed.

The local fixes remain available for inspection through the current F03 review.
On explicit resume, reload the complete active HDAE skill/references/instructions
and this checkpoint, begin a fresh blocked audit, inspect actual upstream and
monitored evidence, then take only the newly enabled local integration step.
Do not infer permission for G02, security dispatch or upstream publication.

### Daybreak local security continuation — 2026-11-07

The user authorized this session to perform the documented local low-level
security checks. The previous no-dispatch/no-local-probe checkpoint is therefore
superseded for this session only; Claude still owns upstream PR work, G02 clock
policy remains held, and no upstream push/comment/PR or storage redesign is
authorized. Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; reload it, its required reference,
repository instructions and this checkpoint after a context shift.

[Daybreak handoff](ginux-contributions/daybreakblue-handoff.md) now records the
exact local receipts and failures. Of the previous 33 unrun security Recipe e2e
cases, 29 independently passed on pinned Ginux `436c4ae5` and Recipe test
`2ad833c`; four directly invoke guest sleep and remain unrun under G02. The
historical complete ledger is therefore 129 passed receipts, one observed
clock-policy failure and five unrun cases (four security, one other clock
case), across its original 135 names. This is **not** a complete current-source
suite pass. Two mixed Recipe negative-control targets passed separately.

Current local Ginux integration source is clean commit `c7b4b73a`, a narrow
`/dev/fd` namespace-escape errno fix for Claude's potential extraction. The
unchanged full script suite, `make check build`, all existing Ginux units,
native supervision and process-native pass on this source. Most other full
Ginux fixture families passed on the prior pinned source. The full storage
suite **fails** at `view-probes` on both revisions: `faccessat2` empty-path
access to an owned, open file returns `ENOENT`; the minimal trace confirms
open, write and fstat succeed. Storage symlink, mapping, ownership and
timestamp modes pass independently. No failing or unrun check is promoted.

A current-source ancestry check also shows the separate G03c resolved-contract
commit is absent from this local integration HEAD; G03c-specific B2 must not
be called a current runtime reproduction. The current source uses mapped
storage arrays, so B3 requires its own revision-specific audit. These findings
and the full remaining security scope are in the downstream handoff. Security
acceptance, G02, full storage, shared-argument and other focused adversarial
criteria remain open. The goal has made progress and remains **incomplete**.
Exact next action: investigate the current `view-probes` failure, audit the
remaining handoff requirements against this exact source, run controlled
independent checks, and rerun affected Recipe cases before claiming acceptance
for a later integration revision. Preserve Claude's inventory edits and all
historical recovery worktrees.

Current-source follow-up: two focused Recipe cases (owned executable permission
rejection; generated owned/image interpreter chains) passed on `c7b4b73a`.
All 23 selectable storage modes other than `view-probes` also passed separately
on that source with exact per-mode receipts. The full default storage suite
still fails at `view-probes`, and default-only path/assignment/no-storage
controls remain unrun after its early stop. Keep storage acceptance pending;
see the handoff's current-revision addendum for the receipt location.

Default-only storage isolation found an existing fixture selector collision:
`missing-owned` entered the earlier mapping branch. Local Ginux integration
commit `b4c69551` makes only `mapping` exact; both affected selections pass
with the rebuilt fixture. A corrected-fixture rerun of the unchanged full
storage runner still fails first at `view-probes` (guest149, runner1). The
current Ginux source is clean `b4c69551`; its immutable runtime remains the
`c7b4b73a` binary because `b4c69551` changes only fixture source. Eighteen
other default-only scenario selections passed through a disposable runner
copy; their receipts are not a full-suite pass. The downstream handoff records
source-visible shared-thread native-path scratch risk and the absent canonical
G03c ancestry. Exact next action remains the current `view-probes` diagnosis
and focused independent security checks, with G02 held and security acceptance
open. No upstream publication or Claude inventory edit was made.

### Current local storage correction — 2026-11-07

The full storage failure is now diagnosed and fixed in its existing owner.
A temporary diagnostic showed that the device hook returned `ENOENT` before
the filesystem hook could claim owned-file `faccessat2` with `AT_EMPTY_PATH`.
Local Ginux integration commit `6d290d48` routes that empty-path form by
its descriptor owner and adds allowed/denied device fixture controls.
The immutable binary `bin/ginux-6d290d48` has SHA-256
`c40cd3a75c5f479f1e761d53d02e3f12c4b748efc0f4b13eeefa2850cd8d7324`.
The unchanged **full** storage, device and FD launch suites pass on that
binary; Ginux `make check build` and its full unit suite pass on the same
source. The historical failed storage receipts remain evidence of the old
revision. Three focused Recipe consumers pass on the new binary, including
absolute host-path write rejection and owned executable permission denial.
The earlier 29 security Recipe passes were on `436c4ae5`; two of those have
now passed again on `6d290d48`, while 27 still need current-source reruns
before a current-revision suite claim. Four security cases and one other case
remain G02-dependent and held/unrun, with the observed G02 failure retained.

Current Ginux integration source is clean `6d290d48`. Security acceptance
remains incomplete: shared-thread writable syscall arguments, contract
admission/audit, assignment symlink authority, protected copyout, races,
negative transport, restore inputs and other handoff criteria still need
revision-specific validation. Exact next action: review those independent
security families against this source and run focused checks in dependency
order; preserve the G02 hold and Claude's upstream ownership. No push, PR or
upstream comment is authorized or made.

The current-revision Recipe security selection has now finished. The
machine-checked `security-ledger-6d290d48.json` in the local evidence
directory reconciles the 33 security names: **29 pass** individually against
the immutable `6d290d48` binary, **four remain unrun** because of the held
guest-sleep dependency, and none of the 29 failed or timed out. This
supersedes the earlier note that 27 security cases still needed a current-source
rerun. The historical ordinary 100 passes remain on `436c4ae5`; the one
observed G02 failure and one other held G02 case remain. Full 135-case
current-revision acceptance and the separate Ginux security handoff criteria
are still incomplete. Exact next action: source-review and controlled focused
validation of the remaining Ginux shared-argument, contract/authority,
copyout, concurrency, transport and restore security requirements, retaining
G02 and Claude's upstream ownership boundaries.

### G07 shared-path security reproduction — 2026-11-07

The current `6d290d48` integration binary **fails** the recovered G07
shared-path race on controlled disposable image files. The distinguished
variant returned guest99 for reading a byte from an image file hidden by the
`memory:/work` assignment; a separate guest100 thread/join error did not
occur. Protected clone-TID fault/rollback and concurrent public descriptor
alias scenarios passed independently on the same binary, with exact per-run
receipts and fixture hashes in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md).
No Ginux source or remote was changed by these probes.

G07 security acceptance is failed, not merely unrun. The next dependent step
is to make kernel-consumed native path and indirect arguments stable across
shared-VM threads without removing Recipe's threaded image/native filesystem
capability, then rerun the controlled failure and Ginux/Recipe regressions.
The tracee's original path and current stack scratch are both shared writable
memory; copying between them alone cannot satisfy that invariant. Independent
handoff checks may continue while this correction is designed. The G02 hold,
Claude upstream ownership and no-push boundary remain in force. Active skill:
`hdae-development` at `.agents/skills/hdae-development/SKILL.md`; reload the
skill, required reference, applicable instructions and this checkpoint after a
context shift. Goal active/incomplete.

### Current-source storage security narrowing — 2026-11-07

The local Daybreak probe on `6d290d48` confirms that launch accepts `memory:/link/nest` when `/link` is an image symlink. This is a source-specific assignment-validation gap; the probe did **not** demonstrate a host filesystem oracle. Claude's redesigned upstream storage plans startup rejection, so no local redesign was made. The old 2 MiB stack-overflow description is not directly applicable to this source: 2 MiB is invalid configuration, and storage metadata is mapped rather than embedded as a 2 MiB stack array. Valid 16 MiB instances with a 3 MiB process stack and both empty/configured storage launched successfully. The first invalid-input and fixture-expectation failures are retained, and exact receipts, source scripts and hashes are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). These checks do not resolve G07 or establish broad security acceptance. Exact next action: continue source-specific controlled checks for the remaining handoff requirements, especially stable native path/indirect arguments; preserve the G02 hold, Claude ownership and no-push boundary.

The unchanged Ginux `qfault` task selector also passed on `6d290d48`, covering four protected/unmapped/cross-page wait-status write faults, child consumption after `EFAULT`, and sentinel preservation. Its exact receipt is in the Daybreak evidence directory. This is a narrow current-revision G06b control; G07 and the broader security criteria remain open.

G07 inherited ignored-`SIGCHLD` controls have been narrowed on the exact `6d290d48` binary: a clone, a true-thread basic case, and both signal/failure teardown cases pass with accurate result/TID assertions. The first wrapper inherited ignored `SIGCHLD` into Python and produced invalid exit-code evidence for clone, basic and teardown; corrected child-only runners and all invalid receipts are preserved in the Daybreak handoff. The shared-path authority failure still prevents G07 acceptance.

### Current-source restore security narrowing — 2026-11-07

The unchanged full Ginux restore suite passed on `6d290d48`, including its malformed, traversal, duplicate, hardlink and assignment negative controls and its complete >2 GiB valid payload. A separate FIFO archive input promptly returned `supervisor-error/2` without guest execution. Exact commands, hashes and receipts are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). Hostile concurrent source changes and injected FD/read/close failures are still unverified. Source review further identified two G07 mutable-argument windows: dispatch and native rewrite reread the original guest pathname, and the kernel later consumes a rewritten name from guest-writable stack scratch. A single-copy correction is insufficient. G07 acceptance remains failed; the G02 hold and Claude upstream ownership remain unchanged. Exact next action: design and implement stable kernel-consumed native path/indirect argument handling in the local integration source, then rerun the controlled path race and affected ordinary/security suites.

### Local G07 tracing prerequisite — 2026-11-07

Configured launch now seizes its tracee before guest execution through a private startup gate, local Ginux integration commit `f9a584e9`. The unchanged full launch, task and thread suites pass with strict ginux-init clippy on the commit's binary; exact receipt and SHA-256 are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). This is a working prerequisite for interrupting shared-VM peers, not a stable-argument fix. The `x-path` race still reproduces guest99 on that binary. G07 security acceptance remains failed, and no new Recipe suite pass is claimed. Exact next action: implement stop/hold/resume around every kernel-consumed guest pathname and indirect argument while keeping threaded native image filesystem use, then rerun the failing race and all affected Ginux/Recipe regressions. Claude owns upstream extraction; no push or PR.


### Local G07 path hold and Recipe consumer — 2026-11-07

Local Ginux integration commit `e0519b483dac254ec2062e3201f511ea9c4b6d31` holds every running configured tracee while a pathname syscall is classified and consumed by the kernel. The hold remains active even when peers start parked, so they cannot wake into shared-memory mutation before the path syscall exits. The existing creation owner retains reserved child identity across kernel `-ERESTARTNOINTR`, and pending clone/fork peers are deferred at their guaranteed ptrace stop rather than interrupted or waited on in a cycle with an autoattached child. This is local integration work for Claude to evaluate; it was not pushed or prepared as an upstream packet.

The new `x-path` Ginux thread selector fails against the preserved seize-only `f9a584e9` binary with guest exit 99 and passes on the instrumentation-free final candidate. The unchanged full Ginux launch, task and thread suites, `make check build` with the dedicated Cargo target, and a fresh ordinary Recipe Cargo build/install consumer pass on that final binary (`SHA-256 3807a25284d442c8e456e8293fbb2b536acc3caf8bbaa3b1827632e631b3d0ab`; Cargo 81.708 s). Exact commands, hashes, logs and receipts are in the local Daybreak evidence directory and [handoff](ginux-contributions/daybreakblue-handoff.md). Two earlier guard candidates failed the same Cargo consumer with `path peer interrupt timed out`; those failures remain evidence and were not counted as passes.

G07 security acceptance remains **partial**. This check addresses the controlled shared-path race, not every kernel-consumed indirect argument, protected copyout, shared mapping, process transition, policy/authority or negative-transport edge in the Daybreak handoff. Current-source full Recipe suite and remote CI are unverified; the G02 clock/sleep policy and its cases stay held. Exact next action: continue source-specific G07 indirect-argument and transition checks on `e0519b48`, then rerun affected ordinary/security Recipe cases as fixes land. Preserve Claude's upstream ownership and no-push boundary. Active HDAE skill and reload requirement remain as recorded above; goal active/incomplete.


### Local exec path-hold follow-up — 2026-11-07

Source review after `e0519b48` found that a multistage exec transaction released the path hold after its first setup syscall, before the kernel consumed the final exec path and arguments. Local Ginux integration commit `1c2b1333c6a03fcd8f190dfb8921ce2583981be9` retains the hold until exec failure cleanup or the successful kernel exec event. The unchanged full launch, task, thread and exec suites pass on its binary (SHA-256 `15bb49d12b34b8d8a0d57126121e7aca70fdc424d7cfa3ef50ac2af530b7bca2`), and the fresh ordinary Recipe Cargo build/install consumer passes in 81.924 s. A corrected controlled live-peer exec control confirms the existing `-ENOTSUP` policy and clean teardown; the first disposable version timed out because its writer was not stopped after the expected deny, so its timeout is invalid as a Ginux failure. Exact receipts are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). No push or upstream action occurred.

The next dependency is the owned-file mapping transaction: `file_mapping.rs` writes a private socket/message frame into guest stack scratch and replays `socketpair`, `recvmsg`, `mmap` and closes. The current path hold does not cover that transaction, so shared writable arguments and temporary private descriptor authority remain security-held. Exact next action: protect that transaction's kernel-consumed scratch and FD lifetime under shared tasks, then run its protected/malformed/teardown controls and affected ordinary Recipe consumers. G07 and whole-goal acceptance remain partial; G02 and Claude ownership boundaries are unchanged.


### Local owned-mapping argument hold — 2026-11-07

Local Ginux integration commit `ad364bd76b8d54b58c90a6a19f3199b35c025eee` extends the configured-task peer hold to owned-file `mmap` admission and every stage of its private socket/descriptor mapping transfer. Initial and parked-retry entries now share one owner predicate; the hold stays active until `MappingCall` restores its scratch and completes. A positive concurrent-thread owned mapping case was added to Ginux's existing thread suite. The unchanged full storage, thread and exec suites, strict `make check build`, and a fresh ordinary Recipe Cargo build/install consumer pass on binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d` (Cargo 83.841 s). Exact receipts are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). No push or upstream action occurred.

This source correction now has a focused hostile-scratch regression, but complete mapping security acceptance remains pending: transient private-FD observation, nonleader/leader-exit races, fixed-region interference, rollback and malformed receipt controls still need focused checks. Exact next action: probe temporary private-descriptor authority during the mapping transaction, then nonleader/failure paths and the remaining G07/G12 indirect-argument and copyout criteria. The G02 policy hold, Claude upstream ownership, current-source full Recipe suite and remote CI remain pending. Goal active/incomplete; HDAE reload requirement unchanged.


### Mapping scratch race regression — 2026-11-07

Local Ginux integration commit `167f57b57a0e5da852099d359c214cf477415aa1` adds the controlled scratch-mutation case to the existing thread fixture and CI runner. It detects the supervisor's guest-stack message frame and changes its iovec pointer while a peer remains runnable. On the exact integrated fixture and runner, the pre-guard binary failed 20/20 with `mapping backing receipt failed`; the guarded `ad364bd7` binary passed 20/20. The unchanged full thread suite (including `k-scratch`) and `make check build` pass after the test-only change; exact fixture/runner/binary hashes and logs are in the [Daybreak handoff](ginux-contributions/daybreakblue-handoff.md). This narrows one shared-scratch risk; private descriptor authority, nonleader/leader-exit, failure rollback and broader G07/G12 acceptance remain pending. No push, PR or upstream comment.


### Private mapping descriptor guest-authority probe — 2026-11-07

Local Ginux integration test commit `c3a3f7b515b16d782f875f05329961f204a90a47` adds `v-private` to the existing thread fixture and runner. A concurrent peer repeatedly attempts `fstat`, `fcntl(F_GETFD)`, `close` and file `mmap` on candidate transient native FD numbers 5–7 while the owner performs 64 owned-file mappings. All attempts returned `EBADF`; the mappings and full Ginux thread suite passed on immutable integration binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `make check build` also passed. Exact command, source/runner/fixture/binary hashes and output are in `ginux-g12-private-fd-guest-probe.json/.log` in the Daybreak evidence directory. Source review confirms these guest calls resolve through the public descriptor table and the argument guard holds peers through `MappingCall` cleanup. The probe does not force a guest syscall at each transient stage; stage-specific authority, nonleader/leader-exit, fixed-map and failure controls remain pending. Exact next action: exercise owned mapping from a nonleader thread and after leader exit, then continue failure-path controls. G02 is held; Claude owns upstream extraction; no push, PR or comment occurred.


### Nonleader and post-leader-exit owned mappings — 2026-11-07

Local Ginux integration test commit `4075d9d599f7934b51bcc33ef1eb855743e6f145` adds `n-mapping` and `j-leader-map` to the existing thread fixture and runner. The first maps and unmaps an owned file from a nonleader thread; the second observes the leader as a zombie, releases the surviving thread from its input wait, then maps and unmaps the owned file. Both focused cases, the complete thread suite (including the private-FD and scratch controls), and `make check build` pass on the unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Exact command, fixture/runner/source/binary hashes and output are in `ginux-g12-nonleader-leader-exit.json/.log` in the Daybreak evidence directory. An initial edit failed to compile because its declaration was missed; a subsequent run used a stale fixture and produced guest 99. That output is invalid as a Ginux failure. The corrected rebuilt fixture passed. Fixed-region interference, mapping failure rollback, malformed receipt and broader G07/G12 criteria remain pending. Exact next action: test hostile fixed-region mapping rejection and preservation of code/stack, then mapping failure cleanup. G02 stays held; Claude owns upstream extraction; no remote action occurred.


### Fixed-region owned-mapping denial — 2026-11-07

Local integration test commit `fdeae41bd0b1c38edbf52af69cfcb18c516c9102` adds `q-fixed`: owned-file `MAP_FIXED` requests over the executing code page and the live stack scratch page both return `-ENOTSUP`; a stack sentinel is unchanged and a subsequent ordinary owned map/unmap succeeds. The focused case, full Ginux thread suite and `make check build` pass on the same guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g12-fixed-region.json/.log` records exact command, hashes, duration and output. This covers those two overlapping regions, not every hostile address layout or failure cleanup. Exact next action: inspect and exercise mapping failure rollback and malformed receipt paths, then remaining G07/G12 controls. G02 remains held, broad acceptance remains pending, and no upstream action occurred.


### Failed owned-mapping rollback — 2026-11-07

Local integration test commit `836061e13cf72ee05ca45c57b546de034c8e7855` adds `w-rollback`: an owned-file map with a page-aligned invalid high offset reaches `MappingCall` (audit confirms mediation) and returns a kernel error. With the tracee still alive at explicit handshakes, the runner observes native FDs `[0, 1, 2, 3]` both before and after. The guest stack sentinel survives, a subsequent ordinary owned map/unmap succeeds, and normal teardown reports no handles. The focused case, complete Ginux thread suite and `make check build` pass on the unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Exact command, hashes and output are in `ginux-g12-failed-map-rollback.json/.log`. This covers the kernel-map-error path; injected socket/receipt/close failures, malformed receipt and broader G07/G12 security criteria remain pending. Exact next action: inspect and exercise malformed receipt and private-stage failure cleanup without weakening ordinary checks. G02 remains held; Claude owns upstream extraction; no remote action occurred.


### Private socketpair failure cleanup — 2026-11-07

Local integration test commit `20b246ce955466dfc801b074d05164573fc8232b` adds `a-pair-fail`. After an explicit guest handshake, the runner lowers only the disposable tracee's soft `RLIMIT_NOFILE` to 5, inducing `EMFILE` at the private mapping `socketpair` stage. The guest receives `-EMFILE`; live tracee native FD snapshots remain `[0, 1, 2, 3]` before and after; the stack sentinel survives; and a normal owned map/unmap succeeds after the runner restores the limit. The focused case, complete Ginux thread suite and `make check build` pass on immutable guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Exact command, hashes, duration and output are in `ginux-g12-pair-emfile-rollback.json/.log`. This covers Pair-stage resource failure, not malformed Receive receipts, injected send/close failures or complete G07/G12 security acceptance. Exact next action: inspect and exercise receipt corruption and later private-stage failures with controlled injection, while preserving the ordinary successful suite. G02 remains held; Claude owns upstream extraction; no remote action occurred.


### Current guarded-source clone and wait copyout controls — 2026-11-07

The unchanged Ginux task fixture and runner passed five selected controls on the current guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`: `junk-child-tid`, `protected-child-tid`, `x-child-tid`, `rollback`, and `qfault`. The selected command exited 0; exact command, source/fixture/runner/binary hashes and output are in `ginux-g06b-copyout-current-guarded.json/.log`. These are current-revision results for the fixture's invalid/protected/cross-page child-TID and wait-status destinations and failure rollback, not proof of every indirect copyout path or full G07 acceptance. Mapping Receive receipt corruption and later private-stage failures remain pending. The G02 clock hold and Claude upstream ownership remain in force; no remote action occurred. Exact next action: continue source-specific remaining G07/G12 indirect-argument, receipt and copyout controls, with failure paths first.


### Mapping receipt corruption and transaction death — 2026-11-07

Two disposable host-side fault probes ran against the unchanged guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Both used a derived local fixture with an explicit handshake and repeated owned mappings; the production source and tracked test fixture were unchanged. `g12-malformed-receipt-host-probe.py` observed the private guest-stack mapping frame via the disposable tracee's `/proc/<pid>/mem` and changed its iovec pointer once. Ginux returned exit 1/result `supervisor-error`, reported `mapping backing receipt failed`, and logged `teardown tasks=0 owned-handles=0` (`ginux-g12-malformed-receipt-host-probe.json/.log`). `g12-mapping-death-host-probe.py` sent SIGKILL to the disposable tracee while that frame existed; Ginux returned exit 1/result `supervisor-error`, reported `task died during private mapping transaction`, and logged the same zero-task/zero-handle teardown (`ginux-g12-mapping-death-host-probe.json/.log`). Each receipt records source, fixture, script and binary hashes. These checks validate exact-source failure behavior under external host injection; they do **not** show guest access to the held frame or cover send/close failures. G07/G12 acceptance and the full current-source Recipe suite remain pending. Exact next action: inspect and verify remaining private transfer send/close failure paths and other G07/G12 indirect arguments. G02 remains held, Claude owns upstream extraction, and no remote action occurred.


### Current guarded-source mapping protection and lifetime controls — 2026-11-07

The unchanged Ginux storage fixture and runner passed selected `mapping`, `ordinary-mapping` and `ordinary-unlinked-mapping` on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Exact command, source/fixture/runner/binary hashes and output are in `ginux-g12-mapping-current-guarded.json/.log`. The `mapping` selector covers writable-shared denial, executable-write mprotect denial, unaligned offsets, fixed stack overlap, shared/private aliasing, resize/SIGBUS, copy-on-write isolation, unlinked backing lifetime, and a live tracee native-FD snapshot of `0,1,2`. The two ordinary selectors add positive and unlinked controls. This refreshes those fixture-specific current-source results; it does not cover every hostile layout, send/close failure or whole G07/G12 acceptance. Exact next action: review remaining private transfer send/close failure handling and indirect argument authority, then continue other Daybreak handoff items. G02 remains held and Claude's no-push upstream ownership remains in force.


### Current guarded-source configured view probes — 2026-11-07

The unchanged Ginux storage fixture/runner `view-probes` selector passed on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. `ginux-g09-view-probes-current-guarded.json/.log` records the exact selected command, exit 0, source/fixture/runner/binary hashes and output. The fixture checks owned-file `access` permission changes, AT-style access calls, empty-path fd access, readlink/readlinkat rejection of a regular owned file, and a permitted read. This refreshes one configured-view authority selector after the user's Daybreak authorization; it does not prove all symlink or assignment confinement cases. Exact next action: run source-specific symlink and cross-assignment checks, then continue the remaining G07/G12 private-transfer gaps. G02 stays held; upstream PRs #232–#234 remained open on read-only status check, so no rebase or remote action occurred.


### Current guarded-source full configured storage suite — 2026-11-07

The unchanged full `scripts/test-launch-storage.py` suite passed on guarded Ginux binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d` in 6.186 s, with exit 0 and summary `test-launch-storage: PASS (isolation, data, metadata, copy protections, bounds, fresh launch)`. `ginux-g09-storage-full-current-guarded.json/.log` records exact command and runner, fixture, source and binary hashes. It includes the default cross-boundary, image/owned symlink, hidden path, scratch restoration, view/access/readlink, mapping, bounds/quota and namespace cases; selected `symlinks` and `ordinary-symlinks` also passed separately. These are current-source fixture results, not exhaustive hostile namespace-race, archive-failure, G07/G12 or entire Daybreak acceptance. Exact next action: source-review and focus any remaining shared-argument/authority gaps beyond the existing fixture, then resume other documented security surfaces. G02 clock policy remains held. Upstream main is `59e2f7aa` (#235 rename) and wave1 #232–#234 remain open; per ownership split, rebase waits for their merge. No push, PR or comment occurred.


### Current guarded-source exec and script-chain suites — 2026-11-07

The unchanged full Ginux exec/interpreter runner passed on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`: five interpreter-ownership starts and seven exec scenarios including path, descriptor, relative and link selection, private native-FD inspection and CLOEXEC/rollback assertions. The unchanged full owned-script runner also passed all 36 named scenarios, including nested, descriptor, unlinked, dir-relative/renamed, fork/vfork/clone and cross-boundary script/interpreter cases; it checks private FD closure and failure rollback. Exact commands, output and binary/runner/source/fixture hashes are in `ginux-g10-exec-current-guarded.json/.log` and `ginux-g11-scripts-current-guarded.json/.log`. These are full existing fixture results on the current runtime binary, not a claim that every malformed chain, protected argument, host-path leak or shared-argument edge has been adversarially exhausted. Exact next action: review uncovered G10/G11 indirect-argument and malformed-input surfaces, then remaining Daybreak handoff criteria. G02 remains held; Claude owns upstream PR work; no remote action occurred.


### Forked shared-path exec regression — 2026-11-07

Local Ginux integration test commit `8601b606d81f478783ef03367e9d1fb1fac926e9` adds a cross-process shared pathname case to the existing exec fixture and runner. A forked child changes a shared anonymous path between allowed `/bin/fixture` and image `/work/hidden`, which is covered by `memory:/work`; the hidden path contains a marker script that exits 99 if executed. The parent retries exec until the allowed image starts, and the new image terminates/reaps the writer and prints `shared-exec: PASS`. The exact final fixture and runner passed the full exec suite **20/20** on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`; `make check build` passed. `ginux-g07-shared-exec-final-repeat.json` records command, 20 outcomes, source/fixture/runner/binary hashes and first-run output. The preserved pre-guard `f9a584e9` control failed 5/5 by exhausting retries with `ENOENT`; no hidden-script execution was observed, so that control shows a liveness difference, **not** a demonstrated authority bypass. The test narrows the shared-process exec path transition; argv/env and other indirect arguments still need focused review. Exact next action: inspect those remaining kernel-consumed exec inputs and private transfer send/close paths, then other Daybreak criteria. G02 remains held; Claude owns upstream extraction; no remote action occurred.


### Protected cross-page exec vectors — 2026-11-07

Local Ginux integration test commit `6cc21e69c25672672fd2838a70708d8b156ca16b` extends the existing `bad` exec fixture with argv and env vector arrays whose next pointer lies in a protected page. Both `execve` calls return `EFAULT`, and the fixture's existing descriptor/CLOEXEC rollback checks still pass. The full exec/interpreter suite (including forked shared-path exec) and `make check build` pass on unchanged guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`. Exact final source, runner, fixture and binary hashes and output are in `ginux-g10-crosspage-exec-current-guarded.json/.log`. This narrows two indirect-vector fault cases; malformed script stack bounds, all shared argument timings and send/close failure injection remain unverified. Exact next action: inspect the remaining G10/G11 indirect exec argument and script-stack surfaces, then other Daybreak criteria. G02 remains held, Claude owns upstream PRs and no remote action occurred.


### Protected cross-page owned-script vectors — 2026-11-07

Local integration test commit `b91b69c1df82d32daf1c1fa875d6011349eca6f6` adds argv and env vector arrays crossing into a protected page to the existing failed owned-script exec setup. Both return `EFAULT` and preserve the fixture's descriptor/CLOEXEC rollback checks. The unchanged full 36-case script runner and `make check build` pass on guarded binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`; `ginux-g11-crosspage-script-current-guarded.json/.log` has exact command, hashes and output. The first fixture revision left mmap's sixth offset argument unspecified and failed before exec with `EINVAL`; it is invalid Ginux-failure evidence. The corrected helper supplies offset zero. This narrows two script indirect-vector faults; other stack/AT_EXECFN bounds, shared mutation and private send/close faults remain pending. Exact next action: inspect the remaining script-stack/AT_EXECFN and private transfer edges, then other Daybreak criteria. G02 remains held; no upstream action occurred.


### Confirmed high-bit public descriptor alias — 2026-11-07

Source review found that current `DescriptorTable::get(u64)` casts the guest value to `i32` before lookup. A disposable fixture opened native public FD 4 on `/input`, then issued `read(0x100000004, ..., 1)`. On the exact guarded integration binary SHA-256 `0d8c6c0305da8d5c2e57bcb4da7458aaa3fe04a0a0427118f9137e50d8fbbb5d`, the call read the file's `a` byte and the guest returned the probe marker code **99**; `ginux-g12-high-fd-alias-probe.json/.log` records exact binary/fixture/source hashes and the `exited/99` result. This is a **failed descriptor-canonicality security criterion**, not a passing probe. It demonstrates a guest high-bit alias through the public descriptor table; it does not by itself reproduce the separate `filesystem.path.read` contract-deny bypass described for the old G03c layer. The first wrapper run could not read its root-owned result file and is invalid diagnostic evidence; the corrected root-run produced the receipt. The user's storage-redesign ownership rule assigns this fix to Claude upstream; no local redesign, PR, push or comment occurred. Keep G03c/G12 descriptor authority acceptance failed pending a corrected integration revision and fresh negative/positive tests. Other independent Daybreak checks may continue; G02 remains held. Exact next action: continue independent current-source security checks, then retest descriptor canonicality after Claude's redesign is integrated.

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
