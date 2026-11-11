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
"Overview", "The next system" and "Getting back"; "Activity" and "Settings" below), the pages, a strip, and a status line.
The Ask, clash and work pages are shown in the same place and the rail keeps
marked the page the person came from. "The next system" has a badge: the number of
changes it holds, when it exists. The strip ("The next system holds N changes." and
"Finish and restart…", which goes to "The next system") shows on every page but
Overview and "The next system", which say it themselves, and on Activity, while the
next system holds changes. Activity lists the agent's
runs from all three apps. The status line has what was read and when, with "Read
again", and the agent (`Agent: Codex, signed in` when `RECAB_GUI_AGENT` is
`levitate-agent`; its file name for a stand-in). Window size 1100 by 700 the first time, then the size it had when it closed (`rememberWindowSize` in `gui/kit`; the picture modes always use 1100 by 700).

## State (2026-11-10)

**The real interface is recab status schema 10.** The existing pages read its
lifecycle and offered operations, with concrete argument vectors and declared
inputs. The software/update/recovery section supports Recipe changes, signed
updates, merge decisions, reproduction and interrupted-work recovery. Every
request retains the preparation revision/generation the person viewed; conflict
choices refresh between submissions. Earlier schemas 7–9 keep their limited
boot/preparation mapping, and schema 3 remains a drawing fixture.

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
| A change is asked for | Say it in words; the page shows what the agent is given. A question first when the place of the system from before will be reused | | |
| Preparing | The steps, how far, the output. Stop keeps what is done | | |
| Open, holding changes | Finish; add the update (older stand-in) | Every change with who and when; what will differ; the three steps, step 1 current: Finish | the system from before is gone: its place holds the next system |
| A clash (older stand-in) | Settle the clash | Settle the clash | |
| Ready | Restart into it | Step 2 current: Restart now, Start it at the next restart | |
| Restart set | Restart now | Restart now, Cancel the set restart | Cancel it |
| On trial | Keep this system | "You are on the next system, on trial": Keep it, Go back | Keep it; Go back |
| Being worked on elsewhere | one sentence | the list, and one sentence: nothing can be started | |
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
  `codex`, which will need its own arguments once this is real).
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
lead where it should. It needs no display:

```sh
QT_QPA_PLATFORM=offscreen RECAB_GUI_RECAB=$F/recab RECAB_GUI_AGENT=$F/agent \
  RECAB_GUI_REBOOT=$F/reboot RECAB_GUI_ROOT_HELPER= \
  RECAB_STANDIN_STATE=/tmp/recab-shots-state RECAB_STANDIN_FAIL_TRIALS=1 RECAB_STANDIN_SPEED=0 \
  .artifacts/recab-gui/build/recab-gui --look levitate --shots /tmp/recab-shots
```

Both stories also check the frame, Activity and the strip. For that they make
runs with `gui/agent/fixtures/records` (stand-in Codex, never a real agent) and
read them through the built `levitate-agent`, so two more variables name them
(build the agent first: `cmake --build .artifacts/agent/build`):

```sh
export RECAB_GUI_RECORDS=$PWD/gui/agent/fixtures/records
export LEVITATE_ACTIVITY_AGENT=$PWD/.artifacts/agent/build/levitate-agent
```

Without them the story says what is missing and fails. The pictures `s0-*` to `s14-*` are Settings: as first read, "Allowed", signing out (and the question first), a sign in that fails, signing in (the address Codex printed), another program, back to Codex, a refused value, "Try it" and the run in Activity, a helper that cannot be asked. The story sets `HOME`, `XDG_CONFIG_HOME`, `XDG_STATE_HOME` and `LEVITATE_AGENT_CODEX` itself for that part and puts them back. The pictures `a-*`
(`r2c-*` in the real story) are Activity: the list, a done run, a run that
waits, this window's own run, the strip on Activity. The pictures `p*` (older stand-in) and `rp*` (`--real`) are "The next system" and "Getting back" in the states each stand-in reaches; `p-steps-old.txt` and `rp-steps.txt` are the steps saved as a file. The other pictures show
Overview and the pages inside the frame.

The story the real recab offers (two copies, a change by the agent, a command
running elsewhere, finishing, setting the restart and cancelling it, the
restart, the trial kept by hand, going back, the reset, discarding) runs against `fixtures/recab-real` with
`--real`. It stops at once when the system cannot be read, before anything is
asked of the agent. Always name a stand-in agent for it: the default agent is
`levitate-agent`, which starts Codex (see `gui/agent/README.md`).

```sh
QT_QPA_PLATFORM=offscreen RECAB_GUI_RECAB=$F/recab-real RECAB_GUI_AGENT=$F/agent-real \
  RECAB_GUI_REBOOT=$F/reboot-real RECAB_GUI_ROOT_HELPER= RECAB_GUI_OS_RELEASE=$F/os-release \
  RECAB_STANDIN_STATE=/tmp/recab-real-shots-state RECAB_STANDIN_SPEED=0 \
  .artifacts/recab-gui/build/recab-gui --look levitate --shots /tmp/recab-real-shots --real
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
| `recab update apply` | add the newer release to the next system, creating it when needed |
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
| Continue (after an interruption) | `recab prepare --preparation ID --revision N` |
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
the answer comes back on its input as `{"question": id, "answer": "..."}`. How a real Codex or Claude Code is started, what it is
allowed to run, and whether it runs as the person or as root are not decided.

## The source

| File | What |
|---|---|
| `src/model.*` | the status, as read from JSON |
| `src/client.*` | the only code that runs `recab`, the agent and the restart |
| `src/pages.*` | Overview, the next system, getting back, ask, a clash, what the agent did, the questions |
| `src/window.*` | the flows between pages, inside the frame |
| `src/shots.*` | the picture mode and flow check |
| `fixtures/` | `recab-real`: a stand-in with the real recab's status and commands (with `agent-real`, `reboot-real`, `os-release`); `recab`: the older stand-in with updates, clashes and proposals (with `agent`, `reboot`) |
| `data/` | the menu entry and the polkit action |
| `../kit/` | shared with `gui/recipe` and `gui/disk`: the frame, Activity, the next-system reader, widgets, the page base, the work page, the command runner, words, stand-in looks (see `../kit/README.md`) |

## Not done

- History: the design has a page that lists past changes. recab has no command
  that lists them (`status` holds only the open preparation's changes and the last
  trial), so there is no page and no rail entry for it.
- On "The next system": no size column and no "Take out" (recab gives no sizes and
  has no command that undoes one change), no "Edit a file…" (recab has `edit PATH`,
  but a person would need a file picker and an editor, not designed), no adding a
  release (see below), no kernel line (status names none), no "waits for your
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
- A run waiting for an answer can be answered only in the window that started
  it; Activity says so and offers "Go to it" for this window's own run.
- The frame, Activity and the strip were checked against stand-ins in the
  picture modes, offscreen; not on a real desktop, and not with a real Codex.
- The rail has no Help: that page would lead nowhere yet. Settings is the frame's own page (`gui/kit/README.md`); the story checks it with the stand-in Codex, in a settings folder of its own under the picture folder, so the person's `agent.json` is never read or written.

- A run on a real A/B system: the mapping above was written from recab's
  generated types and checked against a stand-in of the same shape, not
  against a booted LevitateOS or AcornOS.
- Taking one change out of the next system: recab lists every change and has
  no command that undoes one.
- Who made a change is only as good as what the caller says: `--actor` is a
  label recab records, not something it checks.
- Production online updates still need a configured distribution origin and keys.
  The window exposes backend check/download/apply and signed media operations.
- A real agent, and the approval of what it may do. Codex and Claude Code do
  not speak the lines above as they are; something has to stand between.
- Desktop conflict dialogs are compiled and use one revision at a time; actual
  booted-system GUI validation is still outstanding.
- It does not notice a change made elsewhere while the window is in front and
  nobody presses anything; it reads again on coming back to it.
- Going back when no restart is wanted yet; a notice when an update appears or
  a trial was kept or failed; translations; shipping it in the desktop image.
