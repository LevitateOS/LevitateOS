# Production update publication (x86_64)

Implementation is available; production enablement is **pending validation**.
No production origin, root/delegation, index secrets, runner runtime or installed
public-origin lifecycle evidence has been configured by this change. Current
LevitateOS/AcornOS installed defaults stay disabled. The approved design is
[the recab release engineering document](../../tools/recab/RELEASE-ENGINEERING.md).

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

Configure the existing Linux x86_64 release runner with labels
`self-hosted, linux, x64, recab-release`. It supplies a pre-existing glibc runtime
tar and exact content reference through repository variables `RECAB_GINUX_ROOTFS`
and `RECAB_GINUX_ROOT`. Its current Recipe binary must run in that runtime;
the builder compiles the checkout's matching Ginux launcher. Workflows do not
fetch, fabricate or build a runtime image as a substitute for this input.

After provisioning, `RECAB_PUBLISHING_READY=true` permits the metadata workflow.
This is a service-provisioning switch, not installed-system production enablement.
Keep `RECAB_STABLE_VALIDATED` unset until the required installed validation.
Set `RECAB_PUBLIC_MONITORING_READY=true` when the secret-free release runner is
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

Daily `operation=refresh` authenticates the old signed index, including an
expired one, and renews its freshness for at most 14 days, capped by authority
expiry. It preserves every selected release ID, transport map and payload byte.
It does not rebuild, choose a new system, apply changes or alter boot selection.
Authority expiry produces a warning 30 days ahead and eventually stops signing;
it is not extended with an index key. Renew authorization offline. Keep signing
key overlap during routine rotation so existing selected releases remain
verifiable; a revoked selected release stops refresh rather than being silently
dropped or trusted. Root-authorized epoch increases reset index counters only.
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
mode, fingerprint and finished status are checked. That record is an operator attestation, not a fourth
signing role or evidence fabricated by the publisher. The runtime scenario's
public-origin support and applicable real installed checks are still required;
this workflow does not supply them or enable installed defaults. Do not set the
switch or manufacture a passing record from local native-fixture results.

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
contracts. The checked-in actions use full reviewed commit pins. Actual GitHub
upload/deployment execution has not been exercised against production resources.
