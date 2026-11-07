# F03 — fresh Recipe consumer verification (partial)

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
