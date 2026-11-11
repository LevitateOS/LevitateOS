# recipe-gui

The graphical front end for Recipe on the desktop images: Plasma on LevitateOS,
LXQt on AcornOS. One window to find software, see what its recipe will do and
may use, install, update, go back a version and remove.

It holds no package rules. It runs `recipe`, reads its JSON and shows it.

Qt 6 Widgets, C++20, CMake. It uses no stylesheet and no colours of its own
beyond the red for "this went wrong", so it takes each desktop's own style.

## The window: the shared frame

The window is `gui/kit/frame.*` (see `../kit/README.md`): the rail on the left
(the three apps, then Browse, Installed, Updates with the number of updates as
its badge, Sources, then Activity, which the frame owns), the pages in the
middle, a strip under them and a status line. There is no sidebar any more:
the search, the box for the source and the box for the kind of software are
the top of the Browse page. Settings is under Activity (the frame's own page, the
same in all three apps; `gui/kit/README.md`); Storage and Help are not in the rail
because those pages do not exist. The story opens Settings (picture `f-settings`),
reading through the built levitate-agent with the stand-in Codex and a settings
folder of its own.

- The status line says where the recipes are from and when they were read (and
  what is being built); "Read again" sits beside it; the agent is the frame's
  (`Agent: agent` for a stand-in, by its file name).
- "Ask for software" hands the agent the app name "Recipe" and the first line of
  the request as the run's title, so Activity lists it; "Go to it" on Activity
  comes back to the run's page. "Let the agent fix it" is run by `recipe
  --autofix`, not by this window's agent, so it is not in Activity.
- The strip ("The next system holds N changes.") is read with `recab status
  --json` (`RECIPE_GUI_RECAB`, default `recab`) after every read of the recipes.
  It shows on every page; when recab cannot be run there is no strip and no
  error. "Finish and restart…" starts System (`LEVITATE_APP_SYSTEM`, default
  `recab-gui`), as System's button in the rail does.

## State (2026-11-10)

Every page works against the stand-in in `fixtures/`, which answers in the
real `recipe`'s shapes. Against the real `recipe` (tools/recipe `971b9b6`) it
reads the collection (`list --json`), the installed list with the earlier
versions kept, the updates with what the newer recipe may use, and what the
store takes; this was run read-only on a hand-made store with
`recipes/ripgrep.rhai`. Installing, going back and clearing through the window
with the real `recipe` have not been run, nor has it run on a real desktop or
with `pkexec`. The tables below say what the front end asks that `recipe`
lacks.

## Build and run from a checkout

```sh
cmake -S gui/recipe -B .artifacts/recipe-gui/build -G Ninja
cmake --build .artifacts/recipe-gui/build

RECIPE_GUI_RECIPE=$PWD/gui/recipe/fixtures/recipe RECIPE_GUI_ROOT_HELPER= \
  .artifacts/recipe-gui/build/recipe-gui --look levitate
```

Needs the Qt 6 base and SVG development files (`qt6-qtbase-devel`,
`qt6-qtsvg-devel` on Rocky and Fedora).

- `RECIPE_GUI_RECIPE`: the `recipe` to run (default: `recipe` on `PATH`).
- `RECIPE_GUI_AGENT`: the agent for "Ask for software" (default `levitate-agent`).
- `RECIPE_GUI_RECAB`: the `recab` that tells the strip what the next system
  holds (default: `recab` on `PATH`).
- `RECIPE_GUI_ROOT_HELPER`: what changes are run through (default: `pkexec`;
  empty runs them directly, for the stand-in).
- `RECIPE_GUI_RECIPES_PATH`: the collection to list and install from
  (default: `recipe`'s own choice).
- `RECIPE_GUI_PACKAGE_ROOT`: when set (even to nothing), one explicit store and
  nothing else: one place, no choice (see "The two places"). The stories and the
  real-recipe check below use it. Unset: the two places.
- `RECIPE_GUI_SYSTEM_ROOT`: the computer's store when the computer is not locked
  (default `/var/lib/recipe`). It exists for checks, which cannot write there.
- `RECIPE_GUI_CMDLINE`: the file read to tell whether the computer is locked
  (default `/proc/cmdline`); locked means `recab.immutable` is one of its words.
  For checks.
- `--look levitate|acorn`: a stand-in look for each desktop, for a machine
  that is neither.
- `--shots FOLDER`: walk the pages, save a picture of each, quit. With
  `QT_QPA_PLATFORM=offscreen` it needs no display:

```sh
QT_QPA_PLATFORM=offscreen RECIPE_GUI_RECIPE=$PWD/gui/recipe/fixtures/recipe \
  RECIPE_GUI_ROOT_HELPER= RECIPE_GUI_PACKAGE_ROOT=/tmp/recipe-shots-store \
  RECIPE_STANDIN_STATE=/tmp/recipe-shots-state \
  .artifacts/recipe-gui/build/recipe-gui --look acorn --shots /tmp/recipe-shots
```

The story also checks the frame, Activity and the strip, so it needs the
stand-in agent, a recab that speaks the real shape with a scratch state of its
own, and the records Activity shows (stand-in Codex, never a real agent; build
the agent first with `cmake --build .artifacts/agent/build`). Always name the
stand-ins: the default agent starts Codex.

```sh
F=$PWD/gui/recipe/fixtures
QT_QPA_PLATFORM=offscreen HOME=/tmp/recipe-shots-home \
  RECIPE_GUI_RECIPE=$F/recipe RECIPE_GUI_AGENT=$F/agent RECIPE_GUI_ROOT_HELPER= \
  RECIPE_GUI_PACKAGE_ROOT=/tmp/recipe-shots-store \
  RECIPE_STANDIN_STATE=/tmp/recipe-shots-state RECIPE_STANDIN_SPEED=0 \
  RECIPE_GUI_RECAB=$PWD/gui/recab/fixtures/recab-real RECAB_STANDIN_STATE=/tmp/recipe-shots-recab \
  RECAB_STANDIN_SPEED=0 RECIPE_GUI_RECORDS=$PWD/gui/agent/fixtures/records \
  LEVITATE_ACTIVITY_AGENT=$PWD/.artifacts/agent/build/levitate-agent \
  .artifacts/recipe-gui/build/recipe-gui --look levitate --shots /tmp/recipe-shots
```

Delete both state folders first (`/tmp/recipe-shots-state`,
`/tmp/recipe-shots-recab`): the story expects no next system at the start, then
makes one with one `prepare` and one `change`. The story needs
`RECIPE_GUI_PACKAGE_ROOT` (its first window is on one explicit store, so the
Before page has no choice) and says so when it is missing. Pictures: `1-…` to
`14-…` are the flows (`2b-before-install`, `4b-before-update` are the Before
step); `f1`–`f4` Browse (also with a search and a kind), Installed, Updates and
Sources in the frame; `s1`–`s3` the strip and a window with no recab; `a-*`
Activity; `p1`–`p6` the two places (below).

The last part of the story opens two more windows of its own, with a scratch
command line, scratch stand-in state and a scratch system store under
`FOLDER/places`: a locked computer (`recab.immutable`) and one that is not.
The stand-in logs the arguments of every `recipe` it is run with
(`RECIPE_STANDIN_LOG`, default `invocations.log` in its state folder), and the
story checks the exact arguments of every change, which route the stand-in
told apart (`system`, `store` or `me`), and that "for me" never reaches the
root helper (`fixtures/helper-log` notes what it is given and runs it). The
locked window is run with `RECIPE_STANDIN_PREPARED=1`, which makes the stand-in
answer `prepared` as it does to root, and with `RECIPE_RECAB_BIN` set, so that
`--system` changes are also made in the recab stand-in and the strip can be
read.

The stand-in keeps what is "installed" in state files
(`RECIPE_STANDIN_STATE`, see the top of `fixtures/recipe`): `installed.json` for
an explicit `--package-root`, `installed-me.json` for no `--package-root` (the
person's own store, which starts with `tree` 2.3.1), and `system.json` for
`--system` (what the running system holds and what the next one will); delete
the folder to start over. The stand-in agent writes the app name and title it was given to
`last-prompt.json` there, which the story reads.

Against the real `recipe` from the checkout, into a test store, with the
Ginux launcher and image selected as `tools/recipe/README.md` describes.
`fixtures/as-root` stands in for `pkexec` with `sudo`. `--try NAME` shows the
recipe, installs it, pictures each point and removes it again; `--read-only`
installs nothing:

```sh
export GINUX_BIN=... GINUX_ROOTFS=... GINUX_ROOT=... RECIPE_WORKER_BIN=$PWD/target/debug/recipe
export RECIPE_STATE_DIR=$PWD/.artifacts/recipe-gui/real/state
QT_QPA_PLATFORM=offscreen RECIPE_GUI_RECIPE=$PWD/target/debug/recipe \
  RECIPE_GUI_ROOT_HELPER=$PWD/gui/recipe/fixtures/as-root \
  RECIPE_GUI_RECIPES_PATH=$PWD/recipes \
  RECIPE_GUI_PACKAGE_ROOT=$PWD/.artifacts/recipe-gui/real/packages \
  .artifacts/recipe-gui/build/recipe-gui --look levitate \
  --shots $PWD/.artifacts/recipe-gui/shots-real --try ripgrep
```

## The source

| File | What |
|---|---|
| `src/model.*` | the data: recipes, what is installed, updates, as read from JSON |
| `src/client.*` | the only code that runs `recipe`; a `Run` reports one change as events |
| `../kit/` | shared with `gui/recab` and `gui/disk`: small widgets (step mark, letter tile, panels, the prompt handed to an agent), the page base, the work page, the command runner, sizes and counts as words, the stand-in looks |
| `../kit/frame.*`, `activity.*`, `nextsystem.*` | the window frame, the Activity page it owns, and the strip's reader |
| `../kit/before.*` | the Before page: a question, "Nothing has started...", sections the app fills, Cancel and one button |
| `src/beforewords.*` | the words for it, from what `recipe` reported |
| `src/pages.*` | Browse (with its search and boxes), a recipe's page, a run, Installed, Updates, the remove question |
| `src/sources.*` | the Sources page, and asking the agent for a recipe |
| `src/window.*` | the frame, its rail and status, and the flows between pages |
| `src/shots.*` | the stand-in looks and the picture mode |
| `fixtures/` | the stand-in `recipe` with its sample catalog and sources, and the stand-in agent |
| `data/` | the menu entry and the polkit action |

## What the front end asks of `recipe`

Reads, as the desktop user, JSON on stdout with `"schema_version": 1` (the same commands without `--package-root` read the person's own store; `recipe --system installed --json` reads a locked computer, in recab's own shape, see "The two places"):

| Command | Answer | In `recipe` today |
|---|---|---|
| `recipe list --json` | every recipe: `name`, `version`, `description`, `about`, `sources`, `needs`, `deps`, `build_deps`, `hooks`; and the source's name | exists, without `sources` and the source's name. `ABOUT` is passed through unchecked; `SOURCES` is still only the design in `docs/recipe-front-end-format.md` |
| `recipe installed --json` | `packages`: each one's `name`, `version`, `recipe`, `files`, `installed_on` (seconds), `size_bytes` and `kept`: the earlier versions still held, newest first, each with `version`, `generation`, `installed_on`, `size_bytes` | exists. A receipt written before 2026-11-10 has no `installed_on` or `size_bytes`; the front end then measures the files. It takes programs from the `bin/` files and menu entries from `share/applications/` |
| `recipe updates --json` | `updates`: for each installed recipe that `recipe update` would change: `name`, `source`, `from`, `to`, `reason` (`version`, or `recipe` when the source publishes another recipe file for the same version) and `changes`; `problems`: installed software it could not look up | exists. `changes` lists every field of NEEDS that differs from what was recorded at install (`field`, `before`, `after`, `kind`: `added`, `removed`, `raised`, `lowered`, `changed`), and is `null` for a receipt that recorded nothing; the page then says the two cannot be compared |
| `recipe storage --json` | `active_bytes`, `earlier_generations`, `earlier_bytes` | exists. Shown on the Installed page |

Changes, through the root helper (as the person for software "for me"; with `--system` first and no `--package-root` for a locked computer; see "The two places"), with events on stderr:

| Command | In `recipe` today |
|---|---|
| `recipe install NAME --machine-events` | exists: runs the recipe in Ginux and activates its `package` output |
| `recipe update NAME --machine-events` | exists |
| `recipe remove NAME --machine-events` | exists, by the receipt. What it removes stays among the earlier versions |
| `recipe go-back NAME --machine-events` | exists: activates the newest earlier version again from the files a generation still holds; sends no events |
| `recipe storage --clear-earlier` | exists: deletes every earlier version. The window asks first ("Clear earlier versions…" on the Installed page). Recipe cannot clear one program's earlier versions alone, so the remove question no longer offers that |
| `recipe install NAME --autofix --autofix-prompt-file FILE --machine-events` | exists for `install`: Recipe's agent repair (Codex or Claude), with the front end's prompt appended to its own. Never run against a real agent |
| `recipe update NAME --autofix --autofix-prompt-file FILE --machine-events` | `update` takes no such options yet |

Events are one JSON object per line. The front end reads these; any other
line is shown as output. "Real" is what `recipe --machine-events` sends since
tools/recipe `89d4753`; of those, only `done` and `failed` have been seen from
the real `recipe` (the others need a Ginux run).

| Event | Fields | Real |
|---|---|---|
| `plan` | `recipes`: each `name` and its `steps`, in the order they will run | yes, for an install or update, before the first recipe runs |
| `plan` | `recipe`, `steps`: the steps that recipe needs this time; the others are marked as not needed | yes, once that recipe's checks have run |
| `recipe-hook` | `recipe`, `hook`, `status` (`running`, `success`, `failed`, `skipped`, and others the front end ignores), `msg` | yes; it is Recipe's own lifecycle event. `step` with `status` `done` is read the same way |
| `fetch` | `recipe`, `source`, `bytes`, `total` (null when the server did not say) | yes |
| `agent` | `recipe`, `hook`, `attempt`, `files`, `summary`: the agent changed the recipe during this run, which starts again | yes, from `--autofix` |
| `done` | `recipe` | yes, the last event |
| `failed` | `recipe`, `hook`, `reason` | yes, the last event; `hook` only where a step failed |
| `failed` … `refused` | `endpoint` and the `declared` list, for a refused connection | no: Recipe does not name the endpoint. Only the stand-in sends it, and the page that names the endpoint depends on it |
| `progress` | `recipe`, `done`, `total` | no: nothing inside a build step is counted |
| `say` | `recipe`, `text` | no |
| `output` | `recipe`, `line` | no: the real `recipe` prints plain lines, which are shown as output |

Without a `plan`, the front end adds a row of three steps for each recipe as
its first step arrives.

## The two places

Software goes to one of two places, and the window says which, in words, on
the Before page, on every row of Installed and Updates, and on the recipe's page.

| Place | Where | What `recipe` is run with |
|---|---|---|
| For me | the person's own store (`~/.local/share/recipe`); in effect at once, for them only | no `--package-root`, no root helper: `recipe install NAME`, `update`, `remove`, `go-back`, `storage`, as the person |
| For this computer, not locked | the system's store (`/var/lib/recipe`, or `RECIPE_GUI_SYSTEM_ROOT`); in effect at once, for everyone | through the root helper: `recipe install NAME --package-root /var/lib/recipe` (and the same for the others) |
| For this computer, locked | the next system; in effect after a restart, for everyone; the running system is not touched | through the root helper: `recipe --system install\|update\|remove NAME`, which `recipe` hands to `recab change`. What is installed: `recipe --system installed --json`, as the person |
| In the next system, after a restart | software only the next system holds | shown only when the answer carries it (below); removed with `recipe --system remove NAME` |
| One explicit store (`RECIPE_GUI_PACKAGE_ROOT`) | that store; the only place, so no column, no choice | as before: the root helper with `--package-root` that store |

The tells, from `tools/recipe` and `tools/recab` (what the window relies on, with
the lines where each is decided):

- `commands.rs` `system_route()` (line 1381): the system route is taken when
  `--system` is given, or when root runs `install|update|remove|installed|isinstalled`
  with no `--package-root` on a kernel whose command line has the word
  `recab.immutable` (lines 1390-1399). Giving `--package-root` turns the second
  condition off, which is why the window used to install around the lock; now
  it names no store for the person's own software and none on a locked computer.
- `metadata.rs` `default_package_root()` (line 71): root's store is
  `/var/lib/recipe`, a person's is `~/.local/share/recipe`.
- `recipe --system installed --json` is `recab change list`, which prints
  `recab::changes()` (`recab/src/main.rs` line 543, `recab/src/change.rs` lines
  628-647): `{"active": Receipt, "prepared": Receipt or null, "preparation":
  Preparation or null, "activation": "pending reboot for prepared changes"}`.
  `Receipt` is `{schema_version, packages: {name: Package}}` and `Package` is
  `{name, version, recipe, recipe_digest, architecture, target, dependencies,
  files}` (`recipe/src/system.rs` lines 15-30). `prepared` is null unless root
  reads it (and no change is in progress; `change.rs` line 632); `preparation`
  (`recab/src/state.rs` lines 140-165) is not hidden, and its `changes` is
  `[{id, action, package, unix_time, actor}]` (`state.rs` lines 127-137).

What the window shows from it: the running system's packages by name and
version ("For this computer"); the next system's packages when `prepared` is
there ("In the next system, after a restart" for what only it holds, "goes in
the next system" on a running row it no longer holds); and, from `preparation.changes`,
which package has an install or update waiting, so that the recipe page and the
Browse row say "after a restart" instead of offering to install it again. Read
as the person, `prepared` is null: the strip says the next system holds N
changes, but the rows of what it holds appear only for a reader that may read
`prepared`.

For software in the running system of a locked computer the window shows only
what recab answers: no size, no date, no earlier versions, no programs and so no
"Open", no "Go back", nothing to clear (no store stands behind it). It offers
"Update" when the collection's recipe has another version (the Updates page
lists it as "Not compared": `recipe updates` cannot look at a locked system,
and what the installed version was allowed to use is not recorded), and "Remove".
Nothing in `recab` says that an update exists, so that comes from `recipe list`.

## Before it happens

An install or an update does not start at once: the window shows the shared
Before page (`../kit/before.*`, words in `src/beforewords.*`), from the recipe's
page, the Browse list, Installed and Updates ("Update all" asks about each
update in turn). Its button ("Install", "Update") starts the run as before;
"Cancel" and Escape come back to the page the person was on, and nothing was run.
"Try again" and "Let the agent fix it" start at once: they repeat what was
agreed to.

- **Where it goes**: for an install, the places that do not hold the software
  yet, "For this computer" first and picked, with two true lines each. Update
  acts where the software is, so it is only the statement ("Where it is").
  One explicit store, or a place that is already taken, leaves nothing to choose.
- **What happens**: only what the catalog says. Which version of what; that it
  is fetched from the hosts of the recipe's declared network endpoints (not
  from `SOURCES`) and built, inside the sandbox; the recipes it needs (`deps`,
  `build_deps`) that are not installed in that place yet, with the ones only
  the build needs said apart; on a locked computer, "The running system is not
  touched."; for an update, from which version to which.
- **What the recipe may use**: network, the most it may download, memory, disk
  while it builds, time, and "The sandbox refuses anything else." For an update
  what `recipe updates` reported as changed, as the Updates page shows it, and
  where nothing can be compared, the recipe's limits now with the reason.
- **Foot**: which source the recipe is from, and "Read the recipe", which opens
  its file with the desktop's default program when `list --json` gives a `path`.

The page does not say how much is downloaded or how much disk is used:
`recipe` reports neither before it has run. (`network_max_bytes` is a limit,
and is called "Most it may download".) Nor does it say that the running system
can be put back before the restart, which the design had: recab says nothing
of it to the window.

## Reading and reading again

The catalog is read as three commands in a row (`list`, `installed`,
`updates`) without holding the window up. The first time, the page says
"Reading the recipes"; later reads leave the page as it is, and the status line
(the frame's) says where the recipes are from and when they were read. It is read again
after every change, on "Read again" in the status line, on F5 or Ctrl+R, and
when the person comes back to the window after 20 seconds or more. That reads
what is on this computer; it fetches nothing. Fetching the sources' lists is
"Check for updates" on the Updates page and "Fetch again" on the Sources page.
A catalog is four commands now: `list`, `installed`, `updates`, `storage`, once
for each place that has a store (for me; for this computer when it is not
locked), and `--system installed --json` for a locked computer. `list` is asked
of the computer's store (its sources). When `updates` fails the catalog is not
shown and the page says why; when `storage` fails only the sizes are missing;
when the locked computer's own list cannot be read, the person's software is
shown and Installed says so.

## Sources

`recipe list --json` names the sources (`sources`: `name`, `title`, `kind`,
`address` or `path`, `commit`, `checked`, `problem`) and each recipe's
`source`. The window shows one source's recipes or all (the box under the
search, shown when more than one source has recipes), says on every recipe
where it is from, and has a Sources page:

| Action | Command |
|---|---|
| Fetch again, Fetch all again, Check for updates | `recipe refresh [NAME]`, as the person |
| Add a source… (an address, or a folder) | `recipe source add ADDRESS` or `recipe source add --path FOLDER`, as root |
| Remove… | `recipe source remove NAME`, as root |

Installing and updating name the recipe as `SOURCE:NAME`. All of these exist in
`recipe` (see `docs/recipe-sources.md`, steps 1 and 2); the window has run them
against the stand-in only, and read the real `list --json`. Installing from a
remote source is step 3 there and stops with a message until then. When two
sources hold one name, the window shows the first and says nothing of the
other.

## Asking for software that has no recipe

"Ask for software…" under the list, or "Ask for “…”…" after a search, opens a
page where the person says what they want. It goes to the agent as `The
request:`, `The issue:`, `The facts:` (where a person's own recipes go, how a
recipe is checked, which recipes exist) and `The expected result:`, which the
person can change. The agent is `RECIPE_GUI_AGENT` (default `levitate-agent`, which starts Codex; see
`gui/agent/README.md`), handed
the prompt as one JSON line, `{"prompt": "..."}`, with its input left open. It
reports with `plan`, `step`, `output`, `agent` and `done`; an `agent` event
with a `recipe` names what it wrote, and the window opens that recipe to be
read. Nothing is installed. It may ask: a `question` event (`id`, `text`,
`choices`) is shown with a button for each choice, and the answer goes back
as `{"question": id, "answer": "..."}`.

## When a run stops: the agent fixes it

Recipe's premise is that an agent does the maintenance. A stopped run is not
handed back to the person to sort out. The page puts together what the agent
is given, in three parts, and shows it:

- **The issue**: what stopped, in one sentence.
- **The facts**: what Recipe recorded (the refused endpoint and the allowed
  list, Recipe's own message, the recipe's path, the output, what is still
  installed).
- **The expected result**: what should be true afterwards. The person can
  change this part; it is the only part that is theirs to say.

"Let the agent fix it" runs the same install or update again with Recipe's
agent repair on and that text as its extra prompt. "Try again" and "Show the
output" stay beside it. What the agent changed is reported when the run ends;
there is no step yet where the person approves the change before it is used.

## Not done

- Not in the frame: the licence and sort boxes of the design (nothing behind
  them), the Storage and Help pages. The Installed page does not show its count
  in the rail any more.
- The two places and the Before step ran against the stand-in only, never a
  real `recipe` install, a real `recab` or a real root helper. A real
  `--system` run's events (and whether `--machine-events` passes through
  `recab change` as `plan`, `fetch` and `done`) are unknown; the run page
  plays what arrives.
- Before: no download size or disk use (`recipe` does not report them); no
  "you can take it out again before the restart" (recab does not say it);
  Remove still asks in its own small window, not on the Before page; Go back
  and Clear earlier versions act without it.
- On a locked computer: rows for the next system only when the reader may read
  `prepared` (root); no earlier versions, storage, sizes, dates or Open for the
  running system's software; nothing tells that an update exists but the
  collection's version; "Update" is not offered for software only the next
  system holds.
- Not read: `recipe isinstalled` (the window never asks it).
- The frame's checks were run offscreen against stand-ins; not on a real desktop.

- A real agent for "Ask for software": Codex and Claude Code do not speak the
  lines above as they are. The stand-in writes no file.
- Answering the agent during "Let the agent fix it": that agent is started by
  Recipe itself, which has no way to pass a question on.
- The source commands and `refresh` against the real `recipe`; a source's
  icons; two sources with one name.

- Opening the recipe file itself, the list of sources (Git repositories) and
  choosing between them, asking an agent for a recipe.
- Icons: recipes show a letter until the catalog carries `<name>.svg`.
- Translations: every string goes through `tr()`, no translation exists.
- "Open" starts the first program a recipe installed, not its menu entry.
- Clearing kept versions as its own action, and a notice when updates appear.
- Building it for each distribution against that distribution's Qt, and
  shipping it in the desktop product (the same gap `docs/recab-gui.md` lists).
