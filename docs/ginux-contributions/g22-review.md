# G22 — bounded archives and link identity

Head `7b2adc4f75acca26f2b356fbf9a47a9644c39bc8`, tree `6326910808a3f9c64b9cbbf158e22220705c4d5a`, on local composition
`9caccb3dd0b9d9866726c186cde04c4962152f4c`. [Patch](patches/G22.patch): seven files, +401/-77.
Source and selected ordinary verification are retained. Security, full CI and
upstream independence remain pending; no PR stack is ready for publication.

Prerequisites are reviewed G13/G15 link identity, G18 dynamic namespace, G17b
private backing budget and G21 startup restore (including G19 final export).
G17b is required by this memfd composition's 5,000-file check, not by archive
syntax alone. The local join preserves generic namespace iteration, both mapping
and link fixtures, current alias-aware metadata/read APIs, checked FD reservations
and startup teardown. Until G22, export refuses identity-losing hardlinks and
restore retains its older fixed entry bound. The exact prerequisite passes its
100-unit/build and ordinary G21 restore checks; its valid linked workload then
fails export explicitly and leaves no final/staging output.

G22 writes one regular payload and earlier-name hardlink aliases, retaining shared
inode/mode/data identity. GNU name/target records carry long links; symlink targets
remain data during import. Exact named slots replace inode arithmetic for duplicate
tracking because aliases share inodes. Dynamic bounded seen storage follows current
namespace capacity. Total archive bytes are checked against the configured data
budget plus 3072 bytes per slot and terminator; data/target bytes and actual storage
allocations retain their existing quota owner. The old fixed 2 GiB cap is removed;
ustar's supported file-size encoding still bounds each output file below 8 GiB.

Separate final self-review read the whole diff and existing archive write/sync/
close/publication, startup/config/completion teardown, VFS alias/data/metadata/
slot lifetime, shared component walker, namespace capacity, backing reservation
and restored file/parent callers. This is self-review, not independent review.

- **Metadata correction:** donor restore accepted arbitrary low symlink mode
  bits, while VFS creation always installs0777. Reject another mode explicitly
  rather than silently changing requested metadata. The default symlink-target
  rejection fixture now uses its supported mode. Adversarial metadata validation
  is written down for DaybreakBlue, not claimed from ordinary results.
- **Identity:** export picks a prior live name for each shared inode. The first
  surviving alias becomes payload when its original name was unlinked. Restore
  accepts only prior exact regular names with matching mode and assignment;
  parents must be actual directories. No new inode or path owner is introduced.
- **Retained lifecycle:** G19 private staging/no-overwrite publication and G21
  pre-workload restore cleanup are unchanged. Control/event wait, thread groups,
  cwd/traversal extensions and held guest/network policy are not imported. Core,
  process, native supervisor, descriptors, mapping and backing source are unchanged
  from the explicit prerequisite. The shared VFS walker is unchanged.
- **Coverage:** extend G21's existing restore target and its existing Linux CI
  invocation; no deferred G31 wiring is needed. Existing static-and-unit tests,
  mapping/link fixtures and full negative defaults are retained.

Final source and strict clean prerequisite-plus-patch replay pass make check
test-unit build and storage/directory/script/owned-exec/FD fixture builds:100 units,
41 VFS,181 matching tracked paths. Ginux and all five fixture binaries match.
Ordinary checks pass in both copies: regular startup reconstruction/restart,
permissions, empty/binary/multi-chunk files, parent ordering, link/inode/nlink
coherence after mutation, long hardlink/symlink targets, and an unlinked original
with its surviving alias. A 5,000-nonempty-file archive restores and verifies data
under its configured namespace. A complete2 GiB+1 source payload is restored and
re-exported under an explicit4 GiB memory budget; length, boundary bytes and a
streaming SHA256 over every exported payload byte match the expected content.
Seven flush/export/config cases, two mapping, ordinary symlink, six ELF,14 script
and growth/bounds/quota controls also pass. Earlier results before metadata review
and before the final digest fixture are labelled separately from the final head.

Full negative/private-FD/hostile archive/path/alias/race/failure validation remains
with [DaybreakBlue](daybreakblue-handoff.md); no cloud dispatch or security
acceptance occurred. Independent review, full Linux suite, macOS/remote CI and
fresh-main standalone validation are unrun. G07 shared-argument security and
G20/G14/G16 cross-candidate joins remain F02 work. Linux-local success is not green CI.

Patch, concise draft, commands, hashes, helpers, logs and a complete prerequisite
reconstruction patch from recorded main3f2fe7df are retained under
.artifacts/ginux-contribution-preparation/G22/. No real Ginux branch, index,
checkout/ref, remote human text or maintainer message changed. F04 remains the
user inspection stop before publication.
