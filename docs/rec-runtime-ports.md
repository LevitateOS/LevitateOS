# Research Plan: What to Port to rec\* Rust

Last updated: 2026-11-15

This is a research plan only. Nothing in it authorizes implementation; its job is to decide, with evidence, what is even worth porting.

## Goal

Find out — with evidence, not vibes — which components of the **running installed OS** (not installer-time tools) are worth porting to our own rec\* Rust implementations, and produce a ranked candidate list with a clear reject list. No code gets written in this phase; `reclogin` is already documented separately and is out of scope here except as a known entry.

## Success Criteria

- A complete inventory of what actually runs in both installed systems (LevitateOS/Fedora packages, AcornOS/Alpine packages), from the locks and rootfs trees, not from memory.
- Every candidate scored on the same criteria, with a named verdict: **port**, **maybe later**, or **never**.
- Each “port” verdict names its consumers (who would break), its interface surface (D-Bus, files, sockets), and the smallest validation that would prove a replacement works.

## Approach

Research in four passes, each producing evidence the next pass consumes:

1. **Inventory the runtime surface.** From `distro/builder/os-recipes/sources/levitate-rpm-lock.rhai`, the desktop lock, and the Acorn apk locks: list installed daemons/services and their units (systemd units, OpenRC scripts) from a built rootfs tree, split by distro and by shared-vs-different.
2. **Classify ownership.** For each component: is it our glue, a small upstream daemon with a stable interface, or a load-bearing upstream stack (init, D-Bus daemon, polkit, PipeWire, network manager, display stack)? Only the first two classes can ever be candidates; the third is rejected on sight with the reason recorded.
3. **Map interfaces and consumers.** For surviving candidates, record who talks to them (desktop shell, GUI apps, CLI tools, other services), through what (D-Bus API, socket, files under `/run` or `/sys`), and what a Rust replacement must keep bit-compatible.
4. **Score and rank.** Rate each candidate on interface stability/size, blast radius if wrong, security sensitivity, and how much of it is actually ours to own. Output the ranked list plus the explicit reject list so “never” decisions are also on record.

## Steps

1. Build the service/daemon inventory for both distros from the locks and a release rootfs tree; note where Levitate and Acorn already differ.
2. Tag each item ours/upstream-small/upstream-stack; drop upstream-stack items into the reject list with one-line reasons.
3. For the remainder, map consumers and interfaces from the tree (unit files, D-Bus service files, GUI/CLI call sites in `gui/` and `tools/`).
4. Score, rank, and write the candidate list with per-item verdicts and the validation each would need.
5. Present the list for the user's decision; only then would any porting be planned, one component at a time.

## Validation Plan

- The inventory is checkable: every claimed service appears in a lock file or a built rootfs tree (cite the file).
- Every “port” candidate names at least one real consumer found in the tree; candidates with no consumer are demoted.
- The reject list is explicit, so nothing is silently left ambiguous.

## Risks / Open Questions

- Effort estimates for daemons are unreliable until their interface surface is mapped (step 3); the plan deliberately makes no porting commitment before that.
- Whether “worth it” weighs user-visible ownership (login screen, guest agent) over invisible plumbing is the user's call at the decision point, not something research can settle.
- Highest-risk step: step 3 — a candidate that looks small (guest agent, power, time sync) can hide a wide D-Bus/protocol surface, which is exactly what the scoring must expose before anything is promised.

## Research results (2026-11-15)

Status: **partial.** Inventories were derived from the package locks and built rootfs trees (service wants/runlevels and `installed-system.rhai` enablement), corrected by a completeness critic; they are not live-boot proven. The full per-component inventory submissions were large and are retained uninspected in the research run's session storage; the verdicts below rest on the agent synthesis plus critic evidence.

### Inventory summary

Both installed systems run essentially the same upstream daemon set under different supervision: NetworkManager, chronyd, auditd, D-Bus, Avahi, BlueZ, CUPS, firewalld, polkit, SDDM, PipeWire/WirePlumber — systemd units on LevitateOS, OpenRC runlevels on AcornOS. The seat manager is systemd-logind vs elogind; the desktop is Plasma/KWin vs LXQt/Openbox+picom. Already ours at runtime: recab (plus recipe `--system` and the recab/recipe GUIs) and recab-trial. Corrections from the critic: `sshd` and `smartd` are installed but **not** enabled on either system; systemd-resolved/timesyncd/networkd/homed/userdbd are preset-listed but have no `wants` symlinks, so their first-boot state (and any conflict with chronyd/NetworkManager) is unproven.

### Worth porting (ranked)

| # | Candidate | Why it qualifies | Consumers that must keep working | Smallest validation |
|---|---|---|---|---|
| 1 | qemu-guest-agent | Small, stable JSON-over-virtio-serial protocol (`/dev/virtio-ports/org.qemu.guest_agent.0`); VM-only; low blast radius | Host QEMU/libvirt, `cargo xtask` scenarios (recqemu/recshuttle) | Boot both distros in recqemu; QMP guest-ping/guest-info/guest-shutdown work through the ported agent; absent port = clean exit |
| 2 | iio-sensor-proxy | Tiny, years-stable D-Bus API (`net.hadess.SensorProxy`); read-only claim/release | KWin/Plasma auto-rotate and brightness, LXQt | Fake-IIO fixture: claim sensor, inject rotation/ALS values, verify `PropertiesChanged`; no device = no claim |
| 3 | power-profiles-daemon | Small D-Bus surface (3 profiles); reversible sysfs writes only | powerdevil/Plasma power settings, LXQt-powermanagement | `platform_profile` changes for all 3 profiles (hold/release included); absent sysfs = clean no-op. Profiles only — full UPower stays upstream |

### Maybe later (only if a consumer need appears)

spice-vdagent, rtkit-daemon, colord, smartd, accounts-daemon, geoclue2 — upstream is adequate today; smartd additionally has no named consumer in the workspace, and geoclue is privacy-sensitive.

### Explicit reject list (never)

NetworkManager/wpa_supplicant, BlueZ, PipeWire/WirePlumber, Avahi, CUPS, firewalld/nftables, KWin/Plasma, LXQt/Openbox, SDDM, elogind/logind, either PID 1 (systemd/OpenRC), D-Bus itself, chronyd, UPower proper, polkit, udisks2, fwupd, bolt, auditd, sshd, eudev/agetty/busybox/musl, the accessibility stack (at-spi/orca/speech-dispatcher), and secret/file daemons (kwallet/gnome-keyring/gvfs): interfaces are huge or protocol-complex, blast radius is privileged/firmware/disk, or there is no advantage over mature upstream. Keep systemd-sysupdate **disabled** (it duplicates recab A/B) and never add a PackageKit backend (it duplicates recab/recipe). Anything overlapping A/B updates, package install, UKI/initramfs/ISO creation, partitioning, extraction, fstab, chroot, setup, or VM/SSH transport duplicates an existing rec\* tool and is rejected by construction.

### Gaps flagged (packaging decisions, not ports)

- **ModemManager and PackageKit exist as libraries/UI only, with no daemon** in either rootfs — WWAN UI and Discover/PackageKit-style backends are currently dead. Fix by packaging decision (enable the upstream daemon or drop the dead UI), not by a rec\* port.
- **recab-gui binaries** are present in the Levitate image with no systemd unit, autostart, or D-Bus activation found — dead binary vs install-time activation is unresolved.
- **Nothing here is live-boot proven.** Exact upstream D-Bus names/paths/sockets and activation (especially Acorn D-Bus-spawned udisks2/fwupd/UPower under dbus-daemon without systemd) need one real boot of each distro before any port is scheduled.

## Replacing more upstream userland (2026-11-15, follow-up)

Question asked: how much more of the Fedora/Alpine userland can be replaced with our own rec* code, so we depend on those distros as little as possible? Answer: the realistic wins are at the edges, in three groups.

**Worth replacing with our own (small, bounded):**

- **SDDM → reclogin** — the biggest remaining upstream runtime piece with a rec* successor; already planned (see `docs/reclogin.md`).
- **iio-sensor-proxy** and **power-profiles-daemon** — the two ports ranked above (after qemu-guest-agent, done as recguest): tiny, stable D-Bus surfaces; read-only claims or reversible sysfs writes only.

**Do not rewrite — stop burning them into the image instead:** `htop`, `smartmontools`, `hdparm`, `pciutils`/`usbutils`, `less`, `vim`, `curl`, and similar standard tools are fine upstream. The improvement is not a rec* rewrite; it is shipping them Recipe-installed (system scope, into the unlocked root) so they are removable — the §2.4 rule in `docs/product-handoff.md`. Zero new code to maintain, same independence win for the user.

**Never replace (the dependency correctly stays):** systemd/OpenRC (either PID 1), glibc/musl, bash/coreutils, NetworkManager/wpa_supplicant, PipeWire/WirePlumber, BlueZ, CUPS, chrony, D-Bus itself, and the rest of the “Explicit reject list” above. Replacing these does not make the OS more independent in any useful sense; it makes us maintainers of an init system, a libc, and a network stack forever, for no user-visible gain.

End state this points at: our kernel, installer, rec* tools, login screen, guest agent, and Recipe on top, with Fedora/Alpine supplying the big commodity userland (init, libc, network, audio) as verified packages. The upstream dependency shrinks at the edges, not at the core — deliberately.
