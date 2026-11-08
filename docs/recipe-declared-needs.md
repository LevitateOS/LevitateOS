# Recipe: declared needs and an explainable contract

Implemented and locally verified release design, 2026-11-08. The user explicitly
included the full proposal in [PRD18](ginux-contribution-task-list.md).
[The implementation record](ginux-contributions/prd18-implementation.md) owns
exact sources, checks, finding dispositions and separate release dependencies. API 2 is the sole recipe interface. Local configured
contract work is authorized separately from held upstream/native policy and any
source-delivery/publication choice. Ginux upstream takes precedence; no upstream
publication is authorized here.

This carries over principles from Ginux PR #237: self-contained authority,
finite compiled rules, deny precedence and explanations. The historical baseline
below explains the move; current code uses a versioned downstream catalog-2
contract and the [Ginux/Recipe boundary](ginux-recipe-interfaces.md).

## Historical problem (before NEEDS)

Authority was selected outside the recipe:

| Authority | Historical owner |
| --- | --- |
| Network hosts | `--network-host`, from `network_hosts`/`preseed_network_hosts` in a variant's `sources.toml`, or hard-coded by a Rust caller |
| Memory, storage, time | `--memory-mib` etc., from `memory_mib`/`preseed_memory_mib` or caller defaults |
| Inputs and their kinds | `--input NAME=PATH` from `inputs` in `sources.toml`, or built by callers (`pipeline/kernel.rs`, `recipe/tools.rs`, `recipe/qemu.rs`) |
| Outputs and their kinds | `preseed_outputs` in `sources.toml`; the recipe only sets `ctx.outputs` at runtime |
| The Ginux contract | One fixed profile for every recipe: `[contract] profile = "gate5-balanced-v1"` (`tools/recipe/src/inputs/config.rs`) |

The recipe itself declares `RECIPE_API`, `deps` and `build_deps`. Everything
else it assumes: that `/inputs/iso` exists, that it can reach a mirror, that
4 GiB is enough.

**Evidence.** The AcornOS rootfs recipe wrote dl-cdn into its repositories and
needed seven packages that exist only there, while `sources.toml` gave that
step `network_hosts = []`. It could never have finished, and nothing could say
so before running it: the two halves of its contract live in different files
(fixed in builder `206e875`). When a recipe fails under Ginux today, nothing
connects the denial to the declaration that caused it, and the caller cannot
tell a Recipe mistake from a Ginux gap.

## What Ginux #237 does, and what carries over

\#237 moves an instance's contract into the instance file, writes it as human
access groups (`read`, `write`, `execute`, `connect`, `listen`, `process`,
`system`, `deny`) compiled to the exact operation table, makes `deny` win and
zero-match selectors an error, records which rule decided every operation, and
adds `validate`, `inspect`, `explain` and `diff`.

| #237 principle | Recipe version |
| --- | --- |
| The contract lives in the file it governs | A recipe declares its needs in the recipe (`const NEEDS`) |
| Human terms compiled to an exact table | Needs in Recipe's terms (inputs, outputs, hosts, limits) compiled to the Ginux instance config, including its contract |
| Default deny; nothing silently widens | Callers supply what is declared and may narrow it, never widen it |
| Fail before launch; no rule may match nothing | Undeclared/missing inputs and malformed/zero-match derived grants fail before launch; unused effective endpoints warn after observation |
| Every decision names its rule | Every grant names its `NEEDS` field; a Ginux denial is traced back to it |
| `validate`, `inspect`, `explain`, `diff` | The same four commands for recipes |

Not carried over: the selector syntax. Ginux's grants are over syscall-level
operations; a recipe's concerns are files, hosts and phases. Recipe keeps its
vocabulary and compiles down to Ginux's.

## 1. `const NEEDS`

A literal map, read from the AST like `deps` (`declared_metadata` in
`src/core/executor.rs`), so it is known without running setup:

```rhai
const RECIPE_API = 2;
const NEEDS = #{
    inputs: #{ apks: "directory", capabilities: "file",
               system_setup: "file", os_release: "file" },
    outputs: #{ rootfs: "directory" },
    memory_mib: 4096,
    storage_max_objects: 131072,
};
```

The preseed that feeds it:

```rhai
const NEEDS = #{
    inputs: #{ target: "directory?" },
    outputs: #{ apks: "directory" },
    network: ["dl-cdn.alpinelinux.org:443"],
    memory_mib: 4096,
};
```

Fields (all optional; an absent field means none, or Recipe's default limit):

- `inputs`: name → `"file"` | `"directory"` | `"file?"` | `"directory?"`.
  Required bindings must exist; optional bindings may be omitted but retain
  their type when supplied. Explicit assets appear under `/inputs/NAME`; code,
  worker metadata and declared dependency products have separate reserved
  coordinator-owned input names.
- `outputs`: name → `"file"` | `"directory"`. `ctx.outputs` must publish
  exactly these.
- `network`: `HOST:PORT` endpoints, the same form `--network-host` takes.
- `memory_mib`, `storage_max_objects`, `storage_max_bytes`, `timeout_secs`,
  `network_max_bytes` (default 8 GiB):
  upper limits, defaulting to Recipe's current defaults.

`NEEDS` joins `RESERVED_NAMES` in `src/recipes.rs`. It must be a literal. A computed value is an error, not a dynamic field:
authority must be readable without running the recipe. Dependencies keep their
own `NEEDS`; a parent never lends its grants to a dependency.

### Rules

- The caller supplies every declared input, and only declared inputs.
  `--input` naming anything else fails before launch.
- `--network-host` and the limit flags may only narrow: a subset of `network`,
  lower limits. Asking for more fails with the field to edit. Widening
  authority means editing the recipe, so it shows up in review. Explicit caller
  restrictions must be valid lowerings/subsets for the named recipe, then cover
  the full dependency graph: each dependency keeps the lower
  of its own limit and the caller ceiling, and intersects its own endpoints with
  the caller subset. A restriction never lends parent grants.
- Completed installation and installed-cache reuse require exactly the declared
  output names and kinds. Intermediate phases may expose a subset; every present
  output is checked before checkpoint/publication.
- A declared network endpoint the invocation never connects to is a warning
  in `inspect` and the result, not an error: mirrors are sometimes unused on a
  warm cache.

### What moves out of callers

- Variant `distro.toml` (`[sources.rootfs_source]`) loses `memory_mib`, `preseed_memory_mib`, `network_hosts`,
  `preseed_network_hosts` and `preseed_outputs`; it keeps which recipe to run
  and which repo paths to bind to its declared inputs.
- The builder reads output names and kinds from the recipe's `NEEDS` instead of
  `preseed_outputs`, and passes no grants.
- `pipeline/kernel.rs`, `recipe/tools.rs`, `recipe/qemu.rs` and xtask's kernel
  tasks stop choosing network hosts and limits.

Recipe has no external users, so this is a direct change to every recipe and
caller, with no compatibility path.

## 2. A contract derived per recipe

The user authorized a versioned downstream configured contract while upstream
decisions remain separate. Recipe renders `[userspace.contract]` from NEEDS:

- a fixed base for build work: read, write and execute within the paths the
  instance assigns (`/inputs`, `/work`, `/out`), processes and signals;
- `connect` only when `network` is non-empty (the endpoints themselves stay in
  `[network]`, where Ginux enforces them as typed constraints);
- no `listen`, ever;
- a fixed `deny` list: direct MMIO, DMA, raw packets.

The catalog-2 selectors and the two expected operation-table digests are pinned
in `tools/recipe/src/needs/contract.rs`; the checkpoint records real Ginux
validation. Owned local-packet send/receive call numbers are admitted separately
from TCP connect, which still requires its semantic grant and pinned endpoint. Recipe pins the catalog version, so a Ginux catalog bump cannot quietly
grant a recipe more. Before launch, Recipe runs `ginux instance validate` on
the rendered config and records the contract digest in the completed context as `runtime.contract` and
in the retained invocation receipt.

Most build recipes will get nearly the same contract. What changes is that
each grant is derived from a declaration that can be pointed to.

## 3. Four commands

All of them read declarations only; none launches anything.

**`recipe validate RECIPE [--input ...]`** checks `RECIPE_API`, that `NEEDS` is
literal and well-formed, that supplied inputs match it (names, kinds, existence),
that the rendered instance config compiles, and that dependency recipes
validate too.

**`recipe inspect RECIPE`** prints resolved authority as JSON, schematically:

```text
alpine-live-source-rootfs  (RECIPE_API 2)
  inputs   apks (directory), capabilities/system_setup/os_release (file)
  outputs  rootfs (directory)
  network  none
  limits   memory 4096 MiB, storage 131072 objects, timeout 3600 s
  contract userspace catalog 2, digest 121cb4…  (connect: not granted)
```

**`recipe explain INVOCATION|RECIPE OPERATION`** traces a decision across both
layers. With #237, Ginux's audit records carry `operation`, `policy-source` and
`policy-rule` (Recipe selects stderr audit, kept in the invocation's
`supervisor-stderr.log`); Recipe maps the rule back to the `NEEDS` field that
produced it:

```text
denied  network.connect dl-cdn.alpinelinux.org:443
  Ginux    no connect grant (default deny)
  Recipe   NEEDS.network is empty in alpine-live-source-rootfs.rhai
  phase    uncorrelated when the retained streams do not establish it
  fix      declare the endpoint in NEEDS.network, or provide the file as an input
```

A denial with no corresponding `NEEDS` field to edit is a Ginux question, and
`explain` says so. This is the "Recipe or Ginux?" split, answered by the tool.

**`recipe diff OLD NEW`** compares the authority of two versions of a recipe
(or two recipes): added and removed inputs, outputs, endpoints, raised limits,
and the contract digest. It is meant for review: "this change now reaches a
new host" is visible without reading the hooks.

## Implementation and release dependencies

Implementation is complete locally, with exact-source results in the linked checkpoint.
Dependencies use separate Ginux instances and bind their own inputs as
`DEP::NAME`; declared products are snapshotted into the consumer. `tools`
directory outputs contribute to its private tools prefix. Parent grants are
never lent. Operation-table digests and diagnostic capabilities are checked
before launch. Worker transport 9 rejects older coordinator/worker combinations.

The user resolved the remaining semantics on 2026-11-08: complete outputs at
install/installed reuse with typed intermediate subsets, and graph-wide caller
ceilings/intersections. Manual checks, remove and cleanup act on the named
recipe's own retained files without installing or checking installation readiness
of dependencies. Declaration/source/input preflight still covers the full graph.
Code, direct callers, tests and documentation reflect these decisions. No PRD18
semantic decision remains open; the linked record retains original failed runs
and the concrete passing replacements, rather than claiming a green full suite.

PRD01 maintained-source delivery and PRD05 required integration joins remain
separate release dependencies. Local implementations and fixture checks do not
establish fresh-checkout delivery, final-source security acceptance or either
distro's production/boot/install/A/B claims.
