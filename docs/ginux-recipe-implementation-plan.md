# Ginux execution for Recipe: implementation and upstream contribution plan

Status: implementation is in progress; individual tasks record completion and evidence.
This plan records the agreed objective,
source investigation, proposed interfaces, dependency-ordered tasks, and parallel
work assignments. Task IDs are scheduling references, not build stages or test
gates. No task requires a review ceremony or additional permission merely to
continue authorized implementation.

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

Follow repository `AGENTS.md` instructions. Preserve unrelated edits. Use
`.artifacts` for current outputs/tooling; do not build kernels. Unit tests,
assertion-based unit doctests, renamed isolated-helper tests, and mock-only suites
are prohibited. Use compilation and focused CLI/integration/runtime checks.
Remove prohibited suites/runners when addressing their owners; keep policy-only
cleanup separate from upstream feature commits. Do not run Ginux's current
aggregate `make test`/`test-unit` or assume its upstream runners satisfy local
instructions. No mandatory local commit pipeline is introduced by this plan.

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

- [ ] **G04 — Connect native threads and synchronization.** Depends: G03b.
  Owner: G-process. Handle required thread creation, thread identity, futex/TLS,
  and thread exit using existing models. Implement only the concrete required
  behavior in bounded patches. Done: a real threaded program completes; Recipe's
  output-draining thread pattern is supported without weakening mediation.

- [ ] **G05 — Complete child waits and normal signal delivery.** Depends: G04.
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

- [ ] **G05a — Connect inherited pipes and close-on-exec descriptors.** Depends:
  G05, G07b. Owner: G-process/descriptor adapter; serialize with G-files changes.
  Implement duplication/shared-offset and pipe inheritance through domain objects.
  Done: a real shell pipeline and an exec with close-on-exec descriptors behave
  correctly without receiving supervisor descriptors.

- [x] **G08 — Add rename and removal behavior.** Depends: G07b. (Done 2026-11-05: worktree `/tmp/ginux-g08`, branch `ginux-recipe/g08-rename`, commit `d4ecd78`. Pure-core `rename_at`/`unlink_at` + VFS `rename` (atomic replace, nofollow, tombstones; fixed nofollow gap + tombstone-shadows-live bug). Coordinator reran source-verbatim reproducer: 70/70 PASS, base blob immutable. LIMIT: claims still deny at the entry layer; supervised wiring + `rmdir` pending.)
  Owner: G-files. Implement the rename/unlink operations required for temporary
  files, compiler output, and cleanup. Keep atomic rename semantics and explicit
  unsupported cases. Done: real tools replace and remove domain files correctly;
  denied mutations have no host effect.

- [ ] **G08a — Add installation permissions and link operations.** Depends: G08,
  G05a. Owner: G-files. Implement chmod and the link variants required by the
  selected tools, preserving path confinement and metadata. Done: ordinary tools
  install a usable executable tree and retain meaningful links and permissions.

- [ ] **G09 — Execute files produced in writable domain storage.** Depends: G08a,
  G03b. Owner: G-files with G-process interface fixed first. Connect generated
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

- [ ] **G12a — Checkpoint a consistent domain storage generation.** Depends:
  G12, G05a. Owner: G-storage. Add bounded quiesce/flush or equivalent consistency
  behavior for all tasks sharing storage and identify recoverable generations.
  Done: interruption cannot present an acknowledged generation with missing
  matching files; inability to obtain consistency is an explicit error.

- [ ] **G13 — Export selected results and deletions.** Depends: G12a, G08a.
  Owner: G-storage. Export selected trees or changes into a fresh confined host
  staging destination with needed metadata/tombstones. Validate traversal, links,
  output bounds, and partial failure. Done: a generic file-transform workload's
  output can be retrieved and removed without modifying arbitrary host paths.

- [ ] **G14 — Provide domain-owned bidirectional byte channels.** Depends: G07b,
  G05a. Owner: G-files/channel adapter, serialized with G-files edits. Supply the
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

- [ ] **R06 — Launch and supervise Ginux from Recipe.** Depends: R04, R05, G02,
  G14. Owner: R-host; subprocess adapter. Wire the selected contract and channels,
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

- [ ] **L01 — Terminate and reap the complete domain workload.** Depends: G05,
  G02. Owner: G-process/lifecycle. Handle normal completion, startup failure,
  explicit cancellation, and supervisor loss according to the supported model.
  Done: real descendant processes do not survive domain termination; storage
  and assigned backends receive the appropriate close/flush behavior.

- [ ] **L02 — Enforce supported CPU and memory declarations.** Depends: L01.
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

- [ ] **I03 — Inventory the first actual worker/compiler incompatibilities.**
  Depends: I02, R06, G09, G12a. Owner: Integration; execute real Recipe/Cargo under
  Ginux with captured diagnostics. Classify failures by existing owner. Done:
  concrete missing operations are identified without silent fallback or fabricated
  outputs. This task does not itself promise the build succeeds.

- [ ] **I04 — Close observed compatibility gaps in small assignments.** Depends:
  I03. Owner: appropriate Ginux lane for each gap. Create one named follow-up per
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

- [ ] **U02 — Open native process/thread integration PR(s).** Depends: G05a.
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
