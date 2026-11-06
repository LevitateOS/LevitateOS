# G09b — bounded growable configured storage

Head `297ad8dde26e02ca32d6becd60d654c1fd67ecd0`, tree `17008873b421921585a93748a341a9f0cb1f21f3`, on composition `ccd42e319c3a76bf85d9cfce81a44cc9ba051911`
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
