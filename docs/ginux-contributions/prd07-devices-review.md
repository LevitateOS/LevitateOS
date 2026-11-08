# PRD07 G04 — assigned null-device extraction (local)

**No publication.** The user directs continued local extraction/integration,
with upstream Ginux taking precedence. This is a reviewed local composition on
the proposed storage chain, not a standalone PR against today's main, a merged
capability or final release acceptance.

## Exact source, dependencies and complete diff

Clean `/home/vince/levitate-development/ginux-prd07-devices` at
`0bd9b4bb8970946080b98672064de20fb3aebc11`, tree
`fb2487874dd22be12f0fc4756c8bd55d7bc492e4`, based on storage rehearsal
`16dd447e0fbdc12f702d7acaedf04f7249a27078`.
[Complete eleven-file +608/-30 diff](patches/PRD07-devices.patch).
Launcher SHA-256:
`97886511b11cda66cdf0a30ed53aa78d072260b36d75b31526b824cf7b9a18f4`.

Commits are `5f31c708` (feature/ordinary errno corrections), `275f7a74`
(storage test contract join), `c8d6a826` (accurate module contract), and
`0bd9b4bb` (virtual-device traversal rewrite and regression).
The base remains the [local storage prerequisite composition](prd07-storage-rehearsal.md).
Current main still reads `53a54add`; fresh read-only inspection on 2026-11-08
finds #232/#233/#234/#236/#238/#243 open at their original reviewed heads,
with unchanged maintainer comments requesting no additional changes for now.
Actual merged-source refresh and upstream/equivalent disposition remain required.

## Complete donor and owner disposition

The retained G04 donor is `8880ddc8`. Its device registry adapter, all original
device fixture assertions, independent runner and Make/CI wiring are retained
and adapted from Domain to Instance terminology. The old `fs_hooks` and
`path_hooks` join is **replaced** by the PRD03 `LaunchFs`/`launch_paths` owner:
one component-ordered namespace walk selects memory, virtual device or image.
The old separate `tracee_io` copy/path implementation is **replaced** by the
existing protected-copy/path helper. Stat/statx serialization is shared, keeping
memory-directory link counts while adding virtual rdev metadata. No useful
protection, directory or metadata test is removed.

The old catalog-1 `faccessat` addition and native/catalog test exception are
**replaced** by the explicit catalog-2 operation already in the base. No
catalog-1 inventory or native core changes are imported. Classic faccessat
decoding joins the existing path owner for memory/device access. The new device
audit lane is selected only by configured dispatch; native classification and
balanced policy are unchanged. A direct core/contract diff against the base is
empty, and the original native suites still pass.

## Contract and failure-path review

The virtual `/dev` exists independently of memory assignments and is empty
unless `null` is explicitly assigned. Unknown/duplicate assignments fail before
the workload. Registry handles and per-open flags are bounded to 64; reads
return EOF, writes discard at most 4,096 bytes without touching the ignored
guest buffer. Access-mode, close, seek, flags and stat/statx operations use the
selected virtual object. Metadata copying honors guest page protections.

Image links use the same resolver as storage, including search checks, `..`
ordering and missing/non-directory components. Hidden image device nodes have
no native fallback. Root/image lookups work without a memory assignment; the
optional memory backend is unwrapped only for assigned memory paths. Device
and memory handle ranges stay separate, so device I/O is not misclassified as
regular-file I/O. Device-only configurations also reserve the native namespace
without increasing an inherited limit.

Review fixed empty stat-path handling without AT_EMPTY_PATH (ENOENT), null
O_DIRECTORY opens (ENOTDIR) and readlink/readlinkat of a non-link (EINVAL).
The added ordinary empty-path oracle fails on the first prepared binary at
guest code 73, then passes after correction; exact source files, binary,
fixture and log hashes are retained. The pre-feature storage binary separately
fails the unassigned-device view oracle at code 12; its baseline is pinned.

The first complete storage run exposed an intentional contract conflict: its
old native-image `/dev/null` success assertion no longer matches G04's already
retained default-hidden device view. **Replaced** that assertion with ENOENT;
assigned-null success, IO, metadata and close remain covered in the complete
device fixture. All other original storage checks stay, including native
directory-FD resolution, private-directory metadata, `/..`, modes, bounds and
partial copies. The failing old assertion/log is retained; it was not suppressed
or counted as a pass.

Further source review found a real namespace mismatch in `c8d6a826`: after
walking virtual `/dev/..`, an image-bound syscall could still receive its raw
path. An image `/dev` symlink then selected a different file, and a regular
image `/dev` could cause ENOTDIR. The new symlink control fails on that pinned
binary at guest code 78 (the wrong file's bytes). `0bd9b4bb` marks device
traversal as requiring a canonical rewrite, using the existing path owner.
Both image forms now select `/image` and preserve all image sentinels. The old
binary, fixture/runner identities and red receipt remain recorded; no earlier
green baseline is presented as verification of this later finding.

G05 still owns shared public descriptors, global operator admission and aliases;
G06 owns successful exec retirement. This primitive does not replace them or
establish PRD04's final public-FD budget acceptance. Device-directory descriptors,
enumeration, mutations and general device passthrough remain unsupported here.
These limits are explicit in the source README; no full Linux ABI is claimed.

## Exact candidate validation

On `0bd9b4bb`: `make check`, all 141 existing units, release/common fixture builds,
launch, native supervision, process-native and setup suites pass. The full
40-case storage fixture and ten device configurations also pass. No macOS,
Recipe consumer or final-source Daybreak claim is made for this composition.

Run from the worktree:

```sh
git rev-parse HEAD
git status --porcelain
make check
make test-unit
make build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-process-fixtures build-gate1-fixture
sudo make test-launch
sudo make test-launch-storage
sudo make test-launch-devices
sudo make test-native-supervision
make test-process-native
sudo make test-gate1-setup
```

Evidence under `.artifacts/ginux-contribution-preparation/prd07/`:
`devices-source.json`, `devices-parent/receipts.json` and complete logs;
`devices-parent-control/red-receipt.json` and exact runner/fixture;
`devices-base-control/receipt.json`; `devices-errno-control/red-receipt.json`
and source snapshot; `devices/first-attempt-storage.log` and its original
receipt. Earlier receipt hashes describe their earlier runs; `devices-final/`
is the old `c8d6a826` baseline, preserved with `devices-c8d6-source.json`.
The final complete run is `devices-parent/`. Initial compile logs retain the port's corrected
exhaustive audit-arm/decoder placement errors. No privileged adversarial campaign
or upstream write was performed in this extraction.

## Retained PR draft — do not submit

Title: **feat(launch): expose assigned null devices through the composed namespace**

Configured launch gains an immutable virtual `/dev`, empty by default, with
explicit `null` assignment through `ginux-devices`. Memory, device and image
paths share one resolver so image aliases cannot expose hidden device nodes.
Bounded virtual IO and metadata use no host device descriptor; the contract
gate still runs before effects. Catalog 1 and native policy remain unchanged.

The full device fixture covers assignment denial, real virtual null IO,
metadata/protected copies, exhaustion, aliases and fresh supervisors. Existing
storage expectations are reconciled with the default-hidden view while keeping
every other assertion. Local Linux CI, 141 units, full storage and device suites
pass on the pinned revision. This draft must be rebased/revalidated after its
actual storage prerequisites land; the user has prohibited PR publication.
