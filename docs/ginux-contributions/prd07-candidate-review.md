# PRD07 exact-source review (open delivery)

Current upstream main is `53a54addea67969abaa240ea478def1b94130eee`, refreshed
read-only on 2026-11-08. No publication has occurred in PRD07. These are local
candidate results; actual merges and final-source acceptance remain required.

## Independent candidates

| Candidate | Exact source / complete diff | Dependencies and disposition |
| --- | --- | --- |
| Bounded supervisor TCP library | `59f855ae7dbd60bacb93ef865928701b04305df9`, tree `05eed3bf673c89bc8c50b42af4f76d7fdc7362a5`; [816-line addition across three files](patches/PRD07-tcp.patch) | Directly on current main. Two commits: `2aa33b2e` feature extraction from retained G28 `0a3cf5f5`, then `59f855ae` review corrections. No storage, task, configured catalog or native-policy dependency. User directed this wave to remain local; no publication. |
| Truncation write gaps / empty writes | `a6dd2f9abc95f36f255808fa8353b0c96f00ade6`; [one-file +43/-1 diff](patches/PRD07-zero.patch) | Directly on main; retained `7a7e8b5c` adapted to the current descriptor shape. Conflicts textually with #232's write-path change. Hold publication until actual #232 merge, then adapt to shared offsets and revalidate. This is not a missing runtime dependency of the present standalone candidate. |

Both worktrees are clean under `/home/vince/levitate-development/`:
`ginux-prd07-tcp` and `ginux-prd07-zero-fill`. Source/patch/static-log identities,
per-command receipts and complete logs are in
`.artifacts/ginux-contribution-preparation/prd07/`. `tcp-source.json` and
`zero-source.json` bind those results to the patches. Neither changes a gitlink.

## Complete diff and failure-path findings

TCP's complete diff consists of `ginux-devices/src/lib.rs`, the README and one
additional Linux device-unit invocation in the existing workflow. Existing
registry, block and isolated-packet owners are preserved. The API copies and
validates a nonempty unique endpoint set, rejects malformed IPv4 tails/zero
ports and invalid limits, checks slot and address-family authority before the
backend, and publishes a handle only after successful backend setup. Every
read/write validates the connected slot, bounds progress to 4,096 and remaining
budget, charges only actual bytes, and rejects impossible backend counts.
Readiness reports transport state, including when quota subsequently denies I/O.
Close takes the slot before releasing it; repeated revoke and Drop cannot close
it twice. Linux closes a failed connection once, preserves EINPROGRESS, uses
MSG_NOSIGNAL, validates sockaddr lengths/families and never retries close.
Aliases are deliberately the caller's responsibility; scoped IPv6 addresses,
DNS, listeners and a general firewall are outside this API.

The first refreshed TCP source failed main's terminology check (`domain` in a
comment). Corrected to `instance`; final static checks pass. Review also found
unchecked `usize` to `u32` slot-ID conversion. The corrected owner rejects an
unrepresentable slot before mutating the table. A retained new refused-connect
case reserves a non-listening loopback port, accepts immediate ECONNREFUSED or
EINPROGRESS followed by SO_ERROR, checks unchanged quota, and releases/reuses
the slot. All four original TCP tests remain, including both IPv4 and IPv6;
the new case makes five. Main's 89 units and existing CI jobs are preserved.

Zero-fill checks descriptor rights before treating empty input as a no-op;
overflow/capacity failures precede any byte or offset mutation. Nonempty writes
clear the gap between current EOF and the retained writer offset before copying
data. Review exposed a second genuine bug in the original gap-only correction:
an empty write through the old descriptor extended a truncated file. The new
regression also checks that a rejected oversized write leaves EOF and offset
unchanged. Both tests fail on current main, gap-only passes the gap test but
fails the empty-write test, and the final candidate passes both. No old VFS
test is removed or weakened. These red/green sources, binaries and individual
exit/log hashes are retained in `zero-controls/`.

## Exact candidate validation and reproduction

`make check` and all commands below pass on each clean candidate: TCP has 94
unit cases; zero-fill has 91. Privileged commands run in the authorized host
environment for ptrace/chroot/setup. These are upstream's functional CI suites,
not final Daybreak security acceptance. No macOS result is claimed locally.

```sh
cd /home/vince/levitate-development/ginux-prd07-tcp
git rev-parse HEAD
git status --porcelain
git diff 53a54addea67969abaa240ea478def1b94130eee HEAD
make check
make test-unit
make build build-launch-fixture build-process-fixtures build-gate1-fixture
sudo make test-launch
sudo make test-native-supervision
make test-process-native
sudo make test-gate1-setup
```

For zero-fill substitute `/home/vince/levitate-development/ginux-prd07-zero-fill`
and require `a6dd2f9abc95f36f255808fa8353b0c96f00ade6`.
The receipt-producing runner is
`.artifacts/ginux-contribution-preparation/prd07/run-candidate-ci.py`; it refuses
a dirty source, records SHA/command/exit/log hash and checks source cleanliness
again. Candidate checks do not establish a merged storage integration or G29's
configured networking behavior.

## Existing required PRs: source review and current-main compositions

All six published heads remain unchanged and open. Their maintainer asks for no
additional changes while foundation review completes. Each isolated local
composition below equals Git's prospective merge tree, byte for byte, verified
by `git merge-tree --write-tree MAIN PUBLISHED_HEAD`. All six pass `make check`
and the seven CI command groups above. These are prospective merge results,
not actual upstream merges or a claim that remote macOS CI ran on these trees.
Full diffs, source/tree identities, logs and receipts are under the artifact
directory (`existing-compositions.json`, `prNNN-current-main.diff`, `prNNN/`).

| PR / published head | Local current-main candidate | Complete diff / failure-path disposition |
| --- | --- | --- |
| [#232](https://github.com/gmkurtzer/ginux/pull/232) / `5e79a172` | `2c3403e53ea9a5da57aecf5f2b7cfc3a791831df` (93 units) | One VFS file: per-open description offsets, alias-sharing, final-reference slot reuse, independent opens and per-descriptor CLOEXEC. Free descriptor bounds description admission before filesystem mutation. Four new cases and the intentionally corrected unlink/alias test survive. Keep zero-fill as a separate correction after merge. |
| [#233](https://github.com/gmkurtzer/ginux/pull/233) / `f751cd6f` | `1d129e6eeab3e43ce2626019577ea255efa27c78` (89 units) | Four files: consolidate host mkdirat number 258; use it in bootstrap; assert absent workdir becomes a directory. Existing workdir behavior, pre-drop creation and generic failure diagnostic are preserved. No #232 compile prerequisite. |
| [#234](https://github.com/gmkurtzer/ginux/pull/234) / `78a534c1` | `69de214ffdc68442f4e9480bdc7cf4410de994cc` (95 units) | Four files: both exit/signal paths adopt children only when no parent thread runs; copy parent TGID, preserve live grandchildren, adopt zombies; leader-only waits remain. All six distinct orphan unit oracles plus both repeatedly run native scenarios survive. T-03 still requires joining later configured aggregate tests during integration. Independent of storage. |
| [#236](https://github.com/gmkurtzer/ginux/pull/236) / `1f43d7fa` | `bbdf4fbe0d40dab7673a1d0e217a51feed0da992` (89 units) | Seven files: refuse directory standard streams; allow EBADF/ordinary streams; fail closed on other fstat errors. Native close_range retires inherited FDs before protected descriptors are created, and fails setup if unsupported. Full stream/regular/closed/O_PATH/high inherited-FD fixture controls retained. This protects workload inheritance; PRD04's supervisor budget/declared leases are a separate owner. |
| [#238](https://github.com/gmkurtzer/ginux/pull/238) / `efaeeca8` | `67680b6cd7a87ab1227d0b577cfbf86f6d808e8f` (90 units) | Seven files: deny target-setting/enabling fcntl/ioctl requests using kernel-width requests; ordinary NONBLOCK/descriptor flags remain. Core width controls plus both launch/native refusal fixtures retained. Catalog descriptions change, not wildcard authority. Logical security prerequisite to release, independently compilable from #243. |
| [#243](https://github.com/gmkurtzer/ginux/pull/243) / `067d992f` | `f525839730a85e39dff201670f2bfb9b477c460b` (91 units) | Ten files across two commits: resolved contract gate before effects, semantic-deny/syscall-deny precedence, explicit unforceable-deny startup refusal, bounded winning-rule audit and disabled decision records. Main's balanced surface remains the second gate; grants cannot widen it. vDSO/console/proc limitations are explicit. Full instance/recipe/exact/default deny, audit-source/index/cap/disabled tests retained. #238 is logical, not compile ancestry. PRD02's versioned configured catalog and later typed storage still need separate delivery. |

No additional current defect was found in these six diffs that justifies
rewriting a maintainer-held published head. Actual merged source may differ;
inspect it and rerun affected checks before using it as the next base.
