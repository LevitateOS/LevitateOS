# Ginux execution for Recipe: implementation and upstream contribution plan

Status: implementation is in progress; individual tasks record completion and evidence.

PR preparation checkpoint (2026-11-05): the user requested prospective upstream
contributions. Three independent VFS bug fixes are prepared locally on upstream
`2489517e`, each in a detached worktree with one signed-off commit, a short PR
draft and runtime/unit evidence. All eight new regressions fail on unmodified
upstream; the reviewed combined fixes pass 63 unit cases and static checks.
Each candidate also passes the CI build and test commands locally on Linux.
Review corrected incomplete copy-up after read errors and image whiteout reuse
through symlinked directories; GitHub Actions has not run for these local commits.
See [the contribution package](ginux-pr-preparation-20261105.md) for exact commits,
paths and validation. User review is next; no push, new PR or public text edit
was performed. Hold further balanced-profile proposals pending the current
maintainer guidance on issue #179 and PR #207. The integration work and its
remaining limitations below are preserved.

Current execution checkpoint (2026-11-05, supersedes older next-action text):
the user authorized this review thread to complete the local integration, using
existing branches and keeping publication under their control. Work order is
invocation/options and complete inputs, prepared compiler environment, real
recfstab build/run/export, retained state and recovery, then representative
consumers. Preserve mandatory Ginux, custom readiness, and upstream unit tests.
Older task evidence below is historical, not proof that a capability is wired
into the current configured launcher; any older instruction to remove Ginux
tests is superseded by the repository's explicit exception.

Active Ginux branch `codex/ginux-recipe-local-integration` contains upstream
`5ed26bf2` through local merge `28f0a7d6`. Recipe is on
`ginux-recipe/r06-launch`, starting at `650f7a3`. The paused interpreter edits
were preserved in a Git stash named `wip: preserve unfinished owned interpreter
chains` before continuing from the verified merge. The stash remains preserved;
its intended interpreter-chain capability is now implemented and verified below.
Baseline: 29 focused Recipe integration cases passed against that merge; the
broad review reproduced dropped defines, nested imports, and missing dependency
inputs. Review/evidence: `.artifacts/ginux-local-integration/broad-review-20261105/`.
Local implementation progress: Recipe `f8766b8` transports invocation defines
and cleanup reasons (protocol v3); `7bb751d` preserves nested modules and stages
literal dependency closures; `b9c0bcb` exposes explicit read-only source-tree
inputs; `e776d22` permits nested paths up to the actual launcher path bound.
The corresponding real CLI regressions pass. Ginux `8752f708` accepts O_NOCTTY
on owned regular files and passes the configured storage suite. A real C source
is now compiled, linked, and executed successfully inside the prepared domain.
The toolchain image is `.artifacts/ginux-env/build-rootfs.tar`, digest
`2ec000344178ac27c1c6d28feb23bc91959723893fdb4421bfe90296d462e909`;
its input provenance is beside it in `build-rootfs.json`. It is an experimental
local assembled image, not a published producer. The copied recfstab source and
vendored offline dependency lock are under `.artifacts/ginux-env/recfstab-source`;
Cargo refreshed that isolated lock from locally cached compatible versions.
Ginux `d9490772` adds image cwd changes and resolves relative paths from the
calling process; configured task/exec/storage suites pass. Owned-directory chdir
remains explicitly unsupported. `3b27b370` adds owned local sequenced-packet pairs
for Rust's fork/exec error channel; `e954010d` raises the bounded channel pool to
16 buffers after a real nested linker launch exhausted eight. Channel and task
suites pass. `32ba41e9` reports `/proc/self/exe` and readlinkat in the domain
namespace, including pinned deleted executables; native and owned exec suites
pass. The prepared Rust toolchain currently needs child LD_LIBRARY_PATH=/usr/lib;
Rust -vV runs and Cargo reaches code generation and linker invocation.
Recipe `db20b58` adds complete-frame read/write deadlines, bounded stderr shutdown,
launcher process-group cleanup, and Linux parent-death cleanup. The real partial
frame CLI fault scenario and 24 managed regression cases pass. `ff880f0` adds a
passing actual coordinator-death scenario that observes launcher/worker termination.
These checks do not prove resumable cancellation.
Ginux `37ca30cb` corrects owned readlink and stat flags. Cargo now compiles
proc-macro2's build script and advances to unicode-ident; Rust then fails with
"failed to create encoded metadata from file: Operation not supported". The
missing capability is private mapping of owned files. Temporary dependency-info
diagnostics were removed exactly before this commit; no diagnostic source edit
remains. The probe's ls output is now useful and does not replace Cargo's error.
Ginux `32e2c85f` replaces anonymous overlay allocations with supervisor-owned
memfd backing; storage quota/truncate, owned exec and task suites pass. `868862bc`
adds coherent private and readonly-shared mappings through a bounded temporary
socket/SCM_RIGHTS transaction. It rejects shared writable mappings and protection
upgrades, guards fixed mappings against overwriting scratch/code, excludes all
pre-existing native fds from scratch-derived selections, serializes descriptor
changes, restores stack bytes and closes every private fd. A real host pidfd_getfd
probe succeeds. The new configured storage fixture passes COW, cross-process
write visibility, truncate/SIGBUS, regrowth, unlink lifetime, invalid flags and
fixed-stack overlap checks; its runner confirms only native fds 0/1/2 remain in
the workload. Native/owned exec, threads and channel suites also pass. Evidence:
`/tmp/ginux-map-test.log` and `/tmp/ginux-map-regression.log`. The current binary
was built before the final audit-label/comment-only change in 868862bc; rebuild
before final exact-commit validation.
Ginux `680d5081` fixes private mapping by nonleader Rust threads, including after
leader exit; configured thread/storage cases pass. `adc79f0a` mediates owned
access/faccessat probes; storage and real cc/lld version-script cases pass.
The actual recfstab build now succeeds and its binary runs inside Ginux. LLD
exceeded the bounded 16-task domain during final linking; the explicit local
probe now supplies `RUSTFLAGS=-C link-arg=-Wl,--threads=1`, in addition to
CARGO_BUILD_JOBS=1. No task limit was weakened. Cargo still reports a nonfatal
SQLite global-cache IO error, requiring follow-up. One earlier retry hit
"launch requires the x86-64 syscall ABI" during compiler thread exit; the actual thread-group-exit regression reproduced it. Diagnostic evidence
confirmed GET_SYSCALL_INFO returned ESRCH after GETREGS as a peer died. The local
fix waits for actual wait4 death events on ESRCH from either syscall-stop read;
30 exit_group and 100 SIGKILL cases pass. Related execution checks are running
before this fix is committed.
Ginux `48b26f8d` adds optional --export-file: exclusive caller-selected archive,
written only after successful root completion and complete task teardown; regular
owned files/directories, modes, long names, no deleted entries or image files.
Configured storage/export, launch and thread cases pass. Static/unit validation
then found two lint issues in earlier changes; those corrections are committed as `c541a67f` and `make check test-unit` passes
(all 59 existing cases).
Recipe `11057ca` validates and publishes only named /work or /out file/directory
outputs under coordinator-owned `.artifacts/ginux-export/<invocation>/<name>`.
Returned outputs are host paths. Fresh staging, exclusive final rename, bounds,
missing-output rejection, and rejection of archive links/traversal/oversized long
names are covered by real CLI fault scenarios. 27 managed regressions passed;
the two publication scenarios passed again after bounded raw tar-header parsing
and long-filename coverage. No public text or remote branch was changed.
The full Recipe recfstab build/run/publication passed. Its published executable:
`.artifacts/ginux-export/recfstab-1793908382636-1646239/program` (1,178,736 bytes).
Evidence: `/tmp/recipe-recfstab-published.{stdout,stderr}`,
`/tmp/recipe-publication-test.log`, `/tmp/recipe-publication-boundary-test.log`,
`/tmp/ginux-export-test.log`, `/tmp/ginux-integration-static-unit.log`.
Ginux `2b306eff` adds the optional private inherited Unix control socket. Bounded
GXS1 requests capture a new exclusively created filesystem tar; GXA1 replies
follow file fsync/close. Mid-run capture reuses the final-export owner. The real
fixture proves two distinct states, prior-snapshot immutability, no guest control
fd, partial-frame buffering, duplicate-sequence rejection, and preservation of
an occupied destination. Checkpoints, storage/export, launch and thread suites
and static checks pass. Evidence: /tmp/ginux-control-{test,regression,static}.log.
Startup restore is implemented and verified: --restore-file loads only bounded
regular files/directories into assigned owned storage before the workload starts.
The real checkpoint fixture restarts from each captured generation and verifies
bytes, modes, empty directories, long names and immutable source snapshots.
Traversal, links, duplicate entries, checksums, truncation and assignment mismatch
fail before workload execution. Checkpoint, storage, launch, thread and static
checks pass; evidence /tmp/ginux-restore-{test,regression,static}.log.
Ginux `9ec95527` supports read-only realtime/monotonic clocks and interruptible
relative/absolute sleeps, rejecting dynamic FD and host CPU-clock IDs. Real task
checks and all existing static/unit checks pass. This fixed the actual sleep
failure exposed by the interruption scenarios; evidence /tmp/ginux-time-*.log.
Recipe `af4e092` wires durable state/input identity, explicit --state-dir and per-user
state storage, shared destination locking, per-recipe v4 checkpoints and recovery.
Only ctx.state is restored into freshly initialized API 2 contexts; sources stay
immutable. Disabled persistence (including the old alias) retains no state.
Successful reuse checks save dependency state, and direct-check results receive a
final state/files capture. Corrupt snapshots fail before guest startup. Previous
complete generations remain available. Field-specific errors reject unsupported
state values rather than stringifying them.
Nine actual managed state scenarios passed (eight in
/tmp/recipe-retained-complete.log, the corrected dependency fixture separately in
/tmp/recipe-retained-dependencies.log). They cover source/input/define identity,
nested values, dependency reuse, application-state defaults outside a checkout,
legacy guidance, disabled persistence, corrupt bytes, shared writers and killing
the coordinator before/after acknowledgement. All 27 existing managed scenarios
and 3 worker-entry boundary checks pass: /tmp/recipe-retained-{regression,entry}.log.
The first dependency fixtures failed correctly because their readiness checks
remained not-ready after completion; the final fixture uses fresh ephemeral
context to exercise reuse while truthfully reporting completion within a run.
Snapshot reads serialize with all owned filesystem mutations; shared writable
mappings remain prohibited. This is filesystem consistency, not process-memory
checkpointing. Ginux result generation stays null; Recipe owns durable generations.
Recipe `c795d5c` removes the unreachable host callbacks, host Rhai engine accessor, old
source rewriting/parser/serializer and source-file lock implementation. All nine
managed recovery cases, the real public-library boundary, three worker-entry and
three cleanup/remove scenarios pass; /tmp/recipe-host-removal-*.log. Manual checks
now expose the actual persistence setting in ctx.runtime.options.
Recipe `3177f69` implements and verifies explicit execution options: protocol v5 carries
native target OS/arch, 1–16 jobs (default one) and main-recipe force flags. Target
and jobs participate in state identity; force does not. CLI/library validation
rejects unsupported targets and invalid policy before setup. Domain config,
worker environment and Rhai runtime agree. Dependencies keep their own readiness
without inheriting parent force flags; post-phase validation remains enforced.
All 43 filtered managed scenarios (including image-boundary cases), plus three
worker-entry checks pass. Build/example compilation is clean. Evidence:
/tmp/recipe-options-{regression,entry,build,examples}.log. The old toy protocol
example was removed under Recipe's no-unit-test rule; real worker and managed
CLI scenarios remain. No Ginux tests were removed.
Recipe `c622527` adds the explicit pinned compiler-image producer and real CLI
verification. Two independent builds produced SHA-256
2e99d8dc80324b185267fcbf646eb407a469751f69a0d794ffc00977e8755fba
(1,014,543,360 bytes). The actual retained recfstab build/run/publication passed
against this image, followed by a successful restored invocation that skipped
compilation, reran the executable and published it. Evidence:
/tmp/recipe-image-final.log and /tmp/recipe-recfstab-retained{,-reuse}.{stdout,stderr}.
Ginux `990fbb7a` replaces fixed per-stop sleeps with SIGCHLD/signalfd and control
socket readiness. The private signal mask and descriptor are created after the
root exec stop. Real 10,000-syscall checks take 0.121 seconds without control and
0.153 with it, including inherited ignored SIGCHLD. Checkpoint, storage, task,
thread and all existing unit/static checks pass (/tmp/ginux-event-{test,static}.log).
Recipe `adcedd0` preserves empty directories and contained relative source links,
validates complete link chains, and streams inputs up to 16 GiB/file and 32 GiB
total. Input kinds/targets/modes participate in retained identity. Indexed private
staging avoids collisions between input names and coordinator metadata. All 44
managed integration scenarios pass, including a real 513 MiB asset and rejected
oversized/escaping/cyclic inputs (/tmp/recipe-assets-regression.log).
Distro-builder `97ce3a9` adds the canonical API 2 recfstab recipe and an explicit
input/retained-state adapter. A real full source compile, version check, install
and publication passed (51 seconds compile time). A six-invocation builder case
passed installation, reuse, removal, expected not-ready, reinstallation and
cleanup using that published executable, preserving input bytes and prior
publications. Evidence: /tmp/recipe-recfstab-canonical.{stdout,stderr},
/tmp/builder-recfstab-consumer.log. Existing install_tools/materializer callers
still need migration; this is one verified consumer surface, not all consumers.
Recipe `3e340a8` adds leased coordinator scratch pools. All 44 managed checks
passed, including concurrent active-directory preservation, actual SIGKILL and
next-invocation reclamation. A final removal-race correction passed the focused
scenario again (/tmp/recipe-scratch-{regression,final}.log). Leases stay with the
supervisor and are closed before guest execution; retained state/publications
are not pruned. Older unleased temporary roots are not automatically removed.
Recipe `f79ad61` rejects persistent legacy execution before evaluating setup. The
real dependency suite exposed setup running before the previous late rejection;
that is corrected. Four dependency cases now pass, including two compiled C tools,
transitive tool lookup, retained files/state/reuse, cycle/missing/declaration
errors and early legacy rejection (/tmp/recipe-dependencies-final.log).
Ginux `cda76faa` completes bounded owned interpreter chains (up to four scripts),
including owned ELF and native image interpreter leaves. Every script stays pinned
until successful exec or failure, and argv/environment/aux reconstruction happens
in the new image. Actual chain, cycle, depth, permissions, lifetime and existing
native/owned/script/thread scenarios pass, together with all static/unit checks.
Recipe `341c61f` verifies real Bash chains and `74e6f20` preserves ordinary source
filenames including spaces, punctuation and Unicode. Evidence:
/tmp/ginux-chain-{test,regression,static,final}.log, /tmp/recipe-owned-chain.log,
/tmp/recipe-input-names.log.
Ginux `909399b4` adds merged component-by-component symlink resolution; `aa8becf2`
wires owned symlink/readlink/stat/open/exec and directory-fd calls with assignment
checks. Snapshots preserve bounded relative, absolute and long link targets;
restore rejects writes through symlink parents. All 61 unit cases and static
checks pass, along with actual storage/checkpoint/owned-exec/script/thread cases.
An existing descriptor-relative script regression exposed incorrect suffix joining;
that was fixed and rerun successfully. Evidence: /tmp/ginux-link-final-{test,static}.log,
/tmp/ginux-link-regression-test.log, /tmp/ginux-link-archive-test.log (the latter
also records the earlier failed script case). Cross-image/owned link traversal
remains unsupported and fails rather than falling through to native storage.
Recipe `85187b2` preserves symlinks inside named file/directory publications,
including long and absolute rootfs targets. Regular bytes and all parents are
materialized before links; parent collisions and named outputs that are themselves
links fail without host effects. Actual retained invocation reuse preserves the
same links, skips rebuilding and leaves the previous publication intact. All 46
managed CLI integration checks pass (/tmp/recipe-link-regression.log); focused
publication attack and retained cases pass in /tmp/recipe-link-{publication,state}-test.log.
Ginux `8b14578c` additionally prevents trailing-slash and terminal-dot namespace
mutations from following a link and deleting or renaming its target directory.
The VFS owns the shared namespace resolver. It also reports image symlink target
length in nofollow metadata. All static/unit checks and the real storage,
directory and checkpoint suites pass (/tmp/ginux-link-dot-{static,test}.log).
`d7f628cc` updates Ginux's own capability documentation without workspace policy.
Current local heads: Ginux `d7f628cc`, Recipe `85187b2`, distro-builder `97ce3a9`.
Current action: continue remaining filesystem capabilities and consumer source/cache
migration. General rootfs products still require hardlink identity, larger bounded
namespace capacity and owned-directory cwd support; source acquisition still needs
the planned bounded network backend. Do not claim the full integration is ready.
All changes remain local; no branch or pull request has been published.
Remaining: representative consumer migration, true owned hardlink identity, scalable
rootfs storage, installed-target views, Ginux SQLite locking, and the broader
integration limitations listed above still need attention. SIGKILL stops workload processes and recovery works,
and newly leased prepared roots are reclaimed on the next invocation. Older
unleased roots and automatic retained-generation cleanup remain unfinished. Cargo global-cache SQLite locking warnings remain unresolved.
Evidence: `/tmp/ginux-{noctty,cwd,packet,channel-bounds,exe-link,path-probe,memfd}-*`,
`/tmp/recipe-{long-input,deadline,cancel}-*`,
`/tmp/ginux-compiler-probe.{stdout,stderr}`, `/tmp/ginux-recfstab-probe.stderr`.
The newest diagnostic trace/audit are `/tmp/ginux-compiler-{ptrace,audit}.log`.
Active skill: `hdae-development` at
`.agents/skills/hdae-development/SKILL.md`; reload fully after context shifts.
Keep this existing plan as the continuation record; no new approval ceremony or
upstream policy files. No pushes or new PRs are authorized by this checkpoint.

This plan records the agreed objective,
source investigation, proposed interfaces, dependency-ordered tasks, and parallel
work assignments. Task IDs are scheduling references, not build stages or test
gates. No task requires a review ceremony or additional permission merely to
continue authorized implementation.

Ownership update 2026-11-05: the user stopped the previous Ginux implementation
agent and authorized this chat to continue local Recipe/Ginux integration and
edit the local LevitateOS Ginux fork. Upstream PR creation belongs to
[review and fix for ginux before PR · infra](codex://threads/01a1aa2b-ee57-7c51-9c42-cd34e76998db?hostId=remote-ssh-codex-managed%3Ainfra).
Prepare concrete local changes and runtime evidence for that thread to review;
this chat does not create or publish upstream PRs. Preserve its review checkouts,
the unfinished I04-M7 edits, and unrelated work. Existing branch restrictions
remain in force.

Local integration restart evidence 2026-11-05: refreshed upstream/main and
inspected accepted commit 6361bbd5. Its configured entry point is `ginux launch
<config> <rootfs> --result-file <path>`, while Recipe R06 at that restart invoked
`--config <config> --result-file <path>`. The I04-M1–M7 branch extends the
separate process-native demo loop and does not contain that configured-launch
entry point. Its loader progress is not evidence that Recipe's configured
boundary is connected. Preserve that work for review rather than continuing
same-fd/mmap relaxations as the next integration action.

Built an unchanged archive of upstream 6361bbd5 under
`.artifacts/ginux-local-integration/upstream-6361bbd5`, with build outputs in
`.artifacts/ginux-local-integration/build-6361bbd5`. Compiled its static launch
fixture and ran the actual upstream focused CLI integration:

    sudo -n python3 .artifacts/ginux-local-integration/upstream-6361bbd5/scripts/test-launch.py \
      .artifacts/ginux-local-integration/build-6361bbd5/x86_64-unknown-linux-gnu/release/ginux \
      .artifacts/ginux-local-integration/launch-fixture

Result: `test-launch: PASS`, covering real exits/signals, argv/environment/cwd,
descriptor handling, bounded audit, invalid configurations, result-file failures,
stopped workloads, incomplete commands, and explicit unprivileged-bootstrap
failure. The temporary chroot checks required execution outside the sandbox's
no-new-privileges restriction. This establishes the reviewed static launcher
baseline only; no Recipe worker, dynamic compiler, checkpoint generation, input
image, or output export is proved. No Ginux source, existing branch/worktree,
remote branch, or upstream PR was changed. The next change is configured
launch/root/input convergence in Recipe, with Ginux changes driven by failures
on that path and handed to the review thread before any upstream PR.

Local continuation evidence 2026-11-05:

- Attached Ginux checkout: `/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux`,
  branch `codex/ginux-recipe-local-integration`, based on reviewed 6361bbd5.
  Local commit **4632b5b2** rejects unknown/incomplete commands with exit 2,
  diagnostics on stderr, empty stdout, and no workload/result-file effects.
  Only no-argument invocation retains the interception demonstration. This
  fixes the misleading demo output triggered by Recipe's incorrect `--config`
  invocation; it does not fix that adapter or claim Recipe execution.
- The expanded actual `scripts/test-launch.py` fails against the unchanged
  6361bbd5 binary on `--config` (demo output, exit 1). After the fix, a fresh
  offline release build and the full configured-launch suite pass, including
  sixteen invalid-command cases. Formatting and diff whitespace checks pass.
  The build is `/tmp/ginux-recipe-local-build/x86_64-unknown-linux-gnu/release/ginux`.
- Recipe local commit **7b06db6** corrects its mistyped default rootfs digest to
  `1026f8d3b36eaea1bcab8e609045991f08c70eb7645cb37f58f48faa8b40be9c`.
  Direct SHA-256 hashing of the cached assembled tar matches that value and
  its recorded rootfs identity. The current Recipe binary rebuild passes.
  `verify-rocky-oci.sh` was not completed: this host lacks `shasum`; do not
  describe that as a fresh full OCI-chain verification.
- A real configured `launch` probe runs that rebuilt Recipe worker as domain
  init, with five runtime libraries read from the cached Rocky tar, explicit
  worker environment, and a temporary root containing `/inputs/main.rhai`.
  Audit records show successful library opens/reads/maps and protections.
  The first denied call is `poll` (7), followed by `gettid` (186) and
  `tgkill` (234); the worker terminates by SIGSEGV (exit 139), emitting zero
  stdout bytes. Its authoritative result is `signaled`, signal 11. These are
  actual configured-path observations, not the old process-native loader
  reconstruction. Which denied call causes which failure remains to be isolated.
- The probe receives EOF on stdin, not a Recipe request. It is a startup
  diagnostic with selected copied libraries, not an assembled invocation
  image, successful Rhai evaluation, coordinator transport, or publication
  proof. Script, config, result, exact argv, stdout, and full audit are saved
  under `.artifacts/ginux-local-integration/configured-worker-probe/`.
  Reproduction: `sudo -n python3 .artifacts/ginux-local-integration/configured-worker-probe/probe.py`.

Configured execution checkpoint 2026-11-05:

- Recipe **2f90e5f** connects R05/R06 to `ginux launch <config> <root> ...`.
  The input owner verifies a snapshot of the assembled rootfs tar, builds a
  fresh root, copies/reverifies inputs, and stages the real worker. It rejects
  image paths that escape, write through symlinks, duplicate files, or replace
  invocation-owned paths. Root cleanup restores directory access without
  following symlinks; **ce4bdfb** counts every archive entry against the bound.
  Image selection, library-worker selection, and exact limits live in Recipe's
  author guide and `inputs/root.rs`. No retained-state claim is made.
- Ginux **3dd05f42** mediates bounded zero-time `poll` queries for assigned
  bootstrap descriptors. Before the change, the real fixture fails on syscall 7;
  after it, the full configured-launch suite passes, including invalid pointer,
  descriptor, count, timeout, ignored-slot, and closed-descriptor cases. All five
  core unit cases pass with the Makefile's `RUSTFLAGS=''` convention; the first
  invocation without that override failed to link with freestanding flags.
- With polling fixed, the actual worker reaches initialization and reports that
  its redundant mkdir of the already-prepared BUILD_DIR is denied. Recipe
  **c780ee8** makes bootstrap require and validate the prepared directory. The
  next actual CLI readiness query succeeds: custom Rhai code returns discovered
  context, the supervisor and worker exit zero, and a host-private 0600 source
  remains unchanged. Evidence is in
  `.artifacts/ginux-local-integration/configured-readiness/`.
- Actual managed inheritance exposed a separate API 2 bug: `iter_raw()` visits
  newer scope bindings first, but initialization selected the last ctx and
  replaced the child's ctx with the base. Recipe **33262d1** selects the newest
  binding without flattening shared values. Its retained regression covers
  child overrides, inherited functions, imported modules, and a ctx captured by
  a closure. The pre-fix failure is retained alongside the passing run.
- Final focused verification: compile `cargo test -p levitate-recipe --test e2e
  --no-run --offline`, then run its emitted binary with `managed_inputs::
  --nocapture`, the real `GINUX_BIN`, verified `GINUX_ROOTFS`, and bootstrap
  privilege available. **8 passed, 0 failed, 0 ignored** (25.00 s). This includes
  the four image/worker guards, three independent custom checks with discovered
  context, explicit not-ready and genuine script errors, inherited/imported
  code and shared ctx, source preservation, and a denied absolute host write.
  Raw before/after output is in
  `.artifacts/ginux-local-integration/managed-inputs/`.

Public entry and checkpoint continuation 2026-11-05:

- Recipe **7992bd9** removes `Backend::Host` and all environment-driven public
  host selection. The default library backend now resolves required Ginux at
  operation time; `RecipeEngine::backend()` returns `Result<Backend>`. The real
  library regression first failed because a spoofed worker environment caused
  host evaluation (runtime recipe directory was a host `/tmp/...` path).
  After the change it passes through Ginux and reports `/inputs`, `/work/build`,
  and `/work/tools`, even with those spoofed variables.
- Coordinator and worker use the private argv entry together. The worker binds
  its request id to bootstrap identity before setup. Two actual CLI guards pass
  for public environment spoofing/legacy host selection and private missing or
  mismatched bootstrap; the latter decodes the real failure/result frames.
  Worker engine construction now uses the shared invocation factory.
- Prepared tools, engine globals/runtime, dependency destination and PATH use
  `/work/tools`. Removal now receives the prepared BUILD_DIR. The eight existing
  managed input/readiness cases pass again with these paths and private entry,
  and the downstream `cargo check -p distro-builder --offline` passes.
- A real file-producing install reaches acquire and is denied at `write_file`.
  Cleanup/removal hooks reached their checkpoint, but acknowledgement waiting
  was denied because their finite poll timeout exceeded the old zero-time-only
  policy. These original failures and source are retained under
  `.artifacts/ginux-local-integration/worker-entry/before-finite-wait-*`.
- Ginux **f8235d56** allows finite waits through 300000 ms on the same assigned
  descriptors. No descriptor ownership or writable-filesystem authority changed.
  The full real configured-launch suite passes, including the maximum accepted
  timeout, rejected overflow/infinite waits and existing descriptor/pointer bounds.
  All five core tests pass with the documented `RUSTFLAGS=''` unit-test convention.
- Two additional actual managed cases pass: cleanup and remove return hook state
  after one acknowledged v1 checkpoint, and missing cleanup is a no-op with zero
  checkpoints. These hooks do not mutate installed artifacts and the root is
  fresh/ephemeral; this is not real installed-target removal or durable recovery.
  The ack still carries placeholder generation 0 and the Ginux result generation
  remains null. Logs for these two cases, two CLI guards, eight managed cases,
  and the real library regression are saved in
  `.artifacts/ginux-local-integration/worker-entry/` (13 passing test cases total).

Writable-file continuation 2026-11-05:

- Ginux **222d29ff** extends the existing VFS with directory/file metadata,
  file positioning, sparse/append IO, chmod and exclusive create. It fixes
  descriptor-exhaustion side effects, reads beyond EOF, and unlink/recreate
  object lifetime. Seven VFS cases pass, retaining all four upstream cases.
- Ginux **10091e4e** connects supervisor-owned storage to configured launch.
  Generic `memory:/absolute/path` assignments declare empty writable namespaces;
  they never assign a writable host directory or permit native write fallback.
  Native image/loader reads remain confined to the prepared root. Virtual
  descriptors start at 1048576, with native soft/hard limits clamped below that
  namespace. Parent-only process_vm copies respect guest page protections;
  failed/partial read copyout restores the correct file offset. Metadata uses
  virtual object identities. The x86-64 bootstrap mkdirat number is corrected.
- This is a bounded ephemeral backend: 32 objects including directories, 64 KiB
  per file, 64 open files, 4096-byte IO calls. Directory descriptors/enumeration/
  removal, rename, links, shared-offset duplicate descriptors, generated-file
  execution/mapping, scalable backing, retention, configured storage quotas and
  export remain unfinished. It is not sufficient for an ordinary Cargo build.
  Do not mark old library-only G06–G13 records as integrated based on this slice.
- The real `scripts/test-launch.py` and new `scripts/test-launch-storage.py`
  both pass with the final release binary. The latter exercises fresh-launch
  isolation, hidden image files, actual bytes and metadata, unlink/recreate,
  exclusive create, bad/readonly/partially inaccessible pointers, native FD hard
  limits, file/descriptor/object exhaustion, malformed assignments and no
  storage assignment. Makefile producers/scenarios and existing Linux CI are
  extended; upstream tests are retained. All 12 focused core/VFS cases pass.
  Release clippy passes with `-D warnings`; the initial dev-profile clippy
  command failed because this no_std binary requires release panic=abort.
- Recipe **ab9fc89** declares memory storage at `/work`, `/work/build`,
  `/work/tools`, `/work/tmp` and `/out`. Its actual acquire/build/install now
  transforms `prepared source` into `prepared source: built: installed`;
  auto.install.success cleanup reads the product and removes the build file.
  Six v1 checkpoints are acknowledged. Separate cleanup/remove scenarios read
  and delete actual domain artifacts created in the same invocation. Source
  files remain unchanged. These prove ephemeral file effects, not target
  removal across invocations, durable generations or host publication.
- Final actual Recipe checks: 12 managed CLI cases, two worker-entry guards,
  and one library-default case pass (15 total), using the final real supervisor,
  verified Rocky tar and CLI worker. Commands, environment and raw logs are in
  `.artifacts/ginux-local-integration/memory-storage/checks.json` and adjacent
  logs. The library case spoofs worker environment and still uses Ginux.
- A subsequent actual image-child probe (`/usr/bin/bash -c 'exit 0'`, image ELF
  verified present) fails during helper startup with ENOENT, before any build
  checkpoint. Source and stdout/stderr are retained under the same directory.
  The helper requests Stdio::null() and two pipes before spawn; the missing
  device/channel/process operation must be isolated on configured launch.
  No syscall-cause claim or successful child execution is made.

Null-device continuation 2026-11-05:

- The previous goal turn made concrete progress (managed file lifecycles and
  tested source changes); no user/external blocker is preventing continuation.
  The next actual child failure was isolated by tracing the supervisor's own
  `process_vm_readv` calls, without attaching a second tracer to its worker.
  The last copied path is `/dev/null`; the corresponding native openat returns
  -2. The helper opens Stdio::null() before creating its stdout/stderr pipes.
- Ginux **e3458063** projects an immutable virtual `/dev` view on configured
  launch, empty by default. Explicit `[devices].assignments = ["null"]`
  connects the existing ginux-devices DeviceRegistry/Null implementation.
  Reads return EOF, bounded writes discard bytes, and stat/fstat/fstatat/statx
  identify a character device with virtual inode and major/minor 1:3. No host
  device node or descriptor is used. Image entries beneath `/dev` remain hidden,
  including when no device is assigned. Unsupported/duplicate assignments fail
  before launch. Access modes, seek, descriptor flags and close are mediated;
  the 64 device descriptors occupy a reserved range and device-only launch also
  clamps native limits. Duplication, device polling and other device types
  remain unsupported on configured launch.
- Files and devices reuse controlled guest-memory copying and stat serialization
  in tracee_io.rs. Configured object resolution is centralized in dispatch.rs;
  device handles resolve before filesystem rejection of unknown virtual FDs.
  No writable native syscall, native child, or host-device fallback was enabled.
- Fresh release build and release clippy with `-D warnings` pass. The actual
  launch, storage and new launch-devices suites all pass. Device integration
  covers IO/metadata/access/close, invalid/read-only copyout, descriptor exhaustion,
  assignment errors, image preservation/hiding, fresh instances, and both
  device-only and storage-plus-device configurations. All six existing device
  unit cases pass. Makefile producers/scenarios and existing Linux CI include
  the device fixture; upstream tests remain intact.
- Recipe **50edb61** explicitly assigns null. A new actual Rhai helper check
  writes bytes to `/dev/null`, reads EOF, distinguishes it from a regular file,
  and sees `/dev` as a directory, while preserving source. The complete focused
  real checks pass: 13 managed CLI cases, two worker-entry guards and one
  library-default case (16 total). Commands/environment and raw output are in
  `.artifacts/ginux-local-integration/null-device/checks.json` and adjacent logs.
- The same actual image-child probe now opens the owned null handle
  (1073741824), then fails at **pipe2 (293), denied ENOSYS (-38)**, and closes
  the null handle. This is a changed, observed failure, not successful child
  execution. Before/after supervisor audits and VM-copy traces, source and CLI
  stdout/stderr are retained in that artifact directory. The child recipe
  remains unchanged, fails explicitly in build and acknowledges zero checkpoints.

Owned-pipe continuation 2026-11-05:

- Ginux **7f072f2c** emulates pipe/pipe2 in the existing channel owner, using
  supervisor-owned memory and handles, not the old native-pipe mirror. Eight
  64-KiB objects and 64 pipe descriptors bound allocation; flags accept only
  CLOEXEC/NONBLOCK. IO is bounded to 4096 bytes, small writes are atomic,
  failed/partial copyout consumes only delivered bytes, and aliases preserve
  endpoint lifetimes/status flags with independent close-on-exec flags.
  EOF, SIGPIPE, metadata, seek rejection, limits and pipe-table duplication
  are implemented. All configured starts now constrain native descriptor limits.
- Blocking empty/full operations stay at entry instead of returning EAGAIN.
  The single-task configured loop cannot run peers or restart interrupted waits;
  a parked self-pipe stays blocked until termination. Real SIGKILL/reaping cases
  pass. Multi-task wakeup, cross-owner/stdio aliases, inherited tables, polling,
  and exec-time close-on-exec remain unfinished. No child was enabled.
- The new real launch-pipes suite and existing launch, storage and device suites
  pass; release build/clippy with `-D warnings` pass. The pipe fixture covers
  byte-exact wraparound, atomicity, bad/read-only/partially inaccessible buffers,
  allocation rollback, alias replacement/flags/lifetime, EOF, object/descriptor
  exhaustion, blocking read/write, SIGPIPE and fresh instances. CI/Makefile include
  the explicit producer/scenario. **f1c6419e** fixes an observation race by reading
  parked audits through a separate file description; the pipe suite passes again.
- All 16 actual focused Recipe CLI/library cases pass against this supervisor.
  The same real child helper now creates both stdio pipes but fails with EBADF
  before process creation. Its audit includes a native prlimit query followed by
  cleanup of owned null/pipe handles; no clone/exec call is reached. Inspection
  of the pinned image libc shows posix_spawn_file_actions_adddup2 checking both
  numbers through __spawn_valid_fd against sysconf(_SC_OPEN_MAX). The current
  sparse public handle ranges exceed the reported native descriptor limit.
  Green raw-syscall fixtures therefore do not prove libc spawn compatibility.
- Before/after audits, source, CLI stdout/stderr, pinned-libc disassembly,
  commands/environment and 16-case output live under
  `.artifacts/ginux-local-integration/owned-pipes/`. Recipe **07b61f2** updates
  its author guide to the observed behavior; executable code is unchanged from
  50edb61. Local Ginux changes remain in this attached worktree for review.

Shared-descriptor continuation 2026-11-05:

- Ginux **3b00c1c8** replaces separate public ranges with a common table of
  256 small descriptors for image files, writable files, null, pipes and stdio.
  Lowest-free allocation reserves before open/create/truncate, with rollback
  on failure. Only actually open bootstrap stdio is registered. Backend handles
  remain private; forged old handle values return EBADF. Aliases across owners
  share offsets/lifetimes; descriptor flags are independent. Last-alias close
  releases the backend, including a supervised native close when needed.
- Native opens commit their kernel result into the reserved public slot.
  Native IO/metadata/mmap translate handles and restore syscall arguments at
  exit; replaced native fd numbers can be reused without changing an owned
  descriptor's meaning. Stdio cannot become a file mapping or directory view.
  Native fcntl locks, leases and signal-ownership operations are denied.
  The obsolete pipe-specific duplication/creation-copyout path is removed.
- The real native soft/hard limit is capped at 256, preserving lower operator
  limits; public allocation uses the same lower bound. No limit reporting was
  inflated. Runtime NOFILE changes are explicitly denied for now. Relative
  directory-fd access, readiness beyond original bootstrap consoles, inherited
  per-process tables, peer scheduling and actual exec-time CLOEXEC remain
  incomplete. The table is currently used by one configured task.
- Five real Ginux suites pass: launch, storage, devices, pipes and new fds.
  The latter checks cross-owner/stdout replacement, native reuse/mmap, shared
  file offsets, argument-register preservation, private-handle rejection,
  exhaustion without truncation, fresh launch, and a real lower limit of eight.
  Existing fixtures keep their data/negative assertions and now expect small
  public numbers; no raw syscall success is used to claim child execution.
  Release build/clippy with `-D warnings` pass. Makefile/CI include the explicit
  descriptor producer/scenario. All 16 real focused Recipe CLI/library cases pass.
- The unchanged actual child probe now passes libc preflight and reaches
  **clone3 (435), then clone (56), both denied ENOSYS**. No child runs; the
  worker returns a structured build failure with zero checkpoints and exits
  normally. Before/after audits, source, stdout/stderr, commands/environment,
  all five Ginux suite logs and the 16-case logs are retained under
  `.artifacts/ginux-local-integration/shared-descriptors/`.
- Recipe **6bbb9a3** updates author guidance to current behavior; executable
  Recipe code is unchanged. The previous goal turn and this turn both made
  concrete implementation/runtime progress; there is no external blocker.

Configured-process continuation 2026-11-05: Ginux **ccd62ca1** on the existing
local integration branch connects the configured launcher to ProcessTable:

- Up to 16 processes, virtual IDs starting at 1, reserved before native creation.
  Ptrace birth stops install ownership and inherited descriptor tables before
  execution. Fork/vfork and clone flags SIGCHLD or VM|VFORK|SIGCHLD are supported;
  clone3, threads, namespace and untraced/shared-fd shapes remain denied.
- Native file tables retain kernel shared offsets. Owned file/device/pipe
  references span all process tables; either process can close or exit without
  prematurely releasing a peer's handles. Blocked owned pipe IO retries after
  object progress/death; unrelated stops no longer exhaust the audit budget.
- Wait4 supports positive virtual child IDs or -1, 0/WNOHANG and null rusage.
  Status comes from observed ptrace death. Copyout precedes reap; EFAULT preserves
  the child. Native wait is used where possible; adopted children already reaped
  by the supervisor return their observed virtual status after native ECHILD.
  Kernel fork failure rolls back the process reservation and owned references.
- Virtual init adopts orphans, and the native supervisor is a subreaper. Root
  death and supervisor failure kill/reap all descendants and release all owned
  state before completion. Signal sender PIDs are projected. Positive-ID kill
  supports 0/SIGKILL. Bootstrap normalizes ignored SIGCHLD; unsupported auto-reap
  dispositions fail explicitly. Job control, parked-call interruption/restart,
  adopted-child SIGCHLD notification and broader wait/signal forms remain open.
- Release build/clippy with -D warnings, eight process unit cases and six actual
  Ginux suites pass. The new task suite covers fork/vfork/both clone shapes,
  identities, cross-process pipes/native and memory file offsets/device lifetime,
  EOF, WNOHANG, failed status copyout, signal status, 16-task bounds, fork rollback
  under an actual kernel process limit, orphan waits and /proc-confirmed absence
  of descendants after root signal and supervisor failure. Existing launch fork
  coverage now verifies an actual child status; Makefile/CI retain the new suite.
- All 16 focused actual Recipe CLI/library cases pass with the real launcher and
  full pinned image. The unchanged real Bash helper now creates supervised child
  2, performs signal and stdio setup, then reaches **execve (59), denied ENOSYS**.
  Child exit/wait and structured build failure complete without hanging; zero
  checkpoints are acknowledged. Bash itself has not executed. Recipe executable
  source is unchanged; **d9473f8** corrects its author guide.
- Raw audits, child source/stdout/stderr, command/environment records, six Ginux
  suite logs, build identity and 16-case results are retained under
  `.artifacts/ginux-local-integration/task-supervision/`. Verification requires
  root for the real chroot bootstrap; an initial unprivileged attempt failed
  before managed code, then the correctly privileged cases passed.

Configured-exec continuation 2026-11-05: Ginux **8f768700** on the existing
local integration branch implements image execve/execveat:

- The configured task owner reserves exec, replays its verified x86-64 syscall
  instruction to synchronize native CLOEXEC flags through controlled fcntl calls,
  and commits public descriptor retirement only at PTRACE_EVENT_EXEC. No code
  bytes are patched. Mixed public aliases preserve their shared native handle;
  owned handles release only after successful replacement. Failed exec retains
  all public descriptors and flags. Bootstrap CLOEXEC stdio is no longer
  represented as a live public console after the initial exec.
- Identity survives replacement; the existing process owner advances exec state.
  Supported paths include current-workdir-relative execve, absolute execveat,
  image-fd AT_EMPTY_PATH and image symlinks. Relative directory-fd execution and
  generated-file execution remain unsupported. The path view is still lexical;
  a unified image/overlay resolver remains future storage work. Signals during
  descriptor synchronization fail the domain explicitly; restart/deferred
  delivery for that interval remains incomplete.
- Seven actual Ginux runtime suites and release build/clippy with -D warnings
  pass. New exec coverage checks failed ENOENT/EACCES/ENOEXEC/EFAULT calls,
  rejected owned/hidden paths, per-alias flags and owned lifetimes, two successive
  replacements, fork isolation, execveat, relative paths and symlinks. The runner
  inspects actual /proc kernel descriptors before new opens can hide a leaked
  native handle. Makefile/CI retain the fixture producer and scenario.
- All 16 focused actual Recipe CLI/library cases pass after the subprocess fix.
  The actual Bash helper now completes a supervised exec event and starts its
  dynamic loader. The Recipe worker then attempts **clone flags 0x3d0f00** for
  its stdout-draining pthread; clone3 fallback and clone remain denied for threads.
  This initially panicked std::thread::spawn, ending the worker with code 101
  and breaking the protocol. It is not a successful Bash command result.
- Recipe **7f30e3c** uses fallible thread builders for both output readers. A
  startup failure kills/reaps the child and joins any reader already started;
  it returns an ordinary command error. Replaying the actual helper now returns
  a structured build failure, zero acknowledged checkpoints and launcher exit 0,
  with the child reaped and no worker panic. Author guidance reflects the limits.
- Before/after panic/error audits, child source/stdout/stderr, exact clone registers,
  build/worker identities, commands/environment, seven suite logs and 16-case logs
  live under `.artifacts/ginux-local-integration/exec-supervision/`. These are
  real privileged chroot runs using the pinned Rocky image and actual worker.

Configured-thread continuation 2026-11-05: Ginux **928dc9a1/e59e2e61** adds
tracked pthreads and shared descriptor ownership on the existing local branch:

- Per-task DescriptorCall transactions are separate from shared process tables.
  The observed x86-64 clone shape 0x3d0f00 registers before execution and shares
  the table. getpid uses tgid, gettid uses virtual TID; native PARENT_SETTID
  copyout is suppressed and replaced with virtual identity at birth. Clear-TID
  and ordinary native futex waits/wakes support joins. Failed native thread
  creation rolls back without changing the caller's TID slot.
- Blocked owned IO pins its original handle through a peer close/reuse. Native
  open/close transactions remain per task; fork waits for table mutations and
  holds a snapshot barrier until child birth. Threads share process child waits,
  with a zombie reservation preventing duplicate concurrent reaps. Thread exit
  retains peer handles, and leader exit waits for remaining threads before the
  process ends. Whole-group signal/failure teardown reaps native TIDs and releases
  pins, aliases and process records. Unpublished creation reservations roll back
  if their creator dies. Read-only getcwd is mediated inside the chroot.
- Exec still requires a sole process leader; other threaded exec forms fail
  explicitly. PI futex operations are rejected before they can write native owner
  TIDs. Robust owner-death behavior, broader clone/wait/signal variants and full
  readiness remain unfinished; this is not complete Linux threading support.
- Release build/clippy with -D warnings, 14 core/process cases, and eight actual
  Ginux suites pass. The new thread suite verifies virtual TIDs, shared close/
  offsets, blocked-read pinning across fd reuse, native-fd concurrency and fork
  snapshots, fork from a nonleader, 16-task bounds/reuse, native rollback, leader
  exit, nonleader exec rejection and /proc-confirmed whole-group teardown on root
  signal and supervisor failure. Makefile/CI retain this producer and scenario.
- The new actual Recipe CLI case now runs Bash, gets child code 0, checks exact
  domain-file bytes through independent readiness checks, installs/rechecks them,
  returns a named output, parses clean JSON and preserves recipe source. Its
  worker reports ok=true and two checkpoints. The case then **fails** because
  child-stdout/child-stderr are absent from CLI diagnostics: run_domain drains
  them into a temporary combined log and discards it after success. This case
  remains uncommitted at tools/recipe/tests/e2e/managed_inputs.rs; do not weaken
  its output assertions or call it passed. Recipe **437c325** updates the guide.
- All 16 previously established actual CLI/library cases pass separately, with
  the new diagnostic case explicitly excluded and recorded as failing. Raw logs,
  commands/environment, exact failure line, prerequisite assertions, build
  identity and source snapshot are under
  `.artifacts/ginux-local-integration/thread-supervision/`. The earlier always-
  not-ready probe now fails at its intentional readiness rule after command
  completion; the new case supplies proper readiness and exposes the log gap.

Managed-diagnostic continuation 2026-11-05: Recipe **096a656/0ffa1bf** completes
the retained child-output case with worker transport v2:

- Op 6 carries typed stdout/stderr/Recipe diagnostic byte arrays. Child binary
  output, Rhai print/debug/log/warn and phase narration reach CLI stderr without
  parsing supervisor audit or contaminating JSON. The worker installs its sink
  explicitly after validating the private request; environment flags do not
  install or select it. All thread sends and checkpoint/result frames share the
  sender lock; it is released before waiting for checkpoint acks.
- Diagnostic messages hold at most 768 bytes. Reads/writes are chunked at 4096
  bytes so larger valid contexts cross the configured console ABI. The worker
  and host independently enforce 16 MiB/32768-message budgets. One empty marker
  reports truncation, excess output continues draining, and lifecycle/result
  frames still complete. Transport failure prevents successful termination.
  Ginux config/result schema remains 1; Recipe API 2 is independent. Historical
  v1 payload documentation is retained; future incompatible persistence is v3.
- All 20 focused actual CLI/library cases pass: 15 managed cases, three entry
  guards and one library case in the grouped run, plus a separate frame-budget
  case. The formerly failing Bash case now checks exact installed domain bytes,
  both streams, NUL/0xff bytes, Rhai print/debug/warn, a 32 KiB context, clean JSON,
  no supervisor-audit leakage and source immutability. Two actual excess-output
  cases prove byte and message truncation while preserving later readiness and
  result. A real private-entry negative rejects v1 before any recipe setup.
- CLI build, integration compilation and examples compile. Strict Clippy was
  attempted and remains red on 11 findings in unchanged executor, invocation,
  torrent and input files; no clean strict-lint result is claimed. No unit tests
  were added or run for Recipe. Ginux code is unchanged this slice.
- Evidence, commands/environment, the grouped 19-case logs, separate frame-limit
  result, binary identity and the next probe are under
  `.artifacts/ginux-local-integration/diagnostic-transport/`. The previously
  uncommitted child regression is now committed and passing. Live-peer
  complete-frame deadlines, durable logging/state and cancellation remain open.

Managed-capture continuation 2026-11-05: Ginux **75acd2be/210a8cca** and Recipe
**4275769** fix the actual captured-output path:

- FIONBIO updates the pipe owner's shared O_NONBLOCK status after a complete,
  validated four-byte copyin. Aliases see the update; access/APPEND and public
  CLOEXEC flags remain intact. Invalid/partial pointers leave flags unchanged.
- Configured poll resolves public handles through their owners, combining pipe
  input/output, HUP/ERR, assigned regular memory files/directories, null and
  console aliases. Closed/unknown descriptors return POLLNVAL; negative slots
  are ignored. No private handle or untracked host FD is polled. Image-file
  polling remains explicitly EOPNOTSUPP.
- Finite deadlines use monotonic nanoseconds and a 1 ms observation tick;
  negative timeouts wait indefinitely. Infinite owned-object waits need no
  periodic retry. Object changes and observed peer deaths wake parked calls.
  A SIGKILL race during retry now waits for the real native death event instead
  of converting ptrace ESRCH into a supervisor failure. General nonfatal
  interruption/restart is still unfinished. Limits remain 16 slots and at most
  300000 ms for a positive timeout.
- All nine Ginux launch suites pass, including the new ten-scenario polling
  runner: masks/aliases/mixed consoles, faulty memory, HUP/ERR/EOF, read/write
  wakeups across actual fork, peer exit, finite timeout under a busy child,
  zero-FD sleep, and finite/infinite parked-task teardown. Existing bootstrap
  assertions now check the newly supported negative timeout and POLLNVAL/EFAULT
  semantics. Producers and independent scenarios are connected to Make/CI.
  Ginux release build and strict Clippy pass.
- All 22 focused actual Recipe CLI/library cases pass (18 managed, three entry
  guards, one library case). New cases capture 80,000 stdout bytes and more than
  a pipe buffer of stderr, preserve whitespace and empty stdout, keep stdout
  return values distinct from diagnostics/JSON framing, and report child exit
  23 as a custom-check error without a panic or fabricated readiness. Source
  files remain unchanged. No Recipe implementation change or unit tests were
  needed for this slice; the existing std output reader now works.
- Commands, environments, logs and binary/source identities are under
  `.artifacts/ginux-local-integration/captured-output/`. The original FIONBIO
  failure evidence remains under `diagnostic-transport/`. These commits are
  local review inputs; no push or PR was performed.

Growable-storage continuation 2026-11-05: Ginux **06d24ae2** removes the
configured launcher's fixed per-file byte array and connects growable anonymous
backing through the existing VFS owner. Recipe **acf174c** verifies a larger
file-producing lifecycle through the mandatory launcher:

- VirtualFs keeps namespace/metadata/open-description ownership; OverlayStorage
  holds bytes. Existing freestanding/small users retain the default inline
  backing and constructor. Configured launch maps byte capacity on demand,
  supports 4096 metadata objects and retains its 64 independently opened-file
  bound. Total mapped capacity (including page rounding/growth reserve) is
  bounded by resources.memory_mib. This bounds overlay backing only; total
  supervisor/tracee RAM enforcement is still unfinished.
- ftruncate grows/shrinks the same owned object without moving its open offset.
  Sparse gaps and truncate/regrow zero exposed bytes, failed growth preserves
  metadata/content/offset, and shrink returns unused mapped pages. Unlink keeps
  bytes while a descriptor/alias/fork reference exists; final close releases
  them. Domain teardown clears all backing and asserts zero mapped bytes.
  No host writable file, mapping handle or backend integer reaches the guest.
- The real storage fixture verifies writes beyond 64 KiB, a sparse 2 MiB offset,
  truncate/regrow zeroing, exactly 4096 total namespace objects, and allocation
  exhaustion across multiple files under a configured 16 MiB bound. Its quota
  checks preserve failed-file size, reclaim shrinking/closed-unlinked objects,
  retain unlinked bytes across public aliases, and leave full-quota data live
  for domain teardown. Fresh launches preserve the prepared host root.
- All nine actual Ginux launch suites and eight VFS unit cases pass. Release
  build and strict Clippy pass. The added VFS case also proves that custom byte
  backing, rather than the legacy inline-size parameter, determines capacity;
  failed resize and truncate preserve the expected bytes and open offset.
- All 23 focused Recipe CLI/library cases pass (19 managed, three entry guards,
  one library case). The new actual CLI install writes and reads 131072-byte
  work/output files, independently rechecks their contents, returns the named
  output and keeps the recipe source unchanged. Existing capture/logging,
  process/thread/descriptor and input guards remain passing. No Recipe unit
  tests were added or run.
- Evidence, exact commands/environment, source and binary/image identities are
  under `.artifacts/ginux-local-integration/growable-storage/`. All commits are
  local review inputs; no push or PR was performed.

Directory-storage continuation 2026-11-05: Ginux **9ba6a1d4** adds owned
directory operations; Recipe **2303bb7** exercises nested production and cleanup:

- VirtualFs owns directory descriptions, object identity and enumeration
  cookies. getdents64 serializes bounded, aligned records with direct children,
  domain inode IDs and types. It commits the cursor only after complete copyout.
  Public aliases/fork share the same description; native handles/private backend
  numbers never bypass ownership. Owned relative openat/mkdirat/fstatat/
  unlinkat/chmodat resolve the current object path, including after rename.
  Relative native-image directory handles remain explicitly unsupported.
- Rename, renameat and renameat2 (zero flags/NOREPLACE) move overlay files/trees.
  Preflight checks every descendant path and replacement rule before mutation.
  Replaced/unlinked open objects retain their bytes/inode; removed directory
  handles retain metadata but enumerate empty and cannot create children.
  Nonempty removal, self-descendant moves, invalid replacements, protected roots
  and moves across the assigned/image boundary fail explicitly. Overlapping
  assignments merely seed directories beneath the minimal protected roots.
  Image subtree mutation, links and a full unified resolver remain unfinished.
- The real copy probe exposed two additional requirements: libc opendir uses
  flags 0x90800 (including O_NONBLOCK), and std copy passes full st_mode 0x81a4
  to fchmod. Status flags now retain NONBLOCK; chmod/fchmod ignore file-type
  bits while preserving the prior denial of special permission bits. The
  supervisor-only register/audit trace of the fchmod failure is retained.
- All ten actual Ginux launch suites and ten VFS cases pass. The new native
  family checks complete/short/faulting dirents, EOF and rewind, shared cursors
  across aliases/fork, relative operations, file/tree replacement with open
  handles, removed directory behavior, NOREPLACE/unsupported flag rejection,
  seeded-directory removal, assignment/input isolation and clean teardown.
  Core checks include failed overlong subtree moves leaving names unchanged.
  Release build and strict Clippy pass; Make/CI have an explicit producer and
  independent directory scenario.
- All 25 focused actual Recipe CLI/library cases pass (21 managed, three entry
  guards, one library case). The new install copies a nested tree containing
  131072 bytes, verifies contents with independent checks, moves the staged
  output, and recursively removes scratch trees via cleanup(ctx, reason).
  Another case rejects replacing the actual staged /inputs/main.rhai before
  mutation. Both preserve source and clean JSON/error transport. The original
  prepared directory-copy and rename probes now succeed. No Recipe unit tests
  were added or run.
- Evidence, exact commands/environment, source/binary/image identities and
  supervisor traces are under
  `.artifacts/ginux-local-integration/directory-storage/`. All commits remain
  local review inputs; no push or PR was performed.

Owned-ELF continuation 2026-11-05: Ginux **c5ae1c5d** executes generated ELF
files through the existing supervised transaction; Recipe **287701b** verifies
streamed/captured execution and content-checked installation:

- VFS executable references validate permissions and reject open writers with
  ETXTBSY. They preserve object identity across rename/unlink, inherit on fork,
  survive until the final process using the executable exits/replaces it, and
  never change an ordinary file offset. Pending references release on failed
  exec/death; process groups release committed references before teardown.
- The transaction injects a private CLOEXEC memfd_create into the stopped
  tracee. The supervisor opens only that registered task's freshly returned
  kernel fd via procfs, fills it from pinned VFS bytes, sets mode 0500 and
  verifies mandatory write/grow/shrink/seal locks. It closes its own fd, then
  injects execveat of the sealed backing. The private fd never enters the public
  table and closes on kernel success or explicit failed-exec cleanup. There is
  no named host output or public memfd syscall allowance. Image exec behavior
  and the PTRACE_EVENT_EXEC CLOEXEC commit boundary remain intact.
- Owned execve/execveat supports paths, owned directory-fd relative paths, and
  AT_EMPTY_PATH on open/unlinked files. A temporary native fd and supervisor
  procfs access to its stopped tracee are required; failures propagate without
  fallback. Sole process-leader exec remains required. Snapshot kernel memory
  is additional to the VFS mapping bound; total process/kernel resource
  accounting is unfinished. Guest mmap of owned files is still unsupported.
- All eleven actual Ginux launch suites and eleven VFS cases pass; release
  build and strict Clippy pass. The new seven-mode family verifies invalid
  argv and malformed ELF rollback, permission/write exclusion, public CLOEXEC
  preservation, source offsets, direct/descriptor/unlinked/relative exec,
  fork/vfork/CLONE_VM lifetime, and eventual write permission after final exec
  reference release. Host /proc inspection independently verifies no private
  fd before/after exec, exact running ELF bytes, required seals and mode 0500.
- All 27 focused actual Recipe CLI/library cases pass (23 managed, three entry
  guards, one library case). The new lifecycle copies the prepared Bash ELF to
  /work/build, runs that owned executable through streamed and captured helpers,
  validates its produced bytes with custom checks, and installs a named product.
  Another case rejects non-executable owned data with EACCES. Source files and
  result/diagnostic framing remain intact. No Recipe unit tests were run.
- Correction to the previous probe description: the prepared /usr/bin/true is
  a **51-byte shebang wrapper**, not an ELF. Direct artifact inspection verifies
  that header and the **1410688-byte Bash ELF** used by the passing probe. The
  old true-wrapper EOPNOTSUPP result remains valid script evidence; it never
  established a failing ELF test. Generated scripts are still unsupported.
  Program-byte evidence, exact source/binary identities, commands/logs and the
  passing ELF probe are under
  `.artifacts/ginux-local-integration/owned-exec/`. All commits are local review
  inputs; no push or PR was performed.

Named-script continuation 2026-11-05: Ginux **73593745** and Recipe
**17d145c** connect generated scripts with image interpreters through the
existing supervised exec transaction:

- The kernel performs shebang dispatch, preserving its single optional argument
  and recursive image interpreter prefix. The supervisor verifies the resulting
  script argument, then restores the caller's pathname in argv and AT_EXECFN in
  the new exec stack. Original arguments/environment remain intact. Old address
  spaces are never scratch, including vfork/CLONE_VM. Stack restoration failure
  after kernel commit stops the domain; it cannot return to the old program.
- The sealed script backing survives kernel exec without public descriptor
  exposure. Its private fd closes before the interpreter's first syscall; the
  verified syscall instruction replays with original registers. Failed kernel
  exec closes its private fd before returning the real errno and preserves all
  public CLOEXEC entries. Pending pins survive until commit or teardown;
  committed scripts release their data pin, while running ELF pins retain their
  process lifetime. The freestanding binary now supplies overlap-safe memmove
  required by the larger transaction state.
- All **twelve** actual Ginux launch suites pass. The new independent script
  family covers nine modes: plain/optional/nested-image interpreter, relative
  name, absolute execveat ignoring invalid dirfd, fork/vfork/CLONE_VM, and 4096
  arguments. It checks exact argv/environment/AT_EXECFN, missing/malformed
  interpreter errors, permission/writer rejection, invalid/oversize arrays,
  CLOEXEC rollback and descriptor-script limits. Host procfs independently
  verifies private-fd cleanup before and after replacement, actual interpreter
  bytes, task reaping, and no named host outputs. Existing ELF suites pass.
- All **28** focused Recipe cases pass (24 managed, three entry guards, one
  library case). The new lifecycle writes and chmods a Bash shebang script,
  validates its exact $0 and arguments with spaces, checks produced bytes with
  custom is_built/is_installed, installs a named product, and directly runs a
  copied coreutils true wrapper. Machine results stay separate from diagnostics,
  and recipe source remains unchanged. The original true-wrapper probe now
  exits zero. Release build and strict Ginux Clippy pass. No Recipe unit tests
  were added or run; VFS implementation was unchanged in this slice.
- Scope is named execve/cwd-relative names and absolute or AT_FDCWD execveat.
  Direct owned interpreters and descriptor-named scripts remain EOPNOTSUPP;
  source/relative directory CLOEXEC returns ENOENT. Script arrays are bounded to
  4096 argv and 4096 environment entries. The first interpreter receives a
  lexical namespace check; nested image interpreter resolution still belongs
  to the kernel's image view. A complete image/overlay path resolver, symlink
  semantics and owned interpreter chains remain unfinished. Source compilation
  has not been demonstrated. Evidence, commands and exact identities are in
  `.artifacts/ginux-local-integration/owned-scripts/`.

Descriptor-script continuation 2026-11-05: Ginux **a08aff7a** and Recipe
**bf41b5c** complete owned descriptor-named scripts through the common owners:

- Owned execveat supports open/unlinked files and owned directory-relative names,
  including renamed directories. Script argv and AT_EXECFN use public
  /dev/fd/N[/path] names. CLOEXEC sources/directories still fail before replacement.
- Readonly open/openat resolves those names through the public table into the
  existing filesystem. Reopening creates an independent offset/status description
  and retains unlinked bytes. Numeric overflow/wraparound, missing/malformed
  descriptors, directory escapes, writes and native/device/pipe handle imports
  are rejected. There is no native/private-fd path fallback. This is readonly
  opening, not general /dev/fd metadata, symlink or procfs support.
- All twelve Ginux launch suites pass. The script family now has fifteen modes,
  covering descriptor/unlinked execution, directory rename, fork/vfork inheritance
  and previous named cases. Host fd inspection still proves private cleanup.
  Twelve VFS unit cases pass, including deleted-object reopening, independent
  offsets, replacement identity, readonly enforcement and directory cursors.
- All **29** focused Recipe cases pass (25 managed, three entry guards, one
  library). The added real Bash case sources an inherited owned descriptor and
  validates captured text and produced bytes. This proves real Bash reopening;
  the native fixture separately proves execveat replacement and unlinked lifetime.
  Release build and strict Ginux Clippy pass. No Recipe unit tests were run.
- Two exploratory probes remain failures, preserved under
  `.artifacts/ginux-local-integration/descriptor-scripts/`: Python stops during
  initialization because the prepared image lacks encodings; it never reaches
  fexecve. Bash attempting /usr/bin/rm before sourcing reports fork ENOSYS.
  Supervisor-only ptrace confirms that Bash requests clone flags 0x1200011
  (CHILD_SETTID | CHILD_CLEARTID | SIGCHLD), rejected by the current process
  owner's exact supported shapes. The successful readonly-source case does not
  prove this ordinary external-command workflow. No host bypass was used.

Libc-fork continuation 2026-11-05: Ginux **9279e71b** fixes the traced fork
and signal-return gaps; Recipe **650f7a3** strengthens the retained CLI case:

- The process owner accepts libc fork's exact 0x1200011 shape. It suppresses
  kernel CHILD_SETTID copyout, publishes the virtual child TID into the child's
  private address space at the confirmed birth barrier, and restores original
  argument registers. CHILD_CLEARTID registration remains native. Invalid and
  readonly child-TID copyout is ignored without forced writes, matching the
  native child-copy convention; failed forks release reserved state.
- With fork enabled, the original Bash probe reached its child and then crashed
  after SIGCHLD. Supervisor-only tracing proved rt_sigreturn was denied with
  ENOSYS. The balanced task policy now mediates native rt_sigreturn, preserving
  the kernel-restored signal frame. The native process fixture covers virtual
  child TID before user execution, invalid/readonly destinations, resource-limit
  rollback, SIGCHLD sender identity, a handler syscall and return to the parent.
- The actual Recipe/Bash workflow now passes: inherit a file descriptor, fork
  the image's rm helper, delete the named script, return from SIGCHLD, then source
  the unlinked script through the public descriptor. The retained case checks
  deletion, exact captured text, product bytes, source immutability and separate
  machine output. This supersedes the earlier ENOSYS failure; the Python image's
  missing encodings remains a separate limitation.
- All **29** focused Recipe CLI/library cases and **twelve** launch suites pass
  with the final binary. The process family includes the new libc and handler
  cases plus root/failure descendant teardown. All **26** focused Ginux unit
  cases pass (core 5, process 9, VFS 12); no Recipe unit tests were run. Release
  build and strict Ginux Clippy pass. Before-sigreturn failure traces, final
  passing probe, full launch/Recipe logs and exact source/binary/image identities
  are under `.artifacts/ginux-local-integration/libc-fork/`.

Next local action: finish owned interpreter chains through the existing exec
owner, preserving script paths, shebang optional arguments, argv/environment,
CLOEXEC, executable lifetime, rollback and fork/vfork behavior. Then implement
guest file mappings through filesystem/memory owners. The remaining full input,
dependency, target/options, resource, durable state, publication and migration
scope is unchanged. General signal restart/job control, robust futex owner death,
image/overlay path resolution and ordinary source builds remain incomplete.
S0/S1/S2 remain in progress. Upstream review/PR creation belongs to the linked
review thread; all commits remain local inputs. No push or PR was performed.

Recovery checkpoint: active skill `hdae-development`, discovery path
`/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux/.agents/skills/hdae-development/SKILL.md`;
reload it and required references after context shifts. This integration task
record and the Recipe simplification plan govern the current authorized work.
Ginux's upstream Gate 8.5 checkpoint is historical context, not this integration's
next task. Git in the attached submodule worktree inherits a relative
`core.worktree` that resolves incorrectly; use explicit `--git-dir` and
`--work-tree` arguments there, without modifying the common repository config.

Recipe direction updated 2026-11-05: Ginux is required for every operation that
evaluates recipe code; macOS support is deferred. The
[Recipe simplification plan](recipe-simplification-implementation-plan.md) owns
this change (S0), custom readiness preservation, common execution, and separate
runtime-state storage (S3). Completed task evidence below describes the behavior
at that time; the host-default behavior originally delivered by R04 is superseded
by S0. This update changes the target plan, not the currently running code.

## Objective

Make Ginux the required execution environment for Recipe. A real recipe must acquire,
build, install, and execute software inside a Ginux-managed Linux environment.
Recipe must retrieve usable outputs and accurately handle state, failures,
cleanup, dependencies, cancellation, and retries.

The full recipe execution boundary includes top-level Rhai evaluation, imported
modules and base recipes, lifecycle checks, dependency recipes, native Rust
helpers, child commands, and cleanup. Wrapping only shell commands is insufficient.

The first demonstration builds and installs `recfstab` from local source using
the existing `AcornOS/deps/recfstab.rhai`, then runs the installed executable in
the domain. Its local-source discovery is not evidence of HTTP acquisition.
Later demonstrations cover mediated acquisition, dependencies, and recovery.

The human objective is a working introduction to Greg Kurtzer: “I integrated
Ginux into Recipe; here is Recipe, here is software it built using Ginux, and
here are general Ginux improvements that made it possible.” Building the
integration and opening useful upstream Ginux PRs are both deliverables.

Broader OS composition, candidate-system previews, A/B boot integration, and
installed application domains are future consumers, not implementation scope.

## Repository ownership and constraints

| Owner | Responsibility |
| --- | --- |
| `tools/recipe` / `LevitateOS/recipe` | Required Ginux coordination, worker, recipe semantics, dependency resolution, locks, checkpoint policy, separate runtime-state persistence, retries, output publication |
| `tools/ginux` / `LevitateOS/ginux` | Generic workload launch, Linux compatibility, domain-owned files/processes/devices, storage, channels, contracts, resources, lifecycle |
| Ginux upstream / `gmkurtzer/ginux` | Review and acceptance of general Ginux contributions |
| Existing recipe files | Package/source facts and acquisition/build/install behavior |
| LevitateOS orchestration | Explicit environment/artifact production, output locations, integration entry points |

Ginux and Recipe are sibling submodules. Recipe invokes a configurable executable;
it does not depend on Ginux being nested in its source tree. Ginux must never
understand `.rhai`, `ctx`, package dependencies, or Recipe phase names.

Follow the current repository `AGENTS.md` instructions. Preserve unrelated edits
and upstream tests/CI. Keep LevitateOS-specific policy out of Ginux contributions.
Use `.artifacts` for current outputs/tooling; do not build kernels. Select
compilation and focused CLI/integration/runtime checks appropriate to the change.
No mandatory local commit pipeline is introduced by this plan.

Existing Ginux historical gate records are lineage, not a new implementation
schedule imposed on Recipe. Follow applicable instructions if explicitly using
its HDAE workflow; do not rewrite accepted historical records. This plan does not
authorize merging upstream PRs or contacting people outside the PR workflow.

## Established upstream direction

Checked against upstream on 2026-11-05:

- [Project intent](https://github.com/gmkurtzer/ginux/blob/main/hdae/PROJECT.md):
  configurable userspace kernel; domain-owned state and resources; explicit
  syscall decisions; managed Linux workloads.
- [Architecture](https://github.com/gmkurtzer/ginux/blob/main/hdae/ARCHITECTURE.md):
  native ptrace execution, immutable OCI root, writable domain overlay, mediated
  devices/networking, no unchecked host effects or exposed host authority.
- [Systemd milestone](https://github.com/gmkurtzer/ginux/blob/main/hdae/gates-todo/9-systemd-baseline.md):
  integrate existing execution/VFS/process/device models; managed login shell;
  bounded DNF repository transaction; flush writable state during shutdown.
- [Contract and audit](https://github.com/gmkurtzer/ginux/blob/main/hdae/gates-todo/10-configurable-contract-audit.md):
  selected validated profiles, bounded audit, fail-closed unknown operations.

Configured execution, connected processes, writable storage, resource ownership,
and reliable teardown align with these goals. A general import/export API is a
new interface proposal, not a previously approved upstream design. Networking
must start with a bounded mediated repository/download path, not unrestricted
outbound connectivity. Preserve the packet-mediation model; a direct socket
passthrough shortcut is not an equivalent implementation.

Check actual upstream changes before implementing each affected area and reuse
capabilities that have landed. Do not duplicate upstream systemd work merely
because a task was written against an older checkout. Architectural alignment
does not guarantee acceptance of a particular implementation.

## Initial source investigation

These findings describe the starting point. Task records below contain later
implementation evidence; re-read current source before acting on a finding.

- `tools/recipe/src/lib.rs`: `RecipeEngine` exposes install/remove/cleanup/check
  entry points and currently runs Rhai in process.
- `tools/recipe/src/core/executor/private/flow.rs`: top-level Rhai runs before
  checks; dependencies and phases use the same engine; persistence occurs after
  successful phases and cleanup.
- `tools/recipe/src/core/executor/private/state.rs`: authoritative `ctx` writes
  atomically replace the main recipe or its base recipe.
- `tools/recipe/src/core/build_deps.rs`: dependencies run in process and install
  tools into `.tools`.
- `tools/recipe/src/helpers/util/shell.rs`: ordinary child commands plus threads
  for output draining; interpreter limits do not constrain these operations.
- `tools/ginux/crates/ginux-init/src/main.rs`: separate native Bash, process,
  filesystem, and device demonstrations. The native launcher hardcodes its
  command/environment, writes diagnostics to stdout, and does not preserve
  arbitrary workload exit status as a public execution result.
- `tools/ginux/crates/ginux-core/src/lib.rs`: current balanced contract rejects
  mutating opens and lacks the general process/filesystem surface needed here.
- `tools/ginux/crates/ginux-vfs/src/lib.rs`: bounded in-memory file overlay;
  teardown clears it; native workload integration, fuller filesystem operations,
  durable storage, and export need work.
- `tools/ginux/crates/ginux-process/src/lib.rs`: existing virtual process/thread
  model should be connected, not replaced with a parallel implementation.
- `tools/ginux/crates/ginux-devices/src/lib.rs`: isolated packet queue, not an
  externally connected network backend.

These are source findings, not runtime evidence that Recipe or Cargo works under
Ginux. Additional compatibility requirements must come from actual execution.

## Intended architecture and interface decisions

### Execution

- Require Ginux at `RecipeEngine`, covering CLI and library consumers. Remove
  public host/in-process selection, host defaults, and fallback under Recipe S0.
  Missing Ginux or unsupported capabilities fail explicitly before recipe effects.
- Use one domain per invocation. Dependencies share that domain initially.
- Reuse the existing lifecycle inside a worker. Do not duplicate orchestration.
- Worker mode interprets Rhai inside its domain and cannot recursively invoke
  Ginux. Internal worker dispatch is not a public host-execution bypass.
- Host coordinator owns the authoritative lock, runtime-state persistence, process
  supervision, result publication, and optional outer retry loop.
- All operations evaluating recipe code run inside Ginux, including
  manual checks, cleanup, and removal. Static metadata parsing/hash operations
  need not create a domain.
- Preserve independent custom is_acquired/is_built/is_installed rules. Ginux
  confines their execution; Recipe interprets readiness and errors. Successful
  process exit, retained state, or file existence alone cannot override a
  recipe's custom not-ready result. Recipe S4 owns the explicit outcome migration.

### Ginux launch and communication

- Keep a subprocess boundary between Recipe and Ginux. No Ginux Rust dependency
  is needed in Recipe for the initial integration.
- Define a small versioned launch/result surface using Ginux-owned configuration
  and channels. Prefer extending existing configuration and execution paths.
- Support executable, arguments, environment, working directory, assigned storage,
  selected contract, and supported resource limits.
- Separate workload stdout/stderr, supervisor diagnostics/audit, and machine
  control traffic. Never parse human output as authoritative results.
- Recipe's worker messages are opaque bytes to Ginux. Managed endpoints are
  domain objects; raw host descriptors are not handed to the workload.
- Invalid configuration and unsupported capabilities fail explicitly. No policy
  auto-widening, unchecked syscall continuation, or writable host bind mounts.
- Resolve the supported privilege/bootstrap mechanism in Ginux. The existing
  privileged chroot demonstration does not justify running Recipe broadly as root.

### Storage and paths

- Prepared immutable environment: Rocky-compatible x86-64 userspace, Recipe and
  runtime libraries, Rust toolchain/linker, basic tools, CA material where needed.
- Explicitly import recipe/base/module/assets, source closure, and prepared caches.
  Copy declared inputs, not arbitrary home directories or secret-bearing caches.
- Stable domain paths identify recipe, build, tools, temporary, and install
  content across retries. Host `.artifacts` paths must not leak into persisted
  worker state as essential execution paths.
- Writable state uses Ginux-owned storage with bounded capacity. Preserve needed
  file metadata, symlinks, deletions, and execution of generated binaries.
- Use retained state plus a bounded consistent snapshot/flush operation. It must
  make files durable before Recipe acknowledges a matching state checkpoint;
  do not build a live-migration subsystem to achieve this.
- Export only selected outputs to a fresh confined host staging directory.
  Publish the completed declared artifact tree through Recipe's coordinator;
  never apply an arbitrary domain path to the host root.
- Package source/version/build facts remain in recipes. Environment production
  is explicit, and ordinary execution consumes existing artifacts.
- Bootstrap Recipe/Ginux executables and the initial environment through explicit
  producers. If an environment producer invokes a recipe, it needs an existing
  prepared Ginux environment; do not resolve that dependency with a host recipe
  execution fallback. Building the Rust executables is separate from evaluating
  recipe code.

### Recipe checkpoint protocol

Use a small framed, versioned request/event/acknowledgement protocol with bounded
messages. It carries invocation identity, sequence, operation, successful `ctx`
checkpoints, structured failure, and final result. Keep it separate from existing
observability events. Validate frame lengths, version, sequence, operation, and
termination state. Managed messages cannot name arbitrary host write paths.

At an existing persistence point: worker reaches the boundary; the coordinator
obtains a consistent durable storage checkpoint; host persists the corresponding
runtime state in a separate record; then host acknowledges. Order checkpoint
operations so interruption yields either the previous acknowledged pair or a
recoverable newer pair. Record the
association in invocation-local state, not a new global package database.

The coordinator derives the state destination from trusted invocation identity.
Main/base recipe sources are immutable inputs. Reuse atomic durable writes for
the state record; Recipe S3 defines state identity, loading, migration, and the
compatibility alias for `--no-persist-ctx`. Preserve useful successful-check and
cleanup state updates. Update worker and coordinator together with a transport
version bump for incompatible payload changes; the existing v1 contract records
the already implemented format. Installation output is published only after
successful final completion; recovery data and logs may be retained on failure.

Do not claim sandboxing makes recipe-reported state or package content truthful.
The boundary confines effects; real output/runtime checks establish behavior.

## How to dispatch tasks to GPT-6 Luna or similar agents

Each task below is intended as one bounded agent assignment. Give an agent its
task text, this plan, and its prerequisites' concrete interfaces. Expected work
is a small cohesive change, usually one owner and a few files. If a task uncovers
a subsystem or many unrelated syscall gaps, split out named follow-ups rather
than asking the same agent to “make Cargo work” indefinitely.

Task completion means the stated concrete result exists, relevant compilation or
focused runtime evidence is recorded, and failures/limitations are reported.
Checks are appropriate to the changed behavior; do not run a full suite for each
task. Documentation-only tasks do not require compilation. A failed experiment
is evidence, not a completed working capability.

Multiple agents can run only tasks whose dependencies are complete and whose
write ownership does not overlap. Dependency IDs are prerequisites, not numeric
test gates. A coordinator integrates changes in dependency order. Preserve other
agents' edits and do not reset shared working trees.

### Parallel ownership

| Lane | Owns | Shared files handled by coordinator |
| --- | --- | --- |
| G-launch | Native launch/options, generic result/stream adapters | `ginux-init/src/main.rs`, crate manifests |
| G-process | Native task supervision and existing process model integration | Central syscall/event dispatcher |
| G-files | Existing VFS operations, native descriptor/filesystem adapter | Central syscall dispatcher, shared object table |
| G-storage | Input packaging, retained backing, checkpoint/export machinery | VFS backend hooks and configuration |
| R-worker | Worker protocol, lifecycle checkpoint transport | `recipe/src/lib.rs`, executor persistence seam |
| R-host | Coordinator, host persistence, recovery, output publication | Public API and CLI entry points |
| Integration | Environment producer, recipe adaptation, named real executions | Top-level wrappers and manifests |
| G-network | Scoped connectivity and socket adapter | Dispatcher and generic configuration |

Small shared-entry-point edits belong to one coordinator task after the relevant
modules are available. Do not let process, filesystem, and networking agents all
edit the current monolithic Ginux entry point simultaneously. Extract only the
native paths required to give those owners separate modules; avoid a repository
reorganization. Proposed module names below are reservations, not requirements
to introduce new crates or parallel behavior owners.

## Task list in dependency order

### Shared foundation

- [x] **F01 — Reconcile the checkout with current upstream.** Depends: none. (Done 2026-11-05: `LevitateOS/ginux` main == `gmkurtzer/ginux` main at `d337713b`, userspace-transition merge; no drift. Plan code evidence stale on `ginux-init` shape — now a 2381-line `no_std` ptrace supervisor — but deltas hold: fixture exit-status=23 rules, hardcoded `/usr/bin/bash` ELF path, balanced contract still rejects mutating opens. Prohibited `#[test]` suites in 8 ginux crates + `make test-unit` must not run; policy-only removal stays out of feature PRs.)
  Owner: coordinator; read Ginux upstream and fork history/status. Identify which
  capabilities already exist or are in active upstream work; adjust only affected
  tasks. Keep accepted history and unrelated local changes. Done: actual bases
  and remaining implementation deltas are known; no duplicate feature work.

- [x] **F02 — Specify the minimal cross-process interfaces.** Depends: F01. (Done 2026-11-05: `docs/ginux-recipe-interfaces.md` fixes protocol v1 framing/limits, stream/result split with supervisor result file, stable domain paths `/inputs /work /out`, checkpoint ack order, allowlisted env, `RECIPE_WORKER` recursion guard, and open gaps per lane.)
  Owner: coordinator; Ginux launch/storage/channel contract and Recipe worker
  protocol. Record concrete fields, endpoint ownership, size/version limits,
  failure categories, stable paths, checkpoint acknowledgement, and bootstrap
  behavior in the relevant repository's existing documentation. Done: agents can
  implement either side without guessing. This is interface work, not an approval
  gate or permission request.

- [x] **F03 — Isolate native runtime extension points.** Depends: F02. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/f03-entry-points`, commit `7c9e2fd1`. `main.rs` −500 lines; `dispatch`/`supervisor`/`launch` + declining `process/fs/channel_hooks`. Release check clean, cross-target build (from `tools/ginux`, project cargo config) produces `ginux`; `config` demo exit 0. Note: dev-profile `cargo check` fails pre-existing on `no_std` unwind; cross-target build must run from `tools/ginux` for its `.cargo/config.toml` link args.)
  Owner: G-launch/coordinator; Ginux entry point and a small native runtime module.
  Move only the relevant launcher/supervision plumbing into owned modules and
  establish dispatch hooks for process, filesystem, and channels. Preserve
  existing demonstrations. Done: binary compiles; later lanes have disjoint files
  and one shared dispatch-wiring owner.

- [x] **F04 — Expose Recipe's persistence and lock seams.** Depends: F02. (Done 2026-11-05: `ExecutionContext` seam in `tools/recipe` branch `ginux-recipe/f04-persist-seams`, commit `25a9ff8`, pushed. Host path byte-identical across fresh install/resume/check/remove CLI runs; `worker_stub` exposes lock-free callback checkpoints for R02, compile-verified.)
  Owner: R-worker; existing executor `attempt.rs`, `state.rs`, `flow.rs`, and
  `actions.rs`. Introduce the smallest internal execution context needed for
  host-owned locking and checkpoint persistence; keep normal host behavior.
  Done: existing real CLI install/resume behavior still works; workers can report
  checkpoints without rewriting authoritative source or acquiring a second lock.

### Ginux launch and process lane

- [x] **G01 — Launch the configured native executable.** Depends: F03. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g01-launch`, commit `19e1037d`. `launch` cmd validates id/root/init/args/env/workdir vs §2 limits; new optional `[domain] workdir` (default `/work`). Coordinator verified with sudo: echo args ✓, env var ✓, relative-path `cat` in workdir ✓, 129B arg rejected exit 2, unprivileged bootstrap fails explicitly exit 2. LIMITS: getcwd denied (G06a); launcher exits 0 regardless of workload code, no result file yet (G02).)
  Owner: G-launch; launch/config modules. Replace fixed Bash-only launch behavior
  for the new general path with validated executable/args/environment/working
  directory and explicit bootstrap failure. Done: ordinary programs receive the
  configured values; existing native demo remains callable.

- [x] **G02 — Return faithful workload completion and streams.** Depends: G01. (Done 2026-11-05: commit `d1e14bb1` on `ginux-recipe/g01-launch`. `--result-file` (O_EXCL+fsync) is authoritative; exit mirrors workload code (139 = 128+11 for signals), startup = 2, supervisor = 1; audit capped 64 KiB; workload stdout byte-clean. Coordinator verified with sudo: exit 3 → launcher exit 3 + `exited/code:3`; SIGSEGV → exit 139 + `signaled/signal:11`; invalid TOML → exit 2 + `supervisor-error`. `generation` always null until G12a.)
  Owner: G-launch. Preserve nonzero exits and signal termination; distinguish
  supervisor failure; separate diagnostics from workload stdout/stderr. Bound
  audit output. Done: a real subprocess caller can distinguish each result and
  capture clean application output.

- [x] **G03 — Connect native fork to managed child ownership.** Depends: F03. (Done 2026-11-05: worktree `/tmp/ginux-g03`, branch `ginux-recipe/g03-fork`, commit `724dd3ae`. Fixture exit-23/32 rules replaced by `ProcessTable::wait` translation with status-word poke-back; pid translation hides host pids. Coordinator rebuilt + reran supervised fork reproducer: virtual pids 2,3, real statuses 7+13, cleanup PASS, no survivors. Follow-up: wire fork events into the G01 `launch_main` loop at merge time; prohibited `#[cfg(test)]` in ginux-process/core left for separate policy cleanup.)
  Owner: G-process; existing process model and native process adapter. Connect
  fork to the general supervisor, replacing fixture-specific completion rules
  with ordinary parent/child ownership. Done: a real fork workload returns child
  status without exposing host process identities or leaving an untraced child.

- [x] **G03a — Support libc process-creation variants.** Depends: G03. (Done 2026-11-05: worktree `/tmp/ginux-g03`, branch `ginux-recipe/g03a-variants`, commit `668a86f2`. clone/fork/vfork claimed; CLONE_THREAD + bare CLONE_VM denied ENOSYS; clone3 untouched so glibc falls back; VFORK/CLONE events translated at creation exit stop. Variant selection measured via strace/ptracer ground truth (glibc 2.39). Coordinator reran supervised reproducer: `VARIANT_REPRO: PASS`, GATE7 PASS, no survivors.)
  Owner: G-process. Add the vfork/clone process-creation variants required by the
  selected libc, sharing the existing managed-task path. Keep clone-thread work
  for G04. Done: a normal libc subprocess launcher creates a managed child; each
  unsupported variant fails explicitly. Split clone3 into its own follow-up if
  required rather than bundling another ABI into this assignment.

- [x] **G03b — Supervise executable replacement.** Depends: G03a. (Done 2026-11-05: worktree `/tmp/ginux-g03`, branch `ginux-recipe/g03b-exec`, commit `bef09013`, pushed to fork. `process_hooks::exec_mediated` claims execve(59)/execveat(322): plain execve and execveat(AT_FDCWD,path,argv,envp,0) execute under supervision, other shapes denied ENOSYS; TRACEEXEC + PTRACE_EVENT_EXEC handler preserves virtual pid with exec-epoch bump. Coordinator reran supervised reproducer: SHELL_REPRO PASS, GATE7 PASS, no survivors. LIMIT: wiring into the G01 `launch_main` loop pending at merge time; exec-target path allowlisting deferred to G05a/G07.)
  Owner: G-process. Handle execve and a required execveat variant, controlled
  loader transitions, and task identity across exec. Done: a managed shell starts
  another executable and its caller observes the real result; executable
  replacement never detaches syscall supervision.

- [x] **G04 — Connect native threads and synchronization.** Depends: G03b. (Done 2026-11-05: worktree `/tmp/ginux-g04`, branch `ginux-recipe/g04-threads`, commit `19bf68c9`, pushed to fork. Supported CLONE_THREAD shape (VM+SIGHAND) executes via existing `ProcessTable::clone_thread`; clone return/parent-tid-cell/gettid translate to virtual tid, getpid reports tgid; arch_prctl/futex/robust/rseq execute on host; thread exit drops host slot, zero-root-exit drains live threads. Other CLONE_THREAD shapes, bare CLONE_VM, clone3 denied ENOSYS. Coordinator reran reproducer: THREAD_REPRO PASS (2 threads, join + detach, 7 denials), GATE7 PASS, exec regression PASS, no survivors. LIMIT: G03a step-5 denial assertion superseded — coordinator to update that reproducer; real glibc-pthread coverage needs mmap-family mediation, still denied.)
  Owner: G-process. Handle required thread creation, thread identity, futex/TLS,
  and thread exit using existing models. Implement only the concrete required
  behavior in bounded patches. Done: a real threaded program completes; Recipe's
  output-draining thread pattern is supported without weakening mediation.

- [x] **G05 — Complete child waits and normal signal delivery.** Depends: G04. (Done 2026-11-05: worktree `/tmp/ginux-g05`, branch `ginux-recipe/g05-waits`, commit `4655aaa2`, pushed to fork. `TaskState::Signaled` + Linux wait-word `ChildStatus` + `WaitTarget` in ginux-process; wait4 fully emulated (blocking park, WNOHANG→0, EINVAL/ECHILD); kill forwards SIGTERM/SIGINT/SIGKILL + signal-0 probe, others denied; SIGCHLD queued+delivered per child death; signaled-but-unreaped renders zombie. Coordinator reran reproducer: WAIT_REPRO PASS (exits 42,3; signals 15,9), GATE7 PASS, G03/G03b/G04 regressions PASS, no survivors. LIMIT: in-tracee SIGCHLD handler observation needs rt_sigaction mediation; job-control stops unmediated (WUNTRACED/WCONTINUED accepted but never produced; SIGSTOP/SIGCONT denied). Follow-ups: (1) rt_sigaction + handler delivery, (2) job-control mediation.)
  Owner: G-process. Cover child wait results, SIGCHLD, and normal forwarded
  termination signals without relying on fixture-specific statuses. Done: parent
  and child observe real completion and cancellation with no escaped task.

### Ginux filesystem and storage lanes

- [x] **G06 — Implement domain directory and working-directory operations.** (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g06-dirs`, commit `dc65b412`. VFS `mkdir`/`list_dir`/merged-view `stat`; `fs_hooks` claims mkdir/mkdirat/getdents64 as explicit `-EPERM` (getdents64 was previously host-mediated — leak closed). Coordinator reran VFS reproducer: `G06_REPRO: PASS` incl. traversal/symlink/loop denials. LIMIT: supervised-path mkdir/getdents emulation deferred — needs G06a/G07 tracee-memory/fd plumbing; also removed a prohibited `#[cfg(test)]` suite.)
  Depends: F03. Owner: G-files; existing VFS and native filesystem adapter.
  Add directory creation and enumeration needed by ordinary tools. Done: managed
  code creates and lists a nested working tree;
  path traversal and symlinks cannot acquire unrelated host authority.

- [x] **G06a — Connect working directories and relative path resolution.**
  Depends: G06. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g06a-cwd`, commit `808c1d16`, fs_hooks.rs only. `DomainCwd` + `resolve_at` proven 46/46 against real VFS via symlinked-source reproducer — coordinator reran: PASS; release check clean. LIMIT: chdir/getcwd/fchdir still deny at the claim layer; supervised emulation needs tracee-memory/fd plumbing + branch merge with g01-launch (G07). Module carries documented `#[allow(dead_code)]` until G07 consumes the API.) Owner: G-files. Implement chdir/getcwd and the shared relative-path
  resolution needed by subsequent filesystem operations. Done: a real process
  changes directory and accesses relative paths correctly inside the domain;
  parent components and symlinks remain confined.

- [x] **G07 — Connect native open and descriptor lifetime to domain objects.**
  Depends: G06a. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g07-open`, commit `ca64fddf`. `open_at`/`close_fd` + bounded fd table consuming G06a resolution; open/openat/close claimed (host-mediation leak closed); G06a `allow(dead_code)` removed. Coordinator reran source-verbatim reproducer: PASS (open/read/close, openat-cwd, symlink-final, denied-write base-intact, overlay copy-up, O_EXCL, close-unknown, fd exhaustion, dirfd rules). LIMIT: supervised emulation still needs tracee-memory/fd plumbing + merge with g01-launch; fchdir/non-AT_FDCWD dirfds stay denied.) Owner: G-files. Cover open/openat/close, access flags, bounded
  descriptor allocation, and immutable-image versus writable backing selection.
  Done: ordinary native opens resolve to the intended domain object and denied
  writes cannot mutate the immutable base.

- [x] **G07a — Connect file data I/O and offsets.** Depends: G07. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g07a-io`, commit `0042dcaa`. `read`/`write`/`lseek` claimed for domain fds (stdio passthrough); pure-core `read_fd`/`write_fd`/`lseek_fd`; VFS seek/short-writes/sparse-zero-fill/O_APPEND. Coordinator reran source-verbatim reproducer: 36/36 PASS, base blob unchanged. LIMIT: supervised tracee-memory/fd wiring still pending (coordinator follow-up with launch merge).)
  Owner: G-files. Add the read/write/seek behavior required by the selected
  workload, including short I/O and bounded transfers. Done: a native program
  writes and rereads real data with correct offsets and errors.

- [x] **G07b — Connect file metadata queries.** Depends: G07a. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g07b-stat`, commit `2eac10bf`. stat/fstat/lstat/newfstatat emulations over merged view (`stat_at`/`lstat_at`/`fstat_fd`/`statat`, both `struct stat` layouts byte-verified vs raw syscalls; strace showed glibc emits only newfstatat/fstat). Coordinator reran source-verbatim reproducer: ALL PASSED. statx/AT_EMPTY_PATH/readlink left as observed follow-ups; supervised write-back still pending.)
  Owner: G-files. Implement the stat-family variants actually required by libc
  and distinguish files, directories, links, modes, and sizes correctly. Done:
  normal filesystem tools inspect domain data accurately without host metadata
  leakage. Split additional metadata ABIs into observed follow-ups.

- [x] **G05a — Connect inherited pipes and close-on-exec descriptors.** Depends:
  G05, G07b. (Done 2026-11-05: worktree `/tmp/ginux-g05a`, branch `ginux-recipe/g05a-pipes`, commit `e97eb58c`, pushed to fork. `PipeSpace`/`FdEntry`/`FdKind` domain objects in ginux-process (also removed a prohibited `#[cfg(test)]` suite in the touched file); host executes pipe/dup/close/fcntl while the loop mirrors outcomes; pipe2 allows only O_CLOEXEC|O_NONBLOCK, dup3 only O_CLOEXEC, other fcntls ENOSYS; CLOEXEC scrubbed at exec + late paths; teardown audits live-pipes. Coordinator reran reproducer: PIPE_REPRO PASS (46 checks), live-pipes=0, GATE7 PASS, G03/G03b/G04/G05 regressions PASS, no survivors. LIMIT: merge-time convergence with the G07 fd table recorded; real-libc shell pipeline needs G06a/G07 file/memory mediation, so acceptance used raw static stages.)
  Owner: G-process/descriptor adapter; serialize with G-files changes.
  Implement duplication/shared-offset and pipe inheritance through domain objects.
  Done: a real shell pipeline and an exec with close-on-exec descriptors behave
  correctly without receiving supervisor descriptors.

- [x] **G08 — Add rename and removal behavior.** Depends: G07b. (Done 2026-11-05: worktree `/tmp/ginux-g08`, branch `ginux-recipe/g08-rename`, commit `d4ecd78`. Pure-core `rename_at`/`unlink_at` + VFS `rename` (atomic replace, nofollow, tombstones; fixed nofollow gap + tombstone-shadows-live bug). Coordinator reran source-verbatim reproducer: 70/70 PASS, base blob immutable. LIMIT: claims still deny at the entry layer; supervised wiring + `rmdir` pending.)
  Owner: G-files. Implement the rename/unlink operations required for temporary
  files, compiler output, and cleanup. Keep atomic rename semantics and explicit
  unsupported cases. Done: real tools replace and remove domain files correctly;
  denied mutations have no host effect.

- [x] **G08a — Add installation permissions and link operations.** Depends: G08,
  G05a. (Done 2026-11-05: worktree `/tmp/ginux-g08`, branch `ginux-recipe/g08a-perms`, commit `80763e50`, pushed to fork. VFS chmod/chmod_fd (plain 0o777, setuid/sticky denied), link (in-domain copy, never replaces), symlink (uninterpreted target), readlink (nofollow, Linux truncation), overlay write-bit enforcement on writable open; pure-core chmod_at/fchmod_at/fchmod_fd/link_at/symlink_at/readlink_at; all 13 perm/link/chown syscalls claimed and denied EPERM at entry layer (closes readlink-89 host-mediation path); base never mutates. Coordinator reran reproducer: G08A 61/61 PASS, G08 baseline 70/70 PASS. LIMIT: supervised tracee-memory/fd wiring pending at merge; hardlinks are content copies (nlink 1); fchmodat/linkat flags 0 only; chown denied unemulated. Follow-ups: (1) merge-time supervisor wiring + per-task fd tables, (2) decide hardlink coherence vs copy semantics for installers.)
  Owner: G-files. Implement chmod and the link variants required by the
  selected tools, preserving path confinement and metadata. Done: ordinary tools
  install a usable executable tree and retain meaningful links and permissions.

- [x] **G09 — Execute files produced in writable domain storage.** Depends: G08a,
  G03b. (Done 2026-11-05: worktree `/tmp/ginux-g09` (base `ginux-recipe/integration`), branch `ginux-recipe/g09-exec-run`, commit `9e4a5fa5`, pushed to fork. Loop-global domain view (seeded base + overlay, staged via `GINUX_DOMAIN_STAGE`); exec-bit files materialize at domain path and load under supervised exec mediation (same pid, epoch bump); scripts stage a domain interpreter (bounded depth, ENOEXEC on bad #!); chmod(90)/fchmodat emulated via G08a path, host untouched; missing falls through to host byte-identical. Coordinator reran reproducer: G09_REPRO PASS (domain prog status 42, interp status 7, EACCES/ENOENT/EPERM faithful, revocations stick), wait+pipe regressions PASS, no survivors. LIMIT: loop-global view, shared cwd, 8×64KiB caps under staged prefix; real-libc interpreters need mmap mediation; per-task fd/cwd tables with G07 merge. Follow-ups: (1) rootfd-backed loading + per-task tables at G07 launch merge, (2) QuiesceGate hookup for staged files.)
  Owner: G-files with G-process interface fixed first. Connect generated
  executables and interpreters to controlled native loading; do not export and
  execute them on the host as a substitute. Done: a prepared compiler creates a
  small executable and Ginux runs that exact domain-produced file.

- [x] **G10 — Supply explicit immutable workload inputs.** Depends: F03. (Done 2026-11-05: `tools/ginux` branch `ginux-recipe/g10-inputs` (separate worktree `/tmp/ginux-g10`), commit `0e790d18` — `inputs`/`read` subcommands on `ginux-vfs-index`: selected files → ustar + `.ginux-index` at `/inputs/NAME` with sha/size/mode; dirs/symlinks/secrets refused; NAME ≤92, ≤256 inputs, ≤512 MiB/file. Coordinator verified: read-back sha matches pins; `/etc/passwd` read rejected exit 1; check clean. LIMIT: supervised `open` at `/inputs` needs the G07 launch merge; `/tmp/g10-pack` artifacts are throwaway — R05 re-runs packaging.)
  Owner: G-storage; existing image/index tooling. Extend existing machinery to
  incorporate selected input files and metadata into a workload view. Done: a
  generic program reads an imported file at a configured domain path; no host
  path or descriptor is disclosed as managed authority.

- [x] **G11 — Add retained writable backing.** Depends: G07a, G10. (Done 2026-11-05: worktree `/tmp/ginux-g11`, branch `ginux-recipe/g11-retained`, commit `a396ab14`. `retained.rs`: checkpoint/restore with validation, per-file/total/count bounds, instance-id isolation. Coordinator reran reproducer: PASS (stop/reopen identical, 4 exhaustion cases explicit, cross-instance rejected with nothing applied). Flush seam defined for G12: publish staged bytes + fsync + atomic rename; reserved header word becomes generation counter; restore reads only G12-published images.)
  Owner: G-storage; existing VFS backend hook and a retained-storage module.
  Replace demo-sized disposable backing for the selected workload with bounded
  retained storage. Preserve isolation between instances. Done: files survive a
  normal domain stop/reopen and storage exhaustion returns useful errors.

- [x] **G12 — Flush retained storage durably.** Depends: G11. (Done 2026-11-05: worktree `/tmp/ginux-g11`, branch `ginux-recipe/g12-flush`, commit `eb2589dd`. Header word = generation counter; `DurableStore` (write-temp/fsync/atomic-rename/dir-sync/read-back) + `flush_retained`/`recover_retained`; restore reads only published images; per-phase FlushError never reports failure as durable. Coordinator reran reproducer: all checks PASS incl. torn-write rejection and generation increments.)
  Owner: G-storage. Implement the backing store's explicit flush and recovery
  behavior with useful error propagation. Done: a completed flush survives
  reopen; incomplete/failed writes cannot be reported as durable.

- [x] **G12a — Checkpoint a consistent domain storage generation.** Depends:
  G12, G05a. (Done 2026-11-05: worktree `/tmp/ginux-g11`, branch `ginux-recipe/g12a-generation`, commit `4d036c07`, pushed to fork. Library-only: `QuiesceGate` trait seam (process lane implements, storage consumes), `quiesce_retained` (bounded polls, fails closed Busy, core does no sleeping), `checkpoint_generation` (quiesce-then-atomic-publish ack order), `verify_published`, `list_recoverable`; failures storage-category, never durable. Coordinator reran reproducer: all 17 checks PASS incl. torn-write rejection, previous-generation recovery, quiesce Busy/fail-closed paths. LIMIT: multi-generation history needs supervisor-rotated publish paths; cross-task settling needs the process-lane gate at launch merge. Follow-ups: (1) process-lane QuiesceGate + supervisor wiring at launch merge, (2) R07 ack transport recording {seq → generation}.)
  Owner: G-storage. Add bounded quiesce/flush or equivalent consistency
  behavior for all tasks sharing storage and identify recoverable generations.
  Done: interruption cannot present an acknowledged generation with missing
  matching files; inability to obtain consistency is an explicit error.

- [x] **G13 — Export selected results and deletions.** Depends: G12a, G08a.
  (Done 2026-11-05: worktree `/tmp/ginux-g13` (base `ginux-recipe/integration`), branch `ginux-recipe/g13-export`, commit `50c6a399`, pushed to fork. New `export.rs`: `export_generation` (verify → replace view → walk merged subtree into fresh staging; symlinks recorded never followed; absolute/escaping targets rejected per-file; tombstones under `.ginux-tombstones/`; max_files stops explicitly, oversize skipped per-file with full report); retained image gains symlink record kind (old images still validate). Coordinator reran reproducer: ALL G13 CHECKS PASSED (24 lines) + G12a 17/17 backward-compatible. LIMIT: child/depth caps 64; lexical symlink containment; publish only when failed==0 && !exhausted. Follow-ups: (1) wire into R08 publication adapter, (2) hardlink coherence decision.)
  Owner: G-storage. Export selected trees or changes into a fresh confined host
  staging destination with needed metadata/tombstones. Validate traversal, links,
  output bounds, and partial failure. Done: a generic file-transform workload's
  output can be retrieved and removed without modifying arbitrary host paths.

- [x] **G14 — Provide domain-owned bidirectional byte channels.** Depends: G07b,
  G05a. (Done 2026-11-05: worktree `/tmp/ginux-g14` (base `ginux-recipe/integration`), branch `ginux-recipe/g14-channels`, commit `a92471ae`, pushed to fork. New `channel_transport.rs`: supervisor-owned `ChannelSpace` (bounded 64 KiB/direction, short I/O, WouldBlock backpressure, half-close drain-then-EOF, full close with freeing, opaque bytes, zero host descriptors); `channel_hooks.rs` rewrite keeps declining + defines fd 0/1 convention and merge seam (`channel_managed_read/write/close` signatures recorded). Coordinator reran harness: G14_REPRO PASS (262218 bytes via real /bin/cat, 4 §4-shaped frames with op 99 proving opacity, byte-exact echo, clean EOF, no survivors). LIMIT: loop wiring pending (G09 merge; WouldBlock pump policy open). Follow-ups: (1) merge wiring + blocking policy, (2) aliasing coherence only if channels ever alias.)
  Owner: G-files/channel adapter, serialized with G-files edits. Supply the
  generic opaque transport agreed in F02, bounded I/O and clean closure. Done:
  an ordinary managed program exchanges bytes with its invoking controller;
  Ginux does not parse Recipe messages or expose raw controller descriptors.

### Recipe worker and coordinator lanes

- [x] **R01 — Implement bounded worker protocol types and framing.** Depends: F02. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r01-protocol`, commit `0b7875b`. `src/worker/protocol.rs` + `worker_protocol_demo` example; `cargo check -p levitate-recipe` clean; real socketpair demo shows request→checkpoint→ack→result plus version/oversize/truncated/replay rejections. Framing fix: F02 §4 corrected to payload = total−14.)
  Owner: R-worker; new small protocol module inside Recipe. Encode/decode the
  agreed requests, checkpoints, acknowledgement, failure, and final result.
  Done: real sender/receiver processes exchange frames and reject malformed or
  truncated traffic. Do not build an in-process assertion suite.

- [x] **R02 — Add a worker entry point using the existing engine.** Depends: R01,
  F04. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r02-worker`, commit `80e45df`. `src/worker/run.rs` dispatches install/remove/cleanup/checks via `*_with_context` under `worker_stub`; `RECIPE_WORKER=1` forces in-process backend. Coordinator independently reran the pipe driver from clean fixture state: 6 checkpoints acquire→cleanup→build→cleanup→install→cleanup, each acked, `result ok=True`, exit 0, sources unchanged under coordinator-held lock. Known: all engine errors surface `fatal=True`/`reason=workload` — R03 refinement.) Owner: R-worker; worker module plus coordinator-owned binary wiring.
  Dispatch install/remove/cleanup/check requests through the existing engine;
  disable recursive domain launch and authoritative locking/persistence.
  Done: an actual worker process executes a real local recipe through the
  protocol. This exercises protocol/lifecycle behavior, not Ginux confinement.

- [x] **R03 — Report existing lifecycle checkpoints and structured failures.**
  Depends: R02. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r03-checkpoints`, commit `e957651`. `CheckpointPoint{phase,reason}` seam; `InstallAttemptError` no longer flattened; worker maps Fatal→`fatal:true`, Phase→`fatal:false` + real phase/reason. Coordinator reran driver: 6 structured checkpoints + ok:true; failing build → `fatal:false phase:build`; missing-cleanup → `fatal:true`. Residual: remove/cleanup hook failures still `fatal:true` — possible R11 follow-up.) Owner: R-worker; persistence seam and worker adapter. Preserve
  current successful-check/phase/cleanup semantics, wait for acknowledgements,
  and report fatal versus phase failures. Done: the controller observes the
  actual checkpoint stream, including a failed phase, without scraping logs.

- [x] **R04 — Implement explicit backend selection for library and CLI callers.**
  Depends: F04, R01. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r04-backend`, commit `61a742b`. `Backend::select` (flag→env→host default) is the one choke point; all six executable commands route through it. Coordinator verified: default host check unchanged exit 0; `--backend ginux` with missing launcher → `backend startup failure ... refusing host fallback` exit 1; `RECIPE_WORKER=1` forces in-process with a stderr note.) Owner: R-host/coordinator; `src/lib.rs`, CLI and commands.
  Add executable/environment selection and route all executable recipe operations
  through one backend choice. Done: CLI and library callers choose the same
  behavior; missing Ginux fails explicitly; default host execution still works.

- [x] **R05 — Prepare invocation inputs and stable paths.** Depends: R04, G10. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r05-inputs`, commit `34868c3`. `prepare_invocation` assembles input set + invocation id + allowlisted env + domain config (all §2 bounds enforced) with host-leak rejection. Coordinator reran demo over `AcornOS/deps/recfstab.rhai`: 1 input staged, 708B-scale config, 7-entry allowlisted env, no-leak check PASS; rejections explicit. Ginux arm prepares for real then returns not-wired (R06).)
  Owner: R-host; input-preparation module. Include base recipes, imports, assets,
  declared source closure and caches. Apply explicit environment allowlisting.
  Done: a domain can resolve those inputs without depending on the developer's
  home or absolute checkout path; unsupported input discovery fails usefully.

- [x] **R06 — Launch and supervise Ginux from Recipe.** Depends: R04, R05, G02,
  G14. (Done 2026-11-05: `tools/recipe` branch `ginux-recipe/r06-launch`, commit `73047d8`, pushed to fork. New `src/launch.rs` subprocess adapter (config + result file + stdio protocol transport + bounded pump); mandatory-Ginux selection with S0 (host/in-process rejected, RECIPE_WORKER without coordinator invocation refused); startup/workload/protocol mapping with authoritative result file. Coordinator reran: missing launcher → exit 1 startup failure refusing fallback; stub exchange exit 0 with exactly-one JSON result + 6 acked checkpoints; transport negatives all protocol failures. LIMIT: stub-verified only (no real supervisor in scope); acks carry generation 0 (R07); e2e suite needs stub/managed migration (fails post-S0 by design). I03 inventory: real-worker stub run hits the I02 layout gap (BUILD_DIR), not syscall denials. Follow-ups: (1) converge adapter argv with real G-launch surface + durable generations under R07, (2) first real-supervisor worker run for I04 syscall inventory.)
  Owner: R-host; subprocess adapter. Wire the selected contract and channels,
  bound protocol/log collection, distinguish startup/workload/protocol failures,
  and implement mandatory launch with Recipe S0. Done: Recipe starts a worker
  through real Ginux by default, rejects public host execution, and receives
  clean structured results from a minimal actual recipe.
  Exercise omitted backend flags, legacy host selectors, RECIPE_WORKER in the
  public command environment, and missing/denied launchers through CLI and library
  entry points. None may evaluate top-level recipe code on the host. R06 and S0
  are one coordinated delivery, not two implementations awaiting each other.

- [ ] **R07 — Persist acknowledged state with matching storage generations.**
  Depends: R03, R06, G12a and Recipe S2. Owner: R-host; checkpoint coordinator,
  implemented together with Recipe S3 as one state/coordinator change.
  Hold the state/output locks, associate storage generation with sequence, persist
  a separate runtime-state record, and support explicitly non-resumable
  no-persist mode. Sources remain unchanged. Done: interruption before/after
  acknowledgement recovers a consistent state/files pair.
  Extend the existing R05 input preparation with explicit target identity,
  domain paths, and matching state import. Fresh invocations need no saved state.
  Verify next-invocation state loading, nested-value round trips, input/target
  mismatch rejection, and unchanged main/base/module bytes. Version changed
  transport payloads explicitly; never reinterpret a v1 payload as v2 or restore
  a whole stale ctx over current package facts.

- [ ] **R08 — Retrieve and publish successful outputs.** Depends: R07, G13.
  Owner: R-host; publication adapter. Export only configured outputs, validate
  destinations, and publish the declared artifact tree after final success.
  Keep recovery storage separate from published results. Done: success returns
  usable files; failed install/export cannot be reported as a successful install.

- [ ] **R09 — Complete manual checks, cleanup, and removal end to end.** Depends:
  R08. Owner: R-worker/R-host sequentially where files overlap. Exercise each
  public operation against retained state, including deleted outputs. Done:
  none of these operations evaluates Rhai on the host.
  Preserve custom readiness rules and check-returned state. Inspect the explicitly
  requested target view; a complete worker base image is not evidence that the
  requested target is installed.

- [ ] **R10 — Preserve dependency execution inside the domain.** Depends: R08,
  Recipe S4's common lifecycle implementation.
  Owner: R-worker; existing dependency resolver and environment setup. Verify
  `.tools`, `.deps`, PATH, base/module resolution, and failed dependency behavior.
  Fix only demonstrated integration gaps. Done: a main recipe executes a tool
  produced by a real dependency recipe within the domain.
  Coordinate with Recipe S4 so transitive dependencies and build hooks use the
  common lifecycle instead of retaining the acquire/install-only dependency runner.
  Include a dependency that actually compiles a tool, a transitive dependency,
  cycle/missing-dependency errors, and successful-check context propagation.

- [ ] **R11 — Add outer autofix/retry coordination.** Depends: R07, R09, R10.
  Owner: R-host; existing autofix entry points. Keep explicitly enabled patch
  application outside the worker, consume structured failure, and retry using
  matching retained state. Unsupported LLM helpers fail explicitly, with no
  credentials or execution silently brokered from the host. Done: a controlled
  failure/patch/retry exercise works; ordinary execution needs no AI credentials.

### Lifecycle and resource lane

- [x] **L01 — Terminate and reap the complete domain workload.** Depends: G05,
  G02. (Done 2026-11-05: worktree `/tmp/ginux-l01`, branch `ginux-recipe/l01-teardown`, commit `c46b132c`, pushed to fork. New `teardown.rs`: cancel-signal arming, kill_remaining, TeardownOutcome, flush/close seams (G12a-owned no-op); root death in any order funnels to teardown_domain (kill-all, drain-reap, pipe release, tasks=0/live-pipes=0 audit); startup failures exit 2 supervisor-error. Coordinator reran: WAIT_REPRO PASS with teardown audit, orphan reaped (status 137), no survivors. LIMIT: process-native loop only — G01/G02 launch loop needs same wiring at merge; storage flush seam no-op until G12a wired. Follow-ups: (1) converge teardown/outcome into launch loop incl. result-file outcomes, (2) durable flush seam + backend closes.)
  Owner: G-process/lifecycle. Handle normal completion, startup failure,
  explicit cancellation, and supervisor loss according to the supported model.
  Done: real descendant processes do not survive domain termination; storage
  and assigned backends receive the appropriate close/flush behavior.

- [x] **L02 — Enforce supported CPU and memory declarations.** Depends: L01.
  (Done 2026-11-05: worktree `/tmp/ginux-l02` (base `ginux-recipe/g01-launch`), branch `ginux-recipe/l02-limits`, commit `a2542d91`, pushed to fork. Named bounds (vcpus 1–64, memory 16–1048576 MiB) validated at config load; vcpus above host CPUs rejected as unenforceable; tracee applies RLIMIT_AS + sched_setaffinity pre-exec (failures exit 131/132 startup). Coordinator reran: 16MiB memhog fails workload exit 41, 256MiB passes; vcpus=1/2 affinity masks host-observed; all invalid declarations exit 2 supervisor-error; G01/G02 regressions intact. LIMIT: launch loop only — process-loop convergence at merge; RLIMIT_AS coarseness noted. Follow-ups: (1) converge supervision into launch loop, (2) consider cgroup memory backing.)
  Owner: G-launch/resources, avoiding concurrent config edits. Enforce the
  supported CPU/memory bounds through the existing domain architecture;
  reject unsupported declarations rather than advertising unenforced limits.
  Done: bounded real workloads hit declared limits and report accurate failures.

- [ ] **L02a — Enforce task and writable-storage bounds.** Depends: L02, G11.
  Owner: G-process/resources and storage adapters sequentially. Connect existing
  bounded allocations to the selected configuration and useful exhaustion errors.
  Done: task/storage exhaustion is confined and reported; teardown still works.

- [ ] **L03 — Connect Recipe cancellation and interruption recovery.** Depends:
  L01, R07. Owner: R-host. Forward cancellation through supported Ginux control,
  wait for teardown, retain the last consistent checkpoint and diagnostics, and
  leave unsuccessful outputs unpublished. Done: cancelling during a real child
  command leaves no descendants and permits a correct subsequent invocation.

### Explicit producers and first integration

- [x] **I01 — Produce the prepared offline execution environment.** Depends:
  F02. (Done 2026-11-05: `.artifacts/ginux-env/PRODUCED.md` + staged `inputs/recipe` (`recipe 0.1.4`), gcc/make/glibc-devel RPMs, pinned Rust 1.99.0 tarball; rootfs SHAs re-verified against `.env` pins. Gaps for R05/I02: no `recipe-worker` binary yet (R02 pending); staged files not yet assembled into `/inputs`.) Owner: Integration; existing producer ownership plus a narrowly scoped
  environment recipe if needed. Build matching Recipe/runtime/toolchain assets
  explicitly under `.artifacts`. Pin/record source inputs; include no developer
  secrets. Done: required files and runtime libraries are present; ordinary
  execution consumes this environment without downloading or rebuilding it.

- [x] **I02 — Prepare the isolated recfstab source/layout inputs.** Depends: I01,
  R05. (Done 2026-11-05: `docs/ginux-i02-recfstab-inputs.md`. `recfstab.rhai` (3124 B, no extends/imports) stages exactly 1 input `/inputs/main.rhai`; source closure pinned at `tools/recfstab @98926aa` (recfstab 0.1.5, clap 4.5.54, 8 src files hashed); build output and install staging verified absent; acquisition is local Cargo.toml-existence check only, no HTTP claimed. Adaptation: NONE (submodule-owned, untestable until R06/G09/G12a). Open gaps H1–H6 handed to I03.)
  Owner: Integration; `AcornOS/deps/recfstab.rhai` if adaptation is necessary.
  Resolve the recipe's checkout-relative paths and Cargo workspace manifest/source
  closure explicitly. Prefer declared input layout; make a small recipe-owned
  adaptation if needed, without required defines or Ginux-specific package logic.
  Done: build/install locations are known and initially empty; local acquisition
  behavior and cache/toolchain inputs are documented honestly.

- [x] **I03 — Inventory the first actual worker/compiler incompatibilities.**
  Depends: I02, R06, G09, G12a. (Done 2026-11-05: `docs/ginux-i03-inventory.md`. Real worker (242 MB dynamic) + `cargo --version` under the integration supervisor (`e7936f57`, rebuilt fresh): both die exit 127 silent in loader startup; harness baseline PASS, fd1/fd2 control proves the silence is the loader's. Denial order (host-strace reconstruction, stated as such): brk → mmap → access/openat (ld.so.cache, liblzma) → fstat. Classified: memory-mapping → new M1; library file access → M2 (G-files); loader metadata → M3 (G-files, needs G07 merge); H1–H6 unreached (death precedes Rhai eval); R06/G14 + L02 nothing failed. I04 follow-ups named: M1 mmap/brk/mprotect, M2 openat/read-on-domain-fd, M3 faccessat/newfstatat. LIMIT: host-exec fall-through (242 MB unstaged); no /inputs in this loop; silent denials; R06/G14 transport unexercised.)
  Owner: Integration; execute real Recipe/Cargo under
  Ginux with captured diagnostics. Classify failures by existing owner. Done:
  concrete missing operations are identified without silent fallback or fabricated
  outputs. This task does not itself promise the build succeeds.

- [ ] **I04 — Close observed compatibility gaps in small assignments.** Depends:
  I03. (In progress 2026-11-05: **M1 done** (`7cfb8070`, pushed); **M2 done** (`698847b4`, pushed); **M3 done** (`6ad28863`, pushed); **M4 done** (`6c507d1f`, pushed); **M5 done** (`ebd68025`, pushed); **M6 done** — worktree `/tmp/ginux-i04m6`, branch `ginux-recipe/i04-m6-anonfixed`, commit `022294a9`, pushed to fork. Confined anonymous MAP_FIXED inside live reservations (zero-filled, exact caps) + exec-image snapshot (supervisor's own /proc maps at exec event: main + ld.so ranges). Coordinator reran: exec-image ranges=8, two anon tails land (24 + 456 B), death now at same-fd offset-aliased RW remaps (libm/libdl, EPERM). Remaining: M7 same-fd offset-aliasing file-fixed.)
  Owner: appropriate Ginux lane for each gap. Create one named follow-up per
  related operation family, normally one to three syscall/ABI behaviors per task;
  identify its files, dependency, contract semantics, and real reproducer. Iterate
  only as new execution evidence requires. Done: follow-ups have actual runtime
  evidence; never use a broad allowlist to obtain a passing compiler run.

- [ ] **I05 — Demonstrate recfstab build, install, execute, and export.** Depends:
  I04 and its required follow-ups, R08, L02a. Owner: Integration. Build with empty
  outputs, install with native helpers and child commands, run the installed
  `--version` under Ginux, and retrieve that executable with correct permissions.
  Done: actual files/status/logs establish the complete offline path. This is a
  command-execution smoke check, not proof of real-disk fstab generation.

- [ ] **I06 — Exercise resume, repeat, cleanup, removal, and cancellation.**
  Depends: I05, R09, L03. Owner: Integration. Use named independent scenarios
  against explicitly prepared artifacts. Include failed build/install and
  interruption around checkpoint acknowledgement. Done: results distinguish
  skipped versus executed work and prove consistent state/files and teardown.
  Include source/configuration changes with existing outputs, explicit forced
  work, a broken readiness hook, and two recipes with different custom readiness
  rules. Readiness false must cause appropriate work; a genuine check error must
  stop execution. Recipes and imported modules remain byte-for-byte unchanged.

- [ ] **I07 — Exercise a real dependency-produced tool.** Depends: I05, R10.
  Owner: Integration. Select a small dependency recipe, run it inside the domain,
  and consume its actual output in the main recipe. Done: dependency errors,
  tools lookup, and retained files behave correctly without host PATH leakage.

### Bounded network acquisition follow-up

- [ ] **N01 — Define the minimal mediated repository backend.** Depends: F02,
  I03. Owner: G-network. Reuse upstream work where available. Define destination
  scope, DNS/redirect behavior, packet mediation, resource bounds, and offline
  fixture mode. Done: the proposed design fits upstream's bounded repository
  direction and identifies the exact existing device/contract integration points.

- [ ] **N02 — Implement the bounded connectivity backend.** Depends: N01.
  Owner: G-network; existing network device/backend owner. Preserve explicit
  assignment, revocation, and packet mediation. Done: real permitted traffic
  reaches the chosen repository endpoint; denied traffic cannot acquire host
  network authority.

- [ ] **N03 — Connect the required socket/readiness ABI.** Depends: N02, G05,
  G14. Owner: G-network; coordinator owns central dispatch wiring. Add the bounded
  syscall behavior required by the selected real HTTP client. Done: the client
  can retrieve known content and report connection/denial errors correctly.

- [ ] **N04 — Demonstrate recipe-owned network acquisition.** Depends: N03,
  I05. Owner: Integration/recipe author. Exercise `download()` and hash verification
  inside the worker, using recipe-owned pinned source facts. Choose/adapt a small
  source-building recipe so one invocation downloads, builds, installs, and runs
  its result; keep normal execution free of required defines. Done: the full
  acquire/build/install/run path works, and cached/offline behavior is explicit.

### Upstream PR delivery is implementation work

These tasks are part of delivery, not an optional final suggestion. Do not wait
for every Recipe feature before opening an independently useful Ginux PR. Keep
only the relevant general feature commits in each PR; do not send Recipe-specific
code, generated artifacts, secrets, or unrelated local policy edits upstream.

If a PR depends on an unmerged contribution, use an explicit dependent branch and
state the dependency, or wait to open it until its base is suitable. Do not
present a stacked diff containing all previous work as an independent feature.
Attach every created PR to the Codex chat. Never merge on Greg's behalf.

- [x] **U01 — Open configured native execution PR upstream.** Depends: G02. (Done 2026-11-05: [PR #153](https://github.com/gmkurtzer/ginux/pull/153) `LevitateOS:ginux-recipe/g01-launch` → `gmkurtzer:main` — F03+G01+G02, configured launch + result file + streams. Pending review; acceptance external.)
  Owner: coordinator. Target `gmkurtzer/ginux`; explain configured domain launch,
  status/stream behavior, and provide the independent demonstration and measured
  limitations. Done: upstream PR URL exists and is attached to the chat.

- [x] **U02 — Open native process/thread integration PR(s).** Depends: G05a.
  (Done 2026-11-05: [PR #154](https://github.com/gmkurtzer/ginux/pull/154) `LevitateOS:ginux-recipe/u02-process` → `gmkurtzer:main` — G03→G05a (6 commits, 7 files) stacked on unmerged #153 with explicit dependency statement (review only `d1e14bb1..b3c53518`; retarget after #153). Pending review; acceptance external. Branch verified: all five supervised reproducers PASS + launch side intact; prohibited-test deletion flagged in PR body.)
  Owner: coordinator. Isolate coherent process changes and their actual runtime
  examples; explain overlap with managed shells/services/systemd. Done: upstream
  PR URL(s) exist, dependencies are clear, and each is attached.

- [ ] **U03 — Open native writable filesystem PR(s).** Depends: G09.
  Owner: coordinator. Show ordinary file operations and execution of generated
  files while preserving the immutable base and mediated boundary. Done: focused
  upstream PR(s) exist with standalone runtime evidence and are attached.

- [ ] **U04 — Open controlled storage/channel contribution PR(s).** Depends:
  G13, G14. Owner: coordinator. Separate durable storage/export from generic byte
  channels if their changes are independently reviewable. Explain the proposed
  interface choices and safety boundary without Recipe semantics. Done: PR(s)
  exist upstream with real input/output/persistence examples and are attached.

- [ ] **U05 — Open lifecycle/resource enforcement PR(s).** Depends: L02a.
  Owner: coordinator. Demonstrate cancellation, teardown, and actual configured
  limits. Done: upstream PR(s) and runtime evidence exist and are attached.

- [ ] **U06 — Open bounded mediated networking PR(s).** Depends: N03, N04.
  Owner: coordinator. Explain the bounded repository use case and packet mediation;
  show a generic client and Recipe as a consumer. Done: upstream PR(s) exist,
  declared constraints are accurate, and each is attached.

- [ ] **U07 — Address upstream review without distorting ownership.** Depends:
  whichever upstream PR receives actionable feedback. Owner: relevant lane plus
  coordinator. Reproduce issues, adapt general interfaces, and update Recipe's
  adapter if needed. Do not silently weaken invariants or abandon the integration
  because upstream chooses a different interface. Done: actionable feedback is
  addressed or an explicit unresolved design question is reported. Never mark
  unaccepted work as accepted.

### Final integration delivery

- [ ] **D01 — Add concise usage and explicit integration wrappers.** Depends:
  I06, I07, N04, R11. Owner: Integration/coordinator. Document environment
  preparation, required Ginux launch configuration, actual supported operations, artifacts,
  independent named scenarios, recovery and current limits. Add useful `just`
  wrappers without automatic builds in scenario commands. Done: another operator
  can reproduce the demonstrations from the documented prepared inputs.
  Explain legacy host-option rejection, separate state storage and migration,
  custom readiness semantics, and deferred macOS support. Show missing-Ginux
  behavior explicitly rather than documenting an alternate host path.

- [ ] **D02 — Deliver Recipe integration and superproject wiring.** Depends:
  D01, relevant Ginux implementation tasks. Owner: coordinator. Commit changed
  submodules before superproject pointers when committing the complete work;
  preserve unrelated edits. Open the requested Recipe/fork integration PRs and
  attach them, accurately distinguishing pending upstream dependencies. Done:
  integration is reviewable and actual code revisions/output provenance are known.

- [ ] **D03 — Produce the working introduction package.** Depends: D02,
  U01–U06 where those changes remain needed. Owner: coordinator. Provide a short
  explanation of Recipe, commands showing the real build/install/run, actual
  outputs, and links to upstream Ginux contributions with their current status.
  Done: the user has a concrete contribution to show Greg. Do not send a separate
  message or email to Greg without explicit instruction.

## Practical parallel schedule

With a coordinator and up to three implementation agents, choose ready tasks
from this schedule. It is not necessary to keep every slot occupied.

| Work period | Agent A | Agent B | Agent C | Coordinator |
| --- | --- | --- | --- | --- |
| Foundation | F03 after F02 | F04 after F02 | I01 after F02 | F01, F02; own shared wiring |
| Independent capabilities | G01 → G02 | G06 → G06a → G07 → G07a → G07b → G08 | R01 → R02 → R03 | Integrate shared changes; dispatch G10 when a slot frees |
| Runtime assembly | G03 → G03a → G03b → G04 → G05 | G10 → G11 → G12 | R04 → R05 | Wire G05a, then G08a; dispatch G12a → G13; open ready U01 |
| Working worker | G09 after prerequisites | G14 after prerequisites | R06 → R07 → R08 | Run I02/I03; assign small observed gaps |
| Offline completion | L01 → L02 → L02a | R09 → R10 | I04 → I05 as gaps close | Open ready U02–U05; then L03/I06/I07 |
| Acquisition and completion | N01 → N02 → N03 | R11 | N04, I06/I07 follow-ups | U06, review responses, D01–D03 |

The task dependency lists are authoritative: the schedule never overrides them.
For example, G12a waits for process consistency support, R06 waits for actual
channels, and I05 waits for actual compiler compatibility. Agents waiting on a
prerequisite should take a ready disjoint task, not fabricate its behavior.

Recipe simplification follows its own task record: pair R06 with S0, implement
R07/S3 together after S2, and reuse S4 in R10. One editor owns overlapping Recipe
worker/executor/coordinator files at a time. Independent Ginux process, storage,
and channel work can continue. Compiler/protocol checks can precede complete
Ginux runtime support but do not complete managed-execution acceptance.

U01–U06 can run as soon as their independent capabilities are demonstrable.
Upstream review can proceed alongside the next implementation lane. Acceptance
timing is external; report pending reviews explicitly rather than treating them
as implementation failures or claiming acceptance.

## Completion and honest reporting

Implementation is complete when required Ginux execution covers Recipe's executable
operations, real network and offline source workflows build/install/run, retained
state matches durable files, outputs and failures are accurate, and resources
are released correctly. Upstream contribution delivery includes opening the
relevant general-purpose PRs and responding to actionable review; upstream
acceptance is reported separately and cannot be promised by an implementation
agent.

Record actual commands, relevant input revisions, output paths, and material
limitations in the final delivery. Report partial capabilities plainly: local
source discovery is not HTTP acquisition; `--version` is not disk-installation
validation; host-side worker execution is not evidence of Ginux confinement;
declared resource values are not proof of enforcement.
