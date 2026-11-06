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
