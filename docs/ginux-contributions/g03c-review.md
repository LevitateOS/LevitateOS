# G03c — configured ephemeral memory storage

Local feature commit `b36e0752caa1c91e4bbbe112c589c85915a6ce6d`.
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
