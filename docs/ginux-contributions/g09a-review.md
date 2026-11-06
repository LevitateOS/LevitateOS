# G09a — bounded VFS byte-storage owner

Head `d29381c72352c33e33a4f164258f9ac80239ee85`, tree `e81178500226544c6d053471abbd53a95d68db3f`, sole local prerequisite G03b
`785bfb9d6b7fcfb901d9f5efb8a845c8d9d4d53a`. [Retained patch](patches/G09a.patch). One VFS file,+220/-25.
Not standalone-ready until G03b and G05a merge upstream. No G06/G07 dependency.

## Change and review

Separate file bytes from VFS metadata/open descriptions through OverlayStorage;
retain the inline default and existing VirtualFs constructor/const arguments.
Backends own allocation and zero newly exposed ranges. VFS keeps aliases,
whiteouts, metadata, offsets and final-close cleanup. Add ftruncate-style library
behavior and chunked image copy-up without a runtime allocator or mapping change.
No runtime storage/thread/network feature or CI deletion is included.

A separate self-review read the final diff, existing path/whiteout/offset owners,
callers, copy-up rollback and teardown. Independent reviewer coverage is not
claimed. The O_TRUNC review case initially failed with FileTooLarge: copying the
image before discarding it required unnecessary capacity. Both exact prerequisite
and initial extraction fail; corrected code creates an empty overlay directly.
An existing image descriptor still reads its original bytes. Failed copy-up
leaves image contents and the storage slot usable; open-unlinked aliases retain
bytes until their final close, and reused backing regrows as zero. Chunk offsets
are checked before allocating/copying. Existing upstream tests remain intact.

## Evidence and limits

`make check test-unit build` passes in source and clean prerequisite-plus-patch
export:80 units,27 VFS tests. All150 tracked paths match the retained head after
strict patch replay. Four selected ordinary process-native smoke scenarios pass
(copied_files, nonblocking_wait, vfork, thread_reuse). Full runtime CI suites were
not rerun; their security/fault scenarios are reserved for DaybreakBlue. macOS
Actions/full Rocky VM are unrun; no remote green CI is claimed. Source review,
functional reproduction and clean replay are complete; R03's full CI evidence and
current-main standalone refresh remain pending. No security acceptance is claimed.

Patch/draft/manifests/logs and baseline failure are retained in
`.artifacts/ginux-contribution-preparation/G09a/`. No branch, real index, checkout,
remote text or PR changed. Publication remains subject to user inspection.
