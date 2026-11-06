# G12 — per-process owned mappings; thread/security joins pending

Head `83072ec3ddab3e4712ce8db9ba232a10342557de`, tree `4943047fea72e3e814ca38794fc573e9bd050238`, sole local prerequisite
G11 `0798d991ab2d652e54682750acb4fdd9f1e6d9ac`. [Retained patch](patches/G12.patch). Ten files,+663/-80.
This is partial G12 preparation: per-process source/ordinary functionality is
retained, but G07 thread ownership, nonleader acceptance, security, full CI and
current-main independence remain pending. Do not publish the local composition.

## Implementation and separate self-review

Move G09's deferred32e2c85f backing change here with its real mapping consumer.
Each live owned file retains anonymous memfd identity across mediated writes and
ftruncate. Supervisor mappings remain MAP_SHARED; capacity and logical length
are distinct. A replacement mapping is obtained before committing file resize,
so a failed map/truncate preserves previous bytes/address/capacity. Zero-size
truncation keeps file identity; final object release unmaps and closes its backing.
No new configuration, enlarged namespace or public FD budget is imported.

A readable owned regular-file descriptor can supply a separately opened read-only
supervisor reference. The current per-process task transaction creates a temporary
socket pair, transfers that reference through SCM_RIGHTS, performs the caller's
mmap, closes all temporary native descriptors and restores scratch/registers.
MAP_PRIVATE and read-only shared mappings are supported. Native/image/anonymous
mapping handling stays with the prior owner. Signals or unexpected task death
while private setup is active fail the domain; teardown avoids recursive abort.

The separate final self-review read the entire exact diff and existing descriptor
translation, VFS positions/executable pins/lifetimes, storage growth/resize,
exec/script cleanup, typed death/reap and teardown callers. No independent
reviewer coverage or security acceptance is claimed.

- **High, port boundary:** the mapping donor assumed G07 DescriptorGroups and
  shared-table mutation barriers. Current source uses per-process Task descriptors
  and MappingCall state instead, with no CLONE_FILES/thread capability imported.
  The donor's PIDFD_THREAD/EINVAL fallback is retained in file_mapping.rs:228;
  its nonleader/leader-exit fixture hunks are saved as held-thread-join.patch in
  the artifact directory for the eventual G07 join. They are not silently dropped
  or claimed to have run. Mapping while shared FD-table mutations occur is pending.
- **High, retained source invariants:** storage_memory.rs:53/70/168 retains one
  backing identity, logical length checks, preflight capacity charging and close
  on clear. fs_hooks.rs:547 obtains backing only through VFS readable_storage;
  no guest-supplied host path or public/private FD interchange is introduced.
  Adversarial authority/rollback validation remains pending.
- **Moderate, actual static failure corrected:** current clippy rejects the
  donor's manually NUL-terminated storage name. Use a C string literal without
  disabling the lint. The first failed log remains retained; final check/build
  succeeds. An initial control command ran in G11 rather than G12; that log is
  explicitly named prior-g11-control-build.log and is not G12 evidence.
- **Moderate, memory contract:** writable-file capacity remains charged, but
  guest mappings/COW pages and mappings retained after unlink are not aggregate
  memory-accounted. README states this limit explicitly; no mapping budget,
  aggregate RAM limit or broader resource claim is added.
- **Blocking for publication:** G07/G10 shared argument/scratch authority remains
  unresolved, and the new pidfd/SCM_RIGHTS/private FD replay requires monitored
  low-level verification. See the [DaybreakBlue handoff](daybreakblue-handoff.md).
  Full mapping protection, fixed-region, SIGBUS, private-FD and interruption tests
  are retained, compiled and unrun here. Nonleader mapping acceptance is unrun.

## Exact ordinary evidence

Both source and strict clean prerequisite-plus-patch export pass `make check
 test-unit build build-launch-storage-fixture build-launch-owned-exec-fixture
 build-launch-script-fixture`:92 units,34 VFS tests and178 matching tracked paths.
Core/native supervisor/process-table and script owner source are unchanged from
G11. No upstream unit/CI coverage is removed. Existing CI already builds and runs
the storage fixture; its default now includes full and ordinary mapping scenarios.

Two selected ordinary mapping modes pass on source and clean exports. Valid
read-only shared and writable private mappings preserve offsets, share mediated
file changes, keep private writes copy-on-write, see a forked child's regular
writes, support ordinary unmap/truncate/regrow, and retain old bytes across
unlink/recreation. No protected access, deliberate fault or private FD inspection
runs in these selected modes. The exact G11 binary fails this same valid mapping
fixture with workload code152, while G12 passes. Storage growth/bounds/quota,
six ordinary owned-ELF and14 ordinary script controls also pass in both exports.

Commands, receipts, binary hashes, source helpers, baseline/static failures and
held thread join are retained under .artifacts/ginux-contribution-preparation/G12.
Full runtime/security fixtures, old-kernel pidfd fallback, independent review,
macOS/remote CI and standalone refresh remain pending. Local Linux success is
not green remote CI or G12 completion. All real branches, checkout/index state,
remotes, human PR text and maintainer communications remain unchanged.
