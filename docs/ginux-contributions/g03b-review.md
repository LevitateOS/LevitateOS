# G03b — bounded writable VFS primitives

Local feature commit `785bfb9d6b7fcfb901d9f5efb8a845c8d9d4d53a`.
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
