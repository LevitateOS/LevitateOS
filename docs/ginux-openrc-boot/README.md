# AcornOS under Ginux: how far OpenRC boots

Experiment, 2026-11-07. Question: how far is Ginux from booting AcornOS's
userspace (musl, busybox, OpenRC) with its real init as PID 1, the way Gate 9
boots Rocky's systemd? This is the use that would let `testing/rootfs-tests`
drop `sudo systemd-nspawn` and give AcornOS init-level tests.

**Answer: close, and the gaps are specific.** Unmodified upstream Ginux stops at
OpenRC's first shell pipe. With 78 lines of scratch changes (`scratch.patch`),
OpenRC runs `sysinit` and reaches the boot runlevel. It then hangs supervising
the first service, on a process-exit problem that needs a Ginux developer's
look.

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
  `gate9-systemd-v1`, no network. `probe.sh` replaces the getty and prints
  OpenRC's state (the files recab's OpenRC detection reads).

## What happened, step by step

| Run | Ginux | Got to | Stopped by |
| --- | --- | --- | --- |
| 1 | upstream | busybox init as PID 1, `openrc sysinit` starts | `pipe` (22) ENOSYS: every shell pipe and `$(...)` fails; getty loops on `getsid` (124) |
| 3 | + musl variants | sysinit mounts `/proc`; probe runs; `softlevel` = `default` | `lstat` (6) denied: `ls`, `rc-status` see nothing; `poll` fails |
| 4 | + `lstat`, `wait4(0)` | `/run` readable, `/run/openrc/*` present | `poll` interrupted by SIGCHLD returns `ERESTART_RESTARTBLOCK`; `restart_syscall` (219) denied |
| 5 | + `restart_syscall` passed through | "Caching service dependencies [ok]", `rc-service … status` works | `dup2(1, 3)` denied as `invalid-console-dup` (only targets 0–2 allowed) |
| 6 | + wider dup, `getgroups` | boot runlevel: "Checking local filesystems [ok]" | freeze inside a passed-through `restart_syscall` |
| 7 | restart → `EINTR` rewrite instead | same point; `poll` now returns `EINTR` and is retried | hang: `openrc-run` (fsck) waits for SIGCHLD; its child's `exit_group` never completes |

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

**2. Interrupted waits (`ERESTART_RESTARTBLOCK`).** musl's `poll` and
`nanosleep`, interrupted by a signal, return -516 and the kernel continues them
with `restart_syscall`, whose state Ginux cannot see. glibc's `ppoll` and
`clock_nanosleep` paths did not hit this under systemd. Passing
`restart_syscall` through froze the instance; rewriting -516 to `EINTR` at the
syscall-exit stop (POSIX allows it for both) worked.

**3. Shell file descriptors.** Native mode lets `dup2`/`dup3` target only fds
0–2. systemd never needs more; every shell script does (`exec 3>&1`,
`2>&1 >/dev/null`). The scratch patch allows targets below Ginux's reserved
fds (240). Whether that is safe with Ginux's descriptor tracking is a question
for upstream.

**4. Process groups.** `wait4(0, …)` (any child in my process group) is
refused (`unsupported-wait-process-group`). The scratch patch treats it as
`wait4(-1, …)`, which is right when all children share the group, as in
non-interactive scripts; a real fix tracks virtual process groups.

**5. The hang: a child's exit is never completed.** In the boot runlevel,
`openrc-run` (supervising `fsck`) forks `openrc-run.sh`, then polls its signal
pipe for SIGCHLD. The child runs its children (`grep`, `fsck`, `eend`) to
completion, calls `exit_group`, and Ginux never finishes that exit: no exit
record, no SIGCHLD, no further events. Not diagnosed; it is the next thing to
look at, with the audit log of run 7.

**Not gaps:** `socket`/`sendto` to syslog are denied by the systemd profile on
purpose; `fsopen` (430) and `sendfile` (40) are denied and busybox falls back.
Every `execve` record reads `surface=invalid result=denied` while the program
runs; that is the audit label, not a denial.

## Things to know when running this

- Ginux writes its virtual `/proc/<pid>` into the instance root on disk, and
  they outlive the instance. A second boot of the same root showed a `getty`
  from the first. Stage a fresh root for every boot, as Ginux's tests do.
- Gate 9's `prepare_root` also stages static `/proc` and `/sys` files and
  systemd-specific workarounds (dbus disabled, getty unit replaced). The
  systemd baseline is that plus Ginux; an OpenRC baseline needs its own
  equivalent (`stage.sh` is a start).
- Audit is capped at 8,192 records; a boot exceeds it. The scratch patch lifts
  the cap to see a whole boot.

## Where this leads

- An upstream Ginux gate, "Alpine/OpenRC baseline", in the shape of Gate 9:
  gaps 1–4 as concrete, testable changes, and 5 diagnosed first. Smaller than
  Gate 9 was: OpenRC is shell scripts and a few C helpers, no D-Bus, no
  cgroups.
- Then `testing/rootfs-tests` can boot AcornOS (and later LevitateOS) under
  Ginux instead of `sudo systemd-nspawn`, and recab's OpenRC detection can be
  checked against a real boot.

## Files

- `scratch.patch`: the experiment's changes to Ginux `59e2f7aa`. Scratch
  quality: it shows what is missing, it is not the fix (the audit cap and
  `wait4(0)` changes in particular).
- `stage.sh SRC ROOT [busybox|openrc-init]`: stage a root, as root.
- `instance.toml`, `probe.sh`: the config and the state probe used.
