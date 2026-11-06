# G21 — startup filesystem restore

Head `33bf08a746bd64306e0375f8d419af4744d31064`, tree `0d531e213782be8ea029d3a5aebb3231633f5f62`, on G19
`a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e`. [Patch](patches/G21.patch): nine files, +482/-7.
Source and selected ordinary checks are retained; security, full CI and actual
upstream independence remain pending. G20 is not a dependency.

`--restore-file` reconstructs a fresh owned memory view before domain creation,
fork or workload startup. The donor bounded parser restores regular files,
directories, modes and GNU long names. It checks headers, total bytes, entry count,
canonical components and configured storage assignments. Existing VFS creation,
quota, write, chmod and close own the imported resources. Failure tears down the
new view and common completion cancels pending final export. The source is read
only and closed before workload execution. No existing filesystem is merged or
restored in place; exact two-block termination is the documented format.

Separate final self-review read the entire diff and current argv, startup/config,
completion/export, assignment, VFS inode/create/truncate/close/teardown callers.
This is self-review, not independent or security review.

- **False dependency removed:** donor restore checks were embedded in checkpoint
  tests. A focused restore runner now consumes ordinary G19 final export directly;
  no G20 CLI, control transport, event wait or guest channel is imported.
- **Owner adaptation:** donor lexical normalize is absent from the reviewed
  component-walker base. The parser already rejects noncanonical archive paths;
  restore uses the existing VFS owner without adding a parallel normalizer.
  The initial unresolved-import build failure is retained. The source retains
  the current 32-slot namespace, including assignment parents, rather than the
  donor's unrelated 4096-object change. G18/G22 scale joins remain explicit.
- **Cleanup:** restore errors explicitly tear down storage before startup failure.
  The read-only input has scoped close ownership. Adding O_NONBLOCK prevents a
  non-regular source blocking before fstat; no hostile FIFO/device probe ran.
- **Ordinary regression corrected:** the larger-data fixture initially assumed
  one read returns 8193 bytes and failed at the existing 4096-byte limit. It now
  loops over valid short reads and verifies EOF and all bytes. G25 remains separate.
  C indentation warnings were also fixed; initial failure logs are preserved.
- **Coverage:** the feature adds its Make target and Linux CI invocation while
  retaining static-and-unit and the existing fixture build. Donor rejection checks
  for corruption, traversal, links, duplicate entries, outside assignments and
  symlink input remain in the default runner; they were not executed locally.

Source and strict clean prerequisite-plus-patch replay pass make check test-unit
build and storage/directory/script/owned-exec fixture builds: 92 units, 34 VFS,
181 matching tracked paths. Ginux and all four fixture binaries match. Ordinary
checks reconstruct binary/empty files, modes and long paths; repeat fresh starts,
modify and re-export without changing the source, and restore 8193 bytes with a
child appearing before its parent directory. Existing seven flush/export/config,
two mapping, six ELF, 14 script and growth/bounds/quota controls pass in both copies.
Exact G19 produces the valid archive, but rejects --restore-file with usage exit2.
An initial baseline invocation lacked sudo and failed before chroot; the recorded
successful baseline rerun uses the required privilege.

Archive/path/collision/FD authority, hostile replacement, oversized input, quotas,
I/O/close failures and concurrent mutation require [DaybreakBlue](daybreakblue-handoff.md).
No such probes or cloud dispatch occurred. Independent review, full Linux suite,
macOS/remote CI and refreshed standalone validation are unrun. Symlink/hardlink,
dynamic namespace and combined checkpoint joins remain G22/F02; G07's inherited
shared-argument security limitation remains unresolved. Do not publish the local
prerequisite composition as a stack. F04 remains the user's review stop.

Patch, generic draft, commands, receipts, hashes and helpers live under
.artifacts/ginux-contribution-preparation/G21/. No real Ginux checkout, branch,
index/ref, remote text or maintainer message changed.
