# PRD15 / PRD09 — corrected optimized-release verification

Security acceptance is open. This request supersedes only the runtime identity
in the earlier [criterion continuation](prd15-daybreak-followup.md); retain every
material outstanding S09-A–H criterion, prior receipt and user decision there.
The active [product checkpoint](prd15-product-contract.md) owns ordinary progress.

## Exact candidate

- Ginux source: `e4d95de10520aa8e91bc2ec69e64da004443e2da`, checkout
  `/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11`.
- Launcher: `/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd15/bin/ginux-native-sigchld-1`,
  SHA-256 `21a54e60bd54a4dd2b07957cf2e139a268eb050cb73929e0b8666b31f60c2333`.
- Compiled Recipe source: `176395aeaa5bb8999c1a6289d18e53c8012ecdf9`.
  Current clean checkout `/home/vince/Projects/LevitateOS/tools/recipe` is
  `b185e0363ceba1290201ba84593aefb313a85cc4`: only README and WRITING_RECIPES
  changed after the frozen run. `prd15/author-documentation-join.json` records
  unchanged Rust/Cargo/test/runner inputs. A subsequent fresh exact b185e036
  archive build with tracked lock and Rust 1.99.0 reproduces the identical
  e7073fc1 CLI/worker and aa13ac9b image-producer bytes;
  `prd15/maintained-recipe-build-corrected.json` retains the complete hashes and
  command. Production inputs equal 91068391; Rust tests equal 3828500. Retain
  actual run/build identities rather than relabeling earlier execution or
  requiring the checkout to undo the documentation correction.
- Matching optimized CLI and worker:
  `/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd15/bin/recipe-release-176395a`,
  SHA-256 `e7073fc17ec0bb029927d0c2e901ca37e0867a7cfdc4c5641d1d6c9b14bb5c03`.
- Acquisition image: `/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd14/ginux-acquisition-image/rootfs.tar`,
  SHA-256 `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`.
- `GINUX_ROOT=oci:recipe-acquisition@sha256:ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15`.

Later producer/documentation joins do not change these runtime bytes: builder
9468d26 removes only forced linker `--threads=1`, verified by an actual managed
two-CPU build yielding the same 13f6a3da recstrap as both metadata guest controls.
Recipe b185e036 and docs/content d938493 correct author/operator pages. The
current source guards are `prd15/normal-linker-combination-sources.json`;
`normal-linker-recipe-join.json` and `embedded-documentation-join.json` identify
all changed inputs. Original full-catalog source/fixture receipts remain intact.
The new six-product/guest wave is ordinary verification, not security acceptance.

The complete source/build/harness map is
`/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd15/release-runtime-candidate.json`.
Original fb87792d, b81f30b5 and all older pinned PRD04 candidates remain preserved.
Do not overwrite a pinned source/binary or use earlier bytes implicitly.

## New observed correction

The existing Linux native-supervision CI case failed after permitted SIGCHLD
with `unexpected native syscall-exit stop`. Gate 11 resumed with PTRACE_SYSCALL
while no mediated exit was pending, producing an entry stop before seccomp.
The correction selects PTRACE_CONT in that state and retains PTRACE_SYSCALL when
an exit is pending. No grant, profile, catalog or native policy was widened.
The retained userspace-signal regression fails on old fb87792d and passes on the
new launcher; original blocked-poll and all other native controls also pass.
Inspect the complete three-file b81f30b5→e4d95de1 diff, affected dispatch/denial
and lifecycle behavior on the final bytes, and disposition of unaffected paths.

Current CI evidence is under the Ginux worktree `.artifacts/prd15/current-ci/`:
old/new native controls, complete static checks, 147 unit tests, 20 Linux runtime
targets with original deadlines/assertions on bounded tmpfs, plus all six quiet
real-XFS retests. Initial timeout failures are retained. Namespace directly
shows completed workload / zero handles followed by XFS fsync delay. This is
ordinary validation, not independent security acceptance.

## Required continuation

Account criterion by criterion for the six approved compiler boundaries and
S09-A–H using the canonical continuation and 504/743/131/170-file returns.
Selected prior results do not close final-source acceptance. Explicitly identify
any valid source-backed reuse and why it applies; do not label old execution as
new execution. Verify remaining material admitted-operation races, real close/
transfer ownership, persistence/publication, resource/identity boundaries and
paired Recipe lease/transport/autofix interleavings. Use the actual optimized
coordinator/worker for paired controls, including rejection of a complete export
with authoritative supervisor-error. Do not waive a criterion because selected
negative controls passed or a test-only oracle exists.

Preserve the approved contract: configured-only owned FD-name and own-stack
views, metadata-only O_PATH, bounded eventfd/epoll, deterministic bounded endpoint
selection, unchanged native policy/limits, per-process VA/real CPU IDs/global
clocks/zero sysinfo/numeric IP:port authority/bounded signal semantics. The
operator contract is Recipe `docs/runtime-limits.md`; Ginux's detailed owner is
`docs/configurable-contract.md`. Link is the export commit point; a complete
post-link archive may remain on late reference-close error, with non-success
required and consumers rejecting it. Pre-publication backing-close failure must
leave no final archive. No general Linux ABI/procfs/PTY/provider interoperability
claim is added.

Dependent-recipe explicit remove/cleanup remains **unresolved by user direction**.
Do not choose or implement that policy. If it blocks a criterion, retain the
specific dependency and required decision. Do not invent a new user hold for
already authorized configured policy.

## Authorization and execution

The user explicitly authorized own Codex CLI with Daybreak's model choice:
`gpt-6-sol`, high reasoning, `daybreak_blue`. The first attempt was rejected by
the server for missing authorized hardware-key authentication; the user has a
pending reauthentication/original-context-results request. Wait for actual
authentication or authorized original-context results. No standard-access
fallback, inherited other-chat execution permission or authentication bypass.
Use normal sandboxing and approval review; do not enable bypass flags.

Within that authorized context, use bounded disposable local/loopback fixtures;
all VMs must be transient operator `qemu:///session` libvirt domains. No direct
QEMU, host-clock changes, external AI-provider testing, package publication,
GitHub write/push/comment/hosted job, kernel rebuild or broader native policy.
Do not mutate source/locks/selected binaries during the running ordinary release
pipeline. Keep verification fixtures/receipts under task-owned `.artifacts` or
isolated exports; preserve useful ports as separately reviewable tests afterward.

Retain exact commands, identities, traces, successful/failed/unrun criteria,
resource/teardown accounting, and a verified manifest at
`.artifacts/prd15/daybreak-release-2026-11-09/receipts.md` in the Ginux worktree.
A written handoff or partial stochastic sample is not completion. Stop for a
genuinely missing decision/permission/environment and state it precisely.
Final source delivery, hosted Linux and the ordinary product/catalog pipeline
remain separate dependencies; do not claim them from security checks.
