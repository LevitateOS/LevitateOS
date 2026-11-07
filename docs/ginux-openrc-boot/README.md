# AcornOS under Ginux: how far OpenRC boots

Experiment, 2026-11-07. It is now a repeatable check that covers LevitateOS
too: [testing/ginux-boot](../../testing/ginux-boot/README.md), with the
staging and config that were here. Question: how far is Ginux from booting AcornOS's
userspace (musl, busybox, OpenRC) with its real init as PID 1, the way Gate 9
boots Rocky's systemd? This is the use that would let `testing/rootfs-tests`
drop `sudo systemd-nspawn` and give AcornOS init-level tests.

**Answer: close.** Unmodified upstream Ginux stops at OpenRC's first shell
pipe. With 86 lines of scratch changes (`scratch.patch`), OpenRC runs all
three runlevels: sysinit, boot (fsck, remount, localmount) and default
(`dhcpcd` starts, `sshd` generates host keys). `rc-status` and
`/run/openrc/{started,failed,…}` report real state, including `dhcpcd` as
crashed (there is no network to keep it up). Three services still fail, each
for a stated reason: `hostname`, `networking` (needs hostname) and `sshd`.

## Setup

- Ginux upstream `59e2f7aa` (main after #235), built with
  `cargo build -p ginux-init --target x86_64-unknown-linux-gnu --release`.
- Host: Rocky Linux 10.2, the host Ginux targets. Launched as Ginux's own tests
  do: `sudo ginux native instance.toml ROOT`.
- Root: Alpine 3.23 packages for AcornOS, installed offline by `apk.static`
  from the pinned ISO plus the pinned extra apks (124 packages; see builder
  `206e875`). `stage.sh` applies the generic parts of Ginux's `prepare_root`
  and AcornOS's own init setup: busybox `/sbin/init` with the inittab
  `distro/builder/src/pipeline/overlay.rs` writes, `networking` in boot, `sshd`
  and `dhcpcd` in default.
- `instance.toml`: Gate 9's shape with `init = "/sbin/init"`, contract profile
  `gate9-systemd-v1`, no network; `instance-network.toml` adds Gate 9's shared
  network (same result). `probe.sh` replaces the getty and prints OpenRC's
  state, which is what recab's OpenRC detection reads.

## Runs

| Run | Ginux | Got to | Stopped by |
| --- | --- | --- | --- |
| 1 | upstream | busybox init as PID 1, `openrc sysinit` starts | `pipe` (22) ENOSYS: every shell pipe and `$(...)` fails; getty loops on `getsid` (124) |
| 3 | + musl variants | sysinit mounts `/proc`; probe runs; `softlevel` = `default` | `lstat` (6) denied: `ls`, `rc-status` see nothing; `poll` fails |
| 4 | + `lstat`, `wait4(0)` | `/run` readable, `/run/openrc/*` present | `poll` interrupted by SIGCHLD returns `ERESTART_RESTARTBLOCK`; `restart_syscall` (219) denied |
| 5 | + `restart_syscall` passed through | "Caching service dependencies [ok]", `rc-service … status` works | `dup2(1, 3)` denied as `invalid-console-dup` (only targets 0–2 allowed) |
| 6 | + wider dup, `getgroups` | boot runlevel: "Checking local filesystems [ok]" | freeze inside a passed-through `restart_syscall` |
| 7 | restart → `EINTR` rewrite instead | same point; `poll` now returns `EINTR` and is retried | hang: `openrc-run` (fsck) never sees its child exit |
| 9 | + deliver SIGCHLD | all three runlevels; `dhcpcd` started, then crashed; `sshd` keys generated | `hostname`, `networking`, `sshd` fail (below) |
| 10–11 | + shared network; `/proc/filesystems` staged | same | same; ownership confirmed |

## The gaps, by kind

**1. musl's syscall choices (cheap; same semantics as existing ones).** glibc
and musl reach the same kernel features through different syscalls, and Gate 9
only met glibc's:

| musl | glibc equivalent Ginux already supports |
| --- | --- |
| `pipe` (22) | `pipe2` (293) |
| `nanosleep` (35) | `clock_nanosleep` (230) |
| `lstat` (6) | `newfstatat` (262) |
| `faccessat` (269) | `faccessat2` (439), `access` (21) |
| `getsid` (124), `getgroups` (115) | identity emulation (`getpgrp`) |

Each needs a catalog entry, a decision, a surface and, for `pipe`, the
`pipe2` exit handling. That is most of `scratch.patch`.

**2. SIGCHLD is dropped in native mode (the hang).** The native supervisor
resumes a tracee stopped for SIGCHLD with signal 0 (`if signal == 17`, from
`e595fc28` "Supervise systemd process trees"), so the signal is discarded.
systemd never notices: it blocks SIGCHLD and reads a signalfd, and a blocked
signal never produces a delivery stop. OpenRC, busybox and shells catch
SIGCHLD with a handler; `openrc-run` polls a pipe its handler writes to, so
the dropped signal left it waiting forever on a child that was already a
zombie. Found live on the host: the child `[openrc-run.sh] <defunct>`, its
parent in `poll` with SIGCHLD caught, not blocked and not pending, and the
tracer already detached from the zombie. Ginux's configured-process path
already delivers SIGCHLD ("resume carrying the host SIGCHLD so delivery is
observed"); delivering it the same way in native mode fixed the boot.
Raised upstream as
[gmkurtzer/ginux#242](https://github.com/gmkurtzer/ginux/pull/242), with a
`test-native-supervision` scenario that fails on `59e2f7aa`.

**3. Interrupted waits (`ERESTART_RESTARTBLOCK`).** musl's `poll` and
`nanosleep`, interrupted by a signal, return -516 and the kernel continues them
with `restart_syscall`, whose state Ginux cannot see. glibc's `ppoll` and
`clock_nanosleep` paths did not hit this under systemd. Passing
`restart_syscall` through froze the instance; rewriting -516 to `EINTR` at the
syscall-exit stop (POSIX allows it for both) worked.

**4. Shell file descriptors.** Native mode lets `dup2`/`dup3` target only fds
0–2. systemd never needs more; every shell script does (`exec 3>&1`,
`2>&1 >/dev/null`). The scratch patch allows targets below Ginux's reserved
fds (240). Whether that is safe with Ginux's descriptor tracking is a question
for upstream.

**5. Process groups.** `wait4(0, …)` (any child in my process group) is
refused (`unsupported-wait-process-group`). The scratch patch treats it as
`wait4(-1, …)`, which is right when all children share the group, as in
non-interactive scripts; a real fix tracks virtual process groups.

**6. Image file ownership is not virtualized.** Inside the instance a process
is `uid=0(root)`, but every image file stats as `1000:1000`: the staging step
(Ginux's `prepare_root` does the same) chowns the root to the unprivileged
host user, and `stat` reports that uid unchanged. Anything that checks
ownership breaks: `sshd` refuses to start ("/var/empty must be owned by
root"), and `doas.conf` and `authorized_keys` checks would fail the same way.
Ginux projects virtual uid/gid 0 for files it owns (writes), not for image
files; projecting the instance owner to virtual root in `stat` results would
close this. Not patched.

**7. Hostname and `/run`.** `sethostname` is ENOSYS (Gate 8 made the hostname
read-only), so OpenRC's `hostname` service fails and `networking`, which
needs it, is never started. Emulating `sethostname` into the instance's
virtual hostname (which Ginux already serves) would fix both. OpenRC's
`tmpfs` mount on `/run` is refused ("permission denied"); OpenRC carries on
because `/run` exists in the root. Neither is patched.

**Staging, not Ginux:** OpenRC reads `/proc/filesystems`, which Gate 9's
staging never creates; `stage.sh` now writes it.

**Not gaps:** `socket`/`sendto` to syslog are denied by the systemd profile on
purpose; `fsopen` (430) and `sendfile` (40) are denied and busybox falls back;
GNU grep's `mincore`/`sigaltstack` (stack-overflow detection) are denied and it
falls back. Every `execve` record reads `surface=invalid result=denied` while
the program runs; that is the audit label, not a denial.

## Things to know when running this

- Ginux writes its virtual `/proc/<pid>` into the instance root on disk, and
  they outlive the instance. A second boot of the same root showed a `getty`
  from the first. Stage a fresh root for every boot, as Ginux's tests do.
- Gate 9's `prepare_root` also stages static `/proc` and `/sys` files and
  systemd-specific workarounds (dbus disabled, getty unit replaced). The
  systemd baseline is that plus Ginux; an OpenRC baseline needs its own
  equivalent (`stage.sh` is a start).
- The instance's stderr goes to the same stream as the audit records; service
  errors ("ERROR: sshd failed to start") are there, not on the console.
- Audit is capped at 8,192 records; a boot exceeds it. The scratch patch lifts
  the cap to see a whole boot.

## Where this leads

- An upstream Ginux gate, "Alpine/OpenRC baseline", in the shape of Gate 9:
  gaps 1–7 as concrete, testable changes. Gap 2 (SIGCHLD) is a one-line
  behavior change with a clear reproduction and is worth raising on its own:
  it affects any init or supervisor that catches SIGCHLD, not only OpenRC.
- Then `testing/rootfs-tests` can boot AcornOS (and later LevitateOS) under
  Ginux instead of `sudo systemd-nspawn`, and recab's OpenRC detection can be
  checked against a real boot: this experiment already produced the
  `/run/openrc` state and `rc-status --crashed` output it reads.

## Files

- `scratch.patch`: the experiment's changes to Ginux `59e2f7aa`. Scratch
  quality: it shows what is missing, it is not the fix (the audit cap,
  `wait4(0)` and dup-target changes in particular).
- `instance-network.toml`, `probe.sh`: the shared-network config and the
  state probe used here. The staging and `instance.toml` moved to
  `testing/ginux-boot/acorn/`; staging now chowns the root to the host
  nobody user, as upstream Ginux runs root-launched workloads since `39731fc2`.
