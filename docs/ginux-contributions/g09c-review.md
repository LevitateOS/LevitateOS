# G09c — VFS directory ownership

Head `94fbfc2d2c67f90efb08f99ed18dee91188ceb17`, tree `13309a21ab7b2a445eea6752baa707adf788ddbb`, on G09a `d29381c72352c33e33a4f164258f9ac80239ee85`.
[Retained patch](patches/G09c.patch). One VFS file,+491/-2. This is a library
review subpacket intended with its future runtime adapter, not a promised
standalone library PR. Current-main independence waits for its prerequisites.

Directory descriptions share cookies through existing open-position ownership;
metadata/inodes survive rename/removal while references exist. Enumeration
merges image/overlay entries, excludes whiteouts and inspects cookies without
consuming them. Rename validates type/replacement, descendant capacity and image
subtree restrictions before mutation. Rmdir retains an open removed directory.
Fchmod/chmod ignore file-type bits while continuing to reject special bits;
nonblocking status is retained. No runtime syscall, threads or mapping join.

A separate final self-review read the whole diff and existing resolution,
whiteout allocation, metadata, offset and storage release owners. Independent
reviewer coverage is not claimed. Adapted the older donor's lexical path code
to current component resolution, including link-before-parent behavior and
nofollow final mutation. Reused the shared position array for directory aliases.
The initial review regression showed rmdir(dir/.) removing dir (Ok instead of
InvalidPath); final dot/dotdot mutations now reject without moving/removing their
parent. This bounded API returns InvalidPath for these unsupported mutation
names; it does not promise every Linux errno. Ordinary trailing-slash removal
continues to work.

All84 units,31 VFS tests, make check/build pass; clean replay matches150 tracked
paths and strictly applies the retained patch. Four new tests cover enumeration,
replacement/open objects/long descendant preflight, alias-cookie reuse/image
links/mode bits, and dot-entry failure preservation. Earlier upstream tests are
unchanged. Source/functional library checks and clean replay are complete; runtime
adapter, full CI/security validation, macOS Actions and standalone refresh remain
pending. No remote green or independent security review is claimed.

Patch/draft/logs/manifests and initial failing review case are retained under
`.artifacts/ginux-contribution-preparation/G09c/`. No real Ginux checkout/index,
branch or remote changed; no PR or maintainer message was created.

## Current-main refresh — 2026-11-06

Refreshed head `1e34ef3641c9a2d93f449818692692ac2b561ebc`, tree
`d75ce26c93b8d8f102f7b3e7d6ffce9bc6ee60e1`, on refreshed G09a
`648ec32a34ece2f1cd7d37b10271e4166eb14683`, incorporating observed main
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`. The historical evidence above
remains historical. Only test-append context changed: production and all four
added directory tests are byte-identical, and G09a's new chunk-copy unit remains.
A separate self-review reread the complete delta and existing component resolver,
metadata, alias positions, storage release and namespace allocation callers.
Independent reviewer coverage is not claimed. Scope remains one VFS file,+491/-2.

Both source and strict clean replay pass `make check test-unit build
build-launch-fixture build-process-fixtures build-gate1-fixture`: 93 units,
including32 VFS tests, with no removed upstream coverage. Eight ordinary
configured-launch/valid wait-status controls pass per export. All187 tracked
paths match the final head; both binaries have SHA256
`41c7556ebe8d8bc3525248ff5d5c88d6750fa9ce0e039da108f2ee5db65d5a81`.
Logs, exact command receipts, patch, draft and manifest are retained in
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G09c/`.

This is local source/ordinary preparation only. Pair G09c with G09d's runtime
adapter and DirectoryNotEmpty errno join; do not publish an intermediate stack.
Actual upstream prerequisite merges, runtime adapter verification, full default
runtime/security, macOS and remote CI remain pending. No branches or remote
publication changed.
