# G19 — flush and quiesced final export

Head `a36e76441b5a7c1a30b3461c9537bb3c6aee9d8e`, tree `a6768f59733e746cc5e92afcf982ef686f5b22d4`, on G12
`83072ec3ddab3e4712ce8db9ba232a10342557de`. [Patch](patches/G19.patch): ten files,+527/-24.
Per-process source/ordinary preparation is retained; thread/security/full CI and
actual upstream independence remain pending. The real fsync consumer uses G12's
memfd backing. Core export alone does not inherently need mappings; no G18/G17
budget, G13/G15 archive identity or held G07 thread ancestry is imported here.

Owned regular-file fsync/fdatasync reaches its supervisor descriptor, including
write-only and empty files; virtual directory sync succeeds without native path
access. Additive VFS descriptor_storage/overlay_entry/read_overlay APIs keep
backing authority and descriptor-free export reads in the existing owner. No
configuration schema or typed process/native supervisor change is introduced.

The CLI adds --export-file alongside the existing optional result path. After
successful root exit, existing teardown kills/reaps remaining processes, releases
descriptions and executable refs and verifies no active virtual state before
reading live file/directory entries. Export completes before filesystem teardown;
failed export still tears down owned storage before reporting supervision error.
Unsuccessful root completion cancels export. Thread-sharing joins remain held.

Separate final self-review read the full diff and current CLI/config rejection,
completion, task teardown, FD translation, backing sync/read/clear, VFS slot/pin
lifetime and archive callers. This is self-review, not independent/security review.

- **High, partial publication corrected:** donor48b26f8d creates/writes the final
  path directly. The candidate creates a mode0600 same-directory staging file,
  writes the complete tar, syncs and closes it, then publishes with
  RENAME_NOREPLACE. Existing destinations are preserved. All regular failure
  paths remove staging; abrupt supervisor death may leave a private staging file,
  never a valid published output. Hostile parent/staging replacement and injected
  write/sync/close/cleanup failures require DaybreakBlue validation.
- **High, early rejection cleanup fixed:** final review found launch_rejected
  bypassed the initial startup cleanup. A known unknown-section config reproduced
  retained staging on4e2ab6c8. Moving cancellation to write_completion_result before
  its optional-result early return fixes it at the common completion owner. The
  same check now passes; late unsupported-assignment rejection also cleans up.
- **High, quiescence:** an ordinary child writes/syncs a file, signals readiness
  over an owned pipe and blocks. Root exits0; teardown settles it before capture.
  Its file is exported and its post-block write is absent. This demonstrates
  supported process settlement, not hostile race or thread acceptance.
- **Owner reconciliation:** c541a67f's backing c-string and readlink error fixes
  are already in prior owners; module ordering stays with this export wiring.
  Symlink/hardlink, scalable namespace and restore joins remain G13/G15/G18/G21/G22
  and F02 work. No unused checkpoint/control protocol is introduced.

Source and strict clean prerequisite-plus-patch replay pass make check test-unit
build and storage/directory/script/owned-exec fixture builds:92 units,34 VFS,
179 matching tracked paths. Ginux and all four fixture binaries match. Clean apply
uses --whitespace=error. Seven ordinary cases cover flush, binary/mode/long-path/
empty-file export, no output before completion, settled child writes, failed
workload cancellation and both early/late config failure. Existing destination
retry preserves content. Two valid mapping, six ELF,14 script and growth/bounds/
quota controls also pass in source and clean copies. Exact G12 returns ENOTSUP
for fsync, causing workload182 on the same valid flush fixture.

The initial archive check passed; the later config-rejection regression and fix
are retained separately. A harness indentation edit error was corrected and its
log preserved. Prior head/clean evidence is labelled before-completion-cleanup;
final evidence belongs to a36e7644. Existing units/default CI/security fixtures
remain intact. Security/path/FD/failure injection and thread acceptance,
independent review, full Linux suite, macOS/remote CI and refreshed standalone
validation remain unrun. Linux-local success is not green CI.

Patch, generic draft, commands, receipts, hashes and helpers are retained under
.artifacts/ginux-contribution-preparation/G19/. [DaybreakBlue handoff](daybreakblue-handoff.md)
records adversarial work without dispatch. No real branch/checkouts/index/ref,
remote human text or maintainer message changed. Publication waits for F04.
