# Recipe simplification implementation plan

Status: implementation in progress, one coherent slice at a time.
Updated: 2026-11-05.

Make recipes smaller by consolidating execution, state handling, and common
operations while retaining package-specific acquisition, build, installation,
and readiness rules. Ginux is required for recipe execution. This iteration
targets Linux; macOS support is deferred. This supersedes the earlier proposal
to preserve a public native/host execution backend.

This is the implementation record for the decisions in the recipe anatomy
discussion. The [Ginux implementation plan](ginux-recipe-implementation-plan.md)
continues to own Ginux integration tasks. This plan owns Recipe semantics and
recipe migration, and specifies the persistence adjustment needed by that work.
Task identifiers express implementation dependencies, not runtime stages or
mandatory approval gates. Implementation was requested after this plan was written.

## Requirements to preserve

- Keep is_acquired(ctx), is_built(ctx), and is_installed(ctx) as independent,
  fully programmable checks. They can inspect revisions, configuration,
  architecture, installation records, modules, or other
  package-specific evidence. File existence is not a universal substitute.
- Custom checks determine readiness. A helper or generic check is used only
  when the author selects it. Engine contracts can reject invalid outputs but
  cannot turn a custom not-ready result into success.
- Checks may return discovered context along with readiness. Genuine script,
  permission, subprocess, and resource errors remain failures.
- Preserve acquire/build/install/remove/cleanup hooks and ordinary Rhai
  functions. Build is optional. Cleanup becomes optional with no-op behavior
  when absent; meaningful cleanup remains supported.
- Recipes own source/package facts and behavior. Rust/TOML owns orchestration,
  output locations, and contracts. Normal public recipes have usable defaults;
  inherently caller-supplied inputs such as a custom rootfs are explicit.
- Keep recipe source unchanged during execution. Durable runtime state belongs
  to a particular build/installation identity, not to a shared recipe file.
- Maintain explicit source preparation, artifact-consuming scenarios, and
  ordinary reuse of kernel artifacts. Do not rebuild kernels for this work.
- Preserve separate logs and machine results, durable checkpoints, useful
  failures, execution limits, and safe filesystem boundaries.
- All operations that evaluate recipe code run inside Ginux, including setup,
  imports, custom checks, dependency recipes, helpers, cleanup, and removal.
  Missing or unsupported Ginux capabilities fail before host-side recipe effects.
  There is no public host backend, host fallback, or environment-only bypass.
  Static metadata inspection and hashing may run on the host without executing
  recipe code. Custom readiness rules remain essential under this boundary.

## Scope and existing work

Work in tools/recipe, the affected distro-builder recipes/adapters, and xtask
callers. Adapt legacy recipes when needed for compatibility or a real execution
example. Keep unrelated edits and in-progress Ginux contributions intact.

The planning baseline includes tools/recipe commits 80e45df (worker), e957651
(structured checkpoints), and 61a742b (backend selection). These are discovery
references, not required checkout pins. Re-read current source before each task;
some audited defects may be fixed by ongoing work. Completed Ginux tasks retain
their historical evidence.

Support for macOS, full package version solving, system-wide file ownership,
transactional external installers, A/B activation, and application runtime domains
are deferred. Do not add macOS implementation or validation tasks to this change.
No new recipe language, global package database, or general build DAG is needed.

## Planned contracts

### Readiness and compatibility

Keep existing phase signatures. Introduce an opt-in literal declaration,
const RECIPE_API = 2, recognized by the existing Rhai compilation path. It is a
recipe semantics version, independent of the worker transport version.

For API 2, register ready(ctx) and needs_work(ctx, reason) constructors and
validate their structured return values in Rust. Each check returns one of
these outcomes. Any thrown error is a failure. For example:

    fn is_installed(ctx) {
        if !is_file(ctx.binary) {
            return needs_work(ctx, "installed executable is missing");
        }
        // Package-specific version/architecture/configuration checks belong here.
        ready(ctx)
    }

Unversioned recipes initially retain the documented legacy check convention in
one compatibility adapter. Treat only a deliberate Rhai runtime throw as the
legacy needs-work signal; undefined functions, wrong return types, and other
interpreter failures are fatal. Existing ambiguous intentional throws must be
migrated to explicit outcomes. Do not broaden error suppression for compatibility.

Missing checks conservatively request work for applicable phases. A missing
build hook skips the build phase. Default checks require an explicit declaration
of sufficient inputs and outputs; an old success flag alone proves nothing.
API 2 rechecks a completed phase when its custom check exists and validates any
declared producer contract before reporting completion. Replace always-throwing
force hacks with invocation policy before enabling this behavior on those recipes.

### Context and state

Retain the familiar top-level ctx declaration during migration. Common invocation
setup supplies the same compatibility globals for all commands, and API 2 adds
three explicit namespaces: ctx.runtime for engine-owned inputs/paths/options,
ctx.state for serializable resumable state, and ctx.outputs for named products.
Do not require authors to move every existing field in the first refactor.

Evaluate initialization once inside the Ginux worker. Restore only runtime
state into the initialized context; never overwrite fresh package facts or
current invocation paths with a stale saved whole-context map. Successful checks
can update in-memory context even when the phase is skipped. Persist such state
at a defined operation/checkpoint boundary if it is needed for resumption.

Use a versioned JSON state record in the selected state root. The distro builder
uses .artifacts; standalone usage uses an explicit --state-dir or the host's
per-user application-state location. Do not require a Git checkout. The host
coordinator owns state storage; these host paths never become worker authority.

Separate stable build/installation identity from unique invocation IDs. The
stable identity includes recipe/base/module closure, declared input identities,
target OS/architecture, destination, relevant options, and environment/toolchain
identity. Execution-only force flags do not create a different installation
identity. Reuse verified source caches separately so configuration changes need
not redownload unchanged sources. Custom local source inputs need content/revision
identity or an explicit
custom invalidation rule; a directory's existence is insufficient. Persist a
format version and checkpoint association. JSON arrays/maps/numbers must round
trip; unsupported values fail with a field-specific error instead of stringifying.

Lock the mutable state and output destination that execution actually shares.
Different recipes or input versions writing the same destination must not bypass
each other's locks. Avoid recursive lock acquisition when dependencies reuse the
executor; retain cycle detection and deterministic resource acquisition.

Legacy migration is read-only with respect to recipe source: evaluate existing
ctx, retain a compatibility state record separately, and restore only documented
runtime fields for migrated repository recipes. Report ambiguous external
whole-ctx state with migration guidance instead of guessing which fields are
package facts. Remove source rewriting after repository callers are migrated.
Keep --no-persist-ctx as a deprecated alias for disabled runtime-state persistence,
with explicit help text; it never means execution or filesystem effects are disabled.

### Backend and publication

The host coordinator prepares inputs, launches Ginux, supervises execution, and
publishes outputs. One engine factory, invocation builder, lifecycle executor,
and dependency runner execute recipe code inside the worker. Worker-local
in-process interpretation is an internal implementation detail, not a host
backend exposed through CLI, environment, or library selection. Keep the target
architecture explicit and reject unsupported targets before starting work.

Producer recipes return named outputs that Rust validates. Missing required
outputs are errors, not guessed paths or hardcoded version fallbacks. Custom
installation checks inspect the requested target through its explicitly imported
or retained domain view. A successful install in an unrelated domain root must
not count as an installation in the requested target. Ginux exports only selected
artifact trees through coordinator-owned staging. Live-system activation remains
outside this artifact-production change.

## Implementation tasks

### S0 Make Ginux the required execution path

Dependencies: Ginux R05, G02, G14 and launch/input work. Status: in progress.
Implement together with R06 as one coordinated delivery.
Ownership updated by the user 2026-11-05: this chat now owns continuing local
launch/worker/coordinator integration and may edit the local Ginux fork. The
previous implementation agent has stopped. Upstream PR creation remains with
the linked review thread in the Ginux plan; this chat prepares local changes and
evidence for its review. Preserve unfinished and unrelated work.

Managed execution slice 2026-11-05: Recipe **2f90e5f** now prepares verified,
invocation-private image/worker/input roots and uses the actual configured
launcher. Ginux **3dd05f42** adds bounded nonblocking bootstrap polling; Recipe
**c780ee8** validates the coordinator-prepared build directory. Actual readiness
queries now execute Rhai inside Ginux and return discovered context. A managed
inheritance regression exposed incorrect ctx shadow selection; **33262d1**
fixes it while preserving shared closure values. All **8** focused
`managed_inputs::` CLI scenarios pass with the real supervisor and full image,
including independent custom checks, errors/not-ready outcomes, bases/imports,
child/shared contexts, source immutability, image guards, and a denied host write.
See the Ginux plan for exact evidence and artifact paths. This establishes
managed readiness queries. Subsequent entry/path/checkpoint progress is recorded
below; file-producing lifecycles, installed-target views, full input/dependency
staging, storage, resources, durable state and publication remain incomplete.
S0 remains in progress. No upstream push or PR was performed.

Entry and path continuation 2026-11-05: Recipe **7992bd9** removes the public
Host backend and worker-environment selection, makes default library operations
resolve Ginux, binds private worker requests to bootstrap identity, and unifies
prepared tool paths. The actual library default previously evaluated on the
host under a spoofed worker environment; its real regression now passes through
Ginux. Two CLI guards, eight managed readiness/input cases, one library case,
and two managed cleanup/checkpoint cases pass (13 total). Ginux **f8235d56**
provides bounded finite readiness waits, allowing cleanup/remove hooks to receive
acks. This proves hook dispatch and v1 acknowledgement, not installed-artifact
removal, retained storage or durable recovery. A real file-producing install
still fails at its acquire write; domain-owned writable filesystem integration
is next. Detailed commands, limitations and raw evidence live in the Ginux plan.

Writable-file continuation 2026-11-05: Ginux **222d29ff/10091e4e** connects
bounded supervisor-owned memory storage to configured launch; Recipe **ab9fc89**
uses it for `/work` and `/out`. A real acquire/build/install transforms files,
acknowledges six checkpoints and runs cleanup that reads the installed product
and deletes the intermediate build file. Cleanup and remove also read/delete
actual domain artifacts within one invocation. All 15 focused managed CLI,
entry-guard and library cases pass. Native launch/storage suites and 12 core/VFS
cases pass. This backend is ephemeral, with 32 objects, 64 KiB per file and 64
open files; ordinary compiler builds, retained target removal, durable state and
publication remain unfinished. S0 remains in progress. A subsequent real image
child command fails at startup with ENOENT; isolate the device/channel/process
cause before relaxing the contract. Evidence and exact limitations live in the
Ginux plan; no upstream push or PR was performed.

Null-device continuation 2026-11-05: actual supervisor tracing identifies the
child helper's ENOENT as its initial `/dev/null` open. Ginux **e3458063** connects
explicit null assignment through the existing device registry; Recipe **50edb61**
requests it. A real Rhai helper regression writes/discards bytes, reads EOF and
checks character-device metadata. All 16 focused CLI/library cases and all three
native launch/storage/device suites pass; six existing device cases and release
clippy pass. The child probe advances to an actual pipe2 (293) ENOSYS denial;
pipes and child execution remain unfinished. S0 stays in progress. Traces, source
and raw logs are saved under `.artifacts/ginux-local-integration/null-device/`.
No upstream push or PR was performed.

Owned-pipe continuation 2026-11-05: Ginux **7f072f2c/f1c6419e** connects
bounded in-memory anonymous pipes, safe copyout, atomic small writes, aliases,
EOF/SIGPIPE and stopped blocking calls. Four real Ginux integration suites,
release build/clippy, and all 16 focused actual Recipe CLI/library cases pass.
The child helper advances past pipe2, but libc rejects the sparse public handle
numbers as exceeding the descriptor limit before any process is created. The
next correction is one bounded table of small public descriptors across file,
device, pipe and stdio owners, preserving private backend authority. Do not
substitute inflated limit reporting for that convergence. Multi-task scheduling
and complete child execution remain unfinished; S0 stays in progress. Evidence
is in `.artifacts/ginux-local-integration/owned-pipes/`; no push or PR occurred.

Shared-descriptor continuation 2026-11-05: Ginux **3b00c1c8** provides one
bounded table of small public descriptors across image/file/device/pipe/stdio
owners, with shared offsets/lifetimes, cross-owner aliases and checked native
translation. Actual limits are capped at 256 while preserving a lower operator
limit; reporting is not inflated. Five real Ginux runtime suites and all 16
focused actual Recipe CLI/library cases pass. The real child helper gets past
libc's fd preflight and reaches clone3/clone ENOSYS denials. Process/thread
supervision, inherited tables/backend references, peer scheduling and exec
semantics are next; S0 remains in progress. Evidence lives under
`.artifacts/ginux-local-integration/shared-descriptors/`; no push or PR occurred.

Configured-process continuation 2026-11-05: Ginux **ccd62ca1** supervises up
to 16 processes using the existing ProcessTable, virtual IDs and registered
ptrace birth stops. Fork/vfork and supported clone shapes inherit native tables
and shared owned-backend references. Pipe peers wake, waits preserve actual
exit status and EFAULT retry, failed native fork rolls back, and root death or
supervisor failure kills/reaps every descendant. Six real Ginux suites, eight
process cases, release build/clippy and all 16 focused actual Recipe CLI/library
cases pass. The real child helper now creates a supervised child and completes
signal/stdio setup, then fails explicitly at execve (59). Bash has not executed;
exec/CLOEXEC is next, followed by thread and remaining runtime semantics. Recipe
**d9473f8** updates author guidance; executable code is unchanged. S0 remains in
progress. Evidence is in `.artifacts/ginux-local-integration/task-supervision/`;
no push or PR occurred. The designated review thread still owns upstream PRs.

Configured-exec continuation 2026-11-05: Ginux **8f768700** handles image
execve/execveat and commits CLOEXEC retirement only after kernel-confirmed
replacement. Mixed native aliases survive; failed exec preserves descriptors.
Seven real Ginux suites and all 16 focused Recipe CLI/library cases pass. The
actual child now reaches a successful exec event and Bash's dynamic loader,
then the worker fails at pthread clone flags 0x3d0f00 for its output reader.
Recipe **7f30e3c** replaces that worker panic with a fallible reader startup,
child termination/reap and structured build error. No successful Bash command
result is claimed. Thread/shared-descriptor ownership is next, alongside remaining
wait/signal/readiness semantics; generated-file execution still awaits storage.
S0 remains in progress. Evidence is in
`.artifacts/ginux-local-integration/exec-supervision/`; no push or PR occurred.

Configured-thread continuation 2026-11-05: Ginux **928dc9a1/e59e2e61** tracks
the observed pthread shape, shares descriptor tables with independent syscall
transactions, publishes virtual TIDs, supports clear-TID joins, pins blocked IO
through peer fd reuse, coordinates fork snapshots, and retains handles after
thread/leader exit. Whole-group termination reaps all native tasks. Eight actual
Ginux suites, 14 core/process cases and release build/clippy pass.

The added real CLI child case now proves Bash exit 0, exact child-produced domain
bytes, custom readiness and installation rechecks, named output, clean JSON and
source immutability. It still fails its child-log assertions because successful
stderr is drained into a temporary coordinator log and discarded. Keep this
uncommitted case and its assertions; diagnostic transport is the next S0/S1
slice. The 16 previously established managed cases pass separately; do not count
the new case as passed. Recipe **437c325** corrects author guidance. Evidence is
under `.artifacts/ginux-local-integration/thread-supervision/`. Threaded exec,
robust futexes and broader runtime support remain incomplete. S0 stays in progress;
no push or PR occurred.

Managed-diagnostic continuation 2026-11-05: Recipe **096a656/0ffa1bf** delivers
bounded human diagnostics over transport v2. Child stdout/stderr, binary bytes,
Rhai print/debug/log/warn and phase narration reach CLI stderr while machine JSON
stays intact. Shared sending prevents interleaved frames. Byte and message caps
report once and keep draining without skipping lifecycle checks/results. Frame IO
is chunked to fit the Ginux console contract, including a 32 KiB context. All
20 focused actual CLI/library cases pass. The prior child-log regression is now
committed and passing. Config schema 1 and Recipe API 2 stay independent; future
incompatible state transport is v3, recorded in the interface document.

Build and integration/example compilation pass. Strict Clippy reports 11 findings
in unchanged files and is not clean. Ginux is unchanged this slice; no push or PR
occurred. The next real capture-helper probe fails at pipe FIONBIO (0x5421), with
owned-descriptor readiness still needed. S0 remains in progress. Commands, logs,
limits and the next failure are under
`.artifacts/ginux-local-integration/diagnostic-transport/`.

Managed-capture continuation 2026-11-05: Ginux **75acd2be/210a8cca** connects
shared pipe FIONBIO and public-descriptor readiness through the existing owners.
Recipe **4275769** verifies large capture, empty/whitespace stdout and fatal
child exit 23 through the actual CLI. All 22 focused Recipe cases and nine Ginux
launch suites pass; release Ginux build/Clippy pass. Polling supports pipes,
assigned memory files/directories, null and console aliases, with peer wakeups,
finite deadlines and parked-task SIGKILL teardown. Image-file polling and
nonfatal interruption/restart remain unsupported. S0 stays in progress; storage,
retention and publication are still missing. See the Ginux plan and
`.artifacts/ginux-local-integration/captured-output/` for evidence and limits.

Growable-storage continuation 2026-11-05: Ginux **06d24ae2** separates VFS byte
backing from metadata and uses growable anonymous mappings for configured
launch. The fixed 64 KiB/file bound is gone; 4096 objects share mapped capacity
bounded by memory_mib (file backing only, not total process RAM). Sparse IO,
ftruncate, quota failure, unlinked aliases and teardown have real runtime
coverage. Recipe **acf174c** adds an actual install with independent content
checks on 131072-byte work/output products and an unchanged source. All 23
focused Recipe cases, nine Ginux launch suites and eight VFS cases pass; release
Ginux build/Clippy pass. S0 stays in progress: directory traversal/removal,
rename, file mappings, generated exec, retention and publication are unfinished.
See the Ginux plan and `.artifacts/ginux-local-integration/growable-storage/`.

Directory-storage continuation 2026-11-05: Ginux **9ba6a1d4** connects owned
directory descriptions/getdents64, relative operations, rename/NOREPLACE and
empty-directory removal through the existing owners. Faulting copyout preserves
cursors; aliases/fork share descriptions and replacement preserves open objects.
The actual copy path also required opendir NONBLOCK and full-st_mode fchmod
handling. Recipe **2303bb7** verifies nested copying with a 131072-byte product,
output-tree movement, recursive cleanup and rejection of staged-source
replacement. All 25 focused Recipe cases, ten Ginux launch suites and ten VFS
cases pass; release build/Clippy pass. S0 stays in progress: owned-file execution
and mappings, full source/compiler projects, resources, retention and publication
remain unfinished. Exact evidence and limits are in the Ginux plan and
`.artifacts/ginux-local-integration/directory-storage/`.

- Complete the existing Ginux launch adapter instead of adding another launcher.
  Route every executable CLI and library operation through it by default.
- Remove public host/in-process backend selection, host defaults, and fallback.
  Reject legacy --backend host and RECIPE_BACKEND=host with useful migration
  errors; a temporary explicit ginux alias can remain redundant compatibility.
- Restrict worker-local execution to the internal worker entry point. Setting
  RECIPE_WORKER alone must not bypass the public coordinator execution path.
- Keep static info/list/hash operations on the host only where they do not
  evaluate Rhai. Dynamic metadata requires a managed invocation.
- Report missing launcher/environment, unsupported target, and capability errors
  before executing recipe setup or native helpers on the host.

Evidence: actual CLI/library installs, checks, cleanup, and removal launch Ginux.
With Ginux unavailable or denied, a recipe containing a top-level filesystem
effect produces no host-side effect and fails explicitly. Child processes and
dependencies stay in the same managed execution boundary. Internal worker
protocol probes alone do not establish this behavior.

### S1 Fix immediate correctness defects

Dependencies: none. Status: in progress.

- In distro-builder/recipes/linux-base.rhai and linux.rhai, initialize cleanup
  targets and reject empty/shared-root deletion; align remove with actual install
  paths. Preserve unrelated downloads.
- In helpers/build/extract_api.rs, apply safe destination checks to ZIP as well
  as tar, including existing symlinks. Report invalid entries.
- Route Rhai print/debug and child diagnostics away from JSON/protocol stdout.
  Preserve documented legacy exec status semantics until callers can migrate.
- Correct kernel KCONFIG/force inputs in xtask and recipes, and honor preseed
  refresh before the installed short circuit. Enforce RPM extraction failures.
- Replace size-only trust acceptance with content verification or a verified,
  immutable cache object. Do not label a mutable same-size file verified.

Evidence: actual CLI runs through Ginux against temporary inputs preserve unrelated
files,
reject archive escape, produce parseable JSON/framed output, honor forced work,
and fail on broken extraction. Fixes can be developed while S0 is underway;
completed runtime verification requires S0. No kernel compilation or real system
installation.

Progress 2026-11-05: tools/recipe a3ecaa8 rejects unsafe ZIP members and existing
symlinks at destination ancestors, destination root, nested directories, and leaf
files. It shares the tar path guard, reports metadata/permission errors, and
preserves valid executable payloads and contained tar links. Actual CLI checks:

    cargo test -p levitate-recipe --test e2e archives:: --offline -- --nocapture

Before the fix, two negative CLI cases failed because extraction reported
success. After the fix, all four archive CLI cases passed, including the four
symlink positions and a tar regression case. These use the current pre-S0 CLI
implementation and prove the helper/CLI behavior only. The required managed
Ginux rerun remains outstanding; S1 is not complete. No launch, worker, coordinator,
or tools/ginux source was changed by this slice.

Kernel lifecycle slice: distro-builder 9e4bade initializes fresh cleanup metadata,
rejects unsafe cleanup components/globs, and removes only the current kernel
output directories. Source/prebuilt removal preserves unrelated files and does
not create missing artifact roots. tools/recipe ef5d654 exercises the actual
repository recipes through the CLI with disposable artifacts:

    cargo test -p levitate-recipe --test e2e kernel_recipes:: --offline -- --nocapture

Three original cases reproduced deletion/unsafe traversal/incorrect removal
before the fix. All four final kernel CLI cases passed, including malformed
artifact-root rejection. No kernel was downloaded or compiled. This is pre-S0
recipe/CLI evidence; managed Ginux revalidation remains outstanding.

Kernel invocation slice: distro-builder d0582d1 now reads the existing
KERNEL_FORCE_REBUILD scope define before its legacy environment fallback and
defaults omitted prebuilt Kconfig paths to empty. Invocation options are refreshed
on load so persisted legacy ctx cannot freeze force policy. tools/recipe 18c18a6
extends the kernel CLI cases for force precedence/fallback and missing optional
Kconfig. The two new cases failed before the change; all six kernel CLI cases
passed afterward. No caller/launcher/worker/coordinator code changed. Managed
Ginux revalidation remains outstanding.

Child output slice: tools/recipe 4cde8fd streams exec child stdout to Recipe stderr
while retaining the documented status return. The actual CLI case runs a child
that emits both streams and exits 23, explicitly handles that status, writes a
payload, and verifies parseable final JSON and stderr diagnostics:

    cargo test -p levitate-recipe --test e2e process:: --offline -- --nocapture

Before the change, JSON parsing failed on the child stdout prefix. The case passed
afterward. Rhai built-in print/debug routing still awaits the common engine
factory/worker integration; the exec change alone does not complete S1 output
discipline or prove managed Ginux behavior.

Preseed readiness slice: distro-builder a77ae1b makes all three checks use each
recipe's verified-asset rule, honoring refresh before a cache hit and hashing
actual ISO/APK contents. It removes 33 repeated lines while retaining every
custom hook. tools/recipe a32a227 runs both repository recipes through the CLI
against small synthetic assets with fixture checksum facts:

    cargo test -p levitate-recipe --test e2e preseed_recipes:: --offline -- --nocapture

The two cases reproduced ignored refresh and accepted same-size corruption before
the fix. They now pass for valid cache reuse, all three check operations, corruption
of each asset, and the supported refresh variables. Vendor source facts in the
real recipes remain unchanged. Rootfs consumer marker checks and failed RPM
extraction still needed fixes at this point; the following slice addresses them.
Managed Ginux verification remains outstanding.

Large-asset diagnostics: tools/recipe 8737107 routes SHA/BLAKE3 progress to stderr.
The real CLI case process::cli_large_asset_hash_progress_does_not_corrupt_json
failed on malformed JSON before the fix and passed afterward for SHA256 and
BLAKE3 against a 101 MiB sparse asset. This covers the large-file path used by
ISO verification, which the small preseed fixtures do not exercise.

Consumer integrity slice: distro-builder a45fa88 verifies both cache and staged
asset contents before reuse and changes the RPM extraction pipeline from ignored
status to checked shell execution with diagnostics. tools/recipe 4178701 adds
preseed_recipes::cli_rootfs_consumers_reject_changed_cache_and_staged_assets;
it reproduced accepted same-size corruption and now passes for both materializers,
including Alpine's ISO and APK. Existing unrelated account/package edits in those
same files were preserved and excluded from the commit. The actual RPM pipeline
failure path and full managed materialization still require runtime verification;
the consumer case checks acquisition/readiness only and does not prove those paths.

Command resource-error continuation 2026-11-05: tools/recipe **7f30e3c** makes
both streaming output readers use fallible thread builders. Failure to start
either reader kills/reaps the command, joins a previously started reader, and
returns an interpreter-visible command error. The real managed Bash helper
previously panicked at its first reader startup and broke protocol framing; it
now returns phase=build, zero checkpoints and a normal worker completion with
no surviving child. All 16 focused managed CLI/library checks pass after rebuild.
The first-reader failure is directly reproduced; second-reader cleanup still
needs a runtime resource-limit case once tracked threads are available. This
does not establish successful command execution or complete S1 acceptance.

Child diagnostic continuation 2026-11-05: the actual threaded Bash helper now
runs and produces verified domain files, but the new CLI case fails after the
successful lifecycle because run_domain only retains stderr in an invocation-
temporary file. Its successful LaunchReport is reduced to context/summary, and
those child diagnostics disappear. Source/JSON assertions pass before the log
assertion. Keep the case uncommitted until bounded diagnostic transport preserves
and surfaces both child streams separately from machine results. No assertion
has been removed, and no log parsing or host execution fallback is authorized.

Diagnostic transport completion for this slice 2026-11-05: tools/recipe
**096a656** makes the retained child-output CLI regression pass without removing
its assertions. Twenty actual CLI/library cases cover byte-preserving child and
Rhai diagnostics, machine JSON, source immutability, large context framing, both
diagnostic limits, and previous-version rejection before setup. Logging is
explicitly versioned transport v2; it does not interpret supervisor stderr or
change readiness. Durable log storage, complete-frame deadlines and the broader
S1 acceptance remain unfinished. Streaming helpers work for the demonstrated
lifecycle; captured helpers expose the separate pipe ioctl/readiness gap.

Capture continuation 2026-11-05: Ginux **75acd2be/210a8cca** resolves that gap;
Recipe **4275769** adds actual CLI acceptance for captured stdout and separately
forwarded stderr beyond pipe capacity. Exit 23 remains a fatal custom-check
error. S0 above records the 22-case passing run. Durable logging and the broader
S1 acceptance remain unfinished.

### S2 Unify setup and command execution

Dependencies: S1 where overlapping. Status: helper/setup and opt-in API 2 implementation partial.

Managed readiness, inheritance, and shared-context validation now run through
Ginux; see the S0 managed execution slice. Lifecycle-wide initialization and
subprocess/environment validation still await the install scenarios.

- Consolidate engine creation in src/lib.rs and worker/run.rs. Build scopes
  through one invocation builder used by executor/private/actions.rs, flow.rs,
  and dependency execution.
- Investigate and preserve the semantics of repeated top-level evaluation before
  replacing it. Rhai's default call_fn reevaluates the AST to load imports, and
  recipes also refresh invocation settings and clear saved paths at top level.
  Prefer setup once per invocation only when imports, inherited functions,
  initialized bindings, and the intended isolation between hooks remain correct.
  Keep repeated readiness checks and any necessary refresh behavior explicit.
- Add a checked argv command helper with cwd/environment/output behavior on top
  of one process implementation. Retain an explicit status-returning variant and
  shell support. Avoid process-global environment mutation for child settings.
- Add API 2 recognition and explicit host/target/path/options context. Resolve
  defaults once; remove inferred target identity from build-directory basenames.

Evidence: a real recipe using globals and imports behaves consistently under
install, all checks, cleanup, and remove; setup runs once. Child commands handle
spaces/quotes, cwd, environment, failure, and clean machine output inside Ginux.
Compilation and worker-protocol checks can precede S0; final runtime evidence cannot.

Command-helper slice 2026-11-05: tools/recipe 8254215 replaces the unused
ShellCmd builder and duplicate shell/argv runners with one subprocess owner.
`exec_checked` has checked two-argument and cwd/env options forms; `exec` retains
explicit status returns and `exec_output` retains checked stdout capture. Child
environment overrides do not mutate the worker environment. Child stdin is
closed, streaming diagnostics use stderr, and errors retain bounded output tails
without printing environment override values. The helper reference, author guide,
and man page describe the implemented API.

    cargo build -p levitate-recipe --bin recipe --offline
    cargo test -p levitate-recipe --test e2e command_options:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e process:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e preseed_recipes:: --offline -- --nocapture

Five command CLI cases pass for literal argv/quotes, cwd, child-only environment,
closed stdin, failure stopping installation, retained diagnostics, invalid options,
and legacy shell/status/capture behavior. Both process cases and all three preseed
cases pass with the shared runner. These checks use the current pre-S0 CLI;
managed Ginux execution remains unverified. Engine factory, initialization-once,
invocation context, and API 2 work remain outstanding and must coordinate with
the Ginux agent before editing shared engine/worker files.

Common invocation slice 2026-11-05: tools/recipe 6a46f56 extracts core/invocation.rs
as the existing engine-configuration owner and the compatibility scope builder
used by install, manual checks, cleanup/removal, and dependency preparation.
CLI/RecipeEngine removal now forwards its known build directory through the
existing compatibility seam. Standard ARCH/NPROC/RPM_PATH/TOOLS_PREFIX bindings
are consistent across those operations. Built-in print/debug callbacks live in
helper registration, shared by normal and worker engines; the conflicting
unit-returning debug overload is removed so Rhai's formatter works correctly.
No backend, worker transport/entry point, input preparation, or launcher file was
changed. R06 should adopt core/invocation::engine in its existing worker builder
and pass the worker's explicit build directory to removal; the current worker
removal seam still lacks that value when its defines are empty.

    cargo build -p levitate-recipe --bin recipe --offline
    cargo test -p levitate-recipe --test e2e invocation:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e kernel_recipes:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e process:: --offline -- --nocapture

Two new CLI cases pass for inherited/imported functions and standard globals
across all six operations, stdout-safe built-ins, and an actual dependency script
installed/executed by its parent. All eight kernel cases and both process cases
also pass. These remain pre-S0 results. Top-level initialization is still repeated
by Rhai's default call_fn path, and imported module resolution must be preserved
when replacing that behavior; this slice does not claim initialization-once,
API 2, or a complete shared worker engine factory.
A disposable actual CLI install with an append-only setup log confirmed five
top-level executions for one install (exit 0, payload produced). This proves
repetition, not that it is unnecessary or a defect. Rhai 1.19.0 documents AST
evaluation before call_fn as necessary to load modules; its call_fn path creates
a fresh runtime state for each call. Linux recipes additionally refresh force
settings and clear saved paths at top level. Before changing this behavior,
distinguish invocation setup from deliberate per-hook refresh and preserve the
existing scope isolation where recipes depend on it. Retaining imports alone
is not sufficient evidence that a replacement preserves recipe behavior.

API 2 invocation/readiness slice 2026-11-05: tools/recipe 930d153 adds a shared
invocation runtime used by install, manual checks, cleanup/removal, and dependency
preparation. A supported top-level `const RECIPE_API = 2` selects initialization
once for the merged recipe; unsupported declarations fail before setup effects.
The runtime retains Rhai's imported modules, global constants, initialized
variables, and inherited functions. Dynamic import expressions are evaluated
during setup rather than replayed on every hook. Checks retain isolated variable
scopes and carry discovered ctx through both ready and needs-work outcomes.
Unversioned recipes deliberately retain Rhai's reevaluation path while repository
recipes are migrated; do not remove that behavior merely because it repeats.

The same slice registers ready/needs_work constructors and validates their
outcomes. API 2 errors are fatal. The legacy adapter accepts deliberate throws
but rejects interpreter failures and tagged native-helper failures. Native errors
remain catchable so a recipe can handle a known condition explicitly. Custom
checks verify API 2 acquire/build/install results; installation is checked again
after success cleanup. Per-hook operation budgets retain bounded failure cleanup.
No generic existence rule replaces custom readiness. API 2 target/runtime/state
namespaces, durable state, optional cleanup, complete dependency execution, and
repository migration remain outstanding.

Five retained real CLI cases in tests/e2e/api2.rs passed before concurrent R06
dispatch changes: inherited/dynamic imports and global constants across lifecycle
operations with one setup per command; a built/installed executable actually run;
legacy top-level refresh and explicit native-error handling; an API 2 dependency
tool consumed by its parent; invalid/error readiness rejection; unsupported API
rejection before effects and failed-producer cleanup. The existing two invocation
CLI cases also passed during the initial runtime refactor. These are pre-S0 evidence.
A sixth case covers failure cleanup after operation exhaustion and rejection of
success cleanup that removes the installed product. Its managed attempt stops at
the launcher/worker protocol boundary before that lifecycle behavior executes.

    cargo test -p levitate-recipe --test e2e api2:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e --offline
    cargo check -p levitate-recipe --lib --offline

The initial broader integration attempt encountered concurrent R06 compile
errors; those have since been resolved by its owner, and the current
`cargo check -p levitate-recipe --lib --offline` passes. A real launcher attempt
using tools/ginux/target/x86_64-unknown-linux-gnu/release/ginux fails closed before
recipe execution: the first worker frame declares 1195986517 bytes (limit 4194304),
the launcher is killed/signaled, and no authoritative result file is produced.
The focused case therefore fails; it does not prove cleanup or resource-limit
behavior inside Ginux. Commit 50ef5de improves its failure diagnostics.
No launcher/backend, worker, input, or public dispatch change is part of this
Recipe slice. Re-run the cases after R06/G14 converge; managed behavior remains
unverified. R06's per-operation worker engine must keep a fresh module resolver;
module initialization cached across separate invocations must not replace fresh
package facts. The pinned Rhai internals dependency is confined to invocation.rs.

Invocation namespace slice 2026-11-05: tools/recipe 35161a0 prepares canonical
orchestration inputs separately from legacy define overrides. API 2 receives the
RUNTIME constant before setup and ctx.runtime before checks/hooks, with host and
target OS/architecture, recipe/base/build/tools/input/output/temp paths, operation,
jobs, and requested persistence policy. The current target is the native Linux
default. Conflicting defines for engine-owned bindings and declarations of
RUNTIME fail before setup. Legacy define semantics and unversioned ctx remain
unchanged. Fresh runtime inputs replace a stale runtime field in source.

API 2 initializes missing ctx.state/ctx.outputs as maps, preserves declared map
entries, and validates namespace types and unchanged engine runtime on returned
check/phase/cleanup contexts. Initialization updates a closure-captured shared
ctx through a Dynamic write lock, preserving the captured reference rather than
using Rhai's panicking nonshared get_value_mut API. This is in-memory context;
separate durable state import, strict nested-value serialization, producer output
contracts, and publication remain pending. persist_state records policy only.

    cargo check -p levitate-recipe --lib --offline
    cargo test -p levitate-recipe --test e2e --no-run --offline

Both compilation commands pass. Two new actual-CLI cases in
tests/e2e/invocation.rs cover lifecycle access to fresh inputs, closure-captured
ctx, execution of a declared output tool, state/output propagation, and rejected
namespace types/runtime replacement/reserved declarations. The real Ginux
launcher attempt fails before recipe execution with the existing oversize frame
(1195986517 bytes versus 4194304) and absent result file. Neither new runtime
case has passed; static compilation does not prove namespace behavior in Ginux.

R07 handoff: supply explicitly selected target/input/options values and canonical
domain paths through the versioned worker contract; the current scope builder
records the paths its caller supplies and is not a target-configuration transport.
In particular, removal still needs the worker's known build directory forwarded
through its existing seam, and the final /work/tools layout must be supplied
consistently. Variant selection in kernel metadata still infers directory names
and remains to be migrated. No launcher, backend, worker, input, or public dispatch
file was changed in 35161a0.

### S3 Separate durable state and adapt the worker boundary

Dependencies: S2; full managed recovery also needs Ginux R06/G12a. Status: not started.
Implement together with R07, extending R05's input preparation for state import.
Neither task waits for the other's completion; they share one state/coordinator change.

- Implement separate state storage using executor/private/context.rs and state.rs
  seams. Retain atomic replace and directory durability behavior supported by the
  platform. Load compatible state before hooks, validate identity, and keep recipe
  source bytes unchanged.
- Implement stable state identity, destination locking, structured-value round
  trips, migration behavior, and the renamed persistence option described above.
- Update worker protocol and coordinator together if payload fields change;
  increment transport version for incompatible changes. A new worker reports
  state and outputs, never an authoritative host source or state-file path.
- Keep the durable ordering: storage generation, matching state record, then
  acknowledgement. Disabled persistence must not be advertised as resumable
  recovery. Source/base files are immutable inputs.

Evidence: fresh/rerun/interrupted CLI and worker executions preserve source bytes,
restore nested state, isolate targets, and reject stale state. Concurrent writers
to the same output serialize. Real Ginux interruption before/after ack recovers
the corresponding state/files pair. Worker-protocol checks do not substitute
for real Ginux storage and recovery evidence.

### S4 Unify lifecycle and dependency semantics

Dependencies: S2, S3. Status: readiness adapter/producer rechecks (930d153),
optional cleanup/shared cleanup errors (f097670), and the common non-resumable
dependency lifecycle (3a462ce) implemented independently. Managed verification,
migration, dependency input closure, and durable dependency checkpoints remain pending.

- Implement ready/needs_work/error handling and keep all three independent custom
  checks. Preserve check-returned context. Migrate repository checks together with
  API 2 activation, including force hacks and checks that currently stage files.
- Make cleanup optional. Keep real reason-aware cleanup; perform engine-owned
  scratch cleanup only for paths owned by this invocation. Preserve the primary
  failure if cleanup also fails.
- Replace the acquire/install-only runner in core/build_deps.rs with the common
  executor. Resolve transitive dependencies, report cycles, and execute build
  hooks with an explicit tools destination and consistent checkpoint behavior.
- Distinguish readiness from force policy and output validation. A custom
  not-ready result schedules work; a broken check stops execution. Resume must
  not infer completion from a state flag when required custom checks fail.

Evidence: real dependency-built tools are consumed by a parent recipe; missing
transitive dependencies/cycles fail usefully; check errors do not trigger a
successful reinstall; custom readiness detects stale version/configuration even
when files exist. These runtime cases run through the required Ginux path.

Optional cleanup slice 2026-11-05: tools/recipe f097670 removes the install-time
cleanup requirement and consolidates normal/dependency cleanup in core/runner.rs.
An absent hook is a no-op for legacy and API 2 recipes. Manual cleanup returns
initialized ctx without removing products or emitting a cleanup checkpoint when
the hook is absent. A present hook must accept ctx and reason; malformed hooks
fail before recipe setup in install, manual cleanup, and dependency preparation.
Successful cleanup still carries returned ctx forward. Errors during success
cleanup fail the command; errors during failure cleanup are reported without
replacing the primary failure. Retained-storage interruption/scratch recovery
remains part of S3/R07; this slice does not establish that behavior.

The CLI fixture writer no longer inserts a cleanup stub into every recipe. Five
real CLI cases in tests/e2e/cleanup.rs cover omitted hooks with an executable
installed/run, reason/context propagation through acquire/build/install and
manual cleanup, success/secondary cleanup failures, malformed-hook rejection
before setup, and dependency cleanup failure stopping the parent. The legacy
refresh case now declares its meaningful cleanup explicitly; inherited cleanup
no longer needs a fixture workaround. Author guidance and man pages describe
optional cleanup and its error behavior.

    cargo check -p levitate-recipe --lib --offline
    cargo test -p levitate-recipe --test e2e --no-run --offline

Both compilation commands pass. The focused optional-cleanup CLI case attempted
the real tools/ginux/target/x86_64-unknown-linux-gnu/release/ginux launcher; it fails
before recipe execution with the same oversize first frame (1195986517 bytes,
limit 4194304) and missing result file. No new cleanup runtime case has passed;
all five still require managed execution evidence. No native bypass or stub
launcher was used to obtain a lifecycle result.

distro-builder 5354025 removes five empty production cleanup hooks from Alpine
preseeding and the DVD/Linux/leviso/QEMU tool recipes. ca41220 keeps the leviso
cache-retention comment accurate. Meaningful cleanup hooks remain; the removals
and comments pass diff checks, but those recipes have not run through Ginux.

Shared dependency lifecycle slice 2026-11-05: tools/recipe 3a462ce removes the
separate acquire/install and retry implementation in core/build_deps.rs. Every
needed dependency now enters the common install executor, including transitive
deps/build_deps, optional build hooks, check-returned context, API 2 producer/final
checks, cleanup handling, and the same retry eligibility as its parent. The
invocation owns a shared tools prefix and canonical-path cycle stack. Cycles are
rejected before another lock/evaluation; depth is bounded to 64 active recipes.
Work directories under .deps include a source-path hash to distinguish equal
names; this is not S3 durable input identity. Names cannot traverse parents, and
malformed dependency arrays/entries fail instead of silently dropping tools.
Diamond graph branches still call custom readiness rather than memoizing success.

The existing v1 checkpoint cannot identify a dependency separately from its
parent. Persistent worker dependency execution now fails before dependency setup
with a non-resumable-mode remediation. With --no-persist-ctx, dependency phases
have no independent checkpoint/state acknowledgements; parent checkpoints still
report through the existing sink. Dependency ctx is never substituted for the
parent snapshot, and dependency sources are never rewritten. R07 must replace
this explicit temporary restriction and its v1-specific regression case with
per-recipe identity/state tied to acknowledged filesystem generations. State/output
locking and complete recovery remain S3/R07 work; this does not complete S4.

    cargo check -p levitate-recipe --lib --offline
    cargo test -p levitate-recipe --test e2e --no-run --offline

Both compile. Six literal Rhai fixtures also compile with cached Rhai 1.19.0 and
optimization disabled, without evaluating recipe code. Four retained CLI cases
cover a transitive dependency actually compiled with cc and consumed by its
parent (including inherited/imported helpers and a diamond reuse check), cycles,
missing/invalid declarations, broken custom checks, and the v1 persistence
restriction. No kernel build is involved. The graph attempt through the actual
Ginux release launcher fails before Rhai with the same oversized frame
(1195986517 versus 4194304) and no result file. No dependency graph runtime case
has passed; the C compiler has not run inside Ginux. R05 currently collects
extends/imports but not deps/build_deps, so dependency closure staging is another
explicit R07/R10 handoff in the interface document. No worker, launcher, backend,
or input-preparation implementation was changed by this slice.

### S5 Consolidate acquisition and recipe-owned shared facts

Dependencies: S2; state integration uses S3/S4. Status: partial helper and recipe implementation.
The independent response-writing change below does not depend on context/lifecycle
refactoring; the remaining recipe/state work still follows those dependencies.

- Unify download implementations around partial-file staging, integrity checking,
  and publication on completion. Fix HTTP-resume status handling if retained.
  Keep revision/origin/destination explicit for retained Git acquisition helpers.
- Share source declarations and verification helpers between Fedora/Alpine
  preseed and materialization recipes. Keep preseed operations explicit.
- Reduce the kernel base/child split to one lifecycle owner with reusable
  metadata/config/module functions. Preserve custom kernel readiness rules.
- Supply cache/work/output paths through invocation context. Remove package
  versions, source facts, filename-based capabilities, and output guesses from
  Rust adapters where recipes already own them.
- Remove genuinely unused duplicate helper implementations after checking Rust
  callers and compatibility documentation. Avoid removing public helpers merely
  because the local recipe corpus has no caller.

Evidence: interrupted downloads never publish completed-looking files; same-size
corruption fails verification; preseed consumers work from explicit existing
assets; named output contracts fail clearly when omitted. Reuse kernel artifacts.

Acquisition completion slice 2026-11-05: tools/recipe dcb989f stages HTTP responses
in sibling temporary files and publishes only after a complete successful write.
Failed reads preserve previous destination bytes and remove the partial file.
GitHub asset downloads reuse this writer and reject non-basename asset names.
Hash verification remains a separate recipe operation; this slice does not claim
digest-verified publication or crash-recovery completion for acquisition.

    cargo build -p levitate-recipe --bin recipe --offline
    cargo test -p levitate-recipe --test e2e downloads:: --offline -- --nocapture

The truncated-response CLI case failed before the change and passed afterward
for both absent and existing destinations; complete-response replacement and
installation also passed. The two CLI cases use a real loopback HTTP fixture,
with elevated execution needed only because the sandbox prohibits localhost
socket binding. The GitHub caller shares the compiled writer; its external API
was not exercised. These pre-S0 CLI checks do not establish mediated Ginux network
behavior, which remains outstanding with the rest of S5.

Resume slice 2026-11-05: tools/recipe e01540f routes HTTP resume through the same
staging/publication owner as ordinary and GitHub downloads. It handles ureq's
416 error response explicitly, requires an exact 206 remainder range and matching
length, requests identity encoding, and replaces the destination only after the
complete response is staged. Failed attempts preserve the old destination and
discard that attempt's new bytes; staging requires space for a full replacement.
A matching size on 416 is explicitly not integrity proof. Recipe digest checks
remain required. The public Rust import from the torrent module remains an alias.

    cargo build -p levitate-recipe --bin recipe --offline
    cargo test -p levitate-recipe --test e2e downloads:: --offline -- --nocapture

Four new CLI cases reproduced published partial files, rejected valid 416, and
accepted incorrect ranges before the fix. All six download cases now pass against
a real loopback HTTP fixture, including seven invalid/interrupted resume responses,
successful resumed installation, and matching/mismatching totals. Compilation
also passes. The checks require sandbox socket permission and are pre-S0 evidence;
managed networking and digest-verified publication remain outstanding.

Digest-publication slice 2026-11-05: tools/recipe 84d5e46 adds optional recipe-owned
SHA256 arguments to `download`, `download_with_resume`, and
`github_download_release`, sharing the existing hash implementation. Supplied
digests are validated before acquisition and checked on staged bytes before
replacement. Resume's 416 reuse verifies the existing file when a digest is
supplied. Legacy calls still require separate hash verification. No package
digest or source facts were added to Rust. All eight download CLI cases pass,
including new valid/corrupt transfers with absent/existing destinations and
valid/corrupt resumed/416 reuse. Corruption never reaches installation or replaces
the old destination in these cases. The GitHub path compiles against the shared
writer but its external discovery/download was not exercised. Managed Ginux
network/storage evidence and recipe migration remain outstanding.

Shared-source slice 2026-11-05: distro-builder ac6ef1b gives Alpine and Fedora
preseed/materialization pairs one recipe-owned source declaration and marker
format each in recipes/sources. Literal imports include file extensions so the
existing invocation input discovery can resolve them. Versions, hashes, and URLs
stay in Rhai. Custom lifecycle/readiness hooks remain in their recipes and still
hash actual cache/staged bytes. Marker size inspection uses checked argv capture,
handling quoted paths without shell interpolation and propagating real errors.
The six production files together remove 49 lines; no custom check was discarded.
tools/recipe 3c80a1f adapts the real CLI fixtures to copy those actual modules and
change only fixture checksum facts, with an added quoted-path case. All four
preseed CLI cases passed. Existing unrelated account/package edits in the two
consumer files remain separate and uncommitted. These are pre-S0 CLI results;
managed import reconstruction and full materialization remain outstanding.

Alpine publication migration: distro-builder 0ad6c55 uses the optional SHA256
download form for both assets, removes impossible empty-success handling, and
preserves old verified assets during refresh/failure cleanup. This removes another
29 recipe lines. tools/recipe 6bc7500 exercises the actual acquire and cleanup
hooks using loopback HTTP with fixture source URLs/digests. All five preseed CLI
cases pass, including successful refresh and failed ISO/APK verification; previous
asset and marker bytes remain usable after either failure. This HTTP case needs
sandbox socket permission. Fedora torrent publication still needs its own staging
work; HTTP publication evidence does not establish BitTorrent behavior or Ginux
network/storage operation.

Git acquisition slice 2026-11-05: tools/recipe daa77d6 makes the retained clone/depth helpers share one
acquisition owner with `git_checkout(url, dest, commit)`. Pinned acquisition uses
an exact destination and full commit ID, validates origin and local changes,
fetches a missing commit, and selects detached HEAD. Legacy calls preserve their
destination-parent convention, selected revision, and local edits on valid reuse;
they now reject wrong-origin or damaged destinations instead of deleting them.
Fresh clones stage beside the destination and publish after successful selection.
Explicit `file://` origins allow local declared inputs. Git commands use the shared
process runner, explicit repository/worktree paths, and child-only removal of
inherited repository/index locators. HTTP low-speed bounds are not advertised as
whole-process deadlines. Author/helper/man documentation describes these semantics.

    cargo build -p levitate-recipe --bin recipe --offline
    cargo test -p levitate-recipe --test e2e git_acquisition:: --offline -- --nocapture

All four real CLI cases pass using real temporary Git repositories: older commit
selection and installation, offline reuse, fetching a newly created commit,
preserved wrong-origin/dirty/damaged destinations (including under another repo),
legacy full/shallow cloning and dirty reuse, inherited Git locator isolation, and
failed acquisition with no published checkout or staging leftovers. These cases
use local file transport; HTTP/SSH and SHA-256-format repositories were not run.
Ignored build files remain outside the cleanliness check; recipe-specific source
readiness still applies. Managed Ginux execution and shared-destination locking
remain outstanding with S0/S3.

Kernel ownership slice 2026-11-05: distro-builder 0dc3be7 makes linux.rhai the
single source-kernel lifecycle owner and moves target/source facts, artifact path
guards, and release checks into kernel/metadata.rhai. The prebuilt recipe reuses
the target/path helpers while retaining independent pinned release facts and
readiness rules. linux-base.rhai becomes a documented deprecated entry point to
the canonical lifecycle, not a second implementation. Source checks now accept
omitted optional Kconfig input and return the discovered release/build directory
on installed reuse. Explicit artifact root, legacy target inference, force hacks,
and source persistence still require the S2-S4 migration; this is not API 2.

leviso 5e7e502 removes its unused 228-line overlay; its Rust caller already
delegates to distro-builder's canonical recipe. With the 231-line reduction in
the shared recipes/helpers, this removes 459 production lines. tools/recipe
502eb3d documents the migration and extends the actual CLI cases:

    cargo test -p levitate-recipe --test e2e kernel_recipes:: --offline -- --nocapture

All eight cases pass, including valid and stale release/module readiness for both
targets, independent source build versus installation checks, force precedence,
safe cleanup/removal, and the deprecated entry point's canonical removal. They use
disposable artifacts and prove readiness/control behavior only. No kernel was
downloaded, built, installed on the system, or booted. Full kernel producer/module
installation and managed Ginux execution remain unverified. Existing source and
prebuilt curl/tar acquisition still need migration to shared checked helpers.

Kernel acquisition slice 2026-11-05: distro-builder d21aaa0 migrates source and
prebuilt acquisition to the shared SHA256-before-publication download and native
safe extraction helpers. Prebuilt scratch cleanup now runs after acquisition
failure, and its filesystem publication uses native helpers rather than quoted
shell strings. tools/recipe b1fbc25 runs the actual prebuilt recipe lifecycle with
fixture URL/digest facts and disposable tar.xz artifact trees:

    cargo test -p levitate-recipe --test e2e kernel_acquisition:: --offline -- --nocapture
    cargo test -p levitate-recipe --test e2e downloads:: --offline -- --nocapture

The new producer case passes for a valid payload and rejects wrong digests,
truncated HTTP responses, missing modules, and an escaping symlink before replacing
prior artifacts. Failed attempts remove extraction scratch and preserve unrelated
downloads. The valid path passes all three custom readiness checks. All eight
existing download cases also pass with the binary-capable loopback fixture.
Socket permission is required by the sandbox. These inputs are not bootable
kernels; no kernel was compiled or booted. Source acquisition's changed branches
were parsed through CLI checks but were not run end to end. Managed Ginux
acquisition/publication and interruption recovery remain outstanding.

Named-output consumer slice 2026-11-05: distro-builder 6bc459f replaces flat-field
fallbacks and guessed rootfs/ISO/APK/kernel/source paths in the four recipe
adapters with shared named-output contracts. Required ctx.outputs entries must
be nonempty absolute paths with the requested filesystem kind. Kernel image and
modules are required; source is optional (LinuxPaths.source is now Option<PathBuf>)
and kernel_version is required metadata instead of an "unknown" fallback.
Installed kernel reuse no longer invents a source tree or reconstructs vmlinuz
under a different output root.

The shared kernel metadata owner declares kernel/module paths for source and
prebuilt lifecycles. Prebuilt context/output assembly is consolidated while its
independent checks still run. Rootfs and preseed producers report named products
on successful install/readiness paths; legacy flat fields remain for migration.
AcornOS d1777d3 adds matching names to the legacy Alpine facade producer.
tools/recipe 0eb2907 extends existing real CLI kernel/preseed cases to check names
on acquisition and reuse, and documents path declarations. Custom readiness
rules remain in recipes; consumer type checks do not replace them.

The builder also uses an invocation-owned temporary result file, removed on
success/failure, instead of reusing .recipe-output.json. distro-builder 8dc194c
adds a passing real-CLI regression: static `recipe info` exits zero without a
lifecycle result, and the adapter rejects it while preserving old cached JSON.
That case does not evaluate Rhai or validate managed publication.

    cargo check -p distro-builder --lib --offline
    cargo check -p distro-builder --bins -p leviso -p acornos --offline
    cargo test -p distro-builder --test recipe_outputs --no-run --offline
    cargo test -p levitate-recipe --test e2e --no-run --offline

These compilation commands pass. All nine changed Rhai sources also pass a
temporary Rhai 1.19.0 compile_file check with optimization disabled; this compiles
syntax without evaluating setup/imports/hooks. The durable integration cases use
the actual CLI rather than that temporary probe.

The new managed producer/consumer case creates a real rootfs tool and tests named
selection, omitted/wrong-kind/missing products, and stale conventional artifacts.
Its positive attempt through the actual Ginux launcher still fails before recipe
execution with the oversize first frame (1195986517 versus 4194304) and no result
file. No managed output-contract case has passed. Publication remains R08 work:
the coordinator must export selected domain products and return published host
paths, never treat a worker path as host authority. The interface records this
requirement explicitly. No kernel was compiled or booted.

Remaining S5 work includes legacy work/cache directories, managed publication/integrity,
and the remaining acquisition helpers. The named-path consumer layer is not
evidence that those responsibilities are complete.

Explicit preseed contract slice 2026-11-05: distro-contract d2b6693 and
distro-builder c949481 let either source kind declare its acquisition script and
required named output kinds.
Canonical Acorn declares its Alpine preseed script with iso/apk_tools files;
Levitate declares its Fedora preseed script with an iso file. A declaration needs
both the script and nonempty output contracts; unknown kinds fail parsing.
Recipe filenames no longer select preseed capability. Rust retains source kinds
for policy and explicit compatibility conveniences, while materialization and
preseeding each use one runner. Preseeding remains a separate command. The CLI
validates the contract before source operations, and reports selected output
paths rather than guessing a trust directory from an ISO parent. Alpine accepts
the common ROOTFS_SOURCE_FORCE_REFRESH request with its legacy alias retained.

    cargo check -p distro-builder --bins --lib --offline
    cargo test -p distro-builder --test preseed_contract --offline -- --nocapture

Compilation and all three actual CLI cases pass. The cases copy canonical recipe
and variant files, confirm filenames alone do not enable preseeding, rename both
declared scripts, and reject incomplete or invalid output declarations before
recipe launch. The renamed case reaches the real Recipe executable and fails
closed without GINUX_BIN. No managed acquisition or output publication is proved.
R07 still needs to transport refresh/options and explicit input/cache paths;
R08 still owns publication. Legacy adapter cache paths and basename inference
remain outstanding, alongside actual managed runtime verification.

Kernel work-path slice 2026-11-05: distro-builder aabc3f8 provides the existing
orchestration path owner as recipe::recipe_work_dir. Kernel readiness checks,
source/prebuilt xtask producers, explicit source preseeding, and the Linux adapter
share .artifacts/work/<normalized-distro>/downloads. The Linux adapter's explicit
linux_for_distro entry point loads and validates canonical variant configuration;
the legacy base-directory API delegates to it. Published kernel roots remain
.artifacts/kernel/<distro>/current. No files are moved and existing artifacts are
not rebuilt. leviso f1c62e9 uses the explicit adapter and no longer treats missing
source caches as a preflight warning or claims that they will be downloaded on
an ordinary artifact-consuming build. The public has_linux_source compatibility
hint remains, now looking in the canonical work directory; it does not determine
readiness. xtask uses the shared path owner and stops sending unused kernel
version/hash/localversion/module-layout defines that the recipes do not consume.

    cargo check -p distro-builder --bins --lib -p leviso -p levitate-xtask --offline
    cargo test -p distro-builder --test preseed_contract --offline -- --nocapture
    cargo run -p levitate-xtask --offline -- kernels check

Compilation and all three CLI cases pass, including a check that renamed explicit
preseeding creates canonical work rather than legacy AcornOS/downloads. The
read-only kernel check fails because both canonical kernel.release files are
absent. No kernel build or managed Linux execution was attempted; compilation
does not establish artifact reuse or production inside Ginux. The legacy
Alpine/package/tool paths and materializer cache-input inference still need
migration. Explicit worker target/options/cache transport remains S2/S3/R07.

Torrent publication slice 2026-11-05: tools/recipe b76d093 makes both torrent
forms use a private staging directory under the selected destination. It resolves
and validates every metadata path before starting payload writes, rejects unsafe
existing destination parents/entries, and stops the session before validation and
publication. Completed regular files must match declared lengths and are flushed
before rename. Legacy calls copy existing cache files into staging for the
client's piece-checked reuse. Multi-file publication is atomic per completed file,
not a transaction over the whole set; an export failure can leave a subset of
complete files. Process termination can leave hidden staging, which is not
automatically resumed or treated as a completed cache entry.

The four-argument torrent(url, dest_dir, filename, sha256) form requires exactly
the requested basename, acquires a fresh replacement, and verifies the recipe's
SHA256 before replacing an old entry. distro-builder 272d62b migrates Fedora to
that form and removes filename-normalization and destructive refresh/failure
cleanup. Its checksum-manifest validation and custom readiness/content/marker
checks remain. A failed transfer or digest check no longer intentionally deletes
the old ISO and trust marker. The recipe removes 34 net lines.

    cargo check -p levitate-recipe --lib --offline
    cargo test -p levitate-recipe --test e2e --no-run --offline

Both compilation commands pass. Fedora's modified recipe and shared facts also
pass syntax-only compilation with cached Rhai 1.19.0 and optimization disabled.
Three retained CLI cases use an actual local librqbit seeder and loopback tracker:
single-file success/digest rejection with absent/existing cache, metadata
selection and legacy multi-file transfer, and the actual Fedora recipe's forced
refresh preserving old ISO/marker on digest failure. Verification runs inside
the worker work directory; returned domain paths are not host filesystem evidence.

The focused single-file attempt through the actual tools/ginux release launcher
fails before Rhai with the same oversize first frame (1195986517 versus 4194304)
and no result file. No torrent CLI case has passed and managed interrupted
transfer/cancellation/publication still need evidence. No native bypass or stub
launcher was used. The interface now records the separate materializer input
gap: public R06 supplies no extra assets, and R05's 512 MiB/file, 2 GiB total bounds
cannot stage the Fedora DVD. R07/input integration owns bounded real-asset
transport and domain paths; merely passing a host path as a define cannot replace
that work. Existing materializer cache inference is still present and must be
removed once this input contract is wired.

### S6 Migrate representative recipes and validate managed execution

Dependencies: S3-S5. Status: not started.

- Start with custom-source-rootfs and a small locally built tool; then preseed,
  source-rootfs, and kernel recipes/callers. Keep meaningful custom checks even
  when that leaves a long recipe. Migrate remaining retained repository recipes
  before removing legacy source-persistence code.
- Use the existing recfstab integration as a real source-built example inside
  Ginux. Exercise all three custom readiness checks, version/configuration
  invalidation, removal, cleanup, cancellation, and recovery in retained storage.
- Exercise a second recipe with different custom readiness rules so the common
  executor cannot assume that every package is complete when one file exists.
- Update current CLI integration runners to enter through the Ginux coordinator
  with explicitly prepared environment artifacts. Keep framing-only probes
  separately labeled; they are not end-to-end managed-execution evidence.
- Compare representative before/after recipes for duplicated bookkeeping,
  clarity, and correctness. Do not enforce a line-count target or shorten checks
  at the expense of their package-specific rules.

Evidence: custom rootfs input changes refresh output; a source tool builds/runs,
reuses valid work, resumes after failure, and detects deleted outputs inside
Ginux. Public host execution attempts are rejected. macOS has no acceptance
requirement in this iteration.

### S7 Reconcile documentation and remove obsolete implementations

Dependencies: migrated behavior from S1-S6. Status: not started.

- Update README, WRITING_RECIPES, man pages, CLI help, and the supported helper
  reference to describe implemented behavior. Clearly label broader package
  management ambitions and obsolete format proposals.
- Replace metadata line scanning with the same declared metadata model used by
  the loader. Keep static inspection free of arbitrary host execution; any
  dynamic resolution must run inside Ginux and be identified explicitly.
- Remove the replaced dependency runner, source-rewriting machinery, and unused
  command framework and public host backend once migrated callers use their
  existing-owner replacements.
- Keep optional AI repair outside normal deterministic execution and preserve
  meaningful custom validation through repair/retry.

Evidence: documented examples run through the CLI; inherited metadata is reported
accurately; recipe files remain unchanged; legacy behavior has a documented
migration path. Report platform/backend limitations precisely.

## Coordination with ongoing Ginux implementation

Ginux launch, process handling, storage, resource control, and export can proceed
independently. Recipe semantics remain opaque to Ginux. R07 consumes the separate
state destination from S3; R10 uses S4's common dependency executor. S0 supersedes
R04's original host-default requirement without rewriting its completed evidence.
Ginux tasks
must not grow a second recipe lifecycle or macOS-specific package logic.

Keep one active editor for tools/recipe src/lib.rs, backend.rs, worker/*, and
core/executor/* during overlapping worker/state changes. Re-read changes and
integrate in dependency order. Existing worker interfaces and completed task
records describe the versions actually implemented, not automatic completion of
the replacement semantics. Do not change worker payload interpretation in place.

The [interface document](ginux-recipe-interfaces.md) distinguishes current v1
from the planned persistence revision. The host coordinator determines state-file
destinations from trusted invocation identity. Worker source/module references
are input identity only. Resume loads only state matching the durable filesystem
generation and input identity.

## Verification and execution notes

Use compilation and named, focused CLI/integration/runtime cases through the
actual executable and filesystem. Extend the existing e2e location where useful;
do not add unit tests, mock-only suites, or isolated-helper equivalents. Scenarios
consume explicitly built artifacts and run independently. No mandatory local
commit pipeline or approval ceremony is introduced.

S1/S2 helper and setup work can proceed alongside unfinished Ginux infrastructure.
Compilation and protocol checks establish only their stated scope. Runtime
acceptance remains outstanding until the required Ginux path works; do not add a
host fallback to obtain a passing result.

Commit coherent working changes under applicable repository instructions; keep
unrelated edits and current branches. Update task status with concrete evidence
as work lands. Temporary audit probes are useful reproductions, not durable
acceptance artifacts; preserve needed cases in the existing integration owner.

Next implementation action in this chat: connect owned-file execution and
mappings through the existing filesystem/exec/memory owners. The actual CLI
probe now copies /usr/bin/true into /work/build/domain-true successfully, then
exec_checked reports EOPNOTSUPP with clean workload failure and unchanged source.
Its source/result are under directory-storage/generated-exec-next. Ginux
**9ba6a1d4** now supports owned directory traversal, relative operations, rename
and removal; Recipe **2303bb7** verifies nested tree production/movement,
recursive cleanup and protection of staged recipe source. All 25 focused
CLI/library cases, ten Ginux launch suites and ten VFS cases pass. Current
limits and source/binary identities are in the Ginux plan and
`.artifacts/ginux-local-integration/directory-storage/`. Storage remains
ephemeral. Image-relative directory access, image mutation, links, mappings,
generated-file execution and ordinary compiler projects are unfinished. Keep machine results
separate, custom checks authoritative and the launcher mandatory. Image-file
polling, broader threaded exec/signal/robust-futex behavior, complete-frame
deadlines, durable logging/state and cancellation remain unfinished. Public entry,
prepared tool paths and bounded checkpoint waiting are connected; process-global
dependency environment cleanup remains outstanding. Explicit cache/input and dependency-state
transport still require R07. Remaining
legacy adapter cache/work paths and materializer cache inference still need
migration using that input contract. Kernel and preseed callers share the
canonical work path; torrent/Fedora publication code now awaits managed evidence.
Target/state transport follows the coordinated S3/R07 change. Revalidate the
invocation, cleanup, and output contracts through the real launcher when
R06/G14 converge. Keep the legacy adapter until recipes migrate. The
complete dependency executor and durable state integration still follow their
shared S3/R07 contracts. This chat now owns local worker/launcher integration
under the user's updated instruction. Re-run implemented slices through the
mandatory Ginux path when the corrected integration is available; upstream PRs
remain the review thread's responsibility.
