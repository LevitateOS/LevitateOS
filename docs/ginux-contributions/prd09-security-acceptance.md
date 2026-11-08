# PRD09 — security acceptance open

Updated 2026-11-08. The current task is to close required security acceptance,
not to declare a handoff complete. **Acceptance is not closed.** No security
probe or adversarial runner has been executed in this general engineering chat.
The separate Daybreak chat's permission is not inherited here. Local source
review, composition, compilation and receipt-file integrity checks below do not
replace authorized security verification.

Two dependencies remain: an actual combined release source and outstanding
criterion-specific authorized results. The user has now resolved the final-path
contract: link publication is the commit point; late reference-close errors
remain supervisor-error and consumers must reject non-success. No new source-delivery direction,
upstream publication or broader native policy is selected by this task.

## Exact source reconciliation

| Source | Identity and security disposition |
| --- | --- |
| Current superproject / Recipe | Root `69adc56`, Recipe `4ee73d7ae7973567e437f40a33c236ab98ebded1`; transport 9 and NEEDS. [PRD18's functional receipts](prd18-implementation.md) do not close PRD09. |
| Checked-in Ginux | `2eac10bf8c9e7a7e72e1ae94990aa2425b2f3ba0`; lacks required configured launch. Not a valid security target for the current Recipe release. |
| Latest NEEDS Ginux donor | `e9b7ce0b40136f4a30892a072e28e65746a37e3b`, binary `bd0c09928c886245fb9a531b4b2d69e39995e804e9f6f232dd25b390ef41d118`. Source review confirms it still omitted the PRD04 corrections listed below. Its ordinary green checks are not final security acceptance. Preserved unchanged. |
| Frozen PRD04 | `5449e2a184bb1147cf618b140a29196b74206b13`, binary `95bc570e92491b1d30c7a14ef66593b0cfa05249b1424ffbd0047e6ea7866eb0`. Authorized items 1–4 and selected 6 pass; item 5 is an exact red control. Preserved unchanged. |
| PRD04 F-01 correction | `2e88c9ad89c487d3fc7c29ba35efba3911587737`, binary `576ee03c8d3a7db19a057306d8223841bc536cbd51b91087fe1e7c2e57f616ed`. Authorized backing-close/affected export controls pass; post-publication link commit point is now explicitly approved. Preserved unchanged. |
| New provisional composition | `/home/vince/levitate-development/ginux-prd09`, clean `fa8be950b4382a8f92c92d90c8df069c36fdb198`, tree `de9b7ad6cf1d40197e2f86597ae44f54daad04a7`, launcher `441de671c8e0f353319e8944fbbdb6817851722ab46acd1b1bd2263739c507d2`. Includes NEEDS diagnostics, authorized PRD08 behavior and all five PRD04 corrections. Built/static-checked; authorized security verification has not run. It is not a selected release source. |
| Typed storage foundation | PRD03 `9802ef036d53ae1ee5276fe59cc719389dda5879` remains separate. The actual PRD05 typed-storage/later-owner integration is absent. No catalog-3 migration, T-01/T-02/T-03 port or final owner replacement is claimed here. |

A fresh read-only GitHub check still finds main
[`53a54addea67969abaa240ea478def1b94130eee`](https://github.com/gmkurtzer/ginux/commit/53a54addea67969abaa240ea478def1b94130eee).
Required #232/#233/#234/#236/#238/#243 remain open, `merged=false`,
`merged_at=null`, at the exact heads recorded in [PRD05](prd05-integration.md).
The owner's replacement foundation has not appeared. The user's upstream-first
instruction and [PRD01 source choice](prd01-runtime-source.md) remain in force.
Do not substitute this local composition for required merges or call it final.

The latest Daybreak-wide record is principally on `020f969b` and `e4a43468`,
with older individual receipts on other revisions. A clean ancestry relationship
or a selected stochastic negative does not transfer every criterion to the new
composition. Use actual owner diffs and criterion-specific evidence.

## Corrected local integration gaps

The nine-file +260/-22 delta from the NEEDS donor is saved as `candidate.patch`
under `.artifacts/ginux-contribution-preparation/prd09/`.

| Existing correction | New local commit | Disposition |
| --- | --- | --- |
| `75d5dcc3` inherited supervisor FDs | `687bc04c` | Retire undeclared inherited descriptors before private reservation, retaining explicit control/result and original public/hard limits. |
| `e91bba31` read/readv semantic policy | `e17a688e` | Classify public reads consistently after console/TCP identity, preserving Linux low-32-bit FD ABI. |
| `eb4a342e` assignment ancestor checks | `9d38279b` | Reject image symlink/non-directory prefixes before restore/init; preserve valid missing/real prefixes and final replacement. |
| `5449e2a1` sticky backing-close errors | `f1d8afee` | Stop before guest resume; propagate cleanup failure through structured completion, without retrying numeric FDs. |
| `2e88c9ad` delayed final export | `fa8be950` | Stage while backing is live; publish only after backing teardown succeeds; retain first-error/cancel behavior. |

The first three patches have equal range-diff entries. The latter two differ
where the NEEDS donor inserts bounded endpoint/contract-denial summaries beside
teardown. Reviewed conflict resolution retains both diagnostics and the original
stage/release/publish error ordering. The complete range-diff and the difference
against the separately verified export source are retained. Native/catalog-1
policy, configured clock implementation and public limits are unchanged.

`make check` and a locked/offline release build pass on the clean composition;
logs are `candidate-check.log` / `candidate-build.log`. These compile/check
commands do not execute its security fixtures. No old security receipt is marked
fresh merely because the correction patches were composed successfully.

## Every material acceptance surface

The eight IDs below cover the current queue's eight security surfaces and every
original G01–G31/F02/F03 handoff family. Existing selected receipts are retained;
none of these rows has final-source closure yet. This is the bounded existing
acceptance set, not permission to run it here or an unlimited new campaign.

| ID / packets | Material criteria to close on the eventual exact source | Existing evidence and remaining gap |
| --- | --- | --- |
| S09-A authority — G01/G03–G05/G09/G12–G16 | Resolved deny precedence/catalog isolation, assigned image/owned/device/proc/native boundaries; Linux FD-width consistency; no private/native FD import; kernel-selected shebang/PT_INTERP/execveat authority; ancestor/final-entry confinement and descriptor/path replacement. | PRD04 items 1/3 establish selected ABI, policy red/green and prefix controls on `5449e2a1`; high register bits alone are not a bypass. Import the real read/readv correction and verify its actual winning rule on the composition. Preserve positive allowed-path controls. No claim all old B1–B3 labels remain defects. |
| S09-B shared VM/tasks — G06/G07/G08/G10–G12 | Stable kernel-consumed path/vector/scratch/TID/signal arguments; shared-table/VM and nonleader/leader birth/death/exec; partial-copy rollback; wait faults consuming a child; SIGCHLD actions/masks; caller-thread signals; reservation cancellation and no deadlock/lost stop. | `qfault`, `x-path`, `X-scratch`, `T-clone-tid-race`, `v-private` have source-bound selected passes. The latest record explicitly lacks an equivalent stable **nonleader peer-death during reserved clone** control. Stochastic negatives do not prove the hold invariant; request source-wide callsite accounting plus synchronized distinct schedules. #234's six cases must survive the join. |
| S09-C descriptors/readiness/network — G04/G05/G08a/G08b/G17b/G28/G29 | Alias/last-close/slot-reuse identity, public/private budgets, CLOEXEC, parked read/write/poll pins and mutable snapshots; full blocking/atomic packet effects, partial copy/retained tails, capacity/peer death/revoke/cancel, SO_ERROR states, byte accounting, endpoint family/scope/mapped forms, MSG_NOSIGNAL and thread SIGPIPE; backend error/short counts and actual close-time ownership. | PRD04 inherited-pressure and selected private/transfer controls pass on frozen source. Packet read/write reuse and capacity kill controls are on `020f969b`; caller-thread SIGPIPE red/green is on `e4a43468`. Retain those exact limits and verify affected joins. Entry-skipped close injection is not a real FD-lifetime proof. General ppoll signal/restart semantics remain outside the accepted PRD08 contract. |
| S09-D file data/namespace/limits — G09/G12–G18/G25 | Malformed/NUL/overflowing indexes and backing failure; growth/copy-up/zero-fill, allocation arithmetic/alignment/initialization and failure cleanup; protected/mutable scalar/vector/chunk copies, prefix/offset/append effects; mmap/mremap/fixed-region permissions and rollback; directory/cwd/link modes, quota rollback/reclaim and shared references. | Historical selected malformed/backing/indirect-copy/namespace controls are preserved. **T-01** public-alias mmap must leave shared offset 3 after native-number reuse. **T-02** must verify all 9,000 copy-up bytes and mid-copy failure after 4,096 with unchanged prior descriptions/data and reclaimed sole slot. Those exact oracles remain missing from current owners; ordinary small-file tests are not replacements. Guest mremap remains denied, not promised implemented. |
| S09-E owned exec/scripts/private transfer — G07/G10–G12/G16/G17 | ELF/shebang/interpreter bounds, argv/env/auxv arithmetic and rollback, exact names/AT_EXECFN; stable indirect arguments, writer exclusion/pins, seal verification/fallback, transient private-FD visibility/collision/retirement, mapping transfer/receipt validation and cleanup at each meaningful allocation/send/receive/map/close/exec failure, interruption and death. | Selected seal rollback, real short write, argument protections, private mapping visibility/death and failed-stage controls exist on historical sources. G10 completed-close/return-override evidence differs from skipped-close evidence; neither proves a real kernel close error. Explicitly resolve guest/private close and reused-number behavior, shared holds and the actual joined owner. An injected old-kernel fallback is not a tested old-kernel platform. |
| S09-F persistence — G19–G22/G31 | Quiescence and pinned publication identity; hostile namespace/archive replacement and same-inode mutation; bounded canonical paths/headers/data/link/alias metadata; short read/write, allocation, sync, close and cancel errors; framing/oversize/replay/partial peer failure, inherited signal mask, task-event readiness, interrupted captures and consistent state/files acknowledgment. | F-01 backing-close absence is verified on `2e88c9ad`, not on this new composition. Its 313-file focused set covers affected selected success/error/mutation controls. Post-publication inode/parent close error reports failure with a complete archive, matching the user's now-approved link commit point. Verify consumer rejection and identity/error ordering on the joined source. Synthetic close delays and one interruption window do not close all persistence races. |
| S09-G locks/time/resources/identity — G02/G23/G24/G26/G27/G30 | Protected/mutable structs and partial copy effects; malformed flags/IDs/times, inode/descriptor lock ownership/release/reuse/exhaustion/deadlock/cancel; truthful per-process VA/CPU limits and fork/thread target selection, bootstrap failures/eligible-CPU changes; strict configured uname and **zero-filled sysinfo** outputs and denial/cross-page/shared-buffer boundaries; virtual ownership with no host mutation and truthful archive precision. | PRD08's accepted contract is settled, not a user hold: global clocks, bounded sleeps, per-process VA (not total RSS), real eligible CPU IDs, numeric TCP endpoints and bounded poll. Source preserves those decisions. Ordinary clock/libvirt checks and selected older query/lock/ownership probes do not establish final security copy/owner/race acceptance. Never change the host clock or claim general Linux ABI/topology/hotplug support from those checks. |
| S09-H Recipe boundary — F03/PRD02/PRD18 | Matching transport-9 bootstrap and anti-spoofing, protected source/inputs, snapshot tampering and archive/link redirects; cancellation, post-worker launcher failure and private scratch/control leases; negative TLS/grants; protected autofix including NEEDS/API/deps/base; static declarations, typed outputs and publication, separate dependency authority, graph-wide ceilings/intersections and no widening; saved denial/execution provenance under bounded observation. | Current Recipe is `4ee73d7`, final CLI/worker SHA `470e3bb2d8b8e0ce990f9d683724649073ada744377198d15d77a08d142c37f6`; image `c089f95c…`. PRD18's 148-name catalog/selected functional replacements are not Daybreak acceptance. R-02 remains a real cross-owner question: Ginux retires undeclared inherited FDs while Recipe supplies scratch leases. The ordinary active-lease/coordinator-death pass is useful, not every teardown/prune interleaving. No private-FD exemption or larger public limit is selected here. |

The detailed historical [handoff](daybreakblue-handoff.md),
[per-source/test inventory](prd06-reconciliation.md) and
[missing-oracle definitions](prd06-test-retention.md) remain the exact inputs.
Old high-bit-as-bypass and invalid 2 MiB stack expectations are superseded by the
PRD04 ABI/valid-config adjudications. Old “user G02 hold” and Claude-exclusive PR
ownership are superseded by current user decisions. Historical results are not
rewritten. General Linux ABI, aggregate RSS/CPU accounting, native policy growth,
external AI-provider interoperability and product boot/install acceptance are
not silently added to this security scope.

## Exact receipts checked without executing probes

- [Original Daybreak six items](/home/vince/levitate-development/ginux-prd04/.artifacts/prd04/daybreak-2026-11-08/receipts.md): **770/770 file hashes verify**, manifest
  `149115adf9d544964e452e1a84d75371da8edfa3283d106056362ae055f02306`.
  Original item 5 remains red; items 1–4 and selected 6 retain their source limits.
- [Focused F-01 retest](/home/vince/levitate-development/ginux-prd04-export/.artifacts/prd04/daybreak-f01-2026-11-08/receipts.md): **313/313 hashes verify**, manifest
  `02a095dea2a5dc4b940579069d0acb1b0d010cbf4645ea404bd9d16f3c8564af`.
  Corrected pre-publication backing failure leaves no archive; late private
  reference-close errors leave an intact archive and report supervisor-error.
- The first original-manifest check used the evidence directory, but that
  manifest's paths are worktree-relative. It failed to locate files; rerunning
  from its worktree verifies all 770. This was a verification-cwd error, not
  missing evidence and not a security result. No probe was rerun.
- Selected historical source invariants still need actual owner comparison and
  a per-criterion returned disposition. No old negative, CI summary, broad test
  count or written request is upgraded to final-source acceptance.

## Outstanding authorized verification and agreed publication boundary

The app exposes thread read/list/wait but **no send-to-chat tool**. Recent and
history thread listings return empty text, so no Daybreak thread can be selected
or dispatched through them. The security cloud tools are a different service,
not an authorization substitute. No cloud scan, agent or external message was
started. No direct app-state or terminal-message workaround was used.

The user explicitly chose **Return Daybreak results** for S09-A–H on 2026-11-08.
Security execution remains unauthorized in this chat. Await the returned exact
receipts; do not infer that choosing this route is itself a verification result. A result must identify
source/tree, launcher/worker/image/config/fixture/runner hashes, exact commands,
exits/results, observations and postconditions; classify each material criterion
as passed, failed or unverified with its actual limit. For races, use a stated
invariant and distinct synchronized controls/source coverage; do not endlessly
repeat equivalent stochastic negatives. Any correction needs affected retests
on its new revision. Verification of this provisional candidate cannot by itself
close acceptance after the future typed-storage/source join.

Existing user decisions already require **supervisor-error** for both
post-publication and cancellation reference-close failures. Do not ask that
again or regress error reporting. The user additionally chose **Keep link as commit point; reject non-success**.
An intact already-linked archive may remain after an inode/parent reference-close
error, while the authoritative result must be supervisor-error. The earlier
reporting decisions also still apply to cancellation. Pre-publication backing,
write/sync/writer-close failures must leave no final archive. Do not delete a
host replacement marker by an unverified pathname. This resolves the contract
choice without widening limits or changing the frozen corrected implementation.

Recipe `launch.rs:390` calls `check_agreement` and `check_launcher_exit` before
private archive preparation, final retention and publication. At lines 1032–1061,
`check_agreement` rejects supervisor-error for either worker success or failure;
launcher non-success is independently rejected. Thus current source already
requires consumer rejection, rather than accepting a leftover archive merely
because its bytes parse. The authorized context still needs exact-source paired
error/publication verification on the new composition; source inspection is not
that runtime receipt.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`;
reload fully with required reference, repository instructions and this record
after context shift. Exact next action: receive Daybreak's S09-A–H results for the exact candidate,
correct any blocking finding and obtain affected retests through that context;
preserve all frozen sources and existing staging. Await actual upstream/source integration before final security closure.
**PRD09 remains unchecked; this record is not completion.**
