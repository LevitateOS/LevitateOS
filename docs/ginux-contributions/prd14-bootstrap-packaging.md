# PRD14 — bootstrap, packaging and fresh host

**In progress: maintained-source reproduction.** On 2026-11-08 the user
explicitly authorized keeping the working version in our own repositories,
keeping upstream Ginux leading, and completing PRD14 against it. This replaces
the earlier local-only source-delivery hold. Publish the prepared history to
`LevitateOS/ginux:main` and `LevitateOS/recipe:master`, then verify fresh network
checkouts and a disposable Rocky host. No upstream PRs/comments, package
publication or hosted job dispatch are authorized. macOS remains indefinitely
deferred. Product kernels are not rebuilt.

The source, packaging and execution receipts below describe the completed
preparation run. They remain bound to those exact revisions; the new obtainable
source run will have separate receipts before PRD14 is marked complete.

## Sources and identities

| Component | Exact identity |
| --- | --- |
| Recipe candidate | `124f378063ce449e3799aad9b43dfac9404754f1` |
| Executed Recipe core | `2e20a031268309b7016898ff71a005b07b995819`; all final Rust bytes unchanged |
| Public OCI assembler | Fresh upstream clone `53a54addea67969abaa240ea478def1b94130eee` |
| Provisional configured runtime | `a498e4f6148d5d94ce4ba8edb79588b7883f6a0f`, production `f7f0d14c` |
| Runtime binary | `0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3` |
| RPM/archive coordinator and matching worker | `506397afa70f3bd717da7c1c1227e1233bdd8a2607bafbd92a6e73fe60e5d97b` |
| RPM/host image producer | `7de227aa9a387f59638e42a826f566c7ee37e23828fe1fcb793ec309ee27455f` |
| VM source/crate-installed coordinator and worker | `e7cf94abfe426c61135e9ea63e362fe504f77edea97498bc602fe3889bb83c1d` |
| VM source/crate-installed image producer | `3f108d729ca73a6f3a71a8891548b45c0c49b3397a226e115b281894bf5f0459` |
| Release Cargo.lock | `48c776bdeb1cf63378e7132fd1dfbc1a819db09a95a23fa3f7ecb1b676e85f65` |

The source and crate/RPM metadata changed after core execution; source and binary
comparisons explicitly establish equivalence. The original and completed RPMs
contain identical executable bytes. No passes from different runtime/worker/
image combinations are combined into a full-suite claim. PRD10/PRD13's earlier
152-case results remain bound to their recorded `c089…` image and source.

## Fresh bootstrap reproduction

Acquisition began in a new empty task-owned directory. Current official
BaseOS/AppStream primary metadata was fetched, its SHA-256 checked against
repomd.xml, and all twelve pinned RPM hashes matched. Both Rust archive hashes
matched the official dated 1.99.0 channel manifest. The newly downloaded cloud
image matched the official checksum and tracked pin. OCI index, manifest,
config, layer and uncompressed diff identities were verified by upstream's own
fetch/prepare producers. No old global input/image cache supplied these bytes.

The fresh Rocky 10.2 VM started without a Cargo registry or rustup cache. It
used its normal CA store and stock Rocky signing key to verify all twelve RPM
signatures. It independently cloned/built the public assembler and downloaded/
prepared the OCI objects. The resulting base matches the host byte-for-byte.

| Derived output | SHA-256 |
| --- | --- |
| Public assembled base | `b93bfd0f6deea2a7a96216655cda3ebbd87a445231548ff491ee30aed20a36ab` |
| Compiler image | `a8edd8faecd8125a19a70356e8969209f9f78efcf1333c250de6c6afc1807776` |
| Acquisition/musl/ISO-tools image | `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15` |

Both compiler and acquisition images match across host and VM producers, and
again when composed by the installed RPM producer with **PATH=/usr/bin:/bin**.
That last path excludes the installed Rust toolchain/private build outputs.
Host Rust is 1.99.1; VM Rust is the verified 1.99.0 SDK; both use RPM 4.19.1.1.
Every provenance input/hash/size is checked against the selected manifests and
actual outputs. The image producer is always built from current source or
installed from its actual package; no migration executable or fixture CA is used.

## Findings and concrete dispositions

| Finding | Disposition |
| --- | --- |
| F14-1: Fresh source cannot build with --locked because Cargo.lock was ignored | Reproduced exit 101 in a new external export. `1e83d06` retains the independently resolved/tested lock; the locked build passes. Separate fresh unlocked stable/MSRV CI exports preserve independent resolution coverage. Workspace Cargo.lock was not replaced. |
| F14-2: Upstream prepare's fallback assembler inherits native -nostdlib flags | Initial standard-library link failure retained. Build the existing public host assembler with RUSTFLAGS='' and use its GINUX_VFS_INDEX override. Both environments reproduce successfully without an unpublished assembler patch. |
| F14-3: Recorded derived base pin 1026f8d3… is not declared public producer output | Two independent host assemblies and the fresh VM all produce b93bfd0f… from the exact pinned OCI objects. The verified raw layer is f0c7ff1a…. `2e20a03` corrects both manifests and the default bare-root reference; it completes all twelve input URLs. Default-bare execution and wrong-root identity rejection pass. No private old base was substituted. The historical B21 default-image finding now has a reproducible pin; bare-image lack of compilers is explicitly documented. |
| F14-4: Rocky has no rpm2cpio-named package | Failed package-list attempt retained; corrected setup installs normal signed tools and checks both rpm2archive/rpm2cpio commands. |
| F14-5: RPM omitted required image producer/bootstrap material | `124f378` adds recipe-image, the rpm converter dependency and setup/author docs, manifests and examples. Automatic ELF requirements stay enabled. The real RPM reinstalls with DNF, rpm -V passes, both files match built binaries, and its producer independently reproduces both images as the ordinary operator. |
| F14-6: Documentation overpromised no final directory after every producer error | Corrected to the actual atomic non-replacing publication contract. Pre-publication failures leave no fresh output; a late error after rename may leave a complete fresh directory and callers must reject non-success. No rollback/late-fault runtime acceptance is fabricated. |
| F14-7: Output usability for the actual privileged operator | Image outputs are operator-owned. Coordinator state/logs/generations are root-owned/private. Privileged inspection/execution works; an explicit administrator copy produces an ordinary-user-owned executable which runs without sudo. No unsafe SUDO_UID-based chown or rootless promise is introduced. |

The real producer's existing-output, wrong-input-hash and missing-converter
negative controls all preserve both valid images/provenance, leave no fresh
failed publication, and clean their private temporary directories. Raw commands,
errors and before/after hashes are in `failure-controls/results.json`. Signal
interruptions may leave private temporary data as documented; automatic signal
cleanup and exhaustive I/O fault coverage are not claimed.

Probe-authoring failures are preserved separately: an unquoted `package` key
is reserved in Rhai, and unconditional build readiness correctly bypasses lower
acquisition phases. Corrected acquisition probes use `artifact` and source-based
readiness. Neither is hidden as a product failure/pass or bypassed by an engine
change. An initial ownership probe's stdout field was overwritten by its local
variable reuse; `user-export-verification.json` independently captures the
actual successful ordinary-user execution.

## Actual execution and packaging outcomes

The supported host exercised is stock Rocky 10.2, x86-64, kernel 6.12, glibc 2.39,
under transient `qemu:///session` libvirt. No direct QEMU launch was used.

- Actual crate packaging/verification, source-crate cargo install of both bins,
  release binary archive inspection/execution and RPM generation/requirements/
  installation/verification pass. Local RPMs are explicitly unsigned candidates;
  their integrity checks do not constitute release authentication.
- Thirteen fixed-tuple requirements pass: hello install/restart/readiness; real
  offline GNU Cargo build, joined-thread execution and cache/state reuse; actual
  pinned public Rocky HTTPS and upstream Git HTTPS acquisition, restart and
  readiness; and fresh denied-network acquisition. The successful public
  acquisitions use stock publisher trust, not loopback servers or fixture CAs.
- An external Rust public-library consumer built against the extracted crate
  launches the configured worker even with spoofed worker environment flags;
  its actual runtime path is /inputs. The source-crate-installed CLI and binary-
  archive CLI each execute with their matching worker.
- The exact versioned threaded example passes install/restart/readiness. A real
  musl-target Cargo build and statically linked program execution also pass;
  readelf confirms no INTERP. Cargo locks, default linker threading, metadata
  validation and isolation remain active.
- Root default selection uses the actual public-derived bare base; an incorrect
  image digest fails. Root-private output ownership and explicit user export are
  recorded, not silently presented as ordinary-user accessibility.
- Package fmt, strict Clippy including all three integration targets, and actual
  locked MSRV 1.93 checking pass after the base correction. No Recipe unit tests
  or unit doctests were added or run.

See Recipe [Linux setup](../../tools/recipe/docs/setup-linux.md), its
[bootstrap input guide](../../tools/recipe/bootstrap/README.md), and the tested
[threaded example](../../tools/recipe/examples/threaded-hello.rhai). The binary
archive contains the coordinator/worker; the completed RPM and source crate
provide its image producer. All roles and privileges are explicit.

## Evidence and remaining required input

Evidence root: `.artifacts/ginux-contribution-preparation/prd14/`.
The final SHA-256 manifest verifies all **587 regular files**, with four recorded
symlinks and zero special files; build caches and .git stores are excluded while
exact source tar archives are retained. Manifest SHA-256:
`d16e82728cfeb3fb878a09b074d23444ccc07768bafea088464d3a126effeae9`.
It contains
fresh acquisition records and publisher metadata, the initial failures and
corrections, public assembler sources/logs, exact candidate archives, both
produced images/provenance, package artifacts and metadata, static logs,
source/binary equivalence, all VM scripts/transcripts and retrieved state.
`final-source-packaging.json` pins current source and artifact hashes.

Canonical final VM evidence archive SHA-256:
`487146afb9eb07fa47e6b683746c32027a089e213077778800f989b40a771f5f`.
It was independently verified before the disposable host was destroyed. Its
312 members retain original metadata; `vm-evidence/` is a safe data-filtered
review view, not a claim to preserve the guest's UID/GID on extraction. The
canonical archive and recorded stat/ownership receipts are authoritative.
The task domain, owned writable disk, NVRAM and private SSH key are removed;
other domains were preserved. Publisher inputs/base image remain retained.

The missing required input is an approved, remotely obtainable maintained
Recipe/Ginux source pair with the integrated capabilities. A private candidate
transfer intentionally tests preparation; it is not fresh release-source
acquisition. Under the user's keep-local direction, do not publish as a shortcut,
select an unapproved maintained fork, or ask again for the same declined push.
When maintained source becomes available, reconcile PRD01/PRD05, rebuild the
actual delivered pair and repeat affected setup/consumer/packaging verification
on that exact source/toolchain/image combination. These receipts do not close
final-source CI, wider distro/product or PRD09 security acceptance.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload it, its required reference,
applicable AGENTS and this checkpoint after context shifts. Root instructions
supersede skill process requirements. Exact next action: wait for maintained
source availability as directed; retain PRD14 unchecked and all publication
permissions unchanged.
