# G14 — per-process owned cwd; security/thread joins pending

Head `ec6f9c480b7de187350672478500448e59a54ea3`, tree `8bb7106066f2fdfdbbc1f2e2a872b3c1cdf2ade7`, sole local prerequisite
G13 `5c3ae8e5068b5da21d2c37f9545c0f7b10eba147`. [Retained patch](patches/G14.patch). Five files,+252/-34.
Per-process source and ordinary functionality are prepared. Full CI, security,
current-main independence and the later G07 thread-sharing join remain pending;
this is not publication-ready. No G12 mapping dependency is imported.

## Implementation and separate self-review

A configured Task retains a private VFS directory descriptor. The filesystem
borrows the calling task's cwd at each syscall entry, including exec replay;
it does not own that reference. Owned chdir/fchdir replace the reference only
when a new directory is valid, fork duplicates it, exec keeps it, and task death,
failed reservation/reap or teardown releases it through the existing task owner.
Directory rename changes getcwd's reported path; removed cwd returns ENOENT for
getcwd/relative operations and can be recovered with an absolute directory change.

Native cwd changes preserve current path-scratch handling and original public
arguments. Only a successful native chdir/fchdir clears the owned reference.
Native getcwd is projected through the existing image-cwd owner; owned getcwd
returns its VFS name with the normal NUL-inclusive byte count. Private cwd
references consume the existing64 VFS description capacity, without new public
FD budgets. Relative owned script and interpreter resolution uses that cwd.
Native kernel-selected relative interpreters from an owned cwd fail explicitly;
they must not use an unrelated native cwd. General relative image fallback remains
with G16. Initial native workdir behavior is preserved until an owned cwd is selected.

A separate final self-review read the whole candidate diff and existing task
entry/exit, native rewrite, FD classification, VFS directory identity/duplicate,
exec/script pins, fork failure, wait/death and teardown callers. This is
self-review, not independent reviewer or security acceptance. Current VFS,
descriptor owner, core/process/native supervisor, script and storage backing
source remain byte-for-byte identical to G13.

- **High, port lifetime correction:** adapt donor DescriptorGroups to current
  per-process Task ownership instead of importing unready thread support.
  launch_tasks.rs:1418 releases cwd with other task references on all existing
  cancellation/death/reap/teardown paths. The donor's thread-only fixture hunks
  are retained as held-thread-join.patch for G07 and are not claimed to run.
- **High, fork rollback:** reserve copied cwd after the current descriptor/image
  snapshot. A cwd-capacity failure releases copied descriptor refs and the extra
  executable pin, cancels the virtual child and returns the real error before
  any native fork. A normal capacity fixture fills63 public owned directory
  handles alongside one private cwd, then performs32 failed fork reservations
  (more than the16 task slots). Freeing one slot permits four successive fork/
  wait/exit cycles reusing that slot. Final descriptor and process teardown passes.
  This exercises resource recovery without malformed pointers or race probing.
- **High, native commit semantics retained:** native chdir/fchdir restores public
  arguments and clears cwd only for rax0; failures keep the old reference. Ordinary
  tests confirm regular-file native chdir/fchdir failures preserve the owned cwd.
  Current native scratch restoration and strict typed wait/copyout are preserved.
- **Moderate, interpreter selection correction:** fs_hooks.rs:615 uses the selected
  virtual cwd for manually resolved owned interpreters. Native relative interpreter
  lookup under owned cwd is rejected rather than interpreted in a different kernel
  directory. Ordinary owned-script execution with a relative owned ELF interpreter
  passes; adversarial interpreter authority remains unrun.
- **Moderate, fixture preservation:** use explicit new mode/child names instead
  of the donor's any-argument exec-child shortcut, which would shadow current
  functional/cwd scenarios. Existing default directory checks remain intact;
  new full getcwd bounds/protected-buffer checks are compiled and registered but
  unrun here. No upstream tests, CI steps or shared-thread fixture evidence is removed.
- **Blocking for publication:** shared-argument races and low-level cwd/path
  authority, protected-copy and interruption validation remain with the
  [DaybreakBlue handoff](daybreakblue-handoff.md). Thread-shared cwd is a held
  G07 join; ordinary per-process results do not establish that capability.

## Exact evidence

Source and strict clean prerequisite-plus-patch export both pass `make check
 test-unit build build-launch-script-fixture build-launch-owned-exec-fixture
 build-launch-directory-fixture build-launch-storage-fixture`:95 units,37 VFS
 tests,177 matching tracked paths. Source/clean Ginux and all four compiled
 fixture bytes match. The existing directory CI target now includes new full
 and ordinary cwd modes; all existing units/default fixture routes remain.

Two new selected ordinary cwd modes pass twice each in source and clean replay.
They cover chdir/fchdir/getcwd, relative file creation, independent child cwd,
renamed/removed directory identity, native cwd recovery, failures preserving cwd,
and cwd through native ELF, owned ELF and owned-script exec. The relative script
interpreter and script name/optional argument are checked. Capacity rollback and
reclamation use the bounds described above. Existing directory functional/native
cwd, ordinary symlink/storage growth/bounds/quota,16 script and six owned-ELF
controls also pass on both exports. Exact G13 rejects owned chdir with ENOTSUP,
causing workload code121 on the same focused valid-cwd fixture; G14 passes.

Full/protected getcwd and host-path/race/authority cases, thread sharing,
independent review, macOS Actions/remote CI and standalone refresh remain unrun.
Linux-local success is not green remote CI. Adaptation helpers include intermediate
steps; the retained patch/manifest and clean apply/build helper identify the exact
final source. Earlier reviewed exports/logs are retained with pre-final-budget or
pre-cwd-reclaim-check names; final clean evidence belongs to ec6f9c48 at
/tmp/ginux-contribution-preparation/G14/verified-head-reclaim.

Patch, draft, manifests, fixture hashes, commands, receipts and baseline logs are
retained under .artifacts/ginux-contribution-preparation/G14/. No branch, real
checkout/index, remote, PR text or maintainer message changed. F04 user inspection
continues to precede publication.
