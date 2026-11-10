# Sandbox build gaps: handoff (2026-11-10)

For whoever continues in a new session, Claude or Codex. It assumes nothing from
the session that wrote it.

## The job

Ordinary build tools misbehave inside the sandbox Recipe runs recipes in. Find
the cause of each of the five gaps below in our Ginux fork, fix them, and prove
it by building `htop` there with no workarounds in its recipe.

Nothing has been fixed or traced to a line yet. The gaps are measured; the
causes are leads.

## How this came up

The plan is a graphical front end for Recipe in which an agent (Codex or
Claude Code) writes the recipe for a program a person asks for. To see what
that takes, an agent wrote three real recipes and ran them through Recipe:

- `tree` 2.3.2 (plain `make`, files copied with Recipe's helpers): worked on
  the first try.
- `ncurses` 6.6 (`configure`, `make`): built and published after three fixes
  to the recipe.
- `htop` 3.5.3 (`configure` from autoconf 2.72, depends on that `ncurses`):
  never finished in seven attempts.

Every failure was the sandbox, not the recipe.

## The five gaps

Measured by a probe recipe on 2026-11-10, with the launcher built from
`tools/ginux` at `da00d25e` and the image
`.artifacts/ginux-env/prepared-image-iso-tools`.

| # | Inside the sandbox | Seen | What it breaks |
|---|---|---|---|
| 1 | `echo x > f; stat -c %Y f` | `0`. Appending to `f` leaves it `0`. `touch f` sets the present time and `touch -d` any time, so setting works; stamping on create and write does not | `make` (targets never look newer than their sources), automake's "newly created file is older than distributed files" check |
| 2 | `chmod 755 f` | `chmod: changing permissions of 'f': Function not implemented`; the mode stays `644`. Recipe's own `chmod()` helper on the same file works | scripts a build writes cannot be run (`Permission denied`, exit 126); autoconf 2.71 and later stops at `could not make ./config.status` |
| 3 | `install -m 644 a b`, and plain `install a b` | `cannot change permissions of 'b': Function not implemented`; `b` is left `0600`. In `htop`'s `configure`, `install` into a fresh directory also gave `cannot create regular file ...: Bad file descriptor` | every `make install` |
| 4 | `mkfifo f`; `make -j4` with a sub-make | `Function not implemented`; then `make: *** pselect jobs pipe: Function not implemented` | parallel `make` (GNU make 4.4 wants a fifo, falls back to a pipe and `pselect`) |
| 5 | `echo x \| grep x > /dev/null` | `grep: (standard input): Function not implemented`, exit 2. `grep -q` on the same pipe is fine; so is a file as input, or any other output | the yes-or-no test in every `configure` script answers "no" without an error. `htop`'s decided `uid_t` does not exist and defined it as `int` |

Gap 5 is the dangerous one: nothing fails, the build is configured wrongly.
`ncurses` very likely took wrong answers too and built anyway.

Also seen, not probed:

- `ls -la` of a dependency folder under `/inputs` printed
  `Function not implemented` once and still listed the files (probably an
  extended-attribute call).
- `htop`'s `configure` printed `here-document at line 1 delimited by
  end-of-file (wanted '_ACEOF')` six times from its generated `conf*subs.sh`.
- `nproc` printed nothing.

## Leads, none verified

A missing call can answer "not implemented" from two places, and both need
checking for each gap:

- the storage code has no arm for the call's number, or
- the contract Recipe derives for the run does not grant the operation
  (Recipe's README: "absent semantic grants still return ENOSYS").

Each run keeps its records under the state folder
(`.artifacts/recipe-agent-trial/state/logs/<invocation>/`: `authority.json`,
`supervisor-stderr.log`, `worker.log`, owned by root), and
`recipe explain <that folder> <operation>` reads them.

Per gap:

1. **Times.** `crates/ginux-init/src/launch_storage.rs` has a `modified` field
   (line 40) and `timestamps()` (about line 1705) behind `utimensat` (280).
   Nothing seems to stamp a file on create or write. Check first whether a
   fixed time is deliberate (reproducible archives); if it is, builds still
   need times that move forward, for example a counter.
2. **`chmod`.** The same file has arms for `chmod` (90, line 1249), `fchmod`
   (91, line 1527) and `fchmodat` (268, line 1254), so the command must be
   making another call. `fchmodat2` (452) is only a name in the catalog
   (`crates/ginux-contract/src/lib.rs` line 1572). Run the command under the
   run's audit to see the number.
3. **`install`.** It uses `fchmod` on a descriptor it just created; the arm
   exists, yet it fails. Find which call returns the error, and where
   `Bad file descriptor` comes from.
4. **Fifos and `pselect`.** `mknodat` and `pselect6` appear only as names in
   the catalog (lines 1260 and 1324).
5. **`grep`.** GNU grep, when its output is `/dev/null` and its input is a
   pipe, drains the pipe with `splice`. No file in `crates/` mentions
   `splice`. grep falls back to reading when `splice` answers `EINVAL`, so
   either implement it or answer `EINVAL`. This is from memory of grep's
   source; confirm it.

## Where the code is

- `tools/ginux`: our fork, `LevitateOS/ginux` main at `da00d25e`, 85 commits
  and about 25,700 lines ahead of `gmkurtzer/ginux` main. The storage the
  sandbox builds on is in that part: `launch_storage.rs` does not exist
  upstream, and `crates/ginux-vfs/src/lib.rs` is 4,388 lines against
  upstream's 1,101. These gaps are therefore ours; upstream's tracker has none
  of them. The nearest upstream issue is 179 (missing calls in the balanced
  profile), which the maintainer has put on hold.
- `tools/recipe/src/needs/contract.rs`: the contract Recipe derives for a run.
- `tools/recipe/docs/runtime-limits.md`: what the sandbox promises today.
  Update it when a gap closes.

## Reproducing

The probe needs no network and takes about a minute and a half. It needs root,
because the launcher does.

```sh
cd /home/vince/Projects/LevitateOS/.artifacts/recipe-agent-trial
. ../work/iso-runtime-fixes/environment.sh
sudo env \
  GINUX_BIN=/home/vince/Projects/LevitateOS/target/ginux/x86_64-unknown-linux-gnu/release/ginux \
  GINUX_ROOTFS="$GINUX_ROOTFS" GINUX_ROOT="$GINUX_ROOT" \
  RECIPE_WORKER_BIN=/home/vince/Projects/LevitateOS/target/debug/recipe \
  /home/vince/Projects/LevitateOS/target/debug/recipe install recipes/probe.rhai -r recipes \
  --build-dir "$PWD/work/probe" --state-dir "$PWD/state" \
  --json-output "$PWD/probe-result.json" > probe-run.log 2>&1
```

The report is printed into the log between `## clock` and
`## nproc and tools`. Recipe treats an edited recipe as a new one, so a changed
probe runs again from the start; an unchanged one is skipped as installed
(`--force-install` reruns it).

`.artifacts/` is not in Git. What is there:

| File | What |
|---|---|
| `recipes/probe.rhai` | the probe; add a line per thing to test |
| `recipes/tree.rhai` | works as it is |
| `recipes/ncurses.rhai` | works, with one workaround (below) |
| `recipes/htop.rhai` | fails at the end of `configure`, with three workarounds |
| `htop-run-1.log` to `htop-run-7.log`, `run4.txt` to `run7.txt` | every attempt; the `.txt` files have the colour codes removed |
| `probe-run.log`, `probe-run-2.log`, `probe-run-3.log` | the three probe runs |
| `state/`, `work/`, `.artifacts/ginux-export/` | Recipe's own state and what it published, owned by root |

## Done when

1. The probe shows: a new file dated now and re-dated by a write; `chmod` and
   `install -m` setting modes; `mkfifo` making a fifo; `make -j4` with a
   sub-make running; `echo x | grep x > /dev/null` exiting 0.
2. `htop.rhai` and `ncurses.rhai` build with these removed:
   - the helper `chmod` on generated scripts (`ncurses.rhai`),
   - `touch -d 1960` on the unpacked source (`htop.rhai`),
   - the `grep` wrapper put first in `PATH` (`htop.rhai`),
   - the hand copies in both `install` hooks, replaced by
     `make install DESTDIR=...`.
3. Each fix has a test in the fork, the way that crate tests its other calls.
4. `runtime-limits.md` says what is now supported.

## Rules that apply

- Read `AGENTS.md` first.
- Ownership is in the "Ownership split" section at the end of
  `docs/ginux-contribution-task-list.md`: Claude does the upstream Ginux pull
  requests, Codex the Recipe and Ginux integration. This work is in the fork's
  storage code, which the storage redesign also touches. Check `git status` in
  `tools/ginux` and `tools/recipe` before editing; other sessions work there.
  When this was written `tools/ginux` was clean at `da00d25e`.
- Nothing goes to `gmkurtzer/ginux` without the user's go-ahead. Upstream
  issues 163, 178, 179 and 219 are on hold by the maintainer; do not start on
  them.
- Running a recipe means running Recipe as root. A recipe that downloads code
  needs the user's yes for that source; the probe downloads nothing.
- Kernel, ISO and VM builds only between 10:00 and 01:00. A Ginux build and a
  probe run are small.

## Not part of this job

Found in the same trial, to be decided separately:

- The build image lacks `cmp`, `diff`, `xz`, `bzip2`, `pkg-config`, a C++
  compiler, `perl`, `m4`, `patch`, `file` and `gpg`, and there is one
  Rocky-based image for both distributions. A program built in it is tied to
  that image's C library.
- Recipe restarts a recipe from nothing after any edit, throws the failed
  build folder away, and reports a missing `acquire` or `build` hook only at
  run time. Every run costs about a minute and a half before the recipe
  starts (measured with a debug build of `recipe`).
- Recipe needs root for every run and does not put anything onto the running
  system.
- The design for what recipe files should declare for the front end is
  `docs/recipe-front-end-format.md` (not built, not committed when this was
  written).
- The front-end draft is a design board at
  `https://claude.ai/artifact/8SrGDbmTs4K4uTK43F9PVR`.
- Open with the user: whether desktop programs are mainly built from source
  in the sandbox, or mainly made from the distributions' own packages as the
  OS images are, with from-source builds as the exception. The second needs
  none of the five fixes.
