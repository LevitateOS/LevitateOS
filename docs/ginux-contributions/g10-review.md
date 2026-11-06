# G10 — owned ELF source and ordinary checks; security held

Head `14866b1de07bc8474557396c09f74275e8fb6bf5`, tree `6fe2f90d36faa7c50143004505d283bab176ad90`, sole local prerequisite
G09d `85b53fe15a64041f451ec7e7aa1e77d08cb2c18a`. [Retained patch](patches/G10.patch). Fourteen files,+1165/-131.
No unmerged stack may be published. Current-main independence, full CI and
security acceptance remain pending; this is not publication-ready.

## Change and findings

Pin owned ELF byte identity in the existing VFS, stage it through private sealed
memfds and join execve/execveat to the current configured task transaction. Keep
current typed waits/statuses, strict copies, native interpreter validation and
successful-exec-only CLOEXEC retirement. Each process owns its executable pin;
fork retains it, failed reservation/exec and death release it, and successful
replacement commits the new pin before releasing the old. Owned directory and
unlinked read-FD execution are supported. No G07/G08 threads/polling, scripts,
G12 mapping identity or G17 resource-budget implementation was imported.

A separate final self-review read the whole diff and relevant callers, VFS
whiteouts/offsets/storage, FD snapshots, exec states, interpreter metadata,
proc-link projection, typed death/wait and teardown. Independent reviewer or
security validation is not claimed.

- **High, source correction:** the old donor's owned ELF check omitted its
  PT_INTERP authority join. fs_hooks.rs:430 now reuses path_hooks.rs:191's existing
  bounded metadata reader and the current interpreter namespace/physical-image
  check. Native and owned programs share one parser; no second ELF loader or
  weaker lexical resolver was added. Hostile-interpreter runtime verification
  remains assigned to DaybreakBlue, not claimed as passed.
- **High, reproduced port correction:** VFS create_overlay at lib.rs:1348 must
  exclude a pinned whiteout from same-name reuse, as well as from general slot
  allocation. Removing that guard reproduces the existing executable-pin test
  failure (exit101); the actual candidate passes. Pin/unlink/recreate/name tests
  retain old bytes until the last executable reference releases them.
- **Moderate, source correction:** launch_tasks.rs:413 prefers the VFS name for
  owned programs even when the native root is '/'. fs_hooks.rs:485 reserves
  MAX_PATH+10 for the deleted suffix. Runtime readlink/protection checks remain
  pending; library name/lifetime tests pass.
- **Moderate, fixture correction:** launch-exec-fixture.c:32 must expect the
  main executable '/allowed-elf' while its PT_INTERP runs, preserving the existing
  allowed-interpreter scenario. The older blanket '/bin/' assertion was too
  narrow. It compiles; its readlink runtime validation remains pending.
- **Blocking for publication:** stable kernel-argument capture under shared
  writable mappings is unresolved, including exec_step at launch_tasks.rs:739.
  See the [DaybreakBlue handoff](daybreakblue-handoff.md). No new exploit-like
  validation was executed here. Excluding G07 does not establish this property
  for other permitted shared mappings. No full security acceptance is claimed.

Sealed executable copies add memory beyond the documented writable-backing
allocation bound; this feature does not establish an aggregate RAM quota.
Signal delivery during injected setup remains a fail-closed limitation. Legacy
memfd/seal compatibility paths and hostile failure cleanup need monitored
validation. No new unsupported-ABI/Linux compatibility promise is made.

## Exact evidence

Both source and final clean prerequisite-plus-patch export pass make check,
test-unit, build and the owned/native exec, directory and storage fixture builds:
91 total units,33 VFS tests. The final strict replay matches 174 tracked paths.
Core/native supervisor/process-table source is byte-for-byte unchanged from the
prerequisite. All existing unit modules and CI steps remain; new default owned
exec coverage is registered in the existing integration job.

Six selected ordinary scenarios pass in both exports: path, read FD, unlinked FD,
owned relative-dirfd execution, child execution and replacement back to an image
program. The compiled fixture is 83,288 bytes and reads a 64 KiB data
object after loading, exercising execution beyond the old fixed file limit.
Missing-file failure preserves live CLOEXEC/public FD state in the reexec case;
successful replacement retires the marked public descriptor. Normal image-cwd
and storage growth/bounds/quota controls also pass. Explicit command/exit receipts
are retained. These checks exclude malformed input, private FD/seal inspection,
protected buffers and readlink authority probes.

The exact G09d binary rejects the focused valid owned-program execution scenario
with workload code128, while the candidate passes. The first comparison hit the
old missing-path error difference at code 129; the failure-preservation assertion
was moved to the separate ordinary reexec case, preserving it while isolating
actual capability measurement. The earlier log is retained.

Full runtime/security scenarios, virtual executable-name runtime acceptance,
macOS Actions, independent review and standalone refresh remain pending. Linux
local checks are not remote green CI. Patch/draft/manifests, binary hashes, source
reproduction helpers, logs and baseline/review failures are retained under
`.artifacts/ginux-contribution-preparation/G10/`.

No real Ginux branch, checkout/index, upstream/fork remote, human PR text or
maintainer communication changed. Source and review material remain local;
F04 user inspection precedes any publication.
