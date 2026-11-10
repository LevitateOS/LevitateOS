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

1. **One frame.** Band (whose system, which app, where you are), step line, rail,
   content, keys: the same five parts in the same places. Nothing is boxed.
2. **One opening.** A large glyph, a title, one line. The glyph is the step's number on
   a step, a diamond on an overview, a tick when the app is done, a warning when
   something stopped.
3. **One rail.** A mark, a name, what was answered. Steps in order are joined by a
   line; things to choose from are grouped under a small label. The current row is a
   raised bar with the accent at its edge.
4. **One set of marks.** ✓ done, ◉ you are here, ● ready, ○ not yet, ◐ running,
   ✗ failed or refused.
5. **One verdict.** A raised panel with a solid edge in its tone, directly under the
   opening: green done, sand a question or warning, salmon a refusal or failure,
   grey-green a note.
6. **One way to ask.** A yes-or-no is the warning panel with its two keys in it.
   Something destructive also asks for a typed word, in a field. Two choices are two
   cards; many are a search field and a list.
7. **One way to show data.** Facts are label and value; tables have a quiet header;
   commands sit on the darker surface behind a prompt; a long job is a thick bar and
   its figures.
8. **One ending.** The large tick, what was done, the next step.
9. **One line of keys.** Keycaps at the bottom left in the order you would press them;
   what the screen is working on at the bottom right.
10. **Two brands.** Only the band, the accent and the second accent differ.

## The second pass: every page has its own picture

Inside the shared frame, each page is laid out for the one thing it does (`Deliberate`
in `kit.py` holds the parts made for this):

| Page | Its picture |
|---|---|
| Hub, the step you are at | a launch pad: what the app does, from what onto what, one bar |
| Hub, a finished step | a receipt: what it left, drawn |
| Hub, Finish | the machine you made, as a card |
| Disk, Overview | the app as a map, with the line where writing starts |
| Disk, choosing a disk | each disk as a bar of what is on it |
| Disk, System and Disk use | each choice as the disk it leaves |
| Disk, Partitions | the selected stretch marked on the bar; what A/B still needs beside the table |
| Disk, Plan and Confirm | now over after |
| Disk, Commands and Apply | a numbered run sheet, the running command raised |
| Disk, Confirm | the word typed a letter a cell |
| Base system, Overview | the image, an arrow, the target |
| Base system, Check | room as a gauge; the twelve checks named |
| Base system, Copy | the percentage three rows tall |
| Base system, fstab | the file as it will be written, as a diff |
| Configuration, Overview, Review, Done | the machine as one card: empty, filled, true |
| Configuration, Keyboard | the keys of the layout, and a line to try it on |
| Configuration, Time zone | the place on a map, its clock now |
| Configuration, Language | today's date, a time and a number as it will write them |
| Configuration, Hostname | the prompt and the login line with the name in them |
| Configuration, passwords | both entries with their state, a strength line, the keyboard in use |
| Configuration, User | the account as it will exist |
| Updates, every page | the two slots as two blocks |
| Updates, a trial | the next boot as a fork you can follow |
| Updates, Write a system | a choice of release media, not three paths |
| Updates, History | a line down the page |
| Guide | a reading measure, numbered steps, where you are on the page |

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
disk-plan, base-system and recab keep their logic and change their pages; the guide
changes its page furniture (notes as panels, commands as blocks, numbered headings).

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
