# G09a — bounded VFS byte-storage owner

Current retained local head `648ec32a34ece2f1cd7d37b10271e4166eb14683`, on refreshed G03b
`5fb12ab4a56efd365d7d451d181f6e2a41c9d31d`, rooted at observed main `8a475e8b`.
The current-main refresh below owns fresh evidence and pending checks; earlier
results remain historical.

Earlier head `d29381c72352c33e33a4f164258f9ac80239ee85`, tree `e81178500226544c6d053471abbd53a95d68db3f`, sole local prerequisite G03b
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

## Current-main refresh — 2026-11-06

The patch applies on refreshed G03b, without G03c/G06/G07 ancestry. Production
code is byte-identical to the previous corrected extraction. One focused unit
adds 69 lines covering a 9,000-byte image copied through three chunks, plus a
second-read error after the first chunk was written, namespace cleanup and
backing reuse. All prior tests remain intact. The final scope is one VFS file,
+289/-25, with 28 VFS tests.

A separate self-review reread the full diff and current path/open/read/write,
position/alias/whiteout, copy-up, truncate and teardown owners and the existing
`vfs_main` caller. Allocation/range validation stays with the backing owner;
metadata and shared positions commit after successful operations. Truncating an
image creates empty owned bytes without requiring the discarded image capacity.
Copy-up checks its extent and clears partially copied storage on error. Inline
backing and the existing constructor remain the upstream defaults. No new
blocking production change was identified; this is not independent reviewer or
security acceptance.

The same existing-API truncation regression fails on exact refreshed G03b with
`FileTooLarge`. Candidate and independent clean G03b-plus-only-G09a replay pass:

- `make check test-unit build build-launch-fixture build-process-fixtures build-gate1-fixture`, including all 89 current unit tests and 28 VFS tests.
- Eight selected ordinary configured-launch/valid-wait controls per export.

The new chunk test checks every copied byte and reuses the sole storage slot
after partial copy-up failure. Existing cases retain truncate/regrow zeroing,
capacity failure without state changes, alias offsets, unlinked lifetime and
unchanged image descriptors. These are library capability results; configured
runtime growth is the separate G09b owner and is not established here.

All 187 tracked paths match the retained head in source and clean exports.
Both binaries hash `46e85e087a99f0bb383c25b8ebcf61a1d846c5a49287807e32ecbe18cf0bc37c`.
Exact source, commands, baseline failure, logs, receipts, old/new patches,
manifest and concise draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G09a/`.
Build targets are isolated; no real Ginux branch/index/checkout or remote text
changed.

Keep G09a/G09b as review subpackets for one useful growable-storage contribution;
no abstraction-only PR or public stack. Actual G03b/G05a upstream merges and
fresh standalone extraction/revalidation remain pending. Malformed image/backend
security acceptance, full default Linux runtime and macOS/remote CI remain unrun.
No DaybreakBlue agent was dispatched. G07 remains unready, and combined F02/F03
acceptance is still pending at the eventual final revisions.
