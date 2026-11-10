# recipe-gui

The graphical front end for Recipe on the desktop images: Plasma on LevitateOS,
LXQt on AcornOS. One window to find software, see what its recipe will do and
may use, install, update, go back a version and remove.

It holds no package rules. It runs `recipe`, reads its JSON and shows it.

Qt 6 Widgets, C++20, CMake. It uses no stylesheet and no colours of its own
beyond the red for "this went wrong", so it takes each desktop's own style.

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
- `RECIPE_GUI_ROOT_HELPER`: what changes are run through (default: `pkexec`;
  empty runs them directly, for the stand-in).
- `RECIPE_GUI_RECIPES_PATH`: the collection to list and install from
  (default: `recipe`'s own choice).
- `RECIPE_GUI_PACKAGE_ROOT`: the store to show and change (default:
  `/var/lib/recipe`, the system's software, which root owns).
- `--look levitate|acorn`: a stand-in look for each desktop, for a machine
  that is neither.
- `--shots FOLDER`: walk the pages, save a picture of each, quit. With
  `QT_QPA_PLATFORM=offscreen` it needs no display:

```sh
QT_QPA_PLATFORM=offscreen RECIPE_GUI_RECIPE=$PWD/gui/recipe/fixtures/recipe \
  RECIPE_GUI_ROOT_HELPER= RECIPE_STANDIN_STATE=/tmp/recipe-shots-state \
  .artifacts/recipe-gui/build/recipe-gui --look acorn --shots /tmp/recipe-shots
```

The stand-in keeps what is "installed" in a state file
(`RECIPE_STANDIN_STATE`, see the top of `fixtures/recipe`); delete it to start
over.

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
| `src/pages.*` | Browse, a recipe's page, a run, Installed, Updates, the remove question |
| `src/sources.*` | the Sources page, and asking the agent for a recipe |
| `src/window.*` | the sidebar and the flows between pages |
| `src/shots.*` | the stand-in looks and the picture mode |
| `fixtures/` | the stand-in `recipe` with its sample catalog and sources, and the stand-in agent |
| `data/` | the menu entry and the polkit action |

## What the front end asks of `recipe`

Reads, as the desktop user, JSON on stdout with `"schema_version": 1`:

| Command | Answer | In `recipe` today |
|---|---|---|
| `recipe list --json` | every recipe: `name`, `version`, `description`, `about`, `sources`, `needs`, `deps`, `build_deps`, `hooks`; and the source's name | exists, without `sources` and the source's name. `ABOUT` is passed through unchecked; `SOURCES` is still only the design in `docs/recipe-front-end-format.md` |
| `recipe installed --json` | `packages`: each one's `name`, `version`, `recipe`, `files`, `installed_on` (seconds), `size_bytes` and `kept`: the earlier versions still held, newest first, each with `version`, `generation`, `installed_on`, `size_bytes` | exists. A receipt written before 2026-11-10 has no `installed_on` or `size_bytes`; the front end then measures the files. It takes programs from the `bin/` files and menu entries from `share/applications/` |
| `recipe updates --json` | `updates`: for each installed recipe that `recipe update` would change: `name`, `source`, `from`, `to`, `reason` (`version`, or `recipe` when the source publishes another recipe file for the same version) and `changes`; `problems`: installed software it could not look up | exists. `changes` lists every field of NEEDS that differs from what was recorded at install (`field`, `before`, `after`, `kind`: `added`, `removed`, `raised`, `lowered`, `changed`), and is `null` for a receipt that recorded nothing; the page then says the two cannot be compared |
| `recipe storage --json` | `active_bytes`, `earlier_generations`, `earlier_bytes` | exists. Shown on the Installed page |

Changes, through the root helper, with events on stderr:

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

## Reading and reading again

The catalog is read as three commands in a row (`list`, `installed`,
`updates`) without holding the window up. The first time, the page says
"Reading the recipes"; later reads leave the page as it is, and the status bar
says where the recipes are from and when they were read. It is read again
after every change, on "Read again" in the status bar, on F5 or Ctrl+R, and
when the person comes back to the window after 20 seconds or more. That reads
what is on this computer; it fetches nothing. Fetching the sources' lists is
"Check for updates" on the Updates page and "Fetch again" on the Sources page.
A catalog is four commands now: `list`, `installed`, `updates`, `storage`. When
`updates` fails the catalog is not shown and the page says why; when `storage`
fails only the sizes are missing.

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
person can change. The agent is `RECIPE_GUI_AGENT` (default `codex`), handed
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
