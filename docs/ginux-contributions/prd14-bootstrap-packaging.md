# PRD14 — bootstrap, packaging and fresh host

Status: active and incomplete. Maintained source/released-input acceptance still
depends on PRD01/PRD05. Upstream-first remains selected; the user chose to keep
the local integration unpublished. Local source archives and private caches
cannot satisfy the final durable-source criterion.

The user authorizes fresh downloads of pinned Rocky OCI/cloud inputs, the twelve
RPMs and official verification metadata, plus required tool installation only
inside a disposable session-libvirt VM. Existing Rust/crate dependency download
approval remains applicable. No push, publication, hosted job or downstream
source adoption is authorized. macOS is indefinitely deferred.

Starting Recipe: `d4801b112326c3793efb0b902aade27f03ca014e`. Starting local Ginux:
`a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`. Neither is an approved remotely
obtainable deliverable. Bootstrap owners are Ginux's OCI fetch/prepare scripts
and Recipe's `recipe-image`, with the two checked-in bootstrap manifests.
The current acquisition manifest has twelve RPMs; the historical eight-RPM
`c089f95c…` test image must not be substituted for fresh composition.

Current work: acquire into a new empty task directory; verify every publisher
input and composed provenance; test producer failure preservation; build and
inspect actual crate/binary/RPM outputs; use a disposable session-libvirt host
with normal trust and empty caches for setup and execution. Any provisional
source use will be labeled explicitly. Actual required-source acquisition and
complete deliverable acceptance remain pending maintained source.

Artifacts: `.artifacts/ginux-contribution-preparation/prd14/` (new; earlier
PRD13 receipts remain sealed and unchanged). Preserve unrelated staging, edits
and other chats' domains/worktrees. Do not run Recipe unit tests or rebuild
product kernels.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload the complete skill, required
reference, applicable AGENTS and this checkpoint after context shifts. Root
instructions supersede skill process requirements. Exact next action: fetch
the pinned public bootstrap inputs and inventory clean-host/package prerequisites.

## Current findings and progress

Fresh clone of public upstream main remains `53a54addea67969abaa240ea478def1b94130eee`.
Fresh official current BaseOS/AppStream metadata matches all twelve pinned RPMs;
the official Rust channel manifest matches both archives, and the published cloud
checksum matches the newly downloaded qcow2. Acquisition began with an empty
input directory; no old global input cache was reused.

F14-1: Recipe source excluded Cargo.lock, and an actual new standalone
`cargo build --locked --offline --release --bins` failed. Commit `1e83d06` retains
the independently resolved/tested release lock and adds separate disposable
unlocked stable/MSRV CI builds. The locked source build now passes.

F14-2: Public Ginux prepare's fallback host-tool compilation inherits native
`-nostdlib` flags and fails linking standard libraries. Explicitly prebuilding
`ginux-vfs-index` with `RUSTFLAGS=''` and supplying the existing `GINUX_VFS_INDEX`
override works, without patching or depending on an unpublished assembler.

F14-3: The recorded derived base pin `1026f8d3…` is not reproduced by that declared
public producer. Two independent public assembler outputs are both
`b93bfd0f6deea2a7a96216655cda3ebbd87a445231548ff491ee30aed20a36ab` from the exact
pinned OCI objects. The verified raw layer is `f0c7ff1a…`; upstream intentionally
materializes layer hardlinks. No old private base is substituted. Both manifests
and the default bare-root reference are corrected to the actual upstream output;
image and affected execution validation are in progress.

The transient VM `rocky-prd14-fresh-host` is running on `qemu:///session`, with
localhost-only SSH, task-owned disk/seed and 6 GiB RAM. It started with no Cargo
registry or rustup cache. Signed ordinary Rocky tools install successfully. The
first package-list attempt failed because Rocky provides the rpm2cpio command
without an rpm2cpio-named package; that failed log and corrected setup are both
retained. No fixture CA is introduced. Final cleanup must destroy only this domain.

Exact next action: finish fresh source image composition; verify package signatures
and image reproduction in the VM; validate crate/binary/RPM and hello/build/restart
with explicitly labeled provisional candidates. Maintained-source delivery remains
unavailable and cannot be replaced by these private candidate transfers.
