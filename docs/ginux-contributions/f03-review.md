# F03 — local Recipe consumer verification (partial)

Current local Ginux source is `436c4ae5843e4d50f4833158d351523f35669909`, based on
upstream `59e2f7aa`, at `/home/vince/levitate-development/ginux-recipe-integration`.
Current Recipe checkout is `2ad833cc93f0e209fe205adb4186a822a13caaea`; its last
change updates two CLI fixture files. Production source/release commands remain exactly
`0c4f0d4e9d1f84ab7f638611e895dbf799500e7a`. Exact source, binary and retained-harness
identities are recorded under the Ginux source's `.artifacts/recipe-integration/`.

**Final local ordinary result:100 passed,1 failed,34 unrun from the135-case catalog.**
All100 independent ordinary cases pass with exact one-test/exit0 receipts. The failure is
`/usr/bin/sleep 3` needing the held G02 guest clock capability. The user explicitly retained
that hold; one other sleep-dependent case stays unrun. The remaining33 unrun cases and mixed
library/image targets await monitored review. Clock/security/full-suite acceptance remains
pending; the integration goal is incomplete.

Actual SQLite, pinned Git, RPM, APK and concurrent GNU/musl Cargo consumers all pass.
The final ledger is `final-local-review.json`; source/binary identities are in
`preseed-consumer-provenance.json`, with the six actual consumers in
`umask-real-consumer-receipts.json`. Earlier failures remain in their original receipt files.
Old harness bytes are retained for replay; unchanged case modules were checked against the
final test source, and all affected ordinary download/preseed cases ran on the new harness.

Latest local corrections for inspection:

| Commit | Actual cause and corrected owner |
| --- | --- |
| Ginux `e9a2f8a0` | Accept musl's exact pthread clone flags through the existing thread owner. |
| Ginux `6a19a70a` | Retain blocking pipe write progress across the fixed buffer; actual rpm2cpio no longer truncates at4096 bytes. |
| Ginux `436c4ae5` | Share/copy/preserve process-group creation masks; actual GCC/SQLite publishes mode0755. |
| Recipe `0c4f0d4` | Distinguish the complete generated image PATH from an imported tool's identically named host parent. |
| Recipe `2ad833c` | Follow the full current Alpine asset facts in real CLI fixtures, retaining validation and publication-preservation checks. |

Earlier per-task transactions, shared threads,32-task capacity and direct instance-schema
adaptations remain prerequisites in the local history. The active source includes upstream's
major instance rename; a final read-only check still finds main59e2f7aa and #232–#234 open.
Claude owns extraction and upstream decisions. No pushes, PRs, upstream comments or security
agent dispatch occurred. Human-authored remote text, forbidden branches/worktrees and all
six historical recovery states remain preserved. core.worktree remains in config.worktree.

## Historical preparation and checkpoints

Recipe snapshot `bde746221988094d4a0b80bb46808beab447e3dc` plus exact tracked dirty overlays
`.gitignore` and `tests/e2e/preseed_recipes.rs` is exported to
`/tmp/ginux-contribution-preparation/F03/recipe-source`. Their hashes and the
source/workspace lock provenance are in state.json. Another task owns these
live edits; this task did not alter its checkout. This snapshot is exact evidence,
not a claim that concurrently changing source remains identical afterward.

Ginux is combined `dfebda7c29b628fc41f50635b07e8cb48d6f883a`, binary SHA256
`c01ff13db7b61b47ce959881ae140de2a250bf53f63841a60da1fca1d581d2b0`. Prepared Git/musl image is
`9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4`. The immutable Recipe binary hashes are
`931052585f10422f0200c78f8d542b3850c3383daeb073f6c484ec24380922d8` (CLI/worker) and
`be94a4c334fa1cac64f20d9d2c201378746df639e587ac9ce4dc06af98e247fb` (integration harness).

The initial locked standalone build correctly refused the workspace-shaped lock.
Offline normalization removed99 unrelated workspace package entries and added
no package name/version/source tuples; the e2e build then passes with --locked.
Original checkout/lock remained untouched. This is a fresh build of the snapshot,
not reuse of historical T25/T27 executables.

The current e2e catalog has135 scenarios. First explicitly selected ordinary
managed define/cleanup-reason propagation passes1/1 in44.94s. The next12-case
ordinary batch is **interrupted at the user’s pause**. It covers valid module/dependency/lifecycle,
cleanup/null/bash/owned-ELF/state/storage metadata and actual C compilation;
selection and exact command are saved. No full-suite pass is claimed.

F02 also now proves actual two-process SQLite locks/data integrity in source and
clean replay. Actual Cargo/Rust compiler, APK/RPM package paths, remaining ordinary
Recipe scenarios and every applicable full-suite criterion remain pending.
G07 thread/shared-argument snapshot remains excluded and unready. Security-looking
worker spoofing, archive/path/authority/protected checks, adversarial fault/cancel
and negative TLS checks remain for [DaybreakBlue](daybreakblue-handoff.md),
enabled and locally unrun. No cloud dispatch or security acceptance is implied.

Evidence/helpers are retained under .artifacts/ginux-contribution-preparation/F03;
/tmp holds exact source and binaries. Do not replace those from changing live
checkouts. Exact next action: inspect the active ordinary batch log/state, diagnose
any normal failures within approved capability/policy boundaries, then continue
ordinary consumers. Complete F03/F04 and publication readiness are not established.
No pushes, PRs, branch/ref creation or maintainer messages are authorized.

## Paused checkpoint

The user paused the goal. The active batch and its verified task descendants
were stopped; no tests are left running from that batch. Partial batch results:
7 passed, 4 failed, 1 interrupted/unreported. The separate first probe
passed. No full-suite success is claimed. Dependency compilation and the Bash
case expose command-output reader thread creation returning ENOSYS: the ordinary
per-process union excludes held G07. Other failure details remain in the log for
review on explicit resume. Do not replace thread support with a workaround or
claim complete Recipe acceptance.

On explicit resume: inspect state.json/ordinary-batch.log and the interrupted
case, then continue only authorized ordinary work. The G07 shared-path security
correction/verification remains assigned to the written DaybreakBlue handoff.

## Destination continuation — 2026-11-06

The user explicitly resumed preparation through F04, with publication excluded.
The paused results above remain historical and unchanged. Destination paths and
binary provenance come from
`.artifacts/development-migration/2026-11-06/destination-report.json`.
Use its `f03-destination-bin/e2e`, not the original harness with embedded old paths.
Future snapshot builds require a separate `CARGO_TARGET_DIR`.

Exact-source verification now passes all 37 patches: each retained patch hash
matches its manifest and applying it to its exact parent through a disposable
index reconstructs its recorded tree. This accommodates the original G05a mail
patch and later plain diffs without changing either. G12 uses its corrected
manifest/head. All 189 F02 source paths match combined commit
`dfebda7c29b628fc41f50635b07e8cb48d6f883a`. All 153 current tracked Recipe paths
other than the independently normalized standalone lock match the snapshot;
current Recipe HEAD is `ce3cae5`. This establishes source identity, not runtime,
current-upstream independence, security or CI passage.

The complete paused log shows the same reader-thread startup ENOSYS in **all
four** failed scenarios: dependency C compilation, Bash child output, owned ELF
execution and storage metadata via touch. Recipe's existing owner is
`src/process.rs:270`, which starts output-reader threads. The combined configured
launch owner, `crates/ginux-init/src/launch_tasks.rs:845`, supports only the three
listed process clone flag shapes and returns -38 for a thread clone. The child
is spawned before reader startup, so these failures do not establish that the
child performed no work; they prevent acceptance of the complete scenarios.
This is a missing G07 capability; changing the command, dropping output capture or importing
the unready G07 snapshot would not establish the required consumer acceptance.
No such workaround or thread import has been made. G07 correction and security
verification remain with the written DaybreakBlue handoff, without dispatch.

New evidence is separate from the preserved recovery logs:
`.artifacts/ginux-contribution-preparation/F03/resume-2026-11-06/` contains
`verify-integrity.py`, `integrity.json`, and fingerprints of all six restored
worktree indexes/statuses. The previously interrupted ordinary state-restore
scenario now **passes 1/1 in 328.38 seconds** on the destination harness, using
the documented privileged coordinator. It covers initial alpha installation,
restored alpha reuse, an independent readiness query and a changed beta define,
checking nested state, product bytes, saved generations and unchanged sources.
This replaces only that case's interrupted status; the four thread-related
failures and all other unperformed criteria remain open.

Thirteen destination scenarios have completed: **ten pass, three fail**, with two
additional cases interrupted and unverified. The
passing cases are state restore, independent custom readiness, 128 KiB file
publication, nested-directory copy/move/cleanup, lifecycle machine-event order,
current API initialization without legacy globals, retained dependency files and
reused-check state over three invocations, changed source/input identity, default
application-state placement/reuse, and publication of 5,000
entries with unchanged selected inputs. Each was selected exactly and run
serially through the actual managed CLI. The retained logs contain complete
harness summaries; no interrupted or unrun case is included in the pass count.

The Bash child case was repeated once on the destination to check whether the
saved failure persisted after migration. It fails again in 86.45 seconds with
`command output reader failed to start` / ENOSYS. The harness exits 101; the
four-case runner correctly exits nonzero. This is fresh failure evidence, not a
passing destination smoke. The other three original failures remain historical
failures pending the same G07 prerequisite; they have not been rerun unchanged.

The initial 23-case ordinary queue exposed a second G07 consumer: both valid tar
extraction cases fail with a worker panic, `failed to spawn thread` / ENOSYS.
The logs show clone3 denied and clone rejected by the configured process owner.
Source tracing identifies `src/helpers/internal/progress.rs:20`:
`enable_steady_tick` starts a progress thread, reached by extraction at
`src/helpers/build/extract.rs:425`. These are valid archive failures, not hostile
archive probes. They do not establish whether archive parsing/link handling
would otherwise pass. Git/download helpers also use this progress owner; their
acceptance remains pending rather than assuming they are thread-independent.

The batch was stopped after this dependency was established. Only its identified
runner and owned descendants were signaled; `batch-stop.json` records cleanup.
The in-flight valid zip case is **interrupted**, with no case exit code or pass
claim. Both tar failures remain failed. No progress suppression, replacement
implementation or unready G07 import was used to bypass this missing capability.

The revised 20-case ordinary queue ran serially from
`ordinary-independent-selection.json`. Its first three cases passed: retained
dependency state, changed source/input identity and default application state.
It was then stopped for the current-main prerequisite refresh below. The in-flight
retained-links case is interrupted/unverified; unstarted cases remain not run.
It covers retained state and dependencies, selected targets, lifecycle options,
cleanup and a valid large-file digest. Original selections and receipts are
retained; `independent-ordinary-receipts.json` records the new commands/results.
`state.json` reconciles fresh and historical statuses against all 135 exact
catalog names. Multi-invocation cases have correspondingly longer time limits.
This is selected ordinary verification, not complete applicable-suite acceptance.

All six restored worktrees retain identical HEAD, raw-index and staged/unstaged/
conflict-status fingerprints in the before/after comparison. Concurrent edits
elsewhere in the main checkout are preserved. No Ginux candidate source, branch,
remote ref, upstream test or CI coverage has changed during this continuation.
The continuation's source comparison also confirms all 153 recorded Recipe
file hashes still match in both the retained snapshot and current checkout;
`source-continuation-comparison.json` records no differences.

### New upstream prerequisite — 2026-11-06

A read-only upstream check now pins main
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`, 52 commits beyond the recorded
`3f2fe7df` base. The merged #228/#229 work substantially changes native process,
filesystem, identity, network and typed domain configuration owners. Configured
`launch_validate` still rejects storage/device assignments, so native systemd
support is not proof that the prepared owned-storage/Recipe path is replaced.
The current network enum also includes `Shared`; future joins must preserve it
and the upstream contract fields rather than copying an old whole module.

The six recorded-main-based patches (G05a/G03a/G06a/G26a/G27/G28) apply cleanly
to that exact new head through a disposable index. This is context evidence
only: no new candidate head, compilation, runtime or security passage is implied.
The exact new source export and preview are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/`.
The commit was fetched by exact SHA without updating refs or FETCH_HEAD.
No remote text or history was changed.

Further old-composition checks cannot establish final-source closure after
these owner changes. Only the identified old runner and its descendants were
stopped; `upstream-refresh-stop.json` preserves cleanup provenance. All ten
passing and three failing receipts remain evidence for the saved F02/F03
revisions, and both interrupted cases remain unverified. This does not remove
any required case or replace the final applicable suite with a smaller suite.

G05a's current-main source refresh is now retained at
`936d0eb7a9b46e58ade5219705c12d3d476dfae9`; its
[review](g05a-review.md) owns the new regression, full-unit/static/build and
selected ordinary source/clean evidence, with default runtime/macOS checks pending.
G03a's independent configured-workdir correction is now refreshed at
`c28dd1321f024ec6b5d2087fd489457f31f20e2c`; its
[review](g03a-review.md) records the exact-base failure, successful source/clean
regression and pending checks. G06a is now refreshed at
`01f22e0615e54fdcf3b85f8cc3b3542fb88b1a27`; its
[review](g06a-review.md) records preserved new job-control coverage and remaining
checks. Exact next action: prepare G03b on refreshed G05a, retaining its upstream
merge prerequisite for standalone delivery, then continue the other owners.
Then reconcile remaining owners before rebuilding the final F02 composition and
rerunning applicable F03 checks. Keep G07, monitored security,
actual compiler/Cargo/APK/RPM, full/default platform CI and current-main
independence requirements open. F04 remains dependent on required F02/F03 work;
the goal is active, incomplete, and not publication-authorized.


## Local consumer ownership resumed — 2026-11-07

The authoritative ownership split ends Codex's upstream packet preparation and
publication-review work. Claude owns that work. Codex resumes local integration
and Recipe consumers on `/home/vince/levitate-development/ginux-recipe-integration`,
initially preserved F02dfebda7c, now local commit
`91e244c175ad73aaef2d5e574045282fe770df98`. It separates pending descriptor results,
restored arguments and poll snapshots/deadlines into each existing Task before
shared-table thread support. Static/build checks,102 existing routine units and
selected ordinary pipe/vector, readiness/deadline and task/exec/lock checks pass.
Three existing policy/security unit tests remain enabled/unrun. No thread
capability or full CI/security passage is claimed from this prerequisite.

The previously interrupted
`managed_state::managed_state_restores_and_publishes_links_without_rebuilding`
now passes1/1 in166.17s using this exact local binary
(`3cdc1e1c1fcdaa96961b73835dc2016515e0c68bb6eaad8fffa3e123eb1e6ac2`), saved
Recipe source/destination commands/harness and the same verified prepared image.
Both managed invocations check retained state, distinct publication paths, actual
link targets and bytes without rebuilding. Receipt/logs are in the integration
source's `.artifacts/recipe-integration/recipe-retained-links.{json,log}`. Earlier
interrupted evidence is preserved. This is saved-snapshot consumer evidence;
current final Recipe source/binaries and all applicable cases remain pending.

Next is shared descriptor-group/executable/cwd lifetime and supported thread
creation, then the actual failed output-reader/progress consumers. Do not disable
capture/progress, redesign known B1–B3 storage behavior speculatively, count unrun
checks as passes, import the unready snapshot or dispatch a security agent.
Known security checks remain downstream and pending. Wave1 #232–#234 remain open;
rebase local integration when upstream merges. No fork push or upstream message
is authorized. Old packet-refresh/standalone-publication next actions above are
historical and superseded by the ownership split.


## Local configured threads — 2026-11-07

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

Final source/provenance/receipt review is in the current F03 summary above
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


### Current Recipe 312d9d1 and Ginux 8a7eb37c selected checks — 2026-11-07

Recipe `312d9d1e9e06a66fc0623e08aae9d41ee181655b` and clean local Ginux `8a7eb37cc0e6fa7be8f5c16e6fa14b4da7fac0e6` are newer than the prior F03 final ledger. An isolated `CARGO_TARGET_DIR=/tmp/recipe-current-312-target` `cargo build --release --locked --bins` and `cargo test --locked --test e2e --no-run` passed. The release Recipe, e2e harness and Ginux binary SHA-256 values are `f4592b119b56603a4a4f019176ec8b5aa645c5f71212d6c7740ab77076937ae1`, `d72777d4632300c3a98a75ee2a27d9b31a87230a5f0d1a0370340c1aec23fafc` and `a21a8a6b21904c8fc820159946e849f7a2fd662366e05682bd7ba89832a19219`. The newly compiled e2e catalog has 136 names. Each exact selection below exited 0 with one passed and 135 filtered out.

The current managed target-metadata publication/reimport/EROFS case passed in 23.17 seconds, and managed verified-download/digest publication passed in 31.55 seconds under the actual configured Ginux launch with the prepared image. Six previously monitored cases also passed separately: public worker environment spoofing, private worker missing-bootstrap rejection, owned nonexecutable file rejection (7.43 seconds under configured launch), metadata query without setup execution, missing/mismatched root image rejection, and root image traversal/symlink-ancestor host-file preservation. The four host-side checks finished in 0.00–0.07 seconds. These are selected fixture assertions on current sources; they do not promote the historical 100/135 result to a current full-suite pass. Twenty-seven of the earlier 33 monitored e2e cases remain unrun on this source. G02 clock policy stays held by user decision, its one failed and one unrun dependent case stay pending, and G03c/G12 descriptor authority stays failed pending Claude's storage redesign. Exact next action: run the next independent monitored Recipe case, `archives::cli_zip_rejects_existing_symlink_components`, then continue the remaining independent cases one at a time and record failures without relaxing checks. No Ginux source change, upstream push, PR, comment or packet work occurred.


### Current Recipe archive, argument, dependency and download boundaries — 2026-11-07

On unchanged Recipe `312d9d1`/Ginux `8a7eb37c` and the pinned binaries in the preceding checkpoint, seven more exact 136-case-catalog selections passed 1/1 each with exit 0: ZIP existing symlink components (four variants, 29.76 seconds), ZIP unsafe member rejection (7.45 seconds), checked argv/cwd/env/control input (7.53 seconds), dependency cycle/missing/invalid declarations (7.47 seconds), invalid download arguments (22.30 seconds), directory download destinations (7.43 seconds), and redirect destination grants plus byte budget (23.89 seconds). The dependency case first failed in the restricted sandbox with `launch ptrace seize failed`, before reaching its intended assertion; the unchanged exact selection then passed under the privileged managed-launch environment. That sandbox attempt is a failure, not a pass. No validation was weakened and no source changed.

Together with the six earlier selected checks, 13 of the old 33 monitored Recipe cases now have current-source passing results; 20 remain unrun. The added target-metadata case and selected ordinary download also passed, but historical 100/135 ordinary results are still tied to their older source/binaries. G02 guest clock policy remains held; its failed sleep case and unrun dependent case remain pending. G03c/G12 descriptor authority remains failed pending Claude's storage redesign. Exact next action: continue the next independent monitored Recipe case (`git_acquisition::managed_pinned_git_selects_exact_commits_and_preserves_invalid_checkouts`) on the same current binaries, inspect any failure before changing code, and retain all interrupted/unrun statuses precisely. No upstream action occurred.


### Current Recipe monitored-case continuation — 2026-11-07

On unchanged Recipe `312d9d1`/Ginux `8a7eb37c` and the previously recorded binaries, 16 further exact e2e selections passed 1/1 with exit 0: pinned Git checkout with dirty/origin/invalid/config variants; prebuilt-kernel archive validation; kernel cleanup path confinement; protected autofix rejection; delayed launcher finalization; failed launcher after worker success; image reserved/duplicate-path rejection; invalid-worker cleanup; partial launcher-frame timeout/reaping; declared source-tree confinement; absolute host-path write denial; staged-recipe replacement denial; malformed export traversal/link rejection (seven variants); invalid target/force options before setup; modified snapshot rejection; and quoted Alpine/RPM preseed paths. The prebuilt-kernel, Git, archive and other fixtures used disposable inputs and retained their asserted host sentinels. No source changed or check was relaxed.

This brings the earlier 33 monitored Recipe e2e names to **29 current-source selected passes and four unrun G02-dependent cases**. Source inspection shows guest `/usr/bin/sleep` in `managed_deadline_stops_silent_work_and_retains_failure_record`, `managed_interrupt_reaps_descendants_keeps_checkpoint_and_reports_cancelled`, `managed_coordinator_death_terminates_launcher_and_worker`, and `managed_state_recovers_before_and_after_checkpoint_acknowledgement`; the user-directed clock hold keeps them pending. The separate ordinary sleep/deadline failure and shared-destination sleep case are likewise not accepted. The historical 100/135 ordinary ledger belongs to older revisions; a selected current managed download and the newly added target-metadata case passed, but the current 136-case complete suite has not run. G03c/G12 descriptor authority remains failed pending Claude. Exact next action: rerun the historical ordinary selections serially on current Recipe/Ginux revisions, stopping and diagnosing the first new failure; keep the G02 cases excluded and unverified. No upstream action occurred.


### Current Recipe ordinary serial replay, first 20 — 2026-11-07

The first 20 of the historical 100 independently passing ordinary Recipe e2e selections were rerun serially on clean Recipe `312d9d1`/Ginux `8a7eb37c`, the new 136-case harness and the pinned binaries above. Each exact selection exited 0 and printed one passed/135 filtered out. The first ten covered API readiness/failure guards, valid tar/ZIP and cleanup paths; the next ten covered CLI lifecycle, help/version, inherited recipes, temp-root resolution and checked command diagnostics. The runner stops on the first nonpass, pins both source HEADs and all three binary hashes, and saves exact command, exit, elapsed time and full output for each selection. Ignored `current-312-ordinary/` under the integration source's `.artifacts/recipe-integration/` retains `000`–`019` logs/JSON and runner SHA-256 `cc822af23cabd6d11f9632504bc38c16dcf696b92cc0b5708f1d15f241038ae5`; all 20 receipts say passed. This is current-source evidence for those 20 only. The other 80 historical ordinary selections have not yet rerun on these revisions; one selected download and the new target-metadata case passed separately. Twenty-nine formerly monitored cases have current selected passes; four sleep-dependent cases remain unrun under the held G02 decision. G03c/G12 authority remains failed pending Claude. Exact next action: continue the serial ordinary replay at indices 20–29 with the same pinned runner, preserving any new failure before diagnosis. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 30 — 2026-11-07

The next ten historical ordinary selections, indices `020`–`029`, passed exactly 1/1 on unchanged Recipe `312d9d1`/Ginux `8a7eb37c` and the pinned current binaries. They cover checked command options, dependency error ordering and actual transitive C compilation, concurrent/offline download cache, interrupted download preservation, range-resume/digest validation, verified download publication, and failed-build recovery/cleanup. The range-resume case took 78.97 seconds and exited 0; it was not inferred from progress. The verified-download case had already passed as a selected check, then passed again in 34.86 seconds to retain an exact per-case log in the serial ledger. Ignored `current-312-ordinary/` now holds `000`–`029` JSON receipts and complete logs; all 30 recorded cases passed with pinned source and binary identities. Seventy of the historical ordinary selections remain unrun on current sources. The 29 monitored current passes, four G02-dependent unrun cases, separate ordinary clock failure/hold, and failed G03c/G12 authority remain unchanged. Exact next action: run indices `030`–`039` serially with the same pinned runner and stop on any failure. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 40 — 2026-11-07

Indices `030`–`039` of the historical ordinary selections all passed 1/1 on unchanged Recipe `312d9d1`/Ginux `8a7eb37c`. The ten cases cover lifecycle event/cleanup ordering, failed and unpinned Git acquisition, API2 context and runtime rejection, current contract initialization, kernel readiness/removal, and the disposable autofix retry. Kernel readiness took 136.8 seconds and returned exit 0 with its exact one-test summary. Ignored `current-312-ordinary/` now retains `000`–`039` per-case receipts and full logs; all 40 receipts are passed and source/binary pinned. Sixty historical ordinary selections remain unrun on current sources. The 29 monitored selected passes, four G02-dependent monitored holds, separate ordinary G02 criteria and failed G03c/G12 authority are unchanged. Exact next action: run ordinary indices `040`–`049` serially and inspect any first failure before changing source. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 50 — 2026-11-07

Indices `040`–`049` passed 1/1 each on unchanged Recipe `312d9d1`/Ginux `8a7eb37c`. They cover autofix-off provider isolation, managed Bash/output/error capture, staged import, cleanup and checkpoint context, independent custom checks, and diagnostic-frame budget behavior. Ignored `current-312-ordinary/` contains `000`–`049` exact JSON receipts and full logs, with 50 passed and zero failed in this current serial replay. Fifty historical ordinary selections remain unrun on current sources. The 29 selected monitored passes, four held G02-dependent monitored cases, separate ordinary clock criteria, and failed G03c/G12 authority remain pending as recorded; no whole-suite pass is claimed. Exact next action: run indices `050`–`059` serially with the pinned runner, stopping at a first failure. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 60 — 2026-11-07

Indices `050`–`059` passed 1/1 each on unchanged Recipe `312d9d1`/Ginux `8a7eb37c`: diagnostic draining, directory lifecycle, owned/image interpreter and shebang chains, assigned null device, nested modules, invocation defines/cleanup reason, streamed/oversized asset handling, large-file content checks and transformed-product cleanup. Ignored `current-312-ordinary/` now retains `000`–`059` exact receipts/full logs; all 60 are passed. Forty historical ordinary selections remain unrun on current sources. The 29 current monitored passes, four guest-sleep-dependent unrun cases, separate ordinary clock criteria and failed G03c/G12 authority remain pending; no complete suite pass is claimed. Exact next action: run indices `060`–`069` serially and diagnose any first failure. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 70 — 2026-11-07

Indices `060`–`069` passed 1/1 each on unchanged Recipe `312d9d1`/Ginux `8a7eb37c`. They cover real script-error propagation, owned ELF and publication, unlinked-script fork reopening, retained-history bounds, forced phases, default and disabled state, changed source/input invalidation, and nonserializable field diagnostics. Ignored `current-312-ordinary/` now retains `000`–`069` exact receipts and full logs; 70 are passed and zero failed in the current serial replay. Thirty historical ordinary selections remain unrun on these revisions. The 29 current monitored passes, four held guest-sleep cases, separate ordinary G02 hold and failed G03c/G12 authority remain pending; no whole-suite claim. Exact next action: run indices `070`–`079` serially and inspect the first failure if one occurs. No source or upstream change occurred.


### Current Recipe ordinary serial replay, first 80 — 2026-11-07

Indices `070`–`079` passed 1/1 each on unchanged Recipe `312d9d1`/Ginux `8a7eb37c`. They cover retained links/dependencies/nested state, storage modes and restart identity, a published payload above 2 GiB under explicit bounds, restore/publication of more than 4,096 entries, and target input/readiness/product rejection. The 2 GiB and entry-count cases finished in 20.65 and 45.51 seconds with exact one-test exit-0 summaries. Ignored `current-312-ordinary/` now retains `000`–`079` receipts and full logs; all 80 are passed. Twenty historical ordinary selections remain unrun on current sources. The 29 current monitored passes, four G02-dependent unrun cases, separate ordinary G02 hold and failed G03c/G12 authority remain pending; no whole-suite claim. Exact next action: run indices `080`–`089` serially, stopping on the first failure. No source or upstream change occurred.
