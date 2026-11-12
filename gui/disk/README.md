# disk-gui

The third desktop app, titled "Recdirstat". It sits in the shared window frame
(`gui/kit/frame.*`, see its README): the rail with the app name, Recdirstat's four
entries, Activity, Settings and Help; the pages; the strip about the next system; the
status line. It does these things, each its own rail entry (it opens on the first):

- **The disk**: how full the place for the person's files is, what could be
  freed and where it is, every partition of the disk, whose room it is.
- **Browse**: the folder being looked at, with the map beside it.
- **Mess**: where the mess is hiding, as a list to tick, handed to the agent.
- **The two systems**: it compares the running system with the other one (the
  A and B root partitions) and says how alike they are and what differs.

The agent does the clearing, and it does not delete: it **sets places aside**
(moves them into a holding folder on the same disk). This window brings places
back, or lets the whole holding folder go after asking. That is the only thing
the window itself ever deletes. The premise is the same as in `gui/recipe` and
`gui/recab`: an agent does the work, and is given the issue, the facts and the
expected result.

Qt 6 Widgets, C++20, CMake. Shared widgets are in `gui/kit`.

## State (2026-11-11)

**The finding is real; the agent is a stand-in.** The scanner and the rules
have run on a real home folder (172 GiB read in about nine seconds, on a warm
disk). **Nothing has been handed to a real agent.** The hand-over goes through
`levitate-agent`, which starts the agent and moves the places itself: the agent
only decides (`SET-ASIDE:`, `KEEP:`, `CANNOT:` lines, `ASK:` for the places
marked to ask first), the request carries `set_aside` (the holding folder, the
places, the ones to ask about first), and the helper moves only places on that
list, so the agent needs no right to write and nobody is asked about a move. The
stories run that path with the helper and the stand-in Codex
(`gui/agent/fixtures/codex`, story `disk`); `fixtures/agent` stays as a stand-in
for another program set as the agent by its path, which gets the same request and
must move the places itself. Neither a real Codex nor a real Claude Code has
followed the convention: whether one writes the lines as asked, and asks about
the marked places by naming their full path, is unchecked. The pages for The
disk, Browse, Mess and what was set aside were built against stand-ins for
`lsblk`, `recipe storage` and the agent (below) and never on a real desktop.

**recab has the comparison now; no real pair of systems has been compared.**
`recab compare` (tools/recab `760a60b`) answers in the shape written down
under "What it asks of recab". It was run on two sample trees with its
`--roots` form, and this window drew that answer; the two-slot form, which
mounts the other partition, has never run on an A/B system. The stories still
use `fixtures/recab`, which plays a system from before, a next system and two
identical ones without a disk.

## What a person sees

### The disk (the first page)

- **The heading** is true of what `statvfs` says for the disk the home folder is
  on: "The place for your files is nearly full" (under 5 GiB or a tenth free) or
  "has room", with how much is left of how much. Never counted from the list.
- **What can be freed**, rows with a size, a sentence and a button:
  *Known leftovers* (the Mess list: the same figure in the same number of
  places; the button goes to Mess); *Kept by Recipe* (`earlier_bytes` from
  `recipe storage --json`, asked as the person; the button starts the Recipe app
  through the frame's `startApp`; no row when Recipe cannot be asked or says
  zero); *Set aside by the agent* (the holding folder's size, while it holds
  anything; the button opens the page for what was set aside). There is no row
  for logs and crash records: they cannot be measured without root.
- **Every partition of this disk**, from `lsblk --json --bytes` (`DISK_GUI_LSBLK`)
  for the disk that holds the home folder (found by its device, else by what is
  mounted at the home folder's filesystem). Which partition is the running
  system and which the other comes from recab's status (`slots[].device`, or the
  stand-in's `running.device`/`other.device`). Used and free are `statvfs` at the
  mount point, so a partition that is not mounted (the other system's) says "not
  mounted, so not read"; when a comparison was kept, the other system's row adds
  what its files took then. Start-up files, swap and the home partition are named
  by what lsblk says (partition type, filesystem, mount point); anything else is
  listed by its label, never guessed.
- **Whose room it is**: a bar and a legend from the scan, in kinds the scanner
  really tells apart: files in the home folder, what programs keep (hidden
  folders of the home folder), known leftovers, what the agent set aside, other
  folders read. They add up to what the scan read. How many folders could not
  be read is said; their size is not known and is not guessed. Where lengths and
  room taken are counted alike, it also says how much of what the disk counts as
  used lies outside the folders read.
- "Look again" reads the scan, the partitions, Recipe's storage and the holding
  folder again.

### Browse

The folder being looked at: a path in which every part can be clicked, "Up", a
search that filters this folder; the folders as rows (size, share bar) beside
the map; the views "Folders", "Biggest files" (files of 1 MiB or more, which is
all the scanner records one by one) and "Not used for a year" (files of 1 MiB or
more not opened or changed for a year, from the later of last read and last
change, as the scanner records). The panel for the chosen place says what the
scan recorded (size, files, last used), what the rules say it is, and offers:
"Go in", "Open in Files" (`QDesktopServices`, or the program in `DISK_GUI_FILES`),
"Copy the path", "Add to the list" (put a place the rules did not name on the
list as "Picked by you"), "Take off the list", and "Never suggest this" (the
place leaves the list at once and is kept in `<config>/levitate/disk/never-suggest.json`;
the rules never name it or anything inside it at any later look; Mess lists these
with "Suggest it again"). A second look keeps the person where they were.

### Mess

The list in the mock-up's layout: kinds with their one-line meaning, each place
with its size, when it was last used (the latest read or change of anything
inside it) and what the agent does with it (`clears`, `asks first`, `finds out,
asks`); a panel for the chosen place ("Show in Browse", "Open in Files", "Take
off the list", "Never suggest this"); "What you told it" (where it looked, what
is never suggested); "Tick only what clears without asking"; and under it how
many places are ticked, how much that is and how much of it goes without
asking, with "Hand it to the agent…". The map moved to Browse; so did "Add to
the list".

### The hand-over, the work, and what was set aside

1. **The hand-over** is the layout of the shared "Before it happens" page
   (`gui/kit/before.*`, as Recipe uses it): the title as a question ("Hand these
   12 places to the agent?"), "Nothing has started...", "What happens" in plain
   sentences, "Cancel" and "Hand it over". The sentences are built from the
   ticked list and say what the agent is asked to do (decide; it is told to move
   nothing itself), that the helper moves what it names, only places on the list,
   into the holding folder (a place on another disk stays where it is) and
   nothing is gone until the person lets it go. They depend on which agent is
   chosen, which only the helper knows: the window asks it (`--settings`, as
   Settings does, without holding the window up) and draws the page when the answer
   is in (`AgentChoice`, `Window::showHandPage`). For Codex or Claude Code the
   sentences are those above, and when Settings let the agent run commands they say
   that only the instruction keeps it from changing files itself. For a program set
   as the agent by its path they say that program is handed the request as it is,
   moves the places itself and Recdirstat only shows what it reports and then reads the
   holding folder, that whether it asks first is its own matter, and the issue in
   the request asks it to move the places itself. When the helper cannot be asked
   the page says so and says what is so for Codex or Claude Code, which places the agent has to ask
   about first and that the helper does not move one of them before it did (or
   that no place is asked about, the hand-over being the approval), that it works
   as the person and a command as administrator runs only if approved, and how much
   room is free once everything is let go (against what the disk has free now).
   Recipe's own folders are never handed over (they cannot be ticked; see
   "What counts as mess"). Under the sentences is the request the agent is given:
   the issue, the facts (one per ticked place: path, size, why it can go, and
   "Ask me first." where it applies) and the expected result, which the person
   can change. The convention itself (the lines, the ask-first places, the list
   of approved places) is not in the request: the helper tells the agent. The shared page has no part a
   person can edit, so this page (`HandPage`) is that layout with the request in
   it, instead of a Before page followed by a second page. Escape and Cancel
   hand nothing over. "Ask the agent what differs" (below) uses the same page.
2. **The agent works.** Before it names anything marked "asks first" it asks,
   and the question is answered in the window; the helper refuses to move such a
   place that was not asked about in the run. Nothing else is asked: the list was
   approved on the hand-over page.
3. **What was set aside** (the page after the run, under Mess; also from The
   disk): a heading with how much is held; "Free before" (measured when the
   request was handed over), "Free now" (measured now) and "Free once you let it
   go" (free now plus what the holding folder takes), all `statvfs` and a
   measurement of the folder, not counted from the list; one row per place:
   *Set aside* (and, when it asked, what the person answered), *Kept* (and why),
   *Left alone* (only for a program that says `left` itself), *Could not* (and why, with "Try again", which hands
   just that place over again), *Brought back*, *Let go*; a place held that was
   not on the list is flagged; a place the agent said it set aside that is not
   in the holding folder is flagged. A place the person said to keep offers
   "Never suggest".
4. **"Bring back"** (one place) and **"Bring everything back"**: the window moves
   them back to where they were, and refuses, saying why, when something is
   there now. "Bring everything back" first shows the shared Before page: how
   many places and how much move, that nothing is deleted, that a place with
   something at its old place stays held, that the room does not change (when
   the holding folder is on the same disk). Cancel moves nothing.
5. **"Let it all go, free X"**: first shows the shared Before page (it replaces
   the question dialog): the holding folder that is deleted, how many places and
   how much, that it cannot be undone and the places cannot be brought back, how
   much room the disk then has, and that nothing outside the holding folder is
   deleted. "Let it all go" deletes the holding folder; Cancel keeps everything. Nothing lets it go by
   itself: not after some days, not at all unless the person presses it.

The rail entry "Mess" carries a badge: how many places of mess are on the list.
A hand-over, the agent's work and the page for what was set aside are steps of
the entry they came from and keep it marked; while the agent works, choosing
"Mess" shows the work. Activity, from the frame, lists the agent's runs of all
three apps; a run started here is listed as "Recdirstat", under the first line of its
issue, and "Go to it" brings the person back to the work page. The strip, "The
next system holds N changes." with "Finish and restart…", is read from `recab
status --json` (`DISK_GUI_RECAB`) and shown on every page; when recab cannot be
read or answers in a shape the strip does not know, there is no strip and no
error. The button and "Open Recab…" start the Recab app the same way
(`LEVITATE_APP_SYSTEM`, else `recab-gui`; the frame starts it).

The status line says when the disk was looked at and that only "Let it all go"
deletes anything from here. "Read again" in the status line, F5 and Ctrl+R look
at the disk again on The disk, Browse and Mess, read what was set aside again on
its page, and ask recab again on the two systems, and read Activity and the
strip again. Coming back to the window after a while asks recab again; the disk
is only looked at again when asked, since that takes a while and drops what was
ticked.

On the fourth entry:

1. **What is there.** On opening, recab is asked what the two systems are
   (quick, no password): the page names both and shows two empty frames. If a
   comparison was kept from before, it is shown instead, with when it was
   made; when either system has changed since, it says so and offers to
   compare again. "Open Recab…" starts the Recab window, where changing,
   updating and going back are done.
2. **Compare.** "Compare" first shows the shared Before page: both systems are
   read from start to end, neither is changed, it takes a few minutes, whether the
   administrator's password is asked (it follows the root helper this window
   really uses) and the command that will run. Cancel compares nothing. Then both
   systems are read from start to end, which takes minutes, and with a root
   helper the administrator's password is asked; without one it runs as the person
   and the pages say so (the page before comparing says "no password is asked",
   the frame while it waits says "Waiting"). The page stays: each frame fills as
   its side is read, and Stop ends it.
3. **How alike they are.** A headline ("The two systems are identical", or
   "99.9% the same", by room taken), how many files differ, and what the other
   system is: a copy, the system from before the last change, or the next
   system with a change that is not started.
4. **The two systems as blocks, side by side.** Two maps, this system left
   and the other right, drawn from one layout: every folder is a block in the
   same place on both sides, so two systems that are the same look the same.
   A block that holds files that differ is coloured, with the number of files
   on it: only on this side, different content, or same content with other
   settings. A block where only the other side has something more is outlined
   with dashes. A block that differs is drawn at least big enough to see,
   however little room the difference takes. A click on a block marks it on
   both sides and says whether it is the same in both.
5. **The list** shows every difference by folder. Files that differ by design
   (each system names its own partition) are counted apart.
6. **Ask the agent what differs.** The same page as the hand-over says first
   what the agent is given and that it is told to change nothing, with the
   request below it; Cancel asks nothing. The differences go to the agent as issue,
   facts and expected result (through the helper, with the agent the person chose;
   the request carries no list of places). The agent changes nothing; what it
   answers is shown above the list.

## Help and the first page

Both are the frame's (`gui/kit/README.md`, "Help" and "The first time"); the window
adds nothing for them: every link of Help goes to a rail entry (The disk, Browse, Mess,
The two systems), Activity or Settings. Help for Recdirstat has the ideas (mess, setting
aside, the two systems), what it can do (see how full the disk is, find what takes the
room, clear mess, bring a place back or let it all go through "Set aside by the agent"
on The disk, compare the two systems), the agent (asked to set aside by moving, what
the page shows read back from the holding folder), and what to do when a place could
not be set aside or the agent does not answer. The first page comes once per person
the first time Recdirstat opens (never in a picture run). Both stories check them
(`../kit/helpstory.*`; pictures `h-help*`, `h-help-went`, `h-firstrun-*`).

## The set-aside convention

What the request tells the agent, and what the window reads back. One holding
folder per request, on the same disk as the home folder:

```
~/.local/share/levitate/set-aside/<yyyyMMdd-HHmmss>/
    manifest.json      {"version": 1, "entries": [{"from": "/home/me/.cache/x", "held": "home/me/.cache/x", "bytes": 123}]}
    home/me/.cache/x   the place itself, moved here under its own full path
```

The request's first line carries `"set_aside": {"holding": "/abs/holding/folder",
"places": [...], "ask_first": [...]}` (the places the person ticked, and those of
them marked "Ask me first"; `Window::send` writes the line, since the shared `Run`
has no field for it). `levitate-agent` then does what a program speaking the
protocol itself would do: the agent only decides, with `SET-ASIDE: path`, `KEEP:
path | why` and `CANNOT: path | why` lines (and `ASK:` for the marked places), and
the helper moves each place it names, only if it is on the list, into the holding
folder keeping its full path beneath it, writes the manifest above, and sends
`{"event": "place", "path", "outcome": "set-aside" | "kept" | "could-not", "why",
"bytes"}` (and `asked`, `answer` for a place the person was asked about). The
window also reads `result` and `reason` in a `place` event, which is what a
program speaking the protocol itself (`fixtures/agent`) says. Recipe's own folders
are not in the list. What the page shows as set aside is what the manifests say
and the holding folder really holds, measured; the events only say what happened
to the places that are not held.

**Moving and deleting are confined.** The window moves a place only from a
holding folder (a direct child of `set-aside` whose name is a time) to the
`from` its manifest records, and only when the record agrees with itself: `from`
is a plain full path inside the home folder or a folder the window reads (and
not inside the holding folder), `held` is a plain relative path with no `..`,
`held` is exactly `from` without its first `/`, and the folders from the
holding folder down to the place are real folders. It refuses when anything is at
the destination (`renameat2` with `RENAME_NOREPLACE`, so nothing is ever
replaced) or when a folder on the way is a link. "Let it all go" deletes only the
direct children of `set-aside` that are holding folders (not links), without
following links and without leaving the disk, and then removes `set-aside` if it
is empty. `src/setaside.*` is all of it.

## What counts as mess

The rules are in `src/mess.cpp`. A place smaller than 20 MiB is not named. A place the person said never to suggest, and anything inside it, is not named; neither is the holding folder. Recipe's kept earlier versions (`package-generations`) are no longer a line here: they are the "Kept by Recipe" row on The disk, from Recipe's own number, so they are not counted twice.

| Kind | What | Agent |
|---|---|---|
| Caches | each folder in `~/.cache`; any folder with a `CACHEDIR.TAG` | clears (sets aside) |
| Build leftovers | `target` beside a `Cargo.toml`, and any tagged folder that looks like Rust output; a folder with a `CMakeCache.txt`; `node_modules` beside a `package.json` | clears |
| Recipe's own | `~/.cache/recipe`, `~/.local/state/recipe`, `/var/lib/recipe/state` | left alone: listed, nothing to tick, never handed over; they belong to Recipe, which has no command to clear them yet |
| The trash | `~/.local/share/Trash` | clears |
| Old downloads | archives, images and packages in `~/Downloads` not opened for 90 days | asks first |
| Large and untouched | a file of 256 MiB or more, not opened or changed for a year | asks first |
| Picked by you | whatever the person added from the map | finds out, asks first |

Build rules do not apply inside `~/.local/share`, `~/.config` and `~/.var`:
what a program keeps there for itself is not a leftover.

The scanner (`src/scan.cpp`) only reads. It does not follow links, does not
leave the filesystem it started on, counts a file with several names once, and
counts the room a file takes on disk, not its length.

## Running it from a checkout

```sh
cmake -S gui/disk -B .artifacts/disk-gui/build -G Ninja
cmake --build .artifacts/disk-gui/build
```

On a sample home folder, with the stand-ins (never the real `codex`, `claude`,
`pkexec` or `sudo`: every run names its stand-ins, and the stand-in agent moves
only inside `DISK_STANDIN_ROOT`):

```sh
A=$PWD/.artifacts/disk-gui; F=$PWD/gui/disk/fixtures
$F/make-sample $A/sample
DISK_GUI_ROOTS=$A/sample DISK_GUI_HOME=$A/sample DISK_GUI_APPARENT=1 \
  DISK_GUI_AGENT=$F/agent DISK_STANDIN_ROOT=$A/sample \
  DISK_GUI_LSBLK=$F/lsblk DISK_GUI_RECIPE=$F/recipe \
  $A/build/disk-gui --look levitate
```

For `--shots ... --hand-over` the hand-over goes through the built helper, so the run
must also name `LEVITATE_ACTIVITY_AGENT=<the built levitate-agent>` and
`LEVITATE_AGENT_CODEX=gui/agent/fixtures/codex` (with the notifier and root helper
emptied, as `gui/agent/fixtures/story` does); without them the run stops before handing
anything over, since the helper would start the real `codex`.

Window 1100 x 700 the first time, then the size it had when it closed
(`rememberWindowSize` in `gui/kit`; the picture modes always use 1100 x 700).

### The stories

`--shots FOLDER` looks, saves pictures and checks expectations; with
`--hand-over` it also hands the list to the agent and goes on to what was set
aside and the comparison of the two systems. Both end with "all as expected" or
say "not as expected: ..." for each thing that failed, and exit non-zero.
They need no display, and no real agent, recab, recipe, lsblk or administrator.
**`--shots` refuses to start (exit code 2, and a sentence naming what is missing)
unless every program it could run and every folder it could write is named in the
environment**: `DISK_GUI_AGENT`, `DISK_GUI_RECAB`, `DISK_GUI_RECIPE`, `DISK_GUI_FILES`,
`DISK_GUI_LSBLK`, `DISK_GUI_ROOTS`, `DISK_GUI_HOME`, `DISK_GUI_ROOT_HELPER` (empty is a
name), the helper (`LEVITATE_ACTIVITY_AGENT`) with its Codex and Claude Code
(`LEVITATE_AGENT_CODEX`, `LEVITATE_AGENT_CLAUDE`) and its root helper
(`LEVITATE_AGENT_ROOT_HELPER`), and `HOME` (not your own) and the XDG folders in scratch
space (the command below names them all). The desktop notifier is off unless
`LEVITATE_AGENT_NOTIFIER` names one. The story runs itself bare to check that. The runs
the records fixture leaves going are stopped when the story ends, also when it fails.
Without `--hand-over` it then starts runs of its own through a second window's hand-over
with the helper as its agent, and checks that they outlive the window and are found,
followed, answered, approved and stopped from Activity (`../kit/livestory.*`; pictures
`a3-live-question`, `a3-live-approval`, `a3-live-follow`).

Both stories also check every Before page in the shared layout (the hand-over,
comparing, and, with `--hand-over`, bringing everything back, letting it all go
and asking what differs): what it says, that Cancel leaves everything as it was
(nothing handed over or moved, no folder made, nothing compared or deleted), and
that its button does it. The hand-over also checks that the expected result can be
edited and the edit reaches the request. Pictures `3-hand-over` (also in the story
without `--hand-over`), `5a-before-compare`, `4c2-before-bring-back`,
`4d0-before-let-go` and `7-ask`.

Both stories (pictures `1-disk`, `2a`..`2f` Browse, `3a-mess`, `3c-mess-told`,
`3-picked`, `3-hand-over`, `1b-settings`, `s1`/`s2` the strip, `a1`/`a2`
Activity):

- the window is 1100 x 700, and **every picture** is taken at that size (no page
  may make the window wider or taller); the rail offers The disk, Browse, Mess,
  The two systems, Activity, Settings and Help and nothing else; the window opens on
  The disk; the Mess badge equals the number of places listed, also after a
  second look; the status line says when the disk was looked at and that only
  "Let it all go" deletes; the strip; "Go to it"; Settings opens.
- **The disk**: the free space and total it names are what `statvfs` says for the
  sample's disk (within 256 MiB, because the disk is shared with other programs
  while the check runs), and the heading follows them; the Known leftovers row is
  the Mess list's figure in the same number of places; Kept by Recipe is what the
  `recipe` stand-in says, and its button starts the Recipe stand-in app; there is
  no row for logs; the partitions are the `lsblk` stand-in's (right disk, the
  other disk's partition left out, the running system and the other one named
  from recab's devices, the home partition measured, the unmounted ones "not
  read", sizes); whose room it is adds up to exactly what the scan read; "Look
  again" reads again.
- **Browse**: starts in the home folder; going in and up; the path; search keeps
  what matches and says when nothing does; "Biggest files" starts with the
  biggest file and runs big to small; "Not used for a year" lists exactly the
  one file that old; "Copy the path" (the clipboard), "Open in Files" (a stand-in
  program is handed the folder); the panel's buttons follow whether the place is on the list;
  "Add to the list" and "Take off the list" move the badge; "Never suggest this"
  removes the place at once, writes it to the config file, and a second look
  does not bring it back; Mess says what you told it; "Suggest it again" puts it
  back and clears the file.
- **Mess**: heading, kinds' words, columns, the panel and every action, "Show in
  Browse" lands on the place, "Tick only what clears without asking" leaves only
  places that clear.

With `--hand-over` as well: the agent asks; it **sets places aside** (every
place handed over is in the holding folder under its own path or still where it
was, never both); the page for what was set aside shows, under Mess, saying
nothing is gone yet and nothing about days; "Free before" is what was measured
when it was handed over, "Free now" what `statvfs` says now, "Free once you let it
go" is free now plus the holding folder; a place is listed as set aside, one the
person kept (with why, and what they answered), one that could not be done (with
why and "Try again"), Recipe's as left alone; The disk lists "Set aside by the
agent" with the same size and a button to the page; confinement (a record from
outside the folders looked at, a held path that climbs out, a record whose held
path is not under the path it came from, a folder on the way that is a link: all
refused with their reason, nothing outside touched); bringing back refuses when
something is there now and leaves what is there; one place brought back is there
again and no longer held; "Let it all go" asks first on the Before page (and says
how much), Cancel deletes nothing, then the holding folder is gone and **a sentinel file beside
it, a file of the home folder, and a file in a folder outside the sample are all
still there**; the page says what was let go; The disk lists no holding folder
any more; then the two systems are compared and the agent is asked what differs.

Without `--hand-over` the story stops after the kept comparison, the strip and
Activity. Both write the "never suggest" list of the person (the story puts it
back at the end), so run them with `HOME` and the `XDG_*` folders in scratch
space.

```sh
A=$PWD/.artifacts/disk-gui; F=$PWD/gui/disk/fixtures; R=$PWD/gui/recab/fixtures
S=/tmp/disk-shots   # all state of the check lives here
mkdir -p $S/home; $F/make-sample $S/sample
printf '#!/bin/sh\ntouch "$DISK_STANDIN_SYSTEM_MARK"\n' > $S/system-app; chmod +x $S/system-app
printf '#!/bin/sh\ntouch "$DISK_STANDIN_RECIPE_MARK"\n' > $S/recipe-app; chmod +x $S/recipe-app
printf '#!/bin/sh\nprintf %%s "$1" > "$DISK_STANDIN_FILES_MARK"\n' > $S/files-app; chmod +x $S/files-app
# The strip reads the real recab's shape: a next system with one change.
export RECAB_STANDIN_STATE=$S/recab-state RECAB_STANDIN_SPEED=0
$R/recab-real --json prepare
$R/recab-real --json change install htop --preparation p2 --revision 1
mkdir -p $S/run; chmod 700 $S/run
A_=$PWD/gui/agent/fixtures
export HOME=$S/home XDG_CONFIG_HOME=$S/home/config XDG_STATE_HOME=$S/home/state \
  XDG_CACHE_HOME=$S/home/cache XDG_DATA_HOME=$S/home/share XDG_RUNTIME_DIR=$S/run \
  LEVITATE_AGENT_NOTIFIER= LEVITATE_AGENT_CODEX=$A_/codex LEVITATE_AGENT_CLAUDE=$A_/claude \
  LEVITATE_AGENT_ROOT_HELPER=
QT_QPA_PLATFORM=offscreen DISK_GUI_ROOTS=$S/sample DISK_GUI_HOME=$S/sample DISK_GUI_APPARENT=1 \
  DISK_GUI_AGENT=$F/agent DISK_GUI_RECAB=$F/recab DISK_GUI_ROOT_HELPER= \
  DISK_STANDIN_ROOT=$S/sample DISK_STANDIN_SPEED=0 DISK_GUI_KEPT=$S/kept.json \
  LEVITATE_APP_SYSTEM=$S/system-app DISK_STANDIN_SYSTEM_MARK=$S/system-app-started \
  DISK_STANDIN_STRIP_RECAB=$R/recab-real \
  DISK_GUI_LSBLK=$F/lsblk DISK_STANDIN_LSBLK_MOUNT=$(df --output=target $S/sample | tail -1) \
  DISK_STANDIN_LSBLK_HOME_SIZE=$(df -B1 --output=size $S/sample | tail -1 | tr -d ' ') \
  DISK_GUI_RECIPE=$F/recipe LEVITATE_APP_RECIPE=$S/recipe-app DISK_STANDIN_RECIPE_MARK=$S/recipe-app-started \
  DISK_GUI_FILES=$S/files-app DISK_STANDIN_FILES_MARK=$S/files-app-asked \
  DISK_STANDIN_COULD_NOT=/Projects/viewer/build \
  RECAB_GUI_RECORDS=$PWD/gui/agent/fixtures/records \
  LEVITATE_ACTIVITY_AGENT=$PWD/.artifacts/agent/build/levitate-agent \
  $A/build/disk-gui --look levitate --shots $S/pics --hand-over
```

The comparison keeps using the Recdirstat stand-in `$F/recab`, which answers `status`
in the older shape the strip cannot read (schema 3, with the partitions
`/dev/vda2` and `/dev/vda3` the `lsblk` stand-in knows); that is also the check
for "no strip". `DISK_STANDIN_STRIP_RECAB` is `recab-real`, used only for the
strip (`prepare` printed `p2` as the preparation's id in a fresh state).
Activity needs `RECAB_GUI_RECORDS` and `LEVITATE_ACTIVITY_AGENT` (build the agent
first: `cmake --build .artifacts/agent/build`; the story gives the helper the runs' sockets
through `XDG_RUNTIME_DIR` itself); without them, or without the
strip's two variables, the story says what is missing and fails. Pictures with
`--hand-over`: `4-after` (what was set aside), `4a-disk-with-set-aside`,
`4b-after-refused`, `4c-after-one-back`, `4c2-before-bring-back`, `4d0-before-let-go`, `4d-after-let-go`, `4e-disk-after-let-go`,
then `5-systems` to `8-explained` (asking what differs goes through the helper with the
stand-in Codex, which only reads; `8b-explained-by-other-program` is the same request handed
to the `agent` stand-in as another program). The hand-over page in the other three
cases: `3d-hand-over-commands-allowed`, `3e-hand-over-other-program`,
`3f-hand-over-unknown-agent`.

Without `--hand-over` and without the settings it reads the real home folder,
hands nothing to anyone and compares nothing.

| Setting | What | Default |
|---|---|---|
| `DISK_GUI_ROOTS` | the folders to read, separated by `:` | the home folder, and `/var/lib/recipe` |
| `DISK_GUI_HOME` | which folder the home rules apply to; the holding folder is below it | the home folder |
| `DISK_GUI_AGENT` | the agent program | `levitate-agent`, which starts Codex (`gui/agent/README.md`) |
| `DISK_GUI_APPARENT` | `1`: count each file's length (the sample is made of empty-but-long files) | off |
| `DISK_GUI_RECAB` | the program asked to compare the two systems | `recab` |
| `DISK_GUI_ROOT_HELPER` | what runs it as root; empty: run it as the person | `pkexec` |
| `DISK_GUI_KEPT` | the file the last comparison is kept in | `comparison.json` in the person's cache |
| `DISK_GUI_OS_RELEASE` | where the running system's name is read | `/etc/os-release` |
| `DISK_GUI_LSBLK` | the program that lists the disk's partitions, asked `--json --bytes -o ...` | `lsblk` |
| `DISK_GUI_RECIPE` | the program asked `storage --json` for what Recipe keeps | `recipe` |
| `DISK_GUI_FILES` | a program started with a folder instead of the desktop's file manager (for checks) | the desktop's own |

The stand-ins, all in `fixtures/`:

- `agent` stands for another program that speaks the protocol itself (the helper,
  `levitate-agent`, is the route a person takes; see `gui/agent`). It moves a place
  only when it lies inside `DISK_STANDIN_ROOT`, into the holding folder the
  request's `set_aside` names, and writes the manifest; it leaves what is marked
  "Ask me first" when the answer is "Keep it". `DISK_STANDIN_COULD_NOT=TEXT` makes
  it fail, as a program holding files open would, on every place whose path
  contains TEXT. It refuses a request that names no holding folder, and answers
  `--settings` as the helper does when a program is chosen by its path (its own path
  as the agent), so that the window words its page for another program. The hand-over
  story runs it once after the main run, and once after the explanation, as "the other
  program also works"; with it as the agent the window is started in the story without
  `--hand-over` too, and the hand-over page then says the program moves the places.
  The main hand-over of the stories goes through the built `levitate-agent`
  (`LEVITATE_ACTIVITY_AGENT`) with `gui/agent/fixtures/codex` as Codex
  (`LEVITATE_AGENT_CODEX`; the story sets `LEVITATE_STANDIN_STORY=disk` and the
  helper's `HOME` to the sample for that run only): the stand-in decides about each
  place (`SET-ASIDE:`, `CANNOT:` for a path containing `DISK_STANDIN_COULD_NOT`,
  an `ASK:` for each place marked to ask first, then `SET-ASIDE:` on "Remove it"
  and `KEEP:` otherwise) and the helper moves. The story checks the helper's
  record (the request carried the list), that Codex ran read-only and was told the
  convention, and that the only questions were those about the marked places.
- `lsblk` answers in lsblk's real JSON shape for two disks. The home folder's
  partition is mounted at `DISK_STANDIN_LSBLK_MOUNT` with
  `DISK_STANDIN_LSBLK_HOME_SIZE` bytes; nothing else is mounted, so no real disk
  of the computer it runs on is mistaken for one of the partitions.
  `DISK_STANDIN_LSBLK_FAIL=1` makes it fail.
- `recipe` answers `storage --json` (`DISK_STANDIN_EARLIER` bytes, default 6 GiB).
- `recab` reads no disk. `DISK_STANDIN_SYSTEMS` picks its story: `before` (the
  other system is the one from before the last change; the default), `next`
  (the other holds a change not started yet), `identical`, or `none` (no second
  system).

## What it asks of recab

`recab status --json`, as the Recab window reads it, for what the two
systems are. From the real recab (schema 7 to 10; recab sends 10 today) it takes the two slots, the
generation each holds, the preparation's revision and `lifecycle.other`, in
which recab itself says what the other system is: the next one, the system
from before, or a copy; the running
system's name comes from `/etc/os-release`. The slots, their generations and
the preparation's revision together are the state a comparison was made under;
under another state it is shown as out of date. The stand-in (schema 3) names
the releases and the role itself and counts a `revision`.

And one command, run as root, that reads both systems and changes neither:

```
recab compare --json --machine-events
```

While it reads, it reports on its standard error with the events below. The
real recab sends `plan`, `step` and `done`, and no `progress`: the bar moves
without saying how far it is. A failure is recab's one JSON error object.
When done it writes one JSON object on its standard output:

| Field | What |
|---|---|
| `schema_version` | `1` |
| `running`, `other` | each: `slot`, `device`, `files` (files and links, not folders), `bytes` (each hard-linked file once) |
| `other.role` | `copy` (made as one, nothing changed since), `previous` (the system from before the last change) or `next` (holds a change that is not started) |
| `same` | `files` and `bytes` that are the same in both |
| `folders` | every folder down to the depth recab chooses (three names below `/`), each with its whole size on both sides: `path`, `running_bytes`, `other_bytes`. The maps are drawn from this |
| `own` | paths that differ by design, because each system names its own partition; not counted as differences |
| `differences` | one per path: `path`, `change`, `running_bytes`, `other_bytes`, and `detail` for `metadata`. A folder that only one system has stands for all it holds |
| `more` | how many differences were left out of the list, if it was cut |

`change` is `only-running`, `only-other`, `content` (the content differs) or
`metadata` (same content; owner, mode, links or attributes differ, said in
`detail`). This is what `inventory` already reads for one system, taken for
both and set side by side.

## What it asks of the agent

The agent is one program handed the prompt as one JSON line on its input,
`{"prompt": "..."}`, with the input left open: `The issue:`, `The facts:` (one
line per place: `- PATH (SIZE): why. how.`, with `Ask me first.` at the end
where it applies) and `The expected result:`. It reports on its standard
error, one JSON object per line, with the same events as the other two apps:
`plan`, `step`, `progress`, `output`, `agent`, `failed`, `done`, and one of its
own, `place` (what it did with one place; see "The set-aside convention"). To ask, it
sends `question` with an `id`, a `text` and `choices`; the answer comes back
on its input as `{"question": id, "answer": "..."}`. When asked what differs between the two systems, what it says in
`agent` is shown as its answer.

## The source

| File | What |
|---|---|
| `src/scan.*` | reads a folder tree from the disk into memory |
| `src/mess.*` | the rules, and the finding for something picked by hand |
| `src/map.*` | the map of blocks, used for the mess and for each of the two systems |
| `src/disk.*` | The disk: measuring free space, reading lsblk's answer, the page, the split of whose room it is |
| `src/browse.*` | Browse |
| `src/pages.*` | the Mess list and the hand-over |
| `src/after.*` | the page for what was set aside |
| `src/setaside.*` | the holding folder: reading manifests, bringing a place back, letting go, and the confinement |
| `src/rules.*` | the places the person said never to suggest (their config folder) |
| `src/systems.*` | the comparison of the two systems: what recab answered, and its page |
| `src/window.*` | the frame, its two entries and the flows from each to the agent and back |
| `src/shots.*` | the picture mode and flow check |
| `fixtures/` | the stand-ins (agent, lsblk, recipe, recab) and the sample home folder |
| `data/` | the menu entry |
| `../kit/` | the frame (rail, strip, status line, Activity), shared widgets, the page base, the work page, the command runner |

## Not done

- **A real agent following the convention has never been run.** The helper moves
  and writes the manifest, so what is held is always as recorded, but whether
  Codex or Claude Code writes the `SET-ASIDE:`, `KEEP:` and `CANNOT:` lines as
  asked, and names the full path of a marked place in its question, was only
  checked against stand-ins (`gui/agent/fixtures/codex` and `claude`). A place the
  agent names that is on another disk than the holding folder is not moved (the
  page says so as "Could not"). A program set as the agent by its path moves the
  places itself: the hand-over page then says so (it asks the helper which agent is
  chosen), and says that only the instruction keeps the agent from changing files
  itself when `run_commands` is at "allowed". The story checks the words for each
  case (the default, commands allowed, a program by its path, a helper that cannot
  be asked), asked of the real helper.
- "Free once you let it go" is free now plus what the holding folder takes as
  this window counts it (room on disk, or the file lengths under
  `DISK_GUI_APPARENT`); on the sample, whose files are empty but long, nothing is
  really freed by letting go, and the page's measured "Free now" says so.
- Logs and crash records (the mock-up's third row), and any figure for what could
  not be read without the administrator's password, are not on The disk: neither
  can be measured as the person, so nothing is claimed. Only the number of
  folders the scan could not read is said.
- The kinds of file in Browse ("By kind of file") are not there: the scanner does
  not record file kinds.
- "Never suggest this" is a list of whole places; there is no rule language
  ("anything named *.qcow2 older than 90 days"), and no list of places not to
  look at.
- "Bring everything back" does not ask and does not retry a place that was
  refused; it says which were refused and why. A place whose folder was
  replaced by a link since is refused.
- The Mess "Add to the list" action is on Browse now (the map is there).
- The partition table lists what `lsblk` lists below the disk. LVM, LUKS and
  btrfs layouts were not tried (the stand-in has plain partitions); an unlisted
  device shows by its label or filesystem. The lsblk columns `MOUNTPOINTS`
  (util-linux 2.37 or later) and a failing lsblk were not run against a real one.
- The strip is read at start, on refresh and after an agent run; not while a
  change is made elsewhere until the window is entered again.
- Help and the first page (the frame's own, `gui/kit/README.md`) say nothing of the kinds of
  file in Browse, a rule language for "Never suggest", logs and crash records, mess
  inside the systems, or a real agent following the set-aside convention; they say
  the page for what was set aside reads the holding folder back. Help
  (`gui/kit/help.cpp`, not changed from here) still says that what belongs to
  Recipe is "cleared through Recipe", and that comparing "asks for the
  administrator's password" whether or not a root helper is set; neither is true now.
- Choosing the other entry while a comparison reads is allowed (the page keeps
  filling). Activity does not list comparisons: only agent runs are recorded.
- Checked only offscreen, with stand-ins; not on a real desktop, and 
  link buttons, the Recipe app and the Recab app were not started for real. The
  holding folder was never measured on a folder of real size (it is read off the
  window's thread, but a very big one takes as long to read as a scan does).
- A comparison of a real pair of systems: `recab compare` has only run on
  sample trees. It says no byte progress, so the bar cannot say how far it is.
- The kept comparison is one file in the person's cache
  (`DISK_GUI_KEPT`); nothing checks that it belongs to this computer's disks
  beyond recab's revision number.
- Seeing what changed inside a file whose content differs; searching or
  copying the list of differences.
- Recipe's own folders are left alone because Recipe has no command that clears
  them (`recipe cleanup NAME` is for one recipe's build files, `recipe storage
  --clear-earlier` for earlier versions). When it has one, Recdirstat can hand them to
  Recipe. The real `recipe storage --json` was not run here: the shape (`earlier_bytes`,
  `earlier_generations`) was read from `tools/recipe` and `gui/recipe`.
- Mess inside the systems. The map reads only the home folder and
  `/var/lib/recipe`; the two systems are compared with each other, not searched
  for leftovers.
- In an image: `distro/builder/os-recipes/desktop-apps.rhai` builds and installs it with
  the other two, and the contract's `DESKTOP_APPS` lists it; no image with it has been built.
