# recab tasks and lanes

Every finding in [findings.md](findings.md) and every allocation in
[desired-tree.md](desired-tree.md) lands in exactly one task. Each task owns
its files exclusively while it runs; tasks in different lanes never write the
same file. Commit each task on recab's `main` when it works (superproject
`AGENTS.md`: no branches, commit early); a task that crosses into another
repository commits there too, then the superproject pointer.

`tests/common/mod.rs` (the shared harness) is written only by T-R0; a lane
that needs a new helper puts it in its own test file.

"Gate" names what must land or be decided first. The checks named are the
ones that show the task works; none is a mandatory pipeline.

## Order

```text
T-R0 (pure moves, serial gate)
  ├─ Lane 1  T-M1 → T-T1
  ├─ Lane 2  T-B1
  ├─ Lane 3  T-C1 → T-S1 → T-K2 → T-S3      (T-K1 needs D-6, T-K3 needs D-4)
  ├─ Lane 4  T-I1                            (host-firmware part needs D-5)
  └─ Lane 5  T-D1                            (may start before T-R0: docs only)
then  T-A1 → T-X1 (needs D-3) → T-V1 → T-D2
```

## Tasks

| Task | Lane | Gate | Writes (exclusive) | Does | Closes | Shows it works |
|---|---|---|---|---|---|---|
| T-R0 | gate | — | all of `src/`, `tests/` | Pure moves to the [desired tree](desired-tree.md): split `lib.rs` (→ `system.rs`, `status.rs`, `trial.rs`; `Slot`/`TrialRecord` → `state.rs`; request types to `stage.rs`/`init.rs`; `stage()` → `stage.rs`), `systemd_boot.rs` (→ `entry.rs`, `efivars.rs`, `root.rs`; `install_loader` → `init.rs`; `slot_views` → `status.rs`), `stage.rs` + `carried.rs` (→ `carry.rs`), drop `store.rs` wrappers, split `tests/lifecycle.rs`. No behaviour change. | allocation; F-A2 | `cargo check -p recab -p install-tests`; `cargo test -p recab` (same 20 pass); `cargo xtask gen --check` current |
| T-D1 | 5 | — | `README.md`, `REQUIREMENTS.md`, `CLAUDE.md` | Correct today's drift: initramfs passages, REQ-CLI-001 (`health`, `--from-media`, `--change-flavour`, `install-unit` writes the carried list), the flavour refusal in REQ-SM-013, header, CLAUDE.md. | F-D1 (docs part), F-D2, F-D3 | read-through against `main.rs --help` |
| T-M1 | 1 | T-R0 | `state.rs`, `system.rs`, `trial.rs`, `tests/slots.rs`, `tests/trial.rs` | `set-next` of the known-good slot refused; set-next no-op only when the state also names the slot; a boot is a trial only when the firmware says one-shot and the state names it, and a stale pending slot is cleared; failed trial selects before it saves; read-only open for a non-trial boot; `TrialRecord` keeps unknown fields; rename the OpenRC initramfs assertion. Each with a scratch-tree test. | F-M1, F-M2, F-M3, F-M4, F-M5, F-M6, F-D1 (test part) | `cargo test -p recab` |
| T-T1 | 1 | T-M1 | `trial.rs`, `services/recab-trial.service`, `services/recab-trial.openrc`, `tests/trial.rs` | Decide "is this a trial" after startup finished; no state write into an unmounted `/var`. | F-T1 (gate part), F-T2 | `cargo test -p recab`; `cargo xtask test ab-update levitate` and `acorn` |
| T-B1 | 2 | T-R0 | `systemd_boot.rs`, `entry.rs`, `efivars.rs`, `root.rs`, `tests/boot_selection.rs` | Non-slot `LoaderEntryDefault` modelled (status shows it, commit repairs it, set-next refuses); set-next writes a missing `LoaderEntryDefault`; parse edge cases (prefix case, `@saved`, `devicetree`, forced non-zero `panic=` on trials, operator crash option kept); `Untouched` message; kernel installed under a temp name before the entry rewrite. | F-B1, F-B2, F-S2, F-S5, F-T1 (default-variable part) | `cargo test -p recab` |
| T-C1 | 3 | T-R0 | `config.rs`, `stage.rs`, `carry.rs`, `init_system.rs` | Running root, `/proc` and the stage directory come from `Config` with `RECAB_*` overrides. | F-C1 | `cargo test -p recab` |
| T-S1 | 3 | T-C1 | `stage.rs`, `carry.rs`, `tests/stage.rs` | Every refusal before any mutation: image `ID`/`REC_INIT`/architecture and flavour, the image's carried list parsed, the running fstab's `/` line, all before `reset_stale_selection` and before the state save. | F-S1, F-S3, F-S4 | `cargo test -p recab` (refusal order where no mount is needed); `ab-update` both distros |
| T-K2 | 3 | T-S1 | `carry.rs`, `tools/recstrap/target-root/src/lib.rs` (`merge_accounts` only) | Carry never follows a symlink inside the new slot; account merge refuses UID/GID collisions and unions group members; signature includes mode and owner. | F-K3, F-K4, F-K6 (mode part) | `cargo check`; `ab-update` both distros |
| T-S3 | 3 | T-K2 | `stage.rs` | Clear stale `/run/recab/*` mounts at start; classify post-save failures as `PartialUpdate`. | F-S6, F-S7 | `cargo test -p recab` |
| T-K1 | 3 | D-6, T-K2 | `carry.rs`; per D-6 the rootfs recipe (`distro/builder/recipes/…`) or recsetup | Every slot carries the baseline of its own image; the first update carries only machine changes. | F-K1 | `ab-update` both distros, plus a check that an image-enabled service dropped by the new image is not enabled after the update |
| T-K3 | 3 | D-4 | `services/carried`; per D-4 `installed-system.rhai` (`/root` link) | Carried list content and `/root` per D-4; list header describes whiteouts. | F-K2, F-K5, F-K6 (deleted files), F-D4 (list header) | `ab-update` both distros |
| T-I1 | 4 | T-R0; D-5 for the host-firmware part | `init.rs`, `tests/install.rs`; recsetup's duplicate `partuuid` (`tools/recstrap/src/bin/recsetup/plan.rs`) | `init` checks slot A is the partition at `--root`, `boot/` and `var/` are mount points, removes a leftover slot-B kernel dir, takes the lock; host-firmware reset per D-5. | F-I1, F-I2 | `cargo test -p recab`; `cargo xtask test all levitate` and `acorn` (install path) |
| T-A1 | join | lanes 1–4 | `lib.rs`, any `pub` item | Narrow the public surface per D-2. | F-A1 | `cargo check -p recab -p install-tests -p levitate-xtask` |
| T-X1 | join | T-A1, D-3 | `status.rs`, `state.rs` (plan dry-run), `contract.rs`, `docs/*.schema.json`, `tui/apps/runtime/recab-tui/src/**` | `status --json` carries each action's verdict and a `schema_version`; `health` gets one too; recab-tui renders recab's verdicts, uses generated `ERROR_CLASSES`, drops its EROFS check, offers `--from-media`/`--change-flavour`. | F-X1, F-C2 | `cargo xtask gen`; `cargo xtask check tui` |
| T-V1 | join | T-S1, T-K1, T-K2 | `testing/install-tests/src/scenarios/ab_update.rs` | Scenario asserts: machine edit survives, untouched shipped file takes the new version, deletion stays deleted, added user survives, `/root` survives (per D-4), flavour or distro mismatch refused. | F-V1 | `cargo xtask test ab-update levitate` and `acorn` |
| T-D2 | 5 | all above | `README.md`, `REQUIREMENTS.md`, `services/carried` header (if T-K3 did not) | Document the landed behaviour: refusal order, baseline and whiteouts, trial judging rule, code table (new files), test files, drop `chroot` from preconditions. | F-D4 | read-through |
| — | builder | none | `distro/builder/recipes/install-tools.rhai` | Packaging for aarch64; readiness checks the service and carried list. Not recab's tree. | F-P1 | builder owner's checks |
| — | owner | — | — | Decide D-1 (repo merge). No recab task depends on it. | F-G1 | — |

## Verification ladder

1. Each task: `cargo check` of recab and its Rust consumers; `cargo test -p recab`.
2. Contract tasks (T-R0, T-X1): `cargo xtask gen --check`, `cargo xtask check tui`.
3. Stage, carry, trial and init tasks: rebuild the live-tools ISO (rootfs
   recipe changes need live-boot rebuilt first), then
   `cargo xtask test ab-update levitate` and `cargo xtask test ab-update acorn`.
   VM and ISO builds run 10:00–23:00 only.
4. Before calling the plan done: `cargo xtask test all levitate`,
   `cargo xtask test all acorn` and both `ab-update` runs on the final tree.

## Blocked until decided

| Decision | Blocks |
|---|---|
| D-3 TUI verdicts | T-X1 |
| D-4 carried list and `/root` | T-K3, the `/root` assertion in T-V1 |
| D-5 init on a non-target host | the host-firmware part of T-I1 |
| D-6 where a slot's baseline comes from | T-K1, and through it T-V1 |
| D-2 library entry points | the scope of T-A1 |
