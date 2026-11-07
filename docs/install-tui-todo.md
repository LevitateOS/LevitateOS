# Install TUI todo list

Updated: 2026-11-07.

Goal: every step of installing LevitateOS and AcornOS has a TUI. The steps can live in
different apps. The docs pane and a shell stay available beside them for anyone who
would rather type the commands.

## Where it stands today

| Step (docs numbering) | LevitateOS | AcornOS |
|---|---|---|
| Install screen | split pane: bash + install-docs TUI | split pane: sh + `install-guide.txt` in a pager |
| Keyboard, network, clock (1, WiFi) | commands in the docs | commands in the guide |
| Disk (2–5) | the docs tell you to use `fdisk` by hand; disk-plan exists but doesn't ship | `recpart` by hand; disk-plan doesn't ship |
| Base system (6–9) | `recstrap`, `recfstab`, `recchroot` by hand | same |
| Configuration (10–14) | `ln`, `sed`, `locale-gen`, `passwd`, `useradd` by hand | hostname and `passwd` by hand |
| Boot (15–18) | `bootctl`, `recab init`, `systemctl enable` by hand | `cp` of the EFI binary, `recab init` by hand |
| Installed system | recab-tui exists but doesn't ship | same |

disk-plan today only offers a disk and a fixed layout (`ab` or `mutable`, wiping the whole
disk). recpart, underneath it, works the same way: it can't see existing partitions,
keep them, resize them, or plan anything except its two templates. A real partition
editor needs recpart work first (section 2).

## Decisions to make first

Each one changes what gets built. My recommendation comes first.

1. **How the step apps fit together.** An `install` hub on the install screen walks the
   steps in order and starts each step's app (disk-plan, base, setup, boot). Each app writes
   its result to a shared state file the next one reads, and the shell stays one key away.
   The alternative is one big installer app.
2. **Where the configuration logic lives.** Make a Rust backend, like recpart, that
   plans and applies hostname, timezone, locale, keymap, users and services into `/mnt`, with
   one implementation per init system. The TUI stays a thin front over its JSON, and the
   install tests use the same backend. The alternative is a TUI that runs shell commands in
   the chroot itself.
3. **How the apps get onto AcornOS (musl).** Compile each app into one binary per target
   with `bun build --compile` (`bun-linux-x64` and `bun-linux-x64-musl`). That way no bun
   runtime is copied from the build host. Today the builder copies the host's glibc bun,
   which can't run on AcornOS.
4. **How far disk management goes.** Recommended first: create, delete, resize, format and
   mount; reuse an existing EFI partition for dual boot; and swap. Disk encryption (LUKS)
   comes later, as its own item.

## 1. Platform: shipping the apps on both ISOs

- [ ] **Bundle any install app, not just the docs.** Today `install_ux.rs` bundles only the
  docs app. The variant should list its apps in `[install_ux]`, and the builder puts each one
  on the live overlay, together with the tools it declares in its manifest (`tools`).
  *Done when:* adding an app to a variant's list is the only change needed to ship it.
- [ ] **Build per target (decision 3).** Each app becomes a single compiled binary for glibc
  and one for musl. Stop copying `usr/local/bin/bun` from the host.
  *Done when:* install-docs runs on an AcornOS live ISO.
- [ ] **Make the docs know which distro they're on.** install-docs shows AcornOS pages on
  AcornOS: OpenRC instead of systemd, sh, no `locale-gen`, and its own boot steps. Then
  delete `distro/variants/acorn/ring2/install-guide.txt` and its `text` docs path.
  *Done when:* both variants set `docs = { tui_app = "install-docs", ... }`, and no
  LevitateOS-only command shows up on AcornOS.
- [ ] **Bring the docs in line with the tools.** The LevitateOS disk chapter teaches `fdisk`
  keystrokes, while the AcornOS guide uses recpart. Both should use recpart, with the manual
  route as the fallback.
- [ ] **Hand state from one app to the next (decision 1).** Use a state file under `/run`,
  say `/run/levitate-install/state.json`. It holds the disk handoff from recpart (target,
  mounts, slot A and B devices) and what each later step did, so later apps don't ask again,
  and a crashed app can resume.
- [ ] **The install hub.** It shows the steps with done, current and pending marks, starts
  each step's app, shows its result, and lets you go back to a finished step while that is
  still safe. It needs a key that drops to the shell and returns. The install screen runs the
  hub in the left pane in place of a bare shell.
  *Done when:* you can install either variant from boot to reboot without typing a command.

## 2. Disk: a real partition editor

### recpart backend

- [ ] **Read what's on the disk.** `list-disks --json` (or a new `inspect --disk`) also
  reports the partition table: type (GPT, MBR, none), every partition (number, start, size,
  type GUID, label, filesystem, filesystem label, UUID, used space where readable, current
  mountpoint) and the free gaps.
- [ ] **Plan from an explicit layout.** Next to the `ab` and `mutable` templates, accept a
  layout request that lists the actions:
  - keep a partition as it is;
  - delete one;
  - create one in free space, at a given size or the rest of the space;
  - resize one, growing or shrinking it together with its filesystem;
  - format one with a filesystem and a label;
  - give one a role: EFI, root (mutable), root A, root B, var/state, home, swap, or other at
    a chosen mountpoint.

  The plan still gives the exact sfdisk or sgdisk script and every command, in order, as it
  does today.
- [ ] **Resize filesystems.** Shrink and grow ext4 (`e2fsck` + `resize2fs`, offline), FAT,
  and NTFS (`ntfsresize`) for dual boot. Refuse, with an `E0NN` code, any filesystem it can't
  resize safely, and any shrink below the space in use.
- [ ] **Check the layout against the install mode.** An `ab` install needs EFI, two root
  slots of the same size and a state partition. A `mutable` install needs EFI and a root.
  Check partition alignment, the minimum sizes, that the EFI partition is FAT and big enough
  to hold both slots' kernels, and that no mountpoint is used twice. Every refusal gets a code
  and a remediation, like the existing ones.
- [ ] **Reuse an existing EFI partition** (dual boot) without formatting it. Check it has
  room for the kernels.
- [ ] **Swap:** a partition, or a swap file on var or root. Its fstab entry comes from
  recfstab.
- [ ] **A per-partition summary for the confirm step:** which partitions get erased, which
  get resized and which stay untouched, so the confirm page can say exactly what's lost.
- [ ] **Later: disk encryption (LUKS2).** An encrypted root and var, the unlock prompt in the
  initramfs (recinit), and the crypttab and kernel cmdline to go with it.
- [ ] **Regenerate the JSON schemas and TS types** (`docs/frontend-handoff/schemas/`,
  disk-plan's `generated/recpart.ts`). Add scenario fixtures for each new refusal.

### disk-plan UI

- [ ] **Choose guided or manual.** Guided keeps the current templates: erase the whole disk
  with the `ab` or `mutable` layout, now with editable sizes. Today disk-plan doesn't even
  expose recpart's size options (`--efi-size-mb`, `--root-a-size-mb` and so on). Manual
  opens the partition editor.
- [ ] **A partition table view.** It lists the selected disk's partitions and free gaps
  under the disk bar, with number, size, filesystem, label, role and mountpoint, plus a mark
  for what will change (new, resized, formatted, deleted, kept). Moving the selection
  highlights the matching part of the bar.
- [ ] **Actions on the selected row:** new (in free space), delete, resize, format (pick a
  filesystem and label), set role or mountpoint, and keep as is. Undo the last action, and
  reset to what's on disk.
- [ ] **Size input** that accepts MiB, GiB, % or "rest of the space". It shows the minimum
  and maximum allowed for that partition (from its used space and the free space next to it)
  and refuses anything outside them as you type.
- [ ] **Live checks** while you edit: recpart's layout check runs on every change, and its
  refusals show beside the table, each with its docs link (the same blocks as the current
  Refusal and ReadDocs).
- [ ] **A confirm page** built from the per-partition summary. It lists the erased
  partitions in danger colours, says which disks and partitions are touched, and keeps the
  typed confirmation.
- [ ] **Stories** for each action and each refusal: manual layout, shrink, a dual-boot reuse
  of the EFI partition, a refused shrink, and a disk with no partition table. Each needs a
  fixture disk in the recpart stand-in.
- [ ] **At the end, write the recpart handoff to the install state file**, so the base step
  knows the target and the slots.

## 3. Before the disk: keyboard, network, clock

- [ ] **Preflight app:** confirm it booted in UEFI mode (`/sys/firmware/efi`), check the
  installation media (`/run/live-rootfs.erofs`), and show time sync. Each failed check links
  to the docs, as disk-plan's refusals do.
- [ ] **Keyboard layout:** a searchable list. It sets the layout for the live session
  (`loadkeys`) and records it for the installed system.
- [ ] **Network:** shows wired status, and lets you pick a WiFi network and enter its
  passphrase. LevitateOS uses `iwctl` (iwd). On AcornOS, decide between iwd and
  wpa_supplicant and confirm which one the live ISO ships. Then test that the network works.

## 4. Base system: steps 6–9

- [ ] **Progress output from recstrap** (it has no JSON or progress output today): bytes or
  files done out of the total, so the TUI can show a progress bar.
- [ ] **Base app:** run recstrap with progress, then recfstab, and show the generated fstab
  for review, flagging any problems, before appending it. It reads the target from the state
  file.

## 5. Configuration: steps 10–14

- [ ] **The configuration backend (decision 2),** with plan/apply JSON and `--root /mnt`. It
  sets the timezone (`/etc/localtime`), locale (systemd: `locale.gen`, `locale-gen` and
  `locale.conf`; AcornOS: `LANG` only, since musl has no `locale-gen`), the console keymap
  (systemd: `vconsole.conf`; OpenRC: `conf.d/keymaps`), the hostname (and hosts), the root
  password, and users with groups, shell and the admin group (wheel, with sudo or doas,
  whichever the variant ships).
- [ ] **Setup app:** a searchable timezone list grouped by region, the locale list, the
  keymap (pre-filled from preflight), the hostname (validated), the root password entered
  twice (masked, with a strength hint), a user form, and a summary page before apply.

## 6. Boot and finish: steps 15–18

- [ ] **Boot backend:** install systemd-boot (LevitateOS: `bootctl install`; AcornOS: the
  manual copy its guide does now), then `recab init` with the slot devices from the state
  file (the `ab` mode) or one boot entry (the `mutable` mode). Also the machine id, and the
  services for each init system (`systemctl enable` or `rc-update add`), such as the network.
  This could be part of the configuration backend.
- [ ] **Boot app:** show the boot entries it will write, the default slot and the services,
  then apply. A finish page offers to unmount and reboot, and reminds you to remove the
  install media.

## 7. Installed system

- [ ] **Ship recab-tui in the rootfs of both variants,** built per target as in section 1, so
  slot status, set-next, and rollback have a UI after install.
- [ ] Decide whether the installed system needs a first-boot screen (none planned).

## 8. Testing

- [ ] Stories for every new app, run by `cargo xtask check tui`, each with stand-ins for its
  tools.
- [ ] **Install tests that drive the TUIs** in QEMU for both variants (install-tests over the
  serial console), from boot to a booted installed system, in guided and manual disk mode.
- [ ] Keep the command route covered: the existing scenario installs that type the commands
  keep passing.
