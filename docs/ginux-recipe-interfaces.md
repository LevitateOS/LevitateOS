# Ginux–Recipe execution interfaces

This describes the current local integration. The [implementation queue](recipe-simplification-implementation-plan.md)
owns scope and evidence; historical versions remain in Git. Recipe API2, worker
transport6 and Ginux domain schema1 are independent contracts. Coordinator,
worker and retained records use one current interface; mismatched versions fail
before recipe setup. This record does not authorize upstream publication.

## Execution and ownership

Every operation that evaluates recipe code requires the configured Ginux
launcher. Metadata/hash commands read declarations without evaluating setup.
There is no host execution selector or fallback. Current execution is native
Linux/x86-64 through Ginux's configured ptrace backend.

The coordinator invokes:

```text
ginux launch CONFIG PREPARED_ROOT --result-file RESULT --export-file EXPORT
```

It additionally passes `--control-fd` for checkpoint requests and
`--restore-file` when restoring matching owned files. These host paths and file
descriptors are coordinator-owned; recipes cannot choose them. The launcher
result is authoritative alongside the worker protocol. Missing/malformed records,
contradictory exits, incomplete frames or missing terminal results fail explicitly.
A supervisor failure after worker activity is distinguished from startup failure.

`GINUX_BIN` selects the launcher. `GINUX_ROOTFS` selects an assembled uncompressed
tar; `GINUX_ROOT=oci:NAME@sha256:DIGEST` identifies those exact tar bytes, not an
OCI manifest. The input owner snapshots and hashes the selected archive, extracts
without following links, and creates a private invocation root. It imports the
matching Recipe worker (`RECIPE_WORKER_BIN` for library callers), recipes and
explicit selected inputs. Roots live until the supervisor is reaped, then are
removed. Active leases prevent concurrent pruning.

## Domain paths and recipe contract

| Purpose | Path and authority |
| --- | --- |
| Worker, recipes, modules, declared assets/source closures | `/inputs`, immutable snapshots |
| Work/build directory | `/work/build`, owned writable storage |
| Dependency tools and cache | `/work/tools`, owned writable storage |
| Temporary files | `/work/tmp`, owned writable storage |
| Installed target and selected products | `/out`, owned writable storage |

Recipes declare `const RECIPE_API = 2;`. Setup runs once per recipe evaluation
and reads immutable `RUNTIME`. Phase hooks receive `ctx.runtime`, resumable
`ctx.state` and named `ctx.outputs`. Package facts come from current source.
Readiness is `ready(ctx)` or `needs_work(ctx, reason)`; thrown errors fail the
operation. Native errors and persisted booleans cannot masquerade as readiness.
Dependencies retain their own checks and state. Completion rechecks installation
after success cleanup. Cleanup is optional; successful failure cleanup becomes a
recoverable checkpoint while the original producer failure remains authoritative.

`--input NAME=PATH` explicitly imports a file/tree. Source imports omit root
`.git`/`target`, preserve contained relative links and reject escapes/cycles.
Named filesystem inputs `rootfs` and `target` preserve filesystem link bytes,
including absolute links meaningful in a future root. Imported hardlinks share
private snapshot backing; they never alias mutable host input bytes. A
coordinator-authored permission manifest lets copying helpers preserve original
logical modes despite read-only imports. A `target` directory seeds `/out` only
for fresh state; an acknowledged matching filesystem takes precedence on reuse.

Named outputs identify files or directories under owned storage. The coordinator
validates the export, materializes selected products privately, then publishes
fresh paths. Failed work cannot replace previous publications. Returned
`ctx.outputs` contains those host paths; the worker never selects host destinations.

## Options, bounds and network policy

Execution options cross the request explicitly, including native target,
`jobs`1..16, phase force flags, timeout, memory/storage bounds, network grants and
machine events. Main-recipe force flags are cleared for dependencies. Defaults
are one job, 3600 seconds, 2048MiB, 16,384 storage objects and networking disabled.
No coordinator-wide environment mutation supplies invocation state.

Resource enforcement is specific: Ginux applies `memory_mib` as inherited
**per-process virtual address-space** limits (`RLIMIT_AS`) and constrains the
child CPU affinity to at most the requested available CPUs. This is not aggregate
domain RSS or cumulative CPU time. Owned storage has separate byte/metadata
accounting, with explicit object/data limits; shared hardlink bytes count once.
Public descriptors and tasks remain bounded. Supervisor-private backing FDs
reserve capacity independently of the public descriptor table and respect the
host hard limit. Unsupported operations fail; this is not complete Linux ABI
coverage. Large source consumers select sufficient explicit storage/memory policy
in their Rust/TOML contract rather than relying on the small-tool defaults.

Outbound networking requires repeated `--network-host HOST:PORT` grants (or
library `ExecutionOptions`). Empty policy produces `network.backend="none"`.
The coordinator resolves at most eight numeric TCP endpoints before launch,
with bounded resolver time/output, and writes an immutable hostname map inside
the private root. Ginux enforces destination, connection (16) and cumulative
send+receive byte budgets (default8GiB); aliases/fork retain ownership and teardown
revokes sockets. No proxy inheritance, arbitrary DNS, UDP or listening access is
provided. Redirect destinations need their own grant. **An IP:port grant cannot
isolate virtual hosts or URL paths sharing that endpoint.**

HTTP helpers validate normal TLS trust and hostnames using the prepared image's
CA bundle; test-only CA images are explicitly separate inputs. Canonical
acquisition is `download(url, exact_destination, sha256[, resume])` and
`git_checkout(url, exact_destination, full_commit)`. Transfers stage privately,
validate complete bytes/revisions, then replace cache entries. Invalid digests,
truncation, denied endpoints and missing commits preserve earlier valid cache.
A grant is permission to contact an endpoint, not permission to skip verification.

The config retains Ginux schema1, balanced contract/profile, ptrace-native backend,
stderr audit and stop-on-failure policy. Recipe checks config≤4096 bytes,
id≤64 bytes, paths≤128, args≤7×128, env≤8×256 and assignment bounds before launch.
The environment contains fixed domain PATH, worker/invocation identity and TMPDIR;
only explicit allowlisted toolchain variables may be added. Host HOME, checkout
paths, credentials and ambient proxy variables do not cross this boundary.

## Worker transport6

Each frame is `[u32 BE length][u32 BE version=6][u32 BE sequence][u16 BE op]`
followed by UTF-8 JSON. Length includes the 14-byte header and is at most4MiB.
Each direction starts at sequence1; gaps, replay, wrong direction, unsupported
versions and truncated/oversized payloads abort that invocation.

| Op | Direction | Payload |
| --- | --- | --- |
|1 request | Coordinator→worker | invocation_id, operation, persist_state, recipe_digest, options, restored_states |
|2 checkpoint | Worker→coordinator | seq, phase, reason, canonical `/inputs/...` recipe identity, state object |
|3 ack | Coordinator→worker | checkpoint seq and durable generation |
|4 failure | Worker→coordinator | fatal, phase, reason, human message |
|5 result | Worker→coordinator | ok, failures, ctx_summary; sent exactly once |
|6 diagnostic | Worker→coordinator | stream, raw byte array, truncated marker |

State from all recipes is keyed by staged identity; package facts and runtime
paths are not restored from saved source text. A checkpoint pauses recipe progress
until Ginux exports owned files and the coordinator durably records their matching
state generation. Ack follows that durable pair. Interrupted writes retain the
last complete generation. `--no-persist-state` suppresses reuse and durable state
claims, acknowledging generation0. It does not enable source rewriting.

Snapshots use validated USTAR with GNU longname/longlink extensions, regular
files, directories, symlinks and hardlink references to prior regular objects.
Hardlinks require matching mode/mtime. Paths are bounded to255 bytes; headers
are bounded by `3 * max_objects`, total bytes by
`max_storage_bytes + 3072 * max_objects + 1024`. Traversal, conflicting parents,
forward/invalid aliases and malformed metadata fail without publication. Export,
restart and publication preserve mode, hardlink identity and mtime seconds.
USTAR does not persist nanoseconds or atime; unsupported timestamps fail export.

Diagnostics carry bytes without checkpoint or result authority. Shared sender
locking prevents interleaved frames. Chunks≤768 bytes, total16MiB or32,768 frames,
then one empty truncation marker; output continues draining. CLI human output
uses stderr; structured context JSON stays on stdout. Machine lifecycle events
are explicitly requested and cross this diagnostic channel. Raw child progress
may remain in the separately retained supervisor log.

## Recovery, cancellation and optional repair

Identity includes transport version, exact image/worker/input content and modes,
network resolution/policy, defines and target/job/storage choices. Changing these
selects fresh state. Operation, force flags and diagnostic choices do not erase
valid recovery points. Destination locks serialize publication and state joins.
Under the selected state directory (default user-state `recipe`), identities retain
current/previous complete generations across four recent identities. Retention
recognizes its own records/files and preserves unknown operator files. Diagnostic
history retains32 inactive invocations and protects live leases.

Durable `logs/INVOCATION/{result.json,worker.log,supervisor-stderr.log,lease}`
survives temporary-root cleanup. Each log is bounded16MiB. Results distinguish
success, workload/protocol/startup/storage failure, deadline and cancellation.
CLI SIGINT/SIGTERM forwards to the process group and awaits teardown (SIGINT exit130).
Library callers opt in to signal installation. Coordinator death also terminates
the supervisor/workload; it cannot publish unfinished work.

`RECIPE_FRAME_TIMEOUT_SECS` (default300, range1..86400) bounds startup and partial
frame transport. After worker-started traffic, a silent legitimate phase is
allowed until the separate total invocation deadline; receiving the first byte
starts a complete-frame deadline. Partial traffic cannot extend it indefinitely.

Optional autofix runs outside the domain in the coordinator after an eligible
structured build/install failure. Attempts are explicitly bounded, providers and
patch processes have timeout/cancellation teardown, and readiness/protected edits
are rejected. Accepted source changes are rehashed and launch a fresh identity.
Ordinary execution requires no provider or credentials. Controlled provider
integration proves retry mechanics; it does not claim external-provider behavior.

See the [task queue evidence](recipe-simplification-implementation-plan.md#current-checkpoint)
for actual workloads, revisions and limits on kernel, full-ISO and VM validation.
