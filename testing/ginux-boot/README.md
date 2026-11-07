# ginux-boot: AcornOS and LevitateOS booting under Ginux

Boots an OS's root filesystem with its real init as PID 1 under
[Ginux](https://github.com/gmkurtzer/ginux), the way Ginux's Gate 9 boots
Rocky's systemd, and checks how far it gets:

- **AcornOS**: busybox init runs OpenRC's sysinit, boot and default runlevels.
- **LevitateOS**: systemd reaches `multi-user.target`.

No VM and no kernel of the OS's own: Ginux supervises every system call. The
check passes when init finishes and everything that does not work is a known
gap listed in `acorn/expect` or `levitate/expect`. A gap that starts working
is reported, so the list only shrinks.

## Running it

```bash
testing/ginux-boot/ginux-boot.sh acorn --rootfs ROOTFS --ginux GINUX_CHECKOUT
testing/ginux-boot/ginux-boot.sh levitate --rootfs ROOTFS --ginux GINUX_CHECKOUT
```

- `--ginux`: a Ginux checkout on the `levitate/boot` branch of
  [LevitateOS/ginux](https://github.com/LevitateOS/ginux/tree/levitate/boot)
  (upstream main plus the fixes below). The script builds it.
- `--rootfs`: the OS's root filesystem tree, copied and never changed. For
  AcornOS, the `rootfs` output of
  `distro/builder/recipes/alpine-live-source-rootfs.rhai`; for LevitateOS, that
  of `distro/builder/recipes/fedora-rpm-source-rootfs.rhai`.
- `--keep` keeps the work directory: the staged root, `console.log`,
  `audit.log` (every system call decision) and `probe.out`.
- Needs a Rocky Linux 10 host (Ginux's target), `sudo` (Ginux and staging run
  as root, as Ginux's own tests do), `script` and `cargo`.

The recorded runs used roots built on the host from the same pinned inputs
(AcornOS with `apk.static` from the pinned ISO and extra apks, offline, as
the rootfs recipe does; LevitateOS by
unpacking the locked Fedora 45 RPMs with the recipe's steps), because
Recipe's builds under Ginux are blocked on the `tools/ginux` pin.

## How it works

1. Copy the rootfs and stage it for Ginux: `acorn/stage.sh` (static `/proc`
   and `/sys` files, devices, AcornOS's inittab) or `levitate/stage.sh`
   (Ginux's own `prepare_root`, as for Gate 9).
2. Boot with `ginux native <os>/instance.toml ROOT` under `script(1)`, which
   gives the console a terminal.
3. When init is done, the probe (`::once` in the inittab, or
   `ginux-probe.service` after `multi-user.target`) writes `/ginux-probe.out`:
   OpenRC's runlevel and service states, or every systemd unit's state.
4. The host reads it, stops the instance and judges it against `expect`.

## Results (2026-11-07, Ginux `levitate/boot` at `5cd55224`)

| OS | Init reached | Time to probe | Working | Known gaps |
| --- | --- | --- | --- | --- |
| AcornOS (Alpine 3.24.4, OpenRC 0.63.2) | default runlevel | 6 s | fsck, root, localmount | 3 |
| LevitateOS (Fedora 45, systemd 262) | multi-user.target ("degraded") | 29 s | logind, udevd, userdbd, getty | 22 units, 6 causes |

Gate 9's Rocky 10 image (systemd 257) boots to `multi-user.target` with no
failed units on the same branch; upstream `53a54add` also reaches login.

## The Ginux branch

`levitate/boot` is upstream main plus:

| Commit | What | Found by | Upstream |
| --- | --- | --- | --- |
| deliver SIGCHLD to native tracees | OpenRC, busybox and shells never saw their children exit | AcornOS | [#242](https://github.com/gmkurtzer/ginux/pull/242) |
| scratch: musl and OpenRC syscall support | musl's syscall variants, shell fd targets, `wait4(0)`, restart → `EINTR`, audit cap | AcornOS | not as is; see `docs/ginux-openrc-boot` |
| TCGETS2 on the virtual console | audit label only: unknown ioctls already pass through | LevitateOS | not needed |
| `waitid(P_PIDFD)` | systemd 258+ waits this way, no fallback: PID 1 exited at its first child | LevitateOS | [#244](https://github.com/gmkurtzer/ginux/pull/244) |
| clone3 `CLONE_INTO_CGROUP` | glibc 2.41+ `posix_spawn`: every service spawn failed (EBADF) | LevitateOS | [#245](https://github.com/gmkurtzer/ginux/pull/245) |
| `PIDFD_GET_INFO` → ENOTTY | leaked host pids; systemd lost track of every service | LevitateOS | [#246](https://github.com/gmkurtzer/ginux/pull/246) |
| release children reaped by `waitid` | process slots leaked; an instance died after 128 processes | LevitateOS | [#247](https://github.com/gmkurtzer/ginux/pull/247) |
| cgroup mkdir creates interface files | units in `/etc`, slices and templates failed at 219/CGROUP | LevitateOS | [#248](https://github.com/gmkurtzer/ginux/pull/248) |

Everything systemd-specific that LevitateOS needed comes from systemd 258+
and glibc 2.41+, which Gate 9's Rocky 10 (systemd 257, glibc 2.39) does not
use yet; Rocky will meet the same calls as it moves forward.

## Known gaps

AcornOS (`acorn/expect`): `networking` (needs `sethostname`, ENOSYS),
`sshd` (image files stat as the host owner, so "/var/empty must be owned by
root"), `dhcpcd` (no network in the instance).

LevitateOS (`levitate/expect`), by cause:

1. **Credentials.** systemd 262 mounts unit credentials with the new mount API
   (`fsopen`), which Ginux denies; nine units fail at that step, among them
   journald, tmpfiles and sysusers, and five journal units follow.
2. **dbus-broker** cannot drop capabilities (`capset`: "No such process");
   `dbus.socket` follows; dbus-daemon exits 1.
3. **homed** exits 1 (its error goes to the journal, which is down).
4. **auditd**: no kernel audit.
5. **tmp.mount**: the tmpfs mount is refused.
6. **sshd** exits 255, most likely the image-ownership gap AcornOS shows.
