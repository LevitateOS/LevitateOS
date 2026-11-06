# G09b — bounded growable configured storage

Current retained local head `a2a70d1c7811886d2fca20d228a94e7dc6b31a61`, based on
composition `ff29e8574f1382355bd4b925bf2aaf634885c305` of refreshed G06b and G09a,
rooted at observed main `8a475e8b`. The current-main refresh below owns fresh
evidence and pending checks; earlier results remain historical.

Earlier head `297ad8dde26e02ca32d6becd60d654c1fd67ecd0`, tree `17008873b421921585a93748a341a9f0cb1f21f3`, on composition `ccd42e319c3a76bf85d9cfce81a44cc9ba051911`
(G06b plus G09a). [Retained patch](patches/G09b.patch). Seven files,+281/-13.
This is local preparation, not standalone-ready on main. Review G09a/G09b
separately for evidence, but plan one useful growable-storage contribution
including its VFS owner; do not create an abstraction-only PR or publish a stack.

## Change and review

Use the existing VFS storage seam with supervisor-only mmap/mremap bytes. Files
share a bound derived from resources.memory_mib; this is mapped file capacity,
including page rounding/growth reserve, not an aggregate RAM quota. Preserve32
metadata objects,64 backend opens and the current public FD budget. No thread,
poll, directory, owned-exec, retained storage or mapping API was imported. Move
32e2c85f's memfd identity to G12, its actual mapping consumer; anonymous mappings
suffice here. Source allocation arithmetic converts/checks config size explicitly.

A separate self-review read all seven changed files and relevant VFS/refcount,
copy/write and teardown owners. Independent reviewer coverage is not claimed.
Failed allocation/remap preserves the old mapping; resize zeroes reused capacity
and anonymous new pages are zero. Shrink unmaps whole unused pages. Unlinked
bytes remain charged while any alias/process references them. Domain teardown
clears every file mapping and checks zero charged capacity after owned refs and
tasks drain. Every old upstream fixture remains; only the obsolete64 KiB failure
expectation changes to successful growth. Default test registration retains all
security cases; explicit scenario selection permits ordinary functional checks.

## Exact evidence and limitations

The new growth workload fails with code121 on exact prerequisite composition
ccd42e31 at the first write beyond64 KiB. Candidate and clean-applied
binaries pass growth, bounds and metadata-quota scenarios. These cover128 KiB
round-trip data,2 MiB sparse writes, truncate/regrow zeroing, shared offsets and
contents across forked aliases, open-unlinked lifetime,16 MiB shared allocation
exhaustion/recovery and failed growth preserving bytes/size/offset. They also
leave linked full-quota data for successful teardown to reclaim.

`make check test-unit build build-launch-storage-fixture` passes in both exports
(85 units). Strict patch replay matches all169 tracked paths. Static/unit/source
and ordinary runtime evidence is complete for this extraction; R03 full runtime
CI, DaybreakBlue security scenarios, macOS Actions and standalone refresh remain
pending. No remote green CI or completed security acceptance is claimed.

Full logs, exact manifests, baseline failure and concise draft are retained in
`.artifacts/ginux-contribution-preparation/G09b/`. No branch, real checkout/index,
remote body/comment or PR changed. No outgoing messages or pushes occurred.

## Current-main refresh — 2026-11-06

The exact prerequisite combines refreshed G06b `7240bb21` and G09a `648ec32a`;
both parents are recorded. Only G09b then applies, unchanged at seven files,
+281/-13. All added/removed source lines match the earlier corrected extraction.
Current upstream native/domain/network/process owners and the new G09a chunk
coverage remain intact. The G06b duplicate memmove is already resolved in its
prerequisite; it is not reintroduced.

A separate full-diff self-review reread VFS resize/write/truncate/copy-up,
MemoryStorage grow/read/write/clear, reference release and task teardown callers.
Page rounding and growth remain checked before publishing mappings; failed
allocation leaves the old object intact. Regrowth zeroes reused capacity, shrink
returns pages and the last reference releases unlinked backing. Teardown drains
refs/tasks before clearing every file mapping and asserting zero charged bytes.
The bound remains mapped writable backing, including growth reserve, rather than
aggregate RAM. No new blocking source correction was identified; independent
reviewer or security acceptance is not claimed.

Candidate and strict clean prerequisite-plus-only-G09b replay both pass:

- `make check test-unit build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-launch-pipe-fixture build-launch-fd-fixture build-launch-task-fixture build-launch-exec-fixture build-process-fixtures build-gate1-fixture`, including all 94 current unit tests, 18 process tests and 28 VFS tests.
- The existing selected `test-launch-storage.py` scenarios `growth bounds quota`.
  They cover 128 KiB byte readback, 2 MiB sparse growth, truncate/regrow zeroing,
  forked aliases, open-unlinked lifetime, 16 MiB shared mapped-capacity exhaustion
  and recovery, failed-growth preservation and full-quota linked-file teardown.
- Forty-six additional ordinary launch/storage/device/FD/pipe/process/exec and
  owned lifecycle controls per export. The ordinary storage fixture's old
  64 KiB failure expectation is updated to growth with the extra byte read back;
  no complete default source test is disabled or weakened.

That is forty-nine ordinary controls per export. The separately built exact
prerequisite runs the identical growth fixture and fails at workload exit 121
on its first write beyond 64 KiB (runner exit 1); candidate and clean replay
both succeed. This is new capacity evidence, not a whole-suite pass.

All 206 tracked paths match the retained head in source and clean exports.
Both binaries hash `cb649ef0bcc70e63056e8bda04a655d0e05de7cb0d39a86dbb52f84968de9832`.
Source, clean and prerequisite builds use isolated targets. Exact commands,
logs, baseline failure, receipts, old/new patches, source exports, manifest and
concise draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G09b/`.
No real Ginux branch/index/checkout or remote text changed.

Keep G09a/G09b as two review subpackets of one useful growth contribution; no
abstraction-only proposal or public stack. Actual prerequisite upstream merges
and fresh standalone extraction remain pending. mmap/mremap rollback under
adversarial conditions, indirect/protected copies, argument authority, full
default Linux runtime and macOS/remote CI remain unrun with the downstream
DaybreakBlue boundary. No agent was dispatched. G07 and final F02/F03 consumer
closure remain pending; ordinary growth does not supply thread acceptance.
