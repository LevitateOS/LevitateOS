# G06b — configured child and exec ownership

Local candidate `8c783e717b839b0e511820e89d29840cae0f3c0c`, tree
`0c416ecb5220d4cec93900c2834cf35099ad294a`, on prerequisite composition
`5cb0370f8b9d96d95f28498cc2d6a1c3698af72f` (reviewed G05 plus G06a).
[Retained source patch](patches/G06b.patch). 19 files,+2343/-351 lines.
This is **not standalone-ready on upstream main**: G05, G06a and their upstream
prerequisites must merge first. Do not publish the local composition as a stack.

## Behavior and boundary

Own at most 16 process slots; hold newborns until GETEVENTMSG confirms identity
and their descriptor snapshot exists. Fork/vfork and three process-only clone
forms use virtual IDs, parent views, child-TID writes and signal senders. Share
owned-object references and open descriptions across copied descriptor tables.
Wake blocked pipe/wait operations on relevant object/death transitions. Typed
ChildStatus/WaitTarget APIs remain authoritative. Unpublished failed forks roll
back through cancel_fork; peek_wait separates observation from consumption.

Image exec synchronizes native descriptor flags and commits CLOEXEC retirement
only at PTRACE_EVENT_EXEC. Failed exec restores public arguments/table state;
canonical-path scratch remains per-task and restores before descriptor finish.
Image scripts and ELF PT_INTERP names must retain the same permitted image owner
under domain and native resolution, including initial exec and execveat on image
FDs. This is bounded metadata validation, not a second ELF loader. 16 script-chain
inspections and 256-byte interpreter paths bound it. Exec remains x86-64 only.

Init exit or supervision failure kills/drains all descendants and verifies zero
owned handles/tasks. ESRCH at transient register/resume stops retains ownership
until terminal wait4. A matching adopted-zombie second receipt is consumed once;
owned references are released only at the first observed death. No signaling of
an already-reaped initial PID on ECHILD/signaled startup failure.

Threads, owned executables, mappings, host writable descriptors, polling, future
networking, global clock/profile changes and checkpointing are excluded. Existing
upstream native supervisor/core and #221 copyout helpers/tests are byte-for-byte
unchanged. Open #224/#226 therefore retain their separate native-process scope.

## Findings and corrections

The final source/diff and relevant callers/failure paths received a separate
self-review pass; independent reviewer coverage is not claimed.

- The initial configured wait copied status before native reap. The qfault
  regression returned 124 because a second wait still consumed the child after
  EFAULT. Reap first without publishing host status, then consume typed virtual
  outcome and strictly publish the word. Four protected/unmapped/cross-page cases
  now pass, including rollback when only the tail is writable.
- Libc CHILD_SETTID had the same partial-word hazard. Suppress native SETTID at
  clone, publish the virtual ID at the child barrier through the same strict
  store, and retain CLEARTID. Invalid/read-only/cross-page faults leave the word
  unchanged; fatal rollback failure tears down the domain.
- Direct exec checks missed kernel shebang/PT_INTERP resolution. An image script
  reached an ELF hidden below memory:/work and exited 77. The retained probe now
  exits 0 after observing denial. Persistent script/ELF, image-symlink, mixed-parent,
  nofollow, image-FD and initial-exec cases deny hidden targets; permitted image
  script and ELF interpreter controls still execute and return 77.
- The old donor APIs did not match current main. Adapt current typed outcomes
  rather than replacing them. Preserve complete bounded audit records, existing
  path isolation, workdir mkdirat and descriptor copy/alias fixes. Add the existing
  freestanding memmove owner required by the larger task state; the initial
  undefined-symbol link failure and corrected linked build are retained.
- Canonical exec replay must retain scratch across injected fcntl calls and
  restore original public registers after failure. New failed-path/FD-state
  assertions cover this. Directory exec reports EACCES and nofollow image links
  preserve ELOOP. CLOEXEC bootstrap stdin must not invent a public console handle.

## Exact local validation

Both source and a fresh prerequisite export with only G06b applied pass:

- `make check test-unit` (82 total units, 17 process tests, 24 VFS tests).
- `make build build-launch-fixture build-launch-storage-fixture build-launch-device-fixture build-launch-pipe-fixture build-launch-fd-fixture build-launch-task-fixture build-launch-exec-fixture build-process-fixtures build-gate1-fixture`.
- `sudo -n make test-launch test-launch-storage test-launch-devices test-launch-pipes test-launch-fds test-launch-tasks test-launch-exec test-native-supervision test-gate1-setup`.
- `make test-process-native` outside the ptrace-restricted execution sandbox,
  including all existing copyout fallback and repeated native child scenarios.

Twenty additional repetitions of configured task and exec suites pass. They cover
birth/death races, libc fork/TIDs, signal-handler return, inherited aliases, task
exhaustion/reuse, failed-fork rollback, orphan/adopted-zombie receipts, child kill,
root death and supervision-failure descendant teardown. Exec fixtures inspect
actual native FDs before allowing the executed task to continue.

Strict patch application/whitespace checks pass; clean replay matches all 168
tracked paths and source hashes match the retained commit. Full logs, failures,
reproduction scripts, command exit receipts, binary/patch hashes and a concise
draft are in `.artifacts/ginux-contribution-preparation/G06b/`. macOS Actions and
full Rocky VM checks are unrun; no remote CI success is claimed. Local review and
composition validation are complete; final current-main independence remains
pending upstream prerequisites and the later publication refresh.

No real Ginux branch, index, checkout, remote text or PR was changed. Source and
review material remain downstream. User inspection at F04 precedes publication.
