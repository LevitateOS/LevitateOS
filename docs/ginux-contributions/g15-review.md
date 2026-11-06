# G15 — owned hardlink identity; security acceptance pending

Head `8ab24d4b47adeb9897623627060846070d41219d`, tree `b5349e7d0684e0b2313141803e78e55a344e3df2`, sole local prerequisite
G13 `5c3ae8e5068b5da21d2c37f9545c0f7b10eba147`. [Retained patch](patches/G15.patch). Eleven files,+441/-72.
Source and ordinary functionality are prepared; security/full CI/current-main
independence remain pending. No G14 cwd, G12 mapping or G07 thread ancestry is imported.

## Implementation and separate self-review

Owned regular-file names refer directly to one canonical VFS data owner. Metadata,
bytes, permissions, inode identity and executable writer exclusion follow that
owner. Link counts count live names. Aliases consume namespace capacity without
allocating another file buffer. Removing the last name preserves backing through
file descriptions and executable pins; the final release returns capacity.

Descriptors retain both data identity and the entry used to open them. Duplicates
retain shared offsets; reopen_readonly retains entry identity with a fresh offset.
Executable references count both canonical data pins and per-entry name pins, so
rename/deletion of an invoked alias remains observable without recycling its slot.
No new lifetime owner or thread abstraction is introduced. Existing task/script
pin retain/release callers remain unchanged. Public FileMetadata adds links:u64;
private stat/statx encoding and device/pipe constructors are updated accordingly.
Directories/symlinks/image entries keep the existing one-link approximation.

link/linkat reuse current path translation and descriptor register restoration.
Only assigned owned regular files are linkable; linkat permits zero flags or
AT_SYMLINK_FOLLOW. No directory links, symlink-inode links, image imports or
AT_EMPTY_PATH linking are promised. Namespace/parent/capacity checks finish before
publishing an alias or changing the link count. Same-inode rename is a no-op.
The current component walker is byte-for-byte preserved.

A separate final self-review read the entire candidate diff and current VFS
allocation/open/duplicate/reopen/close, rename/unlink, pin retain/release/name,
metadata/enumeration, runtime dispatch, descriptor translation, script/ELF and
task cleanup callers. This is self-review, not independent reviewer coverage.

- **High, executable alias identity:** the donor pins only the canonical data
  entry, reporting /program when /invoked was executed. A focused ordinary VFS
  API comparison fails on exact donor9e2a39e8 and passes on the candidate through
  alias rename and deletion. Per-entry descriptor/name pins prevent reuse while
  an old alias is still referenced, even after a new alias gets the same name.
  A focused unit checks retained/released pins, duplicate versus reopened offsets,
  old/new alias names, slot exhaustion/reclamation and writer exclusion.
- **Moderate, live file counting:** the initial adaptation counted only canonical
  live names. Removing the original name left a live alias but reported zero
  overlay_files. The focused unit reproduces that failure and passes after live
  alias entries are included. Metadata and directory enumeration use canonical
  inode identity; chmod affects every alias.
- **High, capacity/lifetime preservation:** same-name whiteout reservation is
  shared with existing create/symlink behavior. Canonical data slots with live
  links, descriptions or pins cannot be reused. Alias entry refs survive FD-based
  exec even when unlinked before exec and the public CLOEXEC descriptor closes.
  Failed creation and unsupported source types preserve names and link counts.
- **Scope reconciliation:** donor overlay_entry/read_overlay and hardlink export
  joins have no consumer in this base; retain their disposition for G19/G22 rather
  than adding unused APIs. G14 cwd and G12 mapping interactions still require F02.
- **Blocking for publication:** shared mutable arguments, adversarial namespace/
  FD authority, protected copy and low-level executable identity need the
  [DaybreakBlue handoff](daybreakblue-handoff.md). Ordinary API/runtime results do
  not establish confinement or eliminate the inherited G07 security hold.

## Exact evidence

Both source and strict clean prerequisite-plus-patch export pass `make check
 test-unit build build-launch-script-fixture build-launch-owned-exec-fixture
 build-launch-directory-fixture build-launch-storage-fixture`:98 units,40 VFS,
177 matching tracked paths. All four fixture bytes and Ginux binaries match.
Clean apply uses git apply --whitespace=error. Unit/CI defaults are retained;
new hardlink modes are registered in existing harness defaults.

Selected ordinary runtime groups pass in both exports: hardlink metadata/bytes,
same-inode rename, unlink/open lifetime, relative linkat and shared chmod; quota
alias creation with a12 MiB file and last-descriptor reclamation; two hardlinked
ELF scenarios (path and unlinked FD); six existing ELF modes;16 script modes;
ordinary symlink, growth/bounds/quota and directory functional/native-cwd controls.
The directory harness runs each selected mode twice. Exact G13 rejects link86
with ENOSYS(-38), producing workload131 on the same new valid-hardlink fixture.

Donor executable-name failure and pre-fix overlay count failure, corrections,
commands and receipts are retained separately. The initial adaptation's accidental
recursive remove_name failed static validation, was corrected immediately, and its
log is retained; no warning was suppressed. Supplemental donor-name comparison is
not included in the98 unit count. Full default runtime/security cases, independent
review, macOS Actions/remote CI and refreshed upstream independence are unrun.
Linux-local success is not green remote CI.

Exact patch, draft, manifest, fixture hashes, reproduction helpers and logs are
retained under .artifacts/ginux-contribution-preparation/G15/. Intermediate helper
steps are not the final source; candidate.patch and manifest identify the final
revision. No branch, real checkout/index, remote, PR text or maintainer message
changed. F04 user inspection continues to precede publication.
