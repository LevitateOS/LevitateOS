# recab-gui

The graphical front end for recab on the desktop images: one window, titled
"System", for changing and updating a locked system. It follows the lifecycle
in `tools/recab/REQUIREMENTS.md` 0.3.0 and the premise that an agent does the
work: the person says what they want, or what went wrong is put together for
them, and an agent is handed the issue, the facts and the expected result.

It holds no lifecycle rules. It runs `recab`, reads its status, and offers
what recab says may be done.

Qt 6 Widgets, C++20, CMake. Widgets shared with `gui/recipe` and `gui/disk` are in `gui/kit`.

The window is the shared frame from `gui/kit/README.md`: a rail (the three apps;
"Overview", "The next system", "History" and "Getting back"; "Activity", "Settings" and "Help" below), the pages, a strip, and a status line.
The Ask, clash and work pages are shown in the same place and the rail keeps
marked the page the person came from. "The next system" has a badge: the number of
changes it holds, when it exists. The strip ("The next system holds N changes." and
"Finish and restart…", which goes to "The next system") shows on every page but
Overview and "The next system", which say it themselves, and on Activity, while the
next system holds changes. Activity lists the agent's
runs from all three apps. The status line has what was read and when, with "Read
again", and the agent (`Agent: Codex, signed in` when `RECAB_GUI_AGENT` is
`levitate-agent`; its file name for a stand-in). Window size 1100 by 700 the first time, then the size it had when it closed (`rememberWindowSize` in `gui/kit`; the picture modes always use 1100 by 700).

## State (2026-11-11)

Since 2026-11-11 the window also has a Release page (a newer release: look,
fetch, add, settle, carry on, take out), a History page (from the new
`recab history --json`), the first view after a restart (on trial, kept, came
back) and a Busy view as designed, all described under "The newer pages" below.
`fixtures/recab-real` speaks schema 10 with `operations`, `updates` and
`history`, and its operations answer as `operations::views`
(`tools/recab/src/operations.rs`) does, with switches for the states a story needs
(see "The stand-in that answers as recab does" below). None of it has run on a real
A/B computer; `recab history` was run on sample files only.

**The real interface is recab status schema 10.** The pages read its lifecycle and
offered operations, with concrete argument vectors and declared inputs. Every
request retains the preparation id and revision the person viewed (the operation's
`argv` is sent as recab gave it); conflict choices refresh between submissions.
Earlier schemas 7 to 9 keep their limited boot and preparation mapping (no
`operations`, no `updates`), and schema 3 is the older stand-in's, a drawing
fixture. What the window reads of each is in `src/model.*`.

`fixtures/lifecycle` supplies schema-10 UI scenarios in either direction, or
passes through a real status snapshot from `RECAB_STATUS_FILE`. Existing older
fixtures remain available. Focused command integration uses real GPT/ext4/vfat
and signed release inputs. Changed Qt objects compile independently; a full Qt
application build and booted-system GUI behavior are not established by those
checks. The default integrations use installed recab, pkexec and reboot;
inspection substitutes fixtures.

## What a person sees

Three pages in the rail, and the pages a flow passes through. Overview says in one
heading and sentence what is true now, shows the two systems side by side, and
offers the one thing to do next with a way to the pages that hold the rest; what
lives on the other two pages is not repeated there as rows.

| State | Overview | The next system | Getting back |
|---|---|---|---|
| Stable, no next system | "Your system is locked"; Make a change | "There is no next system yet": Make a change, Install software | the way back (when there is a system from before), nothing to cancel |
| A change is asked for | Say it in words; the page shows what the agent is given. The Before page first when the place of the system from before will be reused (see "Before it happens") | | |
| Preparing | The steps, how far, the output. Stop keeps what is done | | |
| Open, holding changes | Finish; "A newer release is available" with "See it…" (the Release page) | Every change with who and when; what will differ; the three steps, step 1 current: Finish | the system from before is gone: its place holds the next system |
| A clash | Settle the clash | Settle the clash (the Release page lists it too) | |
| Ready | Restart into it | Step 2 current: Restart now, Start it at the next restart | |
| Restart set | Restart now | Restart now, Cancel the set restart | Cancel it |
| On trial | "You are trying the new system": what is judged, Go back now…, Keep it now | "You are on the next system, on trial": Keep it, Go back | Keep it; Go back |
| Being worked on elsewhere | "not an interruption", a "Change, finish, reset" row that is off with why | the list, and one sentence: nothing can be started | |
| Interrupted | Continue where it stopped | Continue where it stopped, Discard | |
| Tried, and the computer came back | the prompt for the agent, try again | step 3 marked failed: Try the same again, Hand it to the agent (leads to Overview) | "Kept already: the computer came back to it on …" |
| After a kept restart | "You are on the new system" | "There is no next system yet" | "Kept already: it started properly on …"; the way back |

Getting back always has the same three rows (go back, keep the running system,
cancel a set restart), "If the desktop does not start" (steps that can be saved as a
text file), "What going back does not turn back" and "How the running system
started" (from `last_trial`).

There are no slot letters on Overview. "The next system" says which slot starts
(and its device), and "Getting back" names the two systems as the start-up list
does, because that is the one place where a person has to tell them apart.

### The newer pages

**Release.** Reached from Overview and "The next system" when recab knows of a
newer release ("A newer release is available: ..."), and, when none is known,
from the link "Check for a new release" on Overview (recab learns of a release
only when asked, so without that link nothing could ask). The page follows where
`updates.phase` and `updates.merge` stand: none known, nothing newer, the check
failed (recab's error), available, being downloaded, downloaded, download
stopped, adding it stopped on a clash or needs carrying on, and "in the next
system". Its one main button is the next step recab offers (`update_check`,
`update_download`, `update_apply`, `update_recover`, "Settle…", then "Finish and
restart…"); "Take it out" is `update_cancel`; "Choose a file…" is `update_media`
with `--manifest PATH`. It says what recab says about the release (version,
channel, the size of all its files, how much is verified) and that recab says no
more before it is added. In the mock-up and not here, because recab gives none
of it: a release medium that is noticed, kernel and package lists, how much
room it needs, "what happens to your changes" per package, when it was last
checked, and "Skip this release".

**When a release merge stops.** recab's merge (`tools/recab/src/merge.rs`) lists
each thing it cannot decide as a `Conflict`: an `id`, a `path` (for
`package_rebuild` the package's name), a `kind` and a `resolution`. It says nothing
of what either side holds, so neither does the window; the Release page, the clash
page, Overview, "The next system" and the facts handed to the agent say, per kind,
only what the source says: `content_or_metadata` (the next system and the release
both changed it, differently), `type` (a different kind of thing on each side),
`deletion` (removed on one side, changed on the other; recab does not say which),
`directory_tree` (settled as a whole; `merge.rs` 520-540), `package_ownership` (a
file of software added through Recipe that the release changes; only "local" is
accepted, `merge.rs` 1533-1536), `package_rebuild` (the release changes system
libraries or the kernel and the package has compiled files or kernel modules; it
must be reproduced, `merge.rs` 1537-1540) and `missing_or_unsafe_baseline` (recab has
no record of the original release; `merge.rs` 1133). The release named is
`prepared_release` (the version of the selected release while one is selected,
`status.rs`), the other is the running release. What recab offers for each is offered
where the person is stopped, each through the Before page, with recab's own label and
verdict: "Settle…"/"Decide by hand…" (`resolve:<id>`, choices worded; asks first for a
file when "edited"), "Adopt it…" (`adopt_baseline`: asks for the original release's
`release.json` with a file dialog, then the Before page), "Reproduce…"
(`update_reproduce`: the package is the conflict's own, so no question) on the
Release page and the clash page; and "Replace it…" (`replace_failed`, only when
recab keeps no preparation for a failed system; so never on the clash page) on the
Release page and on the Overview of a failed start. The agent is given the open
conflicts that recab lets be settled, each with its sentence and recab's command
(`recab update resolve ID --preparation P --revision N --choice ...`, `recab update
reproduce ... PACKAGE`); a baseline needs a file only the person has, so it is
not handed over. The clash page scrolls under its heading (at 700 pixels high the
list of what recab stopped at, what it offers and the agent's panel do not fit), which the
story checks by bringing "Let the agent fix it" into view.

The commands this window sends were checked against `tools/recab/src/main.rs`
(2026-11-11, read only). With the real recab every operation's arguments are recab's own
`argv` (`operations.rs`), then the inputs recab names after it, so `update reproduce
--preparation P --revision N PACKAGE`, `update resolve ID --preparation P --revision N
--choice C [--file F]`, `update adopt-baseline --preparation P --revision N --manifest
FILE`, `update apply --preparation P --revision N --manifest FILE` (the medium: there is
no `update media`), `update recover`, `update cancel`, `update check` and `update download
[RELEASE]` match what clap reads; flags and the one positional word may come in either
order. What differed was the stand-in `recab-real`, which now refuses a missing
`--preparation` or `--revision` for `reproduce`, `resolve` and `cancel`, one of the pair
alone for `apply`, `recover` and `adopt-baseline`, a missing package or conflict, and an
edited resolution without `--file` (or a file with another choice), as clap and
`merge.rs` do.

**When recab says the system needs recovery.** `lifecycle.state` `unusable` is
not called an interruption: Overview and "The next system" say "recab says this
system needs recovery" with `recovery_reason`, show no step of their own, and list
every operation with its verdict (recab refuses all but `trial_recover` then,
`operations.rs` 450-465). An interrupted preparation lists the same, and its
`change_recover` is recab's own way on (`operations.rs` 420-429).

**First view after a restart.** Overview, by state. *On trial* is the mock-up's
page: what is judged, "Go back now…" and "Keep it now", the two systems, and why
nothing can be changed meanwhile in recab's own words (the refusal of
`prepare`). *Kept* is "You are on the new system" with the time recab recorded.
*Came back* is the failed-start page with the prompt for the agent. Each says its
one thing to do once; the catalogue of recab's operations is shown only when the
system needs recovery. Not shown, because recab reports neither: the start-up
progress ("so far the login screen and the network started"), and "4 minutes
ago". recab does say that `recab trial` waits up to 15 minutes for start-up
(`TRIAL_TIMEOUT_SECS`), and the page says that. On trial recab still calls the
other system "next" (the preparation that made the running system is kept until
it is kept); the window shows it as the system from before, which is what
`rollback` leads to. A system kept by hand with "Keep it now" leaves recab's last judged trial as it was (recab's `commit` records none), so Getting back then says it holds no record of one.

**Busy.** A command running elsewhere (`lifecycle.working`): a heading that says
it is not an interruption, the cards, a "Change, finish, reset" row that is off
with why, and a status line that says it reads again every two seconds. There is
no "Stop" and no "if it stands still" detection: recab reports no process and no
progress for work started elsewhere.

**Single system.** Not built. A computer installed with one system has no recab
state; `recab status` then fails while reading the boot selection and says
nothing that tells it from another failure, so the window shows its ordinary
"The system could not be read". Telling them apart needs a new field in recab's
status, which is not a small addition.

**History.** A rail entry, from `recab history --json` (`tools/recab/README.md`).
A table of when, what, who and how it ended, "All / Restarts / By the agent", the
entry's detail under it ("recab recorded: ..."), and "Save as a file…". Only a
change has a "made by" (recab keeps `--actor` only with a change). How a change
ended is worked out from the other entries of its preparation (given up,
finished, the restarts after it, the last counting). With a recab that has no
such command (the older stand-in, or a recab from before it) the page says "This
recab keeps no history it can show."

## Build and run from a checkout

```sh
cmake -S gui/recab -B .artifacts/recab-gui/build -G Ninja
cmake --build .artifacts/recab-gui/build

F=$PWD/gui/recab/fixtures
RECAB_GUI_RECAB=$F/recab RECAB_GUI_AGENT=$F/agent RECAB_GUI_REBOOT=$F/reboot \
  RECAB_GUI_ROOT_HELPER= .artifacts/recab-gui/build/recab-gui --look levitate
```

- `RECAB_GUI_RECAB`: the `recab` to run (default: `recab` on `PATH`).
- `RECAB_GUI_AGENT`: the agent; it reads the prompt on its input (default:
  `levitate-agent`, which starts Codex or Claude Code; see `gui/agent/README.md`).
- `RECAB_GUI_REBOOT`: the restart (default: `reboot`).
- `RECAB_GUI_OS_RELEASE`: where the running system's name is read for the real
  recab (default: `/etc/os-release`).
- `RECAB_GUI_ROOT_HELPER`: what changes run through (default: `pkexec`; empty
  runs them directly, for the stand-in).
- `--look levitate|acorn`: a stand-in look for each desktop.

The stand-in keeps its state in `RECAB_STANDIN_STATE` (see the top of
`fixtures/recab`); delete the folder to start over. `RECAB_STANDIN_DISTRO`
picks LevitateOS or AcornOS, `RECAB_STANDIN_FAIL_TRIALS=1` makes the first
restart into the next system fail.

`--shots FOLDER` walks the whole story and saves a picture at each point. It is
also the check that every flow works: it exits non-zero when a step does not
lead where it should. It needs no display. **It refuses to start (exit code 2, and
a sentence naming what is missing) unless every program it could run and every
folder it could write is named in the environment**: the stand-ins below, the
helper (`LEVITATE_ACTIVITY_AGENT`) with its Codex and Claude Code
(`LEVITATE_AGENT_CODEX`, `LEVITATE_AGENT_CLAUDE`), the root helpers (empty is a name),
and `HOME` and the XDG folders in scratch space. It also turns the desktop notifier off
unless `LEVITATE_AGENT_NOTIFIER` names one. The story runs itself bare to check that.
Two variables below (`RECAB_GUI_RECORDS`, and the helper) are for Activity:

```sh
F=$PWD/gui/recab/fixtures; A=$PWD/gui/agent/fixtures; S=/tmp/recab-shots   # all state of the check lives here
rm -rf $S; mkdir -p $S/home $S/run; chmod 700 $S/run
QT_QPA_PLATFORM=offscreen HOME=$S/home XDG_CONFIG_HOME=$S/home/config XDG_STATE_HOME=$S/home/state \
  XDG_DATA_HOME=$S/home/data XDG_RUNTIME_DIR=$S/run LEVITATE_AGENT_NOTIFIER= \
  LEVITATE_AGENT_CODEX=$A/codex LEVITATE_AGENT_CLAUDE=$A/claude LEVITATE_AGENT_ROOT_HELPER= \
  LEVITATE_ACTIVITY_AGENT=$PWD/.artifacts/agent/build/levitate-agent RECAB_GUI_RECORDS=$A/records \
  RECAB_GUI_RECAB=$F/recab RECAB_GUI_AGENT=$F/agent RECAB_GUI_REBOOT=$F/reboot RECAB_GUI_ROOT_HELPER= \
  RECAB_STANDIN_STATE=$S/standin RECAB_STANDIN_FAIL_TRIALS=1 RECAB_STANDIN_SPEED=0 \
  .artifacts/recab-gui/build/recab-gui --look levitate --shots $S/shots
```

Both stories also check the frame, Activity and the strip. For that they make
runs with `gui/agent/fixtures/records` (stand-in Codex, never a real agent) and
read them through the built `levitate-agent` (build the agent first:
`cmake --build .artifacts/agent/build`). The runs the fixture leaves going are
stopped when the story ends, also when it fails. Then each story starts runs of its
own through a second System window with the helper as its agent, and checks that they
outlive the window and can be found, followed, answered, approved and stopped from
Activity (`../kit/livestory.*`; pictures `a-live-*` and `r2c-live-*`: `-live-question`,
`-live-approval`, `-live-follow`). The pictures `s0-*` to `s14-*` are Settings: as first read, "Allowed", signing out (and the question first), a sign in that fails, signing in (the address Codex printed), another program, back to Codex, a refused value, "Try it" and the run in Activity, a helper that cannot be asked. The pictures `s9b-claude-listed` and `s9c-claude-and-notices-off` show Claude Code as a row and the notices row. The story sets `HOME`, `XDG_CONFIG_HOME`, `XDG_STATE_HOME`, `LEVITATE_AGENT_CODEX` and `LEVITATE_AGENT_CLAUDE` itself for that part and puts them back. The pictures `a-*`
(`r2c-*` in the real story) are Activity: the list, a done run, a run that
waits, this window's own run, the strip on Activity. The pictures `r8a` (no update source), `r16` (a command failed), `r17*` (the restart failed after `set-next`), `r18*` (a change cut off, the recovery list, recovered through `change_recover`), `r19*` (recovery needed), `r20*` (merge blocked on the baseline, adopted), `r21*` (blocked on a package, reproduced), `r22*` (six kinds, one decided by hand), `r23*` (a failed system with no preparation, replaced) are the paths added for the dead ends; `07b`, `07c`, `07d` (older stand-in) are a clash decided by hand, and `08` is the agent's result on the Proposed page. The pictures `r8` to `r12` (`--real`) are the release page from "none known" to "in the next system" (a failed check, nothing newer, available, downloaded, a clash, settled, added; `r10b` is Overview with the clash), `r4a` is the first view on trial, `r13` and `r14` the first views after a restart that came back and one that was kept, `r13b` and `r15*` are History (a restart that came back, all, restarts only); `p12` and `p13` (older stand-in) are the release page with a one-step update and History without a command. The pictures `p*` (older stand-in) and `rp*` (`--real`) are "The next system" and "Getting back" in the states each stand-in reaches; `p-steps-old.txt` and `rp-steps.txt` are the steps saved as a file. The other pictures show
Overview and the pages inside the frame.

The story the real recab offers (two copies, a change by the agent, a command
running elsewhere, finishing, setting the restart and cancelling it, the
restart, the trial kept by hand, going back, the reset, discarding) runs against `fixtures/recab-real` with
`--real`. It stops at once when the system cannot be read, before anything is
asked of the agent. The same variables as above, with the real-shaped stand-ins:

```sh
rm -rf $S; mkdir -p $S/home $S/run; chmod 700 $S/run     # the rest of the environment as above
RECAB_GUI_RECAB=$F/recab-real RECAB_GUI_AGENT=$F/agent-real RECAB_GUI_REBOOT=$F/reboot-real \
  RECAB_GUI_ROOT_HELPER= RECAB_GUI_OS_RELEASE=$F/os-release RECAB_STANDIN_STATE=$S/standin RECAB_STANDIN_SPEED=0 \
  .artifacts/recab-gui/build/recab-gui --look levitate --shots $S/shots --real
```

## What the front end asks of recab

Read, as the desktop user:

`recab status --json` with `"schema_version": 3`:

| Field | Meaning |
|---|---|
| `distro` | the name shown before every release |
| `state` | `stable`, `preparing`, `ready`, `trial_pending`, `failed`, `interrupted` |
| `running` | `release`, and `holds`: the changes in effect in it |
| `other` | the place beside it: `role` (`previous`, `next`, `none`), `release`, `state` (`kept`, `open`, `ready`, `failed`), and `changes` (`id`, `kind`, `title`) |
| `update` | the newer release not yet added: `release`, `size_bytes`; or null |
| `conflicts` | each `path` with what is `local` and what the `release` did |
| `proposal` | what an agent did about a clash: `path`, `summary`, `scope`, and `checks` (`expected`, `verdict`, `note`) |
| `last_trial` | `outcome` (`kept`, `failed`), `when`, `detail`, `log_lines` |
| `interrupted` | `during`: what was being done |
| `actions` | for `change`, `update`, `finalize`, `restart`, `go_back`, `discard`, `recover`: `verdict` (`ready`, `refused`) and recab's `reason` |

Changes, through the root helper, each with `--machine-events`:

| Command | For |
|---|---|
| `recab update apply` | add the newer release to the next system, creating it when needed (older stand-in; the real recab's is below) |
| `recab resolve PATH --local\|--release\|--proposal` | settle a clash |
| `recab finalize` | check and close the next system |
| `recab set-next`, then the restart | start it once |
| `recab rollback`, then the restart | go back |
| `recab recover` | continue an interrupted preparation |
| `recab discard` | give the next system up |

The names follow the "Proposed command surface" in
`tools/recab/IMPLEMENTATION.md`. `--proposal` and the `proposal` field are
this front end's additions: an agent's result has to wait somewhere until the
person accepts it.

Events, one JSON object per line on stderr:

| Event | Fields |
|---|---|
| `plan` | `steps`: pairs of an id and a title, in order |
| `step` | `step`, `status` (`running`, `done`, `skipped`, `failed`) |
| `progress` | `step`, `done`, `total`, `unit` (`bytes` or a count) |
| `say` | `text` |
| `output` | `line` |
| `agent` | `summary` of what the agent did |
| `failed` | `reason` |
| `done` | |

## The real recab

| This window | The real recab |
|---|---|
| the running system's name | `NAME` and `VERSION_ID` of `/etc/os-release` (recab names no release) |
| where things stand | `lifecycle.state`, which recab works out: `stable`, `open`, `working`, `interrupted`, `ready`, `restart_set`, `on_trial`, `trial_failed`. The window works nothing out from slots and phases. `unusable`, or a state it does not know, is shown as a problem with recab's `recovery_reason` |
| "The second copy", "The system from before", "The next system" | `lifecycle.other`: `copy`, `previous`, `next` (`unknown`: no other system is drawn) |
| "is being worked on" | `working`: a recab command holds the run lock. Nothing can be started; the page reads again every two seconds |
| "This system is on trial" | `on_trial`: the running system is not kept yet |
| the next system's changes | `preparation.changes`: every finished change, oldest first. Overview and the strip title them with `action` and `package` ("htop, installed"), and mark one whose `actor` starts with `agent` "by the agent". "The next system" has a row for each: its title, who (`actor`: `person` is "You", `agent…` is "The agent", another label as given, nothing when recab names none) and when (`unix_time`) |
| the badge on "The next system" | the number of `preparation.changes`, when the other system is `next` |
| "Starts" (The next system) | `inactive_slot` and that slot's `device` from `slots[]` |
| "Software", "Files" (The next system) | the `action` and `package` of the changes: install, update, remove (software), edit (a path) |
| "Your files: the same: home is shared by both systems" | `tools/recab/README.md` ("Shared homes/state ... are excluded" from the copy, "User data and caches may be shared on /var") |
| the three steps of the next system | where the lifecycle stands (`lifecycle.state`) and the verdicts below; the sentence about start-up is `recab trial` in `tools/recab/README.md` (a healthy boot is committed, a failed one reboots into the known-good slot) |
| "Go back to the system from before" | `actions[rollback]`, offered only when the other system is `previous` (or the running one is on trial): with a restart set recab's rollback would only cancel it |
| "Keep the running system" | `commit` verdict on trial; otherwise "Kept already", with `last_trial` (time, committed or not, `detail`) when it is about the running system |
| names in the start-up list | `"<NAME of os-release> (Slot A)"`, as `publish_installed` and the candidate publishing write them (`tools/recab/src/systemd_boot.rs`, "(Slot {slot})"); `status` does not carry the title, so the window puts it together from `distro` and the slot. The entry files come from `slots[].entry` (`loader/entries/<entry_prefix>-a.conf`) |
| "make that choice stay" | `recab commit`: "makes the running slot known-good and the default" (README, `commit`), and it needs root (README, "Every operation that writes … requires root"). Not `recab rollback`: from an uncommitted slot that selects the known-good slot, the opposite |
| "the list stays 5 seconds" | `loader_with_default` writes `timeout 5` in a loader.conf it creates (`tools/recab/src/entry.rs`) |
| what going back does not turn back | REQ-PERSIST-006 (`REQUIREMENTS.md`): `/var` is shared and a rollback does not reverse it, including application data migrations |
| Make a change | allowed when a preparation is open, else by the verdict on `prepare` |
| Finish | `recab finalize --preparation ID --revision N` |
| Reset to the running system | `recab discard --preparation ID --revision N`, then `recab prepare`: discarding only forgets the next system and leaves its files; preparing again copies the running system over them. recab has no single command for it |
| Discard (only after an interruption) | `recab discard --preparation ID --revision N` |
| Continue (after an interruption) | with the real recab, the first of recab's own `trial_recover`, `change_recover`, `update_recover`, `finalize_recover`, `sync_recover` operations that it reports ready; with the old stand-in, `recab prepare --preparation ID --revision N` |
| Restart into it | `recab set-next SLOT`, then the restart; when the restart is set already (`restart_set`), the restart alone |
| Start it at the next restart | `recab set-next SLOT` |
| Cancel the set restart | `recab commit`: committing the running system takes the set restart back and leaves the next system ready |
| Keep this system (on trial) | `recab commit` |
| Go back | `recab rollback`, then the restart |

Each button follows recab's verdict in `actions` (`ready`, `no_change`,
`refused` with recab's reason). Commands are sent with `--json`, so a refusal
comes back as one object with recab's message, which the window shows. The
real recab sends no progress events: the work page shows a moving bar and what
recab prints. When `status` fails for lack of rights (recab reads the boot
entries), the page offers "Read it as administrator".

The agent is told what this recab offers (`recab prepare`, `recab change
install|update|remove RECIPE`, `recab edit PATH`, `recab finalize`), the open
preparation's id and revision, and the two slots.

### Releases and history, from the real recab

Each is read from `status --json` (schema 10) and driven through the operation
of that id, whose `argv` the window sends as it is (with `--json --actor person
--machine-events`). Source: `tools/recab/src`.

| This window | The real recab |
|---|---|
| what is known of a release | `updates` (`UpdateStatus`, `update.rs` 43): `phase` (`idle`, `checking`, `up_to_date`, `available`, `check_failed`, `downloading`, `downloaded`, `interrupted`, `awaiting_merge`, or the merge's phase), `available` (`ReleaseSummary`, `update.rs` 19: `id`, `sequence`, `version`, `channel`, `total_bytes`), `download` (`DownloadProgress`, `update.rs` 28: `verified_bytes`, `total_bytes`, `downloaded`), `pending_release`, `last_error`, `merge_complete`, `merge` |
| the merge and its clashes | `updates.merge` (`MergeStatus`, `merge.rs` 37: `phase`, `unresolved`, `failure`, `conflicts`) with each `Conflict` (`merge.rs` 28: `id`, `path`, `kind`, `resolution`) |
| the running and the prepared release | `running_release` and `prepared_release` (`status.rs` 271) |
| "Check now", "Download it", "Add it to the next system…", "Continue adding it", "Take it out", "Choose a file…", "Settle…" | operations `update_check`, `update_download`, `update_apply`, `update_recover`, `update_cancel`, `update_media` (+ `--manifest PATH`), `resolve:<id>` (`operations.rs` 242, 255, 274, 300, 419, 284, 324), each with its `verdict` and `reason` |
| the operations' shape | `Operation` and `Input` (`operations.rs` 9, 18): `id`, `label`, `argv`, `inputs`, `verdict`, `reason`, `destructive` |
| "the restart is set" and "restart needed" | `lifecycle.reboot_needed`, `lifecycle.trial_current` (`status.rs` 110) |
| "up to 15 minutes" | `TRIAL_TIMEOUT_SECS` = 900 (`init_system.rs` 35), the default of `recab trial --timeout` |
| "No change can be made while on trial" | the refusal of the operation `install`, as recab words it: while the trial's ticket exists (it is written by `set-next` and cleared by `commit`, `rollback` or when `recab trial` judges the start: `system.rs` 141-172, `trial.rs` 124) `operations.rs` 454-455 refuses everything but `trial_recover`, `commit` and `rollback` with "Reconcile the pending trial before changing either system", which replaces `ensure_preparation_allowed`'s own reason (`state.rs` 748) |
| the History page | `recab history --json` (`history.rs`): `History` (76: `entries`, `records`, `unreadable`, `truncated`, `log_found`) and `HistoryEntry` (39: `time`, `kind`, `slot`, `preparation`, `change`, `action`, `package`, `actor`, `outcome`, `detail`), read from the audit log (`AuditEvent`, `store.rs` 32) and the state file's changes (`ChangeRecord`, `state.rs` 127) |

## The stand-in that answers as recab does

`fixtures/recab-real` builds its operations like `operations::views` does
(`operations.rs`): the six base operations from the actions (finalize and discard are
listed only while a preparation exists, else "No preparation is open", 126-136);
prepare and finalize refused while a merge is pending (141-151); `replace_failed` and
the refusal of prepare when no preparation is kept for a failed system (152-164);
install, update, remove, edit and `update_media` by `writable` (165-239);
`update_check` refused with "No usable update source is configured" (240-248);
`update_download` ready when a release is known (249-261); `update_apply` (262-280);
`update_recover` (297-308); `resolve:<id>` for conflicts that are neither
`package_rebuild` nor `missing_or_unsafe_baseline`, with the choices of their kind
(309-352); `update_reproduce` (353-374); `adopt_baseline` (392-413); `update_cancel`
(414-419); `change_recover` ready when a change was cut off (420-429);
`sync_recover` (430-441); `trial_recover` ready while the trial's ticket exists
(442-449); and the last loop (450-465): another command running, a pending trial
(refuses all but `trial_recover`, `commit`, `rollback`), recovery needed. The ticket
exists from `set-next` until `commit`, `rollback` or the judged start. Switches, read
when a command runs:

| Variable | Makes |
|---|---|
| `RECAB_STANDIN_UPDATE_SOURCE=none` | no update source: `update_check` refused, `update check` fails as `update.rs` 158 |
| `RECAB_STANDIN_MERGE=baseline\|rebuild\|ownership\|kinds` | what `update apply` stops on (unset: a `content_or_metadata` conflict on an edited file) |
| `RECAB_STANDIN_TRIAL=fail-no-record` | the restart fails and recab keeps no preparation for it (`replace_failed`) |
| `RECAB_STANDIN_STOP_AT=change` | the agent's change is cut off as `change.rs` 500-510 leaves it |
| `RECAB_STANDIN_UNUSABLE=TEXT` | recab needs recovery, for this reason |
| `RECAB_STANDIN_FAIL=COMMAND` | that command is refused with a message recab words (`finalize.rs` 112) |
| `RECAB_STANDIN_REBOOT_FAILS=1` | `reboot-real` fails |

## Before it happens

Every action that changes something shows the shared "Before it happens" page
(`../kit/before.*`) first, in the place of the dialogs there were: a title as a
question, "Nothing has started. This is what will happen if you go on.", "What
happens" in plain sentences for the state the system is in, "What runs" (the
recab command, then the password), "Cancel" and one named button. Cancel, and
Escape, return to the page the person came from and change nothing. The words
are in `src/beforewords.*`; they are taken from the pages, from recab's own
state and `destructive` mark and from the commands the window runs, and say
nothing recab does not report (how long a command takes, how much it changes).

| Action | Title | Button | Runs |
|---|---|---|---|
| Finish the next system | Finish the next system? | Finish | `recab finalize` |
| Restart into it | Restart into the next system? (Restart now? when the restart is set) | Restart into it / Restart | `recab set-next`, then `reboot` (`reboot` only when set) |
| Set it for the next restart | Start the next system at the next restart? | Set it | `recab set-next` |
| Cancel a set restart | Cancel the set restart? | Cancel the restart | `recab commit` |
| Keep the running system | Keep the running system? | Keep it | `recab commit` |
| Go back | Go back to the system from before? | Restart into it | `recab rollback`, then `reboot` |
| Reset the next system | Reset the next system to the running one? | Reset | `recab discard`, then `recab prepare` |
| Discard it | Discard the next system? | Discard | `recab discard` |
| Continue after an interruption | Continue where it stopped? | Continue | `recab recover` (what recab's operation says) |
| Add a release (also from a file) | Add LevitateOS 2026.12 to the next system? | Add it | `recab update apply` |
| Take a release out, carry on adding it | Take the release out of the next system? / Carry on adding the release? | Take it out / Carry on | `recab update cancel` / `recab update recover` |
| Adopt the original release's baseline (a file is chosen first) | Adopt the original release as the baseline? | Adopt it | `recab update adopt-baseline --manifest FILE` |
| Reproduce a package for the release being added | Reproduce PACKAGE through Recipe? | Reproduce it | `recab update reproduce PACKAGE` |
| Replace a failed system recab keeps no preparation for | recab's label, as a question | Replace it | `recab prepare` (recab's argv for `replace_failed`), says recab's reason and `destructive` mark |
| A request that would reuse the place of the system from before | Make this change in the next system? | Prepare the next system | the agent |

"What runs" says whether a password is asked from the root helper: with
`pkexec` "The administrator password is asked for" (for each command, when two
run: `pkexec` asks again for `reboot`); with `RECAB_GUI_ROOT_HELPER` empty (the
stand-in) "This window runs it itself; no password is asked." When recab marks
the discard operation destructive (the real recab does) the reset and discard
pages say the changes cannot be recovered afterwards; without that mark they do
not claim it. Operations of the recovery list that recab offers with their own
label (`Reboot`, `finalize`, `discard`, ...) get the page with the matching
words, or one built from the label and the `destructive` mark. Not given the
page: handing a request to the agent (it has its own page: the request, the
facts, the expected result), looking for a release and downloading it (they
look and fetch, they change no system), the steps that ask for values first
(install, edit, remove, update, a clash decided by hand: those dialogs are the
question), and what the agent itself runs through recab (the page cannot say
which commands, or whether a password is asked).

The direct calls of the story (`finish(false)`, `restart(false)`,
`goBack(false)`, ...) do the thing at once, without the page.

The checks: both stories end with `all flows ended where they should` and, for
each action in a state where it applies, take a picture (`03-goes-into-the-next-system`
and `b02-*` to `b09-*` in the first story; `br01-*` to `br14-*` in `--real`),
expect the page's words and press Cancel to see that nothing was started and
the system reads the same. The restart set for later and its cancelling are
also done through the page's own button (`br03`, `br04`). To see the password
line as a root helper gives it, name a helper that only runs its arguments as
`RECAB_GUI_ROOT_HELPER` for a run.

## Help and the first page

Both are the frame's (`gui/kit/README.md`, "Help" and "The first time"); the window adds
nothing for them. Help for System has the ideas (the running system and the next
system, finishing, the restart on trial, coming back), what it can do with a link to
Overview, "The next system", "Getting back" or History where it is done, the agent, and
what to do when a run stopped, the agent does not answer, or the desktop does not start
(a link to Getting back). Its links to the rail entries are chosen as a click on the rail
is, so `Frame::chosen` already shows the page; the window has no extra page to offer
(`addHelpTarget`). The first page, shown once per person the first time System opens
(never in a picture run), is Help's first sentences and the agent's state, with "Go on
to System". Both stories check them with `../kit/helpstory.*`: pictures `h-help*`,
`h-help-went`, `h-firstrun-*` (`rh-*` in `--real`).

## Reading and reading again

`recab status --json` is run without holding the window up. The first time, the
page says "Reading the system"; later reads leave the page as it is and the
status bar says when the system was last read. It is read again after every
change, on "Read again" in the status bar, on F5 or Ctrl+R, and when the person
comes back to the window after 20 seconds or more, since an agent or a terminal
may have changed the system meanwhile. When it cannot be read, the page says
why and offers "Try again".

## What it asks of the agent

The agent is one program that does the work by calling recab (and Recipe) for
the next system only. It is handed the prompt as one JSON line on its input,
`{"prompt": "..."}`, and the input stays open. The prompt has up to four
headed parts: `The request:` (when the person asked for a change), `The
issue:`, `The facts:`, `The expected result:`. It reports with the same
events as above, and may ask: a `question` event with an `id`, a `text` and
`choices` (a list, or none for a free answer) is shown on the work page, and
the answer comes back on its input as `{"question": id, "answer": "..."}`. The
default agent is `levitate-agent`: it starts Codex (or Claude Code) as the person
and asks before any command outside the agent's sandbox and before every command
as administrator (`gui/agent/README.md`).

## The source

| File | What |
|---|---|
| `src/model.*` | the status, as read from JSON |
| `src/client.*` | the only code that runs `recab`, the agent and the restart |
| `src/pages.*` | Overview, the next system, getting back, History, the release, ask, a clash, what the agent did, the questions |
| `src/beforewords.*` | the words of the Before page for each action |
| `src/window.*` | the flows between pages, inside the frame |
| `src/shots.*` | the picture mode and flow check |
| `fixtures/` | `recab-real`: a stand-in with the real recab's status (schema 10), `history` and commands (with `agent-real`, `reboot-real`, `os-release`; see its top for the variables that make a check fail, find nothing, make a restart fail, or hide `history`); `recab`: the older stand-in with updates, clashes and proposals (with `agent`, `reboot`) |
| `data/` | the menu entry and the polkit action |
| `../kit/` | shared with `gui/recipe` and `gui/disk`: the frame, Activity, the next-system reader, widgets, the page base, the work page, the command runner, words, stand-in looks (see `../kit/README.md`) |

## Not done

- History: lists what recab's audit log and state hold (`tools/recab/README.md`,
  `history`). Not in it, because recab does not record it: who asked for anything
  but a change, when a `prepare` ended, the end of a `recab change`'s own
  audit record (changes are read from the preparations the log holds), a changelog
  of the release, the log of a start (the mock-up's "Read that start's log, 212
  lines") and "Hand to the agent" from a past entry. Entries are in UTC order of
  recab's clock; the page shows local time.
- Release: no release medium noticed and offered, no package or kernel lists, no
  room needed, no last-checked time, no "Skip this release" (see "The newer
  pages"). "Choose a file…" asks for a path with a file dialog; it was only run
  in the stand-in (never in a dialog).
- Single system, and "Stop" or stand-still detection for work started elsewhere:
  recab reports neither (see "The newer pages").
- On "The next system": no size column and no "Take out" (recab gives no sizes and
  has no command that undoes one change), no "Edit a file…" (recab has `edit PATH`,
  but a person would need a file picker and an editor, not designed), no kernel line (status names none), no "waits for your
  approval" row (the agent's questions are in Activity).
- On "Getting back": "hold Space" is left out, because nothing in recab's source says
  which key shows the start-up list (only `timeout 5`, which keeps it up for 5
  seconds where recab wrote loader.conf: an existing loader.conf keeps its owner's
  settings). Two of the design's four "does not turn back" lines are left out:
  software installed for yourself only, and settings of the computer itself such as
  the start-up order; recab's text confirms neither. The names in the start-up list
  are put together from the os-release `NAME`, not read from the entry file.
- The start-up steps are written for today's state; they were not tried on a real
  A/B computer's start-up list.

- Activity lists only the agent's runs. Installs, updates and restarts are not
  recorded anywhere yet, so they are not listed and nothing is made up for them.
- A run waiting for an answer can be answered in any window's Activity, with its
  choices as buttons (and Approve or Refuse for a command); "Go to it" stays for this
  window's own run. A window that closes lets go of its run, which goes on and is found
  in Activity; "Stop" ends it for real (`levitate-agent --stop`).
- The frame, Activity and the strip were checked against stand-ins in the
  picture modes, offscreen; not on a real desktop, and not with a real Codex.
- Settings and Help are the frame's own pages (`gui/kit/README.md`); the story checks Settings with the stand-in Codex, in a settings folder of its own under the picture folder, so the person's `agent.json` is never read or written.
- Help and the first page say nothing about: the single system (not built), "Stop" or stand-still detection for work started elsewhere, a release medium noticed and offered, "Skip this release", the log of a start, the kernel line, "Edit a file…" and "Take out" on the next system, and "hold Space" at start-up. They say that a newer release can be found only when recab has a place to get releases from, and say of the start-up steps only what Getting back itself says (they have not been tried on a real A/B computer's start-up list).

- A run on a real A/B system: the mapping above was written from recab's
  generated types and checked against a stand-in of the same shape, not
  against a booted LevitateOS or AcornOS.
- Taking one change out of the next system: recab lists every change and has
  no command that undoes one.
- Who made a change is only as good as what the caller says: `--actor` is a
  label recab records, not something it checks.
- Production online updates still need a configured distribution origin and keys.
  The window exposes backend check/download/apply and signed media operations.
- A real agent doing a System request: `levitate-agent` stands between and asks
  before administrator commands, but no System request has been run through it
  against the real Codex, and recab changes need root, so each is an asked
  `RUN-AS-ADMINISTRATOR` command.
- Desktop conflict dialogs are compiled and use one revision at a time; actual
  booted-system GUI validation is still outstanding.
- It does not notice a change made elsewhere while the window is in front and
  nobody presses anything; it reads again on coming back to it.
- Going back when no restart is wanted yet; a notice when an update appears or
  a trial was kept or failed; translations; shipping it in the desktop image.
