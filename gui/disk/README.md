# disk-gui

The third desktop app, titled "Disk". It does two things, each on its own tab:

- **Mess**: it finds where the mess on the disk is hiding and hands the list
  to the agent. A disk map in the manner of QDirStat with one job added.
- **The two systems**: it compares the running system with the other one (the
  A and B root partitions) and says how alike they are and what differs.

It removes and changes nothing itself; the premise is the same as in
`gui/recipe` and `gui/recab`: an agent does the work, and is given the issue,
the facts and the expected result.

Qt 6 Widgets, C++20, CMake. Shared widgets are in `gui/kit`.

## State (2026-11-10)

**The finding is real; the agent is a stand-in.** The scanner and the rules
have run on a real home folder (172 GiB read in about nine seconds, on a warm
disk). Nothing has been handed to a real agent: `fixtures/agent` stands in for
it and removes only inside a sample tree. How a real Codex or Claude Code is
started, and that it answers with the events below, is not built.

**The two-systems tab runs against a stand-in only.** recab has no `compare`
command: it has `inventory`, which reads one system, and
`prepare` finds the differences in order to copy them, not to show them.
`fixtures/recab`
plays the answer this window needs; what it needs is written down under "What
it asks of recab". No real pair of systems has been compared.

## What a person sees

1. **Looking for mess.** The window reads the home folder (and
   `/var/lib/recipe` when it can be read) and counts as it goes.
2. **The map and the list.** Left, the disk as a map: each rectangle's area is
   the room that thing takes, the biggest folders carry their name and size,
   and mess is coloured by kind. Right, the mess by kind, each line ticked,
   with what the agent will do with it: `clears` or `asks first`.
3. **Pointing.** A click on the map, on a folder's name strip or on a line
   selects that thing; the line under the map shows its path, every part of
   which can be clicked to go up. Anything the rules did not name can be put on
   the list with "Add to the list", as "Picked by you": the agent is told to
   find out what it is and ask before removing anything.
4. **The hand-over.** The issue, the facts (one per ticked place: path, size,
   why it can go, how it should be cleared) and the expected result, which the
   person can change.
5. **The agent works.** Before it removes anything marked "asks first" it
   asks, and the question is answered in the window. Then the window reads
   the disk again and says what was really freed.

"Open the folder" shows what is pointed at in the file manager. "Read again"
in the status bar, F5 and Ctrl+R look at the disk again on the first tab and
ask recab again on the second. Coming back to the window after a while asks
recab again; the disk is only looked at again when asked, since that takes a
while and drops what was ticked.

On the second tab:

1. **What is there.** On opening, recab is asked what the two systems are
   (quick, no password): the page names both and shows two empty frames. If a
   comparison was kept from before, it is shown instead, with when it was
   made; when either system has changed since, it says so and offers to
   compare again. "Open System…" starts the System window, where changing,
   updating and going back are done.
2. **Compare.** Both systems are read from start to end, which takes minutes
   and needs the administrator's password. The tab stays: each frame fills as
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

## What counts as mess

The rules are in `src/mess.cpp`. A place smaller than 20 MiB is not named.

| Kind | What | Agent |
|---|---|---|
| Caches | each folder in `~/.cache`; any folder with a `CACHEDIR.TAG` | clears |
| Build leftovers | `target` beside a `Cargo.toml`, and any tagged folder that looks like Rust output; a folder with a `CMakeCache.txt`; `node_modules` beside a `package.json` | clears |
| Recipe's own | `~/.cache/recipe`, `~/.local/state/recipe`, `/var/lib/recipe/state`, `package-generations` | clears through Recipe; asks first for the kept versions |
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

On a sample home folder, with the stand-in agent:

```sh
A=$PWD/.artifacts/disk-gui; F=$PWD/gui/disk/fixtures
$F/make-sample $A/sample
DISK_GUI_ROOTS=$A/sample DISK_GUI_HOME=$A/sample DISK_GUI_APPARENT=1 \
  DISK_GUI_AGENT=$F/agent DISK_STANDIN_ROOT=$A/sample \
  $A/build/disk-gui --look levitate
```

`--shots FOLDER` looks, picks the biggest unnamed folder by hand, opens the
hand-over, and saves a picture at each point; with `--hand-over` it also sends
the list to the agent, waits for the second look, then compares the two
systems and asks the agent what differs. It exits non-zero when nothing was
freed, the systems could not be compared, or the agent said nothing. It needs
no display:

```sh
QT_QPA_PLATFORM=offscreen DISK_GUI_ROOTS=$A/sample DISK_GUI_HOME=$A/sample DISK_GUI_APPARENT=1 \
  DISK_GUI_AGENT=$F/agent DISK_GUI_RECAB=$F/recab DISK_GUI_ROOT_HELPER= \
  DISK_STANDIN_ROOT=$A/sample DISK_STANDIN_SPEED=0 \
  $A/build/disk-gui --look levitate --shots $A/shots-levitate --hand-over
```

Without `--hand-over` and without the settings it reads the real home folder,
hands nothing to anyone and compares nothing.

| Setting | What | Default |
|---|---|---|
| `DISK_GUI_ROOTS` | the folders to read, separated by `:` | the home folder, and `/var/lib/recipe` |
| `DISK_GUI_HOME` | which folder the home rules apply to | the home folder |
| `DISK_GUI_AGENT` | the agent program | `codex` |
| `DISK_GUI_APPARENT` | `1`: count each file's length (the sample is made of empty-but-long files) | off |
| `DISK_GUI_RECAB` | the program asked to compare the two systems | `recab` |
| `DISK_GUI_ROOT_HELPER` | what runs it as root; empty: run it as the person | `pkexec` |
| `DISK_GUI_SYSTEM_APP` | what "Open System…" starts | `recab-gui` |
| `DISK_GUI_KEPT` | the file the last comparison is kept in | `comparison.json` in the person's cache |
| `DISK_GUI_OS_RELEASE` | where the running system's name is read | `/etc/os-release` |

The stand-in agent removes a place only when it lies inside
`DISK_STANDIN_ROOT`; it leaves what is marked "Ask me first" and what belongs
to Recipe.

The stand-in recab reads no disk. `DISK_STANDIN_SYSTEMS` picks its story:
`before` (the other system is the one from before the last change; the
default), `next` (the other holds a change not started yet), `identical`, or
`none` (no second system).

## What it asks of recab

`recab status --json`, as the System window reads it, for what the two
systems are. From the real recab (schema 6) it takes the two slots, the
generation each holds, `rollback_slot` and the preparation: the other system
is the next one while a preparation exists, the system from before when it is
the rollback slot, and a copy when both hold the same generation; the running
system's name comes from `/etc/os-release`. The slots, their generations and
the preparation's revision together are the state a comparison was made under;
under another state it is shown as out of date. The stand-in (schema 3) names
the releases and the role itself and counts a `revision`.

And one command, run as root, that reads both systems and changes neither:

```
recab compare --json --machine-events
```

While it reads, it reports on its standard error with the events below
(`plan`, `step`, `progress` with `unit: "bytes"`, `failed` with a `reason`).
When done it writes one JSON object on its standard output:

| Field | What |
|---|---|
| `schema_version` | `1` |
| `running`, `other` | each: `slot`, `device`, `files`, `bytes` |
| `other.role` | `copy` (made as one, nothing changed since), `previous` (the system from before the last change) or `next` (holds a change that is not started) |
| `same` | `files` and `bytes` that are the same in both |
| `folders` | every folder down to the depth recab chooses, each with its whole size on both sides: `path`, `running_bytes`, `other_bytes`. The maps are drawn from this |
| `own` | paths that differ by design, because each system names its own partition; not counted as differences |
| `differences` | one per path: `path`, `change`, `running_bytes`, `other_bytes`, and `detail` for `metadata` |
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
`plan`, `step`, `progress`, `output`, `agent`, `failed`, `done`. To ask, it
sends `question` with an `id`, a `text` and `choices`; the answer comes back
on its input as `{"question": id, "answer": "..."}`. When asked what differs between the two systems, what it says in
`agent` is shown as its answer.

## The source

| File | What |
|---|---|
| `src/scan.*` | reads a folder tree from the disk into memory |
| `src/mess.*` | the rules, and the finding for something picked by hand |
| `src/map.*` | the map of blocks, used for the mess and for each of the two systems |
| `src/pages.*` | the map with its list, and the hand-over |
| `src/systems.*` | the comparison of the two systems: what recab answered, and its page |
| `src/window.*` | the two tabs and the flows from each to the agent and back |
| `src/shots.*` | the picture mode and flow check |
| `fixtures/` | the stand-in agent, the stand-in `recab compare`, the sample home folder |
| `data/` | the menu entry |
| `../kit/` | shared widgets, the page base, the work page, the command runner |

## Not done

- `recab compare` does not exist. With the real recab the second tab opens
  with what the two systems are, and "Compare" ends with recab's refusal.
- The kept comparison is one file in the person's cache
  (`DISK_GUI_KEPT`); nothing checks that it belongs to this computer's disks
  beyond recab's revision number.
- Seeing what changed inside a file whose content differs; searching or
  copying the list of differences.
- Starting a real agent. `DISK_GUI_AGENT` is run with the prompt on its input
  and is expected to answer with the events above; Codex and Claude Code do
  not do that as they are.
- Clearing Recipe's own folders "through Recipe": the prompt says so. Recipe
  has `recipe cleanup NAME` for one recipe's build files; whether anything
  clears its whole cache or its saved build steps was not checked.
- Mess inside the systems. The map reads only the home folder and
  `/var/lib/recipe`; the two systems are compared with each other, not searched
  for leftovers.
- It is in no image: no recipe builds it and no desktop lists it yet.
