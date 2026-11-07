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

The 2026-11-06 handoff initially reserved exploit-like validation for
DaybreakBlue. On 2026-11-07 the user designated this chat as the Daybreak
security model and explicitly authorized local security validation on disposable
fixtures for the documented scope. [DaybreakBlue handoff](ginux-contributions/daybreakblue-handoff.md)
records exact evidence and remaining criteria. Unperformed or failed checks
remain pending; preserve Ginux tests and CI. The G02 clock-policy hold and
Claude's upstream PR ownership remain in force. No cloud dispatch is implied.

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

On clean local integration `e354efd5` and exact binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture connected to one numeric loopback peer, sent `X`, and waited while that peer closed with `SO_LINGER(1,0)` to reset the connection. A nonleader shared-table thread then observed poll error readiness, consumed `SO_ERROR=ECONNRESET`, read EOF, and received `EPIPE` from `sendto(MSG_NOSIGNAL)` without termination. Its subsequent plain write received `EPIPE` and forwarded SIGPIPE; the launcher exited 141 with result `signaled/13`. The audit identifies virtual task 2's write, records `signal-forwarded signal=13`, task 2 and task 1 exits, and teardown `tasks=0 owned-handles=0`. The peer received exactly `X`. Exact probe script SHA-256 `6c74ab989a7988e8eeadf8bb25a6ee7e8e3dfa7b24aae4b2e55d311c8c66cac9`, generated C fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-thread-sigpipe-*` in the ignored local Daybreak evidence directory. The initial unprivileged invocation failed before guest execution because chroot requires host privilege; the corrected root run is the only runtime pass.

This verifies default SIGPIPE behavior from one nonleader thread after a controlled reset, including `MSG_NOSIGNAL` suppression and group teardown. It does not cover installed handlers, signal masks, `tgkill`-directed delivery, simultaneous network calls, a pending send, or other reset timings. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe a bounded pending-send pin or shared-thread cancellation path, then continue G30 rename/exec overlap and retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 shared-thread parked read cancelled by virtual SIGKILL — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture connected fd 5 to one granted numeric loopback listener and spawned virtual task 2 to read four bytes. The host observed the task's `syscall=0 action=park` on `lane-channel fd=5` before releasing virtual task 1. Task 1 printed `network-kill-ready` and issued virtual `kill(2,SIGKILL)`; audit sequence 15 records syscall 62 emulated with result 0. Both tasks exited, the peer saw EOF, launcher exit was 137 with result `signaled/9`, and teardown reported `tasks=0 owned-handles=0`. No read data was sent, so the parked call did not complete normally. Exact probe script SHA-256 `ed9da6a2f871c08ba31e9b082ce8a4ba7e9f134221b1fdb0c9d128ef3ef447b1`, generated C fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-thread-cancel-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized shared-thread network-read cancellation through virtual SIGKILL and eventual transport reclamation. It does not cover graceful per-thread cancellation, a signal mask or handler, pending send, simultaneous aliases, a concurrent public close, or all death interleavings. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe a bounded pending-send pin under transport backpressure, then continue G30 rename/exec overlap and retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 shared-thread pending send survives public slot replacement — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture sent 4,096-byte `A` blocks from nonleader task 2 to one numeric loopback peer that withheld reads with a small receive buffer. The configured transport budget was 16 MiB. The host observed task 2's `syscall=1 action=park` on `lane-channel fd=5` from audit sequence 447 before releasing task 1. Task 1 then used `dup2(4,5)` at sequence 455 to replace the public socket slot with its native image fd 4, read byte `a` from that replacement at sequence 458, and reported `send-slot-replaced`. Task 2 remained parked through sequence 461. After the peer drained the connection, task 2's pending write returned 4,096 bytes at sequence 463, then exited. The peer received exactly 1,798,144 `A` bytes and EOF. Launch exited 0/`exited/0` with `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact probe script SHA-256 `3f0af321357a18a4f5791c53e620821bdbea97f7baf73674311ec1c77db06b75`, generated C fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-thread-send-pin-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized backpressured in-flight send retaining its original socket after shared-table `dup2` replacement, including final transport cleanup. It does not cover send cancellation, peer reset during the park, partial backend writes, simultaneous aliases, variable payload mutation or all schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 rename/exec overlap and remaining G28/G29 backend-failure/revocation cases, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G30 shared-thread rename and ownership with exec denied — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured-thread fixture opened owned `/work/a` as dirfd 5 and created `item` within it. Peer task 2 repeatedly renamed `/work/a` to `/work/b` and back while leader task 1 attempted `execve("/bin/fixture",...)` and made 1,024 `fchownat(5,"item",0,0,0)` plus `fchown(3,0,0)` ownership pairs. The exec returned `-95` while the peer was live (audit sequence 18 in the first run; 17 in the traced run), and every checked ownership call succeeded. After the peer joined, relative and final `/work/a/item` ownership calls succeeded while `/work/b/item` returned `ENOENT`. Both runs exited 0/`exited/0` with exact `g30-rename-exec: PASS` and `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. A supervisor-only `strace -e trace=chown,lchown,fchown,fchownat` replay recorded zero host ownership syscalls. Exact ordinary and traced scripts SHA-256 `c6e1799af52095a1845954403dd64d132011c5cd45bf46676d685e8402911cfe` and `c13307f442f58b9307cce1e39fe9b59ac4c8b389efe3d146c91ff65b91e0c8f2`, generated fixture/binary, source/binary hashes, receipts, audits and trace are `ginux-g30-rename-exec-overlap-*` in the ignored local Daybreak evidence directory. The supervisor audit truncates after 65,536 bytes; the fixture's exit assertions cover the later ownership loop, while the trace covers only supervisor syscalls.

This verifies one shared-thread directory-rename/descriptor ownership schedule and fail-closed exec attempt before peer exit. It does not cover successful exec after join, simultaneous descriptor close/reuse, host mutation of the image, exact per-call rename snapshots, all interleavings or the known high-bit alias. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect G30 successful exec ownership/descriptor retirement and independent G28/G29 backend-failure/revocation paths, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G30 owned descriptor authority across successful exec — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured fixture opened owned `/work/a` as directory fd 3, created `item` as file fd 4, marked only fd 4 `FD_CLOEXEC`, then successfully executed the same image. The post-exec image found directory fd 3 still live and `fchownat(3,"item",0,0,0)` successful. `fchown(4,0,0)` and empty-path `fchownat(4,"",0,0,AT_EMPTY_PATH)` both returned `EBADF`; reopening `/work/a/item` reused public fd 4, and `fchown(4,0,0)` then succeeded with virtual UID/GID zero on `fstat`. The audit records `task exec virtual-pid=1`, the expected results, and teardown `tasks=0 owned-handles=0`; ordinary and supervisor-only traced launches both exited 0/`exited/0` with exact `g30-exec-owner: PASS`. The traced supervisor made zero `chown`/`lchown`/`fchown`/`fchownat` host syscalls. Exact scripts SHA-256 `3c60838825a6dc8bbfdb48a98cd942a0c31d174637ad04f4942c76467e009c47` and `608142efee4b4914ab356a1de1fc35a1c02d861210f3bdd115b7f8a9bbf61840`, generated fixture/binary, source/binary hashes, receipts, audits and trace are `ginux-g30-exec-owner-*` in the ignored local Daybreak evidence directory.

This verifies one successful single-process exec transition with independent owned descriptor `CLOEXEC` retirement and subsequent public-number reuse. It does not cover concurrent exec after a shared-thread join, late exec failure rollback, directory rename during successful exec, native/private FD collisions or all ownership authority schedules. G30 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect independent G28/G29 backend-failure/revocation and G30 native/private descriptor edges, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G28 Linux TCP adapter injected send/receive errors and quota — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture connected to one granted numeric loopback peer with an eight-byte transport budget. Supervisor-only `strace` injected `EIO` into the first host `sendto` and first host `recvfrom`. The guest saw `write("X")=-EIO` at audit sequence 4 and `read=-EIO` at sequence 6, then retried successfully: it sent exactly `PING` and read exactly `PONG`. Subsequent one-byte write and read both returned `EDQUOT` (`-122`), showing only the successful four-byte send and four-byte receive spent the budget. The peer received exactly `PING`, observed EOF, and no second connection. The host trace shows one `SOCK_STREAM|SOCK_CLOEXEC|SOCK_NONBLOCK` socket, one `EINPROGRESS` connect, the two injected calls, successful retries and `close(5)=0`. Launch exited 0/`exited/0` with exact `g28-io-fault: PASS` and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `342d52830642a42f096bec626b87a995c89ab4eeb471add7be8c22d5a9c79baf`, trace SHA-256 `17cd01e032d1464fa4c1d149eb6999909462c1fbf2ace8e944622cce1e484f82`, generated fixture/binary, source/binary hashes, receipt and audit are `ginux-g28-io-fault-*` in the ignored local Daybreak evidence directory.

This verifies one selected Linux adapter error/retry and budget-accounting path. `strace` skips injected kernel calls, so it does not prove actual kernel behavior under a real host I/O fault. Concurrent revoke/connect/close, in-progress cancellation, other backend errors and native FD lifetime under close faults remain unverified. G28/G29 remain partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe a bounded Linux adapter close/fault or G29 alias/revocation race, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 parked read wakes on TCP reset — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture connected to one granted numeric loopback peer, sent `X`, and entered a four-byte read. The host observed `syscall=0 action=park` on `lane-channel fd=3` before setting `SO_LINGER(1,0)` and closing the peer to reset the connection. The parked read resumed with `-ECONNRESET` (`-104`, audit sequence 8); a subsequent `SO_ERROR` query returned zero and a second read returned EOF. Launch exited 0/`exited/0` with exact result vector `[-104,0,0]`, `g29-reset-park: PASS`, and teardown `tasks=0 owned-handles=0`; the peer had received exactly `X`. Exact script SHA-256 `61f25e81543f2f5d73f56edae5a53a7ab549ca690e32b8e75a9ec430574c9a7b`, generated fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-pending-reset-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized reset of a parked single-task read and subsequent error/EOF ordering. It does not cover a parked send, concurrent aliases, shared-thread reset, simultaneous close, graceful FIN, or all SO_ERROR consumption schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe a bounded pending-send reset or G28 close/fault lifetime path, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 parked shared-thread send wakes on TCP reset — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured TCP fixture gave nonleader task 2 a 16 MiB transport budget and one numeric loopback peer that withheld reads. The guest installed ignored SIGPIPE, then task 2 sent 4,096-byte `A` blocks until its next write parked on `lane-channel fd=5` at audit sequence 448. Only after that audited park did the host reset the peer with `SO_LINGER(1,0)`. The parked write resumed with `-ECONNRESET` (`-104`) at sequence 453; the fixture reported 1,794,048 bytes accepted by earlier writes, exited 0/`exited/0` with exact `g29-send-reset: PASS` and `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. The peer did not read those accepted bytes, so this count is guest/backend acceptance rather than remote delivery. Exact script SHA-256 `89ef1abe4c156fa1540c7324f80962d75e64ac1eabc3f6e67a45eb68082d33ee`, generated C fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-pending-send-reset-*` in the ignored local Daybreak evidence directory.

This verifies one synchronized reset of a parked shared-thread send without guest termination under ignored SIGPIPE. It does not cover default SIGPIPE on a later `EPIPE`, delivery of queued bytes, concurrent alias close, partial backend writes, multiple senders or all reset schedules. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe G28 close/fault lifetime or G29 concurrent alias/SO_ERROR consumption, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.

### G29 shared-thread SO_ERROR consumption across socket aliases — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured-thread fixture granted one numeric loopback port that was bound but not listening. A nonblocking socket connected with `EINPROGRESS`; poll reported completion, and task 1 duplicated socket fd 5 to alias fd 6. After one shared start gate, task 1 queried `SO_ERROR` on fd 5 while task 2 queried it on fd 6. This run returned `ECONNREFUSED` (`111`) to task 1 and zero to task 2; the audit ordered the two queries at sequences 9 and 10. Subsequent read and write through the aliases returned persistent `ECONNREFUSED` even after the original public fd closed. Launch exited 0/`exited/0` with exact result vector `[111,0]`, `g29-soerror-thread: PASS`, `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. Exact script SHA-256 `41445f22c8f8b539bdfa9c242390a0a94533f2e0551897bfd7325ac63d456345`, generated fixture/binary, source/binary hashes, receipt and audit are `ginux-g29-soerror-thread-*` in the ignored local Daybreak evidence directory. The earlier serial alias-first refusal probe covers the opposite consumer order.

This verifies one same-start shared-thread query schedule with exactly one consumable `SO_ERROR` and a persistent refused connection state. Supervisor mediation serialized the two calls; this does not prove every guest scheduling interleaving, reset-state contention, malformed output pointers under concurrency, or alias-close races. G29 remains partial; G03c/G12 high-bit descriptor canonicality remains failed pending Claude's correction, and G02 clock policy remains held. Exact next action: inspect and probe a bounded G28 close/fault lifetime or G29 alias-close/connect race, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


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


### Daybreak G08a internal poll clock failure — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable configured fixture called zero-timeout poll on an empty owned pipe. The positive control returned `(0, 0)` and its supervisor trace showed one successful `clock_gettime(CLOCK_MONOTONIC)`. A second launch injected `EIO` into that supervisor call with `strace`; poll returned `-EIO` (`-5`) and left `revents` at its original `0x7f7f` sentinel. Both launches exited 0/`exited/0` with exact `owned-poll: PASS`, and audit teardown reported `tasks=0 owned-handles=0`. The injected trace explicitly marks the monotonic call `INJECTED`; audit sequence 2 reports poll `-5`. Exact probe script SHA-256 `929b93222c45cb95891435314bf3612d2961da2a34ea4932a996e83c0fb3db10`, fixture source/binary, control and injected traces, audit logs, hashes and receipt are `ginux-g08a-poll-clock-fault-*` in ignored local Daybreak evidence. The first probe invocation stopped at a fixture compiler warning and was corrected before either runtime case; it is not a Ginux result.

This verifies one internal supervisor monotonic-clock failure path and untouched poll output. It does not authorize or verify guest clock/sleep calls (G02 remains held), other clock errors, longer poll waits, cancellation, or every output fault. G08a remains partial; G03c/G12 descriptor canonicality remains failed pending Claude. Exact next action: inspect independent G06b/G07 security criteria and run the highest-value bounded ordinary case, then retest source-dependent storage after upstream changes. No Ginux source commit, upstream push, PR, comment or agent dispatch occurred.


### Current-source G06b copyout and G07 lifecycle controls — 2026-11-07

On clean local integration `e354efd5` and binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, both fixtures were rebuilt from current source. The unchanged task runner passed five selected G06b scenarios: `junk-child-tid`, `protected-child-tid`, `x-child-tid`, `rollback`, and `qfault` (exit 0). The unchanged thread runner then passed G07 `limit`, `rollback`, and both `teardown` variants (signal and supervisor failure), also exit 0. The runners assert their own guest exits, result files and teardown; the selected thread cases additionally check bounded creation and killed/stopped group cleanup. Exact wrapper SHA-256 `36b4946e26b4d31e6dfd014e5d7b16afc638fc96fd0d0a1392d16c0f66567634`, source/runner/fixture/binary/log hashes, commands and output are in `ginux-g06b-g07-current-lifecycle*` in ignored local Daybreak evidence.

These selected ordinary controls refresh current-revision behavior, not full G06b/G07 security acceptance. In source review, configured `rt_sigaction(SIGCHLD)` checks a 16-byte guest action for `SIG_IGN`/`SA_NOCLDWAIT` in `launch_tasks.rs`, then allows native kernel consumption of the same guest pointer; `needs_argument_guard` currently covers path calls and owned mappings, not this signal action. A concurrent shared thread may change the action between the check and kernel use; this is a source-level race hypothesis, not a reproduced bypass. Exact next action: run a bounded disposable shared-action control/probe on this revision, then correct the existing signal-action owner if it reproduces. G03c/G12 high-bit descriptor canonicality remains failed pending Claude, G02 guest clock policy held, and G07 broader indirect-argument/security and Recipe acceptance remain open. No Ginux source commit or upstream action occurred.


### G07 configured SIGCHLD shared-action race fixed locally — 2026-11-07

On clean pre-fix integration `e354efd5`, binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, a disposable stable-action control made 256 `rt_sigaction(SIGCHLD)` calls and exited 0. A shared-thread writer then changed only the guest action handler between default and `SIG_IGN`; after three successful calls and two denials, a query observed `SIG_IGN` installed despite the configured anti-autoreap check. The guest reported `(successful,rejected,exposed)=(3,2,1)` and exited 154. This is a reproduced shared-argument authority failure, not an ordinary runner pass. Baseline probe script SHA-256 `f848613dca25e474433fbeac7e1e9a212a2afcbdbc416438d570f11cb6f31622`, generated fixture, exact commands, audit, result and source/binary hashes are `ginux-g07-sigchld-action*` in ignored local Daybreak evidence.

Local integration commit `cde7dc22` (`Guard configured SIGCHLD action through kernel use`) adds the existing argument guard to that configured syscall before checking guest action bytes; peers remain stopped until kernel consumption completes. Ginux's existing thread fixture and runner now include stable and shared-action regressions. A separate-Cargo-target release build, `cargo fmt --check`, strict `ginux-init` clippy, focused regression, unchanged full thread runner and full task runner all passed on corrected binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402`; the three runner checks exited 0. Fresh Recipe `cargo-consumer` build/install also passed in 82.314 s, with `installed=true`, exact result `compiled rust 42\n`, two checkpoints acknowledged and launcher exit 0. Exact suite wrapper SHA-256 `57bfb60b1535b751d66eea2df86d217b8642b06b1eb200867d7171000b972770` and Recipe wrapper SHA-256 `b09e941b67c41f22a8d5952b46afc3c9f1b7d239ba1bc1d30db82e793f8ed76a`; `ginux-g07-sigchld-regression-suites*` and `recipe-cargo-sigchld-guard*` retain commands, hashes, logs and receipts.

This closes only the reproduced SIGCHLD action race on this local integration revision. Other kernel-consumed indirect arguments, thread teardown interleavings, full Recipe/F03 verification and G07 security acceptance remain open. G03c/G12 descriptor canonicality remains failed pending Claude, and G02 guest clock policy remains held. Exact next action: inspect the next configured kernel-consumed guest argument and verify its shared-thread authority with an allowed control; continue independent Recipe cases, then rebase after wave-one upstream merges. No push, PR, upstream comment, packet preparation or agent dispatch occurred.


### G07 SA_NOCLDWAIT shared-action regression — 2026-11-07

The second configured SIGCHLD policy condition was tested independently. A clean `git archive` of pre-fix integration `e354efd5` built in `/tmp/ginux-e354-sigchld-target`, separate from the active Cargo target, produced binary SHA-256 `e634fb1b2f4cb53f62b4a203c95f6982bf8f5cfc97ef34f9ba61ecbe903902e6`, exactly matching the earlier pre-fix binary. A disposable shared-thread fixture alternated only the `SA_NOCLDWAIT` flag in the guest action. Stable-action controls on pre-fix and corrected binaries both passed 64/64 calls. On the pre-fix binary the first flags run reported `(3 successful, 2 denied, 1 exposed)` and exited 153: a successful `rt_sigaction(SIGCHLD)` had installed `SA_NOCLDWAIT`. On corrected binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402`, three independent 1,024-call runs passed with no exposure; the `(successful,denied)` counts were `(325,699)`, `(295,729)`, and `(299,725)`. All launches reported complete teardown. Exact disposable comparison script SHA-256 `0e6e87f1766ea03816256006902d7ae3e9f61d485426d9e20e8f8eb039f773e4`, fixture source/binary, commands, audit logs, results and hashes are `ginux-g07-sigchld-flags-baseline*` in ignored local Daybreak evidence.

Local integration test commit `0d23d34d` extends Ginux's existing thread fixture and runner with `V-flags`. The focused stable/handler/flags selection and the full thread runner both passed, exit 0, on the unchanged corrected binary; wrapper SHA-256 `dc103c1837573a197cb8e1de58385ed8a3f9b5aa9ca674dbff566532352e3443` and `ginux-g07-sigchld-flags-runner*` retain exact commands/hashes/logs. This establishes the two configured SIGCHLD denial branches under the selected shared-write schedules, not all signal argument authority or complete G07 security acceptance. The exact next action is to inspect configured clone parent/child TID copyout under shared-memory mutation and failure cleanup with an allowed control. G03c/G12 descriptor canonicality remains failed pending Claude, G02 guest clock policy held, and full Recipe/F03 verification still open. No source runtime change, push, PR, upstream comment, packet preparation or agent dispatch occurred in this step.


### G07 parent-TID copyout fault teardown — 2026-11-07

On clean local integration `0d23d34d` and binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402`, the unchanged `basic` thread fixture passed under supervisor-only `strace`: exit 0/`exited/0`, `owned-threads: PASS`, and teardown `tasks=0 owned-handles=0`. The positive trace contains two four-byte `process_vm_writev` calls to the same parent-TID address `0x405080`: a preflight write of zero and a birth write of virtual TID 2; a later 16-byte path write is separate. Injecting `EFAULT` only on the second write produced launch exit 1/result `supervisor-error` with null result code, no guest stdout, `thread identity copyout failed`, and teardown `tasks=0 owned-handles=0`. The trace marks exactly that second call `INJECTED`; the traced parent PID no longer existed after the launch. The first injected run reached this same behavior but its wrapper incorrectly expected a numeric supervisor-error code; its trace/audit/receipt remain as `*-initial.*` and are not counted as a verified pass. The corrected wrapper passed. Exact positive and fault wrapper SHA-256 `31846e56f00dd52dd5783b0c0ecc2effe81505fe043d7402c43e5220c5c35320` and `6ff3fed8a6275c87809dfb2bb808632e51828ae6087e9c5143ab0a1bbd601dec`; `ginux-g07-parenttid-*` ignored local evidence retains commands, hashes, traces, audit and results.

This verifies one supervisor write-failure cleanup schedule, not a guest-driven shared-memory mutation, every clone/birth interleaving, or rollback of a partially written TID word. G07 security acceptance remains partial. The exact next action is to build a bounded guest peer that changes only a dedicated parent-TID page around clone and check both safe success and failed-birth teardown without confusing a synthetic fault with a guest race. G03c/G12 descriptor canonicality remains failed pending Claude, G02 guest clock policy held, and full Recipe/F03 verification open. No Ginux source commit or upstream action occurred.


### G07 shared parent-TID page race fixed locally — 2026-11-07

A disposable configured guest used a dedicated mapped parent-TID page. Its ordinary control completed 16 clones with vector `(16 successes, 0 EFAULT, 0 capacity)`. With a peer toggling that page between read-only and writable, pre-fix local integration `0d23d34d`/binary SHA-256 `d7b4ff2585fe49ef3ce3fb133f0d948b6895cce52bc77e6dfd076aee250fa402` first handled some preflight `EFAULT` calls, then entered another native clone and failed at birth with `thread identity copyout failed`. Launch exited 1/result `supervisor-error` with null code and teardown `tasks=0 owned-handles=0`. Exact baseline script SHA-256 `d5b3e0403c8340a3f7cc110ffccc30bfd660420c2b58c94cfc352b0f90a16e60`, generated fixture, audit, source/binary hashes and receipt are `ginux-g07-parenttid-guest-race*` in ignored local Daybreak evidence. This is a guest-driven protection race; the earlier `strace` fault control was a separate synthetic failure-path check.

Local integration commit `a0419e24` (`Hold shared TID page through thread birth`) extends the existing configured argument guard only to the two supported shared-thread clone flag words, keeping peers stopped from TID preflight through virtual-ID birth copyout. The existing Ginux thread fixture and runner now include ordinary `U-clone-tid` and shared-protection `T-clone-tid-race` modes. An initial corrected-binary fixture run exited guest 166 because its assertion demanded a clone success during the race even when all 16 attempts safely returned preflight `EFAULT`; this was a fixture expectation error, not a supervisor failure. The corrected fixture requires an explicit successful clone after the peer stops and restores write access. On corrected binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`, a diagnostic fixture passed ordinary `(16,0,0)` and three shared race vectors `(3,13,0)`, `(1,15,0)`, `(0,16,0)`, all exit 0/`exited/0` with `owned-threads: PASS` and complete teardown. Diagnostic script SHA-256 `41e42aadf9097d46d7a1876026c2c69c5d64e8728eabece7dcb9acc570998b6f`; `ginux-g07-parenttid-guarded-*` holds hashes, outputs and audit.

The focused clone selection, unchanged full thread runner and full task runner passed on the corrected binary; release build, `cargo fmt --check`, and strict ginux-init clippy passed. Fresh Recipe `cargo-consumer` build/install passed in 82.316 s with `installed=true`, exact result `compiled rust 42\n`, two checkpoints acknowledged and launcher exit 0. Exact suite wrapper SHA-256 `d6f2eadb7b12b3cb24ac7d0462ca0f388531094e7f0dd0d03c46440fcfac37a3`, Recipe wrapper SHA-256 `31acf96dfc95e9cccbc8025358e768e621fc338db271968d3c72efaed2cf7a61`; `ginux-g07-parenttid-regression-suites*` and `recipe-cargo-parenttid-guard*` retain commands, logs and hashes. This covers one shared page-protection schedule, not every clone/birth interleaving, concurrent unmap/reuse, cross-page partial write, nonleader fault, full Recipe/F03 or remote CI. G07 remains partial; G03c/G12 high-bit descriptor canonicality failed pending Claude, and G02 guest clock policy held. Exact next action: inspect the remaining cross-page/nonleader TID copyout and indirect shared-argument surfaces on `a0419e24`, then retest source-dependent storage after upstream wave-one merges. No push, PR, upstream comment, packet preparation or agent dispatch occurred.


### G07 cross-page parent-TID rollback — 2026-11-07

On clean local integration runtime `a0419e24` (binary SHA-256 `a452adc59a88fb202470f7999bddd9bc1c2486af61a63919ad95eb4137a68015`), a disposable thread fixture placed its parent-TID word across a page boundary. With the tail page inaccessible, clone returned `-EFAULT`; with the head page readable but read-only and tail writable, clone again returned `-EFAULT` after the strict partial-write/rollback path. All four original bytes remained `11 22 33 44` in both cases; a later aligned valid clone returned virtual TID 2. The exact guest vector was `[-14,-14,2,0x44332211]`, with launch exit 0/`exited/0`, `owned-threads: PASS` and teardown `tasks=0 owned-handles=0`. Exact disposable script SHA-256 `34fd046117efdd7b3dd889ba874f98127451d79bc3d6ee2c06702eaf55d29877`, generated fixture source/binary, command, audit, hashes and receipt are `ginux-g07-parenttid-crosspage*` in ignored local Daybreak evidence.

Local Ginux integration test commit `5dc0fd8c` adds `B-cross-tid` to the existing thread fixture/runner. The focused case and full thread runner passed, both exit 0, on the unchanged guarded binary; exact runner wrapper SHA-256 `4c80bfa31709c168461191d2170d0b7dd6d4bb7ba71852cd8857a200ddc5ecfa` and `ginux-g07-parenttid-crosspage-runner*` contain commands, hashes and logs. This covers selected cross-page parent-TID preflight and rollback, not a fault occurring after native clone, all nonleader clone paths, every concurrent unmap/reuse schedule or complete G07 acceptance. G03c/G12 descriptor canonicality remains failed pending Claude, G02 guest clock policy held, and full Recipe/F03 verification open. Exact next action: inspect and exercise nonleader parent/child-TID birth fault behavior and remaining configured indirect arguments on the current source, then source-dependent storage after upstream wave-one merges. No runtime source change, upstream push, PR, comment, packet preparation or agent dispatch occurred.

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

### Current G09a malformed image extent fix — 2026-11-07

On local integration `158e11a6`, an authorized disposable release probe built a sorted two-record image index with `/file` size 2, `data_offset=u64::MAX`, and backing bytes `AB`. A valid offset-zero control sought to file position 1 and returned `B`; the malformed record was accepted and the same read returned `A` from backing offset zero. This **failed** the selected G09a malformed-offset confinement criterion: the VFS read path wrapped `entry.data_offset + offset` in the release build. The result is limited to this synthetic index and strict two-byte reader; no broader runtime disclosure claim is made.

Local Ginux integration commit `b168c2c5` rejects file index records whose declared offset plus size overflows `u64`. The corrected release probe returned `Err(InvalidIndex)` for the malformed record and kept the valid `B` result. A copied byte-identical VFS crate ran 42 unit tests with 0 failures using separate target directory `/tmp/ginux-g09a-unit-target`. Direct workspace unit-test attempts failed to link because its freestanding `rust-lld -nostdlib` flags also reached the host test binary; those attempts are not counted as passing tests. `git diff --check` passed. The ignored local `ginux-current-g09a-offset-wrap-b168c2c5/` bundle retains pre/post probe source and outputs, VFS source snapshots, unit logs, commands and SHA-256 manifest. G09a backend-length behavior, other malformed records and configured runtime acceptance remain unverified. The separate Recipe e2e host process was still live at the last check and its partial log already contained failures; full suite status remains pending. G03c assignment confinement, G03c/G12 descriptor canonicality and low-stack launch remain failed; G02 clock policy held. Exact next action: inspect the Recipe e2e result when complete, then continue remaining independent Daybreak criteria and retest Ginux behavior after upstream wave-one merges. No fork push, upstream PR or comment occurred.

On the corrected local integration `b168c2c5`, a separate release-mode G09a backend control used a two-byte file and a disposable reader initially exposing one byte. Its first two-byte read returned `Err(Io)`, partially changed only the caller buffer to `A?`, and left the VFS position at zero. After the reader exposed the second byte, the same descriptor read `AB` and advanced to position two. The probe exited 0 with `g09a-short-backend: PASS`; `ginux-current-g09a-backend-failure-b168c2c5/` retains exact source, command, stdout, receipt and hashes. `HostRootfs::read_at` source requires a full-length `pread64`, but this probe did not induce an actual host short read or test concurrent backing changes. G09a remains partial; full Recipe e2e is still pending its independent run.

### Selected current G06b copyout fixtures — 2026-11-07

The existing Ginux task fixture was rebuilt without source changes and run as root against the preserved configured binary from integration `158e11a6` (SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`). `scripts/test-launch-tasks.py` selected `qfault`, `protected-child-tid` and `x-child-tid`; all three printed PASS and the runner exited 0 after asserting each launcher result, guest output and complete teardown. `qfault` covers four read-only, unmapped and cross-page wait-status destinations, each returning `EFAULT`; a second wait returned `ECHILD`, and protected/adjacent sentinel bytes remained unchanged. The two child-TID cases cover a read-only word and a cross-page word: the fork-like clone succeeded and sentinel bytes stayed unchanged in the child. The ignored `ginux-current-g06b-copyout-158e11a6/` bundle retains exact fixture, runner, binary reference and hashes, stdout/stderr and receipt. The later local VFS-only commit `b168c2c5` was not rebuilt into this binary, so these are exact `158e11a6` runtime results. They do not prove every clone flag, race or rollback path; G06b/G07 remain partial and the reproduced shared-path failure remains unresolved. The independent Recipe e2e suite was still live at its last direct check with 13 partial-log failures, so full suite status is pending. Exact next action: inspect that final result when available, then continue source-specific Daybreak cases and upstream-dependent storage retests. No upstream push, PR or comment occurred.

### Current G07 shared-path selected run — 2026-11-07

The existing `x-path` thread fixture was rebuilt and run once against the preserved configured Ginux binary from integration `158e11a6` (SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`). It made 2,048 calls while a peer toggled the pathname between allowed `/allowed` and image `/work/hh` hidden by `memory:/work`. The runner exited 0 with `PASS launch threads: x-path` and full teardown; this run did not observe hidden byte `S`. It is one stochastic negative result, **not** complete G07 security acceptance. The earlier `6d290d48` reproduction exposed `S` and exited 99, but that revision predates the local peer guard. Although `path_hooks::Rewrite::prepare` uses writable tracee stack scratch, current `launch_tasks.rs` stops running peers before pathname classification and holds them through the kernel syscall. The prior source-only assertion that the current scratch race remained unguarded was incorrect; the paired regression below verifies this specific protection. The ignored `ginux-current-g07-xpath-selected-158e11a6/` evidence bundle retains exact fixture/runner, selected stdout, exit code and hashes. The later local VFS-only fix `b168c2c5` was not built into this pinned runtime. G07 remains partial; other indirect arguments and interleavings are pending. The independent Recipe e2e suite remains pending its final output; G02 clock policy held and G03c storage defects remain failed. No fork push, PR or upstream comment occurred.

### Current owned-script cross-page vector tables — 2026-11-07

An authorized disposable root on pinned configured Ginux binary from integration `158e11a6` (SHA-256 `e019cc90bfd6682a8d4ab420eaa17de245c2e18f80b49a9f2603ac17f01344b8`) placed the first argv pointer, then the first envp pointer, in the final eight bytes of a writable page; the next pointer slot lay in a `PROT_NONE` page. Both `execve` calls on the same owned `memory:/work` script returned `-EFAULT` (`-14`); the envp first-pointer word remained unchanged. A later valid vector executed through `/bin/second` in a child, exited 77 and was reaped. Launcher exited 0/`exited/0`, stdout was `owned-vector-crosspage: PASS`, untruncated 3,939-byte audit showed both denials and `teardown tasks=0 owned-handles=0`. The ignored `ginux-current-owned-vector-crosspage-158e11a6/` evidence directory retains runner, generated source/binaries, config, audit, result, stdout and hashes. Its receipt records checkout `b168c2c5` at probe time, but the runtime binary was built from `158e11a6`; the intervening local fix changes only VFS index validation. This verifies one selected G11 cross-page pointer-table boundary, not every alignment, shared mutation, pointee-string behavior, auxv bound or full G11 security acceptance. G07 shared-path authority remains failed; G03c storage defects remain failed; G02 clock policy held and full Recipe e2e pending. Exact next action: inspect final Recipe e2e output when available, then continue independent G11/Daybreak cases and upstream-dependent retests. No fork push, PR or upstream comment occurred.

### Current G07 native pathname scratch regression — 2026-11-07

Local Ginux integration test commit `f4b2e240` adds `X-scratch` to the existing thread fixture and runner without changing runtime behavior. The fixture keeps its original pathname `/./allowed` constant, exposes the exact syscall stack pointer, and has a shared peer overwrite the supervisor's `rsp-512` canonical-path scratch with `/work/hh`. The image `/allowed` contains `A`; image `/work/hh` contains `S` but is hidden by `memory:/work`. The runner allocates two vCPUs, checks an allowed `A` control, requires at least one allowed `A` read during the race, rejects unexpected read/close results, and fails if `S` is observed.

With the same final fixture/runner, the preserved pre-guard `f9a584e9` binary (SHA-256 `0dca4cc81193b3869e7a9377fadda4678479086518bccec214496510bf77ed0b`) returned hidden byte `S`, guest exit 205 and runner exit 1. The current guarded binary built from `f4b2e240` (SHA-256 `12fea894f0545d447e48750e5a8a55a06577a503dc2a62ccbd8e4ffab9fa6c79`) passed the focused case with launcher exit 0/`exited/0` and complete teardown. The full configured thread runner, now including this case, passed. `make check`, a release build using separate `/tmp/ginux-g07-scratch-regression-target`, and the unchanged full configured storage runner passed on the same source/binary. `ginux-current-g07-scratch-regression-f4b2e240/` under ignored local Daybreak evidence retains exact source/fixture, binary, commands, pre/post/full-suite logs, exit statuses and hashes.

Current `needs_argument_guard` includes pathname calls; `begin_argument_guard` stops running peer tasks before classification and rewrite, and the hold lasts through the native syscall exit. This **supersedes** the prior current-source assertion that writable stack scratch alone meant G07 path authority was still failing. The old `6d290d48` failure predates this guard. This paired regression establishes selected native path-scratch confinement, not full G07 acceptance: other indirect arguments, scheduling paths, private descriptors and mapping/exec transitions remain partial or unverified. An exploratory FIFO peer-stop diagnostic could not exercise its intended path because `ImageRoot::lookup` denies FIFO metadata; it is not a Ginux security failure or a passing guard check. The separate Recipe e2e process had 13 failures in its partial log and was still live at the last direct check; final suite status remains pending. G03c assignment confinement and descriptor canonicality remain failed, G02 clock policy held. Exact next action: inspect the completed Recipe e2e result when available and continue remaining independent G07/G12 security cases; retest storage after upstream wave-one merges. No fork push, PR or comment occurred.


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
