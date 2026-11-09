# Local Ginux PR updates — 2026-11-09

Prepared against current upstream main **55b719db9c52889543fbb177fab4cf2b2f90955e**,
confirmed again with `git ls-remote upstream refs/heads/main` after testing.
**No push, PR update, comment, closure or hosted run occurred.** This is local
contribution preparation. The maintained integration, release pins and frozen
security candidates were not changed.

Working checkout: [/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11).
Evidence and local PR drafts: [/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09).
The [original complete review](/home/vince/.codex/worktrees/prd15-owned-fd-path/ginux-prd11/.artifacts/open-pr-review-2026-11-09/report.md)
remains the source for all fifteen published heads, original full diffs and discussions.

## Exact local candidates

All entries below pass `make check build build-launch-fixture`, diff whitespace
checks and their targeted runtime runners on the listed committed revision.
Rebuilds match the retained launcher hashes. Each branch is based directly on
main55 except #243, which depends on refreshed #238. The original published
branches and their occupied worktrees were preserved.

| PR | Local ref | Full revision | Targeted runtime |
| --- | --- | --- | --- |
| [#233](https://github.com/gmkurtzer/ginux/pull/233) | `codex/pr233-current-main` | `e3b93f44a64c9443af1826b039df4766cf7da524` | launch |
| [#236](https://github.com/gmkurtzer/ginux/pull/236) | `codex/pr236-current-main` | `9b147ae28a7111b9e516edadb568a07afa81d02b` | launch and native |
| [#238](https://github.com/gmkurtzer/ginux/pull/238) | `codex/pr238-current-main` | `51f164c8e3970c54bf2b2da35d6f7148667322d7` | launch and native |
| [#242](https://github.com/gmkurtzer/ginux/pull/242) | `codex/pr242-current-main` | `480c8c348bc89f6818f2f8bf3e1a4f47bd91772c` | native |
| [#243](https://github.com/gmkurtzer/ginux/pull/243) | `codex/pr243-current-main` | `5c02ed877e6be55bf181087909d976e0a09ba55f` | launch and native |
| [#244](https://github.com/gmkurtzer/ginux/pull/244) | `codex/pr244-current-main` | `072d0ea8bef5bd18ae9f366e0584c7aeaad167f9` | native |
| [#245](https://github.com/gmkurtzer/ginux/pull/245) | `codex/pr245-current-main` | `fe363cf291173589af6bdbb38ef9a89d7fb067f9` | native |
| [#246](https://github.com/gmkurtzer/ginux/pull/246) | `codex/pr246-current-main` | `02cd907b5b766cd9ff7503e152d8ecf984f000fd` | native |
| [#247](https://github.com/gmkurtzer/ginux/pull/247) | `codex/pr247-current-main` | `4c09332eec5df52a9fc6ea185616d2f25246d988` | native |
| [#248](https://github.com/gmkurtzer/ginux/pull/248) | `codex/pr248-test-successor` | `a276afe12fe3f50bf86b23b1811c72e7d870926b` | native |
| [#207](https://github.com/gmkurtzer/ginux/pull/207) | `codex/pr207-current-main` | `401aa878888a929e2d8eb7269691aa70b8665a4e` | launch and native |

[Final exact command receipts](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/final-exact-results.json)
map every command to its head, exit status and log. Initial failures are
retained separately: the #244 audit deadlock, #233 unused import, #207 loop
lint and obsolete #243 schema/audit test assumptions. Corrected final checks
pass; old failures are not counted as successes.

## Findings and replacement map

### #238

Retain the signal_driven_io helper, core 32-bit request tests, fcntl allowed/denied launch and native cases and regenerated catalog documentation. Add the missing guard before native mode 3 ioctl admission.

Safe invalid-FD runtime controls establish the actual owner: baseline requests return EBADF, candidate requests return EPERM; unrelated TCGETS remains EBADF. No signal-driving host-target probe or broad security acceptance is claimed.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/238.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/238-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/238.json).

### #242

Replace unconditional PTRACE_SYSCALL with PTRACE_SYSCALL only when awaiting_exit, otherwise PTRACE_CONT, delivering SIGCHLD in both cases. Migrate the fixture and retain blocked-poll plus userspace-busy handler cases.

Standalone. Old profile-string substitution is replaced by a hierarchical contract and explicit configured invocation. Original blocked-handler test fails on main; both final controls pass.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/242.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/242-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/242.json).

### #233

Share syscall 258 from supervisor.rs and retain the missing-workdir directory regression. Remove the now-unused native declaration/import after upstream virtual cgroup ownership.

Standalone; no dependency on storage redesign. Initial unused import lint is corrected in the final revision.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/233.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/233-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/233.json).

### #236

Retain stream checks, pre-bootstrap close_range, high FD 239/242/300 checks, protected 240/241 checks, regular-file and closed-stdin behavior; add native O_PATH stdin refusal.

Standalone. Standard stream fstat errors other than EBADF refuse startup. Private descriptors are created only after inherited cleanup; public limits stay unchanged.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/236.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/236-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/236.json).

### #243

Retain runtime_rule precedence, semantic classification, contract provenance, unsupported-denial refusal, network non-reachability assertion and disabled-audit behavior. Rewrite legacy profile/userspace tests for contract catalog 2 and bare syscall names. Reject native-only typed controls instead of ignoring them.

Depends on refreshed #238. Global and syscall deny provenance is exact-deny under the new schema. The 65536-byte decision cap remains strict; larger normalized configuration is separately bounded. Resource metadata is not launch enforcement; host vDSO/proc/sys/input alias limitations remain explicit. No downstream storage/catalog-1002 integration is included.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/243.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/243-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/243.json).

### #244

Upstream already handles managed-child P_PIDFD waits and si_pid translation. Retain positive wait, self ECHILD and non-pidfd EBADF controls; add invalid FD and high-bit alias/WNOWAIT controls. Normalize id_t to low 32 bits and return EBADF/ECHILD rather than generic EPERM on rejected targets.

Do not restore the obsolete whole wait implementation or its thread-parent restriction. The negative case exposed an early-continue audit-lock deadlock; the corrected denial finishes its record before resuming. The inherited #242 runtime patch is removed; no #242 runtime dependency.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/244.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/244-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/244.json).

### #245

Retain clone3 child creation/status 7 test, migrate to NativeVirtualKernelFd ownership and validate that the caller owns an existing virtual cgroup directory before clearing CLONE_INTO_CGROUP. Add invalid and ordinary FD denials; finish denial audit records.

The child starts in the parent host cgroup. This does not implement host placement, virtual membership attachment or cgroup resource enforcement. The original in-place clone_args translation remains: read-only/unreadable flags cannot be rewritten. Mutable clone-argument schedules, restoration of caller bytes and complete Linux clone3 ABI equivalence are not established. The inherited #242 runtime patch is removed.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/245.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/245-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/245.json).

### #246

Retain PIDFD_GET_INFO ENOTTY fallback and child-wait control. Match the 32-bit request family while ignoring growing struct-size bits; add upper-register/size variants, invalid-FD guard and unrelated-ioctl control.

The upstream synthetic FD ENOTTY path does not protect real pidfds. The new real-pidfd control ran only on the guarded candidate; no host-information disclosure probe ran on main. The inherited #242 runtime patch is removed.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/246.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/246-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/246.json).

### #247

Retain WNOWAIT peek then reap and 200-child reuse regression. Release exit-status children only (CLD_EXITED/KILLED/DUMPED), after tracing ends, and fail explicitly on invalid process-table accounting.

No slot release on WNOWAIT, stop or continued notifications. The updated path still requires non-null siginfo. Linux waitid with null siginfo and concurrent siginfo mutation/races are outside the verified contract; general waitid accounting closure is not claimed. The inherited #242 runtime patch is removed.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/247.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/247-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/247.json).

### #248

Upstream NativeVirtualKernelState replaces the old host-backed mkdir helper. Keep a test-only successor covering mkdir, duplicate EEXIST, cgroup.procs write, events/kill opens, absolute mkdirat, relative virtual dirfd and mirror-symlink containment.

Old host materialization assertions are replaced with workload API results and absence of unexpected host writes. A task-owned outside sentinel replaces the unsafe global escape path. The virtual owner ignores the planted host mirror and returns ENOENT. No old host-tree code or inherited #242 runtime patch is restored.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/248.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/248-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/248.json).

### #207

Poll a private array over borrowed FDs 0–2, with 16 slots and timeout 0–300000 ms. Retain all original bounds, closed-FD and readiness tests; add positive 16-slot, read-only result and repeated-query cases in launch and native.

The old validate-then-forward helper is replaced. Upstream #179/#207 policy hold remains open. This is an unpublished proposal, not adoption in maintained native policy. Copy-out uses upstream process_vm_writev with its existing ptrace fallback and reduced read-only fidelity if unavailable; normal-path copy-out was verified. No adversarial mutable-memory schedule or general signal restart claim.

[Full own-change diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/207.patch), [local PR draft](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/207-pr-draft.md), [source and launcher identity](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/207.json).

### #232, #234 and #241: retained without implementation churn

The published heads remain `5e79a172`, `78a534c1` and `a0a304f2`. Their clean
merges against this unchanged main already had exact-source evidence in the
original review. The joined verification below repeats their relevant checks:
16 VFS units, 19 process units, both orphan runtime scenarios and the real
OCI tar/index proof. #234's native host-wait limitation remains explicit; it
does not replace #247's waitid accounting. No original regression was removed.

### #250: superseded by merged #256

The workflow, classifier and classifier test blobs are byte-identical to main;
the Makefile target is present. `make test-ci-changes` passes.
[Exact equivalence](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/250-equivalence.json),
[classifier results](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/250-ci-equivalence.log) and
[local disposition](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/250-disposition.md) preserve the basis.
No duplicate implementation refresh is needed. No remote closure/comment was made.

## Combined compatibility verification

A separate disposable validation ref, `codex/pr-refresh-validation`, joins all
eleven prepared candidates and the unchanged #232/#234/#241 heads. This is a
compatibility experiment, including the still-held #207 proposal, not a new
maintained source or native-policy selection.

Source: **`93749c3aeaa6022e55f68bfb3314318957cb7228`**. Launcher SHA-256: **`fd2eae5263481c3aab7856bbed784bff49ac136aebe46e6a7c4e8f272af44e04`**.

Checks on this exact joined tree:

- `make check test-unit build build-launch-fixture`: pass; **105 unit tests**, zero failures.
- `sudo -n make test-launch test-native-supervision`: both runners pass, including **21 native checks**.
- Real producer-verified Rocky OCI tar/index with `ginux vfs`: `GINUX_VFS_CHECK: PASS`.
- `orphan_reparent` and `orphan_zombie` via `test-process-native.py`: **20 successful runs each**.

[Combined receipts](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/joined-results.json) and
[complete combined diff](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/joined.patch) are retained. Merge
resolutions preserve all assertions: identical configured setup is shared,
#236 stream options remain available, #248 uses a distinct task-owned outside
directory, and the combined native poll scenario uses `b` to avoid #244's `p`.
No test is disabled to make the combination pass. These compatibility-only
resolutions do not move any standalone candidate ref.

## Reproduction

Run in the working checkout, with a clean tracked tree. These commands switch
only this isolated checkout. Root is needed by the native/launch chroot harness.

```sh
git switch codex/pr238-current-main
git rev-parse HEAD
make check build build-launch-fixture
sudo -n make test-launch test-native-supervision

git switch codex/pr242-current-main
make check build build-launch-fixture
sudo -n make test-native-supervision

git switch codex/pr-refresh-validation
make check test-unit build build-launch-fixture
sudo -n make test-launch test-native-supervision
make build-process-fixtures
python3 scripts/test-process-native.py target/x86_64-unknown-linux-gnu/release/ginux .artifacts/process target/x86_64-unknown-linux-gnu/release/process-native-fixture orphan_reparent
python3 scripts/test-process-native.py target/x86_64-unknown-linux-gnu/release/ginux .artifacts/process target/x86_64-unknown-linux-gnu/release/process-native-fixture orphan_zombie
```

The complete sequential exact-candidate procedure is
[`verify-all.py`](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/verify-all.py). Numbered JSON identities contain
full bases and launcher hashes; numbered patches contain the full per-PR
changes. #243's [full diff including #238](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/243-full.patch) is also retained.
Compiler/kernel details are in [environment.txt](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/environment.txt).

## Publication order and boundaries

1. Keep #232/#234/#241 available unchanged. Independent refreshed fixes are #233/#236/#238/#242.
2. #243 follows #238. Narrowed #244 and refreshed #245/#246/#247 have no inherited runtime dependency on the obsolete #242 patch; their configured fixture setup must be reconciled when combined, as verified here.
3. #248 has a test-only successor; replace the old implementation proposal with this disposition when a later upstream action is authorized. #250 needs a superseded disposition, not another implementation.
4. #207 remains a local proposal pending the maintainer's native-policy direction under #179.

Every push, PR text edit, comment or closure still needs explicit permission.
Local drafts are suggestions; existing human-authored public prose is unchanged.

No full VM/systemd boot, adversarial schedule, hosted/macOS, final delivery
integration or global security acceptance is established by these checks.
The #245 clone-argument and #247 null-siginfo limitations above remain visible;
they must not be presented as complete Linux ABI coverage. Recipe production
readiness and the existing release/security blockers are unaffected.

[Preservation receipt](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/preservation.json) confirms the maintained
head and frozen PRD04/export heads, and the original staged task-list/handoff
patch hash `f4d8c81be2612a4d8d472564b25c7ffab6e7043e06b0f2b2176cba78dad5a47e`.

Active skill: hdae-development at
`.agents/skills/hdae-development/SKILL.md`; full reload after context shift is
required. This local preparation is complete within the bounded contracts
above. No upstream delivery action is inferred. The resumable evidence
checkpoint is [STATUS.md](/home/vince/.codex/worktrees/ginux-pr-updates/ginux-prd11/.artifacts/pr-updates-2026-11-09/STATUS.md).
