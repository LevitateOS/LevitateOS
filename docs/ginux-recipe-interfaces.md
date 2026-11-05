# Ginux–Recipe execution interfaces (F02)

Cross-process contract for Recipe's required Ginux execution environment.
`docs/ginux-recipe-implementation-plan.md` is authoritative on objectives and
ownership; this file fixes the concrete fields, limits, paths, and behaviors
implementation lanes build against. Ginux owns §2 stream/result behavior and
its config/parser changes. Recipe owns §4 framing and the worker/host split.

Direction update 2026-11-05: Ginux is mandatory for executable recipe operations;
macOS support is deferred. Sections 1–6 retain the existing v1 interface
reference. Sections 4 and 5 describe legacy worker/persistence behavior; they
must not be used to implement new source-rewriting or public host bypass paths.
Section 7 defines the target migration and supersedes those behaviors. This
document change does not claim
that the new execution policy or transport revision is implemented.

## 1. Versioning and failure reporting

- Worker protocol version: `1` (`u32`, every frame header). Version mismatch is
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
failure — never fall back to host execution). Per invocation Recipe supplies:

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
- `workdir`: new optional `[domain]` field owned by Ginux (G01), domain path,
  default `/work`. Until it lands, the worker starts at `/work` by supervisor
  convention.
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
| Tool tree (`.tools`) | `/work/tools` | Writable retained storage |
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

## 5. Environment and legacy worker bootstrap

- Worker environment is an explicit allowlist only: `PATH=/work/tools:/usr/bin:/bin`,
  `RECIPE_WORKER=1`, `RECIPE_INVOCATION=<id>`, `BUILD_DIR`, `TMPDIR`,
  `ARCH`, `NPROC`, plus toolchain variables recorded by I01/R05. No `HOME`,
  no absolute host checkout paths, no secret-bearing variables.
- `invocation_id` format: `[A-Za-z0-9_-]{1,64}`, generated by the host
  (`<recipe-name>-<unix-millis>-<pid>`).
- Worker executable: R02 uses the Recipe executable in worker mode, packaged
  inside `/inputs` and started as domain init. Its existing RECIPE_WORKER=1
  dispatch is legacy behavior. S0/R06 restrict dispatch to the internal worker
  entry path; setting this variable in the public CLI/library environment must
  not enable host recipe execution. The worker never recursively launches Ginux.
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

Keep the current v1 framing/payload reference intact while its callers exist.
Implement incompatible request/checkpoint/result changes as transport v2 with
matching coordinator and worker changes and explicit mismatch rejection. Recipe
API 2 and transport v2 are independent versions. In the replacement checkpoint,
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
