# G24 — inode timestamps and archive mtime

Head `b33fbd24ace566d8db20c8e2d7a295680084ac7b`, tree `b72397b4a78e5363279b1884628752c0da67f709`, on G22
`7b2adc4f75acca26f2b356fbf9a47a9644c39bc8` (includes G15 inode identity). [Patch](patches/G24.patch):12 files,
+261/-8. Source/ordinary preparation is retained; clock policy/security,
full CI and refreshed upstream independence remain pending.

Access/modified timestamps live with the canonical owned inode, shared across
hardlink names and retained descriptors. New owned objects and VFS index metadata
start at zero. Explicit setters validate both nanosecond values before mutation;
reads/writes do not automatically update timestamps. Existing stat/statx encoders
project the fields at their x86-64 ABI offsets, with zero values for current
pipe/device metadata. The existing VFS component walker and assignment owner are
reused. Core syscall policy, configuration/index format, process ownership and
backing code are unchanged.

Archive export encodes representable nonnegative whole-second mtime; import
restores it with zero nanoseconds and checks a hardlink's mtime agrees with its
prior regular entry. Atime/runtime nanoseconds are intentionally not persisted.
Existing private staging, no-overwrite publication and failed-startup cleanup
remain. UTIME_NOW/implicit NOW uses supervisor CLOCK_REALTIME; this is documented,
not advertised as virtual time or upstream policy consent. It does not enable
workload clock syscalls. Ordinary NOW/null-times checks pass; policy disposition
remains pending.

Separate final self-review read the entire diff and current alias/object/FD
lifetime, stat serializers, shared path/device dispatch, public descriptor
translation, creation/reset, dynamic metadata charging, archive timestamp fields,
matching alias import and failure/teardown callers. This is self-review, not
independent or adversarial review.

- **High, reproduced valid-call failure fixed:** the initial adapter returned
  EFAULT for a valid futimens descriptor/null-path call. Device dispatch consults
  shared request_path before filesystem dispatch, which tried to read that null
  pathname. The shared resolver now returns no path for this descriptor form;
  existing descriptor translation and filesystem FD validation still own it.
  The same ordinary fixture passes. Earlier failure logs remain labelled before
  the null-path correction; exact G22 separately returns unsupported and exits72.
- **Current owner adaptation:** legacy patch rejects are reconciled with current
  metadata constructors and descriptor transactions. No old lexical normalizer,
  G23 lock owner, G20 control or held thread/poll policy is imported. The donor
  network Stat initialization is explicitly assigned to G29, where that absent
  backend will be introduced.
- **Contract limits:** only existing owned objects support these setters. Public
  nanosecond values are retained at runtime; USTAR persistence remains seconds
  only. Negative/unrepresentable export times fail rather than silently changing.
  Clock-boundary/failure behavior and invalid fields/authority/races remain unverified.
- **Coverage:** existing storage and restore targets/CI invocations include the
  new cases. Full invalid-pointer and mismatched-hardlink-mtime fixtures are
  retained in default runs. Local ordinary selection excludes those cases.

Final source and strict clean prerequisite-plus-patch replay pass make check
test-unit build and storage/directory/script/owned-exec/FD fixture builds:
100 units,41 VFS,181 matching tracked paths. Ginux and all five fixture binaries
match. Ordinary checks in both copies cover explicit nanoseconds, stat/statx,
UTIME_OMIT preserving one field, ordinary UTIME_NOW and null-times single-capture
updates, path and descriptor setters, alias and retained
FD updates after removing the original name, and a valid no-follow update to a
contained owned symlink. Linked archive restart preserves mtime seconds, resets
nanoseconds/atime as documented, and retains shared identity after mutation.
Existing5,000-file and complete2 GiB+1 archive checks with streaming payload digest,
seven flush/export/config, two mapping, ordinary symlink, six ELF,14 script and
growth/bounds/quota controls also pass.

Clock-boundary/failure cases, invalid flags/fields, protected or mutable time structs,
no-follow/assignment bypass, timestamp overflow/export failures, conflicting
alias metadata and races remain with [DaybreakBlue](daybreakblue-handoff.md).
No such probes or cloud dispatch occurred. Independent review, full/default Linux
suite, macOS/remote CI, current native-realtime policy resolution and fresh-main standalone
acceptance are unrun. G20/G23/remaining filesystem interaction and actual consumer
closure remain F02/F03; G23's SQLite/Cargo acceptance is still partial.

Patch, generic draft, commands, receipts, hashes, helpers and failure/reject logs
are retained under .artifacts/ginux-contribution-preparation/G24/. No real Ginux
branch, checkout/index/ref, remote human text or maintainer message changed.
Publication remains stopped at the F04 user inspection boundary.
