# G03b — bounded writable VFS primitives

Current retained local commit `095f89c5bfb2f8d07404f2304a5088f709ef7652` on refreshed G05a
`25d4f05a30a2b367e24aea9460149bd55baa3406`, rooted at observed main `ff926d0c`.
The current-main refresh below owns fresh evidence and pending checks; earlier
results remain historical.

Earlier local feature commit `785bfb9d6b7fcfb901d9f5efb8a845c8d9d4d53a`.
Explicit local prerequisite: G05a `bb2c82bf18bb30a7ebc514af5131fa5b03a5fcc5`.
Underlying upstream baseline: `3f2fe7df`. One file, +425/-6 relative to G05a.
[Retained source patch](patches/G03b.patch).

This adds the primitive VFS API needed by the configured-storage adapter: virtual
inode/type/mode/size metadata, owned directories, chmod/fchmod, seek, append and
exclusive create, and descriptor flag access. It keeps fixed object/file bounds,
uses the existing component resolver and preserves every current upstream VFS
test. It adds9 tests; none are removed or weakened. Existing G05a alias tests also
remain intact. Generic metadata/layout docs stay with their defining types.

## Review corrections and evidence

The whole final diff and existing lookup/open/read/write/unlink/copy-up callers
were reviewed after extraction. The original222d29ff/10091e4e feature normalized
paths lexically, followed final links during exclusive creation, and used separate
descriptor seek offsets. Its older whiteout allocation also did not reuse the
slot once the old object closed. Four actual review cases fail on exact donor
10091e4e (9pass/4fail); those cases and the complete current24-test VFS suite pass
on the corrected extraction. The original helper is superseded, not replayed.

- Component-wise lookup preserves upstream #218 link-before-parent behavior and
  rejects file/../file, file/. and missing/../name without mutating metadata.
- mkdir and exclusive create inspect the final entry without following it;
  existing looping/final image links fail as already present. mkdir retains
  trailing-slash support. Directory open/unlink through file APIs stays denied.
- Seek uses G05a's shared position. Negative/overflow/unknown-origin failures do
  not change it; EOF reads and empty writes remain safe beyond file capacity.
  Append chooses current EOF, sparse holes are zeroed, and truncate cannot expose
  discarded bytes through later sparse writes.
- Metadata uses virtual IDs. Descriptor-local CLOEXEC stays distinct from shared
  offsets and inode modes. Permission updates on image objects or unsupported
  mode bits are rejected before mutation; existing logical owned-mode semantics
  do not claim full Linux ownership/DAC fidelity.
- mkdir reuses the existing safe create/whiteout owner from #217. A still-open
  unlinked file retains its bytes and inode while the same name becomes a distinct
  directory; capacity failure leaves no partial directory behind.

The feature introduces no unsafe code, host path/FD authority, persistence format
or execution fallback. Object/file capacities remain explicit. Review is a
separate pass by the author; independent reviewer coverage is not claimed.

## Validation and delivery boundary

Both the isolated local composition and a second clean G05a export with only the
G03b patch applied pass77 repository unit tests, including24 VFS tests, plus:

- `make check test-unit`
- `make build build-launch-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch`
- `sudo -n make test-native-supervision`
- `make test-process-native`
- `sudo -n make test-gate1-setup`

All150 tracked paths in the clean-applied source match the tested composition.
Patch/head/tree hashes, failures, commands, timings, full logs and concise draft
are under `.artifacts/ginux-contribution-preparation/G03b/`.

R01–R03 pass on the stated local prerequisite. R04's clean composition proof
passes, but **standalone delivery is blocked until G05a is upstream**. This is
not a PR against current main and must not be published as a stack. After that
merge, refresh this isolated feature on main and repeat affected checks before
publication. macOS Actions/full Rocky VM checks are unrun. The broader G03 storage
adapter remains pending. No real Git index/ref/worktree/remote was modified.

## Current-main refresh — 2026-11-06

The retained patch still changes only `crates/ginux-vfs/src/lib.rs`, +425/-6
relative to refreshed G05a. Its full source delta is byte-identical to the
previous corrected extraction. All current upstream VFS tests and the three
G05a offset tests remain, along with nine capability/boundary tests. The new
native/domain/network code and CI wiring are unchanged.

A separate self-review reread the complete diff, component resolver,
lookup/open/read/write/unlink/copy-up owners and existing `vfs_main` caller.
Directory creation reserves the existing bounded overlay owner; metadata uses
virtual IDs, seek updates G05a's shared position only after validation, append
uses current EOF and sparse writes zero holes. Exclusive creation checks the
final entry without following links. Mode updates and CLOEXEC use their existing
object/descriptor owners. No new blocking source correction was identified;
independent reviewer or security acceptance is not claimed.

Both source and independent clean G05a-plus-only-G03b replay pass:

- `make check test-unit build build-launch-fixture build-process-fixtures build-gate1-fixture`, including all 85 current unit tests and 24 VFS tests.
- Eight selected ordinary configured-launch/valid-wait controls per export.

The nine added VFS cases demonstrate directories/metadata, sparse/append I/O,
shared alias seek, final-link exclusive creation, capacity failure without
mutation, unlinked-object lifetime, mode/CLOEXEC separation and discarded-byte
handling. These are library capability evidence; the configured runtime adapter
is a separate G03c packet and is not established by these checks.

All 187 tracked paths match the retained commit in source and clean exports.
Both binaries hash `fd6d6987625672f8d2e7c8a3619459bab6428f9d5cc431e187c65e8f97b67706`.
Exact commands, logs, receipts, earlier/new patches, source exports, manifest and
concise draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G03b/`.
Builds use separate target directories and no real branch/index/checkout or remote
text was changed.

Standalone delivery remains blocked until G05a actually merges upstream; this
local composition must not be published as a stack. Full default Linux runtime
jobs contain authority/protected/fault cases and remain unrun under the written
security boundary. macOS/remote CI also remains unrun. R03/R04 keep those
acceptance and prerequisite requirements pending; ordinary success is not full
CI, security acceptance or publication authorization.

## PR231 latest-observed-base refresh — 2026-11-06

Head `095f89c5bfb2f8d07404f2304a5088f709ef7652`, tree
`71bc806e4550a92c1f63b10e3b130ac9690d191d`, is on refreshed G05a
`25d4f05a30a2b367e24aea9460149bd55baa3406`, rooted at observed upstream
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a`. Complete VFS-file bytes match
previous corrected head5fb12ab4; scope remains one file,+425/-6. A separate
self-review reread the whole delta, all nine added tests and existing resolution,
open/read/write, copy-up, whiteout allocation/reuse and metadata callers. All
pre-existing VFS tests are retained. Upstream contract/domain/native/CI additions
are unchanged. Independent reviewer coverage is not claimed.

Source and strict clean G05a-plus-only-G03b replay pass make check/test-unit/build
and launch/process/setup fixture builds:95 units, including24 VFS tests. Nine
feature/boundary cases cover sparse/append/empty I/O, seek aliases and failure
preservation, metadata/CLOEXEC ownership, final-link exclusive creation, capacity
and unlinked lifetime. Eight ordinary configured-launch/valid wait-status controls
pass per export. All196 tracked paths match the head; both binaries have SHA256
`ce0e0a895dfebe39ff9a5382ca39604ea32e3be57a01635782804a36eb65ee5a`.
Exact source/clean exports, logs, receipts, patch, draft and manifest are in
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G03b/`.

This remains local-prerequisite source/ordinary preparation: actual G05a upstream
merge, full default runtime/security, macOS and remote CI are pending. Configured
storage and upstream contract-policy joins belong to G03c and are not established
by these library checks. No branch, real checkout/index, remote text or publication
changed.
