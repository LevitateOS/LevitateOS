# One design for every terminal screen (2026-11-10, a design, not built)

Every page of the install screen and the six apps, drawn as mock-ups from one set of
parts. The mock-ups are the design; this page is its rules and what building it needs.

```sh
cd docs/tui-design
python3 build.py /tmp/design-board.html   # the board: the rules, then all 54 pages
python3 check.py all pages_system         # the same pages as plain text
```

`kit.py` is the parts, `pages_install.py` and `pages_system.py` are the pages. A page is
built only from the kit's parts, which is what keeps the pages alike. Every value in a
mock-up is sample data.

## The rules

1. **One frame.** Band, step line, rail, content, keys: the same five parts in the same
   places. Nothing is boxed.
2. **One thing to look at.** Each page has one picture of what it is doing, and only
   that (the table below).
3. **One thing to press, in one place.** An action is one bar, always on the same row at
   the foot of the page; a page with nothing to press has one quiet line there.
4. **A large figure only where it says something.** A step's number, a tick at the end,
   a warning when something stopped, a percentage during a long job.
5. **One rail, one set of marks.** ✓ done, ◉ you are here, ● ready, ○ not yet,
   ◐ running, ✗ failed or refused.
6. **Panels are for verdicts and questions.** Explanation is plain text.
7. **Destructive steps ask for a word**, a cell a letter.
8. **Few labels, short sentences.** A title and one line; small capitals only as table
   headers.
9. **One ending.** The tick, the thing made, the bar to the next step.
10. **Two brands.** Only the band, the accent and the second accent differ.

The design went through three passes: one frame for everything, then a picture for each
page, then balance: taking out what competed with the picture.

## Each page's picture

Inside the shared frame, each page is laid out for the one thing it does (`Deliberate`
in `kit.py` holds the parts made for this):

| Page | Its picture |
|---|---|
| Hub, the step you are at | from what onto what, and the bar |
| Hub, a finished step | a receipt: what it left, drawn |
| Hub, Finish | the machine you made, as a card |
| Disk, Overview | the line where writing starts |
| Disk, choosing a disk | each disk as a bar of what is on it |
| Disk, System and Disk use | each choice as the disk it leaves |
| Disk, Partitions | the selected stretch marked on the bar; what A/B still needs beside the table |
| Disk, Plan and Confirm | now over after |
| Disk, Commands and Apply | the run sheet, the running command raised |
| Disk, Confirm | the word typed a letter a cell |
| Base system, Overview | the image, an arrow, the target |
| Base system, Check | room as a gauge; the twelve checks named |
| Base system, Copy | the percentage three rows tall |
| Base system, fstab | the file as it will be written, as a diff |
| Configuration, Overview, Review, Done | the machine as one card: empty, filled, true |
| Configuration, Keyboard | the keys of the layout, and a line to try it on |
| Configuration, Time zone | the place on a map, its clock now |
| Configuration, Language | today's date, a time and a number as it will write them |
| Configuration, Hostname | the login line, the prompt and the network name with it in them |
| Configuration, passwords | both entries, a strength line, what the choice means |
| Configuration, User | the account as it will exist |
| Updates, every page | the two slots as two blocks |
| Updates, a trial | the next boot as a fork you can follow |
| Updates, Write a system | a choice of release media, not three paths |
| Updates, History | a line down the page |
| Guide | a reading measure and numbered steps |

## What building it needs

In the kit (`tui/kit/core`), beyond what is there since the first restyle:

- `Heading`: the diamond, tick, warning and slots glyphs beside the numbers.
- `Panel`: keys inside it (the yes-or-no), and facts.
- Cards for a choice of two; the list rows and search field moved from system-setup.
- A thick `ProgressBar`; a table row as the selection bar.
- The rail as a grouped list (recab, the guide), sharing `StepPath`'s rows.
- A step line with named segments (recab's two slots).
- `ready` drawn ● in `StepPath` (today ○).

In the apps: every page laid out as its mock-up. system-setup is closest; the hub,
disk-plan and base-system keep their logic and change their pages; the guide
changes its page furniture (notes as panels, commands as blocks, numbered headings).
Recab's current pages can inform the layout, but its backend and frontend also
need the [required update and continuation lifecycle](../../tools/recab/REQUIREMENTS.md):
matching initial systems, incremental preparation, changes and downloaded updates
in one preparation, conflict handling and finalization in either direction.
The existing destructive staging flow is not the final interaction model.

In the install screen (`distro/builder/install-split-pane`, Rust): no boxes around the
panes, the pane without the keyboard drawn quieter, and its own keys as keycaps on one
row. The guide needs a way to be told it is the second pane, so it draws its band flat
and without the distro's name.

What the second pass asks of the tools, beyond the apps:

- `recstrap --check --json` reports only how many checks passed; the Check page names
  them (the names are in `tools/recstrap/src/bin/recstrap/guards.rs`).
- `recab stage` prints no progress; the Write page shows a bar. Choosing release media
  needs a way to list them (`recab stage --from-media` takes one already).
- recpart's dry run gives one verdict; the rehearsal page lists what it tested.
- The keyboard picture needs each keymap's layout family (kbd keeps its maps in
  `qwerty`, `qwertz`, `azerty`, `dvorak` directories); the language preview needs the
  locale's formats (Intl has them).
- The hub's launch pad and receipt need each step's app to say what it will do and what
  it left; the install state file holds part of that today.

## Open

- recab's name in the band is "Updates" in the mock-ups; the app is `recab-tui`.
- The mock-ups are 100 by 28; the install screen is 160 by 30. Smaller terminals drop
  the joining lines of the rail first, as system-setup does today.
- The bare Linux console has 16 colours and no braille: the map needs a fallback there.
