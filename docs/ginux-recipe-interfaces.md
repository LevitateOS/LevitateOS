# Ginux–Recipe execution interfaces

Status of 2026-11-23: this describes the **public-lifecycle cutover**, written
and never run. Recipe no longer calls the downstream `ginux launch`; it runs each
recipe that starts programs as an instance of Ginux on the public lifecycle.
The design is `tools/recipe/docs/ginux-cutover-design.md` (fixed record), the
plan and its status are `tools/recipe/docs/ginux-public-lifecycle-migration.md`
(section 6), and the limits Recipe states are
`tools/recipe/docs/runtime-limits.md`. The [production-readiness
queue](ginux-contribution-task-list.md) owns remaining work. Recipe API 1, the
worker records version and the Ginux instance schema 1 are independent
contracts; coordinator, worker and retained records use one current interface
and mismatched versions fail before recipe setup. This record does not authorize
upstream publication. Everything about frames, checkpoints, autofix, catalog 1002
and `GINUX_BIN`/`GINUX_ROOT*` that earlier versions of this page recorded is
gone (see `tools/recipe/CHANGELOG.md`).

## What the interface is

The coordinator (`recipe`) drives the `ginux` CLI, which drives the
root-owned helper through `sudo`, behind Recipe's root-owned guard
(`/usr/libexec/recipe/recipe-ginux-guard`, the only program the sudoers rule
names: it allows Recipe's instance verbs on `recipe-<uid>-*` only, re-renders
the imported config from validated fields, accepts an image for a non-root
caller only if its reference is in the root-owned allowlist
`/usr/libexec/recipe/runtime-images`, and clears the environment).
Instance directories are root-private (0700) and the helper streams the console
output to the caller (`start --wait --stream-console`). `import-image` is root's
only (`sudo recipe runtime fetch`), never through the guard; the shared-host
exposure that remains (R13-R15, no per-uid caps) is in
`tools/recipe/docs/runtime-limits.md`:

```text
ginux --state-root S --run-root R import-image LAYOUT      once per host and image, as root (not via the guard)
ginux ... import NAME instance.toml                        create
ginux ... inject  NAME --input inject.tar --into / ...     the worker, request and inputs, one tar
ginux ... start   NAME --wait                              run to completion
ginux ... status  NAME --format json                       the outcome record (authority)
ginux ... extract NAME --output EXPORT ... /work /out      the results, one tar
ginux ... delete  NAME                                     always, on every path out
```

Each run is one instance named `recipe-<uid>-<pid>-<random>`; the invocation log
records the name. Recipe's configuration is Ginux schema 1: `[instance]`
(root = `local/recipe-compiler@sha256:<manifest digest>`, init
`/inputs/recipe-worker`), `[resources]`, `[execution] backend = "ptrace-native"`,
`[access] uid/gid = 1000`, `[contract]` at **catalog 2** (the derived allow/deny
lists and the reviewed build-base digests, one with and one without an outbound
grant), `[contract.filesystem]` (read `/`, write and create `/work` and `/out`),
`[contract.network]` (`outbound = ["tcp:PORT", ...]`, only with a grant),
`[storage] assignments = []`, `[devices]` (`null`, `random`, `urandom`),
`[network] backend = "shared"` with a grant and `"none"` without, audit enabled
and stop-on-failure policy. Recipe validates the complete derived contract
(`ginux validate`, whose printed digest is checked against Recipe's pin; the
compiled-in catalog it also prints is not trusted) before creating the instance.
The digests are placeholders until the verification pass derives them from a
built fork binary (design section 6), so until then every Ginux validation
fails by design and names the digest Ginux reported.

## Caller and release boundaries

The builder uses the current Recipe library and its matching CLI (or explicit
`RECIPE_BIN`), binds typed declaration inputs and consumes validated named
outputs. For a run it copies the Ginux CLI, helper, image tool, runtime and
allocator into a root-owned folder under `/var/lib/levitate-builder/ginux/`,
writes a root-owned wrapper there that exports the runtime and image-tool paths
and execs the helper copy, and sets `RECIPE_GINUX` (the CLI),
`GINUX_INSTANCE_HELPER` (the wrapper), `RECIPE_IMAGE[_LAYOUT]` and
`RECIPE_MINIMAL_IMAGE[_LAYOUT]` and `RECIPE_WORKER_BIN`, whatever the
environment says; it forwards `RECIPE_OUTPUT_TIMEOUT_SECS`; other ambient
Recipe/Ginux controls are removed. The builder is root, so no sudoers rule is
involved and the host's `/usr/libexec` is never touched. Appropriately
privileged consumers are required for products a root build published. See
[Linux setup](../tools/recipe/docs/setup-linux.md) and
[runtime limits](../tools/recipe/docs/runtime-limits.md).

The included product contract is the queue's PRD19 table, for both distros.
Build a fresh test-base-rootfs → test-live-ssh → test-install-base chain explicitly; existing
parent artifacts are consumed as they stand. Installed-system policy belongs
to the recipe, live/test policy to its overlay, deployment to recstrap and A/B
activation to recab. Neither an assembled ISO nor an older VM pass certifies the
final delivered combination.

## Privileges and hosts

Every privileged verb is the Ginux helper run as root, behind Recipe's guard:
`sudo /usr/libexec/recipe/recipe-ginux-guard --protocol 1
--state-root S --run-root R VERB` (Recipe's verbs only; images for a non-root
caller from `/usr/libexec/recipe/runtime-images`; no `import-image`). Roots run
the helper without a rule; an administrator runs the guard after `recipe runtime stage --sudoers GROUP` wrote
`/etc/sudoers.d/recipe-ginux` (`%wheel` on Fedora, Rocky, LevitateOS and
AcornOS; `%sudo` on Ubuntu), pinned to `/var/lib/recipe/ginux` and
`/run/recipe/ginux`; nobody else can run a recipe in Ginux. The installed
LevitateOS and AcornOS images ship the rule and the five Ginux files
(`/usr/libexec/ginux/{ginux-helper,ginux-image,ginux-runtime}`,
`/usr/lib64/ginux/libginux_allocator.so`, the CLI at
`/usr/libexec/recipe/ginux`), and the compiler and minimal image layouts at
`/usr/share/recipe/runtime/{compiler,minimal}.oci.tar`. The standalone release
installs them as root on Fedora 44 and Ubuntu 24.04 (not yet run on either).
There is no user-namespace launch and no AppArmor profile any more.

## Execution and ownership

Every operation that evaluates recipe code that starts programs uses Ginux;
a recipe that starts none runs on the host (`--execution auto`), and
metadata/hash commands read declarations without evaluating setup. Current
execution is native Linux/x86-64 through Ginux's native configured runtime.

Selection: `RECIPE_GINUX` (the CLI), `RECIPE_GINUX_STATE_ROOT` /
`RECIPE_GINUX_RUN_ROOT` (default `/var/lib/recipe/ginux`, `/run/recipe/ginux`),
`GINUX_INSTANCE_HELPER` (the CLI's own variable, passed through),
`RECIPE_IMAGE` = `local/recipe-compiler@sha256:<OCI manifest digest>` with
`RECIPE_IMAGE_LAYOUT` (the layout directory or its uncompressed tar, needed
only while the image is not imported), and the minimal pair. The old
`GINUX_BIN`, `GINUX_ROOT`, `GINUX_ROOTFS`, `GINUX_MINIMAL_ROOT` and
`GINUX_MINIMAL_ROOTFS` are refused with the name of their replacement. The
image identity is the manifest digest; the import verifies every blob and
prints the digest, which must equal the reference's. Each run's root is a
private instance root made from the imported image; the instance is deleted
when the run ends, and a sweep at the start of the next Ginux-using command
deletes the instances of dead runs of the same user (root: every user).

The coordinator's authority is `ginux status --format json` (the outcome
record `exited`/`signaled`/`supervisor-error`), read after `start --wait`
returns, in agreement with the worker's result file; contradictions, a killed or
nonzero `start`, and a missing result fail the run without publication. The
results come out as one tar of `/work` and `/out` (`ginux extract`, bounded by
the recipe's storage bounds) only after a clean `exited` outcome.

## Instance paths and recipe contract

| Purpose | Path and authority |
| --- | --- |
| Worker, request file, recipes, modules, declared assets/source closures | `/inputs`, copies injected before the run, read-only by contract and mode |
| Work/build directory | `/work/build`, the instance root |
| Dependency tools and cache | `/work/tools` |
| Temporary files | `/work/tmp` |
| Installed target and selected products | `/out` |
| Hosts file | `/etc/hosts`, always present: `localhost`, then the pinned names |

Recipes declare `const RECIPE_API = 1;` and their own literal `const NEEDS`.
NEEDS owns required/optional typed inputs, defines, exact typed products,
endpoint grants and upper limits; the coordinator reads it without evaluating
the recipe. Recipe reads the other declarations (`SOURCES`, `FILES`, `PACKAGES`,
...) the same way, downloads and unpacks the sources itself, and calls the
recipe's optional functions (`build(src, out)`, `configure(tree)`, `verify(out)`,
`latest()`, `cleanup(reason)`) with the engine's constants visible. A thrown
error fails the run. Recipe decides a program's state from its own records; no
recipe code runs to decide it. Dependencies run in separate instances with
separate grants. Callers bind `DEP::NAME` inputs explicitly; parent inputs and
grants are never lent. Declared dependency outputs reach the consumer through
`DEPS`, and directory outputs named `tools` contribute to the consumer's `PATH`.
An interrupted step keeps nothing and starts again; there are no checkpoints.

`--input NAME=PATH` binds a declared `file`, `directory`, `file?` or
`directory?` input. Required missing, unknown, duplicate or wrong-kind inputs fail
preflight across the complete declared graph. Source imports omit root
`.git`/`target`, preserve contained relative links and reject escapes/cycles.
Named filesystem inputs `rootfs` and `target` preserve filesystem link bytes,
including absolute links meaningful in a future root. Inputs travel into the
instance as one tar of directories, regular files and symlinks, so **hard links
are not preserved** (a file linked twice arrives as two files) and live system
roots (`--system`, a root's effects recipe) are **copied, not bound**. A
coordinator-authored permission manifest lets copying helpers preserve original
logical modes despite read-only imports. A `target` directory seeds `/out` only
for fresh state; an acknowledged matching filesystem takes precedence on reuse.
The offline cache reaches a run as copies of the payloads whose pins the recipe
declares.

NEEDS.outputs declares exact names and file/directory kinds under `/out`. The
worker checks returned names/kinds; the coordinator independently prepares and
validates the extracted archive privately before committing final state and
publishing fresh paths. Failed work cannot replace previous publications. The
final result's `outputs` contains those host paths; the worker never selects
host destinations. The coordinator adds completion metadata (contract and network
usage) to the result. Each invocation saves its actual declaration, config
(and the instance name), pins and contract in `authority.json`.

## Options, bounds and network policy

Resolved execution values cross the request explicitly: native target, `jobs`
1..16, phase force flags, timeout, memory and storage bounds, network endpoints
and machine events (a `network` grant record with the pinned endpoints and the
byte cap goes to the worker for Recipe's own downloads). NEEDS supplies
authority; caller restrictions may only lower limits/select endpoint subsets.
Main-recipe force flags are cleared for dependencies. Missing NEEDS limits
default to 3600 seconds, 2048MiB, 16,384 storage objects and 8GiB network bytes;
a missing network grants none. Explicit caller restrictions cap every
dependency's own limits and intersect its endpoints. They never lend grants.

What is enforced, and what is not (the accepted regressions are stated plainly
in `tools/recipe/docs/runtime-limits.md`):

- `memory_mib` is the per-process virtual address-space bound; jobs select real
  CPUs by affinity; the deadline is enforced by Recipe signalling the run's
  process group. None is aggregate RSS or CPU time.
- **No storage limit inside a run.** The instance root is a directory tree on
  the host file system holding the state root. `storage_max_bytes` and
  `storage_max_objects` bound only the archive taken out of the instance, an
  output over them fails the run after the fact, and a free-space watchdog
  cancels a run whose host disk drops below `max(2 GiB, 5 %)`.
- **Network grants are per port.** Outbound networking requires NEEDS.network
  endpoints; repeated `--network-host HOST:PORT` (or library
  `ExecutionOptions.narrowing`) can only select a subset; an empty policy
  produces `network.backend="none"`. The coordinator resolves at most eight
  numeric TCP endpoints before launch, with bounded resolver time and output,
  and injects the hostname map; Ginux is given `tcp:PORT` for each distinct
  port on the shared host network. A program a recipe starts can connect to any
  address on a granted port, with no connection or byte cap, and sees the host's
  network. Recipe's own HTTP client in the worker is held to the declared
  `HOST:PORT` at the pinned addresses and counts every body against
  `network_max_bytes`; `network_usage` is what the worker counted. No proxy
  inheritance, arbitrary DNS, UDP or listening access is provided. Redirect
  destinations need their own grant. **A port grant cannot isolate addresses,
  virtual hosts or URL paths.**

HTTP helpers validate normal TLS trust and hostnames using the prepared image's
CA bundle; test-only CA images are explicitly separate inputs. Canonical
acquisition is `download(url, exact_destination, sha256[, resume])` and
`git_checkout(url, exact_destination, full_commit)`. Transfers stage privately,
validate complete bytes/revisions, then replace cache entries. A grant is
permission to contact an endpoint, not permission to skip verification.

Recipe checks config≤4096 bytes, name≤64 bytes (the instance name is
`recipe-<uid>-<pid>-<8 hex>`), args≤7×128, env≤8×256 and assignment bounds
before the instance is created. The environment contains the fixed domain PATH,
worker/invocation identity and TMPDIR; only explicit allowlisted toolchain
variables may be added. Host HOME, checkout paths, credentials and ambient proxy
variables do not cross this boundary.

## Worker records

There is no frame protocol. The request is the staged input
`/inputs/.recipe-request.json`; the result is the file
`/work/.recipe-result.json` (written atomically, naming its version and the
invocation id, read from the extracted archive as untrusted input); diagnostics
and progress are JSON lines on the worker's stdout, which in a Ginux run is the
instance's console (`console.log`, read while the run lives) and on the host is
a pipe. A line that is not a record stays plain log text; lines starting
`GINUX_` are runtime text kept in the supervisor log, not relayed. The
worker's stderr and the runtime's audit lines land in the instance's
`audit.log`, retained as `supervisor-stderr.log`. Diagnostics carry bytes
without result authority; a forged line can become plain text, never success.
Diagnostics are bounded (`MAX_DIAGNOSTIC_BYTES`, `MAX_DIAGNOSTIC_RECORDS`),
then one truncation marker; output continues draining. CLI human output uses stderr; structured context
JSON stays on stdout. `RECIPE_OUTPUT_TIMEOUT_SECS` (default 300, range
1..86400) bounds the wait before the first line and for a line begun but not
ended; a silent legitimate phase afterwards runs until the execution deadline.

## Recovery and cancellation

Identity includes the request/result records version, exact image reference (the
OCI manifest digest), worker and Ginux behaviour versions (Ginux behaviour 2 is
the public lifecycle), input content and modes, network resolution/policy,
defines and target/job/storage choices. Changing these selects fresh state; the
cutover itself changes every saved identity once. Destination locks serialize
publication and state joins. Diagnostic history retains 32 inactive invocations
and protects live leases; each log is bounded.

CLI SIGINT/SIGTERM signals the process group of `ginux start --wait` (sudo
relays it to the helper, whose teardown stops the instance within about 7.5 s)
and awaits it, then deletes the instance; SIGINT exits 130. If the coordinator
dies, the CLI dies with it, the helper notices within a fraction of a second and
tears the instance down, and the next Ginux-using command deletes the stopped
instance. Nothing that is unfinished is published. There is no provider, agent
or autofix anywhere in Recipe.

See the [task queue evidence](recipe-simplification-implementation-plan.md#current-checkpoint)
for historical workloads, revisions and limits on kernel, full-ISO and VM
validation: they were measured on the retired runtime and certify nothing about
this one.
