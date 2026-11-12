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
its badge, Sources, Storage, then Activity, which the frame owns), the pages in the
middle, a strip under them and a status line. There is no sidebar any more:
the search, the box for the source and the box for the kind of software are
the top of the Browse page. Settings is under Activity and Help under Settings (the
frame's own pages, the same in all three apps; `gui/kit/README.md`). The story opens
Settings (picture `f-settings`), reading through the built levitate-agent with the
stand-in Codex and a settings folder of its own, and Help and the first page (pictures
`h-help*`, `h-firstrun-*`, below).

- The status line says where the recipes are from and when they were read (and
  what is being built); "Read again" sits beside it; the agent is the frame's
  (`Agent: agent` for a stand-in, by its file name).
- "Ask for software" hands the agent the app name "Recipe" and the first line of
  the request as the run's title, so Activity lists it; "Go to it" on Activity
  comes back to the run's page. It also names the person's own recipe folder (the
  "local" source's, made first if it is missing) as `may_write`, so the agent can write
  the recipe although its commands are asked each time; the page says that the agent
  may write in the person's own recipe folder only, unless Settings allowed it more. If the folder
  cannot be made the agent is not started and the status line says so. "Let the agent fix it" is run by `recipe
  --autofix`, not by this window's agent, so it is not in Activity.
- The strip ("The next system holds N changes.") is read with `recab status
  --json` (`RECIPE_GUI_RECAB`, default `recab`) after every read of the recipes.
  It shows on every page; when recab cannot be run there is no strip and no
  error. "Finish and restart…" starts Recab (`LEVITATE_APP_SYSTEM`, default
  `recab-gui`).

## What the audit of 2026-11-11 changed

An audit of the window found seven gaps; each is closed against the stand-in and
none against the real `recipe` (see the end of this section).

- **"Open" starts the program from the profile of the place the software is in.**
  `installed --json` names the store's `profile`; the person's own store's `bin`
  is usually not on `PATH`, and this computer's is another store. The window
  reads the `Exec` line of the package's own menu entry
  (`PROFILE/share/applications/NAME.desktop`, field codes dropped, a bare program
  looked up in `PROFILE/bin` and then on `PATH`) and starts that, as the
  applications menu would; only when no menu entry gives a program it starts the
  first `bin/` file the receipt lists. The program is started with `PROFILE/bin`
  first on its `PATH`. A menu entry with `Terminal=true` is not started: the
  status line says to run it in a terminal. When it cannot be started the status line
  says what is missing and where ("Krita could not be started: krita, which the
  menu entry krita starts, in PROFILE/bin is not there."). Software only the next
  system holds, and the running system of a locked computer (whose list names a
  version and nothing else), has no "Open"; the program's page, Installed and the
  status line say why ("It cannot be opened until then." / "This window cannot open
  it: the running system's list names the software and its version, not its
  programs.").
- **Closing the window while something runs asks first.** An install or update
  (not the agent's run, which the frame already lets go of and Activity follows),
  or a short change, makes `closeEvent` ask "… is still running", with what stops
  ("If you close the window now it is stopped"), what is kept (what is already
  fetched and built), what is true of the software (nothing changed / `X 1.2 stays
  installed and works as before`), that it can be started again, and, in an Update all,
  how many updates still waiting are not started. "Stop it and close" asks the run to
  end (`SIGTERM`, as Stop does) and closes when it has (eight seconds at most, then
  the window's end ends it); "Keep the window open" is the default and the escape.
  The question does not block: a check answers it (`Window::closeQuestion`).
- **Short changes have a page** (`ChangePage` in `pages.*`, in the window's stack): remove, go
  back, keep at a version and let it update again, add or remove a source, fetch
  the lists, clear earlier versions. It shows what is being done, a Stop with what
  stopping means, the command's own output, and the status line says what runs; the
  window is no longer disabled, and a second change or an install asked meanwhile
  is told "One thing at a time". When it works the page is left for the one the person
  was on (drawn from a read made after the change) and the status line says what was
  done; when it did not finish the page stays with Recipe's own message, what is true
  (`fd 10.2.0 is still installed.`), the output, and "Try again" (the same command again)
  and "Back". Recipe's agent repair is an option of `recipe install` only, so these
  offer no "Let the agent fix it". The remove question is asked without blocking too
  (`Window::removeQuestion`). A read asked for while another is going is now done as
  soon as that one ends, so a page that waits for it is not left waiting.
- **The agent is given the run's last lines, and where the log is.** The facts of a
  stopped install carry the last 20 lines of the run's output (each cut at 240
  characters, "N in all") and, when `recipe` printed it, the place of the run's log
  and retained record: the plain line `recipe: diagnostics: DIRECTORY` it prints at
  the start (`tools/recipe`, `invocation_log.rs`, one directory per invocation holding
  `worker.log`, `supervisor-stderr.log`, `authority.json`, `result.json`). The page
  scrolls the facts and keeps its buttons below them. A stopped update names the log
  and keeps the output under "Show the output".
- **The password prompt says what the action covers** (`data/org.levitateos.recipe.policy`):
  install, update, remove, keeping at a version, going back, adding or removing a source,
  and clearing earlier versions. Fetching the lists runs as the person and asks for nothing.
- **A program with no recipe is not a dead end.** Where `recipe list` has no recipe for
  an installed name, Installed says so and what the receipt lists and where it came from
  ("It came from the source oldmirror, which is not among the sources any more"), and has
  "Its receipt", which opens the program's page: name, version, date, size, source and
  commit when recorded, the recipe file the receipt names and whether it is still there,
  the files, "Where it is", and what works from a receipt: keep at the version and let it
  update again, go back (where an earlier version is kept), remove. The Remove question
  names the menu entry from the receipt too. A name nothing carries gets "No recipe named
  X" with a sentence and a way back. `recipe updates` already says "no recipe for it is in
  the collection or the sources any more", which Updates shows under "Not looked up".
- **The story reaches them**: pictures `17a` (the remove question), `17b`–`17d` (the change
  running, stopped, not finished), `17e` (fetching that did not finish), `18a`–`18d` (no
  recipe: Installed, its page, "No recipe named", and going back with no recipe in the
  open-computer window), `19a`/`19b` (closing during an install and during a remove), `8a`
  (what the agent is given), `p8`/`p9` ("Open" for krita in the person's own store, and
  when its program is gone). The stand-in's `krita` has two programs, `kritarunner` first and
  `krita` in its menu entry, so a story can tell which one Open chose; its `fd` is installed
  with no recipe, from a source that is gone, with an earlier version kept.

Not verified: none of this ran against the real `recipe` (the changes ran only against the
stand-in), nor on a real desktop with `pkexec` or a real polkit prompt. Whether the real
`recipe` leaves a store as it was when it is stopped in the middle of `remove`, `go-back`,
`hold`, `source add`, `refresh` or `--clear-earlier` is taken from `tools/recipe/README.md`
("one atomic symlink switch… Failed acquisition or activation preserves the previous
profile") for the first three and is not claimed for the others (their pages say what to look at
instead). Whether the real `recipe` prints `recipe: diagnostics: DIR` into a pipe with
`--machine-events` was read in the source, not run. A `.desktop` file's `DBusActivatable`,
`TryExec` and `Path` are ignored.

`gui/kit` was left alone by this work. `WorkPage` offers only "Back" on a failure and always
reports its title as the agent's note, so short changes got a page of Recipe's own. (The
frame used to let go of Activity's attached runs on every `Close` event of the window, even
when the window then refused to close; it now does so only when the window really closed,
and the live story checks it with a window that refuses.)

## State (2026-11-11)

Since 2026-11-11 the window also has Storage, a program page in the redesign's
layout, hold, and the refused connection named by its numeric address; Installed,
Updates and Sources follow the redesign as far as `recipe` carries what it shows
(the sections below). The story covers each against the stand-in; Storage,
Installed and the held flag were also read against the real `recipe` (tools/recipe
`7f3e836`, the debug binary of 2026-11-11 02:06) on a store made by hand, see
"Reading a hand-made store with the real recipe". Hold and going back through the
window with the real `recipe`, and the refused connection, have not been run: the
real events of the last are compiled, not run.

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
  `QT_QPA_PLATFORM=offscreen` it needs no display. **It refuses to start (exit code 2,
  and a sentence naming what is missing) unless every program it could run and every
  folder it could write is named in the environment**: `RECIPE_GUI_RECIPE`,
  `RECIPE_GUI_AGENT`, `RECIPE_GUI_RECAB`, `RECIPE_GUI_PACKAGE_ROOT`,
  `RECIPE_GUI_ROOT_HELPER` (empty is a name), the helper (`LEVITATE_ACTIVITY_AGENT`)
  with its Codex and Claude Code (`LEVITATE_AGENT_CODEX`, `LEVITATE_AGENT_CLAUDE`) and
  its root helper (`LEVITATE_AGENT_ROOT_HELPER`), and `HOME` (not your own) and the XDG
  folders in scratch space. The desktop notifier is off unless `LEVITATE_AGENT_NOTIFIER`
  names one, and no app is started that `LEVITATE_APP_*` does not name. The story
  runs itself bare to check that.

The story also checks the frame, Activity and the strip, so it needs the
stand-in agent, a recab that speaks the real shape with a scratch state of its
own, and the records Activity shows (stand-in Codex, never a real agent; build
the agent first with `cmake --build .artifacts/agent/build`):

```sh
F=$PWD/gui/recipe/fixtures; A=$PWD/gui/agent/fixtures; S=/tmp/recipe-shots   # all state of the check lives here
rm -rf $S; mkdir -p $S/home $S/store $S/run; chmod 700 $S/run
QT_QPA_PLATFORM=offscreen HOME=$S/home XDG_CONFIG_HOME=$S/home/config XDG_STATE_HOME=$S/home/state \
  XDG_DATA_HOME=$S/home/data XDG_RUNTIME_DIR=$S/run LEVITATE_AGENT_NOTIFIER= \
  LEVITATE_AGENT_CODEX=$A/codex LEVITATE_AGENT_CLAUDE=$A/claude LEVITATE_AGENT_ROOT_HELPER= \
  LEVITATE_ACTIVITY_AGENT=$PWD/.artifacts/agent/build/levitate-agent RECIPE_GUI_RECORDS=$A/records \
  RECIPE_GUI_RECIPE=$F/recipe RECIPE_GUI_AGENT=$F/agent RECIPE_GUI_ROOT_HELPER= \
  RECIPE_GUI_PACKAGE_ROOT=$S/store RECIPE_STANDIN_STATE=$S/state RECIPE_STANDIN_SPEED=0 \
  RECIPE_GUI_RECAB=$PWD/gui/recab/fixtures/recab-real RECAB_STANDIN_STATE=$S/recab RECAB_STANDIN_SPEED=0 \
  .artifacts/recipe-gui/build/recipe-gui --look levitate --shots $S/shots
```

The runs the records fixture leaves going are stopped when the story ends, also when it
fails. After the records, the story starts runs of its own through a second window's
"Ask for software" with the helper as its agent, and checks that they outlive the window
and are found, followed, answered, approved and stopped from Activity (`../kit/livestory.*`;
pictures `a-live-question`, `a-live-approval`, `a-live-follow`), and that the helper was
started with a sandbox that may write in the person's own recipe folder (`may_write`).
After the strip checks it puts the stand-in recab's running system on trial, where recab
still calls the other system "next", and checks that no strip is shown (`s4-no-strip-on-trial`).

The state folders must be new (the `rm -rf` above): the story expects no next system at the start, then
makes one with one `prepare` and one `change`. The story needs
`RECIPE_GUI_PACKAGE_ROOT` (its first window is on one explicit store, so the
Before page has no choice) and says so when it is missing. Pictures: `1-…` to
`14-…` are the flows (`2b-before-install`, `4b-before-update` are the Before
step); `f1`–`f4` Browse (also with a search and a kind), Installed, Updates and
Sources in the frame; `s1`–`s3` the strip and a window with no recab; `a-*`
Activity; `p1`–`p6` the two places (below); `15a`–`15c` the program page (not
installed, installed, installed with an update and an earlier version kept),
`15d`/`15e` Storage before and after clearing, `16a`–`16d` hold (Installed,
Updates, the program page, the refused update of a held program), `17*`–`19*`, `8a`,
`p8`, `p9` the gaps the audit found (the section above). The refused
connection is `4-stopped`. The story ends with `all flows ended where they should`; it
needs `pgrep` (it checks that closing the window ended the stopped install).

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
the folder to start over. Beside each store file it keeps the history of what was
done to it (`installed.json.history.jsonl`, in `recipe history --json`'s fields) and a
package's `held`; `history`, `hold` and `unhold` answer in the real shapes, `installed`
and `updates` carry `held`, `update` refuses a held package in the real sentence, and the
refused connection names `203.0.113.9:443`, a numeric address as the real one does.
`list --json` reports one unreadable recipe once a second source exists, in the real
`problems` shape (`path`, `problem`), so Sources has something to show. Each store has a
profile (`<store>/profile`; the person's own: `<state>/me/profile`), named as `profile` in
`installed`, `storage` and `hold`, holding `bin/NAME` for every program and
`share/applications/NAME.desktop` for every menu entry a receipt lists; each program is a
script that notes `NAME started from PATH` in `started.log` in the state folder, so the story
sees what "Open" started and from where. `RECIPE_STANDIN_STOP_AT` also takes `NAME:remove`,
`NAME:go-back`, `NAME:hold`, `NAME:unhold`, `storage:clear`, `refresh:refresh`,
`source:add` and `source:remove`, and `RECIPE_STANDIN_FAIL` (`remove:NAME`, `go-back:NAME`,
`hold:NAME`, `unhold:NAME`, `clear`, `refresh`, `source-add`, `source-remove`) makes that
short change fail with plain lines and recipe's own message. An install prints
`recipe: diagnostics: DIR` first and hands the prompt file it was given to
`last-autofix-prompt.txt`. `fd` is installed in every store's start state with no recipe in
the catalog, from the source `oldmirror`, and `updates` lists it under `problems`. The stand-in agent writes the app name and title it was given to
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
| `src/pages.*` | Browse (with its search and boxes), a program's page, a run, Installed, Updates, the remove question |
| `src/sources.*` | the Sources page, and asking the agent for a recipe |
| `src/storage.*` | the Storage page: what the earlier versions and what is installed take, per store, and what was done to the stores (the history as sentences) |
| `src/window.*` | the frame, its rail and status, and the flows between pages |
| `src/shots.*` | the stand-in looks and the picture mode |
| `fixtures/` | the stand-in `recipe` with its sample catalog and sources, and the stand-in agent |
| `data/` | the menu entry and the polkit action |

## What the front end asks of `recipe`

Reads, as the desktop user, JSON on stdout with `"schema_version": 1` (the same commands without `--package-root` read the person's own store; `recipe --system installed --json` reads a locked computer, in recab's own shape, see "The two places"):

| Command | Answer | In `recipe` today |
|---|---|---|
| `recipe list --json` | every recipe: `name`, `version`, `description`, `about`, `sources`, `needs`, `deps`, `build_deps`, `hooks`; and the source's name | exists, without `sources` and the source's name. `ABOUT` is passed through unchecked; `SOURCES` is still only the design in `docs/recipe-front-end-format.md` |
| `recipe installed --json` | `packages`: each one's `name`, `version`, `recipe`, `files`, `installed_on` (seconds), `size_bytes` and `kept`: the earlier versions still held, newest first, each with `version`, `generation`, `installed_on`, `size_bytes`; and `profile`, the store's active profile, whose `bin` and `share/applications` "Open" starts programs from; `source` and `commit` where the receipt records them | exists. A receipt written before 2026-11-10 has no `installed_on` or `size_bytes`; the front end then measures the files. It takes programs from the `bin/` files and menu entries from `share/applications/` |
| `recipe updates --json` | `updates`: for each installed recipe that `recipe update` would change: `name`, `source`, `from`, `to`, `reason` (`version`, or `recipe` when the source publishes another recipe file for the same version) and `changes`; `problems`: installed software it could not look up | exists. `changes` lists every field of NEEDS that differs from what was recorded at install (`field`, `before`, `after`, `kind`: `added`, `removed`, `raised`, `lowered`, `changed`), and is `null` for a receipt that recorded nothing; the page then says the two cannot be compared |
| `recipe storage --json` | `active_bytes`, `earlier_generations`, `earlier_bytes` | exists (`commands.rs` `Commands::Storage`). Shown on the Storage page, per store; Installed says only what the earlier versions take |
| `recipe history --json` | `{"schema_version": 1, "history": [...]}`, newest first, each entry `at` (seconds since 1970), `what` (`install`, `update`, `remove`, `go-back`, `clear-earlier`, `hold`, `unhold`), `package`, `from`, `to`, `source` (null where one does not apply; `source` is set only for a recipe of a remote source) | exists: `commands.rs` `Commands::History` prints it, `src/packages/history.rs` lines 22-35 define the entry, `packages.rs` records it after each change. Shown on the Storage page, as sentences with the time |
| `held` in `installed --json` and `updates --json` | a boolean on every package / every listed update | exists: `commands.rs` `Commands::Installed` (`entry["held"] = package.held.into()`) and the `updates` function (`"held": installed.held`). `recipe updates` still lists a held package |

Changes, through the root helper (as the person for software "for me"; with `--system` first and no `--package-root` for a locked computer; see "The two places"), with events on stderr:

| Command | In `recipe` today |
|---|---|
| `recipe install NAME --machine-events` | exists: runs the recipe in Ginux and activates its `package` output |
| `recipe update NAME --machine-events` | exists. Refuses a held package before anything runs, in the sentence `NAME is held, so it is not updated. Release it with: recipe unhold NAME` (`commands.rs` lines 253-257), as the final `failed` event's `reason` |
| `recipe remove NAME --machine-events` | exists, by the receipt. What it removes stays among the earlier versions |
| `recipe go-back NAME --machine-events` | exists: activates the newest earlier version again from the files a generation still holds; the final event only (`changed_package` in `commands.rs` covers it) |
| `recipe storage --clear-earlier` | exists: deletes every earlier version. The window asks first ("Clear earlier versions…" on the Storage page; Installed points there). Recipe cannot clear one program's earlier versions alone, so the remove question no longer offers that. It adds a `clear-earlier` line to the history |
| `recipe hold NAME`, `recipe unhold NAME` (with `--machine-events`) | exists (`commands.rs` `Commands::Hold`/`Unhold`, `packages.rs` `set_held`): `held` on the receipt, a new generation; the final event is `done` or `failed`. Through the root helper where the place needs it. Not for software in a locked computer's system: `system_route()` (`commands.rs` line 1381) hands only install, update, remove, installed and isinstalled to recab, so the window does not offer it there |
| `recipe install NAME --autofix --autofix-prompt-file FILE --machine-events` | exists for `install`: Recipe's agent repair (Codex or Claude), with the front end's prompt appended to its own. Never run against a real agent |
| `recipe update NAME --autofix ...` | does not exist: `update` takes no such options, so the window never sends it. A stopped update offers no repair and says so (the stand-in refuses it as the real `recipe` does) |

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
| `failed` … `refused` | `endpoint` and the `declared` list, for a refused connection | yes, since tools/recipe `7f3e836`, compiled and not run (it needs a failed install under Ginux): `src/lib.rs` lines 107-114 add `refused` to the final `failed` event, `src/needs/report.rs` `refused_connection` (line 270) builds it from the first denied `ginux: audit` line. `endpoint` is the **numeric address** the audit names (`203.0.113.9:443`, or `[2001:db8::1]:443`), never a host name; `declared` is the recipe's NEEDS.network. The page says "the address 203.0.113.9, port 443, which is not on the recipe's list" and that recipe reports an address, not a name; the stand-in sends `203.0.113.9:443` |
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

## Storage

`src/storage.*`, in the rail after Sources. One section of numbers per store a
place has (`recipe storage --json` for each, as the catalog reads it): what the
earlier versions take and in how many generations, with the names of the newest
ones (from `kept` in `installed --json`) and "Clear earlier versions…" (one button,
or one per store when there are two); what is installed takes, per store;
and what was done to the stores (`recipe history --json`, both stores merged,
newest first, "today at 14:05", with "for me"/"for this computer" when there are
two). Only numbers `recipe` reports: no build-cache or download sizes exist in
`recipe`, so none are shown (the page says so). A locked computer's running
system has no store behind it, so it has no row of numbers (it says recipe reports
no size for it); the next system's software likewise.

The sentence for each history entry is made in `historySentence`: "Installed Element
1.12.3, from levitate", "Updated Element from 1.12.3 to 1.12.4", "Removed …", "Went
back to GIMP 3.0.2, from 3.0.4", "Kept GIMP at 3.0.4", "Let GIMP update again",
"Cleared the earlier versions". A kind of change the window does not know is
shown as `recipe` named it. The count of generations is `recipe`'s own
(`earlier_generations`): a hold also makes a generation, so it need not equal the
number of earlier versions listed beside it.

## A program's page

From Browse, Installed or after a run: the name, what it is, the recipe's version,
its kind and source; "Where it is" (its state and version in each place that holds
it: for me, for this computer, in the next system; "Not installed" where it is
not); what can be done to what is installed (keep it at its version or let it update
again, go back, remove, each saying which place when two hold it); what it may
use, and what comes with it (what it adds, what it needs and what only its build
needs, what it keeps, licence, home page); the recipe (its file, its source, "Read
it", and the three steps with the author's notes). Everything is from the catalog
(`list --json`, `installed --json`, `updates --json`). Not on the page, because
`recipe` carries nothing for it: "Repair", "Tell the agent it does not work",
"Copy to your own recipes", "Needed by", and the limits that applied when this
version was built (`installed --json` carries them as `needs`, which the window does
not read yet).

## Hold

"Keep at this version" and "Let it update again" (`recipe hold NAME` / `unhold NAME`,
through the root helper where the place needs it) on Installed and the program page,
and "Let it update again" on Updates. Not offered for software in a locked computer's
system or in the next system, where `recipe` does not support it. A held program:

- is marked on Installed ("kept at this version") and Updates ("Kept at this version"),
  and Browse says "Kept" in place of "Update";
- is listed by `recipe updates` but not counted: the Updates badge and the heading
  count the others, and "Update all" leaves it and says so ("Update all leaves GIMP
  (kept at 3.0.4) alone");
- has no Before step until it is released: the update buttons are not drawn, and
  asking anyway (a stale page, "Try again") says so at the status line;
- if its update is run anyway, `recipe` refuses it with its own sentence, which the
  page shows as it is under "GIMP is kept at version 3.0.4", with only "Back" (there is
  nothing for an agent to fix). The page tells this from the catalog (held when the run
  began) or from the words " is held, so it is not updated" in the reason, which is
  `recipe`'s own wording and would have to change here if `recipe`'s did.

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

## Reading a hand-made store with the real recipe

The Storage, history and held checks of 2026-11-11 were run against the real
`recipe` (tools/recipe `7f3e836`, `target/debug/recipe`) on a store made by hand in
scratch space: two generations under `package-generations/` (`g-…` folders with
`.installed.json` and `.packages/NAME/bin/NAME`), `profile` a symlink to the newer one,
the receipt of the newer naming the older as `previous`. Then `recipe hold tree` and
`recipe go-back ripgrep` with `--package-root` on it made the history and the held
flag. `--try ripgrep --read-only` opens Installed and Storage and prints whether what
they show equals what `recipe storage --json`, `recipe history --json` and `recipe
installed --json` print for that store (`same:` or `DIFFERENT:` lines):

```sh
export HOME=$PWD/.artifacts/recipe-gui/real/home RECIPE_STATE_DIR=$PWD/.artifacts/recipe-gui/real/state
QT_QPA_PLATFORM=offscreen RECIPE_GUI_RECIPE=$PWD/target/debug/recipe RECIPE_GUI_ROOT_HELPER= \
  RECIPE_GUI_AGENT=$PWD/gui/recipe/fixtures/agent RECIPE_GUI_RECAB=/nonexistent/recab \
  RECIPE_GUI_RECIPES_PATH=$PWD/recipes RECIPE_GUI_PACKAGE_ROOT=$PWD/.artifacts/recipe-gui/real/packages \
  .artifacts/recipe-gui/build/recipe-gui --look levitate --shots $PWD/.artifacts/recipe-gui/shots-real --try ripgrep --read-only
```

## Help and the first page

Both are the frame's (`gui/kit/README.md`, "Help" and "The first time"). Help for
Recipe has the ideas (recipes, the sandbox and what a recipe may use, the two places
software goes), what it can do with a link to Browse, Installed, Updates, Sources,
Storage or "Ask for software", the agent, and what to do when an install or update
stopped or the agent does not answer. Its links to the rail's entries need nothing
from the window; "Ask for software" is not an entry, so the window calls
`frame_->addHelpTarget("ask")` and shows `WishPage` on `Frame::helpLink("ask")`.
Help says what the "When a run stops" section above says and no more: "Let the
agent fix it" is named only for an install (`recipe update` takes no `--autofix`
yet), the repair is not approved first, and Activity does not list it. The first
page is shown once per person the first time Recipe opens (never in a picture
run). The story checks both (`../kit/helpstory.*`; pictures `h-help*`, `h-help-went`,
`h-firstrun-*`).

## Reading and reading again

The catalog is read as three commands in a row (`list`, `installed`,
`updates`) without holding the window up. The first time, the page says
"Reading the recipes"; later reads leave the page as it is, and the status line
(the frame's) says where the recipes are from and when they were read. It is read again
after every change, on "Read again" in the status line, on F5 or Ctrl+R, and
when the person comes back to the window after 20 seconds or more. That reads
what is on this computer; it fetches nothing. Fetching the sources' lists is
"Check for updates" on the Updates page and "Fetch again" on the Sources page.
A catalog is five commands now: `list`, `installed`, `updates`, `storage`, `history`, once
for each place that has a store (for me; for this computer when it is not
locked), and `--system installed --json` for a locked computer. `list` is asked
of the computer's store (its sources). When `updates` fails the catalog is not
shown and the page says why; when `storage` or `history` fails only the sizes or the history are missing;
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
remote source is step 3 there, built in `recipe` since 2026-11-10; the window
names it as `SOURCE:NAME` and has not run it against the real `recipe`. When two
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

"Let the agent fix it" runs the same install again with Recipe's agent repair
on (`recipe install NAME --autofix --autofix-prompt-file FILE`) and that text as
its extra prompt. It is offered for a stopped install only: `recipe update` takes
no `--autofix`, so a stopped update says that Recipe's agent repairs a recipe
only while installing it, and offers "Try again" and "Show the output". "Try
again" and "Show the output" stay beside the repair too. What the agent changed is reported when the run ends;
there is no step yet where the person approves the change before it is used.

## Not done

- Not in the frame: the licence and sort boxes of the design (nothing behind
  them). The Installed page does not show its count in the rail.
- Help says nothing of: Update all for programs that ask for nothing more, "Repair" or
  "Tell the agent…", "Copy to your own recipes…", download sizes, sizes of builds,
  installing from a source at an address. It names Remove's own small window and says only that "Clear earlier
  versions…" asks first.
- Not built from the redesign, because `recipe` does not carry it: the search and
  sort of Installed, a library line ("4 libraries are installed because a program
  needs them"), "Look…"/"Update…" buttons on each Updates row, "Newer releases no
  recipe has yet", a source's "added by you on", "Switch off", "Hand to the agent…",
  "Hand both to the agent…", "Repair", "Tell the agent…", "Copy to your own recipes…",
  the sizes of downloads, builds, saved steps and run records on Storage, and
  "which build is running" as the reason something cannot be cleared.
- "Update all" updates every update that is not kept, each after its own Before step;
  the redesign's button updates only those that ask for nothing more.
- Hold, unhold, going back and clearing through the window with the real `recipe`,
  and a real refused connection: not run.
- The two places and the Before step ran against the stand-in only, never a
  real `recipe` install, a real `recab` or a real root helper. A real
  `--system` run's events (and whether `--machine-events` passes through
  `recab change` as `plan`, `fetch` and `done`) are unknown; the run page
  plays what arrives.
- Before: no download size or disk use (`recipe` does not report them); no
  "you can take it out again before the restart" (recab does not say it);
  Remove still asks in its own small window, not on the Before page; Go back
  and Clear earlier versions act without it (Clear asks in a box first); all of them have a
  page of their own while they run, since 2026-11-11.
- On a locked computer: rows for the next system only when the reader may read
  `prepared` (root); no earlier versions, storage, sizes, dates or Open for the
  running system's software; nothing tells that an update exists but the
  collection's version; "Update" is not offered for software only the next
  system holds.
- Not read: `recipe isinstalled` (the window never asks it).
- The frame's checks were run offscreen against stand-ins; not on a real desktop.

- "Ask for software" against a real agent: `levitate-agent` stands between and
  starts Codex, but no recipe request has been run through it against the real
  Codex. The stand-in writes no file.
- A repair of a stopped update: `recipe update` has no `--autofix`.
- Answering the agent during "Let the agent fix it": that agent is started by
  Recipe itself, which has no way to pass a question on.
- The source commands and `refresh` against the real `recipe`; a source's
  icons; two sources with one name.

- Icons: recipes show a letter until the catalog carries `<name>.svg`.
- Translations: every string goes through `tr()`, no translation exists.
- "Open" ignores a menu entry's `DBusActivatable`, `TryExec` and `Path`, and does not start
  `Terminal=true` entries (it says to use a terminal).
- A notice when updates appear.
- Building it for each distribution against that distribution's Qt, and
  shipping it in the desktop product (the same gap `docs/recab-gui.md` lists).
