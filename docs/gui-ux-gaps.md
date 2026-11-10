# The three desktop apps: what the UX lacks

Every gap found on 2026-11-10 in `gui/recipe` ("Recipe"), `gui/recab`
("System") and `gui/disk` ("Disk"): missing actions, pages, tabs, states and
feedback. It is a list of findings and of what they say about a redesign. It
is not a plan, and nothing here has been decided.

How it was made: each app was read by a reviewer who had not built it, against
the whole command line of its backend (`recipe`, `recab`), the backends'
documents, and what a person on a desktop wants to do. What spans all three
was read separately. No app was run for this. A handful of findings were
checked again by hand; the rest stand on the evidence given with each.

| Part | Gaps | Core | Hurts | Polish |
|---|---|---|---|---|
| Across all three (X) | 27 | 8 | 14 | 5 |
| Recipe (R) | 65 | 16 | 36 | 13 |
| System (S) | 58 | 14 | 28 | 16 |
| Disk (D) | 81 | 19 | 44 | 18 |
| **All** | **231** | **57** | **122** | **52** |

**Core** blocks a main job, **hurts** makes it worse, **polish** is finish.
"Backend" says whether the command line already offers what the app would
need. 41 of the 231 need something a backend does not have at all. For the
other 190 the backend has it, has part of it, or the work is the app's own.

## What a redesign has to settle

The gaps are not 231 separate holes. They come from eight things about how the
three apps are built.

1. **Software and the system are one thing on a locked system, in two apps.**
   Installing for everyone is a change to the next system and takes effect
   after a restart; installing for oneself is immediate. The Recipe app knows
   neither (X1 to X3, R9, R10); the System app cannot list or make software
   changes (S7, S35). The split that holds is "for me, now" against "for this
   computer, after a restart".
2. **The agent is the centre of the product and has no place.** Each app
   rebuilds a hand-over and a progress page. None lets a person choose the
   agent, see what it did, approve a step, correct it, or find an earlier
   request (X6 to X14, R15, R16, S10, S13, D7, D17). This is not a call for a
   coding workspace: it is one shared way to ask, watch, answer, review and
   look back.
3. **A run is not a thing the apps keep.** It lives while its page shows.
   Failure and success are both thrown away the moment the state is read
   again (S1, S11, D7), nothing survives closing the window (R31, R32, S32,
   D9), nothing is queued (R33), there is no history anywhere (R30, S26,
   D51).
4. **The apps guess the state the backends should report.** The System app
   turns slots and a preparation into six states of its own and gets several
   wrong (S2, S4, S5, S6, S15). The Recipe app joins two lists by name and
   breaks when a recipe is gone (R1, R2). The state belongs to recab and
   Recipe; the apps should draw it.
5. **Everything is pull.** What a person most needs to be told reaches nobody
   unless a window happens to be open: an update exists, the restart worked
   or fell back, the disk is full, the agent is waiting (X15, X16, R29, S9,
   S23, D6, D52).
6. **The Disk app has no notion of the disk**, only of the folders it reads,
   and is half a disk tool: no partitions, no folder tree, no zoom (D1, D2,
   D12 to D16). And its second tab may belong in the System app (D19).
7. **Trust and consequence are shown where they can be skipped.** What a
   recipe may use (R3), what an update changes (R4, R5), what a source means
   (R25), what differs after a restart (S12), what going back does not undo
   (S42), whether a removal can be undone (D18).
8. **There is no common ground under the three windows:** no settings, menus,
   keys, help, way from one to the other, memory of where one was, or use
   without a mouse and eyes (X18 to X27 and their like in each app).

## What the backends lack

Needed by the gaps above and not offered today.

| Backend | Lacks |
|---|---|
| recab | a ready copy in one command (S22); each slot's size and use (D23, S28); the single-system mode in `status` (D10); live progress events (S29); finding a new release (S9); undoing one change of the next system. Added on 2026-11-10 (status schema 7, read by the System and Disk apps): the lifecycle state in `status` (S2, S4 to S6, S15), a mark that a command is running (S6), every change of the next system (S7) and who asked for it (S26, as a label the caller gives); `recab compare` (D11, D53; run on sample trees only, no byte progress) |
| Recipe | build progress inside a step, and the endpoint of a refused connection in `failed` (R15); a way to pass an agent's question on (R46); installing from a remote source (R11); a history command (R30); sizes of its build state and caches (D25, D26); download and disk needs of a recipe (R51); a hold (R65). Added on 2026-11-10 (tools/recipe `971b9b6`, used by the Recipe app): `updates --json` with what the newer recipe may use (R4, R35), `go-back` and the kept versions (R38), `storage` with the room of the earlier versions and `--clear-earlier` (R37); and in `89d4753` the events `plan`, `fetch`, `agent`, `done` and `failed` (R15, R52; `plan`, `fetch` and `agent` compiled and not run) |
| The agent | anything that lets Codex or Claude Code be started by the apps at all (X7); what it ran and changed, step by step (X8, D17); approval of a step (X10); honouring "use the trash" (D18) |

## Defects found on the way

Not gaps in the design but things that are wrong as built.

| # | What is wrong |
|---|---|
| S1 | A failed run's reason is replaced by the main page at once |
| S4 | After the next start is set and the restart fails, the app is stuck and says "The change was not made". Fixed on 2026-11-10: a set restart is its own state, with "Restart now" and "Cancel the set restart" |
| S5 | "Continue" runs the wrong command for an interrupted change |
| R1 | "Remove…" does nothing for software whose recipe is gone |
| R38 | "Go back to an earlier version" calls a command Recipe does not have. Fixed on 2026-11-10: `recipe go-back` exists |
| D5 | Closing the window during a scan destroys a running thread |
| D31 to D33, D79 | Sentences that are untrue in the Disk app |
| S44 | "Finish and restart" does not restart |

## Across all three apps

Found by reading the three apps side by side, the shared code in `gui/kit`, and
the READMEs of Recipe and recab. Weight: **core** blocks a main job, **hurts**
makes it worse, **polish** is finish.

### The system and its software are split the wrong way

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence |
|---|---|---|---|---|---|
| X1 | page | Install software for the whole system the way the locked system really takes it. On an A/B system root's `recipe install` goes into the next system and takes effect after finishing and a restart. The Recipe app installs with `--package-root /var/lib/recipe`, a separate profile, and then says "installed" and offers "Open" | exists (`recipe --system install`, `recab change install`) | core | `tools/recipe/README.md` "System software on immutable A/B installations"; `gui/recipe/src/client.cpp:29` |
| X2 | state | See that an install or removal is pending until the restart. `recipe installed` reports active and prepared receipts; the Recipe app shows one flat "Installed" list | exists (`recipe --system installed`, `isinstalled --prepared`) | core | `tools/recipe/README.md:53`; `gui/recipe/src/pages.cpp` InstalledPage |
| X3 | action | Choose "for me" or "for everyone on this computer" when installing. For me is immediate and needs no password; for everyone is a system change | exists (the person's own store, and `--system`) | core | `gui/recipe/src/client.cpp` (one store, from an environment variable) |
| X4 | navigation | Get from the Recipe app to the System app when what was asked for is a system change, and back from a change in the System app to the recipe it installed | possible today | hurts | `gui/recipe/src/pages.cpp:786` (a sentence, no link) |
| X5 | page | See in the System app which software the next system gains or loses. Only the last change is named | partly (`recab change list`, `recipe --system installed`) | hurts | `gui/recab/README.md` "Not done" |

### The agent has no home

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence |
|---|---|---|---|---|---|
| X6 | settings | Choose which agent does the work (Codex, Claude Code), see whether it is installed and signed in. It is an environment variable in each app, default `codex` | missing everywhere | core | `gui/recipe/src/client.cpp:16`, `gui/recab/src/client.cpp:11`, `gui/disk/src/window.cpp:30` |
| X7 | action | Have any real agent do anything. The apps hand the prompt over as one JSON line and expect events back; no agent speaks that, and nothing stands between | missing (an adapter) | core | the three READMEs, "Not done" |
| X8 | page | See what the agent did: which commands it ran, which files it changed, with a way to read the difference before accepting | missing | core | `gui/kit/work.cpp` (a text box of output) |
| X9 | action | Say something to the agent while it works, beyond picking one of its offered answers: correct it, add a fact, tell it to stop and explain | missing | hurts | `gui/kit/work.cpp` `ask()` |
| X10 | action | Approve or refuse a step before the agent runs it as administrator | missing | core | no approval anywhere; `pkexec` only wraps whole commands |
| X11 | page | Find an earlier request again: what was asked, what the agent answered, when, in which app. Nothing is kept once the page changes | missing | hurts | no history in any app |
| X12 | state | Know that an agent is already working in another of the three windows. Two can change the same system at once | missing (no lock) | hurts | no `QLockFile` or the like in `gui/` |
| X13 | action | Change the issue or the facts before handing over. Only the expected result can be edited (and, in two places, a request) | possible today | hurts | `gui/kit/parts.cpp` PromptPanel |
| X14 | feedback | See what a hand-over will cost or how long it may take, and that it is still alive when it prints nothing | missing | polish | `gui/kit/work.cpp` |

### Nothing happens when no window is open

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence |
|---|---|---|---|---|---|
| X15 | feedback | Be told that software updates exist, that a new release of the system exists, that the last restart into a new system was kept or fell back, or that the disk is nearly full. No app runs in the background and none sends a notification | partly (the data exists when a window asks) | core | no tray or notification code in `gui/` |
| X16 | feedback | See after a restart, without opening the System app, whether the change took | exists (`last_trial`) | hurts | same |
| X17 | state | Have a window notice a change made elsewhere while it is in front. It reads again only on F5 or on coming back to it | possible today | polish | `gui/kit/refresh.cpp` |

### What every app lacks

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence |
|---|---|---|---|---|---|
| X18 | settings | Change any setting in any app. There is no settings page and nothing is remembered between starts except the Disk app's last comparison | possible today | hurts | no `QSettings` in `gui/` |
| X19 | navigation | Use a menu, or any key other than F5 and Ctrl+R: no search key, no quit key, no way through the pages by keyboard alone | possible today | hurts | `gui/kit/refresh.cpp:13` is the only shortcut |
| X20 | page | Read what the app is for: no first-run explanation of the locked system, the next system, sources or the agent; no help; no About | possible today | hurts | no help or about code |
| X21 | feedback | Copy an error, or find the log behind it. Backend messages are shown as text with nowhere to go | possible today | hurts | `showProblem` in each app |
| X22 | state | Understand the password prompt before it appears: which command will run as administrator and why. The prompt has never been run | partly (polkit actions exist for Recipe and recab; the Disk app has none of its own) | hurts | `gui/*/data/*.policy` |
| X23 | state | Use the apps on the live desktop ISO, before installing: recab is not set up there and Recipe would install into memory. No app says so | missing | hurts | not handled in any app |
| X24 | feedback | Use the apps in another language. Every string is marked for translation and no translation exists | possible today | polish | no `.ts` files |
| X25 | navigation | Reach the drawn parts by keyboard or screen reader: the disk maps, the step marks, the cards' pictures | possible today | hurts | five `setAccessibleName` calls in all of `gui/` |
| X26 | navigation | Open one app at the right place from another ("show this recipe", "show the next system"), or from a notification | possible today | polish | no arguments but the checkout ones in any `main.cpp` |
| X27 | state | Run the shipped programs without the checkout's switches: `--look`, `--shots`, `--try` and `--real` are in the binaries | possible today | polish | each `main.cpp` |

### What this points to

1. **Software and the system are one thing on a locked system, and two apps.**
   A system-wide install is a change to the next system (X1 to X5). The split
   "Recipe installs, System changes" does not hold; the real split is "for me,
   now" against "for this computer, after a restart".
2. **The agent is the product's centre and has no place of its own.** Each app
   rebuilds a hand-over page and a work page; none can show what the agent
   did, keep a history, take a correction or ask for approval (X6 to X14).
3. **Three windows, no shared ground.** No common settings, history,
   notifications or way from one to the other; three different shapes (a
   sidebar, one page that changes, two tabs) for the same kinds of step.
4. **Everything is pull.** The apps show a state when asked. What a person
   most needs to be told (it worked, it failed, there is an update, the disk
   is full) reaches nobody (X15, X16).

## Recipe (`gui/recipe`)

From an independent read of the app's source against every `recipe` command
and its `--help`, `tools/recipe/README.md`, the two design documents and
`docs/desktop-todo.md` section 6. 65 gaps.

### Core: blocks a main job

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| R1 | action | Remove software whose recipe is gone from the catalog (its source was removed, or the recipe deleted or renamed). "Remove…" quietly does nothing. The source-removal question even promises such software "stays installed" | exists (`remove` works from the receipt alone) | `window.cpp` `askRemove` |
| R2 | state | Open an installed program that has no recipe: a dead end, "No recipe named X", with no way back and no action | exists (`installed` gives the receipt's `recipe` path) | `pages.cpp:275` |
| R3 | action | Be stopped at the trust step. Install and Update on Browse start the run at once and skip the page that says what the recipe may use | UI only | `pages.cpp:232` |
| R4 | feedback | Be stopped or warned when a newer recipe asks for more than before. "Update all" queues everything; every row reads "Not compared" | partly (`recipe diff OLD NEW` exists; `updates --json` does not) | `window.cpp:156`, `pages.cpp:799` |
| R5 | action | See what an update changes. The receipt names the recipe file it was installed from and `recipe diff` compares two recipes; the app reads neither | exists (`diff`, the receipt's `recipe`) | `client.cpp:84` |
| R6 | action | Get a newer release of a program. `update` only installs the version the recipe pins. Under the product's premise this is the agent's job, and no button asks it | missing in recipe; the agent route is UI only | `recipe update --help` |
| R7 | state | See which recipes could not be read and why, and hand them to the agent. They are a count in the status bar | exists (`problems` in `list --json`) | `client.cpp:72`, `window.cpp:189` |
| R8 | feedback | See progress, output or a Stop for remove, go back, add or remove a source and fetch. The window is disabled; a failure is a message box with no hand-over | exists (`--machine-events`) | `window.cpp` `quietChange` |
| R9 | action | Install into one's own store without the administrator's password. Every change goes through `pkexec` into `/var/lib/recipe` | exists (a person's own store) | `client.cpp:14`, `:162` |
| R10 | state | Know whether software goes into the running system or the next one, see what is prepared but not active, or get to the System app to finish it | exists (`--system`, `--prepared`) | `client.cpp:28`; see X1 to X3 |
| R11 | action | Install from a remote source: the run stops with a message, and Browse gives no sign beforehand | missing in recipe (step 3 of the sources design) | `docs/recipe-sources.md` section 5 |
| R12 | navigation | Get back to a run after leaving its page. The status bar's label cannot be clicked | UI only | `window.cpp:73`, `:312` |
| R13 | feedback | Stop a change that runs through `pkexec`: the stop signal goes to a root process the person may not signal. Not verified | partly | `kit/run.cpp:58` |
| R14 | action | Read, open or copy the recipe file itself, though its path is known | exists (`path`, `info`) | `model.h:51` |
| R15 | action | See and approve what the agent changed. It is one summary line after the run; the event's `files` are ignored | missing in recipe (no `agent` event, no hold before use) | `pages.cpp:537` |
| R16 | settings | Choose or set up the agent. "Ask for software" uses an environment variable; "Let the agent fix it" uses Recipe's own `llm.toml`. The two can differ, and neither is checked | exists (`--llm-profile`, `--autofix-attempts`) | `client.cpp:16` |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| R17 | action | Report a program that installed but does not work. The hand-over exists only for failed runs | UI only | `pages.cpp:598` |
| R18 | feedback | Give the agent the run's output. The prompt says "N lines" and gives neither the lines nor the log folder | exists (the invocation's log folder) | `pages.cpp:624` |
| R19 | action | Ask why the sandbox refused something | exists (`recipe explain`) | `recipe explain --help` |
| R20 | feedback | See the limits that will really apply, and what the dependencies may use. The page shows `NEEDS` as written | exists (`inspect --json`) | `pages.cpp:337` |
| R21 | state | See before installing what else will be installed. Only build dependencies appear, inside a sentence | exists (`deps`) | `pages.cpp:85` |
| R22 | state | Tell libraries from programs on Installed, see what needs a library, or be warned before removing one others need | partly (no reverse query) | `pages.cpp:687` |
| R23 | state | See both recipes when two sources hold one name. The first is shown, the other hidden | exists (`SOURCE:NAME`) | `client.cpp:73` |
| R24 | state | Act on a source with a problem: no hand-over, no hint; a folder source has no "Read again" | partly (`problem`) | `sources.cpp:57` |
| R25 | feedback | Be told what adding a source means: trusting its keeper. No preview of its recipes, no branch, no check before root changes anything | exists (`source add --branch`) | `sources.cpp:28` |
| R26 | action | Switch the shipped source off, or one that was switched off back on | exists | `sources.cpp:75` |
| R27 | action | Install or open a `.rhai` file from the file manager or a dialog | exists (`install ./file.rhai`) | `main.cpp`, the menu entry |
| R28 | navigation | Be sent into the app at a place: a recipe, Updates, a source, a failed run | UI only | `main.cpp` |
| R29 | feedback | Be told an update exists, or that a long build finished while the window was hidden | missing in recipe (`updates`, no scheduler) | README "Not done" |
| R30 | page | See the history: what was installed, updated or removed, when, how it ended, and whether an agent did it | partly (the last 32 invocation logs are kept) | no such page |
| R31 | state | Have a run that was cut off (window closed, crash, restart) noticed at the next start and offered to continue | exists (resumable state) | `window.cpp:164` |
| R32 | feedback | Be warned when closing the window during a run. The run is killed, or orphaned under `pkexec` | UI only | no `closeEvent` |
| R33 | action | Queue a second install. "Update all" keeps a hidden queue, empties it at the first failure, and gives no summary | UI only | `window.cpp:312`, `:333` |
| R34 | feedback | Update all with one password. It asks once per update | partly | `window.cpp:156` |
| R35 | state | Trust "Update to": without `updates`, any differing version counts, including an older one | missing in recipe | `client.cpp:125` |
| R36 | action | Repair or reinstall a program, or check that it is intact | exists (`isinstalled`, `--force-install`) | `recipe isinstalled --help` |
| R37 | page | See what Recipe itself keeps on disk (build caches, kept generations, fetched sources) and clear it | partly (`cleanup` per recipe) | `pages.cpp:663` |
| R38 | action | Go back to an earlier version. The button calls a command that does not exist and shows only for a field recipe never sends | missing in recipe (`go-back`, `kept`) | `client.cpp:195` |
| R39 | action | Remove a program's settings and data in the home folder with it | missing in recipe | `pages.cpp:907` |
| R40 | action | Open a program properly. The recipe page offers "Open" for command-line tools, started with no terminal, and always starts the first file in `bin/`, not the menu entry | partly | `pages.cpp:294`, `window.cpp:468` |
| R41 | feedback | Be told an install succeeded, and told again when a restart or a new login is needed | UI only | `window.cpp:341` |
| R42 | navigation | Go back to where one came from. "‹ All software" always goes to Browse | UI only | `pages.cpp:284` |
| R43 | feedback | Know the computer is offline. A failed fetch is a message box and the kept list is used silently | partly | `window.cpp:446` |
| R44 | state | Find a way forward with no sources or no recipes: "Nothing here." | UI only | `pages.cpp:238`, `sources.cpp:83` |
| R45 | state | Be led from an empty Installed page to Browse, or see when updates were last checked | partly (`checked`) | `pages.cpp:678`, `:787` |
| R46 | action | Answer the agent during "Let the agent fix it" | missing in recipe | README "Not done" |
| R47 | state | See that a recipe was written by the agent and not yet read by a person, or ask the agent to change or delete it | UI only | `window.cpp:112` |
| R48 | feedback | Be told before handing over that the agent program is missing | UI only | `client.cpp:226` |
| R49 | action | Copy a recipe into one's own folder and change it, or publish one to a source | partly (the own folder; `index`) | README "Not done" |
| R50 | action | Report a broken recipe to its source's keeper | missing in recipe | `docs/recipe-sources.md` |
| R51 | feedback | See how much will be downloaded and how much disk an install needs. Only upper limits are shown | missing in recipe | `pages.cpp:339` |
| R52 | feedback | See real progress. `fetch`, `progress`, `plan` and `say` are read but never sent, so the bar has no end and the steps are guessed | missing in recipe | `pages.cpp:475` |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| R53 | action | Search by what a program does. Search reads name, title and description only | exists (`about`) | `pages.cpp:199` |
| R54 | action | Filter Browse to installed or not, or by licence; sort it | UI only | `pages.cpp:194` |
| R55 | action | Sort or search the Installed and Updates tables | UI only | `pages.cpp:130` |
| R56 | settings | Set preferences: agent, store, how often sources are fetched, limits (`--no-network`, `--memory-mib`, `--jobs`) | exists (global options) | none in `window.cpp` |
| R57 | navigation | Use a menu, Help or About, or reach the manual | UI only | `main.cpp` |
| R58 | navigation | Use Ctrl+F, Escape or Alt+Left, Ctrl+Q | UI only | `kit/refresh.cpp:12` |
| R59 | settings | Find the window as it was left: size, page, chosen source | UI only | no `QSettings` |
| R60 | feedback | Use it with a screen reader: every Install button reads "Install", step marks are drawn with no text, Browse is a grid of labels, failure is shown only in red | UI only | `pages.cpp:232` |
| R61 | state | See a library as a library when opened by name: it shows an "Install" page like an app | UI only | `pages.cpp:302` |
| R62 | feedback | Tell "Read again" (local) from "Check for updates" and "Fetch" (network). The difference is only in tooltips | UI only | `window.cpp:130` |
| R63 | feedback | See a program's own icon, or the app in another language | missing in recipe (icons) | README "Not done" |
| R64 | action | See the licences of the dependencies | partly | `pages.cpp:372` |
| R65 | action | Hold a program at its version | missing in recipe | `recipe --help` |

### What this points to

1. **Runs are not things the app keeps.** A run exists while its page shows
   it. Leaving loses it (R12), quiet changes have no progress (R8), nothing is
   queued (R33), a cut-off run is forgotten (R31), there is no history (R30).
   It needs a list of runs that pages link to.
2. **The agent has no place of its own.** Two unrelated flows, configured
   apart (R16), its changes never approved (R15, R47), and absent from the
   other maintenance jobs: unreadable recipes (R7), source problems (R24),
   broken programs (R17), newer releases (R6).
3. **The app does not decide where software goes** (R9, R10, R34): the
   person's own store, the root store, or the next system.
4. **Installed and catalog are joined by name and assume a recipe exists.**
   That breaks removal and the page (R1, R2), hides duplicates (R23), and
   leaves dependencies and libraries without a place (R21, R22, R61). A
   program's page should start from the receipt.
5. **Trust is one page that can be skipped** (R3). Updates cannot compare
   (R4, R5), real limits are not shown (R20), adding a source has no warning
   (R25). Every install, update and source needs to pass one check, fed by
   `inspect` and `diff`.
6. **The app uses five of recipe's commands.** `inspect`, `diff`, `explain`,
   `isinstalled`, `cleanup`, the receipt's `recipe` path and the logs are
   unused, while it calls `updates` and `go-back`, which do not exist. R5,
   R18, R19, R20, R36 and R37 need no work in Recipe, only in the app.
7. **Navigation is flat:** no back (R42), no deep links (R28), no menus or
   keys (R57, R58), nothing remembered (R59).

## System (`gui/recab`)

From an independent read of the app's source against every `recab` command
and its `--help`, recab's README and REQUIREMENTS, its generated status types
and the terminal app `recab-tui`. 58 gaps. Paths are in `gui/recab/src` unless
they say otherwise.

### Core: blocks a main job

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| S1 | feedback | Read why a long run failed. The "It stopped" page is replaced by the main page as soon as the status is read again | exists (recab's error message and class) | `kit/work.cpp` `onFinished`; `window.cpp:57` |
| S2 | state | See that the running system is a trial that is not kept yet. It is shown as a normal locked system and offers "Make a change", which recab refuses | exists (`committed`, `scheduled_trial`) | `model.cpp` never reads `committed` |
| S3 | action | Keep the running system by hand. The terminal app can | exists (`recab commit`) | `recab commit --help` |
| S4 | state | Recover when "Restart into it" set the next start but the restart itself failed or its password was refused. The button is greyed out with "There is nothing to do", and the warning says "The change was not made", which is false | partly (`trial_pending`) | `window.cpp:280`, `:213` |
| S5 | action | Continue the right thing after an interruption. "Continue" always runs `recab prepare`; a change cut off in the middle needs `recab change recover`, and recab refuses other work until then | exists (`recab change recover`) | `client.cpp:84` |
| S6 | state | Tell "recab is working right now" (the agent, a terminal, the Recipe app) from "it was interrupted". Every phase but two is shown as interrupted, with Discard on offer | missing in recab (no "a command is running" in status) | `model.cpp:161` |
| S7 | page | List the next system's changes. Only the last one is shown, and the agent is told "the next system already holds: X" when it may hold more | exists (`recab change list`) | `model.cpp:148` |
| S8 | action | Update the system to a new release. The row is always off with a fixed sentence | exists (`recab stage --from-media`, `recab release import`) | `model.cpp:217` |
| S9 | feedback | Be told that a new release exists | missing in recab (discovery and download are not built) | recab README |
| S10 | action | Review what the agent did before finishing. After an agent's run on the real recab the window goes back to the main page; the expected result is never checked | missing in recab (no proposal); partly (`recab change check`) | `window.cpp:239` |
| S11 | feedback | See what a successful run did: the output, the agent's summary, the synchronization report are dropped | exists (`preparation.synchronization`, `last_change.report`, `finalization`) | `window.cpp:57` |
| S12 | action | See exactly what will be different after the restart: neither the boot selection before and after, nor the files or packages | exists for the boot selection (`ActionView.after`, ignored); partly for content | `model.cpp:193` |
| S13 | state | Know with what rights the agent runs. It is started as the desktop user, and every recab change needs root | the app's decision | `client.cpp:100` |
| S14 | page | Learn how to go back when the desktop does not come up at all: the boot menu entry for the other system, `recab rollback` from a console | exists (rollback, boot entries) | recab README:315 |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| S15 | state | Be told plainly "this computer has one system only", "recab is not set up", "the installation is unfinished", "no UEFI". All end on "The system could not be read" with raw text | exists (the error's class; `init --mode mutable`) | `client.cpp:43` |
| S16 | state | See recab's own reason for a needed recovery | exists (`recovery_reason`) | never read |
| S17 | feedback | Read the system without a password prompt each time. After "Read it as administrator" every read asks again, every change asks again, a restart costs two prompts, and the restart is not covered by the polkit action | partly | `window.cpp:77`; the policy file |
| S18 | state | Know that once the next system is ready, any restart starts it, not only the button | exists (`trial_pending`, `next_boot`) | `pages.cpp:129` |
| S19 | action | Cancel a set restart into the next system and keep starting the running one | exists (`recab rollback` with a trial pending) | `pages.cpp:251` |
| S20 | action | Set the next system and restart later; go back without restarting now | exists (`set-next`, `rollback` without the restart) | `window.cpp:280` |
| S21 | action | Make the other partition an exact copy when there is no next system. Reset shows only with a next system | exists (`recab prepare`) | `pages.cpp:297` |
| S22 | state | Trust the reset. It leaves an open next system that cannot be started, not the ready second copy there was before | partly (no "make a ready copy" command) | `window.cpp` `reset` |
| S23 | feedback | Be told after a restart whether it was kept or came back, or that the agent asks something while the window is behind others | exists (`last_trial`); no push | no notification code |
| S24 | page | See why a start failed: the failed units, that start's journal | partly (`last_trial.detail`) | `pages.cpp:161` |
| S25 | state | Rely on the failed-start page appearing. It needs the preparation to still be there, ready, after the fallback; whether the real recab leaves it was never checked | partly | `model.cpp:184` |
| S26 | page | See the history of changes and restarts, and who or which agent made them | partly (`/var/lib/recab/audit.log`, no actor) | no such page; `recab-tui` has one |
| S27 | page | Inspect or export logs: the agent's transcript, recab's output, the audit log, the trial record | partly | `kit/work.cpp:62` |
| S28 | feedback | See what a change cost in disk space, or the free space on either slot | partly (the reports' byte counts; no free space) | status types |
| S29 | feedback | See real progress of a preparation | partly (`completed_steps`, the counters; no live events) | `kit/work.cpp:53` |
| S30 | state | Know the two systems by more than the running system's name: generation, kernel, readiness, device. After a staged release the names would be wrong | exists (generation, `seal.kernel_release`, `readiness`, `device`) | `model.cpp:111` |
| S31 | action | Know what Stop does. On a recab command through `pkexec` it probably cannot stop recab; on the agent it stops the agent, not a recab it started. Not verified | partly | `kit/work.cpp:38` |
| S32 | action | Be asked before closing the window ends an agent in the middle of a change | UI only | no `closeEvent` |
| S33 | feedback | Have the window notice a change made elsewhere while it is in front | exists (`/var/lib/recab/state.json` could be watched) | `kit/refresh.cpp` |
| S34 | action | Be warned about unsaved work in other programs before a restart, or restart in a few minutes | UI only | `window.cpp:280` |
| S35 | action | Install or remove software for the system directly, or get to the Recipe app | exists (`recab change install|update|remove`) | `pages.cpp:281` |
| S36 | navigation | Get to the Recipe app and the Disk app. Recipe can also add changes to the same next system without this window showing more than the last | UI only | none in `gui/recab/src` |
| S37 | settings | Choose the agent, what it may run, the restart command. The status bar always says "Agent: codex" | UI only | `client.cpp:7` |
| S38 | action | Approve or refuse the agent's single steps, or see which commands it ran | missing | README |
| S39 | feedback | Find an agent's question again after the run ended or the window closed | UI only | `kit/work.cpp:195` |
| S40 | action | Hand an interrupted preparation, a refused command or a failed run to the agent. Only a failed start and a clash build a prompt | exists (the error's class and message) | `pages.cpp:241` |
| S41 | feedback | Understand a refusal. It is a box with recab's raw message, with no guidance by kind of error | exists (error classes) | `window.cpp:218` |
| S42 | feedback | Know what going back does not undo: data in `/var` and what programs changed in their own data | exists (in recab's requirements) | `window.cpp:288` |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| S43 | feedback | Stop seeing "You are on the new system" some day. It stays until the next trial, with no date | exists (`last_trial.unix_time`) | `pages.cpp:143` |
| S44 | feedback | Trust "Finish and restart": it only finishes | UI only | `pages.cpp:268` |
| S45 | action | Edit a configuration file oneself | exists (`recab edit PATH --file`) | only told to the agent |
| S46 | page | See how the running system's start-up ended | exists (`recab health --json`) | unused |
| S47 | action | Change the flavour (desktop or base), or finish with another kernel | exists (`--change-flavour`, `finalize --kernel`) | unused |
| S48 | action | Have an inserted release medium noticed and offered | partly | `docs/recab-gui.md` |
| S49 | feedback | Be told that "Make a change" on a ready next system throws its finishing away and cancels the restart | exists (the verdict's `after`) | `pages.cpp:255` |
| S50 | feedback | Tell the agent everything recab offers: the prompt leaves out `change list`, `check`, `recover` and `recipe --system` | exists | `pages.cpp:339` |
| S51 | settings | Use a menu, Help, About, or read what A and B are | UI only | `window.cpp` |
| S52 | settings | Use keys for the main actions, or Esc for Back | UI only | `kit/refresh.cpp` |
| S53 | settings | Use it with a screen reader or by keyboard: cards and pictures carry the state by drawing and colour, "‹ Back" may not take focus | UI only | `pages.cpp:52` |
| S54 | settings | Find the window as it was left; use another language | UI only | `main.cpp:33` |
| S55 | state | Reach the clash and proposal pages with the real recab. They exist only for the stand-in | missing in recab | `model.cpp:41` |
| S56 | state | Decide several clashes safely: one box per file, never run, and cancelling midway still applies the earlier answers | missing in recab | `window.cpp` `decideByHand` |
| S57 | feedback | See what the next system holds when a restart is set and no preparation is left | partly | `model.cpp:169` |
| S58 | navigation | Read another installed system, or the full inventory (for repair) | exists (`status --root`, `inventory --root`) | unused |

### What this points to

1. **One page that changes shape, with rules it was not meant to hold.** The
   app turns recab's slots and preparation into six states of its own. That
   mapping is lifecycle logic. It has no "trial running, not kept", no
   "something is working elsewhere", no "one system only", no "restart set
   but not made", and cannot tell one interruption from another (S2, S4, S5,
   S6, S15). recab should report the lifecycle and the next steps; the app
   should draw them.
2. **Work in progress is a dead end and its result is thrown away.** A run
   takes the whole window; success and failure both end on the main page
   (S1, S11). Nothing records what ran, what it made, what failed or what it
   cost.
3. **The agent is bolted on:** behind three buttons, with undefined rights
   (S13), no settings (S37), no history (S39), and its result never checked
   against what was expected (S10). Refusals and failures do not hand over
   (S40).
4. **What recab lacks is hidden, and what it has is unused.** Absent though
   recab has them: `commit`, `stage --from-media`, `change list`, `check`
   and `recover`, `ActionView.after`, `recovery_reason`, `committed`, the
   slots' generation and device, the reports, `health`.
5. **No sense of time:** one read of now, no history, no notice after a
   restart, a permanent "you are on the new system" (S23, S26, S43).
6. **Restart is treated as one step and is two.** Choosing what starts next
   and restarting are chained, with no later, no cancel, and a bad end when
   the second half fails (S4, S19, S20).
7. **The way out lives outside the app.** Getting back when the desktop does
   not come up happens in the boot menu or a console, and the app never
   prepares the person for it (S14).

## Disk (`gui/disk`)

From an independent read of the app's source, recab's and recpart's READMEs,
and what QDirStat, WinDirStat, Baobab, Filelight, ncdu and BleachBit offer. 81
gaps. Paths are in `gui/disk/src` unless they say otherwise. "Possible today"
means the app could do it with what it already reads or with read rights.

### Core: blocks a main job

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| D1 | state | See the partition that really fills up: the state partition, which holds `/var` and `/home`. The app reads the home folder and `/var/lib/recipe`, so it can say "No mess" while `/var` is the cause | possible today (read rights or a root helper) | `window.cpp:41`, `scan.cpp:55` |
| D2 | page | See the partitions: EFI, the two systems, state, swap; each with size, used, free | possible today; the other system's fill needs recab | `window.cpp:194` |
| D3 | state | Be told which folders could not be read. They are collected and thrown away, so totals are too low without a word | possible today | `scan.cpp` `unreadable()` is never used |
| D4 | state | Have the places that grow looked at: logs and the journal, `/var/tmp`, core dumps, container and virtual machine images, the swap file | possible today (read rights) | no rule in `mess.cpp` |
| D5 | state | Stop a scan, or close the window during one without a crash. There is no Stop, and a running scan thread is destroyed with the window | possible today | `pages.cpp:46`, `window.cpp:177` |
| D6 | state | Be warned that the disk is nearly or wholly full, with what to do first | possible today | `pages.cpp:90` |
| D7 | state | See what the agent did. After it succeeds the window looks again and jumps to the map; its output, answers and summary are gone | possible today (keep the run) | `window.cpp:421` |
| D8 | state | Know before handing over that the agent program is missing or cannot speak to the app. It ends as "It stopped" | needs the agent (a launcher) | `window.cpp:30` |
| D9 | state | Close the window safely while the agent works. It is killed partway through removing, without a question | possible today | no `closeEvent` |
| D10 | state | Be told "this computer has one system only" as a fact. It is shown as an error with "Try again"; with the real recab in single-system mode the other is even named "The second copy" and Compare is offered | needs recab (the mode in status) | `systems.cpp:583` |
| D11 | state | Compare at all with the real recab. Compare always ends in a refusal, and "Try again" leads nowhere | needs recab (`recab compare`) | `systems.cpp:619` |
| D12 | action | Get one answer to "my disk is full, what do I do now": which partition, which three things free the most, go | possible today, with D1 | `pages.cpp:75` |
| D13 | navigation | Zoom into a folder on the map. The path line only moves the selection; below a depth or size, or inside a finding, the map draws one block | possible today | `map.cpp:61` |
| D14 | navigation | Go up, go back, double-click to go in, or move by wheel or keyboard on the map | possible today | `map.cpp:217` |
| D15 | page | Browse a folder tree with sizes beside the map, the core of every such tool. The list shows only what the rules named | possible today | `pages.cpp:126` |
| D16 | feedback | Be told whether it is one's own files, software, or the system that takes the room | possible today for home and Recipe; needs recab for the systems | `pages.cpp:90` |
| D17 | action | See, path by path, what the agent removed, kept, or asked about and was refused | needs the agent (an event per path) | README, the agent's events |
| D18 | action | Get anything back. The prompt never asks for the trash, and there is no undo | needs the agent to honour it; possible today in the prompt | `pages.cpp:351` |
| D19 | action | Act on a difference between the two systems: make them the same again. The agent is told to change nothing, and "Open System…" passes nothing along | needs recab; possible today as a hand-over to the System app | `systems.cpp:431` |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| D20 | feedback | Tell how much of the disk was looked at. "X of Y looks like mess" uses the bytes scanned, not the disk | possible today | `pages.cpp:207` |
| D21 | state | See other people's home folders and root's | possible today with a root helper | `window.cpp:41` |
| D22 | state | See the EFI partition, where a full disk blocks the next system change | possible today | recpart README |
| D23 | state | See how full each system partition is and whether the next change will fit | needs recab (slot size and use in status) | `systems.cpp:506` |
| D24 | action | Point the scan at another folder or a removable drive | possible today | `window.cpp:41` |
| D25 | state | Know which of Recipe's kept versions is in use and which are old. The rule goes by a folder's name | needs Recipe (a listing with in-use marks; a prune command) | `mess.cpp:86` |
| D26 | feedback | Trust "At least X is free again": it counts Recipe's folders, which are to be cleared "through Recipe" by a command nobody checked exists | needs Recipe | `pages.cpp:351` |
| D27 | state | See how far a scan is, and which folder is being read | possible today | `pages.cpp:58` |
| D28 | state | Open the app without waiting: every start scans from nothing | possible today | `window.cpp:135` |
| D29 | state | Learn where the room went when no mess is found: "No mess worth naming" and nothing more | possible today | `pages.cpp:207` |
| D30 | feedback | Trust "freed": it is scanned bytes before minus after, not the change in free space | possible today | `window.cpp:191` |
| D31 | feedback | Trust "Nothing has been removed" after an agent's run that freed nothing net | possible today | `pages.cpp:90` |
| D32 | feedback | Trust "The running system was not touched" after a failed clearing run. The sentence is the System app's and untrue here | possible today | `kit/work.cpp:216` |
| D33 | feedback | Trust "What is done is kept; you can continue later" on Stop: every tick and pick is then forgotten | possible today | `kit/work.cpp:44` |
| D34 | state | Notice the agent's question when the window is behind others | possible today | `kit/work.cpp:174` |
| D35 | state | See the map while the agent clears, to check a path it asks about | possible today | `window.cpp:162` |
| D36 | state | Tell a kept comparison is old when recab cannot be read: it is drawn as if current | possible today | `window.cpp:246` |
| D37 | state | Look at the disk while a comparison runs for minutes. The other tab is switched off | possible today | `window.cpp:275` |
| D38 | state | Find the agent's explanation of the differences after a restart of the app. The comparison is kept, the explanation is not | possible today | `window.cpp:327` |
| D39 | feedback | Trust "% the same": it is by bytes, so thousands of small changed files still read 99.9%. That works against the goal of seeing that they look the same | possible today | `systems.cpp:694` |
| D40 | state | Reach the differences recab left out when there are thousands. The agent is given only the 25 biggest | needs recab (paging) | `systems.cpp:412` |
| D41 | page | List the biggest files | possible today | `scan.cpp:17` |
| D42 | page | See old files, or colour by age | possible today (`lastUsed` is read) | `scan.cpp:85` |
| D43 | page | See by file type | possible today | `scan.cpp:106` |
| D44 | action | Search for a file, a name or a type | possible today | no search |
| D45 | action | Sort the lists | possible today | `pages.cpp:127`, `systems.cpp:790` |
| D46 | action | Leave a folder out of future scans | possible today | no setting |
| D47 | action | Mark a finding "never suggest this again" | possible today | no ignore list |
| D48 | action | Add a rule of one's own. The rules are C++ | possible today (rules as data) | `mess.cpp:59` |
| D49 | action | Take a hand-picked item off the list. Every new look and every agent's run forgets the picks | possible today | `pages.cpp:274` |
| D50 | feedback | Trust the issue: it calls hand-picked, unknown things "leftovers" | possible today | `pages.cpp:348` |
| D51 | page | See the history of past clean-ups | possible today | nothing is written |
| D52 | action | Be warned before the disk fills, or have a check run on a schedule | possible today, with something running outside the window | no timer |
| D53 | page | See what changed inside a file whose content differs | needs recab (one path from both systems) | README "Not done" |
| D54 | action | Search or filter the differences | possible today | `systems.cpp:790` |
| D55 | navigation | Zoom both system maps into one folder together | possible today | `systems.cpp:162` |
| D56 | action | Have differences explained against what was changed: the agent is asked which change each belongs to and is given no list of changes | needs recab (the change list) | `systems.cpp:429` |
| D57 | settings | Set anything: the agent, the thresholds (20 MiB; 256 MiB and a year; 90 days), the places, what is left out, the rules | possible today | `mess.cpp:15`, `window.cpp:29` |
| D58 | navigation | Use a menu, Help, About, or keys for zoom, up, search, copy | possible today | `kit/refresh.cpp:12` |
| D59 | action | Right-click a block or a line: open, copy the path, add, leave out | possible today | no context menu |
| D60 | feedback | Read the map: no legend but the list's colours, no highlight under the pointer, everything unnamed one grey | possible today | `map.cpp:123` |
| D61 | navigation | Get from Recipe's own findings to the Recipe app | possible today | `mess.cpp:79` |
| D62 | feedback | Use it with a screen reader or keyboard: the maps cannot take focus, kinds are told apart by colour alone, "Picked by you" is nearly invisible on a light look | possible today | `map.cpp:13`, `mess.cpp:166` |
| D63 | action | Keep ticks and picks through "Look again", "Read again" and F5. They are dropped without a question | possible today | `window.cpp:117` |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence |
|---|---|---|---|---|
| D64 | state | Use anything while a scan runs | possible today | `pages.cpp:48` |
| D65 | state | Be sure a kept comparison is of this computer's disks | needs recab | README "Not done" |
| D66 | state | Be told a comparison was stopped. An older kept one quietly reappears | possible today | `window.cpp:299` |
| D67 | state | Read "identical" with its exception in the headline: "except N files that name their own partition" | possible today | `systems.cpp:840` |
| D68 | feedback | Be told the password was refused or cancelled | possible today | `systems.cpp:649` |
| D69 | action | Find duplicate files | possible in the app, with hashing the scanner does not do | `scan.cpp` |
| D70 | action | Open the file itself, copy its path, open a terminal there | possible today | `window.cpp:149` |
| D71 | action | Change the issue or a fact, or add an instruction ("use the trash") | possible today | `kit/parts.h:86` |
| D72 | action | Export or copy a report | possible today | README "Not done" |
| D73 | feedback | Know that a small difference is drawn bigger than it is. Nothing on the screen says so | possible today | `systems.cpp:197` |
| D74 | page | See earlier comparisons, before and after each change | possible today | `window.cpp:321` |
| D75 | feedback | Read more in a block's tip: files, last used, the rule that named it | possible today | `map.cpp:138` |
| D76 | navigation | Get from the Mess tab to the System app when the systems or EFI are the problem | possible today | `pages.cpp` |
| D77 | feedback | Notice that the System app failed to start: six seconds in the status bar | possible today | `window.cpp:97` |
| D78 | settings | Find the window as it was left | possible today | `main.cpp:34` |
| D79 | feedback | Trust the status bar: "This window only reads the disk", also while the agent it started removes files | possible today | `window.cpp:76` |
| D80 | feedback | See when each item was last used and why it is listed, without hovering | possible today | `pages.cpp:127` |
| D81 | action | Tick in bulk: only what clears without asking, none, only above a size | possible today | `pages.cpp:192` |

### What this points to

1. **Two unrelated jobs share one window, one run and one hand-over page.**
   "Where is the mess" (the person's things) and "are A and B the same" (the
   locked system) differ in data, rights, time and outcome, and block each
   other. The comparison may belong in the System app, where acting on a
   difference already lives (D19, D37).
2. **The app has no notion of the disk, only of the folders it reads.**
   Partitions, free space on each, and what fills first are absent, so "my
   disk is full" is answered for the home folder only (D1, D2, D6, D12, D16).
   The partitions should come first and the map be a way into one of them.
3. **It is half a disk-usage tool with a mess finder squeezed in.** No folder
   tree, zoom, up, sort, search, type or age view (D13 to D15, D41 to D45).
   What the rules miss can be found only by hovering.
4. **The rules are fixed in C++ and the app remembers nothing a person
   said:** nothing left out, no "never again", no own rules, picks forgotten
   at every look (D46 to D49, D63).
5. **The agent is a dead end on both sides.** Before: no check that it
   exists, no word about the trash. After: output thrown away, no record per
   path, no undo, no history, and "freed" is a guess (D7, D8, D17, D18, D30,
   D51).
6. **The comparison shows and never leads to doing** (D19, D53), rests on a
   command that does not exist (D11), shows normal facts as errors (D10), and
   its headline by bytes undercuts its own goal (D39).
7. **Sentences borrowed from the other apps are untrue here** (D31 to D33,
   D50, D79). Every page's claims need checking against what this app knows.
