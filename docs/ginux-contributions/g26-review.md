# G26 — inherited process resource bounds (partial)

Current release contract: [PRD08](prd08-public-contracts.md) records the user's
configured downstream approval and explicit limits, independently of upstream
holds. The candidate/evidence below remains historical; its security, current-
main delivery and final-source acceptance are not upgraded by that decision.

Two source packets are retained from0c571cbb. They are work units, not a promise
of two PRs. Public metadata-to-enforcement policy, thread/security acceptance,
full CI and refreshed upstream disposition remain pending.

| Packet | Exact head | Base | Patch |
| --- | --- | --- | --- |
| G26a bootstrap | `1d0824e6634aca994a38a9f9531b90344c787ac7` | main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177` | [5 files,+103/-7](patches/G26a.patch) |
| G26b task queries/guards | `2ed520c4437a8459c70dc79208d40b2923390939` | G06b/G26a composition `687eb942fb9d0628558d1d25094f5d53a9e3094f` | [5 files,+116/-6](patches/G26b.patch) |

G26a applies checked memory_mib-to-bytes conversion, then child-only hard/soft
RLIMIT_AS=min(configured bytes, inherited soft, inherited hard). It selects at
most vcpus logical CPUs from the launcher's inherited affinity mask before exec.
The1024-byte bootstrap mask supports CPU IDs within8192 bits; unavailable bounds
fail bootstrap explicitly. Configuration already validates1–64 vcpus and16–1048576
MiB, without changing schema. This bounds per-process virtual AS and eligible
CPUs, not aggregate RSS, aggregate storage or CPU time. Parent/global policy is
unchanged. Existing core policy still denies workload affinity syscalls on main.

G26b uses the existing registered process task and strict copy_memory owner for
self sched_getaffinity queries (zero, virtual PID/TGID), bounded to1024 bytes.
Masks retain actual logical CPU IDs from the operator subset; CPUs are not
renumbered as virtual CPU IDs. Affinity mutation returns EPERM; existing public-FD
prlimit mutation guards also
cover RLIMIT_AS. It introduces no host PID/FD assignment, generic scheduler,
thread ownership or new core syscall rules. Kernel inheritance, rather than
extra bookkeeping, retains the bounds through supported process fork/exec.

Separate full-diff self-review inspected both patches and current config bounds,
bootstrap failure/privilege/FD ordering, native prlimit rules, registered task
identity, query copyout, fork/exec/death, descriptor guards, fixtures, docs and
CI callers. This is self-review, not independent or adversarial review.

- **Arithmetic correction:** donor's unchecked memory-size multiplication now
  uses checked conversion/multiplication even though current config bounds fit
  the x86-64 representation. Invalid/hostile boundary probes were not executed.
- **Minimal split:** bootstrap needs no unmerged process/thread/storage packet;
  task-aware queries use G06b. G26b's local composition preserves subreaper,
  private descriptor reservation, existing CLOEXEC-stdin setup and all prior
  launch fixtures. Plain patch-check conflicts were recorded, then these owners
  were reconciled explicitly; no failed partial patch was applied.
- **Required thread join:** donor queries always use the calling task's host ID.
  That is correct here, where each supported process has PID==TGID. A future
  G07 nonleader query of its TGID must resolve the registered leader's host ID;
  mark that interaction unresolved at F02 rather than claiming thread acceptance.
- **Contract review still required:** prior resource metadata now enforces AS and
  CPU eligibility. No maintainer consent or fresh policy resolution is inferred.
  G26a permits the existing native prlimit behavior under its hard bound; G26b
  adds the explicit AS-mutation denial. State the distinction when delivering.

Source and strict clean replay pass make check test-unit build and existing
launch/task fixture builds: G26a65 units/150 matching tracked paths;
G26b82 units/168 paths. Respective Ginux and fixture hashes match. Ordinary checks
in both copies demonstrate32 MiB AS, lower24 MiB operator limit,64 MiB virtual
reservation refusal, successful1 MiB mapping touching only two pages, and brk
refusal without host pressure. The task-aware composition verifies exact masks:
operator CPUs[0,1], configured1 gives[0], configured3 retains[0,1], including self
queries by zero/virtual PID, fork and exec. The bootstrap helper's source bytes
are checked identical between G26a and G26b. Host runner limit/affinity stays
unchanged. Ten existing ordinary process lifecycle/control modes also pass.

Fresh exact main lacks the AS cap and the final fixture exits160. Exact G06b/G26a
composition installs bounds but lacks self affinity queries and exits207. Receipts
include exact base/binary hashes. Existing Make/workflow invocations already run
the new launch/task cases with all old default scenarios; no coverage is removed.
The first G26a build command ran from /tmp and failed without changing source;
correct-cwd builds above are the actual evidence, not that failed attempt.

[DaybreakBlue](daybreakblue-handoff.md) owns mutation/bypass/authority, invalid or
protected/mutable buffers, syscall/CPU-ID boundaries, hostile exhaustion, CPU
hotplug/reconfiguration, failures and races. Those probes and cloud dispatch are
unrun. Full/default Linux runtime, macOS/remote CI, independent review, G07 thread
and final combined consumer/F02/F03 acceptance are pending. G26b is not standalone
until its actual prerequisites merge. G26a source applies alone to recorded main,
but fresh main/public contract/security/full CI still prevent ready-to-submit claims.

Retained patches, generic drafts, command/log receipts, exact SHAs/hashes and
helpers live in .artifacts/ginux-contribution-preparation/G26a/ and G26b/.
G26b retains its full prerequisite reconstruction patch. No Ginux real checkout,
index, branch/ref or remote text changed; no PR or maintainer message was sent.
Publication stops at F04 for user inspection.
