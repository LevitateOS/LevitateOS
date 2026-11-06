# G18 — configurable bounded namespace; security acceptance pending

Head `f58ba480942e91ad7a0a43158a9c57496acc202a`, tree `6ed8bc3ca7a42f718b23c1b8f67c4a6a5c601d1c`, sole local prerequisite
G09d `85b53fe15a64041f451ec7e7aa1e77d08cb2c18a`. [Retained patch](patches/G18.patch). Seven files,+324/-37.
Source/ordinary preparation is retained. Security/full CI/current-main independence
remain pending. G10–G16/thread/archive ancestry is not required by this feature.

## Implementation and separate self-review

Add DomainConfig storage_max_objects/max_bytes, parsed as storage.max_objects and
storage.max_bytes. Preserve the current32-object default rather than importing the
donor's unrelated16384 default increase. Explicit object capacities accept1–1000000;
positive max_bytes is optional. Normalization emits max_objects and emits max_bytes
when configured; the zero internal sentinel means the remaining memory budget.
Public DomainConfig gains fields: this is an upstream configuration/API extension,
not a claim that every consumer's literal construction has already been checked.

The VFS gains a default InlineNamespace generic and with_namespace constructor;
new and with_storage retain their inline behavior and shared descriptor positions.
OverlayFile becomes public with private fields and an empty constructor. Custom
namespace and storage owners must service matching capacities. Runtime supplies
both from one checked object count. Enumeration, teardown and rename iteration use
actual namespace length instead of the old compile-time FILES bound. A focused
unit uses a three-slot dynamic namespace with FILES0, checks all slots/enumeration,
exhaustion, unlinked-open lifetime and final-close reuse. No abstraction-only PR is
intended; these changes travel with the configured runtime consumer.

MappedArray initializes each element before exposing slices, rejects unsuitable
alignment/zero or unrepresentable byte lengths, and unmaps its pages on Drop.
Page-rounded namespace plus backing metadata bytes are calculated with checked
arithmetic before either allocation. They are subtracted from memory_mib; explicit
max_bytes must fit the remainder. Runtime anonymous data growth is unchanged and
continues charging rounded capacities, not logical file sizes. This accounts for
owned namespace/backing bytes, not aggregate process RSS or every supervisor object.
A positive data budget below one page can hold empty names but cannot allocate a
file page. Seeded directories and removed names with live descriptions count slots.

A separate final self-review read the full final diff and current configuration
parse/validate/normalize, VFS constructors/offsets/open/allocation/whiteout/rename/
enumeration/teardown, anonymous growth and runtime setup/cleanup callers. This is
self-review, not independent reviewer or security acceptance. Core/process/native
supervisor, descriptor/task/device/dispatch owners remain identical to G09d.

- **High, current position ownership:** port the donor constructor with the current
  shared positions array. The initial static check caught its missing initializer;
  it was corrected without removing/suppressing tests or warnings. Existing shared
  dup offsets remain covered by all current VFS tests.
- **High, allocation bounds:** reject over-aligned or isize-unrepresentable mapped
  arrays before pointer/slice construction; checked conversions/product/addition/
  subtraction precede allocation. Default32 is preserved. Adversarial size/failure
  confirmation is handed off, not inferred from source review.
- **High, actual capacity and lifetime:** enumerate/clear the supplied namespace,
  preserve unlinked-open slots until final close, and test actual nonempty files.
  With5003 configured slots and three seeded directories,5000 four-byte files are
  written and individually read back. The next create fails ENOSPC; an unlinked
  still-open file keeps the namespace full until close permits a replacement.
- **Moderate, byte-budget fixture precision:** the existing16 MiB data-quota scenario
  now explicitly sets max_bytes16 MiB inside memory_mib32, accounting for metadata
  separately. It still exercises the same data ceiling/reclamation, rather than
  quietly weakening quota assertions. Default quota/growth behavior is retained.
- **Scope reconciliation:** donor restore seen-table/capacity loops stay with
  G21/G22; no archive/restore consumer exists in this base. Donor hardlink byte
  sharing assertions stay with G15/F02; importing them would create a false G15
  prerequisite. G14 cwd/G10 exec/G12 mapping accounting joins need F02 validation.
- **Blocking for publication:** adversarial arithmetic/allocator/initialization/
  namespace authority and later alias/pin interaction validation remain in the
  [DaybreakBlue handoff](daybreakblue-handoff.md). The broader security holds and
  combined feature prerequisites are not satisfied by these ordinary checks.

## Exact evidence

Source and strict clean prerequisite-plus-patch export both pass `make check
 test-unit build build-launch-directory-fixture build-launch-storage-fixture`:
91 units,32 VFS,171 matching tracked paths. The Ginux binary and both compiled
fixtures match byte-for-byte. Clean apply uses --whitespace=error. New configuration
unit checks explicit normalized round-trip and ordinary rejected bounds/duplicates;
all earlier units/default fixture/CI routes remain intact.

The5000-file namespace scenario passes on both exports, alongside growth, explicit
16 MiB data bounds, old default namespace quota and directory functional/native-cwd
controls. Directory harness modes repeat twice each. Exact G09d, using the same
compiled fixture but a temporary harness omitting unsupported max_objects, fails
open with ENOSPC(-28) and workload160 at its old32-slot bound. This measures the
runtime limit rather than merely rejecting a new config field. Baseline harness,
commands, logs and receipts are retained.

Protected/view-probes and adversarial allocation/authority checks, independent
review, full Linux runtime suite, macOS Actions/remote CI and refreshed standalone
validation remain unrun. Linux-local success is not green remote CI. Intermediate
adaptation helpers/failures are retained; candidate.patch/manifest identify final
source. No real Ginux source/index/branch/ref, remote or human text was changed.

Patch, draft, manifests, helpers, hashes and logs live under
.artifacts/ginux-contribution-preparation/G18/. F04 remains the inspection stop
before publication. G18 correctly precedes G17; no PR has been created.
