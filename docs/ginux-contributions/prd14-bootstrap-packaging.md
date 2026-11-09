# PRD14 — reproducible bootstrap, packages and fresh host

**Complete on the maintained source pair.** The selected maintained sources are
Ginux `28f4368f6fd70d0e485386892a08d6734b5988dd` and Recipe
`e7bfc166343164989794c917e67f187d0ac49ba8`. See
[the preserved baseline manifest](delivery-sources-2026-11-08-baseline.json)
for this exact source/artifact selection. [PRD16](prd16-delivery.md) owns the later
corrected source delivery; these fresh-host/package results remain historical
and are not relabeled as testing its new sources. Our repositories supply both histories; no upstream PRs/comments,
crate publication, release tag, package publication or hosted workflow dispatch
were performed. macOS is indefinitely deferred.

The user explicitly authorized maintaining our working version in our own
repository, keeping upstream Ginux leading, and including Gate 11 PR #249.
Ginux contains every commit of current upstream main `53a54adde` and PR #249
head `9759daaec`. The last refresh found those unchanged. PR #249 is still open;
its performance gate is partial and no accepted performance baseline is claimed.
New upstream changes remain authoritative; replace/drop downstream changes as
upstream supplies their behavior. This replaces the previous source-delivery
hold. At that baseline delivery, superproject gitlinks and HTTPS source URLs selected
the obtainable pair. The current selection is now owned by PRD16.

## Exact fresh-host combination

The new transient `qemu:///session` domain `rocky-prd14-delivery` used the verified
stock Rocky 10.2 cloud base, kernel 6.12 and glibc 2.39. Both repositories were
cloned/fetched over public HTTPS inside the guest, without credentials, local
source archives, Git alternates or developer Cargo caches. Its registry/rustup
caches were absent initially. All fourteen RPM/Rust inputs were downloaded anew
from their documented publishers; all twelve RPM signatures passed with Rocky's
stock key. The dated Rust channel manifest matched both SDK/component hashes.
OCI objects were independently fetched, verified and assembled by the maintained
Ginux scripts, with no prebuilt helper override. No product kernel was rebuilt.

| Role | Exact identity |
| --- | --- |
| Ginux launcher | `360d22582219d6910fe616d724d4dfb690af24fd891926fa422e4283b6e34cb6` |
| Recipe coordinator and matching worker | `901543ff7462db3348371eee1f84fa9e3a9f056e43b3e48f21a5beac3c39bbc1` |
| Recipe image producer | `104cce02442d530e7d8e62ad902ec78206ecf40639d8a4c72166ee8395720c59` |
| Standalone Cargo.lock | `48c776bdeb1cf63378e7132fd1dfbc1a819db09a95a23fa3f7ecb1b676e85f65` |
| Assembled public base | `b93bfd0f6deea2a7a96216655cda3ebbd87a445231548ff491ee30aed20a36ab` |
| Compiler image | `a8edd8faecd8125a19a70356e8969209f9f78efcf1333c250de6c6afc1807776` |
| Acquisition/musl image | `ffe04d8671e45036ac77acc958fb6cfd3c24a939075dc7e6c5437b69d7f05c15` |
| Source crate | `0e3d19bba5dee6d9eaade4ce171336181c15c48e70376a3af250659c5c883c4b` |
| Local RPM | `dcff3100c1901445d2e20af479dee5952922be1a590e5b7ff4b8cc3afab9d67e` |
| Local binary archive | `46a49861cf771f4b3b888d58f7b205fa99f5a9aff7178e6afe1893b7d0be19e8` |

The guest used the pinned official Rust 1.99.0 GNU SDK and RPM 4.19.1.1. Both
images match the independent prior host/VM producer outputs byte-for-byte. The
installed RPM producer reproduces them again with **PATH=/usr/bin:/bin**, as the
ordinary operator, without Rust or private producer binaries in that path. Every
provenance input/hash/size matches its manifest and actual input/output bytes.
Fresh produced images and package artifacts were copied back and rehashed.

## Required outcomes and evidence

New evidence root: `.artifacts/ginux-contribution-preparation/prd14-delivery/`.

| Required outcome | Exact-source evidence |
| --- | --- |
| Obtain/build the maintained source without private worktrees | `vm-runtime-build.log`, corrected `vm-runtime-verify.log`, `vm-recipe-build.log`, source availability receipts and final clean-source identities |
| Public pinned inputs and ordinary publisher trust | `vm-inputs.log`, `public-inputs.json`, dated Rust metadata, stock RPM signature checks; independent OCI verify/assemble output |
| Real compiler/acquisition production and provenance | `vm-producer-controls.log`, both retained `fresh-images/*/rootfs.tar` and provenance; installed-RPM reproduction in `vm-followups.log` |
| Failure preservation | Existing output, wrong input digest and missing converter all fail; both prior valid images/provenance stay unchanged, no fresh failed output or private temporary directories; `failure-controls/results.json` |
| Crate, binary and RPM packaging | `cargo package --locked` verifies the real crate; source-crate cargo install builds both bins in a separate empty target; actual RPM/DNF install and rpm -V pass; required recipe-image/docs/manifests/examples/converter dependency and automatic ELF requirements remain |
| Hello/build/restart/public network | `operator/execution-receipt.json`: all thirteen requirements pass on one tuple, including hello, actual GNU Cargo build and six joined threads, readiness/reuse, pinned public HTTPS and Git acquisition, and fresh denied-network acquisition |
| Actual musl program and installed producer | `rpm-complete-receipt.json`: static ELF has no INTERP, runs with `threaded-hello:42`, install/reinstall/readiness preserve one build/install; normal locks/linker threading remain |
| Matching worker, public library and package execution | Real extracted-crate external Rust library consumer and installed source-crate/binary-archive CLI executions in `vm-followups.log`; matching worker selected explicitly for each CLI |
| Operator ownership and defaults | Ordinary image outputs; root-private coordinator state/generations; explicit administrator copy executes as ordinary user; default bare root succeeds and wrong root digest fails; ownership/default and user-export receipts |
| Catalog migration regression | All eleven NEEDS integration cases pass on the final pair, plus library_backend and image_producer; Ginux's 47 configured allow/deny cases and four native dispatch controls pass |
| Build/static regression | Ginux make check, 140 retained unit cases, contract-doc and benchmark-report checks pass. Recipe fmt and strict Clippy for lib/bins/three integration targets pass. No Recipe unit tests or unit doctests were added or run |

## Findings and disposition

| Finding | Concrete disposition |
| --- | --- |
| Fresh source lacked a tracked release lock | `1e83d06`, retained in Recipe e7bfc166: standalone lock and locked fresh builds; independent unlocked CI resolution coverage retained |
| OCI fallback helper inherited freestanding linker flags | Ginux `a538e178`, retained in 28f4368f: build the std host tool with RUSTFLAGS=''; fresh guest prepares the base without GINUX_VFS_INDEX or a temporary helper binary |
| Historical base pin 1026f8d3 did not reproduce | Recipe `2e20a03` pins actual public-derived b93bfd0f in both manifests and default bare root. New fresh OCI assembly agrees; wrong root identity is rejected |
| Incorrect Rocky setup/package contents | Normal signed tools provide rpm2archive/rpm2cpio. Recipe `124f378` supplies both bins, rpm dependency, bootstrap material and docs/examples, retaining automatic ABI dependencies. Actual package installation and verification pass |
| Overpromised failure/output ownership | Documented real publication boundary and explicit privileged coordinator/user export. The real producer's bounded failures preserve valid outputs; ordinary user execution after explicit export passes |
| Gate 11 owns upstream catalog number 2 | Retired the conflicting local number. Explicit downstream catalog 1002 contains upstream 2 plus the ten configured forms, 193 total operations. Recipe migrates rendering, pinned reviewed network/non-network digests, tests and documentation. Known Recipe callers select the new catalog explicitly and reject incompatible digests |
| Gate 11 dispatch parser reused a local field bit | Dispatch moves to unused bit 57, preserving storage/network fields and strict duplicate rejection. All 25 instance tests and round trips pass |
| Gate 11 explicit yield denial failed startup | Reproduced on the disposable VM. Only allowed mediated yield creates a notification broker; denied yield follows the ordinary contract owner. Retained regression verifies allowed and denied yield in selective and paranoid modes |
| Probe/command mistakes | Preserve the nested Cargo workspace refusal, nonexistent redundant fixture target, stdin-only shell wrapper that ran no child checks, and missing required spoof environment on the first library harness attempt. Corrected commands execute successfully on unchanged candidate source. Unrun/failed attempts are not passes; successful NEEDS cases were not needlessly repeated |

## Reproduce

Use the supported [Linux setup](../../tools/recipe/docs/setup-linux.md),
[bootstrap input URLs and manifests](../../tools/recipe/bootstrap/README.md), and
[versioned threaded example](../../tools/recipe/examples/threaded-hello.rhai).
Start outside another Cargo workspace, install the documented signed host tools
and verified official SDK, then obtain the exact pair:

```sh
git clone https://github.com/LevitateOS/ginux.git ginux
git -C ginux checkout --detach 28f4368f6fd70d0e485386892a08d6734b5988dd
git clone https://github.com/LevitateOS/recipe.git recipe
git -C recipe checkout --detach e7bfc166343164989794c917e67f187d0ac49ba8
(cd ginux && make build)
(cd recipe && cargo build --locked --release --bins)
(cd recipe && cargo package --locked)
cargo install cargo-generate-rpm --version 0.21.1 --locked
(cd recipe && cargo generate-rpm)
```

Acquire and verify the manifest's inputs into a new directory, run Ginux's
`fetch-rocky-oci.sh`/`prepare-rocky-oci.sh` with its documented cache/work directory
overrides, then run each actual recipe-image producer. Select that image's
provenance SHA, the built Ginux launcher and the identical coordinator/worker
binary. Run the whole coordinator with the documented bootstrap privileges.
All exact VM scripts and commands are retained with the evidence; no source or
bootstrap binary in the procedure depends on a private checkout.

## Limits and evidence retention

This closes PRD14 bootstrap/package/fresh-host reproduction only. It does not
close the complete current e2e catalog, hosted Linux CI, Daybreak/PRD09 security,
or either distro's boot/installer/PTY/A-B product acceptance. Old 152-case receipts
keep their original source and c089 image identities. Gate 11's performance
budget failures are retained upstream and remain explicit. macOS stays deferred.

Local RPMs are unsigned candidates, not authenticated published releases. The
publisher inputs used ordinary CA/key trust; no fixture CA or metadata/locking/
isolation bypass was used. The coordinator remains privileged; private output
state is accessed/exported explicitly. Late errors after atomic publication may
leave a complete final path: consumers reject non-success. Signal interruption
may leave private temporary data; automatic signal cleanup and exhaustive I/O
fault coverage are not claimed. Versioned publisher URLs may disappear; pinned
hashes must never be silently substituted.

The previous preparation manifest (587 regular files, SHA-256
`d16e82728cfeb3fb878a09b074d23444ccc07768bafea088464d3a126effeae9`)
remains historical evidence for its own provisional revisions. New exact-source
receipts independently establish this maintained pair; earlier passes were not
combined to claim a new full-suite pass.

Canonical final VM evidence is `evidence-final.tar.gz`, **858 members**, SHA-256
`fb38051284e8a053f075f5628ec4ec4dafbe27863cb8125974f4ec45b1de3ca1`.
Its embedded final-source receipt agrees with the independently fetched JSON.
The new SHA-256 manifest verifies **118 regular files**, zero symlinks and zero
special files (2,698,425,447 logical bytes). Manifest SHA-256 is
`b913d2361300e6888dedd0335e27df28b18c2d5bfe5a4a9ba115de9657b98f96`.
Run `sha256sum --check --status .artifacts/ginux-contribution-preparation/prd14-delivery/SHA256SUMS`
from the superproject root. Intermediate checkout/Git stores and Python bytecode
are excluded; final exact source archives remain included.

The archive retains guest UID/GID/modes; no extracted review view is asserted to
retain that metadata. The final source receipt verifies clean published sources,
launcher/coordinator/worker/producer and harness hashes, all required outcomes,
and no remaining task Ginux/worker processes. The archive and copied package/
image digests were independently verified before destroying the disposable VM.
Its transient domain, writable disk, NVRAM and private SSH key are removed;
other domains were untouched. Public inputs/base and exact source archives remain.

Active skill: `hdae-development`, discovered at
`.agents/skills/hdae-development/SKILL.md`; reload the complete skill, required
reference, repository instructions and this checkpoint after context shifts.
Root instructions supersede skill ceremony. No PRD14 decision or required
verification remains open. Other named release/security/product tasks retain
their own acceptance requirements.
