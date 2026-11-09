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
repository instructions and this checkpoint after a context shift. Root
instructions prohibit mandatory process gates and Recipe unit tests; existing
CLI/integration/product owners provide verification.

Current local pair: Ginux `b81f30b52e523d142071603f477c6891c7fd9df9`
adds only retained tests after runtime commit `3cd1e2de`; its clean exact rebuild
remains launcher `fb87792d5b4363ca669d20935ce6f3d3f6d08fb97ee0cdc74e60bc324c200e6e`.
Recipe is clean `91068391cdc7e0e8ef10c59e2b8b1bcc65521554`, matching CLI/worker
`467140e9b036a8ed20b58f87020ad7a63cfe7c68d311dc8e19cfe51c464cf9ef`;
image remains `ffe04d86…`. Standalone fresh GNU/musl production passes. The
cached-dependency manual-readiness correction passes old-fail/new-pass, all
13 NEEDS and four dependency cases. Explicit dependent-recipe remove/cleanup
remains unresolved by user decision; it is not repaired or accepted here.
The corrected builder is still uncommitted pending real product completion.
Both product builds are running; no new full ISO/guest/install/A/B pass is claimed.
Returned Daybreak follow-up has 743 verified evidence files and 20/20 selected
Recipe passes, including paired non-success publication rejection. All six
security items and S09-A–H remain partial. See the latest checkpoint below.

Evidence is under `.artifacts/ginux-contribution-preparation/prd15/`.
`sources-initial.json` and adjacent patches pin the initial checkout, including
pre-existing local edits. Shared staging is preserved. Current builder/contract/
xtask and their linked consumers compile with `cargo build --locked -p
levitate-xtask` (`caller-build.log`). This establishes compilation only.

Initial-baseline Recipe documentation was committed at `cca5bb7` (preceded by `a53c247`) over candidate
`124f378063ce449e3799aad9b43dfac9404754f1` (production Rust unchanged); configured
Ginux is `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`. The PRD14 compiler and
acquisition images are respectively `a8edd8fa…` and `ffe04d86…`; their durable
input and fresh-host preparation evidence remains in the PRD14 report. These
are provisional local candidates, not a released combination.

Existing product/scenario manifests include successful runs produced in another
checkout and older input combinations. They must retain their original source
bindings. A recent Levitate base-rootfs attempt failed. None of these is being
promoted to current final-source acceptance.

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
