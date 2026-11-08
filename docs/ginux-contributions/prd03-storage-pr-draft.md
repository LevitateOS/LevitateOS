# Draft: feat(launch): add bounded assigned memory storage

Local draft only. Do not publish against current main. The feature-only range is
`f1c3d3a7727f5a8027b448b4dcad0ad52c896c4c..9802ef036d53ae1ee5276fe59cc719389dda5879`.
See the [review and full dependency record](prd03-storage-review.md) before
choosing a publication base. Refresh this text after prerequisites merge.

---

Configured launch currently rejects storage assignments. Add explicit
`memory:/absolute/path` assignments that provide empty writable namespaces in
supervisor memory, hiding image contents below each assignment and discarding
all data at teardown. Writes never create host files.

The VFS and its real launch consumer ship together. Memory files support bounded
I/O, positioned I/O, shared offsets/append across duplicates, metadata and mode
changes; directories support directory opens, relative `*at` calls,
chdir/fchdir/getcwd, getdents64 and empty-directory removal. Copy-up preserves
inode identity and existing readers' view. Directory metadata and file data use
separate capacity accounting. Invalid paths, permissions, copy faults and
capacity failures have explicit errors and regression coverage.

Storage runs after the resolved syscall/semantic contract, retaining deny
precedence and audit ownership. It explicitly selects catalog 3, which adds
fchmodat to configured catalog 2; saved catalogs 1 and 2 keep their original
authority and digests. Nonempty assignments with an older catalog fail startup.

This depends on #232, the latest #233, #238, #243, #236, the separate zero-fill
fix and the separately reviewed explicit configured-catalog versioning change.
The current tested composition is based on main `53a54add`; it is not yet a
standalone PR against that main. #234 is independent of this single-task storage
foundation. Preserve the prerequisite fixes rather than folding unrelated
process work into storage.

Validation on Linux/x86-64 at `9802ef03`: `make check`, all 138 unit tests,
catalog-doc check, complete CI fixture/release build, and every launch-integration
command pass. The storage suite covers 40 launches, including contract denials,
older-catalog refusal, directory lifecycle, shared append, mode enforcement,
failed truncation without data loss, assignment confinement, partial copies,
capacity, fresh teardown and the 2 MiB stack case. Four focused red controls fail
on their preceding sources and pass on the candidate. Existing upstream tests
and CI jobs remain intact.

Limits are deliberate: memory modes use common read/write/search classes and
fixed umask 022; full Linux DAC/ACL and metadata fidelity are not promised.
Eight assignments, 32 namespace objects, 64 KiB per file and 64 memory descriptors
remain fixed. Memory links/rename, target-number dup2/dup3, mappings/execution,
persistence/generations/export and process supervision are outside this feature.
Directory cookies assume an unchanged namespace; removed memory cwd operations
fail ENOENT. The README documents path, syscall and unsupported-operation limits.
