# Wave 1 PR drafts (not published)

Three local branches in `/home/vince/Projects/ginux-review`, each one commit on
upstream main `ff926d0c`. Nothing is pushed. The PR body is the commit message
plus the testing note below it.

Reworked after an independent review: clearer naming (G05a), one shared syscall
constant (G03a), a simpler adoption rule plus native scenarios (G06a), and commit
messages that state the bug before the change.

---

## fix/vfs-dup-offsets: fix(vfs): share offsets between duplicated descriptors

`VirtualFs::duplicate` copied the whole descriptor, offset included, so a
descriptor and its duplicate advanced independently. dup(2) shares one
open-file description, and so one offset, between them.

Each open now gets its own description slot holding the offset. `duplicate`
shares that slot, and it is freed when the last descriptor using it closes.
Separate opens keep independent offsets, and close-on-exec stays per descriptor.

No runtime path calls `VirtualFs::duplicate` yet; this fixes the model before
configured storage starts handing out duplicated descriptors.

The existing test `unlink_retains_open_bytes_and_recreated_names_are_independent`
read the full contents through a descriptor and again through its duplicate,
which relied on the separate offsets. It now reads part through each and checks EOF.

Testing: four VFS tests fail on main (three new, plus the corrected one) and
pass here. A fifth new test guards release of the description after its last
close. `make check`, `make test-unit`, and the Linux launch-integration commands
from `check.yml` pass locally.

---

## fix/launch-mkdirat: fix(launch): use mkdirat when creating the workload directory

The launch bootstrap creates `[domain].workdir` before entering it, as the README
describes, but `launch.rs` declared its own `SYS_MKDIRAT` as 259, which is `mknodat`
on x86-64. `mknodat` created a regular file, `chdir` failed with ENOTDIR, and launch
stopped with "configured domain workdir unavailable". Any root without the workdir
was affected, including the default `/work`, and the stray file kept later launches
in that root failing.

The existing `SYS_MKDIRAT_HOST` (258) moves from `main.rs` into `supervisor.rs`,
next to the other shared host syscall numbers. Both files now use it, so the
number is declared once. The bug came in with 19e1037d.

Testing: `test-launch.py` gains a workdir that does not exist in the test root
and checks that it is created as a directory. On main that case exits 2. Here it
passes, along with `make check`, `make test-unit`, and the Linux launch-integration commands.

Not changed here: the directory is created as root before the privilege drop,
and a `mkdirat` failure is still reported as "workdir unavailable".

---

## fix/process-orphan-adoption: fix(process): adopt orphans after the last parent thread exits

When a process died, `ProcessTable` left its children pointing at the dead pid.
`getppid` reported that pid, and since only the recorded parent may wait for a
child, nobody could reap them. An orphan that exited stayed a zombie in its table
slot until the domain shut down.

When the last running thread of a process exits or is killed by a signal, its
children go to init (tgid 1), including children that already exited. While
another thread of the process still runs, it keeps its children, as on Linux.

Testing:
- Unit tests: exit and signal deaths, a child that exited before its parent,
  a fork from a non-leader thread, grandchildren keeping their living parent,
  and an init thread reaping after the init leader exited.
- New native scenarios `orphan_reparent` and `orphan_zombie`, 20 runs each.
  On main the orphan reports `getppid=2` and init's `wait4(-1)` returns ECHILD.
  Here it reports `getppid=1` and init reaps it with exit status 7.
- `make check`, `make test-unit`, and the Linux launch-integration commands pass.

Known limits:
- No virtual SIGCHLD is queued to init for a child that had already exited
  when it was adopted. Parked waits still find it, because every death
  re-scans parked waiters.
- In systemd mode, `wait4` still runs on the host, where the real parent is
  unchanged, so it returns ECHILD for an adopted child as it did before.
