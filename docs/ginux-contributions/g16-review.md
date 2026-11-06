# G16 — unified image/owned traversal; security acceptance pending

Head `4682b0b0535d5a2f16b8ed55634ad7b3a7b26dd7`, tree `90eca06a2790cc43995a5fcc1808f4e5218ce142`, sole local prerequisite
G14 `ec6f9c480b7de187350672478500448e59a54ea3`. [Retained patch](patches/G16.patch). Thirteen files,+274/-85.
Source/ordinary preparation is retained; security/full CI/current-main independence
remain pending. No G15 hardlink, G12 mapping or G07 thread ancestry is imported.

## Scope and separate self-review

The shared component walker, image lookup, native scratch transaction, assignment
precedence, owned final-entry mutation and current null-device absence owner
already exist in G03c/G13/G14. Do not copy the donor's obsolete walker or task-group
owners. This delta removes the temporary prohibition on owned links reaching image
components, and on owned cwd/dirFD paths resolving to image paths. /dev retains
NotFound for unassigned entries; /proc and /sys remain explicitly denied.

Registered native image directory descriptors get an internal bounded tag before
path resolution. Only public-table lookup can produce that tag; the existing
tracee-root projection obtains its image name and verifies directory type. Native
fallback receives the existing canonical absolute rewrite, and saved public
arguments are restored through current descriptor/task owners. Relative execveat
uses the translated dirFD for resolution while retaining original public registers
for failure restoration. Explicit native exec validates the already-resolved path
and preserves final symlink nofollow handling; it does not reinterpret a public
image dirFD as AT_FDCWD.

Owned scripts can select an image interpreter through an owned alias or relative
virtual cwd. Only the sealed private backing's shebang is canonicalized; original
VFS bytes are unchanged. The common header reader records the payload start;
missing-newline input skips the complete original header. Existing pin/task owners
retain and release script images. On exec commit, the known native argv prefix
position is restored to the requested interpreter name, preserving nested native
arguments and original script names. Native kernel-selected interpreter chains
still require image/virtual namespace agreement; relative native interpreters from
an owned cwd remain explicitly unsupported.

A separate final self-review read the complete final diff and relevant current
walker/image lookup, device dispatch, descriptor translation/finish/reservation,
native rewrite, task entry/exit/exec/failure, script argv and private populate
callers. This is self-review, not independent or security acceptance. VFS, core,
process/native supervisor, storage backing, devices and dispatch implementations
remain byte-for-byte identical to G14. No public configuration or VFS API changes.

- **High, ownership/port correction:** keep current per-process Task and descriptor
  restore paths; no donor DescriptorGroups or second Rewrite owner. Relative
  image-dirFD exec resolves using the same capability as ordinary path operations.
  The new ordinary exec fixture succeeds with an owned cwd and image directory FD.
- **High, interpreter source preservation:** resolve only the private kernel
  execution header; retain original virtual argv names and source bytes. Five
  ordinary cases exercise absolute and relative aliases, native nested scripts,
  no optional argument, and relative interpreters from an owned cwd. Existing16
  script cases still pass. The interpreter reopens original owned script data.
- **High, canonical token boundary:** source review found that an image target
  containing shebang separators could change kernel tokenization after canonical
  rewriting. private_header now rejects space/tab/newline/NUL targets with ENOTSUP
  before publishing an execution backing. Oversized rewritten headers fail with
  E2BIG. Adversarial confirmation remains with DaybreakBlue, not claimed passing.
- **Moderate, donor reconciliation:** 93a1a56d hidden-device absence is already
  present in G13 and preserved, rather than applied twice. Donor hardlink-specific
  joins stay with G15/F02; donor image-directory exec scenario is covered by the
  new ordinary directory fixture without private-FD inspection. Old G13 full-mode
  cross-image expectations change to successful readonly behavior, with explicit
  read/close checks. Default security modes remain compiled and registered.
- **Blocking for publication:** adversarial cross-namespace authority, private FD,
  mutable arguments/races, protected scratch, implicit interpreter and exec-stack
  verification remain in the [DaybreakBlue handoff](daybreakblue-handoff.md).
  The inherited shared-argument hold remains; ordinary results prove no security
  acceptance. Combined hardlink/mapping/thread interactions remain for F02.

## Exact evidence

Source and strict clean prerequisite-plus-patch export both pass `make check
 test-unit build build-launch-script-fixture build-launch-owned-exec-fixture
 build-launch-directory-fixture build-launch-storage-fixture`:95 units,37 VFS,
177 matching tracked paths. Ginux and all four fixture binaries match. Apply uses
--whitespace=error. Existing tests/default CI coverage remain intact.

Three new ordinary directory modes pass twice each in both exports: image reads
through owned links, link-before-parent, image aliases into storage, relative
native directory-FD open/stat/statx/mkdir, owned-cwd parent traversal, native chdir,
exec through owned links and relative image-dirFD exec preserving owned cwd.
Five new ordinary script modes pass, plus16 earlier script controls, six owned ELF
modes, ordinary owned-cwd/capacity/functional/native-cwd directories and ordinary
symlink/storage growth/bounds/quota. Exact G14 rejects the owned-link image open
with ENOTSUP(-95), producing workload172 on the same valid-cross fixture.

The initial adapted static check caught an unused legacy native_exec_path; it was
replaced with a canonical-path helper preserving nofollow, without warning
suppression. Initial/final logs and intermediate helpers are retained. Full cross
confinement/hidden-device/loop/protected modes are compiled but unrun here. No
independent reviewer, macOS Actions, remote CI or refreshed standalone validation
is claimed. Linux-local success is not green remote CI.

Patch, generic draft, manifests, commands, receipts, hashes and logs live under
.artifacts/ginux-contribution-preparation/G16/. No branch, real source checkout/
index, remote, PR text or maintainer message changed. F04 remains the inspection
stop before any publication.
