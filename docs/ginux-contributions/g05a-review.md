# G05a — shared VFS descriptor offsets

Current retained candidate: `936d0eb7a9b46e58ade5219705c12d3d476dfae9`, solely on
upstream `8a475e8b2318425fd2fc15246a5c6f4bca6caf86`. See the current-main refresh
below; the earlier evidence remains historical and is not a new full-runtime pass.

Earlier standalone candidate: `bb2c82bf18bb30a7ebc514af5131fa5b03a5fcc5`.
Earlier sole parent: upstream main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`.
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

## Current-main refresh — 2026-11-06

Upstream advanced 52 commits to `8a475e8b`. The VFS file is byte-identical to the
old base, so the original one-file fix remains necessary and applies without a
prerequisite proposal. Its production/test diff remains +95/-21; no native
runtime, domain/network policy, workflow or other packet enters this revision.
All 187 tracked source paths match the recorded candidate tree in both the
tested export and an independent clean-main-plus-retained-patch replay.

The complete diff, VFS implementation, allocation/error/close paths and current
VFS caller were reread in a separate self-review. Shared position ownership,
reservation before mutation and descriptor-local CLOEXEC retain the earlier
invariants. No new blocking source finding was identified. This is self-review,
not independent reviewer or security acceptance.

All three duplicate-offset regressions fail on exact new upstream production
code with only those tests appended (exit 101). The candidate passes all 15 VFS
tests and all 76 repository unit tests. In both candidate and clean replay:

- `make check test-unit build build-launch-fixture build-process-fixtures build-gate1-fixture` passes.
- Seven selected ordinary configured-launch controls pass: exit statuses,
  argv/environment, configured/root cwd and directory enumeration.
- The existing valid unaligned `wait_status` fixture passes directly under
  `process-native`, without the default runner's inherited host-FD injection or
  fault variants.

Both exact binaries hash
`745654b931672079480724c7f14ed136b86654f96d113ec9fc049d2cdd33acd9`.
Builds use isolated target directories. The retained patch is regenerated for
the new sole parent; its earlier version, logs, source exports, strict replay,
manifest and a concise new draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G05a/`.
The local commit is unreferenced; no branch, shared index or checkout was changed.

The current upstream Makefile and workflow are preserved byte-for-byte. Their
full default Linux runtime jobs include authority/protected/fault scenarios and
remain unrun under the security handoff boundary. macOS/remote CI is also unrun.
R03 therefore retains those required checks; selected ordinary controls are not
a full CI result. This current-main source refresh does not close the overall
goal or authorize publication.

## PR231 latest-observed-base refresh — 2026-11-06

Head `25d4f05a30a2b367e24aea9460149bd55baa3406`, tree
`30ad30ad45297fead5efe7975d59adf86ee7cab0`, is directly on upstream
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a` after PR231. The complete VFS
owner is unchanged by PR231, and the candidate's VFS bytes are identical to
previous head936d0eb7. Separate source self-review reread the full +95/-21
one-file delta and existing allocation/offset/close/whiteout paths; it imports no
contract/catalog/audit changes. Independent review is not claimed.

The exactff926d0c base fails all three duplicate regressions (0 pass,3 fail).
Source and strict clean replay pass `make check test-unit build
build-launch-fixture build-process-fixtures build-gate1-fixture`:86 units,
including15 VFS tests and all new upstream contract/domain tests. Eight ordinary
controls pass per export; all196 tracked paths match the final head. Both
binaries have SHA256
`3a710e46d78023aca0a51797b0933f5a0c712a6fb1f98c2b5ae0056fd8168093`.
Exact logs/receipts/draft/manifest/retained patch are in
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G05a/`.

This refresh establishes source/ordinary preparation on the newly observed base.
Full default runtime/security suites, macOS Actions and remote CI remain pending;
new contract enforcement is not inferred from these controls. No upstream/fork
ref, real checkout/index, human PR text or publication changed.
