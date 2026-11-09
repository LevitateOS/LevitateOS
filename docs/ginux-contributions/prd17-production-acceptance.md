# PRD17 — production acceptance is incomplete

Recipe is **not production ready**. Audit in progress against selected Ginux
e4d95de1 / launcher 21a54e60, Recipe 16141938 / CLI-worker e3e28457,
image producer 55b9e50a and acquisition image ffe04d86. The two corrected owning heads are still local; delivery-sources.json and
delivery-provenance.json retain the previous public selection and products.
They must not be relabeled as acceptance of this new combination.

Active skill: hdae-development, project discovery path
.agents/skills/hdae-development/SKILL.md. Reload the complete skill, required
reference, root instructions and this checkpoint after context recovery. Root
rules prohibit mandatory process gates and Recipe unit tests. Existing managed
CLI, producer and product owners provide verification. No subagents requested.

## Current findings and decisions

- Nine authorized owning-source pushes are complete; anonymous public acquisition
  passes 16/16. Root pin commit 89863d5 recursively obtains those sources and
  passes locked metadata. Exact parent publication request 1b2d96f remains
  unanswered; PRD17 is not that permission. Audit corrections create a later
  candidate, which needs its own exact source/delivery reconciliation.
- Ginux e4d95de1 hosted run 42609102747 succeeds, including the required Linux
  job and compiler FD/stack/O_PATH/event controls nested in its existing make
  targets. macOS remains deferred; no macOS investigation.
- Recipe b185e036 run 42609021436 fails managed CI before execution: all four
  runtime variables are empty. Hosted build/fmt/Clippy/MSRV and independent
  stable/MSRV unlocked resolution pass; release is skipped. No managed pass.
- docs-content d938493 run 42609031135 fails frozen installation: standalone
  bun.lock lists removed Biome/React/Vite dependencies and misses @types/bun.
  packageManager is absent, so setup selected Bun 1.4.4 instead of pinned 1.4.2.
  Correct in its owner, retain frozen-lock enforcement and verify independent
  install/typecheck/build with unchanged authored content.
- Upstream main remains 53a54adde. #232/#233/#234/#236/#238/#243 and #249 remain
  open/unmerged. PRD05 must wait for actual merges/replacement foundation; the
  maintained-source choice does not waive this explicit completion criterion.
- Final-source Daybreak results/authentication remain missing. The authorized
  gpt-6-sol/high/daybreak_blue CLI was rejected for missing authorized hardware-key
  authentication. Do not retry unchanged, use an ordinary-model fallback or run
  the adversarial matrix in the parent. Prior partial receipts remain partial.

The user resolved two additional decisions during this audit:
1. Hosted managed CI must build its image with tracked public bootstrap producers
   and pinned inputs, checking the expected digest. Do not upload a substitute
   image release or weaken input/trust validation. New source publication and
   hosted dispatch still need exact permission.
2. Explicit dependent-recipe remove and cleanup must require matching cached
   dependency products, validated with installed checks, and never run dependency
   install hooks. This supersedes the former held decision. Implement in the
   existing backend, migrate documentation and retain real CLI controls covering
   nested dependencies, cache/input misses and prior-state preservation.

## Implemented corrections and current verification

Recipe functional commit ef07382 implements the user's cached-only dependent
remove/cleanup decision in the existing backend. Every non-install dependency
uses installed checks and its declared products; neither operation installs
missing dependencies. The retained same-harness old-source red/new-source green
control covers nested build dependencies, changed inputs, preservation before
parent hooks, correct saved state/files and durable remove/cleanup effects.
The first unprivileged red attempt could not chroot and is not behavioral
proof. An initial fixture incorrectly used RUNTIME inside a helper; its failure
is retained and the fixture now uses ctx.runtime and explicitly restores its
published proof. The earlier debug NEEDS group is 13/14, not a full pass. The
corrected focused control passes in 168.78 seconds. Final fmt and strict Clippy
pass without Recipe unit tests. See lifecycle-fixed-fixture-red-green.json,
lifecycle-final-clippy.log and the retained failed-attempt receipts in prd17.

Clean Recipe master **16141938f5b82e09b93051cd4fab87fde4642e68** adds the tracked
public-producer managed CI path. bootstrap/managed-ci.json pins public Ginux,
OCI/RPM inputs, publisher key and image digest. Its wrapper verifies every input,
uses a private ordinary RPM trust database, runs the real image producer and
checks ffe04d86. On enforcing SELinux the wrapper labels only its temporary RPM
database using the normal system RPM database as context reference. The failed
unlabelled attempt is retained; no host policy or trust store was changed.
The official binary Rocky key verifies the same pinned fingerprint and all
twelve RPM signatures. Corrected local image production and wrong-input/
existing-output failure-preservation controls pass, as do all 15 public input
URL checks. That local image receipt used the previous byte-pinned aa13ac9b
producer; final 55b9e50a producer composition still requires its own receipt.
Hosted execution of the corrected CI is not yet available.

Clean docs-content master **bb3a110232d501b25efce53c1d6ac2aaad71d104** pins Bun
1.4.2, corrects its standalone frozen lock, generates content before typechecking
and documents the resolved remove/cleanup rule on both relevant pages. The user
approved the exact locked npm downloads. Independent frozen install, generation,
typecheck, lint and format pass; retained original failure is not suppressed.
See docs-complete-validation.json. Actual root generated content must be rebuilt
through its owner before the new TUI/product wave.

Both full source diffs and their exact approval scope are in
.artifacts/ginux-contribution-preparation/prd17/correction-publication-request.json.
**Permission for these two exact source pushes and automatic CI is pending.**
No source push, upstream action, parent publication, tag or release is inferred.
The old parent 1b2d96f request no longer describes the final corrected candidate.

The clean exact 16141938 standalone archive builds with Rust 1.99.0, its tracked
lock, a fresh target and normal release profile. Frozen outputs are CLI/worker
**e3e2845715179ae0c7de78846ca6649d1400bb4d256cf05391d60f42ee937292**,
image producer **55b9e50a2b88fa78151fbc8123ea35ad292b912391eccee7c178c08c8caea5eb**,
and the exact library/image/e2e harnesses in final-runtime-build.json. The source
and executable targets under /tmp/prd17-final-source-5cyxm5bc must remain while
the harnesses run; image_producer uses Cargo's actual producer path.
Builder and scenario harness were rebuilt against this Recipe library without
source changes; final-consumer-build.json binds their bytes.

The existing scripts/managed-integration.py is running the complete current
**155** cases (153 e2e, library_backend and image_producer), full-products enabled,
against unchanged Ginux e4d95de1 / 21a54e60 and image ffe04d86. Active session
**77137**; inspect full-runtime-driver.log and eventual full-runtime/results.json
and integrity.json before starting another catalog. No complete new-suite pass
is claimed. Keep Recipe source and its actual target binaries frozen; do not
combine the old 154-case pass with the new regression. Heavy product/VM I/O
follows the catalog to avoid known host fsync contention.

## Current audit dispositions and new findings

The first frozen 155-case run has two recorded kernel fixture failures. Manual
remove/cleanup now correctly requires the linux build dependency, while the old
controls never prepared it. The production prebuilt-kernel release path has no
such compiler dependency. An isolated fixture correction prepares a disposable
compiler product through an actual public graph install, then uses cached-only
manual operations. Both affected controls now pass: imported kernel/source bytes
survive removal, and the cleanup control reaches the actual unsafe-path guard.
No compiler acquisition, kernel rebuild, dependency-policy weakening or product
code change is involved. The first correction prototype misspelled the existing
build_deps declaration; its failed control is retained. The first prototype
build inherited Ginux's cwd Cargo linker configuration; rebuilding from the
standalone source with ordinary flags passes. These are fixture/build attempts,
not newly observed production behavior defects. See kernel-fixture-correction-v2/
receipt.json. Proposed source is kernel-recipes-proposed.rs. Wait for session
77137 to finish before applying it to Recipe; preserve that failed run's frozen
source/integrity. Then commit the test correction and rerun all 155 on one exact
resulting source/harness combination. Do not combine the two focused passes
with the failed full run.

Historical R-01/R-02/R-03 are traced to current owners, not closed from old
checkboxes. rootfs-producer.rhai no longer has its quoted shell copy calls;
Fedora link/sysusers commands use a root derived from the fixed domain /out/rootfs
and literal suffixes, not caller host paths. domain.rs, inputs/root.rs and
needs binding stage inputs and outputs under fixed roots. The existing quoted-
host-path preseed control remains required in the current catalog. The source-
kernel/linux-deps shell callers still have unescaped arbitrary path construction;
kernel rebuilding/toolchain acquisition is excluded from this release, and this
limitation remains recorded rather than presented as a repaired general shell
API. Checked argv helpers still own supported arbitrary arguments/cwd.

R-02's selected source explicitly assigns scratch leases to the coordinator
through launcher teardown. Ginux intentionally closes inherited duplicates;
PDEATHSIG and ptrace exit-kill own death cleanup, rather than a new unbounded
inherited-FD exception. The exact current coordinator-death CLI control passes
concurrent active-scratch protection, five-second launcher/worker death, next-
invocation abandoned-scratch reclamation and unrelated-file preservation, with
no observed process/scratch leak. R-03's dedicated stalled-Git control passes
bounded teardown and destination-lock reuse. These ordinary receipts establish
their stated assertions; outstanding security interleavings remain S09 criteria.

The current reciso FAT command gives fresh equal-sized inputs different volume
IDs (iso-fat-reproducibility.json), and ISO dates are not pinned by run_xorriso.
PRD16 explicitly did not claim byte-identical ISO rebuilding. The user has been
asked whether this release must now establish byte-identical ISOs or explicitly
exclude that property while retaining public-source/input/build provenance.
**Answer pending; do not choose the scope or start dependent ISO corrections.**
Ordinary runtime and caller verification can continue.

## Release criterion reconciliation

Every row is bounded by its original source-specific report and the current
runtime combination above. A historical completed preparation/scope decision is
not a claim of final combined acceptance. The latest read-only upstream refresh
is prd17/upstream-refresh.json: main remains 53a54adde and all seven observed
PRs (#232/#233/#234/#236/#238/#243/#249) remain open and unmerged.

| Task | Present evidence/disposition | Remaining completion dependency |
| --- | --- | --- |
| PRD01 maintained source | Authorized public Ginux e4d95de1 and previous Recipe b185e036 obtain/build independently; local corrected 16141938 reproduces e3e28457/55b9e50a. | Publish only exact approved corrections; obtain final selected parent/source from public URLs and build final worker/image combination. |
| PRD02 authority contract | Versioned configured catalog 1002 preserves upstream catalog allocation/native policy; earlier complete enforcement/denial ownership report retained. New backend uses each node's own declaration and caller narrowing. | Reconcile against any actual upstream replacement; final security/source join remains PRD05/09. |
| PRD03 storage candidate | Complete retained feature/prerequisite review and candidate-specific checks remain valid preparation. | Actual prerequisite merges, main refresh and authorization before any upstream contribution. No draft is called standalone against an absent foundation. |
| PRD04 recorded failures | Frozen ABI/FD/prefix/backing fixes and authorized export retest have explicit source-bound closure; link remains commit point with authoritative failure rejection. | Final joined-source security belongs to PRD09; do not promote old selected tests. |
| PRD05 merged-source integration | All required merges still absent; no replacement owner or test retired. | **Wait for actual upstream merges** and inspect/adapt their behavior owners before semantic integration/replacement map. Downstream checks do not substitute. |
| PRD06 full accounting | Original exhaustive hunk/test ledger and later native SIGCHLD correction accounting retained; T-01/T-02/T-03 ports exist in current tracked tests. R-01/R-02/R-03 dispositions are explicit above. | Source-dependent security/native limits stay named; rejoin on actual upstream foundation rather than replay donor stack. |
| PRD07 contributions | Existing complete waves/diffs/drafts retained; required configured behavior is available in authorized own-repository Ginux e4d95de1. No new upstream publication. | Independent/extraction wave boundaries still wait for their actual foundations; unchanged TCP wave remains local by user choice. No upstream PR/comment permission requested again. |
| PRD08 public contracts | Configured clocks and accepted per-process VA, real CPU IDs, numeric endpoint, zero-sysinfo and bounded poll contracts retained; native policy not expanded. | Final-source security acceptance and any future upstream/native maintainer decision remain separate. No new user clock hold. |
| PRD09 security | All returned partial receipts retain exact identities and limitations. Latest release request now binds e4d95de1/21a54e60 +16141938/e3e28457 +ffe04d86 and resolved cache-only lifecycle rule. | **Authorized Daybreak hardware-key authentication or actual final-source results**, criterion-by-criterion races/close/transport/resource dispositions and affected retests. No parent security fallback. |
| PRD10 complete catalog | Current 155-case run frozen at exact 16141938; two mismatched kernel fixtures fail; isolated corrected controls pass. | Commit fixture correction after frozen run ends, then one complete successful exact-source 155-case replay with intact source/artifact/resource receipts. |
| PRD11 real consumers | Historical six-consumer receipts remain on their own tuple. | Run all six current consumers on final e3e28457/21a54e60/ffe04d86; retain locks, normal threading, trust/metadata/isolation. |
| PRD12 lifecycle/storage/metadata | Current coordinator-death and stalled-Git controls pass with actual cleanup accounting; older large-storage/metadata guest receipts remain historical. | Complete current catalog, exact target EROFS/deployment and failure preservation, no missing/interrupted outcome or resource leak. |
| PRD13 coverage/CI | Ginux e4d95de1 required hosted Linux succeeds. Corrected Recipe CI builds tracked public image inputs; docs standalone frozen install/build/typecheck/lint/format pass. Corrected Recipe local locked MSRV and disposable stable/MSRV unlocked checks all pass. | Corrected automatic hosted Recipe managed job and docs CI after exact permitted pushes. macOS indefinitely deferred. No dispatch inferred. |
| PRD14 bootstrap/package | Public pinned image inputs/publisher signatures/ordinary trust remain verified; corrected CI wrapper reproduces ffe04d86 with aa13ac9b and failure preservation. | Final 55b9e50a image production, final crate/binary/RPM packaging and fresh selected-source/execution joins; do not relabel baseline VM/archive receipts. |
| PRD15 callers/docs | New caller binaries built against Recipe161; cached-only docs corrected and actual docs exports regenerated/typechecked with Bun1.4.2. | Run current documented commands and three complete embedded examples on final bytes; product/TUI producer and final operator joins. |
| PRD16 delivery/provenance | Previous nine authorized source pushes and 16/16 anonymous source acquisition complete; root pin89863d5 reproduces that prior selection. | Two new exact source permissions/availability, final corrected parent pins/provenance and separately authorized exact parent publication. Preserve shared staging/worktrees. |
| PRD17 production acceptance | This checkpoint owns current finding dispositions, exact runtime, missing evidence and publication boundaries. | All required rows/outcomes satisfied, outstanding decisions resolved and user accepts supported production scope. **Not production ready.** |
| PRD18 NEEDS | Full inclusion remains binding; implementation and caller migration present. Cache-only check/remove/cleanup selection now has explicit user policy and passing focused real CLI proof. | Complete new declaration/dependency/output/authority catalog and final security/delivery/product join. No deferred NEEDS scope invented. |
| PRD19 products | Both distros base/live-boot/live-tools, Recipe-owned TUI, real installer PTY/install/installed boot and A/B remain included. All earlier six products and guest scenarios retain exact historical receipts. | New affected products/guest checks on final sources; ISO byte-determinism decision pending. Kernel rebuild/general ABI/AI-provider interoperability remain excluded. |

The publication package currently contains the two complete exact owning-source
patches and their permission request. After the test-only fixture correction,
record its complete additional diff and source join before requesting any new
exact ref. Earlier approval requests are not standing permission for later heads.
No final tag, package/ISO release, upstream PR/comment, root push, repository-
variable write or separate hosted dispatch is authorized. Missing acceptance
is not resolved by preparing a publication draft.

## Exact next actions

Finish the exact 155-case replay and inspect every case/resource/integrity result.
Then run affected consumers, current examples, six real products, both distro
whole scenarios plus explicit A/B, actual metadata deployment, final packaging
and fresh execution on the new combination. Reuse only receipts whose exact
source/input owners remain valid. Reconcile historical R-01 shell paths and
R-02 leases against actual admitted paths and current owners; retain missing
security races rather than promote selected controls. Update the PRD01–PRD19
replacement/acceptance map and concrete publication package. If the two exact
pushes are approved, recheck public bases/ancestry before mutation, verify public
availability/automatic CI, then prepare a separately authorized exact parent.
Wait for actual Daybreak authentication/results, required upstream merges and
final user acceptance. Recipe is not production ready.

Read-only source/CI metadata and selected logs are retained under
.artifacts/ginux-contribution-preparation/prd17/initial-evidence.json and
{recipe-failed,docs-failed,ginux-linux}.log. The first attempt to save excessive
raw GitHub content exceeded the OS argument limit and wrote nothing; trimmed
exact metadata and complete selected logs were then saved in bounded chunks.
Original shared staged patch f4d8c81b remains protected. No existing human-authored
public text is edited.
