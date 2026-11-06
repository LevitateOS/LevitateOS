# G30 — preserve virtual root ownership on assigned storage

Head `2186548d5af38b6ab06379a38b9f8fc7a08636bc`, tree `4e4ab30c24f1cc5f8cdaf7c360f5c353d9e23f5c`, directly on G15
`8ab24d4b47adeb9897623627060846070d41219d`. [Patch](patches/G30.patch):5 files,+88/-7,
adapted from7cf2638a. No G25/G24/G14/G07/network/archive ancestry is needed for
these runtime operations or the demonstrated cpio consumer. G15 already supplies
public descriptors, inode aliases, directories and shared symlink/path resolution.
This is local preparation; upstream prerequisites/security/full CI remain open.

Existing assigned-storage metadata projects uid/gid0. Chown/fchown/lchown/fchownat
now verify the addressed object exists and accept each32-bit ID as0 or unchanged
(-1), returning EOPNOTSUPP for a different identity. These are identity-preserving
operations, not arbitrary Linux ownership changes. There is no host chown call,
new stored identity field or separate inode/path owner. Native fchown stays EPERM;
unassigned path operations retain the immutable default contract. Nofollow and
AT_EMPTY_PATH use existing metadata/descriptor dispatch, including relative
owned directory handles and aliases that outlive their final pathname.

Separate full-diff self-review read all5 changes, existing path walker, dispatch
precedence, public FD translation/close, VFS metadata/fstat/alias lifetime, ABI
metadata serialization, default contract, storage runner and existing CI target.
This is self-review, not independent or adversarial review.

- **Medium, reproduced/corrected — empty-path dispatch:** the first port returned
  ENOENT for valid fchownat(fd,"",...,AT_EMPTY_PATH); the device owner's shared path
  lookup ran before filesystem descriptor dispatch. request_path now leaves that
  operation to the existing descriptor owner. The ordinary fixture failed at106
  before this correction and passes after it. Invalid flags are checked before
  nonempty-path resolution, and descriptor dispatch retains its own flag check.
- **Dependency correction:** source tracing and actual cpio prove no G25 vectors,
  G24 timestamps or G14 owned cwd dependency for root-preserving ownership. The
  actual combined package pipeline still joins those owners at F02/F03; no
  no-preserve-owner flag or extraction bypass was introduced.
- **Coverage preserved:** existing units and storage target remain. The runner's
  default now includes the full ownership scenario, with separate ordinary mode
  for valid calls. Donor dangling/absent/unsupported-ID/invalid-pointer/flag/native
  checks remain enabled and unrun locally for DaybreakBlue. Existing CI invokes
  this updated runner, so no extra wiring-only packet is required.

Source and strict clean base-plus-patch replay pass make check test-unit build
build-launch-storage-fixture:98 units including40 VFS,177 tracked paths matched,
identical Ginux/fixture hashes. Ordinary ownership checks exercise root/unchanged
IDs by path and FD, nofollow symlink, relative directory FD and AT_EMPTY_PATH,
stat/statx uid/gid0, preserved modes/bytes/link counts and unlinked open aliases.
Existing ordinary symlink/growth/bounds/quota controls also pass in both copies.

Actual trusted GNU cpio2.15 runs normally in a supervised child and extracts a
locally produced valid newc archive with root UID/GID. Its arguments are -i
--quiet; ownership preservation is not suppressed. Parent checks every8229
payload byte through file/hardlink/symlink names, shared inode/nlink2, mode0640,
uid/gid0, symlink type/ownership and EOF. Source and clean replay both exit0.
The same frozen archive and wrapper on exact G15 produce fchown/lchown ENOSYS,
cpio ownership errors and wrapper exit83. Archive/wrapper/binary/runtime file
hashes and provenance are retained. No host packages or external downloads were
needed. The initial harness mistakenly included work/ in names despite the
configured default cwd already being /work; that failure is retained and the
corrected relative archive uses the existing supported cwd behavior.

Initial local port formatting was attempted from /tmp, and its first check
caught a missing helper insertion; neither is an upstream defect or a final
check. Corrected source and exact replay pass the checks above.

[DaybreakBlue](daybreakblue-handoff.md) must validate protected/mutable paths,
invalid UID/GID/flags and missing/dangling targets, native/private descriptor and
assignment authority, nofollow/rename/close races, shared-thread transactions,
copy/lookup errors and host ownership nonmutation under adversarial conditions.
Full/default storage runtime, Linux/macOS remote CI, independent review, G07's
shared-path hold and fresh upstream independence remain pending. This focused
cpio evidence does not complete APK/RPM/Recipe integration at F02/F03.

Patch/draft/helpers/logs/results/failures/provenance are retained under
.artifacts/ginux-contribution-preparation/G30/. No Ginux branches/refs/real index/
checkouts, remote text or publication changed. F04 remains the user inspection
stop before publication.
