# G04 — assigned virtual null devices

Local feature commit `5036b7e8a61c4cf1e18abad8746208d03d268ea5`.
Explicit local prerequisite G03c `b36e0752caa1c91e4bbbe112c589c85915a6ce6d`
(on G05a/G03a/G03b), underlying upstream3f2fe7df.
[Retained source patch](patches/G04.patch). Twelve files,+601/-119.

Use the existing device registry for the configured immutable /dev namespace,
empty by default. Only an explicit null assignment opens null handles. Data and
metadata stay virtual; no host device node/descriptor is used. Device dispatch
precedes memory-file dispatch because their public descriptor ranges differ.
The shared strict copy/path helpers move to tracee_io; memory metadata uses that
same serializer rather than maintaining a second copy.

## Review findings and corrections

The entire final diff and registry/copy/path/bootstrap callers were reviewed in a
separate self-review pass; independent reviewer coverage is not claimed.

- The original donor e3458063 allowed an image alias to bypass its lexical /dev
  classification and open a hidden image null file. The final controlled alias
  case exits55 on that exact donor and passes here. The shared component walker
  now resolves both direct and aliased device names, including when no storage
  or devices are assigned. Missing/regular-file parents retain ENOENT/ENOTDIR.
- The first adapted device dispatcher incorrectly consumed memory-file
  AT_EMPTY_PATH metadata (existing storage fixture exit105). Empty paths belonging
  to memory/native FDs now remain with their owner. A negative AT_FDCWD value
  cannot be mistaken for a positive device handle; its empty-path native metadata
  behavior is retained and checked.
- Device stat/fstatat/statx reuse the corrected supported flags, reserved-mask
  and conflicting-sync validation. Character mode, virtual inode and major/minor
  1:3 are checked through both descriptor and path metadata. Read-only destination
  pages and bad pointers fail through strict process_vm copies.
- Native descriptors stay below the virtual memory/device ranges even for
  device-only/default configurations; lower inherited native limits are never
  raised. Unknown/closed handles fail before registry access. Open mode, CLOEXEC,
  slot reuse/exhaustion, zero-byte EOF, ignored buffers, discarded-write bounds,
  immutable metadata and new-launch teardown are covered.
- Only explicit null is accepted; unknown/duplicate assignments fail before
  workload execution. Existing source and CI coverage remain, including all
  current unit modules, native supervision and #221 copyout/fallback fixtures.
  No downstream policy, network implementation or unrelated feature is included.

The runtime deliberately reuses G03c's configured namespace/copy owner, so the
prepared runtime depends on that feature. The basic device library already exists
upstream; duplicating the storage/path implementation to claim independence would
not be a coherent extraction. Later device/cwd/link/exec joins remain G16/G05+.

## Validation and delivery boundary

Both candidate and clean G03c-plus-only-this-patch sources pass77 unit tests and:

- `make check test-unit`
- `make build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-process-fixtures build-gate1-fixture`
- `sudo -n make test-launch test-launch-storage test-launch-devices test-native-supervision test-gate1-setup`
- `make test-process-native` outside the ptrace-restricted execution sandbox

The final two added C assertions explicitly check invalid ignored buffers and the
4096-byte discarded-write cap. No executable implementation changed after the
whole-suite pass; the final fixture was rebuilt and passed in both source trees.
A fresh strict application of the final retained patch matches all157 tracked
paths. Current supervisor.rs and core contract remain byte-for-byte unchanged.
Exact hashes, explicit runtime exit receipts, original failure, complete logs and
short draft are retained in `.artifacts/ginux-contribution-preparation/G04/`.

R01–R03 and clean composition proof pass locally. **Standalone delivery waits for
G03c and its prerequisites to merge upstream**. Refresh on new main and revalidate
before publication; no public stack. macOS Actions/full Rocky VM checks are unrun.
Only null,64 slots and4096-byte discarded writes are provided; duplication,
readiness, additional devices and configured child/thread ownership remain later
work. The shared canonical native path scratch restrictions remain documented.
No real Git branch/index/worktree/remote was changed.
