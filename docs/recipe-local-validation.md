# Local Recipe integration validation

This delivery implements the [ordered task queue](recipe-simplification-implementation-plan.md).
The [interface record](ginux-recipe-interfaces.md) describes current behavior.
All changes stay local: no Ginux PR, push, remote-text edit or maintainer message
is part of this work. Ginux contribution candidates remain in that queue's backlog.

## Workspaces and revisions

The superproject stays on `master`; Recipe stays on `ginux-recipe/r06-launch`.
Local Ginux work is in
`/home/vince/.codex/worktrees/ginux-recipe-local-integration/ginux`, branch
`codex/ginux-recipe-local-integration`. Its upstream reconciliation base is
`2a82f61e`, joined by `8b5cf5dd`. The original `tools/ginux` checkout and branch
are not switched. Its superproject gitlink is deliberately not used to imply
that the unmodified checkout contains this separate worktree's changes.

Current validation baseline supervisor: `93a1a56d`, immutable binary
`/tmp/ginux-t24-hidden-devices`, SHA256
`641db1d511ae0ba9ce538eb0bf764ab63cae20a5e015a8cb949c519009503df3`.
Its final extraction/large-input corrections are tracked in the task checkpoint;
newer focused evidence must name its own revision rather than inherit this pass.

The actual combined CLI/worker binary is `/tmp/recipe-t25-bin`, SHA256
`48f9c4ba2a30f857ea23a7481b3f28bfe4f2c510653a5ae562fcec3033edd3ff`.
Actual CLI harness `/tmp/recipe-t25-e2e` SHA256
`e9bf97b2cad48d330edbf3dc20990d92a133167733336c221560aab5a12968a4`.
It includes the current preseed closure fixture, committed as `a6ac832`.
Coherent implementation commits and source HEADs are recorded in the task
checkpoint; immutable executable hashes distinguish runs during parallel builds.

## Prepared images and provenance

| Prepared image under `.artifacts/ginux-env` | Assembled tar SHA256 |
| --- | --- |
| `prepared-image` (GNU compiler) | `2e99d8dc80324b185267fcbf646eb407a469751f69a0d794ffc00977e8755fba` |
| `prepared-image-git` | `8bd5ad2e715c163ea201cc629856fb7ee970104f31e7d899913e185e56d3117b` |
| `prepared-image-git-musl` | `9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4` |
| `prepared-image-acquisition` (adds cpio) | `c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e` |
| `prepared-image-network-fixture` (test-only CA) | `fcc70d8c3052ff988a2223839a1050e5305c19ad6344f80cb3bd369c8b3132b8` |

Each producer output contains `provenance.json`. The committed
[acquisition manifest](../tools/recipe/bootstrap/ginux-acquisition-image.json)
and [bootstrap instructions](../tools/recipe/bootstrap/README.md) record the
exact ordinary inputs, publisher-metadata pin verification, source URLs and
producer command. The cpio input is official Rocky2.15-3.el10, SHA256
`ffd887348d611f779ae02cef89d7e1cae53adfe7901d31840bb9d5730915bb5a`.
The Rust1.99 musl component is separately pinned and selected, rather than
substituting GNU output for musl. Generated images, caches, test CA keys and
private diagnostic directories are not committed. Fixture trust is never
installed into host trust or the ordinary production image.

## Reproduction

Build only explicit producers/binaries from the workspace:

```sh
cargo build --offline -p levitate-recipe --bin recipe --bin recipe-image
cargo test --offline -p levitate-recipe --test e2e --no-run
cargo check --offline -p levitate-xtask -p distro-builder -p distro-contract \
  -p levitate-recipe -p acornos -p leviso --all-targets
```

Ginux's native bootstrap needs the documented privileges. Run the coordinator
and inherited channels with those privileges; wrapping only the launcher in
plain `sudo` closes required descriptors. A representative actual combined run
uses immutable copied artifacts so a parallel build cannot replace them:

```sh
sudo -n env \
  RECIPE_TEST_BIN=/tmp/recipe-t25-bin \
  RECIPE_WORKER_BIN=/tmp/recipe-t25-bin \
  GINUX_BIN=/tmp/ginux-t24-hidden-devices \
  GINUX_ROOTFS="$PWD/.artifacts/ginux-env/prepared-image-git-musl/rootfs.tar" \
  GINUX_ROOT=oci:recipe-compiler@sha256:9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4 \
  /tmp/recipe-t25-e2e --test-threads=6
```

The full132-scenario result is **pending**, log
`/tmp/recipe-t25-final-managed.log`; an earlier interrupted run is not acceptance.
Separate real public-library execution with spoofed ambient worker markers
passes1/1 (`/tmp/recipe-t25-library.log`); the actual image-producer composition
and rejection scenario passes1/1 (`/tmp/recipe-t25-image-producer.log`).
All six affected Rust packages pass all-target compilation.

Actual managed TLS (trusted, wrong hostname, untrusted CA), HTTPS pinned Git and
publisher-pinned Alpine acquisition→extraction→execution→publication pass again
with the final baseline binaries. Exact commands/case outputs are retained under
`/tmp/recipe-lane-b-network/`; final logs are
`final-{tls,https-git,alpine-source}-93a1a56d.log`.
The production Alpine version output is
`apk-tools 3.0.9-r0, compiled for x86_64.`. The verified package SHA256 is
`ea77b2533566e70c9566c03748199b79348010c3199ba02d97e0634cf0fa9ef7`.
QEMU/RPM extraction and split-pane are still partial, as recorded in the queue.

## Working demonstration

The committed [hello recipe](../tools/recipe/examples/hello.rhai) is the same
current-contract example as the authoring guide. With the configured launcher
and verified image, run:

```sh
recipe install tools/recipe/examples/hello.rhai \
  --build-dir .artifacts/ginux-env/demo/hello-work \
  --state-dir .artifacts/ginux-env/demo/hello-state \
  --json-output .artifacts/ginux-env/demo/hello-result.json
```

This actual local invocation passed with one acknowledged checkpoint and intact
source. Its named program is mode0755, exact payload
`#!/bin/sh\nprintf 'hello\\n'\n`, SHA256
`f869aed15d808e5e860a3202973185b70255b03ebd936437b381aa3eed6c18c5`.
Evidence: `/tmp/recipe-t27-hello.log` and the result JSON. Products/state remain
owned by the coordinator and publication parents remain private; privileged
production does not automatically make a product accessible to an ordinary user.

For real compilation use `distro-builder artifact prepare-tool-source recfstab
.artifacts/recfstab-source`, then the canonical `distro-builder/recipes/recfstab.rhai`
with `--input source=...`; the recipe compiles inside Ginux. For QEMU preparation
use `just tools-install`; boot scenarios consume the selected existing product.

## Verification limits

Prepared kernel source/prebuilt cases validate the caller/lifecycle migration;
no kernel was rebuilt for it. Small prepared rootfs/preseed fixtures and actual
package transfers do not claim a full Fedora ISO materialization or VM boot.
Repair mechanics use a controlled provider, not external credentials.
Ginux's inherited per-process address-space/CPU-affinity limits are not aggregate
RSS or CPU-time accounting. Endpoint grants do not isolate cohosted virtual names.
The current tasks must resolve their actual extraction/build failures before this
record or the goal may be marked complete.
