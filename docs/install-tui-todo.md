# Install TUI: what is left

Goal: the install screen is a root terminal on the left and the installation guide on the right (there is no install hub, by the owner's decision of 2026-11-15). Each of the first three parts has a helper app, an alternative to typing its commands, never required work. disk-plan (with its partition editor on top of `recpart`), base-system and system-setup (configuration over `recsetup`, written 2026-11-10, not yet run on an ISO) exist; their READMEs under `tui/apps/install/` say what each does. The steps' commands are the install procedure in `@levitate/docs-content/install`.

Open work:

- **disk-plan:** editable template sizes (it passes none of recpart's size flags); a swap-file option (recpart supports `swap_file`); stories for a refused shrink, a disk with no partition table, and the new-table and format actions.
- **recpart:** disk encryption (LUKS2) with the unlock prompt in recinit.
- **Before the disk:** a preflight app (UEFI, install media, time sync) and a network/WiFi picker. The keyboard layout is chosen in system-setup, which loads it on the live console. Neither distro ships WiFi firmware yet, and LevitateOS lacks `iwd`.
- **Base system:** recstrap cannot be stopped half-way from the app.
- **Configuration:** system-setup creates one administrator; more users, groups, shells and services still go through `recsetup` by hand. Its recsetup types are written by hand: have `cargo xtask gen` write them.
- **Boot and finish:** typed in the terminal, with no helper app: `recsetup --boot` prints a line per step and `recab verify-installation` ends with `PASSED`; the guide says the system is installed only after that. Installation must finish with equivalent personalized systems and boot artifacts in A and B; current recab only populates A, so backend completion is still needed.
- **Installed system:** ship recab-tui in both rootfs images and expose recab's full continuation/update lifecycle: incremental preparation in either direction, changes and downloaded updates in one preparation, conflicts, progress and finalization. Current destructive `stage` and boot actions are only part of that scope; see [recab's requirements](../tools/recab/REQUIREMENTS.md).
- **Testing:** install tests that drive the TUIs over the serial console in guided and manual disk mode; keep the command-typing scenarios passing; stories for every new app.
