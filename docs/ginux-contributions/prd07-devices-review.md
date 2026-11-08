# PRD07 G04 — assigned null-device extraction (local)

**No publication.** The user directs continued local extraction/integration,
with upstream Ginux taking precedence. This is a reviewed local composition on
the proposed storage chain, not a standalone PR against today's main, a merged
capability or final release acceptance.

## Exact source, dependencies and complete diff

Clean `/home/vince/levitate-development/ginux-prd07-devices` at
`c8d6a826954988530ff997cb985ea7658e5ff8b0`, tree
`2fec1a716fb957c1985716f6d8d2329eb833f4e5`, based on storage rehearsal
`16dd447e0fbdc12f702d7acaedf04f7249a27078`.
[Complete eleven-file +573/-30 diff](patches/PRD07-devices.patch).
Launcher SHA-256:
`d1bf9498a5ab4fa214753d78068b81976a437e146a94560889c483223dbc6deb`.

Commits are `5f31c708` (feature/ordinary errno corrections), `275f7a74`
(storage test contract join), and `c8d6a826` (accurate module contract).
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

G05 still owns shared public descriptors, global operator admission and aliases;
G06 owns successful exec retirement. This primitive does not replace them or
establish PRD04's final public-FD budget acceptance. Device-directory descriptors,
enumeration, mutations and general device passthrough remain unsupported here.
These limits are explicit in the source README; no full Linux ABI is claimed.

## Exact candidate validation

On `c8d6a826`: `make check`, all 141 existing units, release/common fixture builds,
launch, native supervision, process-native and setup suites pass. The full
40-case storage fixture and eight device configurations also pass. No macOS,
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
`devices-source.json`, `devices-final/receipts.json` and complete logs;
`devices-base-control/receipt.json`; `devices-errno-control/red-receipt.json`
and source snapshot; `devices/first-attempt-storage.log` and its original
receipt. Earlier receipt hashes describe their earlier runs; the final complete
run is `devices-final/`. Initial compile logs retain the port's corrected
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
