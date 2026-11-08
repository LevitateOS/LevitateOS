# PRD11 — six local consumers pass; final delivery acceptance open

Updated 2026-11-08. **PRD11 remains unchecked.** All six required ordinary
consumer checks pass on one corrected local launcher/coordinator/worker/image.
The delivered combination required by the queue is still absent:
[PRD01](prd01-runtime-source.md) has no selected obtainable maintained runtime;
[PRD05](prd05-integration.md) still requires actual upstream merges and semantic
storage integration. Local preparation continues under the user's instruction;
these results do not select a downstream delivery route or substitute for merges.
No publication, native-policy expansion, public-limit change or PRD09 security
execution occurred in this task.

A fresh read-only GitHub check still finds upstream main
[`53a54addea67969abaa240ea478def1b94130eee`](https://github.com/gmkurtzer/ginux/commit/53a54addea67969abaa240ea478def1b94130eee)
and #232/#233/#234/#236/#238/#243 open, unmerged, with null merge timestamps.
Exact heads/time are in `upstream.json` below. The ordinary sandbox's attempted
`git ls-remote` failed DNS; the connected GitHub reads succeeded. This is an
external integration dependency, not a new user hold on local work.

## Exact tested local combination

| Component | Source and built identity |
| --- | --- |
| Ginux | Clean detached `/home/vince/levitate-development/ginux-prd11`, `f7f0d14cc5abc1874b755e6990e4bd87bfacf050`, tree `5abbf7aa2566e396ac6730864b634d240ceee083`. Launcher SHA-256 `0dd243fa90ea12117448dfdfbfd170ac8260e0ab3116f116acaa37c40c263ce3`. |
| Recipe CLI and matching worker | Clean `tools/recipe` master `8014c9b62316eb1799e568564a72fcb0ac4db9d0`, tree `c327d62e8b5611797cd153b414fc9b1f2cd1634f`. Both use binary SHA-256 `4cb2b900995a76f4055420790ee0a49e0b9b1c2da9c33f1b941747dd3104e620`. No library/API compatibility adapter. |
| Ordinary compiler/acquisition image | `.artifacts/ginux-env/prepared-image/rootfs.tar`, SHA-256 `c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e`, selected as `oci:recipe-compiler@sha256:…`. Both Rust GNU/musl libraries, GCC, Git, cpio/rpm2cpio and SQLite come from it. No added fixture CA or host workload tool. |
| Frozen PRD09 | `fa8be950b4382a8f92c92d90c8df069c36fdb198`, launcher `441de671c8e0f353319e8944fbbdb6817851722ab46acd1b1bd2263739c507d2`, preserved unchanged. Daybreak's partial receipts remain specific to that source. |

Builds used the exact working production source subsequently committed at these
revisions. Final check commands compile that committed source. The consumer
README was added after execution; all executable fixtures/runner bytes match
the recorded successful-run hashes. `final-source.json` binds both commits,
trees, workspace lock, binary/image identities and all six successful receipts;
it verifies they use the same four artifact hashes. Immutable executables are
in the artifact directory's `bin/`. Superproject gitlinks remain unchanged.

## Successful consumer evidence

All receipt paths below are relative to
`.artifacts/ginux-contribution-preparation/prd11/`. Each command invokes the real
CLI and matching worker through configured Ginux with whole-coordinator bootstrap
privileges and a fresh private output/state directory. There is no host workload
fallback, disabled lock, suppressed pipeline error or single-thread linker flag.

| Consumer / final receipt | Exact successful behavior |
| --- | --- |
| `sqlite-corrected/receipt.json` — exit 0, 10.793 s | C client compiled inside Ginux. Second process receives SQLITE_BUSY during BEGIN IMMEDIATE, then commits after first releases. Two rows sum to 33; guest integrity and independent read-only checks of the published database pass. Published ELF mode 0755. |
| `cargo-corrected/receipt.json` — exit 0, 11.757 s | Four simultaneous Cargo clients, two GNU and two musl, use one real CARGO_HOME and target directory. All four finish; logs show package-cache/build-directory lock waiting and exactly two compilations (one per target). Both resulting programs start/join a Rust thread and return `compiled rust 42` inside Ginux. Both mode 0755; musl ELF has no INTERP. GNU retains its compiler driver and musl its ordinary image rust-lld threading. |
| `git-http-corrected/receipt.json` — exit 0, 11.165 s | Actual HTTP clone and cached exact selection from a disposable loopback bare repository. Commit and payload hash verified before named publication, and published bytes checked independently. Host Git only prepares the upstream fixture. |
| `git-https-corrected/receipt.json` — exit 0, 11.771 s | Actual public HTTPS clone/reuse of `LevitateOS/recipe` commit `43d9dd4e1296799467f42b2c026aa107c3022d8d`. README SHA-256 `18d54af802f6db610cb5fa06f3442087be470e44fc2671889ac5723a0ee2c946` verifies before/after publication. Original hostname, normal image trust and TLS verification remain enabled; only declared github.com:443 is granted. |
| `rpm-corrected/receipt.json` — exit 0, 10.890 s | Fedora zlib-ng-compat RPM SHA-256 `1bab05fecdd71fc4a946852d83405f238c89cccb63987c0941c7f2f0184a99e2`; actual image rpm2cpio→cpio with pipefail. All 169,536 library bytes hash to `bcc97bb783dd4986a54482af2182841712728646ed19236aac3f4aad6605e172`; mode 0755, UID/GID 0, SONAME and build-ID links match after publication. No imported host extractor. |
| `apk-corrected/receipt.json` — exit 0, 12.710 s | Acquire canonical producer-lock-pinned apk-tools-static/musl/BusyBox over HTTPS, validate hashes and release-key signatures, install musl/BusyBox using virtual UID/GID 1000 and execute installed BusyBox through its installed musl loader. Capture/publish target metadata with intended BusyBox mode 0755 and UID/GID 0, independently compare installed bytes to the pinned package payload, and verify all published package hashes. Installation uses no scripts/network, matching the product producer. |

These are successful local-candidate receipts, not totals assembled from older
sources. Historical runs and all initial failures remain intact.

## Concrete corrections and every failed attempt

**Ginux `f7f0d14c`:** `cargo-01` exposed `multiple held stops for one task`.
Bounded diagnostics captured virtual PID 60 with held status **34175** (syscall
stop) followed by **0** (normal death). Linux can terminate a stopped thread
when a peer exits its group. The single held-status slot must preserve the
terminal event instead of trying to process the obsolete stop. The correction
allows only a stopped status to be superseded by terminal status; conflicting
nonterminal or repeated terminal transitions still fail. It never resumes the
dead task or inspects its stale registers. No task/FD bound changes. Final diff:
`ginux-final.patch`, 11 changed lines. Cargo passes after the correction.

**Recipe `8014c9b`:** repeated real HTTPS failures said getaddrinfo thread could
not start. Exact negative syscall diagnostics show eventfd2 returns ENOSYS
before a resolver thread is created. Curl's [resolver context](https://github.com/curl/curl/blob/curl-8_14_1/lib/asyn-thrdd.c)
requires its wakeup descriptor; its [eventfd build](https://github.com/curl/curl/blob/curl-8_14_1/lib/socketpair.c)
returns failure without a runtime pipe fallback. This explains the misleading
thread-start diagnostic; it is not an observed missing clone flag.

The Git helper now passes the coordinator's already pinned immutable hostname
addresses through [Git's `http.curloptResolve`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-httpcurloptResolve)
for clone and any later fetch. It bounds/parses `/etc/hosts`, requires a mapping
for named HTTP origins, preserves URL hostname/port and TLS identity, and keeps
Ginux's numeric endpoint enforcement. No host DNS/proxy helper, unchecked TLS,
eventfd capability or native policy is added. Cached configuration restrictions,
exact commit checks, dirty-checkout preservation and staging remain in their
existing owner. Numeric HTTP and imported file origins retain their paths.

Fixture failures were corrected rather than hidden:

- `cargo-diagnostic` wrongly applied direct musl linker flags to GNU too, so
  GNU could not find libc/libgcc. Musl flags are now target-specific; GNU uses
  its normal driver. `cargo-diagnostic2` passed, but the next run reproduced the
  real held-stop collision; that one green run did not close it.
- `apk-01` used nonexistent `copy_dir`; `apk-02` used the real directory-content
  helper without first creating its destination. Both failures remain recorded.
  Destination creation and the actual helper fix them, without masking errors.
- `apk-03` completed the real Recipe pipeline but the independent oracle expected
  a hash field the captured target manifest does not promise. The corrected
  oracle still checks target UID/GID/mode and additionally compares every BusyBox
  byte against the digest-pinned APK member. It does not remove metadata checks.
- The diagnostic-3 build failed on a misplaced temporary field initializer.
  Two already-started diagnostic-3 consumer runs used the existing diagnostic-2
  binary; their recorded hash `ec45f393…` identifies that fact. They are not
  evidence for the failed source build. Diagnostic-4 compiled successfully,
  hash `fc91c7e6…`, and supplied the complete negative-result trace. Temporary
  diagnostics are absent from the final source and normal audit bounds remain.
- The Recipe build reused an existing build cache. Its old PRD18 executable
  bytes were preserved and restored at their historical path; SHA-256 remains
  `470e3bb2…`. The affected Cargo binary fingerprint was invalidated so a future
  current-source build must relink instead of reusing those restored historical
  bytes. New runs use only the separate immutable corrected binary.

## Checks, reproduction and remaining delivery dependency

Ginux `make check` and locked/offline release build pass. Seven existing ordinary
thread scenarios pass: basic, musl, umask, stress, fork, early and tiny. No
security/fault selector was run here. Recipe's locked/offline release build and
changed-file rustfmt check pass. Strict `cargo clippy … -- -D warnings` fails on
three pre-existing diagnostics outside the changed Git owner: nested format in
`needs/contract.rs`, iter/cloned/collect in `needs.rs`, and large FramePayload
variant in `worker/protocol.rs`. No lint was suppressed and no Clippy pass is
claimed; final CI disposition remains PRD13.

The retained [consumer runner instructions](../../tools/recipe/tests/consumers/README.md)
require explicit binaries and image, a new output directory for every attempt,
and the actual RPM/source lock inputs. For this candidate:

```sh
sudo -n env \
  RECIPE_TEST_BIN="$PWD/.artifacts/ginux-contribution-preparation/prd11/bin/recipe-corrected" \
  RECIPE_WORKER_BIN="$PWD/.artifacts/ginux-contribution-preparation/prd11/bin/recipe-corrected" \
  GINUX_BIN="$PWD/.artifacts/ginux-contribution-preparation/prd11/bin/ginux-corrected" \
  GINUX_ROOTFS="$PWD/.artifacts/ginux-env/prepared-image/rootfs.tar" \
  GINUX_ROOT=oci:recipe-compiler@sha256:c089f95c462582cc0fcc2a8b2ba86c7d2d64c22c82efa499ce20e3704a7fe15e \
  python3 tools/recipe/tests/consumers/run.py cargo \
    --output .artifacts/ginux-contribution-preparation/prd11/cargo-next
```

Each successful receipt includes its complete case-specific command. Final
source patches, build/check logs, exact identities and file manifests accompany
the receipts. The final manifest verifies **428/428 files**, including failed
reproductions, diagnostics and successful products; SHA-256
`9170d5fed6d480b35b202e73a4f8fa39c75585128868e5ee475ef9a8b91f3a31`. These are prepared-image/runtime checks, not a fresh-host
bootstrap claim or full production rootfs/package-solving acceptance.

PRD11 can close only after the actual maintained runtime/storage integration
is delivered, these corrections survive its owner replacement, and all required
consumers pass on that exact launcher/Recipe/image. The source choice remains
unapproved; the standing upstream-first preference remains. No new permission
request or invented user hold is needed to describe that dependency. Preserve
this local candidate and rerun affected consumers after future corrections.
[PRD09](prd09-security-acceptance.md) must separately verify affected task-hold
and paired Git/Recipe authority boundaries through its authorized context.

Active skill: `hdae-development`, `.agents/skills/hdae-development/SKILL.md`;
reload fully with required reference, repository instructions and this checkpoint
after context shift. Current task: PRD11, locally verified and incomplete for
final delivery. Next action: await actual upstream/final-source integration,
carry forward both corrections and retained consumers, then run all six on the
selected delivered combination. PRD09's returned partial receipts remain open.
