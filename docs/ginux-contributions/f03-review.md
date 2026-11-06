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

Ten destination scenarios have completed: **seven pass, three fail**, with one
additional case interrupted and unverified. The
passing cases are state restore, independent custom readiness, 128 KiB file
publication, nested-directory copy/move/cleanup, lifecycle machine-event order,
current API initialization without legacy globals, and publication of 5,000
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

The revised 20-case ordinary queue is now running serially from
`ordinary-independent-selection.json`, beginning with retained dependency state.
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

Exact next action: monitor the active serial ordinary runner and inspect each
failure before deciding its fix or dependency. Keep G07, monitored security,
actual compiler/Cargo/APK/RPM, full/default platform CI and current-main
independence requirements open. F04 remains dependent on required F02/F03 work;
the goal is active, incomplete, and not publication-authorized.
