# G25 — bounded owned vectors and chunked regular writes

Head `9afd60cca140881288498fc21058b404814c08e4`, tree `1caf52f3f4d2cf5374de29991cd7c525491a6de9`, directly on G17a
`94baebbec5e887ae659718c3f6162d08ca267b99`. [Patch](patches/G25.patch):8 files,+253/-25.
Ordinary source preparation is retained. **The full packet remains partial**:
actual APK/RPM, security, full CI and current-main independence are pending.

Readv/writev use a single snapshot of up to16 x86-64 iovecs with checked aggregate
length bounded by isize::MAX. Existing strict process_vm copies perform scatter/
gather, reporting only the transferred prefix. Owned vectors and scalar reads
retain the4 KiB per-call limit. Regular writes/pwrites process up to1 MiB in4 KiB
scratch chunks; later errors return progress already committed. Positioned writes
restore the original shared offset, including after a partial quota result.
Pipes retain their4 KiB transfers and use the entire vector length for small-write
atomicity, nonblocking capacity checks, partial consumption and EOF.

The capability base is G17a (including G09/G05), without G17b, mappings, archives,
timestamps, locks, threads or socket channels. Existing public FD translation,
VFS quota/offset owners, strict copy and pipe ring-buffer ownership remain in
place. Core native policy, descriptors.rs, launch/task dispatch, storage backend,
VFS implementation and all other tracked paths are unchanged. The74835089 native
read clamp and its native FD fixture are held separately under #178/#163; no
consent or native-policy resolution is inferred from owned-file checks.

Separate whole-diff self-review covered all eight changed paths and their VFS
read/write/seek, shared-offset, permission/quota, public descriptor, pipe claim/
park/retry and tracee-copy callers. This is self-review, not an independent reviewer
or security acceptance.

- **Length validation correction:** reject aggregate iovec lengths above the
  signed transfer-result range as well as arithmetic overflow. Bounds follow
  the declared16-vector contract; hostile overflow/protected-buffer probes were
  not executed.
- **Verification correction:** first draft of the large-read fixture passed a
  request larger than its destination array, relying on this adapter's short
  read bound. The final fixture allocates the entire requested span, clears it
  after writing, then verifies every returned byte. Prior49988e22 is labelled
  before-buffer-fixture-fix and is not the final candidate.
- **Build correction:** removed the obsolete channel copy_memory import after
  routing through the shared vector helper. Initial failed static output remains.
- **Coverage preserved:** existing storage/pipe default runners execute the new
  cases alongside all old cases; Make/workflow already call those runners.
  Selected local modes avoid protected buffers and prior adversarial scenarios.

Final source and strict clean base-plus-patch replay pass make check test-unit
build and storage/pipe/directory fixture builds:89 units,31 VFS,171 tracked paths
byte-matched. Ginux and all three fixture hashes match. Both copies pass:

- zero/empty iovecs,16 entries, rejected17-entry request, vector-boundary short
  transfers, EOF and duplicate descriptors sharing offsets;
- complete2 MiB+37 regular-file payload written using short1 MiB calls, reread
  using short4 KiB calls and checked byte-for-byte, plus large positioned writes
  preserving duplicate offsets and content;
- regular write quota exhaustion after one committed chunk, correct size/offset
  and next-call ENOSPC, plus existing positioned/growth/bounds/quota controls;
- vector pipe atomicity at capacity, nonblocking short transfer, scatter/gather,
  duplicated writer lifetime and EOF, plus directory/cwd controls.

The same final fixtures fail on exact G17a: file vectors exit181, large write197,
pipe vectors91. Baseline receipts name the exact binary hash. The earlier plain
source logs predate the fixture correction; functional receipts and final/clean
static receipts identify final evidence. Native read clamp remains untested here.

[DaybreakBlue](daybreakblue-handoff.md) owns protected/mutable iovec and payload
validation, partial faults, overflow/authority/FD reuse and thread/park races.
No cloud dispatch occurred. Full/default Linux runtime suites, independent review,
macOS/remote CI and fresh upstream standalone extraction remain unrun. Actual
APK/RPM on the full combined capabilities and G08 socket integration belong to
F02/F03; fixtures do not substitute for package-manager acceptance. Preserve G23
SQLite/Cargo and G24 native-time policy/security holds as well.

Patch, generic draft, commands, receipts, hashes, helpers and diagnostic logs live
under .artifacts/ginux-contribution-preparation/G25/. Real Ginux branches,
checkouts/indexes/refs and remote human text remain untouched. No publication or
maintainer message before the F04 user inspection boundary.
