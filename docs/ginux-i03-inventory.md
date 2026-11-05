# I03 — First worker/compiler incompatibility inventory

Date: 2026-11-05. Owner lane: Integration (I03 only).
Scope: this note only (superproject `docs/`). No edits to `tools/recipe`,
`tools/ginux`, `AcornOS`, `tools/recfstab`, any other submodule, worktree,
or plan. All probes are throwaway `/tmp/i03` sources/binaries/logs;
the stage dir is `/tmp/i03-stage`.

Read first: `docs/ginux-recipe-implementation-plan.md` (tasks I02, I03, I04,
R06), `docs/ginux-i02-recfstab-inputs.md` (gaps H1–H6),
`docs/ginux-recipe-interfaces.md` (§§1–3).

Context: R06 adapter exists (`tools/recipe`, branch `ginux-recipe/r06-launch`)
but was stub-verified only. The real supervisor is the integration binary
`/tmp/ginux-merge/target/x86_64-unknown-linux-gnu/release/ginux`
(process supervision G03–G05a, L01 teardown, G09 domain exec;
branch `ginux-recipe/integration`, commit `e7936f57`).
Rebuilt it fresh this session (see §1); no stale binary.

Result: **failed runs with captured diagnostics IS the deliverable.**
Both the real recipe worker and `cargo --version` die in dynamic-loader
startup (exit 127, silent) before any Rhai evaluation. No success is
claimed; nothing was fabricated.

## 1. Commands (exact)

Supervisor rebuild (from `/tmp/ginux-merge`, HEAD `e7936f57`):

```
cd /tmp/ginux-merge && cargo build -p ginux-init \
  --target x86_64-unknown-linux-gnu --release
```

Stage prep (R05 input file, byte-identical to the I02 record):

```
cp /home/vince/Projects/LevitateOS/AcornOS/deps/recfstab.rhai /tmp/i03-stage/main.rhai
```

Probe builds (no static libc on host; raw-syscall sources, same idiom as
`/tmp/g03b-repro/shell.c`; `-fno-builtin` stops gcc turning the length
loop into a `strlen` reference):

```
cc -nostdlib -O0 -fno-builtin -o /tmp/i03/recipe-driver /tmp/i03/recipe-driver.c
cc -nostdlib -O0 -fno-builtin -o /tmp/i03/cargo-driver /tmp/i03/cargo-driver.c
cc -nostdlib -O0 -fno-builtin -o /tmp/i03/fd2probe /tmp/i03/fd2probe.c
cp /tmp/g05a-repro/gate.toml /tmp/i03/gate.toml
```

Sanity baseline (must still PASS — it does):

```
sudo /tmp/ginux-merge/target/x86_64-unknown-linux-gnu/release/ginux \
  process-native /tmp/i03/gate.toml /tmp/g03b-repro/exec-shell
```

Recipe worker probe (real dynamic binary + worker env + staged input):

```
sudo env GINUX_DOMAIN_STAGE=/tmp/i03-stage \
  /tmp/ginux-merge/target/x86_64-unknown-linux-gnu/release/ginux \
  process-native /tmp/i03/gate.toml /tmp/i03/recipe-driver
```

Toolchain probe:

```
sudo env GINUX_DOMAIN_STAGE=/tmp/i03-stage \
  /tmp/ginux-merge/target/x86_64-unknown-linux-gnu/release/ginux \
  process-native /tmp/i03/gate.toml /tmp/i03/cargo-driver
```

Tracee-stderr control (static probe writing to fd 1 and fd 2):

```
sudo /tmp/ginux-merge/target/x86_64-unknown-linux-gnu/release/ginux \
  process-native /tmp/i03/gate.toml /tmp/i03/fd2probe
```

Host ground truth (unsupervised, read-only) for denial correlation:

```
strace -f -o /tmp/i03/recipe-host.strace \
  /home/vince/Projects/LevitateOS/target/debug/recipe --help
strace -f -o /tmp/i03/cargo-host.strace \
  /home/vince/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo --version
```

Survivor check after every supervised run (precise patterns only):

```
ps -ef | grep -E "ginux|recipe-driver|cargo-driver|fd2probe|exec-shell" | grep -v grep
```

## 2. Sanity baseline (harness works)

`SHELL_REPRO: PASS`, supervisor exit 0, full log 75 lines
(`/tmp/i03/baseline.log`). Tail:

```
SHELL_REPRO: PASS
GINUX_NATIVE_PROCESS_AUDIT: teardown reaped=1 tasks=0 live-pipes=0
```

Fork/vfork/clone + execve/execveat mediation, wait translation, and
teardown all behave. The gaps below are workload gaps, not harness
breakage.

## 3. Recipe worker run (full trace)

`/tmp/i03/recipe.log`, supervisor exit 0 (exit code is the *driver's*;
the worker outcome is the `status=` line). Driver
(`/tmp/i03/recipe-driver.c`): static, raw syscalls; `chmod`s the staged
input through the domain path; `fork`+`execve`s the real worker
`/home/vince/Projects/LevitateOS/target/debug/recipe`
(ELF dynamic, `ld-linux-x86-64.so.2`, debug, ~242 MB) with the §5
allowlisted env (`RECIPE_WORKER=1`,
`RECIPE_INVOCATION=i03-recipe-1`, `BUILD_DIR=/work/build`,
`TMPDIR=/work/tmp`, `ARCH=x86_64`, `NPROC=8`,
`PATH=/work/tools:/usr/bin:/bin`); `wait4`s and reports. The tracee
entry execs the driver with an empty environment, so the driver is what
supplies `RECIPE_WORKER=1` — running the worker binary as the direct
fixture would leave it the empty tracee env.

```
GINUX_NATIVE_PROCESS_AUDIT: exec-stage file=/tmp/g09/main.rhai
GINUX_NATIVE_PROCESS_AUDIT: exec-stage files=1
GINUX_NATIVE_PROCESS: virtual-init=1 host-pid=unexposed ptrace=attached
I03-DRIVER self=1
I03 check self-is-virtual-init: ok
I03-DRIVER chmod-staged rc=0
I03 check staged-input-visible: ok
GINUX_NATIVE_PROCESS_AUDIT: fork variant=fork host-result=252482 virtual-result=2
I03 check fork-virtual: ok
GINUX_NATIVE_PROCESS_AUDIT: wait parked caller=1 selector=2
GINUX_NATIVE_PROCESS_AUDIT: exec syscall=59 host-pid=252482 virtual-pid=2 epoch=1 cloexec-dropped=0
GINUX_NATIVE_PROCESS_AUDIT: exec-complete virtual-pid=2
GINUX_NATIVE_PROCESS_AUDIT: sigchld queued parent=1 child=2
GINUX_NATIVE_PROCESS_AUDIT: wait wakeup caller=1 child=2 status=127
GINUX_NATIVE_PROCESS_AUDIT: sigchld delivered parent=1
GINUX_NATIVE_PROCESS_AUDIT: sigchld delivered parent=1
GINUX_NATIVE_PROCESS_AUDIT: sigchld delivered parent=1
I03-DRIVER worker waited=2 status=32512 exited=127 signaled=0
I03-RECIPE-DRIVER: DONE
GINUX_NATIVE_PROCESS: root-exit=0 tracing-remaining=0
GINUX_NATIVE_PROCESS_AUDIT: storage-flush=seam-g12a backends-closed=0
GINUX_NATIVE_PROCESS_AUDIT: teardown reaped=1 tasks=0 live-pipes=0
GINUX_NATIVE_PROCESS_AUDIT: descriptors live-pipes=0
GINUX_NATIVE_PROCESS: outcome=completed code=0
GINUX_NATIVE_PROCESS: fork=mediated child=ptrace-attached pid-translation=PASS
GINUX_NATIVE_PROCESS: wait=translated child-status=real tasks=0 cleanup=PASS
GATE7_NATIVE_PROCESS: PASS
```

What worked: staging (`exec-stage files=1`, sha `91118aaf…0af91`
matches I02 §2), domain-visible input (`chmod-staged rc=0` through the
G08a path, host untouched), fork/exec/wait mediation, SIGCHLD
accounting, full teardown (`reaped=1 tasks=0`, `outcome=completed`).

What failed: the worker child (`virtual-pid=2`) exits **127 with no
output** before any recipe code runs. No `I03-CHILD execve returned=`
line, so `execve` itself succeeded (host-side fall-through for the
unstaged path, `exec-complete` recorded) and the death happens inside
dynamic-loader startup under mediation.

First-denial sequence (reconstructed from host ground truth +
dispatch table; this loop audits mediations but has **no per-syscall
deny line**, so the exact sequence is strace order mapped against the
`_` catch-all at `crates/ginux-init/src/main.rs:2461`, which answers
every unclaimed syscall with `-ENOSYS`). Host order for the worker
(`/tmp/i03/recipe-host.strace`):

```
execve(...) = 0
brk(NULL) = ...                        -> denied -ENOSYS (FIRST denial, syscall 12)
mmap(NULL, 8192, ...)                  -> denied -ENOSYS (syscall 9)
access("/etc/ld.so.preload", R_OK)     -> denied -ENOSYS (syscall 21)
openat(AT_FDCWD, "/etc/ld.so.cache")   -> denied -ENOSYS (syscall 257)
fstat(3, ...)                          -> denied -ENOSYS (syscall 5)
mmap(...)                              -> denied -ENOSYS ...
close(3)                               -> mediated, executes (desc arm)
read(3, ...) on any opened fd          -> -EBADF if ever reached
                                         (SYS_READ arm, main.rs:2404:
                                         untracked non-stdio fd)
openat("/lib64/liblzma.so.5" ...)     -> denied -ENOSYS ...
```

Already-mediated syscalls the loader would need next (pass-through,
not gaps): `arch_prctl`, `set_tid_address`. Everything mapping- or
library-shaped after them is denied, so no `DT_NEEDED` library
(`liblzma`, `libgcc_s`, `libm`, `libc`) can load and the loader exits
127. Its silence is loader behavior, not broken stderr: the
`fd2probe` control prints `FD2-PROBE-VISIBLE` to fd 2 under the same
supervisor (see §5), so a loader error message would have appeared.
On the host the same binary prints its `--help` and exits 0.

## 4. Cargo probe (full trace, toolchain gap)

`/tmp/i03/cargo.log`, supervisor exit 0. Driver
(`/tmp/i03/cargo-driver.c`): same skeleton, `fork`+`execve` of
`/home/vince/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo`
(dynamic, `ld-linux-x86-64.so.2`) with `argv=["cargo","--version"]`.
Host ground truth: `cargo --version` → `cargo 1.99.0 (5f94df478
2026-08-27)`, first syscalls `brk(NULL)`, `mmap(NULL, 8192, ...)`,
`readlinkat(AT_FDCWD, "/proc/self/exe", ...)`,
`access("/etc/ld.so.preload")`, `openat(...hwcaps...)`
(`/tmp/i03/cargo-host.strace`, 201 lines).

```
GINUX_NATIVE_PROCESS_AUDIT: exec-stage file=/tmp/g09/main.rhai
GINUX_NATIVE_PROCESS_AUDIT: exec-stage files=1
GINUX_NATIVE_PROCESS: virtual-init=1 host-pid=unexposed ptrace=attached
I03-CARGO-DRIVER self=1
I03 check self-is-virtual-init: ok
GINUX_NATIVE_PROCESS_AUDIT: fork variant=fork host-result=255537 virtual-result=2
I03 check fork-virtual: ok
GINUX_NATIVE_PROCESS_AUDIT: wait parked caller=1 selector=2
GINUX_NATIVE_PROCESS_AUDIT: exec syscall=59 host-pid=255537 virtual-result=2 ...
```

(identical shape to §3; full 24-line log in `/tmp/i03/cargo.log`)

```
GINUX_NATIVE_PROCESS_AUDIT: exec-complete virtual-pid=2
GINUX_NATIVE_PROCESS_AUDIT: sigchld queued parent=1 child=2
GINUX_NATIVE_PROCESS_AUDIT: wait wakeup caller=1 child=2 status=127
...
I03-CARGO-DRIVER cargo waited=2 status=32512 exited=127 signaled=0
I03-CARGO-DRIVER: DONE
...
GINUX_NATIVE_PROCESS_AUDIT: teardown reaped=1 tasks=0 live-pipes=0
GINUX_NATIVE_PROCESS: outcome=completed code=0
GATE7_NATIVE_PROCESS: PASS
```

Same verdict as the worker: `exec` mediated, child exits 127 silent,
teardown clean. The toolchain gap is concrete and identical in kind:
no dynamic toolchain binary (`cargo`, and therefore no `rustc`
invocation it would spawn) can start under this supervisor today.
`cargo --version` never prints.

## 5. Control: tracee stderr works

`/tmp/i03/fd2.log` (static probe, direct fixture, exit 5 — the
nonzero exit is the probe's own design, reported by the loop as
`outcome=workload-failed code=5`):

```
FD1-PROBE-VISIBLE
FD2-PROBE-VISIBLE
GINUX_NATIVE_PROCESS_AUDIT: teardown reaped=1 tasks=0 live-pipes=0
GINUX_NATIVE_PROCESS: outcome=workload-failed code=5
```

Short writes to fd 1 and fd 2 both pass mediation (write arm,
`rdi` 1|2 and `rdx` ≤ 4096). The §3/§4 silence is therefore the
loader's, not the harness's.

## 6. Per-failure classification by existing owner

| # | Failure | Owner |
| --- | --- | --- |
| F-A | `brk(12)` / `mmap(9)` denied at loader startup; no library can map (worker exit 127, cargo exit 127) | **New memory-mapping follow-up** (I04-M1). Nothing owns anonymous mapping mediation yet. |
| F-B | `openat(257)` / `access(21)` for `ld.so.cache` + every `DT_NEEDED` lib denied; loader `read(3,…)` would answer `-EBADF` | **G-files follow-up** (I04-M2): domain file access for staged libraries over the G07/G10 path. Content staging itself stays R05/G10-owned. |
| F-C | `fstat(5)` / `newfstatat(262)`-family + `faccessat(269)` metadata the loader needs (cache size, lib identity; cargo also `readlinkat(267)`s `/proc/self/exe`) | **G-files follow-up** (I04-M3): read-only metadata over the merged view (G07b pure-core exists, unclaimed in this loop). |
| F-D | H1 layout gap (double-`dirname(BUILD_DIR)` → `/work`+`/`, monorepo `target/` write, `AcornOS/output/staging` write) | **R05 / I03-follow-up (recipe-owned)**, citing I02 H1. **Not dynamically observed**: the worker died in the loader before Rhai eval, so no domain path ever resolved. Recorded here as still-open static gap, not as a runtime trace. H2–H6 likewise unreached (H5 worker-init binary: this probe used the host worker path via fall-through, which is exactly the shape H5 must replace). |
| — | stdio / protocol wiring | **R06/G14. No failure observed.** Driver stdout, fd-2 control, and wait-status reporting are byte-clean. The v1 frame transport over G14 channels was not exercised — this loop has no channel transport — so no protocol verdict is claimed. |
| — | Resource / config declarations | **L02. No failure observed.** `gate.toml` declares `vcpus=1 memory_mib=256`; the process-native loop does not enforce `RLIMIT_AS`/affinity (L02 was launch-loop only, convergence pending), which flatters these runs rather than blocking them. |

## 7. I04 follow-ups (named, not implemented)

Three, one per operation family, each 1–3 syscalls. No gap work was
started; the reproducer for all three is the existing throwaway probe
(`/tmp/i03/recipe-driver.c` + `/tmp/i03/recipe.log`, host ground truth
`/tmp/i03/recipe-host.strace`).

- **I04-M1 — Anonymous memory mapping: `mmap(9)`, `brk(12)`,
  `mprotect(10)`.** Files: `crates/ginux-init/src/main.rs` entry
  dispatch (claim the three from the `_:2461` catch-all) +
  `ginux-runtime` `AddressSpace` (or wherever the lane puts the
  mapping model — no new parallel owner). Dependency: none beyond the
  loop; pure mediation first (anonymous private mappings never touch
  storage). Contract semantics: balanced profile gains a bounded
  anonymous-mapping surface (bounds TBD by the lane, fail closed past
  them); file-backed `mmap` stays denied until M2 content exists.
  Reproducer: `recipe-driver` run — first denial is `brk`, second
  `mmap`, both before any recipe code.
- **I04-M2 — Domain library file access: `openat(257)`,
  `read(0)`-on-domain-fd** (`close(213)` already mediated).
  Files: `main.rs` `SYS_READ` arm (`:2404`, extend tracked-fd notion
  to domain fds) + claim `openat` (+ G06a `resolve_at` /
  G07 `open_at` wiring per the G07 launch-merge plan) with G10 staged
  inputs as backing. Dependency: R05/G10 must stage real library
  content (libc, libm, …) — mediation without content still fails
  `ENOENT`, honestly. Contract semantics: library opens resolve
  against `/inputs`-staged content only, never host `/lib64`;
  mutating opens stay rejected per the balanced profile.
  Reproducer: same loader runs (`openat` of `ld.so.cache` /
  `liblzma.so.5` is the third/fourth denial).
- **I04-M3 — Loader metadata: `faccessat(269)`,
  `newfstatat(262)`.** Files: `main.rs` dispatch + G07b pure-core
  `stat_at`/`fstat_fd`/`statat` (byte-verified layouts already exist,
  supervised write-back pending). Dependency: G07 launch-merge
  (tracee-memory/fd plumbing) per the G07b LIMIT. Contract semantics:
  read-only metadata over the merged view; no host metadata leaks
  (sizes/modes of staged domain objects only). Reproducer: same runs
  (`access(ld.so.preload)`, `fstat` of cache/lib fds in strace order).
  Adjacent (observed only in cargo ground truth, not proposed here):
  `readlinkat(267)` for `/proc/self/exe`.

Explicitly **not** follow-ups: H1–H6 (recipe/R05-owned, I02 handoff
stands), R06/G14 transport (nothing failed), L02 enforcement
(nothing failed).

## 8. LIMITs

- Process-native loop only: loop-global view, shared cwd, at most 8
  staged files × 64 KiB under the `/tmp/g09/` prefix. The 242 MB
  worker cannot be staged, so it ran via the unstaged host-exec
  fall-through (`DomainExec::Host`) — loader mediation is what this
  inventories. A staged 242 MB worker is neither possible nor
  proposed in this seam.
- No `/inputs` in this loop: staged content is visible only at
  `/tmp/g09/main.rhai`, not at the §3 stable path `/inputs/main.rhai`.
  R05/G10-owned wiring decides how real inputs arrive.
- Denials are silent in the audit: the loop overrides with `-ENOSYS`
  without a per-syscall deny line. The §3 sequence is host-strace
  order mapped against the dispatch table, not supervisor-emitted
  deny lines — recorded as reconstruction, not as quoted audit.
- H1 (and H2–H6) not dynamically observed: worker death precedes
  Rhai eval. Their I02 text stands unchanged.
- R06 adapter and G14 channels unexercised: no worker-protocol bytes
  were exchanged in these runs; the drivers report raw wait status
  only.
- Survivor checks after all three supervised runs: `ps` with the
  precise patterns above returned nothing (exit 1, no matches).
  Teardown lines in every log read `tasks=0 live-pipes=0`.
