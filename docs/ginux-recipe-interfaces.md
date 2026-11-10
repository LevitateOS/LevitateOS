# Ginux–Recipe execution interfaces

This describes the current local integration. The [production-readiness queue](ginux-contribution-task-list.md)
owns remaining work; the [completed implementation plan](recipe-simplification-implementation-plan.md)
and source-specific validation records retain historical scope and evidence. Recipe API2, worker
transport9 and Ginux instance schema1 are independent contracts. Coordinator,
worker and retained records use one current interface; mismatched versions fail
before recipe setup. This record does not authorize upstream publication.
Configuration uses `[instance]` and a derived `[userspace.contract]` pinned to
configured downstream catalog 1002 and the reviewed build-base digests. It requires
`configured-network-audit=1 configured-denial-audit=1` from `instance-validate`.
Upstream Gate 11 catalog 2 and native policy stay separate. [PRD02's review](ginux-contributions/prd02-contract-review.md)
records the prior profile-based reconciliation; [PRD18's active checkpoint](ginux-contributions/prd18-implementation.md)
records the new implementation, exact-source checks and remaining decisions.

[Recipe configured contract 1](ginux-contributions/prd08-public-contracts.md)
records the user-approved release policy and exact limitations. The current
configured runtime preserves global clocks, confines large-image-read admission
to configured launch and returns zero-filled configured sysinfo. Native policy
choices remain separate. The [PRD15/19 checkpoint](ginux-contributions/prd15-product-contract.md)
binds the tested runtime and remaining source-delivery/security acceptance.

## Caller and release boundaries

The builder uses the current Recipe library and its matching CLI (or explicit
`RECIPE_BIN`), binds typed declaration inputs and consumes validated named
outputs. It forwards `GINUX_BIN`, `GINUX_ROOT`, `GINUX_ROOTFS`,
`RECIPE_WORKER_BIN` and `RECIPE_FRAME_TIMEOUT_SECS`; other ambient Recipe/Ginux
controls are removed. Whole-coordinator bootstrap privileges and appropriately
privileged consumers are required; launcher-only sudo wrappers lose channels.
See [Linux setup](../tools/recipe/docs/setup-linux.md) and
[runtime limits](../tools/recipe/docs/runtime-limits.md).

The included product contract is the queue's PRD19 table, for both distros.
Build a fresh base-rootfs → live-boot → live-tools chain explicitly; existing
parent artifacts are consumed as they stand. Installed-system policy belongs
to the recipe, live/test policy to its overlay, deployment to recstrap and A/B
activation to recab. Metadata EROFS production and actual deployment need their
separate matching receipts. Guest installer PTY interaction is separate from
version/help and split-pane smoke controls. Neither an assembled ISO nor an
older VM pass certifies the final delivered combination. Maintained source,
integration of subsequent local corrections, final product acceptance and security
remain open as recorded in the
[PRD15/19 checkpoint](ginux-contributions/prd15-product-contract.md).

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
A successful worker frame precedes final archive export: the coordinator awaits
actual launcher completion within the remaining execution deadline. The short
teardown grace applies to errors/cancellation. A killed/nonzero launcher cannot
publish products even if the worker and an existing result file reported success.

`GINUX_BIN` selects the launcher. `GINUX_ROOTFS` selects an assembled uncompressed
tar; `GINUX_ROOT=oci:NAME@sha256:DIGEST` identifies those exact tar bytes, not an
OCI manifest. The input owner snapshots and hashes the selected archive, extracts
without following links, and creates a private invocation root. It imports the
matching Recipe worker (`RECIPE_WORKER_BIN` for library callers), recipes and
explicit selected inputs. Roots live until the supervisor is reaped, then are
removed. Active leases prevent concurrent pruning.

## Instance paths and recipe contract

| Purpose | Path and authority |
| --- | --- |
| Worker, recipes, modules, declared assets/source closures | `/inputs`, immutable snapshots |
| Work/build directory | `/work/build`, owned writable storage |
| Dependency tools and cache | `/work/tools`, owned writable storage |
| Temporary files | `/work/tmp`, owned writable storage |
| Installed target and selected products | `/out`, owned writable storage |

Recipes declare `const RECIPE_API = 2;` and their own literal `const NEEDS`.
NEEDS owns required/optional typed inputs, exact typed products, endpoint grants
and upper limits; the coordinator reads it without evaluating setup. Setup runs once per recipe evaluation
and reads immutable `RUNTIME`. Phase hooks receive `ctx.runtime`, resumable
`ctx.state` and named `ctx.outputs`. Package facts come from current source.
Readiness is `ready(ctx)` or `needs_work(ctx, reason)`; thrown errors fail the
operation. Native errors and persisted booleans cannot masquerade as readiness.
Dependencies run their own checks and state in separate Ginux instances with
separate grants and state/files pairs. Callers bind `DEP::NAME` inputs explicitly;
parent inputs/grants are never lent. Manual readiness commands require matching
cached dependency products, validated through installed checks in graph order;
they never repair a miss by running dependency acquire/build/install hooks.
Those same product snapshots retain the installed parent's input identity.
Declared outputs are snapshotted under
`/inputs/recipe-dependencies/DEP/OUTPUT`, and directory outputs named `tools`
contribute to the consumer's private tools prefix. Completion rechecks installation
after success cleanup. Cleanup is optional; successful failure cleanup becomes a
recoverable checkpoint while the original producer failure remains authoritative.

`--input NAME=PATH` binds a declared `file`, `directory`, `file?` or
`directory?` input. Required missing, unknown, duplicate or wrong-kind inputs fail
preflight across the complete declared graph. Source imports omit root
`.git`/`target`, preserve contained relative links and reject escapes/cycles.
Named filesystem inputs `rootfs` and `target` preserve filesystem link bytes,
including absolute links meaningful in a future root. Imported hardlinks share
private snapshot backing; they never alias mutable host input bytes. A
coordinator-authored permission manifest lets copying helpers preserve original
logical modes despite read-only imports. A `target` directory seeds `/out` only
for fresh state; an acknowledged matching filesystem takes precedence on reuse.

NEEDS.outputs declares exact names and file/directory kinds under owned storage.
The worker checks returned names/kinds; the coordinator independently prepares
and validates the export privately before committing final state/files and
publishing fresh paths. Complete installation and installed reuse require the
exact output set. Intermediate phases may expose a subset; every present output
is checked before checkpoint/publication. Failed work cannot replace previous publications. Returned
`ctx.outputs` contains those host paths; the worker never selects host destinations. The
coordinator adds completion metadata at `ctx.runtime.contract` and
`ctx.runtime.network_usage`, preserving ordinary top-level recipe facts. Each
invocation saves its actual declaration/config, pins and contract in
`authority.json`. Invocation explanations include that saved `execution`, making
resolved bounds and caller restrictions visible alongside the declaration.
Finite Ginux endpoint and contract-denial summaries remain
available after syscall-trace truncation; a missing observation is explicitly
unknown, never an implicit success or proof that a grant was unused.

## Options, bounds and network policy

Resolved execution values cross the request explicitly: native target,
`jobs`1..16, phase force flags, timeout, memory/storage bounds, network endpoints
and machine events. NEEDS supplies authority; caller restrictions may only lower
limits/select endpoint subsets. Main-recipe force flags are cleared for
dependencies. Missing NEEDS limits default to 3600 seconds, 2048MiB, 16,384
storage objects and 8GiB network bytes; a missing network grants none. Explicit
caller restrictions cap every dependency's own limits and intersect its endpoints.
They never lend grants. Manual readiness checks require matching cached
dependency products and never install them. Explicit dependent-recipe `remove`
and `cleanup` selection remains unresolved by user direction; this release
cannot claim that boundary accepted until a separate decision and verification.
No coordinator-wide environment mutation supplies invocation state.

Resource enforcement is specific: Ginux applies `memory_mib` as inherited
**per-process virtual address-space** limits (`RLIMIT_AS`) and constrains the
child CPU affinity to at most the requested available CPUs. This is not aggregate
domain RSS or cumulative CPU time. Owned storage has separate byte/metadata
accounting, with explicit object/data limits; shared hardlink bytes count once.
If `max_bytes` is omitted, the actual data cap subtracts page-rounded namespace/backing-table
metadata from the selected memory budget. Recipe uses the full memory value only
as a conservative outer archive bound. Public descriptors and tasks remain bounded. Supervisor-private backing FDs
reserve capacity independently of the public descriptor table and respect the
host hard limit. Unsupported operations fail; this is not complete Linux ABI
coverage. Large source consumers select sufficient explicit storage/memory policy
in their literal recipe NEEDS rather than relying on small-tool defaults.

Outbound networking requires NEEDS.network endpoints; repeated
`--network-host HOST:PORT` (or library `ExecutionOptions.narrowing`) can only
select a subset. Empty policy produces `network.backend="none"`.
The coordinator resolves at most eight numeric TCP endpoints before launch,
with bounded resolver time/output, and writes an immutable hostname map inside
the private root. Ginux enforces destination, simultaneous connection (16) and cumulative
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

The config retains Ginux schema1, the derived configured catalog-1002 contract,
ptrace-native backend,
stderr audit and stop-on-failure policy. Recipe checks config≤4096 bytes,
id≤64 bytes, paths≤128, args≤7×128, env≤8×256 and assignment bounds before launch.
The environment contains fixed domain PATH, worker/invocation identity and TMPDIR;
only explicit allowlisted toolchain variables may be added. Host HOME, checkout
paths, credentials and ambient proxy variables do not cross this boundary.

## Worker transport9

Each frame is `[u32 BE length][u32 BE version=9][u32 BE sequence][u16 BE op]`
followed by UTF-8 JSON. Length includes the 14-byte header and is at most4MiB.
Each direction starts at sequence1; gaps, replay, wrong direction, unsupported
versions and truncated/oversized payloads abort that invocation.

| Op | Direction | Payload |
| --- | --- | --- |
|1 request | Coordinator→worker | invocation_id, operation, persist_state, cleanup_reason, options, restored_states |
|2 checkpoint | Worker→coordinator | seq, canonical `/inputs/...` recipe identity, state object |
|3 ack | Coordinator→worker | checkpoint seq and durable generation |
|5 result | Worker→coordinator | ok, optional typed failure (kind, phase, message), ctx_summary; sent exactly once |
|6 diagnostic | Worker→coordinator | stream, raw byte array, truncated marker |

Op4 is retired. Transport9 rejects older coordinator/worker grant models before
setup. Each separately launched recipe has state keyed by its staged identity; package facts and runtime
paths are not restored from saved source text. A checkpoint pauses recipe progress
until Ginux exports owned files and the coordinator durably records their matching
state generation. Ack follows that durable pair. Interrupted writes retain the
last complete generation. `--no-persist-state` suppresses reuse and durable state
claims, acknowledging a null generation. It does not enable source rewriting.

Snapshots use validated USTAR with GNU longname/longlink extensions, regular
files, directories, symlinks and hardlink references to prior regular objects.
Hardlinks require matching mode/mtime. Absolute domain paths are bounded to255 bytes including their leading slash
(tar member names at most254); individual file sizes must fit the USTAR size
field (less than8GiB). Headers
are bounded by `3 * max_objects`, total bytes by
`max_storage_bytes + 3072 * max_objects + 1024`. Traversal, conflicting parents,
forward/invalid aliases and malformed metadata fail without publication. Export,
restart and publication preserve mode, hardlink identity and mtime seconds.
USTAR does not persist nanoseconds or atime; unsupported timestamps fail export.
Owned objects currently project virtual uid/gid0; chown preserves0 or unchanged
IDs and rejects other owners. The supported archive mode contract is basic
permission bits, not arbitrary Linux ownership/setuid metadata. The canonical
QEMU payload excludes its unused privileged host bridge helper explicitly.
Prepared rootfs cases do not establish complete distribution metadata fidelity.

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
