# recab-gui

The graphical front end for recab on the desktop images: one window, titled
"System", for changing and updating a locked system. It follows the lifecycle
in `tools/recab/REQUIREMENTS.md` 0.3.0 and the premise that an agent does the
work: the person says what they want, or what went wrong is put together for
them, and an agent is handed the issue, the facts and the expected result.

It holds no lifecycle rules. It runs `recab`, reads its status, and offers
what recab says may be done.

Qt 6 Widgets, C++20, CMake. Widgets shared with `gui/recipe` and `gui/disk` are in `gui/kit`.

## State (2026-11-10)

**It reads and drives the real recab, and that has not run on a real A/B
system.** The window reads recab's own status (schema 6: two slots and a
preparation) and sends recab's own commands (`prepare`, `finalize`, `discard`
with the preparation's id and revision, `set-next`, `rollback`). This was
checked two ways: against `fixtures/recab6`, a stand-in that speaks exactly
the real recab's status and commands, where the whole story passes; and
against the real `recab` binary on a development machine that is no A/B
system, where the window shows recab's own refusal and offers to read as
administrator.

What the real recab does not have, the window does not show with it: an
update from within the window, a clash between an update and a change, and an
agent's proposal waiting to be accepted. Those exist only in the older
stand-in (`fixtures/recab`, schema 3), which stays for drawing those pages.

Still a stand-in everywhere: the agent, the restart and `pkexec`.

## What a person sees

There are no slot letters anywhere, so the window is the same in both
directions.

| State | The window |
|---|---|
| Stable | "Your system is locked": the running system, and beside it the system from before. Make a change, add an available update, go back |
| A change is asked for | Say it in words; the page shows what the agent is given. A question first when the place of the system from before will be reused |
| Preparing | The steps (make it level, make the change, record it), how far, the output. Stop keeps what is done |
| The next system holds changes | Its changes listed. Finish, add the update to this same system, change something else, discard |
| A clash | The update and a change altered the same file: the prompt for the agent, or decide by hand |
| What the agent did | Its result against each thing that was expected; accept, ask again, or decide by hand |
| Ready | Restart into it; it comes back by itself if it does not start |
| It did not start | Back on the untouched running system; the prompt for the agent, try again, or discard |
| Interrupted | Continue where it stopped, or discard |
| After a kept restart | "You are on the new system" |

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

The story the real recab offers (two copies, a change by the agent, finishing,
the restart, going back, discarding) runs against `fixtures/recab6` with
`--real`. It stops at once when the system cannot be read, before anything is
asked of the agent. Always name a stand-in agent for it: the default agent is
`codex`.

```sh
QT_QPA_PLATFORM=offscreen RECAB_GUI_RECAB=$F/recab6 RECAB_GUI_AGENT=$F/agent6 \
  RECAB_GUI_REBOOT=$F/reboot6 RECAB_GUI_ROOT_HELPER= RECAB_GUI_OS_RELEASE=$F/os-release \
  RECAB_STANDIN_STATE=/tmp/recab6-shots-state RECAB_STANDIN_SPEED=0 \
  .artifacts/recab-gui/build/recab-gui --look levitate --shots /tmp/recab6-shots --real
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
| "The second copy" | no preparation, and both slots hold the same generation |
| "The system from before" | no preparation, and `rollback_slot` is the other slot |
| "The next system", open | a preparation in phase `prepared` |
| "The next system", ready | a preparation in phase `ready`; with `trial_pending`, the restart is set |
| "interrupted" | a preparation in any other phase, or with a `failure` |
| "did not start, so the computer came back" | `last_trial` names the other slot and was not committed, and it is still ready |
| the next system's changes | only `preparation.last_change` and `change`: recab's status names the last one, not all |
| Make a change | allowed when a preparation is open, else by the verdict on `prepare` |
| Finish | `recab finalize --preparation ID --revision N` |
| Discard | `recab discard --preparation ID --revision N` |
| Continue (after an interruption) | `recab prepare --preparation ID --revision N` |
| Restart into it | `recab set-next SLOT`, then the restart |
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
| `src/pages.*` | the system page, ask, work, a clash, what the agent did, the questions |
| `src/window.*` | the flows between pages |
| `src/shots.*` | the picture mode and flow check |
| `fixtures/` | `recab6`: a stand-in with the real recab's status and commands (with `agent6`, `reboot6`, `os-release`); `recab`: the older stand-in with updates, clashes and proposals (with `agent`, `reboot`) |
| `data/` | the menu entry and the polkit action |
| `../kit/` | shared with `gui/recipe` and `gui/disk`: widgets, the page base, the work page, the command runner, words, stand-in looks |

## Not done

- A run on a real A/B system: the mapping above was written from recab's
  generated types and checked against a stand-in of the same shape, not
  against a booted LevitateOS or AcornOS.
- Everything of the next system's changes: recab's status names the last one;
  `recab change list` is not read.
- Updates from within the window: the real recab takes a release from media
  (`recab stage`), which the window does not offer.
- A real agent, and the approval of what it may do. Codex and Claude Code do
  not speak the lines above as they are; something has to stand between.
- Deciding several clashes by hand was written but never run: the stand-in
  only ever makes one clash.
- It does not notice a change made elsewhere while the window is in front and
  nobody presses anything; it reads again on coming back to it.
- Going back when no restart is wanted yet; a notice when an update appears or
  a trial was kept or failed; translations; shipping it in the desktop image.
