# Recipe simplification implementation plan

Updated: 2026-11-06. Status: implementation active under the user’s “go and make a goal” authorization.

This is the authoritative remaining-work queue for Recipe simplification and
its local Ginux dependencies. It supersedes the old S0–S7 and F/G/R/L/I/N/U/D
checkbox states from the previous plan and in [the Ginux implementation history](ginux-recipe-implementation-plan.md).
Older entries remain in Git history as evidence of what ran on particular
revisions, not current instructions. [The interface record](ginux-recipe-interfaces.md) remains the
cross-process reference and must track changes made by these tasks.

## Current direction

- Recipe has **zero external users**. Maintain one current interface. Make direct
  breaking changes and update all in-repository callers, recipes, fixtures and
  examples. Do not add adapters, deprecated aliases, old-format readers,
  migration tools, dual execution paths or a compatibility period.
- Use the existing API 2 semantics as the sole recipe contract: setup once,
  explicit `ready(ctx)` / `needs_work(ctx, reason)`, and thrown errors are errors.
  Keep the three independent custom readiness hooks and meaningful cleanup.
  `RECIPE_API = 2` may remain a strict format marker, not a choice of engines.
- Keep `RUNTIME` available during setup and `ctx.runtime`, `ctx.state`,
  `ctx.outputs` during hooks. Remove duplicated compatibility globals and
  whole-context persistence. Package facts and custom checks remain recipe-owned.
- Use one coordinator/worker protocol and state format. Reject unsupported
  versions without translating them or silently deleting old state. Explicit
  removal of disposable development state is sufficient; preserve source inputs
  and published artifacts.
- Ginux is mandatory for every recipe-code operation. Remove public backend
  selection machinery instead of retaining host/in-process selectors for migration
  errors. Keep launcher/image configuration and fail clearly when unavailable.
- This plan is Linux/x86-64 only. Live-system activation, macOS, package solving,
  global ownership databases, A/B activation and complete Linux ABI compatibility
  are outside this effort.
- **Do not create Ginux PRs, push Ginux branches, or modify existing Ginux PRs as
  part of these tasks.** Keep necessary local Ginux work and record prospective
  contributions in the backlog below. Existing PRs are not implementation gates.
  Upstream policy decisions still govern future upstream proposals, not our
  ability to inspect or develop the local integration.
- Keep work proportional: existing behavior owners, direct fixes and meaningful
  CLI/runtime verification. No new workflow framework, approval ceremonies,
  artificial line-count targets or kernel rebuilds merely to test recipe changes.

## Verified starting point

| Component | Inspected revision | Already demonstrated; preserve rather than rebuild |
| --- | --- | --- |
| Recipe, `tools/recipe` | `85187b2` on `ginux-recipe/r06-launch` | Required Ginux; protocol v5; explicit inputs/options; separate state/files checkpoints; named publication including symlinks; compiled transitive tools; 46 managed scenarios in the latest recorded run |
| Local Ginux integration | `d7f628cc` on `codex/ginux-recipe-local-integration` | Configured processes/threads, owned storage and mappings, script chains, snapshots/restore, bounded channels and symlinks |
| Distro-builder | `97ce3a9` on `codex/streamline-development` | Canonical API 2 recfstab and explicit-input adapter; real build/run/publication and six-invocation reuse/removal/reinstall/cleanup exercise |
| Upstream Ginux, last verified | `2489517e` | Integration branch still needs reconciliation with this newer upstream history; the three small VFS PRs were prepared independently |

The inspected starting filesystem lacked true owned hardlinks, owned-directory cwd and
cross-image/owned link traversal. The namespace has 4,096 object slots, while
Recipe publication and Ginux restore have 2 GiB bounds. Cargo reports SQLite
cache-locking errors. Acquisition helpers have earlier host-side evidence, not
complete managed-network evidence. Managed autofix is explicitly rejected.
`recipe info` still scans source lines. Many production recipes remain unversioned.
These observations, rather than historical unchecked boxes, define the work.

## Four parallel lanes and file ownership

Use up to four active implementers, with the coordinator also implementing lane C.
The user authorized this rolling parallel schedule with “go and make a goal”. Keep
existing branch/worktree choices; this plan does not authorize new branches.
Paths below are repository-relative. Ginux means the integration worktree at
`/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux`, not the old
`tools/ginux` checkout or the three published PR worktrees.

| Lane | Exclusive physical writer | Responsibilities and handoffs |
| --- | --- | --- |
| A — Ginux filesystem/runtime | Ginux `crates/ginux-vfs/src/lib.rs`; `crates/ginux-init/src/{fs_hooks,launch,launch_tasks,supervisor,descriptors,file_mapping,storage_memory,storage_export,storage_restore,checkpoint_control,dispatch,main}.rs`; shared Ginux manifests/config composition | Upstream reconciliation, cwd/links, storage bounds, locking, central syscall wiring and resource validation. Applies B's exact network-dispatch joins and coordinates archive changes with C. |
| B — Network/acquisition | Ginux network device implementation in `crates/ginux-devices/src/lib.rs`; Recipe `src/helpers/acquire/**`, related acquisition CLI scenarios | Bounded network design/backend and canonical acquisition helpers. Supplies parser/dispatch changes to A rather than concurrently editing A's files. A also owns `ginux-domain/src/lib.rs` during shared config changes. |
| C — Recipe engine/coordinator | Recipe `src/{lib,backend,launch,retained,publication,temporary,options,logging}.rs` and `src/inputs/**`; `src/core/**`, `src/worker/**`, `src/helpers/{build,install,internal,util}/**` and helper registration, `src/bin/recipe/{cli,commands}.rs`, `src/bin/recipe.rs`, `src/bin/recipe-image.rs`; integration/protocol fixtures | Single current semantics, target views, state/publication, cancellation/logs, cleanup, retry and final integration. Owns shared manifests, protocol fields and test registration in `tests/e2e.rs`. No concurrent writers inside this set. |
| D — Recipes/consumers/docs | `distro-builder/recipes/**`, `distro-builder/src/recipe/**`, affected `distro-contract`, `distro-variants`, `xtask`, `AcornOS/deps`, `leviso/deps` and their callers; Recipe `src/bin/recipe/metadata.rs`; consumer CLI scenarios and docs | Direct recipe/caller migration, static metadata consumer, removal of old entry points and operator documentation. Requests changes to C's API/loader instead of editing it. |

Each lane owns the tests for its behavior; shared fixture helpers and module
registration have one C writer. Root and crate manifests/lockfiles have one
coordinator writer. D writes shared planning/interface documentation from the other lanes’ supplied edits. During T02, A alone writes the Ginux checkout; B’s T03 is design/fixture preparation only. A file handed to another lane is released by its former
writer until returned. Do not parallelize edits to different hunks of the same
file. Network-specific fixtures can run independently; expensive compiler,
rootfs and VM validation share one operator to avoid host resource contention.

## Remaining tasks in dependency order

Completed focused tasks: **T02, T03, T05, T06, T08, T10, T20**.
Active or partial: **T01, T04, T09, T11, T13, T14, T15, T18, T19**.
Other tasks remain todo; partial source preparation is not runtime acceptance. Dependencies refer to completed implementation and
the stated evidence, not merely a draft or a compiling helper. The list is a
valid topological order; lane availability also constrains execution. Source
preparation may overlap a dependency, but do not mark its dependent behavior
complete before the real prerequisite works.

| ID | Lane | Depends on | Task and completion evidence |
| --- | --- | --- | --- |
| T01 | C | — | **Make Recipe's engine/API single-contract.** Remove legacy throw-as-not-ready, repeated top-level evaluation, backend selection shims, old persistence names including `--no-persist-ctx`/`with_ctx_persistence`, and duplicated engine globals. Use current runtime/options/state names in CLI, library and worker. Expose one non-evaluating declared-metadata reader through the existing loader for T20. Pass dependency PATH/tool environment through invocation-owned command options instead of mutating process-global environment. Update core CLI fixtures; real API 2 setup/import/check/error and source-immutability cases pass. Unmigrated production callers are tracked by T04/T14/T15/T19/T23, never kept alive by a new adapter. |
| T02 | A | — | **Reconcile local Ginux with upstream.** Preserve local integration/stashes and upstream tests; identify already-landed behavior before combining changes. Build and rerun the existing configured-launch/storage/exec/checkpoint cases. Record the resulting base and head; do not rewrite published PR branches. |
| T03 | B | — | **Specify the smallest needed network contract.** Map actual HTTP/Git/asset consumers to bounded destinations, DNS, redirects, TLS verification, connection/readiness limits, cancellation and offline fixtures. Prefer bounded HTTPS sources; do not build unrestricted peer networking solely to retain an old torrent helper. Record actual required syscall/backend joins and a testable allowed/denied endpoint fixture. No upstream proposal is sent. |
| T04 | D | — | **Convert offline recipe sources and dependency declarations.** Use the contract above to migrate custom-rootfs and tool/package recipes, including recstrap/recchroot and recfstab callers, directly to current readiness/runtime/state/output fields. Inspect the 35 discovered Rhai files and their Rust callers; mark each as shared module, retained recipe or evidenced unused removal. Compile syntax without host evaluation; existing recfstab behavior remains intact. Full consumer execution belongs to T14. |
| T05 | A | T02 | **Implement owned-directory cwd.** Route chdir/fchdir/getcwd and relative operations through owned task state, including fork/exec and directory-fd semantics. Real programs enter newly created build directories; invalid/removed paths fail without entering hidden image or host paths. |
| T06 | A | T02 | **Implement true owned hardlinks.** Names share identity, bytes and link counts; open references survive unlink. Bound allocation and rollback. Real link/write/unlink/rename cases prove identity; byte copying is not accepted as a hardlink implementation. |
| T07 | A | T05 | **Complete image/owned symlink traversal.** Use the existing VFS resolver to cross declared image/storage boundaries with correct relative targets, cwd and directory-fd behavior. Retain assignment enforcement, nofollow and loop bounds; test both permitted traversal and denied boundary escapes. |
| T08 | B | T02, T03 | **Implement the bounded network backend.** Reuse Ginux device/backend ownership, connection budgets and revocation. Exercise the real backend against allowed/denied fixture endpoints. It is not a working recipe-network claim until T13 wires supervised calls. |
| T09 | C | T01 | **Connect explicit installed-target and cache views.** Extend existing input/identity/locking owners, not host-path defines. Checks/removal inspect the requested imported or retained target; immutable compiler-image files cannot establish installation. State identity reflects target/cache input content; actual CLI cases cover changed/missing target products and independent destinations. |
| T10 | A | T06 | **Make storage capacity suitable for rootfs products.** Replace the fixed namespace bottleneck with bounded allocation and account shared hardlink data once. Define enforceable object/byte limits and archive identity for C; do not merely raise giant stack arrays. A representative tree exceeding 4,096 entries works; object/byte exhaustion is explicit and teardown releases resources. |
| T11 | C | T01, T06, T10 | **Align inputs, export/restore and publication with that storage contract.** C owns Recipe readers/writers; A applies the coordinated Ginux archive changes. Preserve hardlink identity, modes and symlinks across checkpoint/restart/publication and support required rootfs sizes above the old 2 GiB cap under explicit bounds. Verify malformed/traversing archives, interrupted export, previous-publication preservation and matching state/files acknowledgements. |
| T12 | A | T02 | **Fix the observed Cargo/SQLite locking gap.** Reproduce the actual cache operation, implement only its needed descriptor/lock behavior with proper ownership/release, and run real concurrent Cargo/cache use. No ignored warnings, disabled locking or broad syscall allowances as a substitute. |
| T13 | A | T01, T08 | **Wire network syscall/readiness support.** Apply B's config/dispatch integration, including only the socket operations required by T03's chosen clients. C wires explicit invocation network policy into Recipe inputs/config and its single current protocol; networking stays off unless assigned. Real supervised clients fetch fixture bytes, reject disallowed destinations and terminate/revoke cleanly. No native host-network fallback. |
| T14 | D | T01, T04, T05, T06, T07, T09, T11 | **Migrate offline consumers end to end.** Update `install_tools`, custom/source-rootfs materialization, Alpine/tool/package adapters and all current callers to explicit inputs, canonical work/cache paths and named published results. Remove basename/output guessing and host-path defines. Exercise real rootfs and small-tool install/reuse/check/remove/cleanup with a second custom readiness model, changed inputs and deleted outputs. |
| T15 | D | T01, T04, T05, T09, T11 | **Finish kernel recipe/caller migration.** Convert source/prebuilt/variant recipes and xtask callers to the single contract. Keep one lifecycle owner, explicit target/Kconfig/options and custom release/module checks. Remove `linux-base.rhai` and old entry APIs when their in-repo callers are updated. Verify with prepared source/prebuilt fixtures and existing artifacts; do not compile kernels merely for this migration. |
| T16 | C | T01 | **Finish cancellation, deadlines and durable diagnostics.** Forward interruption, await descendant teardown and return a distinct cancellation outcome. Separate transport stalls from legitimate silent work using bounded progress/deadline behavior. Retain bounded diagnostic logs and machine results outside invocation scratch. Real silent-long-command, partial-frame, Ctrl-C and coordinator-death cases preserve the last complete checkpoint and never publish failed work. |
| T17 | C | T11, T16 | **Bound retained storage and scratch cleanup.** Implement deliberate retention/pruning under existing locks; protect active leases, current/recoverable generations, diagnostics still in use and published products. Recognize older unleased scratch conservatively without broad directory deletion. Kill/restart and simultaneous-writer scenarios prove safe cleanup; no old-state conversion layer. |
| T18 | B | T01, T03 | **Finish canonical acquisition helpers and remove redundant forms.** One staged, checked HTTP/resume/hash publication path and one pinned Git acquisition owner; retain distinct status/capture/checked command behavior only where it has a real purpose. Update callers with the helper change; remove compatibility aliases and unused acquisition variants, including torrent support if T03's chosen sources do not require it. Compile the updated helpers and verify network-independent argument/path failures here; T22 owns real managed transfer/error acceptance after T13. |
| T19 | D | T01, T04, T18 | **Migrate preseed and network-backed recipes.** Convert Fedora/Alpine source modules, preseed/materializer recipes and remaining acquisition consumers to explicit current inputs/cache/output contracts. Package URL/revision/digest facts stay in Rhai; refresh cannot destroy valid cache on failure. Replace unnecessary torrent callers with pinned supported acquisition, not a torrent compatibility wrapper. Syntax/caller checks here; real mediated acquisition in T22. |
| T20 | D | T01 | **Replace metadata line scanning.** Use T01's loader-owned declared metadata representation for list/info and inherited metadata. No second parser or arbitrary host Rhai evaluation. Actual CLI cases report inherited facts, malformed declarations and unavailable dynamic metadata honestly. |
| T21 | C | T09, T16 | **Connect optional outer autofix/retry.** Keep explicit repair/provider invocation and source patching in the coordinator, outside normal execution. Use structured failures and bounded attempts; protect readiness functions and publication boundaries. A changed recipe gets a new identity rather than stale state restored across a patch. Exercise a controlled failure/patch/retry through the real coordinator; normal execution requires no AI credentials. |
| T22 | B | T05, T11, T12, T13, T14, T18, T19 | **Prove managed acquisition and cache recovery.** Run real HTTP/resume/pinned-Git and selected production-source paths inside Ginux. Cover bad digests, truncated responses, redirects/denials, interrupted transfers, concurrent cache users, offline reuse and acquisition→build→install→run. Include actual extraction failures, especially the RPM pipeline; no pre-Ginux or mocked-helper result counts as managed evidence. |
| T23 | D | T14, T15, T19, T20 | **Finish the repository-wide caller/example cutover.** Reconcile every discovered recipe/module and Rust/CLI caller, including AcornOS/leviso facades, variant build-host recipes, inline e2e scripts, examples and man-page command spellings. Delete obsolete entries rather than forwarding through deprecated shims. C removes any residual central compatibility code and registers changed fixtures. No shipped caller requires the retired semantics, globals, flags or old source-state format. |
| T24 | A | T05, T06, T07, T10, T12, T13 | **Validate combined Ginux bounds and failure handling.** Exercise actual task/file/storage/memory/CPU/network exhaustion and fork/exec/descriptor interactions on the integrated revision. Fix demonstrated gaps in existing owners; do not claim full Linux support. Teardown leaves no tasks/handles, accounting reflects shared storage, and all existing upstream tests/CI coverage remain. |
| T25 | C | T11, T14, T15, T16, T17, T20, T21, T22, T23, T24 | **Run final combined Recipe/consumer acceptance.** Rebuild the worker and pinned compiler image only when their inputs changed. Run the complete applicable managed CLI and builder scenarios at recorded revisions: custom checks, dependency tools, full rootfs publication, install/reuse/remove/cleanup, target/config invalidation, concurrent writers, failures/cancellation/recovery, source immutability and no host bypass. Verify every promised resource/limit outcome; separately report unavailable real-asset or VM evidence. |
| T26 | D | T25 | **Finish usable documentation and explicit wrappers.** Update README, WRITING_RECIPES, helper reference, man pages, interface record and examples to the one supported contract. Document image preparation, ordinary invocations, state/log/retention paths, limitations and recovery; examples run. Add only useful named wrappers around existing producers/scenarios. No compatibility or migration guide for nonexistent external users. |
| T27 | C | T26 | **Assemble local delivery and contribution records.** Commit coherent changes in their owning repositories, then record intended superproject pins without absorbing unrelated edits. Capture revisions, image/input provenance, reproduction commands and the remaining Ginux contribution backlog. Leave a concise working demonstration package. No Ginux PR creation, remote publication or messages to maintainers. |

T13 also requires a short C-owned Recipe network-policy/config join before its
end-to-end behavior is complete; B never edits C's inputs/protocol files.
T11 is a coordinated archive-format cutover: A has exclusive access to its archive
files while C edits Recipe consumers. Neither side independently advertises
compatibility with the old format. T18/T19 similarly land helper/caller changes
as one current interface, not a supported mixed-version product. A task may finish
its source portion early, but its stated managed evidence stays pending.

## Wall-time schedule

Start **T01 (C), T02 (A), T03 (B), T04 (D)** together. Do not make Recipe cleanup
wait for all Ginux work, and do not make filesystem work wait for networking.

| Lane | Preferred ready-work sequence; skip blocked items rather than idle |
| --- | --- |
| A | T02 → T05 → T06 → T10 → T07 → T12 → T13 → T24. Insert T13 earlier when B is ready and the dispatch writer is free. Apply T11 archive handoff immediately after T10. |
| B | T03 → T08 → T18 → T22. While T08 waits for T02, start T18 if T01 is ready; while waiting for T13, prepare real endpoint/source fixtures. |
| C/coordinator | T01 → T09 → T16 → T11 → T17 → T21 → T25 → T27. Start T21 while storage work is pending when T09/T16 are complete. Handle shared-file joins promptly. |
| D | T04 → T20 → whichever of T15/T19/T14 is ready → T23 → T26. Draft documentation alongside each completed interface; final execution evidence waits for T25. |

The likely longest chains are filesystem identity/capacity → archives → rootfs
consumers, and network contract → backend/dispatch → managed acquisition. Actual
wall-time estimates should come from the first implementation runs, not task
counts. There are no artificial whole-wave barriers: start any task whose listed
dependencies and exclusive files are available. Final integrated verification
waits for both chains; focused verification runs with each coherent change.

## Coverage and verification boundaries

- Recipe S0/S2/S3/S4 foundations already demonstrated are inputs to T01/T09/T16,
  not fresh projects to rebuild. T01 removes the remaining dual semantics.
- S1 correctness and S5 acquisition work finish through T14/T15/T18/T22/T25;
  pre-S0 host evidence is retained as history, not accepted as managed execution.
- S6 recipe migration is T04/T14/T15/T19/T23; S7 metadata/docs/removal is
  T20/T23/T26. Ginux networking remains a required local dependency of T22.
- Preserve Ginux's unit tests and run relevant upstream checks. Outside Ginux,
  use genuine CLI/integration/runtime scenarios, not unit tests or mock-only
  substitutes. Compile affected callers before expensive runtime work.
- Validation reuses matching prepared artifacts. Kernel production, full Rocky
  VM coverage, Internet access and real external AI provider calls are separate
  evidence categories; never claim them from fixtures or compilation.
- This planning pass inspects source and validates task dependencies/ownership.
  It does not execute these implementation tasks or rerun their runtime tests.

## Ginux contributions to record, not open

Keep this backlog in this superproject, never as downstream policy in Ginux.
The existing [VFS contribution record](ginux-pr-preparation-20261105.md) covers
PRs #216–#218; #207 remains a separate policy-dependent proposal. Those existing
PRs are not permission to open new ones or change their human-authored text.

| Potential contribution | Local prerequisite/evidence | Future publication condition |
| --- | --- | --- |
| Configured writable filesystem, descriptor ownership and generated execution | Existing local integration plus T05/T06/T07/T12/T24 | Extract coherent behavior and its fixtures on then-current upstream; identify shared prerequisites first. |
| Scalable owned storage and hardlink identity | T06/T10/T11/T24 | Preserve bounded ownership and archive consistency; no copying bytes to pretend hardlink semantics. |
| Private channels, checkpoint/export/restore and teardown | Existing local integration plus T11/T16/T24 | General Ginux interface, without Recipe protocol or policy text. |
| Configured process/thread, mapping and resource integration still absent upstream | T02 reconciliation and T24 | Check what upstream already includes; prepare only the remaining coherent delta. |
| Bounded repository networking and readiness | T03/T08/T13/T22/T24 | Record contract choices and reconcile with maintainer policy before any future proposal. |
| Small standalone defects found during these tasks | A reproduction and focused fix/tests | Record separately; do not accumulate unrelated fixes in a feature proposal. |

For every future candidate record its purpose, exact local commits, current
upstream overlap, prerequisite capabilities and actual validation. Prefer one
standalone candidate after its prerequisites are upstream; do not silently stack
pending PRs. Publishing requires a later explicit user instruction.

## Current checkpoint

Goal active: complete T01–T27 with four rolling lanes. Root implements C;
`lane_a_runtime`, `lane_b_acquisition`, `lane_d_consumers` own A/B/D. No new
branch, Ginux publication or maintainer message is authorized.

- T02 reconciled upstream `2a82f61e` in local `8b5cf5dd`, preserving unit tests;
  configured launch/storage/exec/checkpoint checks and static/unit suites pass.
- T05 `6a45d8f8`, T06 `9e2a39e8`, T10 `38fa14fd`: owned cwd, shared hardlink
  backing and scalable bounded storage pass their focused real-program cases.
  Storage defaults to 16,384 objects with configurable data/metadata budgets;
  an 8,192-object exhaustion fixture proves this replaces the 4,096-slot limit.
- T03/T08: numeric TCP grants, immutable hostname mapping, connection/byte
  budgets, no proxies/UDP/listening; backend commits `8ff83d64`–`9a70427b`.
  T13 central Ginux dispatch `66de2880` passes supervised socket/readiness,
  denial/quota/timeout/fork/revocation cases; real HTTP then exposed missing
  owned-file fsync, fixed in `53dec6b3`. Recipe HTTP/TLS acceptance is running.
  Endpoint grants do not isolate virtual hosts sharing a granted IP/port.
- Recipe `fb3b38e`: sole API2 engine, protocol6, explicit invocation environment,
  no backend/persistence aliases, canonical verified acquisition helpers and
  loader-owned non-evaluating metadata. T20 actual CLI scenarios: 3 pass.
  T01 managed input batch: 33/34 pass; the remaining fixture lacked the strict
  API marker and was corrected to reach its intended filesystem boundary.
  Retained-state batch: 12/12 pass, including changed input identity, separate
  state/files recovery and simultaneous destination users.
- T09 fresh selected-target seeding and T11 Recipe archive joins are in progress.
  Initial target fixture found GNU-header/USTAR mismatch, corrected at the
  writer; a selected-file trailing-slash publication bug is also corrected.
  Ginux T11 archive half `e3374593` preserves aliases across export/restart and
  rejects invalid link targets; it validates an above-2GiB header under explicit
  4GiB capacity, but no actual above-2GiB payload has been demonstrated yet.
- D prepared canonical offline/source-rootfs, kernel and preseed sources/callers
  in distro-builder `3cb94ef`–`45d7f5e`, distro-contract `ceecd63`, AcornOS
  `40dc9a0`/`3e0db2b`, and leviso `b423f9b`–`6d15e89`. Affected Rust callers and
  all retained Rhai sources compile. Current inventory: original35 +2canonical
  tools -4unused entries =33 paths; 11 legacy executable facades still require
  actual caller cutover/removal through T14/T23. Kernel/rootfs/acquisition
  acceptance stays partial. Acorn live-tools still requires explicit musl
  compiler-image support; GNU output is not an acceptable silent substitution.
- T18 network-independent argument acceptance passes five actual domain cases.
  Real Git2.52 image is prepared from a pinned git-core RPM, without changing
  the original compiler image. Exact image SHA256:
  `8bd5ad2e715c163ea201cc629856fb7ee970104f31e7d899913e185e56d3117b`.
  Fixture-only CA image SHA256:
  `fcc70d8c3052ff988a2223839a1050e5305c19ad6344f80cb3bd369c8b3132b8`.
  TLS/Git startup and full positive transfer scenarios are not yet accepted.

Exact next actions: finish real target/storage publication acceptance; A runs
cross-image/owned resolution and actual Cargo locking; B runs managed HTTP/TLS
and pinned Git; D connects offline source closures and removes superseded
callers. C continues cancellation/durable diagnostics, bounded retention and
optional coordinator repair after those joins. Validation uses immutable copied
CLI/worker/supervisor artifacts so concurrent rebuilds cannot corrupt a run.
Evidence logs currently include `/tmp/recipe-go-t01-stable-{inputs,state}.log`,
`/tmp/recipe-t11-check.log`, `/tmp/recipe-t09-ustar.log`,
`/tmp/recipe-lane-b-network/`, and `/tmp/ginux-t11-*.log`. The inputs log contains
one known corrected-fixture failure; it is not an overall green claim.
Active existing planning skill: `hdae-development`; discovery path
`/home/vince/.codex/worktrees/ginux-pr-combination-check/ginux/.agents/skills/hdae-development/SKILL.md`.
Reload after context shifts. The user's direct-work/no-bureaucracy instructions
apply; this queue does not introduce approval gates.

## Earlier implementation evidence

The previous detailed simplification journal is preserved in Git at
`c65c577:docs/recipe-simplification-implementation-plan.md` (read with `git show`).
The [Ginux implementation history](ginux-recipe-implementation-plan.md) also
records the later integrated demonstrations and their artifact/log paths.
Old compatibility requirements, checkbox status and publication permissions are
superseded by this queue. Earlier runtime evidence applies only to the code and
workload it actually exercised.
