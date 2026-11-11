# disk-gui

The third desktop app, titled "Disk". It sits in the shared window frame
(`gui/kit/frame.*`, see its README): the rail with the three apps, Disk's four
entries, Activity and Settings; the pages; the strip about the next system; the
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
disk). Nothing has been handed to a real agent: `fixtures/agent` stands in for
it and moves only inside a sample tree. **A real agent following the request
(set aside by moving, a manifest, `place` events) has never been run**; how a
real Codex or Claude Code is started, and that it answers with the events below
and keeps the set-aside convention, is not built or checked. The pages for The
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

1. **The hand-over**: the issue, the facts (one per ticked place: path, size,
   why it can go, how it should be cleared) and the expected result, which the
   person can change. The issue also carries the convention the agent must
   follow (below); it is the part the person cannot rewrite.
2. **The agent works.** Before it moves anything marked "asks first" it asks,
   and the question is answered in the window.
3. **What was set aside** (the page after the run, under Mess; also from The
   disk): a heading with how much is held; "Free before" (measured when the
   request was handed over), "Free now" (measured now) and "Free once you let it
   go" (free now plus what the holding folder takes), all `statvfs` and a
   measurement of the folder, not counted from the list; one row per place:
   *Set aside* (and, when it asked, what the person answered), *Kept* (and why),
   *Left alone* (Recipe's), *Could not* (and why, with "Try again", which hands
   just that place over again), *Brought back*, *Let go*; a place held that was
   not on the list is flagged; a place the agent said it set aside that is not
   in the holding folder is flagged. A place the person said to keep offers
   "Never suggest".
4. **"Bring back"** (one place) and **"Bring everything back"**: the window moves
   them back to where they were, and refuses, saying why, when something is
   there now.
5. **"Let it all go, free X"**: asks first (the question says how much and that
   it cannot be undone), then deletes the holding folder. Nothing lets it go by
   itself: not after some days, not at all unless the person presses it.

The rail entry "Mess" carries a badge: how many places of mess are on the list.
A hand-over, the agent's work and the page for what was set aside are steps of
the entry they came from and keep it marked; while the agent works, choosing
"Mess" shows the work. Activity, from the frame, lists the agent's runs of all
three apps; a run started here is listed as "Disk", under the first line of its
issue, and "Go to it" brings the person back to the work page. The strip, "The
next system holds N changes." with "Finish and restart…", is read from `recab
status --json` (`DISK_GUI_RECAB`) and shown on every page; when recab cannot be
read or answers in a shape the strip does not know, there is no strip and no
error. The button and "Open System…" start the System app the same way
(`LEVITATE_APP_SYSTEM`, else `recab-gui`; the frame starts it, as the rail does).

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
   compare again. "Open System…" starts the System window, where changing,
   updating and going back are done.
2. **Compare.** Both systems are read from start to end, which takes minutes
   and needs the administrator's password. The page stays: each frame fills as
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
6. **Ask the agent what differs.** The differences go to the agent as issue,
   facts and expected result. The agent changes nothing; what it answers is
   shown above the list.

## The set-aside convention

What the request tells the agent, and what the window reads back. One holding
folder per request, on the same disk as the home folder:

```
~/.local/share/levitate/set-aside/<yyyyMMdd-HHmmss>/
    manifest.json      {"version": 1, "entries": [{"from": "/home/me/.cache/x", "held": "home/me/.cache/x", "bytes": 123}]}
    home/me/.cache/x   the place itself, moved here under its own full path
```

The request names the holding folder (`The holding folder is "..."`), says to
move, not delete, and to say what was done with each place as an event
`{"event": "place", "path", "result": "set-aside" | "kept" | "left" | "could-not",
"reason"}` (and `asked`, `answer` when it asked). A place marked "Ask me first"
is asked about, as before; a yes means set aside. What belongs to Recipe is
cleared through Recipe and is not moved. What the page shows as set aside is
what the manifests say and the holding folder really holds, measured; the
events only say what happened to the places that are not held.

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
| Recipe's own | `~/.cache/recipe`, `~/.local/state/recipe`, `/var/lib/recipe/state` | clears through Recipe |
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

Window 1100 x 700 the first time, then the size it had when it closed
(`rememberWindowSize` in `gui/kit`; the picture modes always use 1100 x 700).

### The stories

`--shots FOLDER` looks, saves pictures and checks expectations; with
`--hand-over` it also hands the list to the agent and goes on to what was set
aside and the comparison of the two systems. Both end with "all as expected" or
say "not as expected: ..." for each thing that failed, and exit non-zero.
They need no display, and no real agent, recab, recipe, lsblk or administrator.

Both stories (pictures `1-disk`, `2a`..`2f` Browse, `3a-mess`, `3c-mess-told`,
`3-picked`, `3-hand-over`, `1b-settings`, `s1`/`s2` the strip, `a1`/`a2`
Activity):

- the window is 1100 x 700, and **every picture** is taken at that size (no page
  may make the window wider or taller); the rail offers The disk, Browse, Mess,
  The two systems, Activity and Settings and nothing else; the window opens on
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
again and no longer held; "Let it all go" asks first (and says how much), "Keep
it" deletes nothing, then the holding folder is gone and **a sentinel file beside
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
export HOME=$S/home XDG_CONFIG_HOME=$S/home/config XDG_STATE_HOME=$S/home/state \
  XDG_CACHE_HOME=$S/home/cache XDG_DATA_HOME=$S/home/share
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

The comparison keeps using the Disk stand-in `$F/recab`, which answers `status`
in the older shape the strip cannot read (schema 3, with the partitions
`/dev/vda2` and `/dev/vda3` the `lsblk` stand-in knows); that is also the check
for "no strip". `DISK_STANDIN_STRIP_RECAB` is `recab-real`, used only for the
strip (`prepare` printed `p2` as the preparation's id in a fresh state).
Activity needs `RECAB_GUI_RECORDS` and `LEVITATE_ACTIVITY_AGENT` (build the agent
first: `cmake --build .artifacts/agent/build`); without them, or without the
strip's two variables, the story says what is missing and fails. Pictures with
`--hand-over`: `4-after` (what was set aside), `4a-disk-with-set-aside`,
`4b-after-refused`, `4c-after-one-back`, `4d-after-let-go`, `4e-disk-after-let-go`,
then `5-systems` to `8-explained`.

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

- `agent` moves a place only when it lies inside `DISK_STANDIN_ROOT`, into the
  holding folder the request names, and writes the manifest; it leaves what is
  marked "Ask me first" when the answer is "Keep it", and what belongs to Recipe.
  `DISK_STANDIN_COULD_NOT=TEXT` makes it fail, as a program holding files open
  would, on every place whose path contains TEXT. It refuses a request that names
  no holding folder.
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

`recab status --json`, as the System window reads it, for what the two
systems are. From the real recab (schema 7) it takes the two slots, the
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

- **A real agent following the request has never been run.** The set-aside
  convention (move into the holding folder, write the manifest, say what was done
  with a `place` event) is asked of the agent in words; the stand-in does it, and
  nothing checks that Codex or Claude Code would. A real agent may also set aside
  something across a disk boundary or not write the manifest; the page then shows
  what the manifests and the holding folder really hold, and says when a place
  the agent claimed is not there, but nothing here can make an agent obey.
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
- The frame's Help is not here (Settings is the frame's own and is).
- Choosing the other entry while a comparison reads is allowed (the page keeps
  filling). Activity does not list comparisons: only agent runs are recorded.
- Checked only offscreen, with stand-ins; not on a real desktop, and the rail's
  app buttons, the Recipe app and the System app were not started for real. The
  holding folder was never measured on a folder of real size (it is read off the
  window's thread, but a very big one takes as long to read as a scan does).
- A comparison of a real pair of systems: `recab compare` has only run on
  sample trees. It says no byte progress, so the bar cannot say how far it is.
- The kept comparison is one file in the person's cache
  (`DISK_GUI_KEPT`); nothing checks that it belongs to this computer's disks
  beyond recab's revision number.
- Seeing what changed inside a file whose content differs; searching or
  copying the list of differences.
- Clearing Recipe's own folders "through Recipe": the prompt says so. Recipe
  has `recipe cleanup NAME` for one recipe's build files; whether anything
  clears its whole cache or its saved build steps was not checked. The real
  `recipe storage --json` was not run here: the shape (`earlier_bytes`,
  `earlier_generations`) was read from `tools/recipe` and `gui/recipe`.
- Mess inside the systems. The map reads only the home folder and
  `/var/lib/recipe`; the two systems are compared with each other, not searched
  for leftovers.
- It is in no image: no recipe builds it and no desktop lists it yet.
