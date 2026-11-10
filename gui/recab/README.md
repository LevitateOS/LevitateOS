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

**It runs against a fake backend only.** `fixtures/recab` is a stand-in that
plays the whole lifecycle from a state file; `fixtures/agent` and
`fixtures/reboot` are the same program standing in for an agent and for the
restart. The real `recab` does not answer in this form yet: it reports schema
2 (slots and staging), and this window refuses anything but schema 3. Nothing
here has run on a real system, with a real agent, or with `pkexec`.

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
| `fixtures/` | the stand-in recab, agent and reboot |
| `data/` | the menu entry and the polkit action |
| `../kit/` | shared with `gui/recipe` and `gui/disk`: widgets, the page base, the work page, the command runner, words, stand-in looks |

## Not done

- The real `recab` (its status is schema 2; most of the lifecycle is not built
  there yet, see `tools/recab/README.md`).
- A real agent, and the approval of what it may do. Codex and Claude Code do
  not speak the lines above as they are; something has to stand between.
- Deciding several clashes by hand was written but never run: the stand-in
  only ever makes one clash.
- It does not notice a change made elsewhere while the window is in front and
  nobody presses anything; it reads again on coming back to it.
- Going back when no restart is wanted yet; a notice when an update appears or
  a trial was kept or failed; translations; shipping it in the desktop image.
