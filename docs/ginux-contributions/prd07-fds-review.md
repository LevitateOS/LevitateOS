# PRD07 G05 — public descriptors and pipes (local)

The user authorizes continued local extraction/integration and prohibits upstream
publication. Upstream Ginux takes precedence. This candidate depends on the
proposed storage/device chain; it is neither standalone on current main nor an
upstream/equivalent delivery disposition. G06/G07 task ownership is still required
before ordinary child-command or concurrent pipe behavior can be claimed.

## Exact candidate and prerequisite boundary

Clean `/home/vince/levitate-development/ginux-prd07-fds`:

- Revision `997ffae2800d40d8ace5944d7fa4e8efa450b91f`.
- Tree `207719e60a28e9fc3fb49c75df0b023013359946`.
- Direct base: [G04](prd07-devices-review.md)
  `0bd9b4bb8970946080b98672064de20fb3aebc11`, over typed storage `16dd447e`.
- Launcher SHA-256:
  `0ba8eca3ec451f64203f5b921ed8e58f0aaf1c9c45cbdc76df6b4e41351e1cd1`.
- [Complete eighteen-file +1430/-119 diff](patches/PRD07-fds.patch), SHA-256
  `f32fea3b3c2ab46b61ddf41c6238853398e32f1836f3c4cc8703bbc3d0093e2c`.

The source is one coherent descriptor/pipe-owner commit. Current-main refresh
still finds `53a54addea67969abaa240ea478def1b94130eee`; the open storage
prerequisites remain as recorded in the [delivery waves](prd07-delivery-waves.md).
The exact local chain is main → #232/#233/#238/#243/zero-fill/#236/configured
catalog → PRD03 typed storage/catalog 3 → G04 → this candidate. Actual merges,
upstream foundation changes and PRD05's real merged composition require a fresh
join and validation. #234 is a prerequisite of the later configured process
join, not of this single-task primitive.

## Retained donor, every implementation/test owner

Donor `2718b860f04bc644c001d7a9cea906a74ddd386a` is semantically extracted;
its obsolete filesystem adapter is not replayed over the redesign.

| Donor paths/hunks | Disposition in this candidate |
| --- | --- |
| `descriptors.rs` | Retained bounded public table, reservations, aliases, CLOEXEC flags, native exit transactions, argument restoration and pipe allocation rollback; adapted to `LaunchFs` and corrected as below. |
| `channel_hooks.rs` | Retained all owned-pipe state, IO/copy accounting, EOF/SIGPIPE, metadata, flags and bounded allocation. Native hook stubs remain unchanged in behavior, including const `claims` needed by #243. The private claim range is narrowed to the actual sixteen endpoints. |
| `dispatch.rs`, `launch.rs` | Replaced the old separate filesystem/device hook join with current composed namespace dispatch. Retained native fallback, entry parking and descriptor exit translation. Lost path-scratch restoration remains fatal; it is never reduced to a guest errno. |
| Old `fs_hooks.rs` and `tracee_io.rs` changes | Replaced by the existing `launch_storage` protected copies/stat serializer and a small native-limit helper. No duplicate resolver or IO owner introduced. The now-unused device `enabled` getter is removed. |
| `main.rs`, `supervisor.rs` | Retained register Copy/Clone and exhaustive native match; unexpected configured states fail closed. Native dispatch never selects the new states. |
| Catalog-1 `pipe` entry/test exception and ABI table growth | Replaced by existing explicit configured catalog 2/3; catalog 1 is unchanged. The pipe runner selects catalog 2 without storage and catalog 3 with storage. No core/contract/instance source delta exists against the base. |
| Original FD/pipe C fixtures and Python runners | Retained all original scenarios and assertions, adapted Instance/config terminology. F_SETOWN expects upstream #238's EPERM instead of the older donor's EOPNOTSUPP. Added the ordinary owner/deny/ABI controls below. |
| Storage/device fixture changes | Retained all current assertions and namespace cases. Private-number expectations become public-number expectations; F_DUPFD_CLOEXEC minimum is 10 rather than private handle 1048586. The audit oracle still checks the exact result, now 3. No directory, 9,000-byte IO, metadata, bounds, copy-protection or traversal oracle is removed. |
| Make/CI and README | Retained separate FD/pipe build/run targets alongside every original CI step. Rewrote obsolete private-FD/unsupported-directory/dup2 claims for the actual bounded candidate. |

## Contract and reviewed failure paths

One public table owns at most 256 descriptors, or the operator's lower inherited
soft limit. Existing open stdio remains represented; allocation uses the lowest
free slot. Each entry selects a native image/console open object or a private
memory/device/pipe handle. Public FD arguments use Linux's low 32 bits; negative
or absent slots fail EBADF. Backend integers do not confer public authority.

Open reserves a public slot before create/truncate/native effects. Native opens
publish their child-kernel handle only at the matching exit; failure releases the
reservation. Native IO/path/mmap arguments are translated and restored at exit,
after any protected path scratch is restored. Duplication shares the selected
open object; close retires the backend only at the last alias, and descriptor
CLOEXEC flags remain local to the slot. Native close ownership drift fails the
supervisor. NOFILE changes are denied rather than invalidating the allocator;
bootstrap lowers real soft/hard limits without raising either bound. This is
not the later inherited-supervisor-FD/backing-lifetime acceptance in G17/PRD04.

Contract admission precedes table mutation, backend effects and path scratch.
Console identity follows its open object across aliases, while redirected stdout
uses the replacement owner's filesystem authority. Other read/readv calls use
the accepted filesystem-read classification. The existing explicit console-input
deny startup refusal remains: aliases alone do not prove every alternate input
path is controlled. Native balanced policy remains separate and unchanged.

Eight supervisor-memory pipes hold 65,536 bytes each. Pipe allocation reserves
two public slots before backend/output publication and rolls back both ends on
copy failure. Transfers are bounded to 4,096 bytes; writes within that size are
atomic, and read consumption follows actual copyout progress. EOF, nonblocking
EAGAIN, shared status flags, per-slot CLOEXEC, SIGPIPE, capacity and alias release
are covered. Blocking operations stay stopped at entry. This single-task stage
has no runnable peer; only the tested termination/reap path is claimed here.

| Finding or failed run | Concrete disposition/evidence |
| --- | --- |
| Old donor rejects relative directory FDs | Replaced with translation into the existing native/memory resolver; current full storage runner passes native/memory anchors, fchdir, enumeration and removed-directory identity. |
| Older F_SETOWN unsupported error conflicts with #238 | Existing low-32-bit signal-driven-IO guard runs before table dispatch/effects and preserves EPERM; original launch owner scenario and the FD fixture pass. |
| `fcntl` command width selects a different owner than the VFS | Initial port fails new fixture code 69. Normalize the command to Linux's unsigned 32-bit value before choosing public duplication/flags. Same control passes; the copied initial sources/patch/binary and red log are retained. Dup2's target narrowing was already correct and is preserved/tested. |
| Unsupported owned operations can fall through with a private handle | Return EOPNOTSUPP rather than native fallback. Non-directory owned fchdir returns ENOTDIR; pipe getdents and null-pointer AT_EMPTY_PATH metadata controls pass. |
| Pipe runner default catalog denies historical `pipe` | Initial full-write mode fails code 13. Runner now explicitly selects the approved configured catalog; no wildcard authority is added to catalog 1. |
| Old storage tests/audit use private FD numbers | Failed code 208 and subsequent exact audit-oracle failure retained. Replace only number expectations with the new public contract, including the minimum-10 alias. Full 40-case runner passes. |
| Partial initial source port/unused getter | Initial compile logs retain incomplete exhaustive arms, then the dead getter. Complete current dispatch and remove the obsolete getter; no warning/test suppression. Generated Python bytecode was removed before clean-source CI. |

G06/G07 still own child/exec lifecycle, inheritance, successful exec CLOEXEC
retirement, concurrent wakeups/retries, full blocking writes and the mature
shared-argument/kernel-use holds. Join #234's orphan accounting and preserve
bootstrap signal-mask clearing and caller-thread SIGPIPE when concurrency is
introduced. G08 owns readiness; G12 owns memory-file mappings (including retained
T-01's mmap-offset=3 assertion); G17 owns backing/inherited-FD lifetime. Unsupported
owned mmap remains explicit EOPNOTSUPP here. None is silently declared delivered
by this candidate, and no final-source security acceptance is inferred from
ordinary CI.

## Exact-source validation and reproduction

`make check` passes. All 141 existing units and all original Linux launch,
native-supervision, process-native and setup checks pass on clean `997ffae2`.
The complete 40 storage cases, ten device configurations, seven FD configurations
and six pipe configurations pass, including lower NOFILE 8, native backend-number
reuse/mmap, public-table exhaustion without truncate, aliases/register restoration,
three denied-family controls, SIGPIPE, blocked read/write SIGKILL and fresh launch.

```sh
git rev-parse HEAD
git status --porcelain
make check
make test-unit
make build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-launch-fd-fixture build-launch-pipe-fixture build-process-fixtures build-gate1-fixture
sudo make test-launch
sudo make test-launch-storage
sudo make test-launch-devices
sudo make test-launch-fds
sudo make test-launch-pipes
sudo make test-native-supervision
make test-process-native
sudo make test-gate1-setup
git diff --exit-code 0bd9b4bb -- crates/ginux-core crates/ginux-contract crates/ginux-instance
```

Evidence under `.artifacts/ginux-contribution-preparation/prd07/`:
`fds-source.json`, `fds/receipts.json` and complete thirteen-command logs;
`fds-final-check.log`; `fds-base-control/` (base fails public-number fixture code
10); `fds-command-control/` (copied initial source/patch/binary, code-69 red receipt
and manifest); the initial/profile/public/audit runtime directories preserve each
failed integration oracle. Earlier uncommitted-port checks are diagnostic,
not exact-candidate acceptance. No macOS, Recipe consumer, concurrent task or
Daybreak result is claimed for this source.

## PR draft — do not submit

Title: **feat(launch): unify public descriptors and owned pipes**

Configured launch now uses one bounded public descriptor table for stdio, image
files, memory files, assigned devices and anonymous pipes. Duplication shares
backend lifetime while CLOEXEC remains descriptor-local; reservations precede
mutating opens and native arguments are restored at syscall exit. The composed
namespace retains directory-FD support and contract denials precede effects.
Catalog 1 and native policy remain unchanged.

The ordinary Linux suites, 141 units, full storage/device runners and retained
FD/pipe scenarios pass on the pinned revision. This bounded single-task change
requires its actual storage/device prerequisites, and later process integration
is needed for child commands, concurrent pipe progress and exec retirement.
Rebase/review/revalidate against actual upstream merges before any proposed PR;
publication is currently prohibited by the user.
