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

## Observed-main refresh — 2026-11-06

Refreshed head `38f7ee402611f9023b4173eff5dc5d64b7d68375`, tree
`43cf4b65274a94d510e594a6e0b3937ac5c885aa`, on refreshed G09d
`ab47378f657b8d13d72077009107d223ba04d3a3`, incorporates observed main
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`. All added/removed lines match
the corrected historical delta; scope remains fourteen files,+1165/-131.
A separate self-review reread the entire diff, executable pins/whiteout reuse,
interpreter parser/namespace joins, native/public descriptors, exec states,
fork/death/rollback and cleanup owners. Core, native supervisor, process-table
and domain source remain byte-identical to the prerequisite. All prior VFS
coverage, including the new chunk-copy test, remains. Independent reviewer
coverage is not claimed.

Source and strict clean replay pass the Makefile's check/test-unit/build and all
applicable fixture builds:100 units,34 VFS tests. Each export passes59 ordinary
controls:49 inherited controls, four directory/cwd launches and six explicit
owned ELF modes. Exact G09d fails the identical ordinary-path fixture at
workload exit128. The83,288-byte program reads its64 KiB initialized data after
loading, preserves public descriptors on missing-file failure, commits CLOEXEC
retirement on successful replacement and releases its executable pin on image
reexec. All211 tracked paths match the head and both binaries have SHA256
`238cfb94f1c73eac57258309347dfccf15c8756836477c18160a3a92c3aa14fa`.

G10's native-exec fixture now unconditionally performs protected-pointer and
readlink probes. The inherited ordinary task/exec helper therefore uses unchanged
G09d image-fixture bytes (SHA256
`6ca9887c748ca9ead4c70f2a4c597cd8f725eeda0edeb462b33fc709aea4bb6b`),
recorded per invocation, instead of running those probes. The contributed full
fixture/CI remain untouched and pending. Exact logs, receipts, patch, local draft
and manifests are in
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G10/`.

A fresh upstream check then found main advanced to
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a` via PR#231, adding configurable
contracts/audit. The results above are exact8a475e8b-composition evidence, not
latest-main acceptance. Reconcile this new upstream owner before proceeding to
G11. Shared kernel-argument capture, malformed input, seal/private-FD/readlink/
protected runtime, full CI/macOS, actual upstream prerequisite merges and final
standalone delivery remain pending. No security agent was dispatched, branch
created, remote text edited or PR opened.
