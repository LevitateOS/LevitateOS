# G17b — private backing budget and captured public limits

Head `6fcdd6183703768a2f4ed1cc48380eaa50da547c`, tree `4a08c6a7c1735664b46e5fc8675518cb4e1c6bc1`, on local composition
`f41ed1b093044b10d45fcab771687dfb089ed3a5` of G12 `83072ec3` and G18 `f58ba480`.
[Patch](patches/G17b.patch): seven files,+80/-27. G17a pwrite is deliberately
independent. Security/full CI and actual upstream independence remain pending.

Capture DescriptorTable capacity before preparing storage. Assigned storage
reserves max_objects+128 private descriptors after metadata/data validation,
raising only the supervisor soft limit when its existing hard limit permits it.
The tracee applies the captured capacity to both native soft/hard bounds, then
uses existing close_range cleanup; its public allocator keeps the same capacity.
No hard limit is raised, and no-storage launches skip reservation. No new public
configuration, process table or resource-control abstraction is introduced.

A separate final self-review read the entire delta and relevant constructor,
resource capture/validation, descriptor allocation/fork-copy, bootstrap cleanup,
memfd backing/clear, mapping and task lifetime callers. Source stays with current
owners. VFS, memory backing, typed process/native supervisor, mapping transactions
and tracee copy implementation are identical to the composed prerequisite. This
is self-review, not independent reviewer/security acceptance.

The composition retains G12 memfd identity and G18 checked/page-rounded metadata
allocation, shared positions and actual namespace length. Four merge conflicts
were resolved locally in the constructor/backing and fixture joins. An outdated
G12 FD fixture still expected owned mmap rejection; its valid readonly-map
assertion is corrected in the prerequisite, with writable-shared rejection retained.
That mapping-owned join and the full prerequisite diff are retained for F01/F02,
not hidden in this contribution's delta. An initial duplicate fixture helper was
caught by -Werror and removed; no warning/test was suppressed. No G07 thread or
G13–G16 ancestry is imported. Those interactions still require F02.

Source and strict clean prerequisite-plus-patch export pass make check test-unit
build and storage/directory/script/owned-exec/FD fixture builds:94 units,35 VFS,
178 matching tracked paths. Ginux and all five fixture binaries match. Clean apply
uses --whitespace=error. Existing default units and harness routes remain intact;
limits-only selection permits ordinary verification without running full authority
or width probes. Its output accurately names only the selected checks.

Ordinary source/clean checks demonstrate public soft/hard limit8, creation/write/
close of20 distinct nonempty owned files through public FD3, alias exhaustion at
public8, and startup rejection when the host hard limit8 cannot reserve backing.
The5000-file scenario starts with supervisor soft64, raises it within the existing
hard bound, reads every file and reclaims the last pinned namespace slot. Two
valid mapping modes, growth/bounds/quota, six owned-ELF and14 script controls pass.
The exact prerequisite fails the same low-limit fixture at workload43 (native
hard bound remains256 rather than8) and the scale fixture at workload160 with
EMFILE(-24). The new candidate passes both. Logs/commands/receipts are retained.

Private-FD authority/leakage, protected-copy, injected failure and raced ownership
remain with the [DaybreakBlue handoff](daybreakblue-handoff.md), including inherited
FD overhead assumptions in the fixed128 reserve. Full/default security modes,
independent review, macOS/remote CI and standalone-main refresh are unrun. These
Linux-local results are not green CI or security acceptance. No branch, real
checkout/index/ref, remote text or maintainer message changed. Patch, prerequisite
diff, generic draft, composition, hashes and helpers are retained under
.artifacts/ginux-contribution-preparation/G17b/. Publication remains after F04.
