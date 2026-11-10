# recab desired tree

> Superseded scope, 2026-11-10: this is a historical refactoring snapshot, not the current recab product plan. Recab owns system updates and symmetric A/B continuation, including equivalent initial systems and incremental preparation that preserves pending changes through updates. Follow [the current requirements](../../tools/recab/REQUIREMENTS.md); retain the old findings below as historical evidence only.

The complete target tree of `tools/recab`, every leaf with its owner slice and
where its content comes from. Language: Rust stays (see
[ideal-filetree-plan.md](ideal-filetree-plan.md#language-and-owner)). The tree
stays flat under `src/`: the splits below separate duties that today share a
file, without adding packages or frameworks.

## Tree

```text
tools/recab/
├── AGENTS.md                      retained                         S12
├── CLAUDE.md                      retained, corrected (F-D3)       S12
├── Cargo.toml                     retained                         S10
├── LICENSE-APACHE                 retained
├── LICENSE-MIT                    retained
├── README.md                      retained, corrected (F-D1,F-D2)  S12
├── REQUIREMENTS.md                retained, corrected (F-D1,F-D2)  S12
├── docs/
│   ├── error.schema.json          generated, retained              S11
│   ├── health.schema.json         generated, retained              S11
│   └── status.schema.json         generated, retained              S11
├── src/
│   ├── main.rs                    retained                         S10
│   ├── lib.rs                     reduced: wiring, re-exports, thin entry points   S10
│   ├── config.rs                  retained, + running-root/proc paths (F-C1)       S10
│   ├── error.rs                   retained                         S10
│   ├── system.rs                  NEW from lib.rs: System, change(), require_root, now, stamped   S10
│   ├── status.rs                  NEW from lib.rs + systemd_boot.rs: Status, SlotView, status(), slot_views   S11
│   ├── contract.rs                retained                         S11
│   ├── state.rs                   retained, + Slot and TrialRecord from lib.rs     S1
│   ├── store.rs                   retained minus the target_root wrappers (F-A2)   S9
│   ├── systemd_boot.rs            reduced: backend read/apply/reset/ensure, bootability, slot kernel dirs   S2/S3
│   ├── entry.rs                   NEW from systemd_boot.rs: names, constants, entry and loader.conf text (pure)   S3
│   ├── efivars.rs                 NEW from systemd_boot.rs: EFI variable I/O, clear_immutable, ensure_can_change   S2
│   ├── root.rs                    NEW from systemd_boot.rs: RootSelector, running root, active slot, slot device   S4
│   ├── init.rs                    retained, + install_loader       S5
│   ├── stage.rs                   retained minus carry, + stage() orchestration from lib.rs   S6
│   ├── carry.rs                   NEW from stage.rs + carried.rs: carried list, baseline merge, copy, accounts   S7
│   ├── trial.rs                   NEW from lib.rs: trial(), TrialOutcome            S8
│   ├── init_system.rs             retained                         S8
│   ├── package.rs                 retained                         S8
│   └── services/
│       ├── carried                retained (content per F-K findings)   S7
│       ├── recab-trial.openrc     retained (per F-T findings)      S8
│       └── recab-trial.service    retained (per F-T findings)      S8
└── tests/
    ├── common/mod.rs              NEW from lifecycle.rs: the scratch `Tree` harness   S13
    ├── slots.rs                   NEW from lifecycle.rs: status, set-next/commit/rollback, state file   S13 (S1)
    ├── boot_selection.rs          NEW from lifecycle.rs: firmware variables, defaults, bootability   S13 (S2/S3)
    ├── trial.rs                   NEW from lifecycle.rs: trial verdicts on both inits   S13 (S8)
    ├── stage.rs                   NEW from lifecycle.rs: stage refusals   S13 (S6)
    └── install.rs                 NEW from lifecycle.rs: init, install-unit   S13 (S5/S8)
```

The test split exists so the parallel lanes in [tasks.md](tasks.md) each own
the tests of their slice; the harness moves verbatim.

Retired: `src/carried.rs` (absorbed by `carry.rs`) and `tests/lifecycle.rs`
(split as above). Nothing else is retired:
no file in the tree is dead. The public modules `carried` and the
unused re-exports listed in [inventory.md](inventory.md#external-joins) stop
being public (F-A1); no external crate imports them.

## Allocation of every current duty

| Current | Lines | Target |
|---|---|---|
| `lib.rs` | 1–53 | `lib.rs` |
| `lib.rs` | 55–96 `Slot` | `state.rs` (re-exported as `recab::Slot`) |
| `lib.rs` | 98–108 `TrialRecord` | `state.rs` (persisted in `PersistentState`; re-exported) |
| `lib.rs` | 109–152 `SlotView`, `Status` | `status.rs` (re-exported) |
| `lib.rs` | 154–171 `StageSource`, `StageRequest` | `stage.rs` (re-exported) |
| `lib.rs` | 173–193 `InitLayout`, `InitRequest` | `init.rs` (re-exported) |
| `lib.rs` | 195–202 `TrialOutcome` | `trial.rs` (re-exported) |
| `lib.rs` | 204–248 `System` | `system.rs` |
| `lib.rs` | 250–271 `status()` | `status.rs` |
| `lib.rs` | 273–280 `init()` | `lib.rs` (thin) → `init::run` |
| `lib.rs` | 282–319 `stage()` | `stage.rs` (`stage::run`); `lib.rs` thin |
| `lib.rs` | 321–334 | `lib.rs` (thin) |
| `lib.rs` | 336–429 `trial()` | `trial.rs` |
| `lib.rs` | 431–448 `health()`, `install_unit()` | `lib.rs` (thin) |
| `lib.rs` | 450–530 helpers, `change()` | `system.rs` |
| `state.rs` | all | `state.rs` |
| `store.rs` | all | `store.rs` |
| `systemd_boot.rs` | 24–39 layout constants | `entry.rs` |
| `systemd_boot.rs` | 40–61 panic/root/trial options | `entry.rs` |
| `systemd_boot.rs` | 63–66 EFI attrs/flags, 24–25 vendor GUID | `efivars.rs` |
| `systemd_boot.rs` | 68–283 backend, read, firmware_trial, reset, apply | `systemd_boot.rs` (`ensure_can_change` → `efivars.rs`) |
| `systemd_boot.rs` | 285–328 bootability, trial options on entries | `systemd_boot.rs` |
| `systemd_boot.rs` | 330–361 `slot_device` | `root.rs` |
| `systemd_boot.rs` | 363–440 slot kernel dirs, entry access | `systemd_boot.rs` |
| `systemd_boot.rs` | 442–507 EFI variable I/O | `efivars.rs` |
| `systemd_boot.rs` | 509–596 running root, active slot | `root.rs` |
| `systemd_boot.rs` | 598–617 `entry_prefix` | `systemd_boot.rs` |
| `systemd_boot.rs` | 619–642 `slot_views` | `status.rs` |
| `systemd_boot.rs` | 645–690 naming, `new_entry` | `entry.rs` |
| `systemd_boot.rs` | 692–720 `install_loader` | `init.rs` |
| `systemd_boot.rs` | 722–839 text parse/edit | `entry.rs` |
| `systemd_boot.rs` | 841–931 `RootSelector` | `root.rs` |
| `systemd_boot.rs` | 933–955 `clear_immutable` | `efivars.rs` |
| `stage.rs` | 27–293, 501–511, 540–621 | `stage.rs` |
| `stage.rs` | 295–499, 513–538 | `carry.rs` |
| `carried.rs` | all | `carry.rs` |
| `init_system.rs` | all | `init_system.rs` |
| `package.rs`, `services/*` | all | same paths |
| `config.rs`, `error.rs`, `contract.rs`, `main.rs` | all | same paths |
| `store.rs` | 148–163 durable-file wrappers | callers use `target_root` directly |
| `tests/lifecycle.rs` | 1–230 harness | `tests/common/mod.rs` |
| `tests/lifecycle.rs` | `status_reads…`, `mutations_refuse…`, `trial_commit_and_rollback…`, `a_failed_firmware_write…`, `state_keeps_fields…` | `tests/slots.rs` |
| `tests/lifecycle.rs` | `set_next_refuses_a_slot_without_its_kernel`, `a_default_naming_an_unbootable_slot_is_ignored`, `set_next_resets_a_stale_firmware_default` | `tests/boot_selection.rs` |
| `tests/lifecycle.rs` | `trial_commits…`, `trial_reboots…`, `trial_that_never_finishes…`, `trial_judges_only…`, `trial_without_the_esp…`, `openrc_trials…` | `tests/trial.rs` |
| `tests/lifecycle.rs` | `stage_refuses_while…`, `stage_refuses_bad_inputs…`, `stage_resets_a_stale…` | `tests/stage.rs` |
| `tests/lifecycle.rs` | `install_unit_writes…` (2), `init_sets_up…` | `tests/install.rs` |

## Joins to keep closed

These must hold after every task; each is checked by compilation or the
named command.

| Join | Kept by |
|---|---|
| `recab::{Slot, Status, ENTRIES_DIR, entry_name, layout_name, state_file, trial_service_link}` used by `testing/install-tests` | crate-root re-exports in `lib.rs`; `cargo check -p install-tests` |
| `recab::contract::export` used by `xtask gen` | `contract.rs` path and the `contracts` feature; `cargo xtask gen --check` |
| JSON field names of `Status`, `SlotView`, `TrialRecord`, `Health`, `ErrorPayload` | moving types does not rename fields; `cargo xtask gen --check` stays "current" |
| `recab init`, `install-unit`, `status --json`, `set-next`, `commit`, `rollback`, `stage`, `trial`, `health` argv | `main.rs` unchanged by the splits; `cargo test -p recab` |
| `include_str!("services/…")` paths in `package.rs` | `services/` stays beside `package.rs` |
| State file format (`PersistentState`, schema 1) | `state.rs` unchanged by the move; `state_keeps_fields_it_does_not_know` |
