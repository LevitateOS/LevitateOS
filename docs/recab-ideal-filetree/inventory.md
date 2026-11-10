# recab inventory

Every tracked path of `tools/recab` at `d4365a9e` (clean), with each mixed
file's distinct duties by line range. Slice IDs are defined in
[slices.md](slices.md). Line counts are `wc -l`.

## Tracked files

| Path | Lines | Kind | Duties → slice |
|---|---:|---|---|
| `Cargo.toml` | 32 | build | package manifest, deps on `distro-profile`, `target-root`, `recpart`, `recinit` → S10; `contracts` feature → S11 |
| `AGENTS.md` | 3 | governance | points at the superproject `AGENTS.md` → S12 |
| `CLAUDE.md` | 12 | governance | purpose and "keep it small" guidance; says the backend is "likely systemd-boot" (it is systemd-boot) → S12 |
| `LICENSE-APACHE`, `LICENSE-MIT` | 202, 18 | legal | retained as is |
| `README.md` | 295 | contract | operator contract: commands, behaviour, trial options, failure table, persistence, preconditions, security, code map, tests, e2e record, exit codes, env overrides, non-goals → S12 (facts owned by S1–S11) |
| `REQUIREMENTS.md` | 380 | contract | normative requirements (REQ-*) → S12 |
| `docs/status.schema.json` | 177 | generated | JSON Schema of `Status` (`cargo xtask gen`) → S11 |
| `docs/health.schema.json` | 42 | generated | JSON Schema of `Health` → S11 |
| `docs/error.schema.json` | 59 | generated | JSON Schema of `ErrorPayload` → S11 |
| `src/main.rs` | 216 | code | clap command model, dispatch, text/JSON rendering of status/health/errors → S10 |
| `src/lib.rs` | 530 | code, mixed | see below |
| `src/contract.rs` | 31 | code | contract export for `cargo xtask gen` → S11 |
| `src/error.rs` | 118 | code | exit codes, `ErrorClass`, classified error, `ErrorPayload` → S10 (payload type also S11) |
| `src/config.rs` | 124 | code | env → `Config`, install-target view, `state_file()` → S10 |
| `src/state.rs` | 333 | code, mixed | see below |
| `src/store.rs` | 163 | code | lock, state load/save, audit log, durable-write wrappers → S9 |
| `src/systemd_boot.rs` | 955 | code, mixed | see below |
| `src/init.rs` | 119 | code | install-time layout → S5 |
| `src/stage.rs` | 621 | code, mixed | see below |
| `src/init_system.rs` | 283 | code, mixed | see below |
| `src/package.rs` | 70 | code | `install-unit` package contents → S8 (carried list copy → S7) |
| `src/carried.rs` | 62 | code | carried-list format and parser → S7 |
| `src/services/recab-trial.service` | 14 | template | systemd trial unit → S8 |
| `src/services/recab-trial.openrc` | 29 | template | OpenRC trial service → S8 |
| `src/services/carried` | 36 | data | the carried machine-state list → S7 |
| `tests/lifecycle.rs` | 866 | test | scratch-tree CLI integration tests (20) → S13 |

Untracked or generated in scope: none in the checkout. `tools/recab/.git` is
the submodule's gitlink file. Build output goes to the superproject `target/`.
`docs/*.schema.json` are generated, but they are committed and read by
`cargo xtask gen` consumers, so they are retained artifacts, not junk.

## Mixed files by duty

### `src/lib.rs` (530)

| Lines | Symbol(s) | Duty | Slice |
|---|---|---|---|
| 1–53 | module list, re-exports | crate wiring and public surface | S10 |
| 55–96 | `Slot` | slot identifier (public, clap, serde) | S1 |
| 98–152 | `TrialRecord`, `SlotView`, `Status` | public status/JSON model | S11 (assembled by S2/S1) |
| 154–202 | `StageSource`, `StageRequest`, `InitLayout`, `InitRequest`, `TrialOutcome` | request/outcome types | S6, S5, S8 |
| 204–248 | `System` | one command's read/locked view | S10 |
| 250–271 | `status()` | status assembly | S10 |
| 273–280 | `init()` | entry point | S5 |
| 282–319 | `stage()` | stage orchestration: plan, checks, firmware reset, state write, slot write | S6 |
| 321–334 | `set_next`, `commit`, `rollback` | entry points | S10 |
| 336–429 | `trial()` | trial gate, wait, verdict, commit or record-and-reboot | S8 |
| 431–448 | `health()`, `install_unit()` | entry points | S8 |
| 450–473 | `require_root`, `now`, `stamped` | helpers | S10 |
| 475–530 | `change()` | apply boot change, then save state, then audit | S10 (transaction across S2/S9) |

### `src/state.rs` (333)

| Lines | Symbol(s) | Duty | Slice |
|---|---|---|---|
| 11–54 | `STATE_SCHEMA_VERSION`, `PersistentState` | persistent schema with forward-compatible unknown fields | S1 (stored by S9) |
| 56–93 | `Selection`, `known_good`, `committed`, `rollback_target` | derived slot facts | S1 |
| 95–159 | `Request`, `Operation`, `BootChange`, `Transition`, `refuse` | transition vocabulary | S1 |
| 161–259 | `plan_stage`, `plan` | transition planning | S1 |
| 261–333 | `Startup`, `TrialVerdict`, `TrialView`, `judge_trial` | trial verdict | S1 (inputs from S8) |

### `src/systemd_boot.rs` (955)

| Lines | Symbol(s) | Duty | Slice |
|---|---|---|---|
| 24–66 | vendor GUID, `LOADER_CONF`, `ENTRIES_DIR`, `KERNEL_FILE`, `INITRD_FILE`, arches, `PANIC_*`, `ROOT_OPTIONS`, `trial_options`, `is_trial_option`, EFI attrs | ESP layout and option vocabulary | S3 |
| 68–118 | `SystemdBoot`, `Boot`, `ApplyError`, `new` | backend handle | S2 |
| 120–171 | `read`, `firmware_trial` | selection read | S2 |
| 173–283 | `reset_stale_selection`, `ensure_can_change`, `apply` | selection write | S2 |
| 285–328 | `ensure_bootable`, `missing_boot_file`, `set_trial_options` | entry bootability and trial options | S3 |
| 330–361 | `slot_device` | slot partition from entry `root=` | S4 |
| 363–417 | `slot_boot_files_dir`, `remove_slot_boot_files`, `install_slot_boot_files` | slot kernel directory | S3 (called by S6) |
| 419–440 | `entry_for`, `entry_content` | entry file access | S3 |
| 442–507 | `efivars_available`, `set_loader_efivar`, `loader_efivar` | EFI variable I/O | S2 |
| 509–596 | `running_root_selector`, `active_slot`, `slot_from_entry_roots` | running-slot detection | S4 |
| 598–617 | `entry_prefix` | entry prefix rule | S3 |
| 619–642 | `slot_views` | status view of each slot | S11/S10 |
| 645–690 | `os_prefix`, `layout_name`, `layout_entry_name`, `entry_name`, `slot_boot_dir`, `new_entry` | naming and new-entry text | S3 |
| 692–720 | `install_loader` | copy systemd-boot onto the ESP | S5 |
| 722–839 | `parse_entry_name`, `loader_default`, `loader_with_default`, `entry_boot_files`, `entry_with_boot_files`, `entry_options`, `entry_with_options` | entry/loader.conf text parsing and editing (pure) | S3 |
| 841–931 | `RootSelector`, `root_selector_from_cmdline`, `root_selector_from_entry_content` | root selector parsing and device resolution | S4 |
| 933–955 | `clear_immutable` | efivarfs immutable flag | S2 |

### `src/stage.rs` (621)

| Lines | Symbol(s) | Duty | Slice |
|---|---|---|---|
| 27–61 | `slot_role`, `open_media` | slot role; release medium → request | S6 |
| 63–101 | `Inputs`, `check_flavour` | flavour guard (mounts the image) | S6 |
| 103–224 | `Target`, `refuse`, `check_inputs`, `check_target` | input and target guards | S6 |
| 226–284 | `write_slot` | remove kernel, mkfs, mount, recstrap, carry, fstab, sync, install kernel | S6 |
| 286–293 | `canonical` | helper | S6 |
| 295–331 | `carry_configuration` | carry by list | S7 |
| 333–441 | `BASELINE`, `signature`, `walk`, `write_baseline`, `read_baseline`, `copy_changed`, `remove_whiteouts` | baseline three-way carry | S7 |
| 443–499 | `metadata`, `copy_tree` | busybox-safe tree copy | S7 |
| 501–511 | `sync_filesystem` | syncfs | S6 |
| 513–538 | `merge_accounts`, `write_preserving_mode` | account-file merge | S7 |
| 540–566 | `write_fstab` | new slot's fstab root | S6 |
| 568–621 | `Mounted` | mount guard | S6 |

### `src/init_system.rs` (283)

| Lines | Symbol(s) | Duty | Slice |
|---|---|---|---|
| 22–53 | `running_profile`, `installed_profile` | distro profile from os-release | S10 (used by S5/S6/S8) |
| 55–63 | `crash_option` | init's crash-reboot option | S3/S8 |
| 65–102 | `Health`, `InitSystemName` | public health JSON | S11 |
| 104–263 | `wait_finished`, systemd and OpenRC startup, failed services, `openrc_running` | startup health | S8 |
| 265–283 | `reboot` | reboot | S8 |

## External joins

| Consumer | Uses | Kind |
|---|---|---|
| `testing/install-tests` (`Cargo.toml:25`, `scenarios/mod.rs:514-520`, `ab_update.rs:9,53-63`) | `Slot`, `Status`, `ENTRIES_DIR`, `entry_name`, `layout_name`, `state_file`, `trial_service_link` | Rust API |
| `xtask/src/tasks/generate.rs:18` | `recab::contract::export` (feature `contracts`) | Rust API |
| `tui/apps/runtime/recab-tui` | `generated/recab.ts` from the contract; `status --json`, `set-next`, `commit`, `rollback`, `stage` argv; `domain/actions.ts` re-derives `state::plan` | CLI + generated types |
| `tools/recstrap/src/bin/recsetup/plan.rs:396` | `recab init --root … --mode …` argv | CLI |
| `distro/builder/recipes/install-tools.rhai:109-111` | `recab install-unit --root … --package <init>` | CLI |
| `distro/builder/src/pipeline/live_tools.rs:43`, `recipe/tools.rs:19` | builds and places the binary in the rootfs | build |
| `testing/ginux-boot/probe.sh:11` | `recab health --json` | CLI |
| Dependencies | `distro_profile::{InitSystem, Profile, Flavour, os_release_value}`, `target_root::*` (fs, mounts, blkid, durable writes, `merge_accounts`), `recpart::{policy, types::{Role, InstallMode}}`, `recinit::live::{FAIL_REBOOT_OPTION, *_ISO_PATH}` | Rust deps |

No crate outside recab calls `status()`, `stage()`, `set_next()`,
`commit()`, `rollback()`, `trial()`, `health()`, `init()` or
`install_unit()`; every runtime caller goes through the CLI. Public items with
no external user: `carried` (module), `layout_entry_name`, `KERNEL_FILE`,
`INITRD_FILE`, `LOADER_CONF`, `error_class`, `stable_exit_code`,
`EXIT_CODE_*`, `crash_option`, `InitSystemName`, `RECAB_BIN`,
`TRIAL_TIMEOUT_SECS` (contract and `main.rs` use some of them in-crate).
