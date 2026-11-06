# G05a — shared VFS descriptor offsets

Local standalone candidate: `bb2c82bf18bb30a7ebc514af5131fa5b03a5fcc5`.
Sole parent: upstream main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
One file, `crates/ginux-vfs/src/lib.rs`, +95/-21. No prerequisite proposal,
branch creation/switch, existing source-tree mutation, push or PR.

[Retained source patch](patches/G05a.patch).

## Reviewed behavior

Each independent open reserves an unused bounded position slot before any
namespace mutation. Duplicate descriptors retain that same slot. Reads/writes
advance it only after their existing successful I/O path; close/CLOEXEC leaves
it referenced while an alias remains. A reused descriptor number cannot acquire
an alias's live position; independent opens retain their own offsets. Access
rights and per-descriptor CLOEXEC behavior are unchanged.

The full final diff and VFS callers were reviewed after authoring. The capacity
proof is that a free descriptor implies at most FDS-1 live position identities;
allocation fails before mutation if no slot can be reserved. No new unsafe code,
host authority, dynamic allocation, global state, overflow-prone arithmetic or
unbounded object lifetime was introduced. Full source review is by the same agent
in a separate pass; independent reviewer coverage is not claimed.

One existing unlink test asserted that a duplicate could reread the entire old
file after the original consumed it. That assumed the bug being fixed. It now
reads the first old byte through the original, closes it, consumes the remaining
two bytes through the alias and verifies EOF; all original old/new-file lifetime
and CLOEXEC assertions remain. No upstream test/module/runner was removed.

## Evidence

Three new behavior regressions all fail at the exact upstream base: duplicate
reads, combined reads/writes, and original-close/slot-reuse/CLOEXEC. After the fix,
all15 VFS tests and all68 repository unit tests pass.

The following passed both in the original isolated candidate and a second clean
export of upstream main with only this patch applied:

- `make check test-unit`
- `make build build-launch-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch`
- `sudo -n make test-native-supervision`
- `make test-process-native`
- `sudo -n make test-gate1-setup`

All150 tracked files in the clean-applied tree match the tested candidate source;
only VFSlib.rs differs from upstream. Candidate commit/tree and source/patch hashes,
commands, timings, exact logs, concise draft body and patch are retained in
`.artifacts/ginux-contribution-preparation/G05a/`. A temporary Git index created
the local commit object; the repository's real index, refs and worktrees were
untouched. The patch also preserves the newer upstream #217/#218/#221 changes.

This candidate has passed R01–R04 locally. macOS Actions and full Rocky VM checks
have not run on it. No remote CI pass is claimed before publication. User review
of the complete package remains F04.
