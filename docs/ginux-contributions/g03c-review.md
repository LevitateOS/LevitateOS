# G03c — configured ephemeral memory storage

Current retained local commit `e9c0a4d494cb9c2c2080289aa198dda42dd65b08`, based on
composition `6c35957d17c54dde9d98e5a91e49d37465d13a6d` of refreshed G03b and G03a,
rooted at observed main `8a475e8b`. The current-main refresh below owns fresh
evidence and pending checks; earlier results remain historical.

Earlier local feature commit `b36e0752caa1c91e4bbbe112c589c85915a6ce6d`.
Local base `603391c66d37915f3db2c9cec63d1c3e832216a6` composes reviewed
G03b785bfb9d (including G05a) and G03a0b563cf8 on upstream3f2fe7df.
[Retained source patch](patches/G03c.patch). Ten files,+1140/-28.

This joins the existing configured-launch/VFS owners: explicit memory assignments,
bounded supervisor-owned files/descriptors, virtual metadata and strict tracee
copies. It adds storage fixtures and CI commands while retaining every existing
unit module, target and native-supervision command. No downstream policy or
Recipe implementation enters the source patch.

## Review corrections

A separate full-diff self-review covered configuration, descriptor authority,
path classification, offsets, copy faults/partial effects, bootstrap and task
teardown, host/native fallback, documentation and CI. Independent reviewer
coverage is not claimed.

Four real cases fail on the preserved initial adapter binary (exit81/82/83/84):
file/../created wrongly creates a file; missing/../existing erases a missing
component; an image regular-file parent can be erased before a memory create;
and an image link into assigned memory exposes a physically hidden image file.
The final fixture passes all four with component-wise resolution using the
existing VFS walker. The small image lookup/canonical rewrite from G16 moves
forward because assignment isolation already requires it. G16 still owns later
owned-link/cwd/device/exec joins; none of those features is imported here.

Canonical image fallback is checked through an assignment hiding an image
symlink: /virtual/../image reads the correct image file. Native AT_EMPTY_PATH
metadata remains functional. Owned access/readlink operations never leak hidden
image objects; unsupported behavior returns an error.

Temporary native paths restore scratch bytes and original argument registers at
the syscall exit before userspace resumes. Controlled mapped pages cover full
success, overlapping output, read-only scratch and a partially writable buffer.
Partial setup failure restores exactly the changed prefix; failed restoration
terminates supervision rather than resuming with altered memory. Stat/fstatat/
statx validate supported flags, reserved mask and conflicting sync bits.

Existing storage checks additionally cover immutable host sentinels, repeated
fresh launches, unknown/duplicate/noncanonical assignments, regular data and
metadata, unlinked-file lifetime, failed truncating open at FD exhaustion, object
quota, per-file capacity, bad pointers, read-only pages and partial read copyout.
The newer upstream supervisor/core files remain byte-for-byte unchanged, including
#221's protected-copyout/fallback helpers and fixtures.

## Validation and delivery boundary

Both the candidate source and a second clean prerequisite export with only this
patch applied pass77 unit tests and:

- `make check test-unit`
- `make build build-launch-fixture build-launch-storage-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch test-launch-storage test-native-supervision test-gate1-setup`
- `make test-process-native` outside the ptrace-restricted execution sandbox

The initial sandboxed process attempt could not stop the fixture after exec;
the complete unrestricted rerun passes. An intermediate harness mode name collided
with the scratch mode and was corrected before final validation. Logs retain both
failures and corrected results. Clean replay matches all153 tracked source paths.
The source patch passes strict whitespace/application checks; existing modes and
symlinks are preserved. Complete logs, exact hashes, manifest and short draft are
in `.artifacts/ginux-contribution-preparation/G03c/`.

R01–R03 and clean composition replay pass locally. **Standalone delivery waits
for G05a/G03a/G03b upstream**, then refresh/extract/revalidate against new main;
this local composition must not be published as a stack. macOS Actions/full
Rocky VM checks are unrun. Bounds are32 objects,64KiB per file,64 descriptors and
4096-byte I/O. Owned symlinks, directory FDs, generated exec, mappings and retention
remain later work. Canonical native paths require writable scratch below the
stack; overlapping output returns EFAULT. Strict process_vm copies do not force
writes into protected pages. No real branch/index/worktree/remote was changed.

A final introductory README correction removes the obsolete blanket rejection of
storage assignments and clarifies the owned-symlink exclusion. The final object
differs from the runtime-tested5ee45634 only in README; all executable source
bytes are identical. A fresh final patch application matches all153 paths.

## Current-main refresh — 2026-11-06

The exact local prerequisite combines refreshed G03b `5fb12ab4` (including G05a)
and G03a `c28dd132`; both parent identities are recorded. Only this storage patch
is then applied. It remains ten files, +1140/-28; all added/removed source lines
match the previous corrected extraction. Current upstream native/systemd/domain/
network additions remain intact. Configuration storage still uses the same
eight bounded entries, and configured `launch` still has its own bootstrap and
supervisor; the newer native storage support does not replace this owner.

A separate self-review reread the full proposed diff and current configuration,
VFS, dispatch, bootstrap, syscall completion and failure/teardown callers.
The one-time root index is immutable after preparation; descriptor reservation
remains confined to enabled assignments, owned operations reuse the bounded VFS,
and unclaimed native operations retain the upstream contract. The shared walker
is exported without changing its implementation. Path scratch and kernel-consumed
arguments retain their existing security review requirements; ordinary success
does not prove authority or race safety. No new blocking source correction was
identified, and independent reviewer coverage is not claimed.

Both the candidate and independent clean prerequisite-plus-only-G03c export pass:

- `make check test-unit build build-launch-fixture build-launch-storage-fixture build-process-fixtures build-gate1-fixture`, including all 85 current unit tests and 24 VFS tests.
- Eight ordinary configured-launch/valid-wait controls and three ordinary storage
  controls per export. The latter exercise valid 64 KiB byte-checked I/O, relative
  creation, pread/seek, sparse/append behavior, metadata/modes/CLOEXEC, open-unlinked
  lifetime, truncation, fresh-launch repetition and bounded object exhaustion.

The exact prerequisite is separately built and rejects the same trusted valid
storage fixture with exit 2 and `launch does not support storage or device
assignments`. Candidate and clean replay execute those identical fixture bytes
successfully. The ordinary fixture is a downstream verification artifact;
existing complete Ginux fixtures and default CI remain unchanged and enabled.

All 190 tracked source paths match the candidate in both exports. Both binaries
hash `932c0cc094cc8d331a39311d0cd50c8bfc9d270ada41e29f77b8d3915af5d20b`.
Source/clean/base builds use separate target directories. Exact commands, logs,
receipts, source exports, previous/current patches, manifest and concise draft
are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G03c/`.
No real Ginux branch, shared index, checkout or remote text was changed.

Full default storage/launch/native/setup suites include authority, protected,
fault and scratch probes and remain unrun under the written security boundary.
Their required validation remains assigned downstream to DaybreakBlue; no agent
has been dispatched. macOS/remote CI is also unrun. R03 retains those checks;
R04 standalone delivery waits for actual G05a/G03a/G03b upstream merges, followed
by extraction/revalidation. Do not publish this local composition as a stack.
The broader combined F02/F03 closure remains pending at its final revisions.
