# Ginux–Recipe execution interfaces (F02)

Cross-process contract for Recipe's required Ginux execution environment.
`docs/ginux-recipe-implementation-plan.md` is authoritative on objectives and
ownership; this file fixes the concrete fields, limits, paths, and behaviors
implementation lanes build against. Ginux owns §2 stream/result behavior and
its config/parser changes. Recipe owns §4 framing and the worker/host split.

Current transport update 2026-11-05: version 3 adds required invocation options
to the request: `defines` (up to 64 unique identifier/string pairs) and
`cleanup_reason`. Coordinator and worker change together; old versions fail
explicitly. Managed autofix/LLM profile requests are rejected before recipe
effects. This options change does not implement the state migration below;
that future incompatible state change must bump the transport version again.
The v1/v2 sections below retain their historical payload/diagnostic descriptions.

Direction update 2026-11-05: Ginux is mandatory for executable recipe operations;
macOS support is deferred. The current worker transport is v3 (options update above).
Section 4 retains the original v1 payload/persistence reference; it does not
authorize source rewriting or public host bypasses. Section 7 defines the
separate state migration, which will require a further transport version. Recipe API 2, Ginux
config schema 1 and worker transport versions describe independent contracts.

## 1. Versioning and failure reporting

- Current worker protocol version: `3` (`u32`, every frame header). Version mismatch is
  a `protocol` failure: the receiver aborts, the host reports failure, and no
  retry is attempted on the same invocation.
- Ginux `DomainConfig.schema_version` stays `1`. Recipe generates config text
  the existing `ginux-domain` parser accepts; new fields arrive only through
  Ginux-owned parser changes (see `workdir` below).
- Failure categories, used identically in result records and logs:
  `startup` (bad config, missing launcher, unsupported bootstrap),
  `workload` (nonzero exit / terminating signal),
  `protocol` (malformed frame, version/sequence mismatch, timeout, missing
  result), `storage` (exhaustion, flush failure — never reported durable),
  `cancelled` (host interruption forwarded, teardown complete).

## 2. Ginux launch and stream surface (Ginux-owned)

Recipe spawns a configured launcher executable (path from `GINUX_BIN` env or
backend config; missing/unexecutable launcher is an explicit `startup`
failure — never fall back to host execution).

The reviewed upstream configured-launch surface at 6361bbd5 is
`ginux launch <config.toml> <prepared-rootfs> --result-file <fresh-path>`.
The rootfs argument is an operator/coordinator-selected host artifact path, not
a worker-selected path. Recipe's R06 adapter now supplies this command with a
fresh prepared root. Its former `--config ...` spelling was invalid; unknown
commands now fail explicitly in the local Ginux continuation.

The coordinator selects an assembled, uncompressed tar via `GINUX_ROOTFS`, or
`<user-cache>/recipe/images/<digest>.tar` when unset. `GINUX_ROOT` names its
content as `oci:NAME@sha256:<assembled-tar-sha256>` (the pinned Rocky identity by
default); an OCI manifest digest is not an assembled-tar digest. Recipe snapshots
and hashes the bytes it extracts, rejects mismatches and unsupported entries,
and builds an invocation-private root. Absolute image symlinks remain valid
inside chroot, while extraction never writes through any symlink. `/inputs`,
`/work`, and `/out` are reserved to the coordinator. No mutable hardlinks to the
image cache are shared with the domain.

The `recipe` CLI copies itself to `/inputs/recipe-worker`; library callers select
an installed Recipe binary via PATH or `RECIPE_WORKER_BIN`. This replaces the
former `RECIPE_LAUNCH_WORKER` stub scaffolding. Selected staged inputs are copied
and verified again against their discovered sizes/hashes. The root remains alive
until the supervisor is reaped, then is removed. This root is currently ephemeral:
retained writable storage, generation-backed checkpoints, full input/dependency
layout reconstruction, and selected output publication still need integration.

Current writable adapter (local Ginux 10091e4e, Recipe ab9fc89): generic
`memory:/domain/path` storage assignments provide empty ephemeral namespaces at
`/work`, `/work/build`, `/work/tools`, `/work/tmp`, and `/out`. Ginux owns their
bytes/descriptors; native image access remains read-only and missing owned paths
never fall through to host storage. Current limits are 32 objects including
seeded directories, 64 KiB/file, 64 open files, and 4096-byte IO calls. The generic
backend's supported operations and limitations are documented in Ginux README.
This is not retained storage, ordinary compiler support or publication. The v1
ack generation remains placeholder 0; result generation remains null. The stable
paths table below describes the target backing, not completed retention.

Current device adapter (local Ginux e3458063, Recipe 50edb61): Recipe supplies
`[devices].assignments = ["null"]`. The configured `/dev` view is virtual and
immutable; image entries there are hidden. The explicitly assigned null device
uses Ginux's existing device registry, returning EOF/discarding bounded writes
and reporting character-device metadata. Device descriptors use a reserved range,
with native limits constrained even for device-only configurations. Other device
assignments and duplicate assignments fail before launch. Pipe2 and child
execution remain unfinished; this does not enable a host device or native child
fallback.

Current pipe adapter (local Ginux 7f072f2c/f1c6419e): pipe/pipe2 create
bounded supervisor-owned byte queues with virtual handles, safe copyout,
small-write atomicity, aliases and EOF/SIGPIPE. Blocking calls can remain stopped;
configured multi-task wakeup and complete child execution are not integrated.
The current separate public file/device/pipe ranges conflict with libc's fd-limit
preflight, so the next adapter revision must provide a common table of small
public descriptors while retaining private backend handles. Raw pipe-data tests
are not evidence of successful libc spawn. Keep the byte protocol unchanged.

Current descriptor adapter (Ginux 3b00c1c8) supersedes the sparse public
ranges mentioned above. One table allocates up to 256 small public descriptors
across image files, writable files, devices, pipes and stdio. Backend handles
remain private. Aliases share an open description and retain it through the last
close; public descriptor flags remain independent. Native operations translate
arguments/results, including mmap, and restore argument registers at exit. Real
soft/hard limits are capped at 256, preserving lower operator limits; reporting
is not inflated. Libc spawn preflight succeeds. The configured task loop still
denies clone3/clone: inheritance, peer scheduling and child teardown are next.
Relative directory-fd access and readiness beyond original bootstrap consoles
remain unsupported. The worker byte protocol is unchanged.

Per invocation Recipe supplies:

- A domain config document: `execution.profile = "balanced"`,
  `execution.backend = "ptrace-native"`, `contract.profile = "balanced"`,
  `network.backend = "none"`, `audit.mode = "stderr"`,
  `failure.policy = "stop"`, plus invocation-specific `domain.id`,
  `domain.root` (immutable image reference), `domain.init` (worker path),
  `domain.args`, `domain.environment` (allowlisted, see §5),
  `resources` bounds, and `storage`/`devices` assignments.
- Current `ginux-domain` hard limits Recipe must fit (no silent truncation;
  R05 fails explicitly when inputs exceed them): id ≤ 64 B, root/init ≤ 128 B,
  args ≤ 7 × 128 B, environment ≤ 8 × 256 B, storage/devices ≤ 8 × 64 B,
  normalized config ≤ 4096 B.
- `workdir`: optional `[domain]` field implemented by G01, a domain path with
  default `/work`.
- Streams: worker process stdin/stdout are the opaque protocol pipes (§4).
  Worker stderr carries human logs (drained to a Recipe log file, never
  parsed). Supervisor diagnostics go to launcher stderr prefixed `ginux:`.
- Completion record (G02-owned, machine traffic): the supervisor writes one
  JSON object to a host-side `--result-file <fresh host path>` Recipe creates:
  `{"version":1,"outcome":"exited|signaled|supervisor-error",`
  `"code":<i64|null>,"signal":<u64|null>,"generation":<u64|null>}`.
  The launcher exit code mirrors the workload code when the supervisor is
  healthy, but the result file is authoritative. Missing or malformed result
  file is a `protocol` failure, never success.

## 3. Stable paths

Domain (fixed across retries; host `.artifacts` paths never enter worker
state or environment):

| Purpose | Domain path | Backing |
| --- | --- | --- |
| Imported inputs (worker binary, recipes, modules, assets, source closure, caches) | `/inputs` | Immutable image (G10) |
| Working directory | `/work` | Writable retained storage |
| Build tree (`BUILD_DIR`) | `/work/build` | Writable retained storage |
| Dependency tool tree | `/work/tools` | Writable retained storage |
| Temp (`TMPDIR`) | `/work/tmp` | Writable retained storage |
| Install output staging | `/out` | Writable retained storage, exported on success (G13) |

Host staging: `.artifacts/ginux-export/<invocation-id>/`, created fresh per
invocation by the coordinator (mode `0700`). Export lands only there; the
coordinator publishes the declared artifact tree from that staging directory.

## 4. Worker protocol v1 reference (Recipe-owned framing, opaque bytes to Ginux)

This records the original protocol, including its source-persistence policy.
New coordinator state work follows section 7 and uses an explicit transport
version change for incompatible payloads. Ginux's byte channel is unchanged.

- Frame: `[u32 BE total-length][u32 BE version=1][u32 BE seq][u16 BE op]`
  followed by `total-length − 14` payload bytes (JSON, UTF-8, `≤ 4 MiB` total
  frame; header is 4+4+4+2 = 14 bytes and `total-length` is the full frame
  size including its own 4 bytes, per R01). Each direction numbers its frames
  from seq 1; any gap, replay, or oversize frame is a `protocol` failure.
  Payloads carry no host paths.
- Ops: `1 = request` (host→worker: `{invocation_id, operation, persist_ctx,
  recipe_digest}`), `2 = checkpoint` (worker→host: `{seq, phase, reason,
  main_ctx, base_ctx, base_path}` with `*_ctx` as JSON objects),
  `3 = ack` (host→worker: `{seq, generation}`), `4 = failure`
  (worker→host: `{fatal: bool, phase, reason, message}`),
  `5 = result` (worker→host: `{ok: bool, failures: [...], ctx_summary}`).
- Checkpoint acknowledgement (binding order): worker sends `checkpoint` and
  blocks (bounded wait, 300 s default); host quiesces/flushes domain storage
  to a durable generation, persists the acknowledged `ctx` content into the
  trusted main/base recipe source files with existing atomic behavior
  (honoring `--no-persist-ctx` by skipping the file write but still acking the
  generation), records `{seq → generation}` in invocation-local state, then
  sends `ack`. Interruption before the ack leaves the previous acknowledged
  pair; after the ack the newer pair is recoverable. The worker never writes
  recipe sources; the host never invents `ctx` content.
- Termination: the worker sends `result` exactly once, then exits 0. A worker
  exit without `result`, or a nonzero exit, is a `protocol`/`workload`
  failure using the §2 result file, retaining the last acked pair.

### 4.1 Historical diagnostic transport v2

The v2 coordinator and worker used header version 2 and rejected version 1 before
recipe setup. Framing stays bounded to 4 MiB with 14-byte headers, independent
per-direction sequence numbers and strict direction checks. Ops 1–5 retain
their v1 payload shapes for now; generation 0 is still a placeholder, not a
claim of durability. No runtime version fallback or source-writing is added.

Op 6 is worker→host diagnostic only:

    {"stream":"stdout|stderr|recipe","data":[0,255,...],"truncated":false}

`stream` distinguishes child stdout/stderr and Recipe narration/print/debug/log/
warn. `data` is a JSON byte array, preserving non-UTF-8 bytes. A data frame has
1–768 bytes; an empty frame is valid only with `truncated:true`. Diagnostic
fields have no result, checkpoint, installation or filesystem authority. The
coordinator writes these bytes only to CLI stderr; JSON results stay on stdout.
Supervisor stderr remains an opaque separately drained audit stream. It is
never parsed as a control message or mixed into successful user diagnostics.

All worker sends share a sender lock, including output-reader threads. Frames
cannot interleave. Checkpoint sends release the lock before waiting for ack.
Reads/writes use at most 4096 bytes per IO call, so large legal contexts still
cross the configured console surface. This does not change the frame-size cap.

The invocation budget is 16 MiB of raw diagnostic bytes or 32768 data frames,
whichever is reached first. If more output arrives, the worker sends exactly one
empty truncation marker, then keeps draining without sending further diagnostic
frames. The coordinator checks both budgets and marker order independently,
emits one truncation notice, and continues accepting lifecycle frames/results.
Limits do not turn a failed recipe into success. A broken diagnostic transport
prevents successful termination. A finished operation closes diagnostic sending
before its terminal frames. Complete-frame deadlines now cover headers, payloads and writes, using an
absolute deadline across short IO. RECIPE_FRAME_TIMEOUT_SECS selects 1–86400
seconds (default 300). Shutdown and stderr collection are bounded. Coordinator
death stops the launcher and traced workload; resumable cancellation remains open.

The real CLI cases cover binary bytes, Rhai logging, large contexts, concurrent
child readers, the byte and frame budgets, and v1 rejection before setup.

## 5. Environment and private worker bootstrap

- Worker environment is an explicit allowlist only. PATH searches
  `/work/tools/{usr/bin,usr/sbin,bin,sbin}`, `/work/tools`, `/usr/bin`, and `/bin`;
  `RECIPE_WORKER=1`, `RECIPE_INVOCATION=<id>`, `BUILD_DIR`, `TMPDIR`,
  `ARCH`, `NPROC`, plus toolchain variables recorded by I01/R05. No `HOME`,
  no absolute host checkout paths, no secret-bearing variables.
- `invocation_id` format: `[A-Za-z0-9_-]{1,64}`, generated by the host
  (`<recipe-name>-<unix-millis>-<pid>`).
- Worker executable: the staged Recipe binary starts as domain init with
  private argv `--__recipe-worker /inputs/main.rhai`. It validates bootstrap
  arguments/environment and requires the request's invocation id to match the
  bootstrap id before evaluating source. Worker environment variables do not
  select host execution in public CLI/library calls. Public `Backend` has only
  Ginux; the internal worker invokes the shared executor directly.
- The prepared `/work/tools` path is shared by readiness, install/remove/cleanup,
  dependency execution, `TOOLS_PREFIX`, API 2 runtime paths, and command lookup.
  Removal receives the prepared BUILD_DIR just like the other worker operations.
- Assigned bootstrap descriptors support bounded `poll` waits from 0 through
  300000 ms. Infinite/larger waits, counts above 16, invalid arrays, and
  descriptors outside 0/1/2 (except ignored negative slots) are rejected.
  This permits checkpoint acknowledgements without opening unrelated host fds.
- Privilege: Recipe never requires host root. If the supported Ginux
  bootstrap mechanism needs unavailable privilege, G01 reports an explicit
  `startup` failure.

## 6. Capability ownership

Task status and evidence live in the implementation plan; this list assigns
interfaces to owners and does not assert that every listed capability is missing.

- G01: configured executable/args/env/cwd (plus `workdir` field above).
- G02: faithful exit/signal status and the §2 result file.
- G10/G11/G12/G12a: inputs packaging, retained backing, flush, generation
  checkpoints (R07 consumes generations via `ack`).
- G13: selected-tree export into fresh host staging.
- G14: opaque bidirectional byte transport the §4 frames ride on.

## 7. Required execution and separate state migration

The [Recipe simplification plan](recipe-simplification-implementation-plan.md)
owns S0 (mandatory Ginux) and S3 (separate state). All public CLI/library operations
that evaluate Rhai run in a Ginux domain, including setup, imports, checks,
dependencies, helpers, cleanup, and removal. Public host/in-process selection and
host defaults are removed. A missing launcher, environment, or capability fails
before host-side recipe effects. An environment variable alone must not bypass
the coordinator. Static metadata/hash operations can stay on the host without
evaluating recipe code. Worker-local interpretation remains an internal operation
inside the managed domain.

Custom is_acquired/is_built/is_installed logic stays inside the worker with the
rest of recipe execution. Ginux transports state and enforces execution policy;
it neither evaluates readiness nor substitutes file-existence checks. Recipe's
explicit ready/needs-work/error contract is independent of transport version.
Requested target identity and paths arrive in prepared invocation inputs. Checks
operate on that target's imported/retained view, not implicitly on the base image.

The target persistence contract is:

- Recipe sources, base recipes, and modules are immutable inputs. The coordinator
  derives a separate runtime-state destination from trusted build/installation
  identity and associates it with the invocation and durable storage generation.
- The worker reports serializable runtime state and named outputs. It cannot
  select an authoritative host path or request that source files be rewritten.
  Separate declared package facts and current runtime paths from persisted state.
- Resume imports only state matching the input identity and retained filesystem
  generation. Saved context never silently overrides current package facts.
- Checkpoint order stays: worker checkpoint, durable domain generation, durable
  matching state record, acknowledgement. Before acknowledgement, the previous
  acknowledged state/files pair remains recoverable. Output publication still
  requires final success and validation of the declared output tree.
- Disabled state persistence permits execution but does not promise resumable
  recovery. The existing --no-persist-ctx option becomes a documented compatibility
  alias for that policy; it does not disable filesystem effects.

Named path products use `ctx.outputs`, a map from selected product names to
nonempty absolute domain-path strings. Rust/TOML consumer contracts select names
and filesystem kinds; the worker does not choose authoritative host destinations.
The current builder consumers require `rootfs` (directory), preseed `iso` (file)
and Alpine `apk_tools` (file), or kernel `kernel` (file) and `modules` (directory).
Kernel `source` is an optional directory; kernel release metadata is the required
`ctx.kernel_version` string. Successful custom readiness checks rediscover the
same names when an installation is reused.

R08 must resolve selections in the matching domain view/generation, export them
through confined coordinator-owned staging, validate them, and return published
host paths under the selected names only after successful publication. Forwarding
a worker's path and finding a file at that path on the host is not publication
evidence. Consumer filesystem-kind checks supplement custom readiness and the
coordinator's publication guarantees; they do not grant host-path authority.
The current final-export bridge (Ginux 48b26f8d, Recipe 11057ca) publishes selected
regular files/directories under /work or /out into fresh coordinator-owned
.artifacts/ginux-export/<invocation>/<name> directories and returns host paths.
It requires successful completion, bounds and validates the archive, rejects
links/traversal/missing selections, and never overwrites an existing publication.
Current limits are 64 named outputs, 4096 archive entries and 2 GiB. Ginux exports
the owned writable view after task teardown; Recipe publishes only selections.
This proves final publication, not retained generations or resumable recovery.
Image-backed selections and symlink-containing products remain unsupported.

Preseeded source assets are explicit inputs to a materializer. The builder selects
the canonical host cache through orchestration metadata; input preparation maps
selected assets and any recipe-used trust records into the domain. The worker
receives their domain paths through invocation inputs rather than reconstructing
a sibling downloads directory from BUILD_DIR. Filenames, hashes, and verification
rules remain recipe-owned. A host pathname supplied as a define does not import
an asset or grant the worker access to it.

Current R05 preparation exposes explicit file/tree imports through --input and
the library, with a 512 MiB per-file and 2 GiB total limit. Tree inputs preserve
relative paths up to 247 bytes (plus /inputs/ and NUL in the launcher buffer),
reject symlinks, and omit root .git/target. These bounds still cannot accommodate
the canonical Fedora DVD. R07/input
integration must provide bounded support for the selected real assets and expose
their domain paths before materializer cache inference can be removed end to end.
This is an outstanding interface requirement, not completed transport evidence.

Dependency checkpoints also require explicit recipe identity. The current v1
checkpoint contains only a main-recipe context, so it cannot safely carry a
dependency's context. The shared dependency executor rejects persistent worker
dependency execution under v1 before dependency setup. With persistence disabled,
dependency phases run without independent checkpoint acknowledgements; parent
checkpoints still use the existing v1 stream, and no resumable dependency state
is claimed. Dependency source files remain unchanged. R07 must replace this
restriction with per-recipe state/identity associated with acknowledged storage
generations; never serialize a dependency as the parent's main context.

R05 discovery now follows extends/imports and literal deps/build_deps arrays,
preserving nested source/module references. Explicit --input selections stage
regular files and trees; symlinks are rejected. Dynamic declarations fail during
preparation. Static preparation must fail
usefully for unsupported dynamic discovery rather than evaluating recipes on the
host. Ginux transports that closure; the shared Recipe executor determines which
checks and phases actually run. The dependency graph CLI case requires an actual
C compiler in the prepared domain and does not substitute a host-built tool.

Keep the historical v1/v2 reference distinct from the current v3 options contract.
Implement the incompatible state checkpoint/result migration with another
transport version bump and matching coordinator/worker mismatch rejection.
Recipe API 2 and worker transport versions are independent. In the replacement checkpoint,
state and outputs replace main_ctx/base_ctx/base_path as persistence data; input
source identity belongs to prepared inputs, not to a write destination. Keep the
existing bounded framing, sequencing, failure reporting, and opaque Ginux channel.
Ginux does not parse or version Recipe's JSON state semantics.

Deliver the coordinator and worker update together under R07/Recipe S3. Fresh
invocations need no saved state. Repeated invocations restore only a matching
state/files pair; corrupted or incompatible records fail with useful recovery
guidance. Keep publication status separate from phase completion so an export
failure is not reported as a completed installation. Exercise protocol mismatch,
interruption on both sides of acknowledgement, disabled persistence, and source
immutability through the actual managed execution path.
