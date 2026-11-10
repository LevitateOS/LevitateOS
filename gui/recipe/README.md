# recipe-gui

The graphical front end for Recipe on the desktop images: Plasma on LevitateOS,
LXQt on AcornOS. One window to find software, see what its recipe will do and
may use, install, update, go back a version and remove.

It holds no package rules. It runs `recipe`, reads its JSON and shows it.

Qt 6 Widgets, C++20, CMake. It uses no stylesheet and no colours of its own
beyond the red for "this went wrong", so it takes each desktop's own style.

## State (2026-11-10)

Every page works against the stand-in in `fixtures/`. Against the real
`recipe` it reads the collection (`list --json`) and the system's installed
list, shown here with `recipes/ripgrep.rhai`. Installing through it with the
real `recipe` has not been run yet, nor has it run on a real desktop or with
`pkexec`. The tables below say what the front end asks that `recipe` lacks.

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
| `src/window.*` | the sidebar and the flows between pages |
| `src/shots.*` | the stand-in looks and the picture mode |
| `fixtures/` | the stand-in `recipe` and its sample catalog |
| `data/` | the menu entry and the polkit action |

## What the front end asks of `recipe`

Reads, as the desktop user, JSON on stdout with `"schema_version": 1`:

| Command | Answer | In `recipe` today |
|---|---|---|
| `recipe list --json` | every recipe: `name`, `version`, `description`, `about`, `sources`, `needs`, `deps`, `build_deps`, `hooks`; and the source's name | exists, without `sources` and the source's name. `ABOUT` is passed through unchecked; `SOURCES` is still only the design in `docs/recipe-front-end-format.md` |
| `recipe installed --json` | `packages`: each one's `name`, `version`, `recipe` and `files`, as today; and asked for: `installed_on`, `size_bytes`, `kept`, `kept_bytes` | exists with the first four fields. The front end takes programs from the `bin/` files and menu entries from `share/applications/` |
| `recipe updates --json` | for each installed recipe with a newer version: from, to, and what changed in what it may use | no such command. Without it the front end counts an installed version that differs from the collection's as an update, and says the change in what the recipe may use is not known |

Changes, through the root helper, with events on stderr:

| Command | In `recipe` today |
|---|---|
| `recipe install NAME --machine-events` | exists: runs the recipe in Ginux and activates its `package` output |
| `recipe update NAME --machine-events` | exists |
| `recipe remove NAME [--clear-kept] --machine-events` | exists, by the receipt; no `--clear-kept` |
| `recipe go-back NAME --machine-events` | no such command; earlier profile generations are kept |
| `recipe install NAME --autofix --autofix-prompt-file FILE --machine-events` | exists for `install`: Recipe's agent repair (Codex or Claude), with the front end's prompt appended to its own. Never run against a real agent |
| `recipe update NAME --autofix --autofix-prompt-file FILE --machine-events` | `update` takes no such options yet |

Events are one JSON object per line. The front end reads these; any other
line is shown as output.

| Event | Fields |
|---|---|
| `plan` | `recipes`: each `name` and its `steps`, in the order they will run |
| `step` | `recipe`, `hook`, `status` (`running`, `done`, `skipped`, `failed`). Today's `recipe-hook` event is read the same way, and is the only event `recipe` sends today |
| `fetch` | `recipe`, `source`, `bytes`, `total` |
| `progress` | `recipe`, `done`, `total` |
| `say` | `recipe`, `text` |
| `output` | `recipe`, `line` |
| `failed` | `recipe`, `hook`, `reason`, and for a refused connection `refused`: `endpoint` and the `declared` list |
| `agent` | `recipe`, `summary`, `files`: the agent changed the recipe during this run. Asked for; `recipe` sends nothing like it today |
| `done` | `recipe` |

Without a `plan`, the front end adds a row of three steps for each recipe as
its first step arrives.

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

- Opening the recipe file itself, the list of sources (Git repositories) and
  choosing between them, asking an agent for a recipe.
- Icons: recipes show a letter until the catalog carries `<name>.svg`.
- Translations: every string goes through `tr()`, no translation exists.
- "Open" starts the first program a recipe installed, not its menu entry.
- Clearing kept versions as its own action, and a notice when updates appear.
- Building it for each distribution against that distribution's Qt, and
  shipping it in the desktop product (the same gap `docs/recab-gui.md` lists).
