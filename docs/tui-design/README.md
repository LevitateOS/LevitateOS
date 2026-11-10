# One design for every terminal screen (2026-11-10, a design, not built)

Every page of the install screen and the six apps, drawn as mock-ups from one set of
parts. The mock-ups are the design; this page is its rules and what building it needs.

```sh
cd docs/tui-design
python3 build.py /tmp/design-board.html   # the board: the rules, then all 51 pages
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

## Open

- recab's name in the band is "Updates" in the mock-ups; the app is `recab-tui`.
- The mock-ups are 100 by 28; the install screen is 160 by 30. Smaller terminals drop
  the joining lines of the rail first, as system-setup does today.
- The bare Linux console has 16 colours and no braille: the map needs a fallback there.
