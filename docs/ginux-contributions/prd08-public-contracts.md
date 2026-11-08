# PRD08 — supported configured contract and delivery impact

PRD08 is complete. The user approved **Recipe configured contract 1** as the
supported downstream release contract below, independently of unresolved
upstream/native policy. The user accepted the listed resource, CPU identity,
numeric endpoint and poll signal limits, and selected zero-filled configured
`sysinfo`. These are explicit release decisions, not inferred maintainer consent.
They authorize neither publication nor PRD01's final source-packaging choice.

The verified local candidate is **`4aa51c1d72664c7d9425d8832dd82a2ce684a1f9`** in
`/home/vince/levitate-development/ginux-prd08`, based on `5300f5ef`. Its launcher
SHA256 is `7a0b8a16a3a1048046076fe0aaea4654cead5b66965336646ff368d3fff7b137`.
It preserves the authorized configured clock implementation from `a7ce73cd`
byte-for-byte and keeps the pinned PRD03 `9802ef03` and PRD04 `5449e2a1` sources
and binaries unchanged. Nothing was pushed, published or sent to maintainers.

This is PRD08's verified successor delta, not a release-ready aggregate. It is
based on `5300f5ef`; PRD04's read/readv semantic classification, inherited-FD,
assignment-prefix and backing-close corrections still need composition, and its
final-export failure still needs correction. The contract below is required of
the shipped candidate; PRD08 does not certify all existing enforcement paths or
close those known defects.

## Supported interface and versioning

This contract covers **`ginux launch`**, including Recipe's `ptrace-native`
backend. That backend name does not select the public **`ginux native`** command.
Recipe selects `configured-balanced-v2`, configured catalog 2, instance schema 1;
self-contained contracts may explicitly select that catalog. The downstream
contract version describes the supported behavior and limits; the catalog version
identifies operation authority. Neither changes Recipe API 2 or transport 6.
Old catalog 1, native profiles and their selector/digest authority remain intact.

PRD03's `configured-storage-v3`/catalog 3 adds the storage operation set and must
retain this configured contract when PRD05 composes the actual source. PRD18's
later NEEDS-derived contracts must use the selected versioned catalog and existing
central enforcement. No NEEDS implementation or final merged-source acceptance
is implied here. PRD01 must choose a reproducible maintained source; an arbitrary
private binary or old gitlink cannot satisfy delivery.

The effective contract gate runs before configured effects, including clocks,
`sysinfo`, descriptor/backend translation and native argument rewrites. Explicit
deny wins. A backend cannot grant an operation denied by that table; endpoint,
assignment, object access and capacity checks further restrict allowed operations.
An allowed catalog operation is a permission ceiling, not a claim that every
Linux flag, backend or ABI variant is implemented. Unsupported variants retain
their documented errors. Audit-off and audit provenance retain PRD02's contract.

## Required shipped behavior

| Surface | Supported contract | Exclusions and error boundary |
| --- | --- | --- |
| Global clock queries | Raw `clock_gettime`/`clock_getres` accept signed 32-bit clock IDs 0/1: actual host CLOCK_REALTIME/CLOCK_MONOTONIC. `clock_getres` accepts a null result. | No virtual instance epoch. CPU/dynamic/other raw clock IDs return EPERM. Kernel vDSO clock access remains available and may answer additional global IDs without a syscall; this is not a clock-observation isolation guarantee. Explicit unsupported clock/vDSO denials are refused before launch. |
| Relative/absolute sleeps | `nanosleep` and `clock_nanosleep` support relative waits and TIMER_ABSTIME for those clocks. The requested timespec is copied once. Relative waits use monotonic time. Valid initial duration is 0–300 seconds inclusive; longer relative duration returns EINVAL. Past absolute deadlines complete. | Unsupported clock IDs return EPERM; invalid flags/times return EINVAL; copy failures return the underlying error. Kernel poll waits use scalar arguments and quanta of at most 100 ms. This does not grant guest `poll` authority or change native clock policy. |
| Clock interruption and adjustments | Caught signals interrupt waits with EINTR even with SA_RESTART. Remaining time is returned for relative requests; absolute requests do not write it. Absolute realtime deadlines are rechecked after adjustments. A backward step cannot indefinitely extend a wait: a 300-second monotonic budget ends it with ETIMEDOUT if the realtime deadline remains future. | This is a monotonic budget, not hard realtime scheduling. A final poll quantum and host scheduling can delay observed completion; suspension/descheduling is not bounded by this API. No sleep is reported complete early merely because a relative realtime clock moved. |
| Configured readiness | `poll` snapshots at most 16 public descriptors and resolves them through their owners. Pipes, local packets, assigned memory/null, inherited console aliases and TCP are supported. Negative FDs are ignored; closed FDs report POLLNVAL. Finite 0–300000 ms and negative infinite poll waits are supported. Finite/console waits use a 1 ms scheduler tick. | Image-file readiness is unsupported. No validated guest pollfd array is passed to a later kernel poll. This removes the specific #207 kernel-array validation/use dependency from configured readiness; it is not blanket race/security acceptance. General nonfatal signal interruption/restart is not promised. |
| ppoll | Supported in the configured TCP/readiness owner with a copied initial timeout and layout, null signal mask and millisecond rounding. | No temporary signal-mask implementation, general signal interruption/restart guarantee or new unbounded scheduler authority. Domain cancellation/kill and teardown remain separate supported lifecycle operations. |
| Networking | `none` disables TCP. `mediated` requires explicit numeric IPv4/IPv6 IP:port endpoints (up to 8), 1–16 concurrent connections and a positive aggregate sent/received byte budget. Private supervisor sockets retain public descriptor/alias/fork ownership. Budget exhaustion is EDQUOT and connection capacity exhaustion is EMFILE. | No DNS, listener, datagram, general socket API, hostname/HTTP-path isolation or network-namespace/firewall claim. A numeric endpoint grant can serve multiple hostnames/protocols. DNS/origin selection belongs to Recipe's coordinator/helpers and TLS verification to the caller. Live sockets are not exported/restored. |
| Network configuration models | Upstream #229's `shared` backend, attachment/source, ingress/egress and translated ports belong to native instance networking. Config validation rejects those shared-only fields with `mediated`/`none`. Recipe emits only its supported configured model. | `shared` is not a supported configured-launch attachment mode and does not turn configured calls into native host networking. No automatic fallback from a mediated endpoint grant to host-shared networking. #229's merger does not approve the separate configured backend. |
| Identity | Configured `uname` returns Linux, instance ID, the Ginux kernel-views package release plus `-ginux`, version `ginux`, `x86_64`, and empty domainname in the six 65-byte fields. Task/credential queries and mutations use their existing registered virtual owners. | This release string names the userspace contract implementation, not a complete Linux kernel ABI. No general PID namespace or host-topology anonymity claim. Existing vDSO clocks, eligible CPU IDs, permitted socket metadata and facts in selected input images remain observable. |
| sysinfo | Configured syscall 99 returns exactly **112 zero bytes** for x86-64 `struct sysinfo`, including padding. This reveals neither host uptime/load/RAM totals nor a fictitious aggregate instance budget. Copyout uses the existing strict helper; failure can leave a written prefix and returns its error. | Native `sysinfo` is unchanged. This view is not resource accounting. Denied syscall 99 returns EPERM before any output change. Invalid/protected/shared-buffer security acceptance remains with PRD09 on final source. |
| Memory resources | `memory_mib` installs child-only hard/soft RLIMIT_AS, capped by lower inherited soft/hard limits before exec. Supported descendants inherit it. Address-space-limit mutation is denied. Storage has its own checked data/metadata capacities within its declared configuration. | Not aggregate process-tree RAM/RSS, supervisor-plus-guests memory, memory bandwidth or a cgroup accounting guarantee. A collection of processes can have more aggregate memory than one process's AS bound. No false aggregate budget is reported through sysinfo. |
| CPU resources | `vcpus` selects at most that many eligible logical CPUs from inherited affinity; fewer available CPUs remain valid. Self/own virtual PID/TGID queries report that actual subset through at most 1024 bytes. Mutation is denied. Bootstrap fails if bounds cannot be installed. | CPU IDs are real eligible IDs, not renumbered virtual topology; the mask supports IDs below 8192. This bounds eligibility, not aggregate CPU time or usage percentage. Recipe's execution deadline is a separate coordinator contract. |
| Configured I/O | Configured image read/pread requests retain a finite **64 MiB** admission bound, preserving actual multi-MiB compiler/header reads. Kernel short reads/EOF remain possible. Owned files, pipes, vectors and console operations retain their existing owner-specific chunk, capacity and partial-progress rules. | This is not a general scalar clamping policy. Historical `74835089`'s 1 MiB native-descriptor length rewrite is excluded; the current image control returns the full 2 MiB requested. Vector limits and backend budgets remain separate. |
| Native I/O and entropy | Preserve current upstream native argument policy: balanced read/pread admit up to 1 MiB and reject oversized requests; systemd native's existing explicit read allowance remains as on main. No new scalar read/write/getrandom clamp or vector aggregate policy is introduced. | #163's check of getrandom flags rather than length remains a known upstream defect. Do **not** promise a 256-byte getrandom limit for this release; kernel/vDSO entropy behavior has no added downstream per-call byte quota. Resource/deadline limits still apply. A future entropy quota is a separate policy change, not a checkbox closed here. |

Supported query buffers and counts remain subject to x86-64 ABI widths, checked
pointer arithmetic and the existing access/strict-copy owners. This contract
does not turn partial implementation labels into general Linux compatibility.
The user's acceptance of these limits does not waive PRD09 security acceptance,
PRD12 resource checks, portability/CI or the final product scenarios.

## Upstream policy and exact delivery impact

Read-only refresh at 2026-11-08 04:08 UTC still finds main `53a54add`.
No maintainer decision was inferred from silence, successful tests or #229.

| Question | Current primary evidence | Release disposition / blocked work |
| --- | --- | --- |
| Clock/process-local/identity expansion | [#179 hold](https://github.com/gmkurtzer/ginux/issues/179#issuecomment-6661708444) and [later no-work request](https://github.com/gmkurtzer/ginux/issues/179#issuecomment-6687844846) | Configured clocks, virtual queries and task-local compatibility are approved downstream by the user. Preserve them in the selected release source. Broader native changes and upstream proposals remain held; upstream-only delivery of those changes waits for a maintainer decision. |
| Bounded console poll / shared array | [#207](https://github.com/gmkurtzer/ginux/pull/207) at `5ae058f2`, [race report](https://github.com/gmkurtzer/ginux/pull/207#issuecomment-6683275754), [current hold](https://github.com/gmkurtzer/ginux/pull/207#issuecomment-6709855418) | Keep published head untouched. Its PEEKDATA validation followed by the kernel's live-array copy is not assumed safe. No race reproduction or hold-racing revision is requested here. The required configured public-descriptor implementation is separate; raw native poll stays denied. A future #207 inclusion needs policy and race adjudication before delivery. |
| Network scope | [Merged #229](https://github.com/gmkurtzer/ginux/pull/229), merge `8a475e8b` | Preserve native shared-network owner and separate configured mediated owner. User approval now supports the latter downstream with numeric endpoint/budget limits. No claim that #229 grants hostname isolation, new listener support or every configured socket syscall. |
| Identity/sysinfo | [#219 hold](https://github.com/gmkurtzer/ginux/issues/219#issuecomment-6672163838) | User chose deterministic configured zero view. Corrected only configured launch at `4aa51c1d`; native remains unchanged. Future upstream #219 work still needs maintainer approval and must not be bundled into uname merely for convenience. |
| Resource enforcement | G26 source and existing config bounds; #179's topology concern | User explicitly accepts per-process AS and real eligible CPU IDs. These are supported downstream behavior, not unapproved claims of aggregate RAM/CPU limits or virtual topology. G26 upstream extraction still needs independent review/consent; stronger guarantees would be new implementation scope. |
| Native length/clamp | [#163](https://github.com/gmkurtzer/ginux/issues/163#issuecomment-6658892629), [#178](https://github.com/gmkurtzer/ginux/issues/178#issuecomment-6661706860), historical `74835089` | No scalar-clamp proposal or fixture is delivered. Keep owned/vector behavior separately reviewable. An accidental shared 64 MiB bound was removed from native policy while preserving configured compatibility. Existing oversized native errors are supported limits; their compatibility improvement waits on the held choice. No extra reproduction is sent to maintainers. |
| Global overflow policy | [#222](https://github.com/gmkurtzer/ginux/issues/222#issuecomment-6677739555), pending #227 | No global release-profile change. Keep checked local resource/index arithmetic; do not claim that every arithmetic path traps overflow. This external choice does not block the accepted bounded configured contract. |
| Other native authority/behavior limits | PRD06 N-01/N-02: ioctl action/audit mismatch, host PID siginfo, bounded cgroup/wait behavior | Still explicit PRD07/09/19 findings. This task does not approve them or promise a general native runtime for Recipe. Any later release claim depending on those native paths requires their own disposition and acceptance. |

There is **no user hold on local configured clocks**. Upstream choices still
block their own publication/merge or broader native claims; they do not remove
required configured behavior from this downstream release. PRD01's source route,
PRD07's contribution waves and explicit publication permission remain separate.
If delivery chooses an upstream-only route, the held required capabilities are
real dependencies and cannot be silently replaced by less functional main.

## Authorized corrections and source ownership

The candidate's complete delta from `5300f5ef` is **4 files, +55/-12**:

- `ginux-core::configured_arguments_allowed` owns configured 64 MiB read/pread
  admission; `balanced_arguments_allowed` again has main's 1 MiB bounds.
  The existing finite-bound unit is retained under its corrected configured name
  and also checks native boundary acceptance/rejection. Native systemd's existing
  explicit read branch is untouched.
- `dispatch::launch_arguments_allowed` is used only by configured `launch_tasks`;
  the public native dispatcher keeps its existing helper and policy.
- `launch_tasks` emulates the configured 112-byte zero sysinfo after the central
  contract gate, using `write_tracee_bytes_strict`; no native query hook changed.
- The image-I/O fixture verifies all 112 bytes, both surrounding guards, explicit
  deny without mutation, and normal large-image/statfs/virtual-credential behavior.

Clock source SHA256 is
`deecd97f873ec7a89220eebe0ba37a44bd04fe5bb78d3830f72756836e7d9c66` both before
and after. No clock implementation change, catalog expansion, hard/public limit
increase or replacement of the tested integration/gitlinks occurred. Carry this
correction into PRD05 along with the separate storage/PRD04 owners. The old PRD06
ledger remains its exact historical snapshot; this successor delta and its
retained test rename are explicitly accounted for here.

## Actual verification

The evidence directory is
`/home/vince/Projects/LevitateOS/.artifacts/ginux-contribution-preparation/prd08/`.
`candidate.json`, `verification.json`, `SHA256SUMS`, full source diff, upstream
comments/decisions, build/unit logs, runtime logs and Recipe receipts pin the exact
sources, binaries, fixtures and commands.

| Check | Exact observed result |
| --- | --- |
| Static/build/units | `make check`, release/affected fixture builds and all 138 existing Ginux units pass. Original test coverage remains; finite configured/native bounds are checked in the retained test. |
| sysinfo failing base / passing fix | Same new valid-buffer oracle exits 51 on `5300f5ef`, 0 on `4aa51c1d`; all 112 bytes zero, guard bytes preserved. Explicit `syscall.sysinfo` denial passes with EPERM, unchanged output and deny audit. |
| Native-bound failing base / passing fix | Source `5300f5ef` violates the intended main-native contract (fixture exit 22: oversized read allowed). Candidate exits 0: valid 1 MiB read/pread succeeds, 2 MiB fails, raw native poll and clock/sleep stay ENOSYS. |
| Configured I/O | Existing 2 MiB read/pread, virtual statfs and child credential/fork/exec control passes; restored native bounds do not break configured reads. |
| Identity/resources | Four synthetic uname identities plus lifecycle controls pass. 32 MiB AS and lower inherited 24 MiB limit pass; host limit stays intact. CPU/fork/exec controls give [0] for configured 1 and [0,1] for configured 3 with operator [0,1]. |
| Configured poll | Five ordinary scenarios pass: ordinary masks, read wake, write wake, hangup and finite timer. These do not claim #207 or all shared-memory interleavings verified. |
| Configured TCP | Eight controlled local scenarios pass: I/O, nonblocking, denied endpoint, quota, timeout, off, shared byte budget and ordinary lifetime. Forbidden listener observes no connection. No external networking or broader security campaign ran. |
| Existing Recipe clock consumers | All six formerly blocked cases pass on Recipe `b4144388` plus this exact candidate: silent work past frame timeout, execution deadline, interrupt/reap/checkpoint, coordinator death, checkpoint recovery, and destination serialization. |

### Isolated clock acceptance

A transient **session-libvirt** domain `acorn-prd08-clocks`, with no network or
disks, booted existing kernel `6.12.0-211.74.1.el10_2.x86_64` and a minimal test
initramfs. This is a backend acceptance guest, not an AcornOS product-boot result.
The PID-1 controller requires the dedicated kernel marker before it can change
time. Only the guest's realtime clock was adjusted; no workstation clock or
system policy was changed. The guest powered off and disappeared from libvirt.

The [clock fixture](prd08-clock/clock-boundary.c),
[PID-1 controller](prd08-clock/vm-init.c) and
[archive/domain builder](prd08-clock/build-clock-vm.py) are retained for review.
`vm-inputs.json` pins the launcher, guest task fixture, controller, libc/loader and
reused kernel. `vm-console.log` is the unedited serial transcript.

- Existing `Z-clocks` passes its query/ID/timespec controls and all three real
  caught-SIGCHLD interruptions: nanosleep, relative clock_nanosleep and absolute
  clock_nanosleep, with SA_RESTART, correct remaining-time behavior and reaping.
- A real **300-second** relative wait returns 0 after **300.001791675** monotonic
  seconds. A 300-second-plus-one-nanosecond relative request is rejected by the
  same worker before each long test. No clock scaling or shortened cap is used.
- During an already-active absolute realtime wait 120 seconds ahead, the guest
  clock advances **240 seconds**. It returns 0 after **0.304855987** monotonic
  seconds, after the realtime deadline has actually passed. The controller waits
  for the runtime's sleep-entry audit before stepping, so this is not merely a
  past-deadline-at-entry test.
- During another already-active 120-second absolute wait, realtime steps back
  **600 seconds**. With its realtime deadline still future, the wait returns
  **ETIMEDOUT (-110)** after **300.066929687** monotonic seconds. Both long tests
  run concurrently, independently asserting their results and timing. Each
  launcher returns `exited/0` with a valid completion record.

The two concurrent ready lines interleave in the raw console; final result lines,
per-child completion records and the final PASS marker are complete. No assertion
or evidence was substituted to account for that output interleaving. Measured
overshoot fits the existing scalar-poll quantum; the supported contract above
states scheduling tolerance rather than promising hard realtime completion.

No synthetic skipped syscall, mock clock, timing acceleration, host clock step,
security runner or direct-QEMU invocation was used for this acceptance. PRD09's
final-source adversarial acceptance and PRD13/19's platform/product requirements
remain open.

## Reproduction and remaining work

From the repository root, inspect `candidate.json` and `SHA256SUMS` first.
The retained artifact helpers are exact copies/commands for this host:

```sh
sha256sum --quiet -c .artifacts/ginux-contribution-preparation/prd08/SHA256SUMS
sudo python3 .artifacts/ginux-contribution-preparation/prd08/verify-contract.py
sudo python3 .artifacts/ginux-contribution-preparation/prd08/run-recipe-clocks.py
python3 .artifacts/ginux-contribution-preparation/prd08/build-clock-vm.py
virsh --connect qemu:///session create .artifacts/ginux-contribution-preparation/prd08/clock-vm.xml
```

The builder consumes the compiled controller/worker/launcher and writes a new
initramfs/domain XML. Recompile retained C with the flags in `candidate.json`
when reproducing from source, then allow the actual five-minute checks to finish.
Do not execute the controller as a host program. Reruns change receipt timings
and serial hashes and therefore require a fresh manifest.

PRD08 has no remaining required user/maintainer decision or clock-boundary test.
The selected contract must survive actual PRD05 composition and PRD18 derivation;
that is final-source work, not an untested native-policy expansion. PRD04's export
publication failure, PRD06's test ports/Recipe joins, native findings, complete
security/consumer/CI/product acceptance and final source packaging remain open
under their existing owners. No release-complete claim is made.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`.
Reload the complete skill, required reference, instructions and current queue
after context shift. PRD08 is complete; carry `4aa51c1d` and this contract forward.
Next independent authorized engineering work remains the PRD04 export correction
and focused Daybreak retest, or PRD07 preparation. No publication is authorized.
