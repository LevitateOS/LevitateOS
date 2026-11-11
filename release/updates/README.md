# Production update publication (x86_64)

The Pages service, immutable-release setting, master-only deployment environments
and per-distro signing authority are provisioned. Production enablement is
**pending validation**. Current LevitateOS/AcornOS installed defaults stay
development-only. The approved design is
[the recab release engineering document](../../tools/recab/RELEASE-ENGINEERING.md).

Public production policies and root-signed delegations are staged under
`release/updates/production/<distro>/`. They are not loaded as installed defaults.
The Pages tree exposes the same public delegations for candidate and stable,
plus `updates/provisioning.json`; it advertises no system releases or update index.
The [metadata-only bootstrap deployment](https://github.com/LevitateOS/LevitateOS/actions/runs/43052973585)
succeeded on 2026-11-11. Recipe/Ginux fetched all nine public files over HTTPS
and verified their exact sizes and SHA-256 hashes. The public
[provisioning status](https://levitateos.github.io/LevitateOS/updates/provisioning.json)
still declares production disabled. This proves public metadata transport, not
an installed system update, release payload or trial boot.
The x86_64 release runner `recab-release-levitate-dev-x86_64` is registered and
online. Matching baseline/new release artifacts and actual installed public-origin
validation remain rollout work, rather than a reason to enable unvalidated updates.

Private PKCS#8 working files are in
`~/.local/share/levitateos/release-keys/<distro>/{root,release,index}.pk8`, with
0700 directories and 0600 files, outside the checkout. Only each index key is
installed as a secret in the `recab-updates` environment. Root/release keys stay
local. Additional offline backup setup is deferred; it does not block this
provisioning step. No private key bytes belong in commits, logs, Recipe inputs
or workflow artifacts.

The builder owns transport and signing. `cargo xtask release` runs the same
orchestration as the GitHub workflows. Recipe inside Ginux performs every public
payload and Pages verification; gh handles authenticated GitHub release APIs.
No command below implicitly builds a system, kernel or ISO, or starts a VM.
Tool compilation is separate from producing release payloads.

## Prepare and upload existing artifacts

Configure the distro's production policy and current public
`distro/variants/<distro>/release-keys.json` through the approved root/delegation
protocol. Production identity uses channel `stable`; candidate/stable are
separate discovery audiences over exactly the same release ID. Confirm the
canonical Pages URL and actual release redirects before sealing installed
policy. The orchestration accepts the approved repository and origins only:

- `https://levitateos.github.io/LevitateOS/updates/`
- `https://github.com/LevitateOS/LevitateOS/releases/download/`
- Explicit HTTPS CDN grants in the production policy, without wildcards.

First produce the signed release using the existing Recipe/builder producer and
external release key. Retain its prepared root, EROFS image-writer provenance,
intended inventory, kernel and installed initramfs. Then, on the release machine:

```sh
cargo xtask release prepare levitate /absolute/release-bundle \
  /absolute/release-bundle/rootfs-source /absolute/publication \
  --source-commit <full-source-commit>
cargo xtask release upload levitate /absolute/publication
```

`prepare` needs the same privileges as the existing read-only release verifier.
It verifies the genuine produced set before splitting the frozen bytes. Parts
are at most 1 GiB and there are at most 64 per artifact. `--part-bytes` allows
smaller boundaries without changing the system manifest. It records the explicit
source commit and signed source digest; this record does not claim a new
reproducibility or source-to-binary proof. No system manifest is re-signed.

The output contains `publication.json`, the original manifest/signature/public
proof, and flat, content-addressed `assets/`. Keep it outside Git history and
Actions artifact uploads. Repeating preparation preserves matching objects and
refuses altered bytes. Upload checks all local part hashes and complete artifact
hashes before creating a draft. Verification exports stay in disposable request
scopes rather than accumulating payload copies in the repository. It resumes matching uploaded assets, requires
server SHA-256/size equality, and never clobbers, deletes or moves a release/tag.
It requires repository immutable releases to be enabled and verifies the final
immutable tag's exact source commit. The operator's gh credential needs release
write and repository administration read access for the immutability check;
these credentials are used by gh, not passed to Recipe. Root/release private keys
remain external files and are never GitHub secrets.

After publishing payloads, `upload` downloads all their parts through Recipe and
checks both part and whole hashes. A failure leaves an immutable payload release
that is not yet discoverable; retry verifies the same objects. It does not
publish a candidate/stable feed. The publication record and immutable GitHub
release provide resumable external progress; local files alone do not establish
remote success.

## Candidate publication, refresh and deployment

Provision the `recab-updates` GitHub environment on the sole default branch and
configure its deployment branch restriction. Set its per-distro secrets
`RECAB_LEVITATE_INDEX_KEY_BASE64` and `RECAB_ACORN_INDEX_KEY_BASE64` to base64 PKCS#8
**index-role** keys via secret input handling, never shell tracing or arguments.
Only workflows on the default branch read these secrets. The signing process
materializes a needed key in a private directory on `/dev/shm` tmpfs, removes it
on exit, and passes only a file path to the builder. Neither build nor Recipe
acquisition jobs receive index secrets. The metadata tool is compiled in a
separate read-only job and transferred as a short-lived tool artifact; the
signing job runs no builds or Recipe workloads. It refuses a changed source
head before using that tool. The job has no root/release keys.

The provisioned Linux x86_64 runner has labels `self-hosted, Linux, X64,
recab-release`. Its enabled host service is `recab-release-runner.service` and
its private state lives under `/var/lib/recab-release/`. The tracked service,
namespace launcher and job hook are in `release/updates/runner/`.
The service uses the vendor `bin/runsvc.sh` entrypoint and graceful signal
forwarding, so a restart closes its GitHub session rather than leaving a stale
interactive-agent session behind.

The launcher gives the runner a separate filesystem, PID namespace and private
65536-entry UID/GID range through systemd-nspawn. Namespace root maps to an
unprivileged host UID, so Ginux can chroot/drop credentials without becoming
host root or the signing-key owner. No host home, signing directory, operator
credential store or daemon socket is mounted. Host `/usr`, the exact existing
compiler tree, runtime and bootstrap programs are read-only inputs. The job hook
accepts only default-branch P14 publication/monitoring workflow dispatches or
schedules before checkout/commands run. This is the runner's host-service
boundary; Recipe continues to execute acquisitions inside Ginux.

Recipe acquired and extracted the checksum-pinned official x86_64 runner
2.338.2 through `distro/builder/os-recipes/release-runner.rhai`. Its updater is
disabled; change that Recipe's pin and deploy its verified output for upgrades.
Registration credentials remain private in the namespace filesystem, outside Git
and the signing configuration. The existing compiler is 1.99.0 and its runtime
is the already-produced glibc image
`8bd12066024f731a796a6fc1734011837d02873d86ffb325999796712134da3d`.
Repository variables select `/opt/recab/runtime/rootfs.tar` and its exact OCI
content reference. No compiler, runtime, system image or VM was built to provision
this runner. The builder still compiles the checkout's matching Recipe/Ginux
tools only when an explicit publication/monitoring operation calls for them.

The focused namespace check passed actual public acquisition through Recipe/Ginux,
with the host home absent and the private UID/GID map verified. Dispatch
`recab-updates.yml` with `operation=runner-check` to exercise the real registered
runner without tool builds, signing, publication, VM tests or stable enablement.
This job receives no signing environment or index secrets and reports its
metadata transport result separately from installed lifecycle validation.
The [registered-runner check](https://github.com/LevitateOS/LevitateOS/actions/runs/43062654582)
passed: the job hook, checkout and Recipe/Ginux verification of all nine public
files succeeded on this runner. A normal service restart also completed without
timeout after correcting shutdown delivery to the vendor wrapper directly.

`RECAB_METADATA_DEPLOY_READY=true` permits explicit `operation=deploy` and
`operation=runner-check`, including bootstrap of the public authority before any
release is advertised. Deployment requires no tool/system build, Recipe runtime,
signing secret or release runner; the runner check requires the provisioned
runtime/service. The metadata-only deployment still uses the master-restricted Pages
environment and the existing serialized workflow. Its signing steps are skipped.

After release configuration/runtime provisioning, `RECAB_PUBLISHING_READY=true` permits the publication/refresh workflow.
This is a service-provisioning switch, not installed-system production enablement.
Keep `RECAB_STABLE_VALIDATED` unset until the required installed validation.
Publishing and public monitoring currently remain false; `RECAB_STABLE_VALIDATED`
is unset. Set `RECAB_PUBLIC_MONITORING_READY=true` when the secret-free release runner is
ready. Configure Pages to deploy through GitHub Actions, with the `github-pages`
environment restricted to the default branch. No `gh-pages` branch is used.
No paid runner, monitoring subscription or second hosting backend is required.

Dispatch `.github/workflows/recab-updates.yml` explicitly with `operation=publish`,
`distro=levitate` (or `acorn`) and the exact immutable payload `tag`. It:

1. Fetches only bounded public metadata and verifies the entire immutable
   payload through Recipe/Ginux in a read-only, secret-free job.
2. Rereads the default branch in the signing job, binds the exact publication
   hash from that verification, allocates the next candidate index sequence,
   verifies release-role signatures and signs the index role separately.
3. Commits only small public metadata under `release/updates/site` on the
   existing default branch. A stale base refuses the push; rerun against the
   current branch instead of forcing or silently merging counters.
4. Rereads the current default-branch head, exports the whole metadata site and
   deploys Pages in that same workflow. It does not depend on a token push
   starting another workflow. The site exporter rejects links, payload files
   and trees larger than 256 MiB. Historical raw indices and local locks are
   excluded from Pages; addressed signatures and public authority history stay
   reachable. Payloads are never put on Pages.
5. Checks exact deployed Pages bytes and representative payload parts through
   Recipe when public monitoring is provisioned. These results are transport
   evidence, not an installed boot or update-service readiness claim.

The tag input also accepts an explicit base tag and desktop tag separated by a
space, for the same distro. Both immutable publications are fetched and every
payload is verified before signing. The signing job binds the whole selected set
to a deterministic digest, and the builder changes the discovery pointer once
after checking both profiles. Single-tag invocations retain their existing digest.

Daily `operation=refresh` authenticates the old signed index, including an
expired one, and renews its freshness for at most 14 days, capped by authority
expiry. It preserves every selected release ID, transport map and payload byte.
It does not rebuild, choose a new system, apply changes or alter boot selection.
Authority expiry produces a warning 30 days ahead and eventually stops signing;
it is not extended with an index key. Renew authorization offline. Keep signing
key overlap during routine rotation so existing selected releases remain
verifiable; a revoked selected release stops refresh rather than being silently
dropped or trusted. Root-authorized epoch increases reset index counters only.

To replace releases signed by a newly revoked release key, first authorize the
replacement signer through the offline root and prepare/upload genuinely newer
signed payloads. Publish the complete affected base/desktop set together. The
publisher authenticates old manifests under their original authority before using
their sequences as floors, then requires current authority for replacements and
every retained profile. An index signature alone cannot invent a release floor.
Refresh, duplicate profiles, lower sequences and replacement sets that leave a
revoked profile selected refuse without changing the reachable index. No entry
is silently dropped. The same atomic operation is available locally:

```sh
distro-builder --repo-root /absolute/repo release publish levitate \
  /absolute/base-publication /absolute/feed candidate --index-key /external/index.pk8 \
  --publication /absolute/desktop-publication
```
Authority history prevents rollback and reactivation of known revoked signers.

Publication, promotion, refresh, redeployment and the independent monitor share
one concurrency group with active cancellation disabled. GitHub may supersede a
pending run; explicit release operations remain resumable and must be checked
for completion. A failed upload/signature check/push/Pages deployment cannot
make a partial release discoverable through the previous deployed site.

Use `operation=deploy` to retry a deployment from current default-branch metadata
without re-signing or selecting another release. Both the deployment retry and
the scheduled monitor reread the current branch, never a triggering SHA or old
queued Pages artifact.

## Stable promotion remains gated

`operation=promote` selects one exact current candidate publication and uses the
same release ID, payload bytes and tag. It allocates a separate stable index
sequence. It requires `RECAB_STABLE_VALIDATED=true` plus a reviewed operator
validation record at `release/updates/validation/<manifest-id>.json`, naming the
release ID, source commit, actual `ab-update` run manifest hash, public origin
and x86_64 lifecycle result. Keep the matching successful actual run manifest
beside it as `<manifest-id>.run.json`; its exact hash, distro, scenario, install
mode, fingerprint, finished status and structured `public_update_result` are
checked against the exact candidate release, profile, transport and public origin. That record is an operator attestation, not a fourth
signing role or evidence fabricated by the publisher. The existing runtime harness now has public-origin mode and explicit base/desktop
artifact selection; its actual installed checks remain unrun. This workflow
does not run those scenarios or enable installed defaults. Do not set the
switch or manufacture a passing record from local native-fixture results.

Promotion accepts the same explicit two-tag selection. Each profile must have
its own matching actual public-origin lifecycle record before either selection
changes in stable; the secret-free verification digest binds both publications.

The existing local factory/updated artifact pairs and full P13 lifecycle evidence
must cover both distros and advertised profiles before enabling stable updates.
Additional CPU architectures remain deferred indefinitely.

## Monitoring and local metadata operations

`.github/workflows/recab-update-monitor.yml` independently checks the current
public metadata and representative parts daily, even when signing/refresh fails.
It has read-only permissions and no signing secrets. It verifies signatures,
expiry, exact default-branch metadata bytes, TLS grants and payload hashes through
Recipe. Normal GitHub workflow failure notifications retain diagnostics; no
messages are sent to an external service. Metadata propagation retries are
bounded and do not turn a stale/signed-but-old site into success.

Recipe retries transient 429/502/503/504 responses at most twice, with short
bounded delays. A numeric `Retry-After` over five seconds, an HTTP-date or an
invalid delay defers automatic retry and reports that the operation should be
retried later; it never substitutes the short default backoff for that header.

Local builder operations are useful for a staged metadata tree:

```sh
distro-builder --repo-root /absolute/repo release publish levitate \
  /absolute/publication /absolute/feed candidate --index-key /external/index.pk8
distro-builder --repo-root /absolute/repo release publish levitate \
  refresh /absolute/feed candidate --index-key /external/index.pk8
distro-builder --repo-root /absolute/repo release verify-online levitate \
  /absolute/feed candidate --representative
```

Local signing has the same role/custody checks but does not prove a remote
upload, deployment or installed update. The focused native integration is
`tools/recab/tests/finalization.py --production-updates --updates <recipe>
--release-builder <builder>` with explicit existing kernel/module artifacts.
It exercises real producer verification, part publication, HTTPS Recipe/Ginux
acquisition, concurrent local refreshes and offline update handling. It is
independently runnable, builds no distribution and boots no VM.

The GitHub integration follows the documented [immutable release
API](https://docs.github.com/en/rest/repos/repos#check-if-immutable-releases-are-enabled-for-a-repository),
[release asset digests](https://docs.github.com/en/rest/releases/assets), and
[Pages custom workflow](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages)
contracts. The checked-in actions use full reviewed commit pins. Metadata-only
Pages deployment has been exercised and its public files verified through Recipe.
Immutable system-payload upload and installed public-origin lifecycle validation
remain unverified.


Public-origin inputs and the independent install/ab-update invocations are
explained in [the install-tests guide](../../testing/install-tests/README.md).
After an actual passing public-origin lifecycle (not a native fixture), use:

```sh
cargo xtask release record-validation /absolute/existing-passing-ab-update-run
```

This exports the matching small operator/run records without signing, pushing,
setting readiness variables or enabling installed production defaults. No
passing records have been generated by this implementation change. Full builds,
VM runs and live public-origin validation remain pending explicit authorization.
