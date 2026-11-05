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

Dependencies: Ginux R05, G02, G14 and launch/input work. Status: not started.
Implement together with R06 as one coordinated delivery.
The concurrent Ginux implementation agent owns launch/worker/coordinator changes.
This chat implements independent Recipe slices and integrates with those changes
when available; it does not take over Ginux implementation.

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

### S2 Unify setup and command execution

Dependencies: S1 where overlapping. Status: not started.

- Consolidate engine creation in src/lib.rs and worker/run.rs. Build scopes
  through one invocation builder used by executor/private/actions.rs, flow.rs,
  and dependency execution.
- Evaluate top-level setup once, then call hooks without reevaluating it. Preserve
  imported module availability and inherited function resolution explicitly.
- Add a checked argv command helper with cwd/environment/output behavior on top
  of one process implementation. Retain an explicit status-returning variant and
  shell support. Avoid process-global environment mutation for child settings.
- Add API 2 recognition and explicit host/target/path/options context. Resolve
  defaults once; remove inferred target identity from build-directory basenames.

Evidence: a real recipe using globals and imports behaves consistently under
install, all checks, cleanup, and remove; setup runs once. Child commands handle
spaces/quotes, cwd, environment, failure, and clean machine output inside Ginux.
Compilation and worker-protocol checks can precede S0; final runtime evidence cannot.

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

Dependencies: S2, S3. Status: not started.

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

### S5 Consolidate acquisition and recipe-owned shared facts

Dependencies: S2; state integration uses S3/S4. Status: not started.

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

Next implementation action in this chat: address independent acquisition completion
and refresh behavior, then helper output routing. Keep the concurrent
Ginux agent's launcher/worker/coordinator work separate. Re-run implemented slices
through the mandatory Ginux path once that agent's integration is available.
