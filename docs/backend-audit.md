# Audit of the installer's backends (2026-11-07)

> Recab scope correction, 2026-11-10: recab owns system updates and symmetric A/B continuation. Its earlier boot-selection and destructive-staging scope below is historical. See [current requirements](../tools/recab/REQUIREMENTS.md); the recorded checks do not establish the missing continuation lifecycle.

The TUIs run on three Rust tools: **recpart** (disk-plan), **recab** (recab-tui) and
**recstrap**'s binaries (recstrap, recfstab, recchroot, recsetup), which the planned base,
setup and boot apps will drive. Each was read in full. Clippy is clean on all three;
recab's 17 lifecycle tests and recstrap's 15 non-root tests pass. Findings marked ✔ were
re-checked against the code after the audit. "Contract" means what a TUI can read.

Fix order proposed at the end.

## recpart (tools/recpart, at 0fb0e80)

### High
- ✅ fixed (recpart 7e60a2e, `apply --expect`; disk-plan passes it) · ✔ **Apply isn't tied to the plan that was confirmed** (cli.rs:272-292, plan.rs:217,
  manual.rs:187). `apply` re-plans from the live disk; template mode checks no identity at
  all, and manual mode's `table_id` is optional (null for `new_table`, and a blank disk has
  none). A USB stick plugged in between listing and apply can make `/dev/sdb` another disk,
  and `wipefs -a` erases it. The GPT GUID also survives outside repartitioning, so
  `delete 3` deletes whatever is number 3 now. Fix: `apply --plan plan.json` that checks
  serial, size, table id and every `summary[].before` against a fresh inspect, then runs that
  exact plan; at least `--expect-serial`/`--expect-table-id` in every mode.
- ✅ fixed (recpart 7e60a2e; disk-plan shows them) · ✔ **Signatures on the whole disk are invisible** (inspect.rs:100-122). Only `pttype` and
  `ptuuid` of the disk row are read. A LUKS container, btrfs or LVM PV on the bare disk shows
  as an empty disk with table `none`; the Confirm page lists nothing lost; `wipefs -a`
  destroys it. Fix: report disk-level signatures (`wipefs -n -J`) and list them as erased.
- **A failed apply hides how far it got; no progress while it runs** (exec.rs:136-140,
  cli.rs:294). Step records are dropped on error, so the UI can't tell whether the shrink or
  the table write happened; retrying after the table was written fails (E015). Fix: NDJSON
  step events during apply, `steps` and `table_written` in the error document, recovery text
  per phase.

### Medium
- **Templates skip the policy manual layouts enforce** (policy.rs:374-432): no EFI minimum,
  slot minimum or equal slots (`--efi-size-mb 1 --root-a-size-mb 1 --root-b-size-mb 9000`
  plans). Fix: express templates as a manual layout and plan them through
  `build_manual_plan`, which also removes the second sfdisk generator.
- **Exit codes collide; some errors aren't JSON**: clap errors exit 2 (= E002 MissingTool) as
  plain text even with `--json`; panics exit 101. Fix: `Cli::try_parse`, mapped to a JSON
  error.
- **Dry-run warnings are strings** (exec.rs:104): code and remediation are lost. Fix:
  `Vec<ErrorPayload>`.
- **The check after the table write compares names only** (exec.rs:528-574); new partitions
  reuse deleted numbers, so a stale kernel partition passes. Fix: compare
  `/sys/class/block/<p>/{start,size}` with the plan.
- **`resize_blocker` blames root for every failed probe** (inspect.rs:318, manual.rs:388):
  "that needs root" even as root, for example on a damaged FAT. Fix: keep the probe's stderr
  as `used_bytes_error`.
- **list-disks offers disks in use**: loop devices (the live squashfs) and the boot medium,
  unmarked. Fix: `blockers[]` from apply's own checks; loop devices only with `--all`.
- **disk-plan duplicates recpart policy**: layout.ts copies free-gap placement and the GPT
  usable range; setup.ts copies minimum and default sizes, the EFI GUID and per-role
  filesystems. Fix: `recpart policy --mode X --json`, and let `plan --layout` return the
  projected summary and *all* refusals with structured fields (it returns the first, as
  prose).

### Low
- Inspect without root silently falls back to lsblk and the default usable range, so a
  non-root dry run can differ from a root apply; add a `privileged` field.
- MBR `table_id` comes as `0X…` from sfdisk and bare hex from lsblk.
- `PartitionTemplate.gpt_type` is `U`/`L` in templates, a GUID in manual plans; `bootable`
  differs for the same ESP.
- The minimum disk size ignores the first MiB and the backup GPT.
- `query_disk_target` is dead; `MB` and `MIB` duplicate; parsed tools aren't run with
  `LC_ALL=C`; every re-plan re-runs the used-space probes on every partition.

### Missing
btrfs and xfs (create, and xfs grow), LUKS, layouts across disks, BIOS boot (and no check of
`/sys/firmware/efi`), ESP reuse checks beyond free space.

### Solid
Stable error table with schema versions and generated types; resize order (shrink before the
table write, grow after the check; a failed shrink stops first); `ntfsresize --no-action`
and `e2fsck -f` first; thorough preflight (canonical path, holders, swaps, O_EXCL open,
read-only flag, `wipefs` without `--force`); sfdisk scripts that keep ids and attributes;
correct nvme/mmcblk names; 4Kn math; rollback of mounts and swaps on failure.

## recab (tools/recab, at 8f76ddf)

### High
- ✅ fixed (recab 5505421) · ✔ **A stale boot selection can go live after `stage`: a reboot loop** (systemd_boot.rs:
  116-127, init.rs:101-106). `read()` drops a `LoaderEntryDefault`/`OneShot` that names an
  entry it can't boot, and stage's checks use only that filtered view; `init` skips resetting
  the variables when efivars aren't available. A reinstall without efivars leaves
  `LoaderEntryDefault=…-b.conf`; status shows default A; `stage` writes B's kernel and the
  firmware variable makes B the default, not as a trial, with `rd.emergency=reboot` on its
  entry. Fix: check the raw variables in `stage` and `set-next` (refuse or reset), make
  `init` fail or warn without efivars, show raw and effective default in status.
- ✅ fixed (recab 5505421) · ✔ **`stage` destroys the slot before checking its inputs** (stage.rs:87-96, 139-161). Only
  existence is checked; the slot's kernel directory is removed and the partition formatted
  before anything verifies the image. `--rootfs vmlinuz` formats the slot, recstrap fails,
  the rollback target is gone, exit 1 untagged; `--kernel /boot/<id>-b/vmlinuz` deletes its
  own input. Fix: EROFS magic, size against the partition, rootfs not the target device,
  kernel/initrd not inside the slot directory, all before touching anything.

### Medium
- **A failed stage is invisible** (lib.rs:224-227): nothing records that the slot is now
  empty. Fix: `RECAB_ERR_STAGE_INCOMPLETE` and an `incomplete_stage_slot` in state and status.
- **The firmware-only trial check misfires** (systemd_boot.rs:138-145, lib.rs:265-293):
  "selected ≠ default" also matches a menu pick and systemd-boot's own fallback, which can
  loop. Fix: mark trial entries (`recab.trial=B`) and read `/proc/cmdline`.
- **Trial options live in two lists** (systemd_boot.rs:29, init_system.rs:109-114); one
  missed option stays on a committed slot. Fix: derive one from the other.
- **`init` leaves an old `<id>-b/` on a reused ESP** (init.rs:48-56), so B looks bootable.
- **Error classes are inconsistent**: an EFI write that changed nothing exits 1 untagged;
  missing tools, "no PARTUUID", lock and input errors exit 1; code 21 mixes refusals with
  "can't read the layout"; 23 is returned even when recab didn't reboot.
- **The UI can't get what it needs**: no schema version on `status`; `pending_trial_slot`,
  `previous_good_slot` and per-slot bootability aren't exposed, so recab-tui's mirror of the
  state machine (actions.ts) misses the bootability refusal and says "unchanged" for the
  rollback after a commit. Fix: `recab <op> --dry-run --json` from `plan()` plus the
  bootability check (verdict, reason code, fix, before and after), a JSON error with a stable
  reason, then delete the TS mirror.
- **The lock blocks forever** (store.rs:69): a commit during a 10-minute stage hangs. Fix:
  `LOCK_NB` with `RECAB_ERR_BUSY`, the holder in the lock file, `busy` in status.
- **`stage` reports no progress** (recstrap runs `--quiet`, output buffered). Fix: progress
  lines per phase, recstrap's progress passed through.

### Low
A trial gets exactly one boot (boot counting would allow more); rollback never reports
"nothing to change"; the failed-trial path releases the lock before rolling back; the merged
shadow is briefly 0644; an unreadable cmdline turns off the running-root guard; init.rs
repeats file names and the blkid call; systemd-boot may not treat a missing initrd as
unbootable as status does (unconfirmed).

### Solid
`plan()` is the whole state machine without I/O; boot selection is written before state,
one-shot cleared before default, `loader.conf` restored if the firmware write fails; durable
writes; forward-compatible state file; stage removes the kernel first and refuses the running
root and mounted partitions; trial judges only what `set-next` scheduled; typed exit classes.

## recstrap, recfstab, recchroot, recsetup (tools/recstrap, at d7695ae)

### High
- ✅ fixed (recstrap 5e6aa42, private mount namespace) · ✔ **A killed recchroot or recstrap leaves mounts behind** (session.rs:18). Only SIGINT,
  SIGTERM and SIGQUIT are handled; SIGHUP (an SSH drop, a TUI closing its pty) skips cleanup,
  leaving binds of the host's `/dev`, `/sys`, `/proc`, `/run` under `/mnt`, where a later
  `rm -rf /mnt/*` deletes host device nodes. recsetup runs a recchroot per step. Fix: a
  private mount namespace per run (mounts vanish with the process); at least handle SIGHUP.
- ✅ fixed (recstrap 5e6aa42) · ✔ **The host-root guard misses `/home` on btrfs or its own partition**
  (bin/recstrap/guards.rs:148-156): it compares `st_dev` with `/`, and every btrfs subvolume
  (Fedora's `/home`) has its own. `recstrap --force ~/x` passes, the case the guard exists
  for. Fix: refuse anything the host has mounted that isn't under the install mount
  (`/proc/self/mountinfo`).
- ✅ fixed in code (release hook builds `initramfs-installed.img` onto the ISO; recsetup 5ce612c defaults to it and refuses the live one); not yet proven by a release build and boot · ✔ **The default initrd is the live initramfs** (bin/recsetup/plan.rs:422-429). On systemd,
  recsetup defaults to `/run/live-media/boot/initramfs.img`, which the ISO fills from
  `initramfs-live`; the live init looks for the live medium and only reports `root=`. The
  build makes `initramfs-installed.img` but ships it beside the ISO, not on it. The E2E test
  passes the same path, so it can't catch this. Very likely an installed system that doesn't
  boot, or boots the live system again; needs one boot to confirm. Fix: put the installed
  initramfs on the ISO and default to it, or take it from the target.
- ✅ fixed (recstrap 5e6aa42, E011/E035) · **Boot devices aren't checked against what is mounted** (plan.rs:498-546): a wrong
  `--root-device` gives an entry pointing at another disk. Fix: require it to be the source of
  `--root` (`findmnt`), and the ESP on the same disk.

### Medium
- **On OpenRC, an unmounted or non-ESP `/boot` is only a warning** (plan.rs:455-482); the
  Copy method writes `BOOTX64.EFI` into the target's `/boot` directory and reports success.
  Fix: an error unless `/boot` is a mounted vfat ESP.
- **No firmware boot entry is created** (`bootctl install --no-variables`; recab sets only
  `LoaderEntry*`): booting relies on the removable-media fallback, which a machine with other
  `Boot####` entries may never reach. Fix: register a `Boot####` entry at install time.
- **recstrap, recfstab and recchroot can't back a TUI**: no `--json`, typed errors or schema
  version; recstrap is silent while `cp -aT` copies gigabytes; recfstab only prints (the E2E
  test appends, so a re-run duplicates entries) and has no protected-path check. Fix: progress
  JSON lines, `recfstab --write` replacing its own block, one shared JSON error shape.
- **recpart's handoff is never read**: recfstab scans mounts, recsetup takes devices by hand;
  the glue is only in the E2E test, and nothing records which step finished. Fix:
  `--handoff FILE` on recfstab and recsetup, checked against the mounts, and an install state
  file in the target.
- **recsetup's JSON doesn't stream or match recpart's contract**: one document at the end,
  error payload without component/expectation/observed, codes not stable names, no generated
  types, no integration tests.

### Low
`partuuid` and the exit-status formatting are duplicated; the README counts 14 and 15 checks
for 12; `bind_dir` leaves directories; recchroot binds the host's `/run` instead of a tmpfs.

### Solid
One owner for mounts, undone in reverse with lazy unmount and without deleting non-empty
directories; recstrap's checks on canonical paths with overrides that can't waive the
protected paths; recsetup resolves every path inside the target, writes files atomically,
reads passwords only from stdin, has a plan and dry run with an exact command per step,
stops at the first failure, re-runs safely, and detects systemd or OpenRC from the target.

## Fix order proposed

1. The data-loss and unbootable-system bugs: recpart's plan-to-disk identity and whole-disk
   signatures; recab's input checks before formatting and the stale-variable loop;
   recstrap's mount namespace and host-root guard; recsetup's installed initramfs (one boot
   to confirm first) and its root/ESP checks.
2. What the planned install TUIs need: progress events for recpart apply, recab stage and
   recstrap; machine-readable plans and refusals (`recab --dry-run --json`,
   `recpart policy --json`, all refusals from `plan --layout`), so the UIs stop mirroring
   backend rules; one JSON error shape across all tools; the handoff and install state file.
3. The medium correctness items and the rest.
