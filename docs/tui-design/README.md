# One design for every terminal screen (2026-11-10)

A design, drawn as mock-ups from one set of parts: the install screen's apps and the other terminal apps. The mock-ups are the design; this page is its rules and what building it needs.

**Parts of the mock-ups are out of date.** They were drawn before a decision in [`docs/product-handoff.md`](../product-handoff.md): the install hub is gone (the install screen is a root terminal beside the guide, section 2.1). The pages named "Hub" in `pages_install.py` show the removed design. They are kept as a record of the layout, not as something to build. The recab pages ("Updates") were deleted on 2026-11-16, because recab has no terminal app (section 2.2). The disk-plan, base-system, system-setup and guide pages still stand.

```sh
cd docs/tui-design
python3 build.py /tmp/design-board.html   # the board: the rules, then every page
python3 check.py all pages_system         # the same pages as plain text
```

`kit.py` is the parts (and the palettes, read from the variants' `distro.toml`), `pages_install.py` and `pages_system.py` are the pages. A page is built only from the kit's parts, which is what keeps the pages alike. Every value in a mock-up is sample data. The pages are LevitateOS's (light), except the two marked AcornOS (dark).

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
10. **Two brands.** LevitateOS and AcornOS differ in their whole dark palette, taken from
    the `[look.dark]` table of the variant's `distro/variants/<id>/distro.toml`, which
    `kit.py` reads. Terminal apps are dark on both distros (a light terminal app is
    unnatural); the mode a distro starts in is for its desktop and website. A page is drawn
    in roles (surface, text, accent, ...), never in colours, so the two brands differ in
    every colour but in no part. Text on a filled accent, second accent or status colour is
    the surface colour, which contrasts with each of them.

The design went through three passes: one frame for everything, then a picture for each
page, then balance: taking out what competed with the picture.

## Each page's picture

Inside the shared frame, each page is laid out for the one thing it does (`Deliberate`
in `kit.py` holds the parts made for this):

| Page | Its picture |
|---|---|
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
| Guide | a reading measure and numbered steps |

## What building it needs

The apps live in [`tools/tui`](../../tools/tui): the kit in `tools/tui/kit/core` and the apps in `tools/tui/apps` (`install/disk-plan`, `install/base-system`, `install/system-setup`, `live-tools/install-docs`).

In the kit, beyond what is there since the first restyle:

- `Heading`: the diamond, tick and warning glyphs beside the numbers.
- `Panel`: keys inside it (the yes-or-no), and facts.
- Cards for a choice of two; the list rows and search field moved from system-setup.
- A thick `ProgressBar`; a table row as the selection bar.
- The rail as a grouped list (the guide), sharing `StepPath`'s rows.
- `ready` drawn as a filled dot in `StepPath` (today a hollow one).

In the apps: every page laid out as its mock-up. system-setup is closest; disk-plan and base-system keep their logic and change their pages; the guide changes its page furniture (notes as panels, commands as blocks, numbered headings).

In the install screen (`distro/builder/install-split-pane`, Rust): no boxes around the panes, the pane without the keyboard drawn quieter, and its own keys as keycaps on one row. The guide needs a way to be told it is the second pane, so it draws its band flat and without the distro's name.

What the design asks of the tools, beyond the apps:

- `recstrap --check --json` reports only how many checks passed; the Check page names them (the names are in `tools/recstrap/src/bin/recstrap/guards.rs`).
- recpart's dry run gives one verdict; the rehearsal page lists what it tested.
- The keyboard picture needs each keymap's layout family (kbd keeps its maps in `qwerty`, `qwertz`, `azerty`, `dvorak` directories); the language preview needs the locale's formats (Intl has them).
- Each step's app should say what it will do and what it left; the install state file holds part of that today.

## Open

- The mock-ups are 100 by 28; the install screen is 160 by 30. Smaller terminals drop the joining lines of the rail first, as system-setup does today.
- The bare Linux console has 16 colours and no braille: the map needs a fallback there.
