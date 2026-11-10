# recab ideal filetree plan

> Superseded scope, 2026-11-10: this is a historical refactoring snapshot, not the current recab product plan. Recab owns system updates and symmetric A/B continuation, including equivalent initial systems and incremental preparation that preserves pending changes through updates. Follow [the current requirements](../../tools/recab/REQUIREMENTS.md); retain the old findings below as historical evidence only.

Living plan for `tools/recab`: its slices, findings, target tree and the tasks
that get there. It is a planning document someone asked for, not a gate:
nothing here is required before a change to recab (superproject `AGENTS.md`).

## Baseline

| Input | Value |
|---|---|
| Checkout | `/home/vince/Projects/LevitateOS/tools/recab`, branch `main`, `d4365a9e` ("G30: stage refuses an image of the other flavour"), clean |
| Superproject | `master` `eea341c6`, pins `tools/recab` at `d4365a9e`; its own dirty files are outside recab |
| Ignored | `/home/vince/Projects/LevitateOS-head` (an older checkout, recab `2b23b273`) |
| Governance | superproject `AGENTS.md` (no unit tests, no branches, no mandatory process; Rust/TOML owns orchestration); recab `AGENTS.md` (points there); recab `CLAUDE.md` |
| Contracts | `README.md`, `REQUIREMENTS.md`, `docs/*.schema.json` |
| Earlier plan touching recab | `UNTANGLE.md` G04/G17/G18/G19/G20/G30 (all landed); decision D12 (see D-1) |
| Audit date | 2026-11-08 |

## Files

| File | Holds |
|---|---|
| [inventory.md](inventory.md) | every tracked path, each mixed file's duties by line range, external joins |
| [slices.md](slices.md) | the 13 slices, their authority/state/lifecycle, completion per slice |
| [findings.md](findings.md) | the shared finding queue (primary and challengers) |
| [desired-tree.md](desired-tree.md) | the target tree, the allocation of every current duty, the joins to keep |
| [tasks.md](tasks.md) | task packets, dependencies, lanes, verification |

## Language and owner

Technical fit: Rust stays. recab links the workspace's Rust crates
(`distro-profile`, `target-root`, `recpart`, `recinit` constants), ships as one
static binary onto busybox/OpenRC and systemd systems with no runtime, and
`testing/install-tests` and `xtask` import its Rust types. The service files
stay init-native text (a systemd unit, an `openrc-run` script) and the carried
list stays a plain data file read by the image's own recab.

Policy, recorded apart: the superproject puts orchestration and contracts in
Rust/TOML, and G18/D12 made the install tools static musl. This plan did not
compare other languages without that policy; it is not proof that Rust is best
in the abstract.

Package: one crate. The splits in [desired-tree.md](desired-tree.md) are
modules inside it.

## Decisions pending

| ID | Decision | Blocks | Default this plan assumes |
|---|---|---|---|
| D-1 | G18's "merge recpart and recab into one install-tools repo" (D12) never happened; keep separate repos sharing `target-root`, or merge? | nothing in recab's tree (it is stated from the crate root); only where the crate lives | keep separate repos |
| D-2 | Keep the library entry points (`status()`…`install_unit()`) that REQ-LIB-001/002 require but no other crate calls? | T-A1's scope | keep them public |
| D-3 | Should `status --json` carry recab's own verdict per action, so recab-tui stops re-deriving `state::plan` (F-X1)? Crosses into `tui`. | T-X1 | yes |
| D-4 | What machine state an update promises to keep: `/root` (carry it, or link it to `/var` like homes), and the uncarried paths in F-K5; whether a deleted `file` entry stays deleted. | T-K3 | carry `/root` via a `/var` link; add the systemd/OpenRC config directories F-K5 lists |
| D-5 | When `recab init` runs on a machine that will not boot the target (installing from an installed system), may it rewrite this machine's firmware variables (F-I1)? | host-firmware part of T-I1 | no: skip the reset when the running system is itself recab-managed, and say so |
| D-6 | Where an installed slot's baseline comes from (F-K1): the image build writes `usr/lib/recab/baseline`, or install (recsetup) writes it after extraction. | T-K1, T-V1 | the image build, so every slot carries its own image's baseline |

## Completion

Counted separately for each slice in [slices.md](slices.md#completion-by-slice).

- **Source coverage:** every line of every tracked file was read by the
  primary; `target-root`'s durable-write helpers were not re-read.
- **Workflow tracing:** every command traced through callers, state and failure
  branches by reading. Boot ordering of the trial service is inferred.
- **Independent challenge:** all 13 slices, by three challengers given the
  code and contracts but not the primary's findings. 36 of the 37 findings
  were raised or confirmed by a challenger; the adopted ones were re-read by
  the primary.
- **Open findings:** 37: 1 high (F-K1), 15 medium, 20 low, 1 decision-only
  (F-G1) ([findings.md](findings.md)).
- **Target allocation:** every current duty has a target file
  ([desired-tree.md](desired-tree.md#allocation-of-every-current-duty)).

## Checks run

| Check | Result | Covers |
|---|---|---|
| `cargo test -p recab` | 20/20 lifecycle tests pass | scratch-tree CLI behaviour; no slot is written |
| `cargo xtask gen --check` | "generated files are current" | committed schemas and generated TS match the Rust types |
| local link and anchor check of this folder | 0 broken | Markdown links only (the skill's `check_artifacts.py` is not installed; an equivalent inline check was used) |

Not run: any VM scenario (`test ab-update`, `test all`), builds of ISOs or
kernels, a real `stage`. README records a seven-boot `ab-update` pass on both
distros on 2026-11-08; that is the repository's record, not evidence produced
by this audit.

## Refresh

After a recab change lands: compare `git -C tools/recab log <baseline>..`,
update the affected inventory rows, slice completion, findings (close what the
change fixed, with its commit), desired-tree allocation and tasks, then move
the baseline above. Replace stale content; do not append a diary.
