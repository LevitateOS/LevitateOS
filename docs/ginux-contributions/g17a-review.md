# G17a — owned positioned writes

Head `94baebbec5e887ae659718c3f6162d08ca267b99` on G09d `85b53fe15a64041f451ec7e7aa1e77d08cb2c18a`, tree `1bc04233b377cbd2f775d7c03915a534da997053`.
[Patch](patches/G17a.patch): four files,+49/-8. This focused part of53a2c9cc has no
namespace/mapping dependency; private backing budgets remain G17b. It is locally
prepared, with security/full CI and actual upstream independence pending.

The current descriptor table already routes syscall18 and restores its public
arguments. Add its owned-file write claim and an optional positioned offset to
the existing bounded write owner. Copy occurs before seeking; failed seek does
not mutate position. Successful or failed writes restore the previous shared
offset. VFS append semantics remain Linux-compatible. Pipes retain ESPIPE and
native descriptors retain their existing contract. No public config/VFS APIs or
process/descriptor ownership change.

A separate final self-review read the whole diff and existing dispatch/descriptor
translation, pipe errno, copy, VFS seek/write/resize/shared positions and cleanup.
This is self-review, not independent/security acceptance. Review found no new
blocking ordinary-functionality defect. The quota fixture now directly verifies
pwrite ENOSPC preserves position, length and bytes; the selected valid fixture
checks duplicate offset sharing, negative offsets, sparse growth and append.
Protected/partial copies and concurrent/raced descriptor state remain with the
[DaybreakBlue handoff](daybreakblue-handoff.md).

Source and strict clean prerequisite-plus-patch replay pass make check test-unit
build build-launch-storage-fixture build-launch-directory-fixture:89 units,31 VFS,
171 matching tracked paths and identical Ginux binaries. Ordinary positioned,
growth, byte bounds, default quota and functional/native-cwd directory controls
pass in both exports. Exact G09d returns ENOTSUP(-95) for pwrite, producing
workload163 on the new valid fixture. Existing default tests/CI remain present;
no security mode was removed or run under another name.

Patch, concise draft, commands/logs/receipts and helpers are retained under
.artifacts/ginux-contribution-preparation/G17a/. Full runtime/security, macOS and
remote CI are unrun. Linux-local results are not green CI. G17 remains partial
until private backing/public-budget preparation. No branch/checkouts/indexes,
remote PR text or maintainer message changed. Publication waits for F04 inspection.
