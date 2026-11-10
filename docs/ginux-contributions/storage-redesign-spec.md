# Configured storage redesign (replaces G03b + G03c)

This is the retained design/implementation brief. The
[consolidated production-readiness queue](../ginux-contribution-task-list.md)
owns current work: Codex now prepares and pre-reviews the contribution, and
publishing requires user permission. Refresh the historical base/dependency
SHAs below against current main before using them; preserve existing worktrees.

## Current disposition — 2026-11-08

PRD03 local preparation is complete on `9802ef036d53ae1ee5276fe59cc719389dda5879`
in `/home/vince/levitate-development/ginux-storage-prd03`. The
[exact-source review](prd03-storage-review.md) owns the final numbered-item
resolutions, complete feature/prerequisite/main diffs, tests, findings, remaining
integration dependencies and [unpublished PR draft](prd03-storage-pr-draft.md).
The retained source and original worktrees remain intact. Current main is
`53a54add`; the feature base `f1c3d3a7` composes open #232, latest #233, #238,
#243, #236, separate zero-fill and explicit catalog-2 selection. Catalog 3
explicitly adds storage's fchmodat without broadening catalogs 1/2. Exact-candidate
checks pass; PRD04/PRD05, upstream prerequisite acceptance and publication remain
open under the current queue. No new user decision is pending for preparation.

The remainder is the retained original brief; its historical base, ownership
and delivery paths do not override this disposition or the current queue.

Owner and implementer: Claude (upstream PR work). Written 2026-11-07 after G03b failed independent review.

## Why

G03b added about 430 lines of `VirtualFs` API (seek, mkdir, chmod/fchmod, metadata/fstat,
O_APPEND, O_EXCLUSIVE, descriptor flags) that nothing upstream calls; G03c is its only
consumer and uses every piece. An upstream maintainer would not take a speculative API, and
the review found design faults a real consumer should have prevented. Build them as **one
configured-storage feature**, with the VFS shaped by what launch actually mediates.

## Base

Compose exactly, in this order, onto upstream main `ff926d0c`:
1. `5e79a172` — fix(vfs): share offsets between duplicated descriptors (open as upstream #232)
2. `44b3f89c` — fix(launch): use mkdirat when creating the workload directory (open as #233)

These are the published PR heads; changes to them go through their PRs. Reuse the existing
#231 contract integration from the G03c refresh (ginux `3801fd45`) rather than redoing it.

## Required design changes (each from a verified review finding)

1. **No Linux ABI values inside ginux-vfs.** `descriptor_flags` returned Linux `O_ACCMODE`
   0/1/2 and `0o2000` while the crate's own flags are `O_READ=1`, `O_WRITE=2`, `O_APPEND=32`.
   Return the crate's own flags (or a small struct); translate to Linux in ginux-init.
   Split file status flags (F_GETFL) from descriptor flags (F_GETFD) instead of `(u32, bool)`.
2. **Typed seek.** Replace raw whence 0/1/2 with an enum. Add `VfsError::InvalidArgument`
   for bad whence, negative and overflowing results (they returned `InvalidPath`, which maps to
   the wrong errno); conversion overflow is not `FileTooLarge`.
3. **Stable inode identity.** Copy-up changed a file's inode: after `open(ro)` then a write
   through another descriptor, `fstat(ro)` and `metadata(path)` disagreed. Carry the base
   record's inode into the overlay on copy-up, and compute inode numbers in one helper used
   by both `metadata` and `fstat`. Note the existing stale read-only binding after copy-up;
   fix it if the consumer can observe it, otherwise document it and test the behavior.
4. **One coherent mode model.** mkdir masked mode bits while chmod/fchmod rejected anything
   above 0o777, and copy-up can carry 0o4755 from the image. Accept 07777 everywhere and keep
   it. Decide whether overlay modes are enforced on open; either enforce consistently with
   image files or document them as advisory. Changing an image (base) file's mode needs its
   own clear error (read-only), not `PermissionDenied`, unless copy-up is the chosen design.
5. **Overlay slot state as an enum** (free / file / whiteout / directory) in a preparatory,
   behavior-neutral commit, replacing magic 0/1/2/3.
6. **Directories.** Today each directory consumes a full file slot including `FILE_SIZE`
   data, is not counted by `overlay_files()`, cannot be removed (no rmdir) and cannot be opened.
   Implement what the launch consumer needs (check G03c's syscalls: openat O_DIRECTORY,
   fchdir, getdents, rmdir) and make every unsupported case fail with a deliberate error.
   Count capacity honestly.
7. **Status flags belong to the open-file description**, not the descriptor (O_APPEND now
   lives on `Descriptor`), consistent with #232's shared description.
8. Fix `open("/x/", O_CREAT)` returning NotFound where Linux gives EISDIR if the consumer
   can reach it; avoid hand-stripping trailing slashes in mkdir only.

Keep: O_APPEND to EOF with offset at new end; zero-length append not moving offset;
O_EXCL ignored without O_CREAT; O_CREAT|O_EXCL not following a final symlink; zero fill of
gaps past EOF (fixes stale bytes after O_TRUNC); read guard for offsets past FILE_SIZE.

## Tests

Use the file's existing conventions (`vfs` variable name, one behavior per test, assert
every return value). Cover at least: fstat(fd) == metadata(path) for base and overlay files
and across copy-up; SEEK_END on base files; two independent O_APPEND descriptions
interleaving; O_EXCL without O_CREAT; O_CREAT|O_EXCL on an existing file and directory;
mkdir with missing parent, file parent, `/`, empty path; chmod on a directory; flag accessors
on an invalid fd. Keep every upstream test and CI job. Launch-level fixtures for the configured
storage behavior, run by the commands in `.github/workflows/check.yml`.

## Commits and output

- A few coherent commits on a local branch `prep/configured-storage` in a new worktree at
  `/home/vince/Projects/ginux-storage`. Preparatory refactor first, then the feature.
- Messages in upstream style: the problem, then the change, then the tests.
  `Signed-off-by: vince <vincepaul.liem@gmail.com>`.
- Verify with `make check`, `make test-unit`, the check.yml build and launch-integration
  commands, and the storage fixture suite. Report actual results only.
- Record the result in the PR inventory: branch head, how each numbered item was handled,
  test results, open questions.

## Out of bounds

Prepared locally only; publication waits for the upstream prerequisites (#232, #233) and the
user's go-ahead. G04 and later packets are not refreshed until this redesign is reviewed.
