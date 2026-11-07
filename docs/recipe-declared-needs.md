# Recipe: declared needs and an explainable contract

Draft design note, 2026-11-07. A proposal, not part of the
[Recipe task queue](recipe-simplification-implementation-plan.md); nothing here
is authorized work until it is put on that queue. It borrows the ideas, not the
syntax, of Ginux [PR #237](https://github.com/gmkurtzer/ginux/pull/237),
"Gate 10.5: intuitive self-contained contract configuration". Interfaces it
builds on are in [ginux-recipe-interfaces.md](ginux-recipe-interfaces.md).

## The problem

What a recipe may do is decided everywhere except the recipe:

| Authority | Where it is set today |
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
| Fail before launch; no rule may match nothing | Undeclared inputs, missing inputs, unused grants are errors before launch |
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
    inputs: #{ iso: "file", apk_tools: "file", extra_apks: "directory" },
    outputs: #{ rootfs: "directory" },
    network: [],
    memory_mib: 4096,
};
```

The preseed that feeds it:

```rhai
const NEEDS = #{
    outputs: #{ iso: "file", apk_tools: "file", extra_apks: "directory" },
    network: ["dl-cdn.alpinelinux.org:443"],
    memory_mib: 4096,
};
```

Fields (all optional; an absent field means none, or Recipe's default limit):

- `inputs`: name → `"file"` | `"directory"`. Exactly these appear under
  `/inputs/NAME`.
- `outputs`: name → `"file"` | `"directory"`. `ctx.outputs` must publish
  exactly these.
- `network`: `HOST:PORT` endpoints, the same form `--network-host` takes.
- `memory_mib`, `storage_max_objects`, `storage_max_bytes`, `timeout_secs`:
  upper limits, defaulting to Recipe's current defaults.

`NEEDS` joins `RESERVED_NAMES` in `src/recipes.rs`. It must be a literal. A computed value is an error, not a dynamic field:
authority must be readable without running the recipe. Dependencies keep their
own `NEEDS`; a parent never lends its grants to a dependency.

### Rules

- The caller supplies every declared input, and only declared inputs.
  `--input` naming anything else fails before launch.
- `--network-host` and the limit flags may only narrow: a subset of `network`,
  lower limits. Asking for more fails with the field to edit. Widening
  authority means editing the recipe, so it shows up in review.
- After a successful phase, `ctx.outputs` must have exactly the declared names
  with the declared kinds.
- A declared network endpoint the invocation never connects to is a warning
  in `inspect` and the result, not an error: mirrors are sometimes unused on a
  warm cache.

### What moves out of callers

- `sources.toml` loses `memory_mib`, `preseed_memory_mib`, `network_hosts`,
  `preseed_network_hosts` and `preseed_outputs`; it keeps which recipe to run
  and which repo paths to bind to its declared inputs.
- The builder reads output names and kinds from the recipe's `NEEDS` instead of
  `preseed_outputs`, and passes no grants.
- `pipeline/kernel.rs`, `recipe/tools.rs`, `recipe/qemu.rs` and xtask's kernel
  tasks stop choosing network hosts and limits.

Recipe has no external users, so this is a direct change to every recipe and
caller, with no compatibility path.

## 2. A contract derived per recipe

Once #237 merges, the profile reference Recipe renders is legacy. Recipe then
renders `[userspace.contract]` from `NEEDS` instead:

- a fixed base for build work: read, write and execute within the paths the
  instance assigns (`/inputs`, `/work`, `/out`), processes and signals;
- `connect` only when `network` is non-empty (the endpoints themselves stay in
  `[network]`, where Ginux enforces them as typed constraints);
- no `listen`, ever;
- a fixed `deny` list: direct MMIO, DMA, raw packets.

The exact selectors must come from the pinned catalog and are an open item
below. Recipe pins the catalog version, so a Ginux catalog bump cannot quietly
grant a recipe more. Before launch, Recipe runs `ginux instance validate` on
the rendered config and records the contract digest in the invocation result.

Most build recipes will get nearly the same contract. What changes is that
each grant is derived from a declaration that can be pointed to.

## 3. Four commands

All of them read declarations only; none launches anything.

**`recipe validate RECIPE [--input ...]`** checks `RECIPE_API`, that `NEEDS` is
literal and well-formed, that supplied inputs match it (names, kinds, existence),
that the rendered instance config compiles, and that dependency recipes
validate too.

**`recipe inspect RECIPE`** prints the resolved authority:

```text
alpine-live-source-rootfs  (RECIPE_API 2)
  inputs   iso (file), apk_tools (file), extra_apks (directory)
  outputs  rootfs (directory)
  network  none
  limits   memory 4096 MiB, storage 16384 objects, timeout 3600 s
  contract userspace catalog 1, digest 3f2a…  (connect: not granted)
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
  phase    build, while running apk.static
  fix      declare the endpoint in NEEDS.network, or provide the file as an input
```

A denial with no corresponding `NEEDS` field to edit is a Ginux question, and
`explain` says so. This is the "Recipe or Ginux?" split, answered by the tool.

**`recipe diff OLD NEW`** compares the authority of two versions of a recipe
(or two recipes): added and removed inputs, outputs, endpoints, raised limits,
and the contract digest. It is meant for review: "this change now reaches a
new host" is visible without reading the hooks.

## Order of work

0. **Unblock Ginux execution.** Recipe renders `[instance]` (after Gate 10.25,
   upstream #235); the pinned `tools/ginux` (`2eac10bf`) parses only `[domain]`
   and rejects every launch with `unknown-section`. Move the pin to a Ginux that
   has the rename plus the local integration commits.
1. **`NEEDS` and `validate`/`inspect`.** Recipe and its callers only; no Ginux
   change.
2. **Derived `[userspace.contract]`.** After #237 merges.
3. **`explain`.** Needs #237's audit provenance at runtime.
4. **`diff`.** Any time after 1.

## Open questions

- Which catalog selectors make up the build base, and whether any build tool
  needs `system` grants beyond memory (for example `cgroup` reads by `nproc`).
- Whether `NEEDS.network` should carry a byte budget, or keep the caller's
  `--network-max-bytes`, which can only narrow.
- Whether optional inputs are needed (the kernel recipe's `kconfig_fragments`
  is passed only sometimes). A `"file?"`/`"directory?"` kind would cover it.
- Naming: #237 calls the Ginux instance TOML an "instance recipe". With a
  project named Recipe on top of Ginux, that will confuse readers of both;
  worth raising on the PR.
