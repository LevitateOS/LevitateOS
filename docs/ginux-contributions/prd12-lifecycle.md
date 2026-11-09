# PRD12 — local outcomes verified; final delivery acceptance open

Updated 2026-11-08. **PRD12 remains unchecked.** All **57/57** required selected
CLI cases pass on one frozen local combination: zero failed, missing, interrupted
or unrun cases. Actual recstrap deployment of the exact resulting zstd EROFS
passes on both existing product kernels under session-libvirt. A real failed
builder invocation preserves both previous valid images. No live task-owned
process, scratch invocation, host mount or loop device remains in the recorded
observations; both transient guests powered off and disappeared.

This closes that bounded local verification, not final release acceptance.
The maintained baseline 28f4368f/e7bfc166 is now publicly obtainable and reproduced
under [PRD14](prd14-bootstrap-packaging.md); the earlier source-choice/missing
baseline condition is superseded. Later correction delivery and PRD05's required
actual merges remain open. Current main is still 53a54adde; #232/#233/#234/#236/
#238/#243 remain open/unmerged. No new user hold is inferred.

The current optimized release combination is recorded in
[the active product checkpoint](prd15-product-contract.md): Ginux e4d95de1 /
21a54e60, Recipe 176395a / matching e7073fc1, image ffe04d86. Its complete **152
e2e + library_backend + image_producer = 154/154** run passes, including the
lifecycle/storage/metadata cases. Exact source/inventory/hash/resource receipts
are retained under `prd15/`; bounded observation reports no matching process,
mount or loop, 702 regular leases unheld and no inspection error.

Fresh actual deployment of that run's **dbe1e993… EROFS** by the newly produced
static **13f6a3da… recstrap** passes all existing target metadata oracles on
**both pinned product kernels** under transient operator session-libvirt.
`prd15/metadata-release-deployment.json` binds the unchanged runtime tuple;
`prd15/release-metadata-deployment/receipt.json` records exact inputs, controller,
consoles, clean observed deployer source 5de6647 and both guest results. The
controller source/assertions and fc182144 compiled bytes are unchanged. The
fixture producer now checks ELF INTERP before collecting dynamic libraries,
supporting the actual static PIE while retaining loader failures for dynamic
inputs. No installer/mount guard is bypassed; both guests power off normally.

These new receipts establish current runtime/metadata outcomes. Whole product,
final maintained-source delivery/security and the user-held dependent-recipe
remove/cleanup policy remain open. The forced `--threads=1` product workaround is corrected in builder 9468d26.
An actual managed two-CPU normal-linker build and the fresh corrected Acorn base
produce the exact same 13f6a3da recstrap; metadata-control reuse is therefore
bound to identical actual bytes, not a presumed source equivalence. The fresh
six-product/whole-guest wave remains in progress.
The older 57-case receipts below retain their original source and counts.

Fresh failure preservation on the **current 4d5928b7 builder** also passes:
`prd15/current-builder-failure-preservation.json` binds its current source guards,
and `prd15/current-producer-preservation/receipt.json` records the precise stale
privileged-content rejection, identical prior target/overlay image hashes and
identical directory entries after failure. The test used a `cp -a` mutable copy;
original catalog/metadata guest inputs are independently checked unchanged. This
replaces reliance on the older cc403cf1 producer result for that outcome.

## Exact source and artifacts

| Owner | Exact identity |
| --- | --- |
| Recipe test/source revision | Clean `85f144f014c32695c41e89d0784edbc29fee5f0f`. Only tests changed from PRD11's `8014c9b`; both have production src tree `ec64464f3ca3dbcf199201b3e5b113955ad33710`. |
| CLI and matching worker | Both SHA-256 `4cb2b900995a76f4055420790ee0a49e0b9b1c2da9c33f1b941747dd3104e620`. Same exact PRD11 production bytes, not older implementation results relabeled as current. |
| Ginux | Clean local `f7f0d14cc5abc1874b755e6990e4bd87bfacf050`; launcher `0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3`. Frozen PRD09 `fa8be950` remains untouched. |
| Ordinary compiler/acquisition image | `oci:recipe-compiler@sha256:c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e`. No fixture CA or host workload substitution. |
| Immutable final harness | `bin/e2e-final`, SHA-256 `8c4452bdfe7b8a3b4e254d992188405fb20c8355dc013ab10db134ff2292e5d1`. Catalog enumerates 149 names; this task selects 57. |
| Actual image producer | distro-builder `cc403cf1e477c74e8860fa477d4d9efba5844d3c`, binary `2d365e854fca3ee6bb98744babdbf3a53692ab96201deb42d1a1341d4bb0fe29`. Uses normal zstd compression, chunk size and tar metadata owner. |
| Actual deployer | recstrap `3ea733f477fd090ea84d2d4187bdbe4bc519ff61`, binary `30b8235aeecee78fc84acdf18b3e3662e8055930d8dcdd1955f610fbe73cd466`. No guard waiver or extraction substitute. |
| Final produced EROFS | `4867194669c4c4c2828bd4438f6c5aa4a00de27ffe00fe861958d0de7b94598d`, identical to both frozen VM inputs and unchanged by the failed rebuild. |

Evidence root: `.artifacts/ginux-contribution-preparation/prd12/`.
`final-source.json` binds sources, production-tree equality, build toolchain/lock,
all binary/image hashes and the final result file. `final/identity.json` records
fixture hashes and environment/selection. Every fixture/log/binary hash was
rechecked against the frozen source and completed run. `final/results.json`
SHA-256 is `f08496d2d072301b6079ed817e03e0a61e17222c65f88e879e987dee283e996a`.
Full source patches, build logs and immutable tools accompany the receipts.
This task did not adopt superproject gitlinks or publish these local sources.

`SHA256SUMS` covers **33,288 regular files** (4,727,534,404 logical bytes),
including complete retained fixtures, logs, immutable tools, earlier failures and
the deployment verification sources. Its SHA-256 is
`576d07cae9743fa0e74ac813b5281b0610d797b430e1192ec0c1c68bb45e20a1`.
`symlinks.json` records 45 links without following them; no special files were
found. Build-cache `target/` and the manifest/report outputs are excluded;
`manifest-summary.json` records this scope. The complete manifest verification
exited 0 with an empty error log. Verify from the project root:

```sh
sudo -n sha256sum -c .artifacts/ginux-contribution-preparation/prd12/SHA256SUMS --quiet
```

The builder revision above is the exact tested producer. Concurrent unrelated
kernel-recipe work subsequently moved the checkout to `2f57cbc`; it does not
replace this receipt or its snapshotted kernel inputs. No result is transferred
to that revision merely because its EROFS owner is unchanged.

## Every required outcome

`coverage.json` maps every selected name to these outcome groups, with no orphan.
Every row below uses only the final 57-case run, except explicitly identified
additional consumers on the same exact bytes. The initial 54-case batch and
four strengthened preparation cases are retained as history, not added to the
final count. PRD11's exact SQLite/Cargo/RPM/APK receipts remain reusable for those
unchanged production consumers; they do not substitute for these outcomes.

| Required outcome | Final evidence and observation |
| --- | --- |
| Setup once, imports/extends, readiness and dependencies | Invocation setup-once, staged base/import/module closure and inheritance checks; transitive dependencies compile and execute their actual tools. Cycle/missing/bad-declaration/check/cleanup failures stop the parent. Separate dependency state and reused check discoveries survive. |
| Install/reuse/check/remove/cleanup | Real lifecycle, custom checks, no-hook cleanup, directory transforms, checked child failures, primary-vs-cleanup errors and retained target removal all pass. Removal changes the domain and preserves the imported source and earlier product. |
| Changed source/inputs/targets and output loss | Source/input byte changes select fresh identities; changed target content gets fresh seed. Invalid target kinds/unsupported targets fail before setup. Deleting a published product restores and republishes its retained bytes without rebuilding. |
| Concurrent destinations, retention and leases | Two real coordinators serialize different inputs sharing one destination, with observed lock waiting and distinct correct products. History stays within the declared bounds; active log leases, foreign notes and previously published products survive. Coordinator death and subsequent scratch reclamation pass. |
| Deadlines/cancellation/diagnostics | Silent work can outlast frame timeout within its execution deadline; actual execution deadline, partial frame and SIGINT paths finish with their required failure/cancel result. Descendants stop; last complete checkpoint and durable diagnostics survive. Successful finalization can outlast teardown grace, while late non-successful launcher exit cannot publish. |
| Before/after-ack recovery and failure preservation | Both synchronized checkpoint interruption points recover the correct files/state and run only the required hooks. A forced reinstall writes incomplete replacement bytes and fails; the prior checkpoint pointer is byte-identical, the prior product remains valid, and ordinary reuse recovers successfully. |
| Bounded storage, >2 GiB and 5,000 members | Actual 2,147,487,744-byte payload is produced inside Ginux and every published byte is checked (zero-filled prefix and tailproof). All 5,000 named nonempty entries are checked individually. Explicit bounds remain; no public limit or storage check is disabled. |
| Links, modes and timestamps | Source file/directory modes, shared hardlink identity, absolute/relative symlinks, second-resolution file/link/directory mtimes and checkpoint/restart identity pass. Restored alias mutation leaves original source and prior product unchanged. |
| Target metadata through the complete path | Real package extraction → checkpoints/reuse → publication → reimport → composition → actual builder EROFS. Target tar checks preserve intended UID/GID, setuid/setgid/sticky modes, links and capability bytes; changed privileged content is rejected. Actual recstrap mounts/copies the exact image in both supported-kernel guests and independently verifies file/directory/symlink UID/GID, modes, link identity, payload, capabilities and epoch mtimes. |
| Failed image replacement | `producer-failure-preservation-corrected/receipt.json`: actual builder exits 1 for stale privileged-content metadata, leaves both prior rootfs/overlay images byte-identical and leaves no extra staging entry. The preserved rootfs is the same image subsequently verified by recstrap. |

Resource evidence is concrete and bounded: each final case records observed
process identities and checks no live descendant after grace, plus no remaining
owned scratch invocation. `final-resource-observation.json` additionally scans
exact PRD12 process root/cwd/command paths and host loop/mount references; all
are empty. Final libvirt listing is empty. Retained logs/state/products and
foreign-file controls are intentional evidence, not open OS leases. These are
ordinary lifecycle outcomes, not proof of every adversarial interleaving;
[PRD09](prd09-security-acceptance.md) remains open.

## Actual deployment environment and failure dispositions

The host kernel `6.12.0-211.74.1.el10_2.x86_64` advertises EROFS but has
CONFIG_EROFS_FS_ZIP_ZSTD disabled. The first actual recstrap mount failed with
algorithm 3 unsupported. That failed test is not a pass. Compression was not
disabled and fsck extraction was not substituted. The retained test now checks
production/archive behavior on the host, and the actual deployment consumer
runs in an appropriate kernel environment with all original metadata assertions.

The **frozen** diskless session-libvirt guests used:

- `6.12.116-levitate`, kernel SHA-256 `ce542acdd0a8909debc1d4893da7a750396dbe5246a3cb85d840163c4d8c9d9f`.
- `7.3.3-acorn`, kernel SHA-256 `e55c1892355d42390e2b76015ae7922458a8ca51ef2727580111da15719bbdaf`.

`vm-frozen-{levitate,acorn}/inputs.json`, XML and unedited consoles bind the
exact recstrap/image/controller/libraries/kernels. Both emit a full PASS after
actual recstrap exit 0, complete assertions, target unmount and poweroff.
`frozen-deployment-results.json` verifies these bindings. Their tiny target is
an empty independent tmpfs with a 3 GiB limit, satisfying the installer's normal
space/mount guards without allocating 3 GiB. No network/disk, host clock change,
kernel rebuild or direct-QEMU invocation. These are backend/metadata guests,
not both distros' full product boot/install acceptance.

All material failed/incomplete attempts have a concrete disposition:

- Host zstd mount failure: unsupported host feature; verified on existing
  supported product kernels instead. No weakened compression or installer guard.
- Early VM startup permission/label failures: fixed ownership of generated
  evidence directories and applied read-only `virt_content_t` to generated boot
  inputs. No global security setting changed; later frozen guests actually pass.
- A new output-loss test was accidentally inserted inside an embedded Rhai
  string. Compilation alone did not register a test; the required catalog check
  rejected it before execution. Placement was corrected, the original fixture
  restored, and both focused checks plus the complete final run pass.
- Earlier produced EROFS hashes differ from the final result. Their VM receipts
  were rejected for reuse; both frozen guests were rerun on the exact final
  image. This is not a bit-reproducibility claim; PRD14 retains that obligation.
- The first producer-preservation verifier ran the command but failed while
  writing its receipt because a string path was not converted to Path. It is
  not counted. Its source/logs remain; corrected verification exits 0 with the
  complete expected exit-1/before-after-image/staging evidence.

No production change was needed for these local PRD12 outcomes. Recipe `85f144f`
retains useful tests: opt-in fixture preservation, new loss/failed-reinstall
coverage, every-entry/every-byte storage oracles, and actual producer coverage.
The old replicated mount/copy test is replaced by the real recstrap consumer
with at least the same owner/mode/link/capability assertions, plus setgid and
explicit timestamp coverage. No tests were ignored, relabeled as units or
removed to hide a failure.

## Reproduction, limits and next action

Build logs record locked/offline harness, builder and recstrap compilation;
the C controller builds with `-O2 -Wall -Wextra -Werror`. The immutable harness's
selected commands/environment are in every final case receipt. The run is:

```sh
python3 - <<'PY'
import json, subprocess
from pathlib import Path
base = Path('.artifacts/ginux-contribution-preparation/prd12')
env = json.loads((base / 'final/results.json').read_text())[0]['env']
env = {k: v for k, v in env.items()
       if k not in {'TMPDIR', 'RECIPE_TEST_ARTIFACTS'}}
# A new directory is mandatory; runner refuses to overwrite existing evidence.
subprocess.run(['sudo', '-n', 'env', *[f'{k}={v}' for k, v in env.items()],
                'python3', str(base / 'run-selected.py'), 'new-final', '::'],
               check=True)
PY
```

The [deployment source and exact procedure](prd12-deployment/README.md) use the
retained output from that new run and existing kernels. Read actual PASS/results;
a created domain or written handoff is not verification. The standalone producer
failure command and before/after hashes are in its receipt. PRD11's full exact
consumer receipts can be reused because production src/binary/image identities
are unchanged; no earlier runtime revision is counted as final here.

The entire 149-case catalog/CI, fresh-host source/bootstrap reproducibility,
full security races, and real product boot/install/A-B acceptance remain in
PRD10/13/14/09/19. EROFS intentionally normalizes mtimes to epoch; Recipe/Ginux
archive persistence remains second-resolution. No aggregate RAM/CPU or general
Linux ABI claim is added. Required final maintained delivery/storage integration
is still absent. Carry these tests and receipts into that actual owner join,
then verify the resulting delivered combination before checking PRD12 complete.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`;
reload fully with required reference, repository instructions and this checkpoint
after context shift. Current task: PRD12, bounded local outcomes passed and final
delivery acceptance incomplete. Exact next action: await the actual PRD01/PRD05
maintained-source/storage join, retain these corrections/oracles, and rerun the
required outcomes on its exact delivered artifacts. Existing unrelated staging
is preserved. No push, PR, upstream comment, gitlink adoption or source-delivery
choice occurred.
