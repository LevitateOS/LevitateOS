# G23 — process and open-description file locks

Head `14c3332121d0f8375649d6af5ddb18fb16633e22`, tree `e6fa27f9672e5b9a2f40f26ab18e7f1f35e83e12`, directly on G15
`8ab24d4b47adeb9897623627060846070d41219d`. [Patch](patches/G23.patch): eight files, +389/-10.
Per-process source and selected ordinary validation are retained. The full G23
packet is **partial**: thread/security, actual SQLite/Cargo, full CI and refreshed
upstream independence remain pending. Ordinary fixtures do not substitute for
actual application acceptance.

The source uses existing G15 inode/description identity and G06 task/descriptor
ownership. G17 capacity/pwrite, mappings, archives, control, cwd/traversal and held
readiness policy are not code dependencies. Actual applications need their other
filesystem/process capabilities; that combined proof remains mandatory at F02/F03.
G07's thread/process-group join is held and is not inferred from per-process tests.

`F_GETLK`/`F_SETLK`/`F_SETLKW` use virtual process owners and bounded signed byte
ranges, including end-relative/current offsets, negative lengths and EOF ranges.
Records split/coalesce in a replacement table before committing, with64 ranges
and ENOLCK on exhaustion. Closing any public owned descriptor for the inode drops
that process's record locks, including dup2 retirement and committed CLOEXEC.
Death releases the process owner directly. Fork copies descriptions without
copying record ownership. A separate flock table follows each existing open
description across dup/fork and retires on its final backend close. Existing
supervisor retry/wakeup handles blocking conflicts; configured cancellation uses
the existing supported SIGKILL path. Deadlock detection, other signal interruption
and thread sharing are not newly provided. Native image locks remain denied.

Separate final self-review covered the entire diff and current descriptor refs,
close/dup2, exec transaction, death/duplicate-reap, parking/wakeup, fstat/flags/
offset, alias inode lifetime, dispatch and pointer-copy callers. This is self-
review, not independent or adversarial review. No remaining blocking source
finding was identified within the supported per-process scope; full scope is
still held as described above.

- **Owner adaptation:** donor thread-group selection is replaced by the reviewed
  per-process task ID. Entry/retry and committed exec explicitly select that
  owner; death releases by explicit ID. No donor thread transactions or pinned-
  descriptor machinery is imported into a base that has separate process tables.
- **Blocking proof strengthened:** the ordinary fixture pauses the releasing
  process until its own audit confirms the competing record/flock call parked.
  It also observes a parked child before supported SIGKILL/reap. This avoids
  treating an uncontended interleaving as blocking-lock evidence.
- **Lifecycle proof:** a child holds record/flock locks through dup and fork.
  Parent alias closes preserve the child's records; successful exec closes one
  CLOEXEC alias and removes its records while another description reference
  retains flock. Final child exit then releases flock for an independent opener.
  Split holes, virtual conflicting PID, negative lengths and child-death release
  use trusted buffers and real simultaneous child processes.
- **Coverage retained:** extend the existing task runner/fixture, already built
  and executed by Make/CI. Default full pointer/child/teardown cases remain.
  Explicit ordinary selections permit local checks without executing those
  adversarial cases; no static-and-unit coverage is removed.

Exact G15 returns unsupported for the first valid F_SETLK, causing fixture exit126.
Final source and strict clean prerequisite-plus-patch replay pass make check
 test-unit build and task/storage/directory/script/owned-exec fixture builds:
98 units,40 VFS,178 matching tracked paths. Ginux and all five fixture binaries
match. Ordinary lock/exec checks and ten existing process scenarios pass in both
copies. Four storage cases,16 scripts,eight ELF/hardlink modes and four directory/
alias/quota controls also pass. No application locks were disabled to get these
results; no new SQLite/Cargo result is claimed from older integrated logs.

Protected/mutable structs, FD/owner races, table-exhaustion rollback, authority,
thread groups and interruption require [DaybreakBlue](daybreakblue-handoff.md).
No such probes or cloud dispatch occurred. Independent review, full/default Linux
suite, macOS/remote CI, thread join, actual simultaneous SQLite/Cargo and fresh-
main standalone checks are unrun. Actual application proof remains an explicit
F02/F03 completion requirement; G24's independent timestamp/archive work may proceed.

Patch, generic draft, commands, receipts, hashes and helpers are retained under
.artifacts/ginux-contribution-preparation/G23/. No real Ginux branch, index,
checkout/ref, remote human text or maintainer message changed. F04 remains the
user inspection stop before publication.
