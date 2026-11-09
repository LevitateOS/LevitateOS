# PRD15 / PRD19 — callers, documentation and included products

**In progress; release acceptance remains open.** The included scope is the
PRD19 table in the contribution queue: both x86-64 Linux distros, real package
acquisition/rootfs, separate installed and live products, prebuilt kernels,
complete live-boot/live-tools ISOs, guest installer interaction, installation,
installed operation and recab A/B recovery. Desktop products, macOS, rebuilding
kernels and external AI-provider interoperability are not added by this task.

The source decision was subsequently resolved in the shared queue: the user
authorized maintained own-repository delivery with upstream Gate 11 included.
`delivery-sources.json` pins Ginux `28f4368f` and Recipe `e7bfc166`. No new
publication is performed or requested by this task. Local corrections below
still require delivery and final-combination consumer/security/product acceptance.

## Active checkpoint

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, required references,
repository instructions and this checkpoint after a context shift. Root rules
prohibit mandatory process gates and Recipe unit tests. Existing actual
integration/CLI/product owners provide verification. No publication is authorized.

**New candidate, not release acceptance:** Ginux **e4d95de10520aa8e91bc2ec69e64da004443e2da**, launcher
`21a54e60bd54a4dd2b07957cf2e139a268eb050cb73929e0b8666b31f60c2333`, frozen at
`bin/ginux-native-sigchld-1`. The prior b81f30b5 / `fb87792d…` source and frozen
`bin/ginux-opath-1` remain intact as historical/control evidence. The old running
jobs still select fb87792d; do not silently relabel them as new-candidate passes.

The complete Linux CI replay exposed an inherited Gate 11 native SIGCHLD resume
bug: it requested PTRACE_SYSCALL while no exit was pending, creating an unexpected
entry stop before seccomp dispatch. A retained userspace-signal regression fails
on the old launcher and passes on the new one. The correction chooses the resume
mode from the existing pending-exit state; no grant/native policy is expanded.
Full native controls and static fmt/Clippy pass. The new candidate also passes
all **147 Ginux unit tests (zero ignored)** and all **20 existing Linux runtime
targets** on an empty **8 GiB tmpfs**, with original assertions/deadlines. The
scratch mount is unmounted and its mountpoint removed. Exact source, command,
old-fail/new-pass and runtime receipts are under the Ginux worktree's
`.artifacts/prd15/current-ci/`.

The preceding XFS CI batch was **13/20 passed**, with six timeout failures plus
the now-fixed native failure. Namespace diagnostics preserve completed workload
exit zero / zero tasks and owned handles, followed by host XFS fsync blocking
past the unchanged 20-second harness limit. The same old launcher and namespace
case pass on tmpfs. **All six complete quiet-XFS retests now pass** with original deadlines and
assertions: storage 5.32 s, restore 30.41 s, threads 11.38 s (all modes; original
10 s per invocation), directories 0.70 s, poll/events 0.81 s, owned-exec 0.36 s.
`ginux-quiet-xfs-retests.json` preserves each command/result. The original failures
remain; only namespace has an independent fsync trace. These observations close
the ordinary CI timeout findings as load-sensitive harness timing; none was
waived or counted as a pass. Required final-source/security acceptance remains
separate.

Compiled Recipe source remains **176395aeaa5bb8999c1a6289d18e53c8012ecdf9**:
production/Cargo inputs match 91068391 and all Rust tests match 3828500. Current
clean checkout is **b185e0363ceba1290201ba84593aefb313a85cc4**, adding only README
and WRITING_RECIPES corrections after the completed frozen checks.
`author-documentation-join.json` verifies that all Rust/Cargo/test/runner inputs
remain identical; no runtime/harness byte or earlier receipt is relabeled.
Old selected CLI/worker is `467140e9b036a8ed20b58f87020ad7a63cfe7c68d311dc8e19cfe51c464cf9ef`;
image remains `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`.
The older complete 3828500 catalog **finished 152/152 passed**, zero failures,
ignored/measured/filtered cases, with before/after artifact hashes equal. Its three former
HTTP fixture-startup failures now pass in this same run. Both earlier complete
catalogs were 149/152 failed and remain preserved. New source/native acceptance
must use a fresh complete combination; do not assemble it from those passes.

The optimized release combination is prepared. Original pipeline **85702**
stopped at the retained Acorn live-tools acquisition failure described below
(`run-release-pipeline.py`, `release-pipeline-progress.json`). Recipe CLI and
matching worker are **e7073fc17ec0bb029927d0c2e901ca37e0867a7cfdc4c5641d1d6c9b14bb5c03**,
image producer **aa13ac9b58671cfda9c4126ecdb56c42a5e3d77b13137bb15030ee90921dd8a6**;
e2e release harness **893441b898da1a0e3462e56ac90c70cce3ee08ed19cf305e3e258cf95d5bf65a**.
All were built from a fresh exact 176395a export, tracked lock, Rust 1.99.0 release
profile. `release-runtime-candidate.json` pins all three harnesses and actual
image-producer path; the harness inventory exactly matches all current 152 names.

On this **new** 21a54e60/e7073fc1/ffe04d86 tuple, fresh library_backend and
image_producer pass (1/1 each), all **16 documented commands** pass including
actual produced-program execution, and all **six required real consumers** pass
with normal locks/linker threading/metadata/isolation. Their new receipts are
`release-library-backend.json`, `release-image-producer.json`,
`documented-release-results.json`, `release-consumer-progress.json` and
`consumers-release-final/`. No old tuple pass is relabeled. At that earlier pipeline checkpoint, Acorn base and
live-boot passed. **Acorn live-tools failed** when the Recipe-owned Bun toolchain
received HTTP 504 from GitHub for its pinned compiler archive. The invocation
connected to its granted GitHub endpoint, returned workload exit zero with
authoritative worker failure, and attempted no release-assets endpoint. A
separate HEAD request, including a diagnostic query variant, also received 504;
the same fresh tuple had downloaded and hash-verified this archive during Acorn
base. `bun-acquisition-504-diagnostic.json` records the environmental failure; no
product success was substituted and no source/trust change made. Levitate fresh
products and all new guest stages were then **not run**; the completed corrected
wave below supersedes that status while preserving the failure.

The first independent release catalog **3804** failed **4/152 passed**, with
148 setup failures because the task runner omitted the requested artifact parent.
Its complete failed receipt/log remain. The corrected **23362** rerun finished
**152/152 e2e passed**, zero failed/ignored/measured/filtered, in 2300.95 seconds.
Combined with the fresh library_backend and image_producer controls on these
exact same runtime bytes, this is **154/154** current required integration cases.
`complete-catalog-release-retry1.json/.log` and
`release-retry1-complete-catalog.json` retain inventory/name equality, frozen
actual producer recipes and before/after source/artifact hashes. No old pass is
added to this count.

Post-catalog resource observation passes: no matching task processes, mounts or
loop devices, all **702** checked regular lease files unheld, no inspection
errors (`release-catalog-resource-observation.json`). This is bounded ordinary
accounting, not independent adversarial security acceptance.

Actual new metadata deployment also passes on **both pinned kernels** under
operator `qemu:///session`: new EROFS **dbe1e993…**, actual Acorn base's static
recstrap **13f6a3da…** (clean observed source 5de6647), and unchanged C controller
**fc182144…**. The unchanged guest assertions verify UID/GID, setuid/setgid/sticky,
hardlink identity, absolute symlink, payload, capability bytes and epoch mtimes.
Both transient guests shut down; metadata inputs stay unchanged.
`metadata-release-deployment.json/.log` and `release-metadata-deployment/` retain
commands/inputs/consoles. The existing fixture producer now inspects ELF INTERP
before selecting dynamic libraries, so the actual static PIE can be tested;
no guest oracle or deployer guard is removed. This is not whole-distro acceptance.

**Product linker finding corrected:** builder **9468d26e4782f3dc556e609432f62292431c3c0c**
removes only `-C link-arg=--threads=1` from `recipes/install-tools.rhai`.
The actual managed source-preparation/build/install producer passes with normal
linker defaults and **two eligible CPUs**, unchanged locks/metadata/isolation/
limits. Its program is **13f6a3da…**, byte-identical to both fresh metadata guest
inputs. `normal-linker-producer/receipt.json` is immutable;
`normal-linker-recipe-join.json` binds the tested file bytes to the clean commit.
All compiled builder inputs and frozen 4d5928b7 executable are unchanged.

The older Acorn retry **1051618** finished exit zero, run **A1MvL15O5k2zwpl4**.
Watcher **58514** stopped only paused future orchestrator 1051306 afterward;
**case_interrupted=false**. Both old sessions 47148/58514 are finished. No old
product receipt is substituted for the corrected recipe's required acceptance.
The GitHub 504 recovered through the normal pinned URL/trust; no digest or grant
was changed.

**Embedded guide finding corrected:** docs/content clean source
**d9384939cfcecf6af37d45040c0404f50dbf69f7** updates seven CLI/format/helper/FAQ/
troubleshooting pages. Every executable recipe requires literal NEEDS, inputs/
outputs have kinds, recipes own grants, callers narrow graph-wide, dependency
instances/state are separate, manual readiness is cached-only, and dependent
remove/cleanup stays unresolved. Fictitious complete examples were replaced by
three actual offline examples. All three pass managed validate/install/reuse/
readiness and real published program execution; defined no-dependency removals
preserve prior publications. Missing NEEDS and explicit-empty undeclared outputs
are rejected for their precise reasons. `embedded-examples-v2/receipt.json`
retains exact code/page hashes, all commands/logs and failures. The first verifier
expected a later error and failed; it is preserved with an explicit disposition,
not counted as a pass. Illustrative acquisition/HTTP fragments are marked as
non-standalone placeholders, not claimed successful downloads. Generation (34
pages), typechecking, installed lint and cached formatting pass. No downloads,
unit tests or publication were needed for those static checks.

**Corrected product/guest wave 94168 completed successfully.** All six
`<product>-<distro>-linker-final` ISO builds pass, with normal linker defaults and
the corrected embedded guides. Both `cargo xtask test all <distro> --force`
commands pass **all eight named scenarios**: preflight, live boot, live tools,
install, installed boot, login, runtime and A/B. Both separately invoked
`ab-update` commands also pass. Each A/B invocation verifies **seven actual
boots**, including degraded-trial automatic rollback, healthy commit with machine
identity, staging/commit of the other slot and rollback. No required command was
interrupted or omitted. Both actual installer PTYs and their warning/marker
contracts pass; the earlier duplicate Facts-key defect is absent.

`final-product-manifest-evidence.json` independently reconciles every actual
manifest/time window/input: all scenario releases are the six current products,
and both explicit A/B runs consume the same current successful installs.
Acorn install **A1N55dy6zUgDfEuN**, explicit A/B **A1N5LJyWNSRU3K7o**; Levitate
install **A1N5ZrRMEMJ2QYuI**, explicit A/B **A1N5yaraoiUrgcUu**. The actual retained
PTY transcripts are hashed there. `normal-linker-product-progress.json` and
`normal-linker-scenario-progress.json` retain all commands and complete outcomes.
No old case is substituted. ISO hashes are unchanged after scoped `virt_content_t`
labelling; host SELinux remains enforcing. Every VM used transient operator
`qemu:///session`, and the final domain listing is empty.

Current source guards are `normal-linker-combination-sources.json`: builder
9468d26, Recipe b185e036, docs/content d938493, recstrap 5de6647 and the unchanged
other owners. Actual new products contain the same **13f6a3da recstrap** verified
on both pinned kernels and **6d2971b9 recab** from the concrete controls;
`normal-linker-{acorn,levitate}-base-byte-proof.json` records this. Core runtime
and harness bytes remain the completed **154/154** tuple compiled at 176395a;
this does not relabel that catalog as executing changed external recipe/docs
inputs. Those changed inputs have their own actual managed build/example/product
and guest receipts.

`final-product-resource-observation.json` passes after completion: **42** exact
producer diagnostic directories present, **796** checked regular lease files
unheld, no matching task process/FD, mount or loop and zero inspection errors.
This is bounded ordinary accounting, not independent adversarial acceptance.
Current actual builder failure preservation also passes on a mutable fixture copy
(`current-builder-failure-preservation.json`): precise stale privileged-content
rejection, both old EROFS hashes and directory entries unchanged, no staging
leftover, original frozen fixture/guest inputs untouched.

**Local source integration:** ordinary `tools/ginux` is now clean detached
**e4d95de1**, replacing its old local 28f4368f checkout. Ordinary `make build`
produces exactly the selected **21a54e60…** launcher bytes, without the separate
worktree as its source (`local-maintained-ginux-join.json`). The first command
wrongly required an attached main branch, made no source change and built the
old pin; its failed guard/old build are retained separately, never counted for
e4d95de1. No branch/ref or remote was changed; parent gitlink/public origin remain
at 28f4368f until actual source delivery is authorized and reconciled.

Maintained Recipe **b185e036** standalone build **78347 passed**, from its exact
Git archive, tracked standalone lock and Rust 1.99.0, using a fresh target and
`cargo build --locked --offline --release --bins`. Both actual outputs are
byte-identical to the frozen verified combination: CLI/worker **e7073fc1…** and
image producer **aa13ac9b…**. Stable copies are
`bin/recipe-maintained-b185e036` and `bin/recipe-image-maintained-b185e036`.
`maintained-recipe-build-corrected.json/.log` retain the exact command, source,
compiler and full hashes. The first invocation mistakenly gave rustup
`+toolchain` syntax to a direct Cargo executable; no build ran and that failure
is preserved. No selected runtime or frozen receipt is replaced. Archive
SHA-256 is **7d8098e4…**, `maintained-recipe-b185e036.tar`.

**Local delivery review is prepared**, not published or adopted:
`prd15/delivery-review/manifest.json` identifies all ten current source owners,
their recorded parent gitlinks and nine complete binary patches. The proposed
parent pin/lock patch is **02178705…**. Current heads and every patch hash were
rechecked after creation. This packet records preparation; it does not imply
remote availability, a new complete-diff review or release acceptance. The
maintained repository choice is already authorized; publishing these exact later
revisions still needs permission. Local workflow inspection finds automatic
push-triggered CI for Ginux, Recipe and docs/content. Recipe release publication
is explicitly opt-in; Ginux's unchanged upstream workflow also includes a macOS
job, which has no release acceptance requirement here. Do not dispatch, focus on
or claim macOS verification. Account for automatic workflows before seeking any
future push authorization. No push, PR, comment, release or hosted run occurred.

The tested workspace lock **36066e42…** passes `cargo metadata --locked --offline`
on current manifests. Its diff changes only five local package records
(distro-builder, distro-profile, removed fsdbg, install-tests, recab); every
registry/git package entry is unchanged. `workspace-lock-reconciliation.json`
records exact edges and commands. Keep it with the matching obtainable source
pins during final delivery; it remains uncommitted and unchanged during checks.

Current Recipe compiler checks pass: independent disposable stable 1.99.1
unlocked binary build/fmt/strict all-target Clippy; independent MSRV 1.93.0
unlocked check/locked build; and both stable/MSRV binary builds against the
actual tracked lock. The two independently resolved locks are `a4eda69a…`,
differing only in itoa 1.0.18→1.0.19 from tracked `48c776bd…`. They are compatibility
evidence and **not adopted**. Workspace locks and selected runtime are untouched.
`recipe-tracked-lock-checks.json` records the explicit disposition. No Recipe
units or macOS checks run; hosted Linux still requires final-source authorization.

Consumer sources: builder 34172e7, recab 61b859c4, recshuttle dfe9ef1, recqemu
f0364bb, install-tests 056a46b0, TUI ade4d9aa (all committed/clean). All **six
recab-corrected base/live-boot/live-tools ISO builds now pass** on the older tuple.
`levitate-corrected-base-source-proof.json` records Levitate base A1MZqU4qyO1yqlJr,
ISO `beb38e3e…`; actual recab bytes `6d2971b9…` match the managed producer and
real Acorn guest completion/cancellation control. All 24 recab CLI integrations
pass. The old whole guest batches remain failed; current ordinary product/guest
acceptance is the separate completed corrected wave above.

Actual earlier PTY transcripts exposed duplicate React keys for repeated Facts
labels. TUI ade4d9aa fixes keys by position+label. Install-tests 056a46b0 commits
bounded host SSH waits and rejects the warning in the actual guest transcript;
old failing transcript is retained. At that earlier checkpoint, **neither final UI product nor final whole
VM acceptance was claimed**. The fresh six-product UI and final VM waiters
were stopped while empty after the native finding: `native-fix-scheduling.json`
records **no test/product case interrupted**. Old sessions 64859/65780 are now
superseded. Do not expect their final receipts. Old producer 60000 finished;
inspect its six exit-zero JSONs before restarting anything.

Earlier same-old-tuple receipts remain useful provisional evidence: six PRD11
consumers (normal locks/linker threading/metadata/isolation), 16 documented
commands including actual produced-program execution, plus freshly executed
library_backend and image_producer. They are not a full new-native/release tuple
claim. Exact docs/catalog corrections and source-equivalence proofs are retained;
the mixed root interface edit remains uncommitted alongside unrelated existing
hunks. The delivered baseline is explicitly e7bfc166/28f4368f; it does not include
all later corrections or close final source/product/security acceptance.

**Security stays open.** Returned Daybreak manifests independently verify
504/504, 743/743, 131/131 and 170/170 files, with selected passes and precise
remaining criteria. Own CLI used authorized gpt-6-sol/high/daybreak_blue but was
rejected for missing authorized hardware-key authentication. A user reauth/
original-context-results question is pending. No standard-access fallback or
parent adversarial matrix is authorized by that error. Preserve the interrupted
CLI evidence; new native source/binary needs affected verification and final-source
join, not an inherited old receipt. User-held dependent-recipe remove/cleanup,
actual source delivery/joins, PRD05 actual-merge requirement and hosted Linux
acceptance remain explicit. macOS and external AI-provider interoperability are
excluded. Original frozen PRD04 candidates remain unchanged.

Author documentation correction is now **committed in Recipe b185e036**:
README and WRITING_RECIPES use separate-instance authority/product snapshots and
cached-only manual readiness, retaining unresolved dependent remove/cleanup.
The former `pending-author-doc-fixes.patch` is retained as exact change evidence
(SHA-256 `8987c0bb…`), and `author-documentation-join.json` records the two changed
files, unchanged compiled inputs and local-link validation. It is not pending
application. Runtime binaries/harnesses remain the exact verified 176395a builds.

**Exact next actions:**
1. Inspect standalone maintained Recipe build **78347** and its corrected result.
   Compare built bytes with e7073fc1/aa13ac9b; preserve the actual source/archive/
   command and do not relabel old checks as executing different bytes. Ordinary
   product/guest wave 94168 and all prior catalog/static/example/producer jobs
   are complete; do not restart them without a concrete affected change.
2. Prepare exact local delivery diffs/source/pin/lock disposition on the reviewable
   candidate. Own-repository source choice is resolved, but new publication and
   hosted Linux actions are not authorized. The old public baseline remains
   28f4368f/e7bfc166; parent gitlinks/pins must not claim private source obtainable.
   Preserve unrelated edits and shared staged patch f4d8c81b…; no PR/comment/push.
3. Required security hardware-key auth/results remain pending. Use the corrected
   [Daybreak release request](prd15-daybreak-release-request.md) after actual
   reauthentication; it records current b185e036 source locator with unchanged
   compiled 176395a/e707 bytes. No standard-access fallback or old-result closure.
   Dependent-recipe explicit remove/cleanup remains held for the separate user
   decision; do not implement, waive or silently exclude it.
4. Latest read-only upstream metadata **10:31:50 UTC** (`Ginux worktree
   .artifacts/prd15/current-ci/upstream-after-products.json`) still finds main
   53a54adde, Gate 11 open at 9759daa and all six required PRD05 PRs unmerged.
   PRD05 remains incomplete. Final release acceptance stays open despite all
   ordinary local product/runtime checks passing.

Evidence root: `.artifacts/ginux-contribution-preparation/prd15/`; current Ginux
CI/security artifacts are in its worktree `.artifacts/prd15/`. Preserve shared
staged queue/handoff patch SHA-256
`f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.
The existing uncommitted workspace Cargo.lock/config hashes are recorded in
`final-harness-root-source.json`; preserve unrelated edits and reconcile final
pins/lock rather than claiming a fresh public checkout already reproduces these
later private candidates. No push, PR/comment, hosted run or release occurred.

## Historical progression and source-bound evidence

The following records preserve earlier findings, corrections and failed
attempts. Their source bindings remain valid; superseded checkpoints are not
current release acceptance. The active checkpoint above governs resumption.

## Current evidence and findings

| Outcome | Receipt/disposition |
| --- | --- |
| Current caller compilation | `caller-build.log`, `builder-build.log`, `caller-pty-build-final.log`; actual builder/contract/xtask and linked consumer code compile. |
| Declaration inspection | All 14 executable builder recipe declarations inspect successfully (`recipe-declarations.json`). Desktop declarations are inspected as current source, not added to executed release scope. |
| Public package acquisition | `preseed-levitate-toolchain.json` and `preseed-acorn.json` pass on the explicit tuple. Both use the actual locked public Fedora RPM/Alpine APK paths with hash checks. The user explicitly authorized missing pinned downloads; locks are unchanged. |
| Runnable examples | Hello and threaded Cargo each pass install, custom readiness, fresh-coordinator reuse and actual published-program execution. `example-results.json` retains all results. Exact author and man-page code blocks also pass (`documented-author-inline.json`, `documented-man-inline.json`). |
| Static commands | `documented-validate`, `documented-inspect`, `documented-diff` and corrected `documented-explain-corrected` pass. The initial probe used nonexistent `filesystem.write`, which correctly failed; the documented `network.connect` operation passes. No implementation defect or false success is inferred from that probe error. |
| Real documented tool producer | Initial source preparation with package `recfstab` fails: it is a binary of package `recstrap`. README now uses the actual workspace builder command with `--repo-root`, prepares `recstrap`, then selects `BIN=recfstab`. Corrected source preparation and managed static compilation pass (`documented-tool-source-corrected.log`, `documented-tool-build.json`). Locks, linker threading and isolation remain enabled. |
| Metadata production | Existing actual CLI case `target_metadata::cli_target_metadata_survives_publication_reimport_and_erofs_production` passes on the current tuple (`target-metadata.json`, retained `metadata/` fixture). The old guide named a removed case and incorrectly attributed deployment to this case; corrected to separate production and deployment receipts. |
| Actual deployment | Both `metadata-vm-{levitate,acorn}/console.log` contain `PRD12_VM PASS`, using the exact new EROFS and current built recstrap with each existing prebuilt kernel. Domains shut down and disappear from session libvirt. This is real metadata deployment, not full distro boot/install acceptance. |
| Helper/man documentation | Registered helper inventory has no missing entries; five target-metadata helpers were undocumented and are now listed. `print`/`debug` are deliberate Rhai built-ins. All three manuals render with groff without warnings; all 20 local Markdown links in the audited Recipe documents resolve. |
| Root privilege/toolchain setup | First privileged xtask attempt fails before Recipe: root has no rustup default. Retained as `preseed-levitate.json`. Retry explicitly selects the already installed operator compiler path and Cargo inputs for the whole coordinator; passes. No launcher-only sudo wrapper or host-execution fallback is used. |

The documentation now covers API 2/NEEDS, typed complete install/reuse outputs,
dependency-owned authority and graph narrowing, exact image/worker selections,
root-private publications, recovery/retention, global/vDSO clocks, zero sysinfo,
per-process address space and real CPU affinity, numeric endpoint authority,
bounded poll limitations and whole-second archive times. `docs/runtime-limits.md`
is shipped through the existing documentation asset wildcard. External-provider
interop is explicitly outside acceptance; the sample is configuration syntax.

The live-tools scenario previously tested only probe/split smoke. A focused
guest PTY interaction is being added to that existing owner, with its actual
terminal transcript retained in each run. It is compiled but **not yet verified**.
Do not count it as support until both new products run it successfully.

## Subsequent product findings and required corrections

- **TUI ownership decision:** the user explicitly requires Recipe-owned TUI
  acquisition and compilation. The current pinned host-Bun producer is not an
  accepted exception. The work must move out of `tui_binary.rs`; successful
  existing host-Bun products cannot close this criterion. Both installed/system
  and live/install binaries are included; desktop scope is not added.
- **Bun acquisition:** new `distro/builder/recipes/bun-toolchain.rhai` owns
  public version 1.4.2, both archive pins, declared GitHub endpoints, checked ZIP
  extraction and exact typed compiler/runtime publication. Its fresh actual
  install passes (`bun-toolchain-acquisition.json`). Official release metadata
  confirms both pins; `bun-source-upstream/availability.json` identifies compiler
  source `744846f844374847c902b5e7fd59b4342a51ef99`. The published compiler's
  SHA-256 is `a83d263767d839e4d2649ca8e35d07159c7afc99afdc96d731ced29e056dda0c`;
  it matches the prior cached compiler byte-for-byte. The musl runtime is
  `16b72935ffd7a503b978c186874539c92aade4e3515b70a5abf5db2581fdef7d`.
  This proves acquisition, not compilation or complete TUI migration.
- **Actual Bun compilation is blocked:** three retained managed reproductions
  fail resolving `/work/build` or `/inputs`, including explicit immutable cwd,
  PWD and source input. The lower-noise `bun-direct.stderr` records `readlink`
  returning EACCES followed by openat EACCES. Exact Bun source
  `src/sys/lib.rs::get_fd_path` resolves Linux descriptors via
  `/proc/self/fd/N`. Configured Ginux currently denies it. The user **approved a
  bounded configured-only readlink view of descriptors already owned by the
  caller, returning domain paths only**. Native policy and host-proc denial stay
  unchanged. No generic procfs, native ABI expansion, publication or maintained
  fork adoption is authorized. Final security verification remains required.
- **Isolated correction source:** managed detached worktree
  `/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11`, clean base
  `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`. Implement the bounded view there,
  using the public descriptor table and existing filesystem/image path owners;
  do not edit the concurrently integrating original worktree. FD-path correction
  is now committed locally at `2477c0d4`, launcher SHA-256
  `a4e0c6a4b08856bca1818be8a6acf5b3ba726d737cb3c8d0c33cf78de87097ee`.
  Baseline fails the new ordinary controls; candidate passes image/owned names,
  aliases, rename/unlink, fork/exec, bounded numeric names, truncation and explicit
  denial. All existing FD controls and all 43 VFS units pass. Initial fixture
  compiler/output-length mistakes remain recorded separately. Native policy and
  catalog digests are unchanged; adversarial/final-source security is not closed.
- **Mutable launcher incident:** Acorn live-tools and Levitate base-rootfs
  stopped at a later launch when catalog 2's digest changed from `121cb495…` to
  `ac6e693e…`. Those runs fail and remain preserved. The shared launcher path was
  rebuilt during the runs; subsequently observed binary hash is `e5d87cb5…`,
  from a conflicted/modified worktree, not a reproducibly pinned candidate.
  Recipe's guard correctly refused it; no new digest was accepted. The
  previously sealed PRD13 executable was copied unchanged into
  `prd15/bin/ginux` (0555), verified as `0dd243fa…`. `run.py` now verifies the
  selected image/worker/launcher hashes before and after each run. Prior receipts
  retain their original paths and limitations. Current upstream main read over
  GitHub remains `53a54addea67969abaa240ea478def1b94130eee`; this incident is not
  evidence of a new upstream main or authority to adopt the conflicted source.
- **Actual product progress:** new Acorn base-rootfs
  `A1LtZzYkkhvNVgkJ` and live-boot `A1LvCQzI8EW3uRlu` succeed. They still use
  the host-Bun boundary which must now be replaced. Levitate's real Fedora rootfs
  recipe succeeds, but its base ISO fails later at the changed launcher; no
  complete Levitate ISO pass is claimed. Acorn live-tools fails before completing
  its tool set. Full current-product VM/install/A/B acceptance has not run.
- **Kernel mismatch:** freshly acquired/verified prebuilt Acorn matches every
  selected bundle entry. Levitate's old selection has the same release string
  but 265 differing entries against the pinned release, including kernel image
  hashes `ce542acd…` versus pinned `05e651ed…`. Red comparison is retained in
  `kernel-levitate-comparison.json`. The normal prebuilt-only caller refresh
  succeeds (`refresh-selected-kernel-levitate.json`), without compiling a kernel.
  Both corrected selections match every freshly verified bundle entry
  (`kernel-*-comparison-selected.json`). The refresh completed before the
  ongoing Levitate build reached kernel assembly. Its affected metadata
  deployment control also passes on the new kernel
  (`metadata-vm-levitate-pinned/console.log`). Version/config-shape status alone
  is not byte-pin provenance; release reproduction must refresh/verify the
  prebuilt selection explicitly.
- **Older-ISO PTY diagnostic:** the current scenario correctly rejects the old
  Acorn ISO for missing `recab-tui`. An independently opened older ISO through
  `cargo xtask run live-tools acorn --ssh` was used only to debug actual PTY
  input. First oracles expected an exit-code label and a login-help banner which
  that UI does not emit. Both failures are retained. Corrected interaction
  passes shell command/file handshake, docs-pane input/exit, terminal-host quit
  and fallback-root-shell command execution (`diagnostic-old-pty-final.log`).
  This does not certify the new product, all apps, Ginux PTYs or a full
  UI-driven disk install. The diagnostic guest was closed normally.

- **Next actual Bun requirement:** the FD-path correction gets beyond directory
  resolution. Actual compilation then fails at event-loop initialization.
  `bun-direct-fd-path.stderr` records syscall 291 (`epoll_create1`) and 290
  (`eventfd2`) returning ENOSYS; the generic Bun message about FD exhaustion is
  not an observed limit failure. The user explicitly **approved bounded
  configured-only event counters and epoll readiness over already-owned public
  descriptors**, preserving existing public resource limits, native policy and
  host-FD/network denial, with final security verification still required.
  The bounded IPC support is now implemented and verified as recorded below. Other denied crash-report calls in
  the log (ptrace/process-memory reads, etc.) are not granted by that approval.
- **Recipe-owned acquisition commit:** builder `bb14537` contains the complete
  standalone Bun toolchain producer. Its actual fresh install, installed check
  and fresh-coordinator reuse all pass on frozen `0dd243fa…`; reuse verifies
  published hashes and performs no acquisition. Compiler/runtime byte identities
  are in `bun-public-payloads.json`. TUI compilation, dependency acquisition and
  direct builder wiring remain unfinished.

## Current continuation: Gate 11, IPC and Bun stack discovery

The maintained source now exists. The earlier source-delivery hold above and
in historical receipts is superseded by the queue's recorded user decision.
Both local corrections were rebased onto Ginux `28f4368f` without conflicts:
`ef8d5d4a` (owned FD names) and `5eaf5f2e0970c8dc3acd4fcb4b2e1ee95561547e`
(bounded eventfd/epoll). The clean IPC candidate's sealed launcher is
`bin/ginux-event-current`, SHA-256
`566381213c73f9fa10230f329e21d160f916185b5bdb6ff96fadcf81bad6c270`.
Earlier commits `2477c0d4`/`4eddb013` and their binary receipts remain historical
source-bound evidence; they are not combined into a final current-source claim.

- **IPC implementation:** counters/epoll share the existing 16-object channel
  pool and 256-slot public table, honoring lower operator limits. Epoll watches
  owned channels/counters and granted TCP only, with 16 registrations, weak
  generation-bound identities, alias/fork/close/exec handling, level/edge/one-shot
  readiness, finite/infinite waits and round-robin delivery. RDHUP observes real
  TCP half-close; it is not synthesized from pipe HUP. A non-null pwait mask is
  supported only when equal to the task's existing mask; other masks are denied.
  Kernel-internal futex restart-block results become EINTR for normal userspace
  retry; restart_syscall remains ungranted. Native policy and public limits stay
  unchanged. The documented support remains bounded, not general Linux ABI.
- **IPC verification:** all 16 new ordinary functional modes pass, including
  bad-buffer state preservation, explicit deny and lower FD limit. Existing
  FD/task/thread/poll/network runners pass on the Gate 11-based candidate
  (`event-current-runtime.log`); strict Clippy and release build pass. Core,
  contract and device units pass, and the retained runtime contract runner passes.
  A mistaken Make target name was corrected; its failed command remains recorded.
  Final adversarial/security acceptance is still required.
- **Compiler evidence:** compilation/publishing succeeds. The first probe lost
  the source extension at `/inputs/source`, so Bun bundled an inert asset; native
  execution exited silently and was not counted as success. Preserving `.ts`
  produces an actual executable printing `bun-compiled-output:42` natively
  (`real-bun-compile-publish-ts.json`, `bun-published-native-ts.json`). A native
  compiler control corroborates the loader distinction. This is diagnostic
  compiler evidence, not full TUI or guest acceptance.
- **Bounded DNS decision and correction:** the user approved deterministic
  address selection after registry.npmjs.org resolved to 24 addresses, exceeding
  the existing eight-endpoint cap. Recipe `e28a60ab05393325a95dee14e2a4a7361e598aa5`
  preserves explicit numeric grants, reserves one address per sorted hostname,
  then fills remaining slots round-robin in numeric address order. Only selected
  addresses appear in Ginux grants, immutable hosts and `authority.json`.
  Overflow warns; a grant set unable to fit still fails. No limit, TLS check,
  workload DNS authority or native policy changes. The real managed regression
  runs NSS against a private child mount namespace, leaves host configuration
  untouched, reverses answer order and checks all three views agree. It passes;
  all 12 affected NEEDS CLI/runtime cases pass on the same sealed CLI/worker and
  launcher (`network-subset-needs-regression.log`). Strict Clippy passes. CLI/
  matching worker SHA-256 is `5ca87251f17567c757499b097e710908f55a0bdcbb3ee98da70f5f918bdee8b3`.
- **Own-stack decision:** frozen npm install then reports false lockfile stack
  overflow; Bun JavaScript initialization also crashes. Pinned Bun parser source
  uses StackCheck; matching glibc 2.39 `pthread_getattr_np` reads
  `/proc/self/maps` for the initial thread. The same ordinary pthread query
  succeeds natively but returns EACCES in configured Ginux. The user approved
  a bounded configured-only own-stack view, with no other mappings or host paths,
  native-policy change or broader proc access. Source reads and hashes are in
  `bun-source-upstream/stack-source-reads.json`.
- **Own-stack implementation in progress:** `stack_view.rs` reads only the
  registered stopped task's maps, scans at most 1 MiB with a 512-byte line bound,
  and emits one canonical `[stack]` range. Open creates an immutable anonymous
  snapshot through existing VFS object/byte/descriptor budgets; no namespace or
  export entry is retained. Path/FD metadata is deterministic. Reopen/mmap and
  metadata mutation are unsupported. Alias/fork/exec and final close use the
  existing public descriptor owner. Native policy and broader proc denial remain.
  The pthread query now passes. Snapshot/deny controls pass, including repeated
  opens, shared offsets/pread, failed read preservation and fork/exec. Source
  bundle and binary identity are in `stack-candidate-2-source/`; implementation
  is committed at `7a291c0470ad476e8c68d11a669e4b9aa0298e41`; strict Clippy
  and all affected FD/IPC/task/thread/network runners pass. Initial metadata adapter build
  errors and an owner-routing failure are preserved and corrected.
- **Acquisition remains unaccepted:** a probe first used invalid kind `dir`;
  corrected to `directory`. `cp -a` requests unsupported archive-mode metadata;
  using the existing input-mode-preserving `copy_tree_contents` helper corrects
  that probe without changing package/target metadata policy. With the stack view,
  actual frozen npm install passes lockfile parsing but reaches its 600-second
  execution deadline and fails (`real-tui-npm-stack.json`); no successful
  acquisition or product is claimed. The task-owned processes are gone.
  Live supervisor ptrace inspection is denied by the existing runtime boundary;
  it was not bypassed. A smaller direct diagnostic first exceeded argv bounds;
  the script-input retry is retained separately. A smaller source-copy loader
  retains the frozen package/lock/workspace manifests while reducing startup
  trace noise. Its first direct contract omitted Recipe's syscall connect grant;
  corrected to match the actual derived contract before diagnosing the candidate.
  A separate bounded diagnostic binary increases trace/record capacity only for
  diagnosis; its patch is retained and immediately removed from committed source.
  Interrupted/timed-out diagnostic runs never count as passes.

Exact next action: inspect the current own-stack affected checks and the bounded
npm diagnostic/status; resolve any actual functional failure without widening
native policy or resource limits. Finish and wire Recipe-owned TUI dependency
acquisition/compilation/publication for both layers, then rebuild both complete
product chains and execute both distros' required session-libvirt/PTY/install/A/B
scenarios on one exact combination. Obtain required final-source/security results
through Daybreak before closure. PRD15 and product acceptance remain open.

### Latest continuation checkpoint

- Own-stack correction is committed at `7a291c04`; the next consumer finding is
  concrete epoll ENOSPC with the first 16-watch draft. The existing contract can
  allocate 16 TCP sockets plus channel wake objects. Local commit
  `45b1f478ac79ccc6893b1f512e0382f875c11f08` sizes the new bounded watch table to
  32, covering those existing owners. It changes no public FD/TCP/channel
  allocation limit. All 17 functional modes pass, including 31 distinct live
  objects watched together, fairness/cap rejection, and rejection of a
  seventeenth TCP socket (`epoll-combined-runtime-final.log`). Strict Clippy
  passes. The normal sealed launcher `bin/ginux-epoll-combined-1` SHA-256 is
  `672e56678ac60b871ed048a30b0cfd0ec5dc413fae424a2eb3b8145fc45c5859`.
- A separate bounded trace confirms the 32-watch correction removes epoll
  registration errors and actual TCP receives progress. Full acquisition is
  still not accepted: the original 16-watch invocation timed out; its filtered
  rerun was deliberately cancelled after the finding; the first 32-watch run
  also reaches its deadline. These are failures/interruptions, never passes.
- Pinned Bun has a separate HTTP-thread ceiling, read from
  `BUN_CONFIG_MAX_HTTP_REQUESTS` in `src/http/AsyncHTTP.rs::load_env`.
  `real-tui-npm-bounded-http` sets it to four as well as the explicit install
  concurrency flags, preserving Ginux's 16-socket cap. It is still running at
  this checkpoint. Source reads/hashes and the uSockets epoll owner are retained
  under `bun-source-upstream/`. Runtime reports show main/event workers parked
  with four TCP watches, all other workers waiting in native futexes; no guard
  is held. The separate diagnostic binaries/patches are not delivery candidates;
  normal source has been restored and the isolated Git worktree is clean.
- A disposable native control is running the identical pinned frozen workspace
  installation with a separate initially empty cache. Its purpose is to
  distinguish Ginux behavior from package/network behavior. Neither its
  node_modules nor its cache will be used as producer inputs. Result is
  `npm-native-control.json` when finished; native success cannot certify managed
  production.
- **Builder migration is uncommitted and unaccepted:** new
  `recipes/tui-binary.rhai` declares a typed source/target and program output,
  a separate `bun-toolchain` dependency, frozen TUI-graph acquisition with
  bounded HTTP settings, compile with the pinned target runtime, hash-checked
  readiness/reuse and publication. Corrected declaration inspection passes;
  the first inspection accidentally selected an older ambient launcher and was
  rejected, retained separately. `src/tui_binary.rs` now prepares authored
  source/manifests/entry without host caches and calls the producer directly,
  passing explicit parent/child target inputs. Host Bun/runtime downloads and
  node_modules dependence are removed from that draft caller. Current builder
  compilation passes (`tui-caller-migration-build.log`), which is not runtime
  acceptance. The root packageManager is aligned to pinned Bun 1.4.2 locally.
  Do not commit/adopt the migration as working until the actual producer and
  resulting products pass. Builder README still needs its final contract edit.

Exact next resumable action: inspect `npm-native-control.json`,
`real-tui-npm-bounded-http` and `bun-npm-state-diagnostic` results; diagnose the
remaining managed acquisition wait using their exact source/inputs. Active tool
sessions at this checkpoint are 81798 (90-second native control), 23257
(600-second bounded managed install), and 10443 (35-second task-state diagnostic;
may already be finished). The isolated Ginux source is clean at `45b1f478`;
there are no intended live VMs. Preserve the shared staged queue/handoff patch
hash `f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.
The accidental report commit that included those files was corrected immediately;
they remain staged exactly as before. Continue the required producer/product work
and obtain final security results through the authorized Daybreak context; no
publication has been performed by this task and acceptance remains open.

### Eventfd wakeup root cause and explicit producer quota

Pinned uSockets `packages/bun-usockets/src/eventing/epoll_kqueue.c` sets
`leave_poll_ready=1` and EPOLLET for its wake eventfd, intentionally leaving the
counter unread. It expects each successful write to notify again. The draft's
readiness-transition-only model loses subsequent completion wakeups. This is a
current implementation defect, not a user/network hold. Commit
`f98281d9f2e362d86fd3a1d702ada8f0a8c1344c` notifies IN on successful counter writes
and OUT on successful reads, while one-shot admission, copy faults and existing
allocation limits remain intact. The exact undrained/alias/error regression
passes natively and on the correction; the former 32-watch candidate fails it.
All 18 event controls and all affected FD/task/thread/poll/network runners pass;
strict Clippy passes. Sealed normal launcher `bin/ginux-counter-notify` SHA-256
`796f70e9a17a4a80d28812e0a2ee2dbb56645a8152c3d9cb6a196fc774e9b76e`.
No diagnostic logging changes remain in source.

The separate native control succeeds with 98 packages and an independently empty
cache (`npm-native-control.json`). Those files are not producer inputs. With the
notification correction, managed frozen install now exits with an explicit VFS
ENOSPC instead of hanging (`real-tui-npm-notify`). Its 16,384 default object quota
is too small: the native control counts 10,149 install entries plus 9,745 cache
entries before source/build scratch. The TUI producer now explicitly requests
32,768 objects within the existing supported resource contract; no Ginux public
limit is raised. Both its standalone acquisition probe (`real-tui-npm-object-budget`)
and full GNU system producer (`tui-system-producer-gnu-objects`) are running.
The first full producer verified/acquired/published its pinned toolchain, then
hit the same parent quota. It is retained as a failure; an unpublished draft or
that dependency success does not complete the producer. The earlier HTTP-bound
and 16-watch filtered acquisitions were deliberately cancelled after the proven
findings and are not passes.

Current sessions: 6380 (object-budget acquisition probe), 20192 (earlier GNU
producer, may have completed its quota failure), and the new GNU producer session
from the adjacent tool result. Inspect logs/results before repeating anything.
The next action is to obtain successful full Recipe-owned TUI acquisition and
compilation, fix actual failures, then verify readiness/reuse, the builder's real
source-preparation caller and both libc/product paths. Final security/guest/ISO/
installer/install/A/B requirements remain open; no publication occurred.

Actual frozen npm acquisition now completes all 98 packages under the normal
notification-corrected Ginux candidate with the explicit 32,768-object request
(`real-tui-npm-object-budget.log`, Bun exit success, 96.72 seconds). The diagnostic
Recipe invocation then correctly fails because its placeholder `is_built` always
returns needs_work; that fixture is not a successful install/publication receipt.
Do not hide or count that overall failure. The real TUI producer has hash/marker
readiness and is running the complete path. Its next result, not that diagnostic
phase alone, determines producer acceptance.

The full GNU producer with 32,768 objects completes its frozen 98-package acquire
and acknowledges its checkpoint, then compilation fails resolving transitive
packages under Bun's isolated linker layout. Native compilation of the same
source/lock/layout succeeds (567 modules, `tui-native-compile-control.json`);
that native output is diagnostic only and is never a product input. Pinned Bun
`src/resolver/lib.rs` opens symlink targets using Linux O_PATH in this path;
configured Ginux did not implement O_PATH. The user initially chose **use and
verify a hoisted TUI layout**. Subsequent evidence and authorization below
supersede that initial O_PATH exclusion; the hoisted layout remains selected.
The producer now explicitly passes `--linker=hoisted`; its full run is
`tui-system-producer-gnu-hoisted`. No success is claimed before it completes.
Workspace TypeScript configs are self-contained and already copied; the
speculative extra root-config input was removed before executing the caller.

### Authorized metadata path references and current producer checkpoint

Fresh full hoisted installation also fails all eight workspace links; a native
control succeeds with the same pinned source/lock. The smaller two-local-package
offline fixture reproduces the difference, and a separate bounded diagnostic
records O_PATH opens of the valid workspace directories (flags 2129920).
Receipts: `tui-system-gnu-hoisted-fresh.json`, `hoisted-local-managed.json`,
`hoisted-local-native-install.json`, and `hoisted-path-diagnostic.patch`.
Diagnostic changes were removed before the delivery implementation.

The user explicitly **authorized bounded configured O_PATH**: metadata references,
with data I/O, mmap and execution denied, existing public FD/filesystem limits
and native policy preserved. Local Ginux commit
`3cd1e2de49cfc16c0d22804849a82a5d24cf22d8` implements that immutable descriptor kind
through the existing descriptor/VFS owners. Only O_DIRECTORY, O_CLOEXEC,
O_LARGEFILE and O_NOCTTY may accompany O_PATH; O_NOFOLLOW and other combinations
remain unsupported. Relative metadata works; a metadata directory does not
authorize relative data open, exec or mutation. FD-path reopen, fchdir, mutation,
directory enumeration and status modification are denied. Dup/fork/exec lifetime,
close-on-exec and existing lower resource limits are retained. No catalog or
native-policy expansion is included. The source contract documents these limits.

`opath-final-source.json` binds the clean revision to launcher SHA-256
`fb87792d5b4363ca669d20935ce6f3d3f6d08fb97ee0cdc74e60bc324c200e6e`;
the exact committed rebuild matches the initial candidate byte-for-byte.
`opath-runtime-final.log` passes native common-subset, configured metadata,
denial, lifetime/copy-fault and lower-budget controls. All affected FD/task/exec/
script/poll/event/thread/network runners pass (`opath-affected-2.log`), as does
strict Clippy (`opath-clippy.log`). The first fixture expected EBADF from a
catalog-unsupported fchmodat; corrected to its actual earlier ENOSYS denial.
The first affected runner omitted building an exec fixture; the completed
second runner builds all fixtures. Both initial failures are preserved.

The offline complete Recipe install now passes (`hoisted-local-opath-root.json`,
two checkpoints). Its first launch lacked required coordinator privilege and
correctly failed chroot (`hoisted-local-opath.json`); the retry elevates the
whole coordinator/worker, with no launcher-only wrapper or host fallback.
The full fresh GNU graph `tui-system-gnu-opath` completes 106-package frozen
hoisted acquisition, 567-module compile, hash-checked publication and three
acknowledged checkpoints. Ginux exits zero with no remaining tasks/handles;
the coordinator is still completing durable filesystem writes at this
checkpoint, so final successful invocation acceptance waits for its receipt.
`tui-system-musl-opath` is running the separate fresh musl graph on the same
launcher/Recipe/image. No native cache or native compiled output is an input.

Exact next action: inspect both producer results (sessions 57151 and 11153),
verify successful install/readiness/reuse and the real builder source-preparation
caller for both libc targets, then complete both product chains and required
session-libvirt/PTY/install/A/B scenarios. Final Daybreak verification of these
new boundaries and the final combined source remains required. The Recipe caller
migration is still uncommitted until runtime acceptance. No publication occurred;
PRD15 and PRD19 acceptance remain open. Shared staged queue/handoff changes retain
SHA-256 `f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.


### 2026-11-09 returned security results and caller corrections

The user returned [Daybreak's exact-source receipts](/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/prd15/daybreak-2026-11-08/receipts.md)
for Ginux `3cd1e2de`, Recipe `e28a60ab`, launcher `fb87792d` and the pinned
CLI/worker/image above. Independent read-only manifest verification succeeds
**504/504**. Selected FD-name, own-stack, O_PATH, all 18 event modes, twelve
selected affected runners, the test-only T-01 oracle and focused persistence
controls pass. Two actual Recipe coordinator runs select the same eight endpoints
under reversed resolver ordering; four negatives reject before authority.
No candidate behavior failure was observed in those completed controls.

**All six requested items and S09-A–H remain partial.** Outstanding criteria
include synchronized lifecycle/parked-pin/revoke/reuse races; full path, flag,
copy-fault, quota and cleanup matrices; T-01's tracked port, T-02/T-03; reserved
clone nonleader peer death; actual close-time transfer/lifetime evidence;
remaining hostile persistence/cancellation schedules; paired launcher-failure
consumer rejection; lock/time/resource/identity matrices; transport, protected
inputs/autofix, TLS and graph-authority/security checks. Source-only consumer
rejection and synthetic skipped-close injection are not runtime ownership
proofs. The resolver's later one-address-per-host overflow branch was not reached
by nine-declaration negatives; its public reachability needs an explicit source
reconciliation. No execution authorization is transferred from Daybreak here.

Both standalone fresh GNU and musl TUI graph installs now have final exit-zero
receipts (`tui-system-{gnu,musl}-opath.json`), as do both fresh-coordinator
installed-cache reuse runs. The documented author and manual examples pass on
the same tuple (`documented-{author,man}-opath.json`). These remain standalone
producer/example receipts, not completed product/guest/security acceptance.

Actual product calls exposed two further integration gaps. First, product
preflight still required host Bun after migration; that obsolete requirement
was removed. Both initial preflight failures remain retained. Second, the real
source-preparation caller rejects checked-in fixture command symlinks even
though Recipe's immutable input/copy owners support confined links. Its local
correction preserves only relative links whose canonical targets remain within
the authored workspace member; it never follows links to import host files or
caches. Special files and escaping/absolute links still fail. It compiles, but
actual product completion remains pending. Acorn's first real caller fails at
that symlink; Levitate's existing package/tool build is still running.

Standalone GNU `isinstalled` also fails (`tui-system-gnu-opath-readiness.json`):
manual operations omit dependency products from the parent's input identity,
so the matching install state/files are not restored. This is an integration
failure, not a successful readiness receipt. The user chose **require matching
cached dependencies; never install them**. The local Recipe correction checks
cached dependencies in graph order with `is_installed`, imports their checked
products using the existing input owner, then checks the parent with the same
input identity. Missing/changed cached products fail explicitly without running
dependency acquire/build/install hooks. New managed CLI coverage includes both
runtime/build dependencies, all three checks, changed typed inputs, rejection
without hooks and preservation of the older matching generation. No unit test
is added. The correction is still uncommitted pending that actual regression.

The new provisional matching CLI/worker `bin/recipe-manual-cache-1` has SHA-256
`467140e9b036a8ed20b58f87020ad7a63cfe7c68d311dc8e19cfe51c464cf9ef`;
`recipe-manual-cache-1-source.json` and its patch bind the source. Daybreak's
Recipe results remain bound to e28a60ab/5ca87251, and do not automatically cover
this correction. Ginux source and its sealed binary remain unchanged.

Exact next action: run the new real dependency-cache regression on old and new
CLIs, resolve any failures, rerun affected NEEDS/dependency checks, then rerun
standalone producer/readiness/reuse and both real product chains with one final
matching CLI/worker. Continue both session-libvirt/PTY/install/A/B checks and
obtain outstanding authorized final-source security results before closure.
PRD15/19 remains open; no publication occurred. Shared staging is preserved.


### 2026-11-09 continuation: exact pair, retained tests and second Daybreak receipt

Recipe's manual-readiness correction is committed at `91068391` and clean. Its
exact committed release rebuild matches sealed CLI/worker `467140e9` byte for
byte (`recipe-manual-cache-final-source.json`). The old CLI fails the new managed
regression; the corrected CLI passes it and all **13 NEEDS + four dependency**
cases. Formatting and strict Clippy pass. The author/operator/interface/man
contract is updated; the manual renders without warnings. The user separately
chose **keep explicit dependent-recipe remove/cleanup unresolved for a separate
decision**. That known cache-selection gap is documented and not implemented.
Automatic cleanup inside install is unchanged.

The user returned [Daybreak's follow-up receipts](/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/prd15/daybreak-followup-2026-11-09/receipts.md).
Read-only manifest verification passes **743/743**. On the exact 3cd1e2de/91068391
pair, **20/20** selected whole-coordinator tests, four pre-authority endpoint
negatives and additional Ginux task/thread/TCP/pipe controls pass. A paired
runtime control supplies a complete actual Ginux archive plus authoritative
supervisor-error/nonzero exit; Recipe refuses final publication. Its retained
checkpoint is correctly distinct from a final product. The wrapper controls the
consumer decision; it is not a real post-link kernel-close error. Source review
resolves the later one-address-per-host overflow branch as publicly unreachable
under the eight-declaration limit. These two earlier gaps have concrete new
evidence; all six security items and S09-A–H **still remain partial**. Remaining
race, ownership, transfer, persistence, TLS, transport, resource/identity and
lease criteria in that receipt remain open. No security permission transfers here.

The retained-test gaps now have local ports without runtime changes:

- T-01: `03d4e00948e5df02d58e3cb19f1439dd4991d7c7` puts the exact previously
  test-only shared-offset/native-FD-reuse mmap oracle into the tracked FD fixture.
  Source SHA-256 `81092713ffccd80b852441b3ae5346f1ee5dea087934ffb61c365aeb5566ca8b`
  matches Daybreak's donor; the ordinary FD runner passes.
- T-02/T-03: `b81f30b52e523d142071603f477c6891c7fd9df9` retains the full
  9,000-byte patterned copy-up and error after 4,096, provisional slot recovery,
  and added preservation of an already-open image description's bytes/offset/
  metadata. It also carries all six upstream #234 orphan model cases verbatim,
  alongside the current aggregate tests. All **44 VFS and 25 process** units pass
  under the explicit Ginux-only unit-test exception. These are owner-model
  results, not new adversarial runtime/race acceptance.
- `.artifacts/prd15/functional-t01/retained-ports-receipt.json` in the Ginux
  worktree binds the source, tests and unchanged production prefixes. The exact
  b81f30b5 release rebuild still hashes to `fb87792d`. Prior direct-Ginux receipts
  retain their exact runtime code/binary identity through this test-only change.
  Final typed-storage/delivery joins still require their own reconciliation.

Fresh GNU and musl graphs both pass on the corrected 91068391 CLI/worker
(`tui-system-{gnu,musl}-manual-cache.json`), each with frozen 106-package hoisted
acquisition, 567-module compile, checked output and clean completion. Explicit
installed checks on these same destinations are running or completing under
`tui-system-{gnu,musl}-manual-cache-readiness`; inspect final receipts before
counting them. Actual Acorn source preparation now preserves confined fixture
links, completes managed acquisition and compiles/publishes its TUI workload;
coordinator/product completion is still pending. Both previous full caller runs
failed at the source-link rejection and remain failed. The older mutable-xtask
wrapper may also reject its changed executable hash; no pass is inferred.

Current product verification uses sealed `bin/distro-builder-manual-cache-1`
SHA-256 `4d5928b73c54c8d2a692fe3f31bd9a9b055977d4ef304abc1cfd1b582adebfc4`;
`manual-cache-product-sources.json` captures its source/recipe inputs. It calls
the actual builder grammar directly, equivalent to xtask's typed builder call,
so later Cargo rebuilds cannot mutate the selected caller executable. No host
Bun or host node_modules is imported. `base-rootfs-{acorn,levitate}-manual-cache`
are still running on the one sealed Ginux/Recipe/image combination. Their actual
pinned APK/RPM paths and prebuilt kernels are retained. On 2026-11-09 read-only
remote checks still report upstream main `53a54addea67969abaa240ea478def1b94130eee`
and maintained main `28f4368f6fd70d0e485386892a08d6734b5988dd`; no upstream/source
publication was performed.

Exact next action: inspect those two product and two readiness final receipts,
run successful fresh-coordinator TUI reuse, complete both product chains and
session-libvirt/PTY/install/A/B acceptance, and finish required exact-source
security verification through Daybreak. Preserve the open remove/cleanup decision
and final-source/delivery dependencies; no checkbox or partial receipt closes
PRD15/19. Shared staged queue/handoff SHA-256 remains
`f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.


### Current product/scenario continuation

Both real base products now pass on the sealed b81f30b5-runtime/91068391/image
combination: Acorn `A1MDzswoTkZc0NI6` (269 MiB ISO) and Levitate
`A1MEAGuorOMmL3Vr` (372.5 MiB ISO). Both include Recipe-owned system TUI
production and actual rootfs/EROFS/tool paths. Acorn's new live-boot parent
`A1MGcVH1xcurJ16u` also builds successfully. Standalone GNU and musl production,
explicit installed readiness and fresh-coordinator installed reuse all have
exit-zero receipts on this same pair.

The first operator-session Acorn live-boot attempt fails at QEMU's ISO open.
The file is mode 0644 and readable by the operator; QEMU-specific SELinux AVC
identifies `svirt_t` denial against `user_home_t`. This is not an ISO-byte or
kernel boot failure. Applying `virt_content_t` only to that public read-only ISO
allows the exact same image to boot, reach its live shell and pass SSH and
`live-boot.sh` (`live-boot-acorn-recipe-tui-selinux.json`). SHA-256 remains
`d0d87ac5ac8ed0d30c24aaf4241e4990f34ae08cfe781c8cd13fd68a4aa6d727`
before/after, with SELinux still Enforcing. No private Recipe permissions or
host SELinux policy is changed. The failed first receipt remains failed.
recqemu commit `f0364bb` corrects its false no-relabel documentation/comment;
builder documentation now includes this exercised operator preparation.

The complete current Recipe catalog is a single ongoing **152-case** run:
`complete-catalog-91068391-run`, sealed harness `bin/e2e-91068391` SHA-256
`089ceae2ed6366de396b9874cd369a5c73d9537103e5de0b0c13ec6046a2dfde`.
The enumerated names, lock hashes and final combination are retained in
`complete-catalog-91068391.json`; source/test artifacts are retained separately.
Do not infer completion before every case and the command finish. Separate
`library_backend` and `image_producer` integration harnesses pass on the same
pair, including the spoofed worker environment and reproducible image/failure
controls; `final-local-integration-harnesses.json` binds their actual binaries.
These are the user's explicitly requested existing integration catalogs; no
Recipe unit tests or extra adversarial security execution is performed here.

Acorn live-tools build is still running after successful managed installer-TUI
acquisition/compile/publication phases. Levitate's live-boot build has started.
Current sessions: 22052 (Acorn live-tools), 99936 (Levitate live-boot), 32215
(full Recipe catalog); each may already have completed by resume. Inspect their
final JSON/logs before repeating. The scenario harness is currently sealed at
`3adeecc26f50aeb421cea87b681b3b423dba8692304b56e4b37b9aae8e5420e3`;
rebuild/pin it after the recqemu documentation-only source change before final
full scenario acceptance. The actual builder is sealed at `4d5928b7` and its
migration remains uncommitted until product completion.

Next: finish both live-tools chains, apply the documented per-ISO SELinux type
where required, run both distros' actual PTY/install/installed/A/B scenarios,
and finish the single complete Recipe catalog plus remaining required checks.
The focused next security controls in `prd15-daybreak-followup.md` cover the
still-unverified live TLS, omitted endpoint and ambient resolver boundaries;
other required S09 criteria remain explicit. The user-held dependent-recipe
remove/cleanup decision and final delivery/source joins remain open. No release
or security completion, remote push, PR/comment or hosted job is claimed.


### Returned focused network evidence and current failures

[Daybreak's focused network receipt](/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/prd15/daybreak-network-2026-11-09/receipts.md)
independently verifies **131/131** manifest entries. It binds clean Ginux
`b81f30b5`, the unchanged `fb87792d` launcher, clean Recipe `91068391`, matching
`467140e9` CLI/worker and pinned `ffe04d86` image. All three requested controls
pass: actual untrusted TLS rejection under normal trust with UnknownIssuer,
ClientHello/unknown-CA alert and no HTTP or publication; selected TCP success
alongside EACCES for a listening omitted address; and immutable guest hosts,
absent ambient-name resolution and unsupported UDP alongside allowed TCP.
The complete Recipe audit reached its 64 KiB cap before the omitted connection.
A separate same-launcher/same-eight-pin Ginux invocation supplies the exact
connect-denial audit; it is explicitly a separate launch. No protocol inspection
or prohibition of DNS bytes over an explicitly granted TCP endpoint is claimed.
These bounded S09-C/H outcomes pass; all other material security gaps and the
final-source join remain open. No Daybreak execution authority transfers here.

Acorn's current `cargo xtask test all acorn --force` run has a final failure
receipt (`scenarios-all-acorn-current.json`). Its base preflight, live boot and
live tools pass, including all 44 tools and the actual guest installer PTY
interaction retained in scenario `A1MMZlWKYe8asr18/install-pty.log`. Installation
`A1MMe0fpKdpN0Ci2` finishes the shipped partition/rootfs/fstab/setup steps and
install evidence, then exceeds the combined **30-second `sync && umount`**
command budget. Installed boot/login/runtime/A/B are **not run** in that batch.
The corrective harness separates the flush (existing 900-second disk-stage
budget) from unmount (existing 30-second command budget), preserving both error
checks and making the stalled operation observable. A rerun must establish
actual successful flush, unmount and clean shutdown; this change alone is not
an installation pass.

The ongoing single 152-case Recipe catalog has reported a kernel-readiness
failure: source `linux` now correctly requires its `linux-deps` cached installed
products before a manual check; the old synthetic kernel fixture did not prepare
them. Its missing `flex` failure is preserved. The production dependency check
must remain enforced. The fixture needs explicit prerequisite preparation while
retaining release/config/modules/input-preservation assertions. The complete
catalog remains incomplete/failed, regardless of other passing cases. Separate
library_backend and image_producer passes remain exact-source selected results.

Root package-manager alignment is committed at `9c9a46e`; builder Recipe-owned
TUI migration is `34172e7`; recqemu documentation correction is `f0364bb`.
Levitate live-tools is still producing real tool payloads. No push, upstream
PR/comment, hosted run or publication occurred. Explicit dependent-recipe
remove/cleanup remains unresolved by user direction. Next: validate the isolated
flush correction with the complete Acorn scenario set, finish Levitate live-tools
and both complete scenario/A/B sets, repair the stale kernel integration fixture
and rerun the exact complete catalog. Obtain remaining required security results
through Daybreak before any acceptance claim.


### Overnight continuation: returned criteria, CLI access and product corrections

The returned [170-file Daybreak criterion map](/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/prd15/daybreak-remaining-2026-11-09/receipts.md)
independently verifies **170/170**. Twelve thread modes, two teardown modes,
eleven task modes and configured zero-sysinfo/virtual-uname controls pass. The
prior exact event receipt already proves actual TCP EPOLLRDHUP; its earlier
unverified label is corrected. All six items and S09-A–H remain partial, with
precise remaining owner/callsite dispositions in that map.

The user authorized running our own Codex CLI with Daybreak's model choice.
Read-only original turn metadata identifies `gpt-6-sol`, high reasoning and
`daybreak_blue`; the CLI requested those exact settings with automatic approval
review and normal sandboxing. It exited 1 with the server error: “You're approved
for Daybreak Blue, but this response cannot be shown as this session was not
authenticated with one of your authorized hardware keys. Please re-authenticate
with your hardware key and try your request again.” The user has a pending
reauthentication/original-context-results request. No standard-access fallback
is authorized or attempted. The interrupted CLI's request/events/partial work,
authentication error and parent accounting are hashed under the Ginux worktree's
`.artifacts/prd15/daybreak-cli-2026-11-09/`; no complete verification is claimed.
No task-owned Ginux subprocess remains after its termination.

All six current base/live-boot/live-tools ISO builds now have exit-zero receipts.
Levitate live-tools is `A1MM58XBtv4kcuhh` (408.7 MiB). Both complete Acorn and
Levitate scenario batches ran on the sealed runtime pair. Acorn installation
passes with separated flush/unmount (`A1MN4xhMaeJN5nNS`); login and 40 installed
tools pass. Installed boot fails on crashed recab-trial, and A/B's degraded trial
fails to reboot. Levitate installation/installed boot/login/runtime and the
seven-boot A/B update pass individually, but live-tools fails because the initial
PTY driver requires unshipped guest `script(1)`. Its batch is failed. Rebuilding
the mutable normal harness while that already-failed batch finished also means
its outer before/after harness guard cannot establish a final batch identity;
no complete same-harness acceptance is claimed. New diagnostic drivers use sealed
harness copies. Final whole scenario sets must run after all corrections.

The PTY driver is being corrected to use the existing SSH transport's real guest
PTY (`-tt`), retaining terminal output and independent guest-written file checks.
No guest package/lock change is needed. A first portable attempt wrongly expected
a contiguous success string through terminal repainting; its failed transcript
is retained. The second uses a separate real SSH file observation. It must pass
both distros before the harness changes are accepted.

Actual Acorn session-libvirt inspection confirms a production recab defect:
OpenRC 0.63.2 `rc-status --crashed` returns **1 with empty output when none
crashed, 0 with names when there are crashes**, as its [pinned manual](https://raw.githubusercontent.com/OpenRC/openrc/0.63.2/man/rc-status.8)
also states. recab incorrectly rejected the healthy result, killing its trial
service. The asynchronous one-shot also needs to mark successful completion
inactive rather than leave a finished daemon marked crashed. Local corrections
preserve unexpected status/diagnostic errors, actual failed-service reporting
and shutdown cancellation. Both affected real recab CLI integration cases pass;
the corrected static tool/service is now building through the actual managed
Recipe producer (`recab-openrc-managed-producer`), before guest and product retest.
The diagnostic guest powered off; it was an overlay, not an install mutation.

The first complete Recipe catalog is **149/152**, with all 152 cases run and
three failures (no skipped/interrupted pass claim). The kernel fixture's explicit
managed dependency preparation now passes all original release/config/modules
and preservation checks, and verifies that manual checks do not install it.
Its first correction mistakenly disabled persistence; that failure is retained.
The target-metadata case passes when its required actual sealed builder and repo
are explicitly selected. The integrity fixture additionally needs both imported
desktop lock modules and a valid `REC_FLAVOUR=base` control input before it can
reach corrupt-byte rejection. The current focused retry is
`preseed-fixture-correction-4`; no production recipe validation is bypassed.
After all affected cases pass, commit only the test corrections and run the
entire enumerated 152-case harness again on one exact source/binary/image tuple.

Next: inspect the managed recab producer and portable PTY results, complete the
remaining integration fixture check and whole catalog rerun, verify recab on real
Acorn boot/trial plus both final scenario sets and explicit A/B commands, and
reconcile delivery sources. Security waits for hardware-key authentication or
returned original-context results. Dependent-recipe remove/cleanup stays a
separate unresolved user decision. No push, PR/comment, hosted run or publication
is authorized/performed by these corrections. Shared staging remains preserved.


### Current exact checkpoint after functional corrections

- Recipe is clean **`85bba068e95b0709a7df53624c48ab2bc0b41ece`**. Only the two
  integration fixtures differ from 91068391; `src`, Cargo.toml and Cargo.lock
  are identical. The original full run remains **149/152 failed**. Its three
  findings now have passing affected receipts: `fixture-corrections-3` passes
  kernel readiness and target metadata; `preseed-fixture-correction-4` passes
  corruption rejection for both real rootfs consumers with their complete
  imported closure and base-flavour input. No production check is weakened.
- A new complete **152-case** run is active under `complete-catalog-85bba06-run`.
  Its sealed e2e harness is `bin/e2e-85bba06`, SHA-256
  `d365c3226df80400271487a9f1414046e55ac49b92458b06ab03b4a3a2d35933`.
  `complete-catalog-85bba06.json` retains every enumerated name, both lock
  hashes, the sealed builder and a frozen/hash-inventoried production recipe
  closure at `catalog-85bba06-recipes/`. The wrapper verifies harness/builder
  and all runtime bytes before/after. This run is **not yet complete**.
  Fresh library_backend/image_producer checks on the same source are also
  active, with `final-extra-harnesses-85bba06.json` binding their actual bytes.
- Portable actual installer PTY checks now pass **both** current products:
  `live-tools-{levitate,acorn}-ssh-pty-2.json`. They use SSH's actual guest PTY
  and separate SSH reads of shell-written files; no extra guest package is
  required. recshuttle `dfe9ef1` and install-tests `d55c089` commit those working
  owners and the verified install-flush stage. A final two-line host timeout
  bound is still uncommitted and will be included in final scenario reruns.
- recab is clean **`61b859c4b0bec5337b6dced96d033a8847d966ce`**. All **24**
  real CLI lifecycle integration cases pass (`recab-lifecycle-final.log`).
  `recab-openrc-managed-producer-2.json` passes actual static musl compilation,
  package/service generation and publication through Recipe on the pinned tuple.
  In a real disposable Acorn session-libvirt guest,
  `recab-openrc-guest-control.json` passes successful asynchronous completion to
  OpenRC inactive, actual health `running` with no failed services, and stop
  cancellation while a startup marker holds the trial: helper and child both
  disappear. This corrects both observed faults without ignoring failures.
  `recab-openrc-final-source.json` binds its exact committed source and receipt.
  This was a diagnostic guest update, **not** acceptance of a rebuilt ISO.
- `run-corrected-products.py` is executing the real base-rootfs, live-boot and
  live-tools producers for Acorn then Levitate with the new recab source. Labels
  are `<product>-<distro>-recab-final`; inspect final JSONs before counting them.
  The diagnostic Acorn guest powered off; SSH's expected connection reset is
  not a clean interactive-CLI exit and is not counted as a scenario pass. No
  domains remain. Final whole scenario sets and both explicit A/B commands must
  consume the new products, a final sealed harness and current PTY script.

Active sessions at this checkpoint: **55290** complete catalog; **60000** both
corrected product chains; **69378** fresh library/image integrations. Inspect
logs/JSON before restarting anything. Runtime remains Ginux b81f30b5 / launcher
fb87792d, matching Recipe CLI/worker 467140e9 and image ffe04d86. No runtime binary
was changed for the fixture-only Recipe commit. Security remains blocked on the
pending hardware-key reauthentication or original-context results; do not retry
Daybreak with standard access. Its interrupted CLI evidence is preserved without
acceptance claims. The user-held dependent-recipe remove/cleanup decision and
actual source-delivery joins remain open. No remote publication is authorized.


### HTTP startup finding and completed six-consumer replay

All six current ordinary consumers have final exit-zero receipts on the same
fb87792d / 467140e9 / ffe04d86 tuple. `current-consumers-exact-source.json`
reconciles their identical runtime identities; `consumers-85bba06/SHA256SUMS`
binds 161 retained files. SQLite contention, concurrent GNU/musl shared Cargo
cache/target locking and built threaded execution, pinned Git HTTP/HTTPS,
actual RPM bytes/metadata/links and signed APK installation/execution pass.
The separate consumer report preserves original and current source bindings;
no delivery or security acceptance is inferred.

The 85bba06 complete catalog reports three HTTP fixture-startup failures:
interrupted download, resumed download and digest verification. Each server
exits its 120-second pre-connect wait under concurrent whole-image preparation.
The retained CAABGH worker result records an actual permitted endpoint attempt,
not connected, with ECONNREFUSED after that listener exit, and successful prior-
destination cleanup. It never exercises the intended truncated response. This
is not counted as an interruption-preservation pass or silently retried away.

Recipe **3828500** changes only the integration fixture: allow up to 600 seconds
for coordinator/image startup before connection, retain the five-second HTTP
read timeout and all production deadlines/limits, and require/capture the actual
GET before accepting interrupted-download preservation. The focused case passes
all four prefix/resume combinations in 179.76 seconds. Its exact source and
harness are bound by `http-fixture-startup-source.json`; failure evidence remains
in the old full run. No application download timeout or integrity assertion is
weakened. All source/lock/build inputs outside the three fixture files remain
identical to production 91068391.

The old full catalog is allowed to finish to retain other possible findings.
`run-final-catalog-after-failed-run.py` then requires all 152 cases accounted for,
unchanged selected binaries/image and **only** those three diagnosed failures
before starting a fresh complete 152-case run on sealed 3828500. Any different
failure stops that scheduler for review. No old pass is combined into the new
full-suite claim. The new source/harness/name/recipe-closure inventory is
`complete-catalog-3828500.json`; session **65760** awaits the old final receipt.

Both corrected ISO chains and their queued exact-harness whole/explicit-A/B
scenarios remain incomplete. Current author/man/public examples are also being
rerun under `documented-current-*`, retaining fresh checks/reuse and actual
produced-program execution. The security hardware-key question, explicit remove/
cleanup decision and upstream/source-delivery dependencies remain open. No push,
PR/comment, hosted job or security-execution fallback occurred.


### Completed documentation checks and corrected Acorn base

All **16** `documented-current-*` commands finish successfully on the sealed
runtime tuple. `documented-current-results.json` and
`documented-current-source-map.json` retain the exact current author/man example
blocks, public hello/threaded recipes, install/readiness/fresh reuse and static
validate/inspect/explain/diff results. All three executable products print their
expected output; threaded compilation/execution also occurs inside the managed
build hook. The man example publishes its declared plain file. Exact 3828500
formatting and strict `clippy --all-targets -- -D warnings` pass; no unit tests
are run.

Corrected Acorn base-rootfs **A1MQo3zVHViKU8C4** passes
`base-rootfs-acorn-recab-final.json`. ISO SHA-256 is
`fd00afc0e8602498dbee331316acfd189530079e0489db738f7a402423220ff4`.
`acorn-corrected-base-source-proof.json` verifies its real recab program
`6d2971b926546ec215a8ff1bb6a62557f6ccc2615a096183a93b7a15e7ff95c6`
and service `80ccf52a48656885160df32fac3a1e77cc85709f9b50ebb6bcae947d166fb6d0`
match the actual Recipe producer and successful guest service/cancellation
control byte-for-byte. Live-boot assembly has started; the other five final
products and all final scenario acceptance remain incomplete.

The quiet coordinator was observed waiting in host XFS `xlog_wait_on_iclog`
inside fsync; `product-fsync-observation.json` retains its own stack/I/O data.
It completed naturally with no durability operation skipped. There is adequate
artifact storage. Before **any** final scenario command had started, the empty
waiter was rescheduled so final guest acceptance follows all six producer runs
and the corrected complete catalog. `final-scenario-scheduling-change.json`
records that no test case was interrupted; all guest budgets/assertions and
source checks remain intact. New waiter session **55602** runs
`run-final-scenarios-after-heavy-io.py`; source bytes stay sealed at dc43b8da…
and PTY-script SHA-256 6c0d4c1b…. Do not rebuild the normal cargo xtask executable
during those eventual commands. Scheduler 65760 separately guards the new full
catalog against any unreviewed old-run finding.

Next: finish producer/catalog runs, inspect final receipts rather than inferring
success, complete both new whole scenario sets plus explicit A/B commands,
retain the tested host timeout bounds and reconcile final delivery. Security
still needs hardware-key authentication/original-context results; remove/cleanup
and actual upstream/source joins stay open. No publication occurred.
