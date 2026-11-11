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

| Part | Gaps | Core | Hurts | Polish | Done | Partly | Open |
|---|---|---|---|---|---|---|---|
| Across all three (X) | 27 | 8 | 14 | 5 | 7 | 12 | 8 |
| Recipe (R) | 65 | 16 | 36 | 13 | 15 | 19 | 31 |
| System (S) | 58 | 14 | 28 | 16 | 22 | 24 | 12 |
| Disk (D) | 81 | 19 | 44 | 18 | 17 | 36 | 28 |
| **All** | **231** | **57** | **122** | **52** | **61** | **91** | **79** |

**Core** blocks a main job, **hurts** makes it worse, **polish** is finish.
"Backend" says whether the command line already offered what the app would
need when the gap was found. 41 of the 231 needed something a backend did not
have at all; some of that has been built since (see the Now column). For the
other 190 the backend had it, had part of it, or the work is the app's own.

**Now (2026-11-11)**, the last column of each table, is where the gap stands
in the apps as built on 2026-11-11, first read by a reviewer who had not built
them with every story run, then brought up to date for the fixes of the same
day (commits `9f78336`, `b6477f8`, `01cf618`, `19630e5` and a last pass over
help, closing, the clash page, the panel of what the agent is given, Disk's
hand-over wording and System's merge commands), with every story run again. **Done** means a person can do it in the app against the
real backend and a story reaches it; **partly** says what is missing, and
counts anything that works only against a stand-in because the real backend
lacks it; **open** means nothing for it is built. The gap texts and their
evidence are as found on 2026-11-10.

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

Needed by the gaps above and not offered today (read 2026-11-11 against
`tools/recipe`, `tools/recab` and `gui/agent`). What a backend gained since
2026-11-10 is in the "Now" column of each gap.

| Backend | Lacks |
|---|---|
| recab | the single-system, "not set up", "installation unfinished" and "no UEFI" cases as facts in `status` or as error classes (S15, D10); each slot's size, use and free space, and whether a change fits (S28, D2, D23); progress events during prepare, finalize, merge and `compare` (S29); one command that makes the other slot a ready copy (S21, S22); undoing one change of the next system; an actor for history entries other than changes (S26); a configured release origin and keys, and detecting a release medium (S8, S9, S48); a place for an agent's proposed resolution before it is accepted (S55); the failed start's journal in `last_trial` (S24); one path's content from both systems, and paging beyond 5000 differences (D53, D40). Has and the app does not use: `health` (S46), `status --root` and `inventory --root` (S58), the seal's kernel and the generations (S30), `ActionView.after` (S12), `change check` (S50), `--change-flavour` and `finalize --kernel` (S47); `edit` is offered only in the recovery list; `replace_failed`, `adopt_baseline` and `update_reproduce` are also offered where a merge or a failed system needs them. A merge operation's argument order and names were checked against `tools/recab/src/main.rs` on 2026-11-11 and System sends recab's own `argv` with the inputs recab names after it |
| Recipe | `--autofix` on `update` (only `install` takes it, so a stopped update has no repair); passing the autofix agent's question to the front end (R46) and a hold before an agent's change is used (R15); progress inside a build step, and `say` and `output` events (`plan`, `fetch`, `agent` and `refused` are compiled, never run) (R52); download size and disk needed before a run (R51); sizes of build caches, fetched copies and run records, and a command that clears its caches and saved build steps (R37, D25, D26); a reverse "needed by" query (R22); `hold`, `go-back`, `storage`, `history` and `updates` on the system route of a locked computer (only install, update, remove, installed and isinstalled go through recab); `prepared` readable by a person, not only root (X2, R10); the actor in `history` (R30); newer upstream releases (R6), icons (R63), reporting to a source's keeper (R50), removing a program's home data (R39) |
| The agent (`levitate-agent`) | a structured event beyond questions, commands, per-path `place` results and its summary (the helper itself now reports a place from the agent's SET-ASIDE, KEEP and CANNOT lines and moves the places, D7, D17; a real agent writing those lines has never been tried; the helper counts a very large place's size before it moves it, which blocks its event loop); a correction or a fact told mid-run beyond answering its question (X9); cost or time (X14); a lock across windows (X12); records of installs, updates and restarts, so Activity lists agent runs only (R12, R30); notices for updates, restarts or a full disk, and any notice seen on a real desktop (X15, X16, D52); a real Claude Code run (only a stand-in); RUN-AS-ADMINISTRATOR, file changes and the real `pkexec` never run against a real Codex. `levitate-agent` is listed in the image build as a helper (2026-11-11, no image built); Codex and Claude Code themselves are in no image and have no recipe, and the helper needs `gdbus`, which the Acorn image appears to lack |

## Defects

Things that are wrong as built, as of 2026-11-11 after the last pass. Fixed
earlier: S4 (a set restart is its own state), S5 (Continue follows recab's own
recover operation), R38 (`recipe go-back` exists), D5 (closing waits for a
scan), D31 and D79 (the sentences are gone), S44 (the row says it is two
steps).

Fixed on 2026-11-11, first round:

| # | What was wrong | Now |
|---|---|---|
| S1 | A failed recab command's "It stopped" page was replaced by the main page at once, while Help said "the page says why" | the page stays; Back leads on; a story reaches it |
| S4 | A failed restart after `set-next` or `rollback` said "The change was not made" | it says the restart did not happen and the step before it was made; a story reaches it |
| R1 | "Remove…" did nothing for software whose recipe is gone | it asks and runs `recipe remove` from the receipt; a story reaches it |
| D32, D33 | A failed clearing run said "The running system was not touched"; Stop said "you can continue later" | the work page no longer says either; Stop says what was set aside stays |
| — | Recipe's "Let the agent fix it" sent `--autofix` with `recipe update`, which takes none (the stand-in accepted it) | the repair is offered for a stopped install only; a stopped update says Recipe cannot repair it yet; the stand-in refuses `update --autofix` as `recipe` does |
| — | Recipe's Updates page sent the person to "System Updates", which does not exist | "the System app" |
| — | Disk said the agent "works without administrator rights" (the helper lets it ask for an administrator command) and that it "reads both systems" (it is given recab's list) | both sentences say what is so |
| — | Disk named an other system recab calls "unknown" "The second copy" | "recab cannot say what it is" |
| — | System yanked a page the person had opened when a read started earlier came back (the story's Finish check failed on it) | a read redraws the page shown when it answers |
| — | recab's polkit message spoke only of changing, though Disk's comparison and System's "Read it as administrator" use the same action | it says read or change |

Fixed later on 2026-11-11:

| Where | What was wrong | Now |
|---|---|---|
| Disk, clearing | The page asked the agent for `{"event": "place"}` lines the real helper could not pass on, and sent no `may_write`, so with Settings at "ask" Codex was read-only and every move was a command put to the person | the agent only decides (SET-ASIDE, KEEP, CANNOT); the helper moves each approved place itself, never replacing and never following a link out of home, writes the manifest and reports each place; the agent is never given the right to write for it (`01cf618`) |
| Disk, Mess | "Recipe's own" folders were to be cleared "through Recipe"; `recipe` has no command for that | they are listed and never handed over; the page, Help and the first page say Disk leaves them alone |
| Disk, hand-over | The page said the helper moves the places and the agent is told to move nothing, whichever agent was chosen | it asks the helper which agent is chosen (`--settings`): for Codex or Claude Code it says the helper moves them, and when Settings let the agent run commands it says so; for another program it says that program moves the places and Disk only shows what it reports |
| Disk, comparing | "needs the administrator's password" also when no root helper is set (picture mode) | the page says so only when a root helper is set |
| System, release | `adopt_baseline`, `update_reproduce` and `replace_failed` appeared only in the recovery list: a merge blocked by a baseline or package-rebuild conflict was a dead end | each is offered where recab offers it, with recab's own label and through the Before page (`b6477f8`) |
| System, clash | With the real recab the agent's facts read "Prepared content (private)" and "Release content (private)", and "was added to the next system" named the prepared release | the page and the facts say what recab's merge reports for each kind of conflict and name the right release |
| System, clash | At 700 pixels high the agent's panel was clipped when a package-rebuild conflict was shown | everything under the heading scrolls |
| System, stand-in | `recab-real` offered `update_check` always ready, never offered `replace_failed`, `adopt_baseline` or `update_reproduce`, and always refused `change_recover`; it also took `update resolve`, `reproduce` and `cancel` without `--preparation` and `--revision`, and an edited resolution without `--file` | it gives each operation the verdict recab's `operations.rs` gives it, with switches for the states a story needs, and refuses what clap refuses (a missing `--preparation` or `--revision`, a missing package) and what `merge.rs` refuses (an edited resolution without `--file`, a file with any other choice) |
| Recipe, Open | started the first `bin/` file by its bare name through PATH, not the menu entry and not the store's profile | starts from the profile of the place the software is in, by its menu entry |
| Recipe, closing | closing the window during an install killed the `pkexec recipe` process without a question (R32) | it asks, and says what stops and what is kept |
| Recipe, facts | the agent was told "The output of the run: N lines" without the lines or the log folder (R18) | it is given the last lines and where the log is |
| All three, closing | a window that refused to close (Recipe's "close anyway?") still let go of the runs Activity follows | they are let go of only when the window really closes; livestory checks it |
| All three, Help | Help and the first pages said Recipe's folders are "cleared through Recipe", that the agent moves places, and that comparing always asks for the administrator's password; nothing said that merges can be settled by adopting, reproducing or replacing, that short changes have a page with Stop, that "Open" starts from the place the software is in, that a program without a recipe shows its receipt, or that a run goes on when its window closes | each page says what is so |
| All three, panel | the shared panel of what the agent is given left large empty gaps beside its text | each part has its name above it and the text has the full width |

Still wrong:

| Where | What is wrong |
|---|---|
| Disk, helper | A very large place blocks the helper while its size is counted before the place is set aside (`gui/agent/src/holding.cpp`). Counting it afterwards, or on another thread, would change the helper's order of work; left as it is and written in its README |
| Recipe, Open | ignores a menu entry's `DBusActivatable`, `TryExec` and `Path` |
| Disk, hand-over | for another program the page can only say what the request asks of it; whether that program asks before moving a place or before an administrator command is its own |

## Across all three apps

Found by reading the three apps side by side, the shared code in `gui/kit`, and
the READMEs of Recipe and recab. Weight: **core** blocks a main job, **hurts**
makes it worse, **polish** is finish.

### The system and its software are split the wrong way

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|---|
| X1 | page | Install software for the whole system the way the locked system really takes it. On an A/B system root's `recipe install` goes into the next system and takes effect after finishing and a restart. The Recipe app installs with `--package-root /var/lib/recipe`, a separate profile, and then says "installed" and offers "Open" | exists (`recipe --system install`, `recab change install`) | core | `tools/recipe/README.md` "System software on immutable A/B installations"; `gui/recipe/src/client.cpp:29` | **done**: Before page "For this computer" on a locked computer runs `recipe --system install`; stories p1-p3 (never run against a real recab) |
| X2 | state | See that an install or removal is pending until the restart. `recipe installed` reports active and prepared receipts; the Recipe app shows one flat "Installed" list | exists (`recipe --system installed`, `isinstalled --prepared`) | core | `tools/recipe/README.md:53`; `gui/recipe/src/pages.cpp` InstalledPage | **partly**: preparation.changes gives "after a restart" on Browse/program page; next-system rows only when root reads `prepared` |
| X3 | action | Choose "for me" or "for everyone on this computer" when installing. For me is immediate and needs no password; for everyone is a system change | exists (the person's own store, and `--system`) | core | `gui/recipe/src/client.cpp` (one store, from an environment variable) | **done**: Before page choice "For me" / "For this computer"; p1, p2 check route and no root helper |
| X4 | navigation | Get from the Recipe app to the System app when what was asked for is a system change, and back from a change in the System app to the recipe it installed | possible today | hurts | `gui/recipe/src/pages.cpp:786` (a sentence, no link) | **partly**: strip "Finish and restart…" and the rail start System; nothing leads from a System change back to its recipe |
| X5 | page | See in the System app which software the next system gains or loses. Only the last change is named | partly (`recab change list`, `recipe --system installed`) | hurts | `gui/recab/README.md` "Not done" | **done**: System "The next system" lists every change (recab status schema 7+) |

### The agent has no home

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|---|
| X6 | settings | Choose which agent does the work (Codex, Claude Code), see whether it is installed and signed in. It is an environment variable in each app, default `codex` | missing everywhere | core | `gui/recipe/src/client.cpp:16`, `gui/recab/src/client.cpp:11`, `gui/disk/src/window.cpp:30` | **done**: Settings page: agent rows, Choose…, Sign in/out via levitate-agent; f-settings |
| X7 | action | Have any real agent do anything. The apps hand the prompt over as one JSON line and expect events back; no agent speaks that, and nothing stands between | missing (an adapter) | core | the three READMEs, "Not done" | **done**: levitate-agent Codex adapter (two real Codex runs); Claude Code side never run against a real one |
| X8 | page | See what the agent did: which commands it ran, which files it changed, with a way to read the difference before accepting | missing | core | `gui/kit/work.cpp` (a text box of output) | **partly**: Activity "What it did" lists commands, writes, searches; no way to read the difference before accepting |
| X9 | action | Say something to the agent while it works, beyond picking one of its offered answers: correct it, add a fact, tell it to stop and explain | missing | hurts | `gui/kit/work.cpp` `ask()` | **open**: only answering a question (typed when it has no choices) or Stop; no correction mid-run |
| X10 | action | Approve or refuse a step before the agent runs it as administrator | missing | core | no approval anywhere; `pkexec` only wraps whole commands | **done**: Activity/work page Approve/Refuse for RUN-AS-ADMINISTRATOR; livestory (real Codex+pkexec path never run) |
| X11 | page | Find an earlier request again: what was asked, what the agent answered, when, in which app. Nothing is kept once the page changes | missing | hurts | no history in any app | **done**: Activity lists every agent run with request, what it did, how it ended, app filter |
| X12 | state | Know that an agent is already working in another of the three windows. Two can change the same system at once | missing (no lock) | hurts | no `QLockFile` or the like in `gui/` | **partly**: Activity badge counts running/waiting runs (fresh only while Activity is shown); no lock |
| X13 | action | Change the issue or the facts before handing over. Only the expected result can be edited (and, in two places, a request) | possible today | hurts | `gui/kit/parts.cpp` PromptPanel | **open**: PromptPanel still lets only the expected result be edited |
| X14 | feedback | See what a hand-over will cost or how long it may take, and that it is still alive when it prints nothing | missing | polish | `gui/kit/work.cpp` | **partly**: "What it does now" shows it is alive; no cost or time estimate |

### Nothing happens when no window is open

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|---|
| X15 | feedback | Be told that software updates exist, that a new release of the system exists, that the last restart into a new system was kept or fell back, or that the disk is nearly full. No app runs in the background and none sends a notification | partly (the data exists when a window asks) | core | no tray or notification code in `gui/` | **partly**: helper notices for an agent run that waits or ends (never seen on a desktop); nothing for updates, releases, restarts, full disk |
| X16 | feedback | See after a restart, without opening the System app, whether the change took | exists (`last_trial`) | hurts | same | **open**: no notice after a restart |
| X17 | state | Have a window notice a change made elsewhere while it is in front. It reads again only on F5 or on coming back to it | possible today | polish | `gui/kit/refresh.cpp` | **partly**: Activity re-reads every 2 s while shown; Recipe pages still only F5/coming back |

### What every app lacks

| # | Kind | What a person cannot do or see | Backend | Weight | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|---|
| X18 | settings | Change any setting in any app. There is no settings page and nothing is remembered between starts except the Disk app's last comparison | possible today | hurts | no `QSettings` in `gui/` | **partly**: Settings (agent, commands, notices) and window size kept; no per-app settings |
| X19 | navigation | Use a menu, or any key other than F5 and Ctrl+R: no search key, no quit key, no way through the pages by keyboard alone | possible today | hurts | `gui/kit/refresh.cpp:13` is the only shortcut | **partly**: rail by keyboard, F5/Ctrl+R, Escape on Before/hand-over; no menu, no quit/search keys |
| X20 | page | Read what the app is for: no first-run explanation of the locked system, the next system, sources or the agent; no help; no About | possible today | hurts | no help or about code | **partly**: Help page and first page in all three (helpstory); no About, no manual |
| X21 | feedback | Copy an error, or find the log behind it. Backend messages are shown as text with nowhere to go | possible today | hurts | `showProblem` in each app | **open**: errors still shown as text/message boxes; no copy, no log link |
| X22 | state | Understand the password prompt before it appears: which command will run as administrator and why. The prompt has never been run | partly (polkit actions exist for Recipe and recab; the Disk app has none of its own) | hurts | `gui/*/data/*.policy` | **partly**: the password prompt's own message now names everything the action covers (install, update, remove, keeping at a version, going back, sources, clearing); the Before page still says it only for an install or update; the prompt has never been run |
| X23 | state | Use the apps on the live desktop ISO, before installing: recab is not set up there and Recipe would install into memory. No app says so | missing | hurts | not handled in any app | **open**: nothing about the live ISO |
| X24 | feedback | Use the apps in another language. Every string is marked for translation and no translation exists | possible today | polish | no `.ts` files | **open**: no translations |
| X25 | navigation | Reach the drawn parts by keyboard or screen reader: the disk maps, the step marks, the cards' pictures | possible today | hurts | five `setAccessibleName` calls in all of `gui/` | **partly**: rail entries, search box, Activity list have accessible names; maps, step marks, tiles do not |
| X26 | navigation | Open one app at the right place from another ("show this recipe", "show the next system"), or from a notification | possible today | polish | no arguments but the checkout ones in any `main.cpp` | **open**: no arguments to open an app at a place |
| X27 | state | Run the shipped programs without the checkout's switches: `--look`, `--shots`, `--try` and `--real` are in the binaries | possible today | polish | each `main.cpp` | **open**: --look, --shots, --try, --read-only, --real still in the shipped binaries |

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

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| R1 | action | Remove software whose recipe is gone from the catalog (its source was removed, or the recipe deleted or renamed). "Remove…" quietly does nothing. The source-removal question even promises such software "stays installed" | exists (`remove` works from the receipt alone) | `window.cpp` `askRemove` | **done**: Remove… opens for a program whose recipe is gone and runs `recipe remove` from the receipt; the question names the menu entry from the receipt; stories 17a to 17d (real `recipe` not run) |
| R2 | state | Open an installed program that has no recipe: a dead end, "No recipe named X", with no way back and no action | exists (`installed` gives the receipt's `recipe` path) | `pages.cpp:275` | **done**: a program whose recipe is gone shows its receipt (version, date, size, source, files, where it is) and what works without a recipe; a name nothing carries gets "No recipe named X" and a way back; 18a to 18d |
| R3 | action | Be stopped at the trust step. Install and Update on Browse start the run at once and skip the page that says what the recipe may use | UI only | `pages.cpp:232` | **done**: Browse Install/Update go through the Before page; 2b-before-install, f1 |
| R4 | feedback | Be stopped or warned when a newer recipe asks for more than before. "Update all" queues everything; every row reads "Not compared" | partly (`recipe diff OLD NEW` exists; `updates --json` does not) | `window.cpp:156`, `pages.cpp:799` | **done**: Updates "More: …" per row, Before page names new needs, Update all asks each in turn; 7-updates, 4b |
| R5 | action | See what an update changes. The receipt names the recipe file it was installed from and `recipe diff` compares two recipes; the app reads neither | exists (`diff`, the receipt's `recipe`) | `client.cpp:84` | **done**: `updates --json` changes shown on Updates and Before (NEEDS changes; not a file diff) |
| R6 | action | Get a newer release of a program. `update` only installs the version the recipe pins. Under the product's premise this is the agent's job, and no button asks it | missing in recipe; the agent route is UI only | `recipe update --help` | **open**: no "newer release" button; recipe pins versions |
| R7 | state | See which recipes could not be read and why, and hand them to the agent. They are a count in the status bar | exists (`problems` in `list --json`) | `client.cpp:72`, `window.cpp:189` | **partly**: Sources lists unreadable recipes with "Read the file"; no hand-over to the agent |
| R8 | feedback | See progress, output or a Stop for remove, go back, add or remove a source and fetch. The window is disabled; a failure is a message box with no hand-over | exists (`--machine-events`) | `window.cpp` `quietChange` | **partly**: remove, go back, keep at a version, sources, fetching and clearing have a page with Stop and the command's own output, and on failure Recipe's message and "Try again" (17b to 17e); no hand-over to the agent, since Recipe's repair is for an install only |
| R9 | action | Install into one's own store without the administrator's password. Every change goes through `pkexec` into `/var/lib/recipe` | exists (a person's own store) | `client.cpp:14`, `:162` | **done**: "For me" runs recipe without --package-root and without pkexec; p2 checks route "me" |
| R10 | state | Know whether software goes into the running system or the next one, see what is prepared but not active, or get to the System app to finish it | exists (`--system`, `--prepared`) | `client.cpp:28`; see X1 to X3 | **partly**: locked-computer words and "after a restart" marks; prepared rows only for root; finish via strip to System |
| R11 | action | Install from a remote source: the run stops with a message, and Browse gives no sign beforehand | missing in recipe (step 3 of the sources design) | `docs/recipe-sources.md` section 5 | **done**: install names SOURCE:NAME; real recipe installs from a remote source since step 3 (README still says it stops); real run not done |
| R12 | navigation | Get back to a run after leaving its page. The status bar's label cannot be clicked | UI only | `window.cpp:73`, `:312` | **open**: install run page cannot be got back to; status line not clickable; Activity lists only agent runs |
| R13 | feedback | Stop a change that runs through `pkexec`: the stop signal goes to a root process the person may not signal. Not verified | partly | `kit/run.cpp:58` | **open**: Stop of a pkexec run is SIGTERM to pkexec; never verified |
| R14 | action | Read, open or copy the recipe file itself, though its path is known | exists (`path`, `info`) | `model.h:51` | **done**: program page "Read it", Before "Read the recipe" open the recipe file (path from list --json) |
| R15 | action | See and approve what the agent changed. It is one summary line after the run; the event's `files` are ignored | missing in recipe (no `agent` event, no hold before use) | `pages.cpp:537` | **partly**: `agent` event's summary shown after the run; files not listed, no approval before use (recipe has no hold) |
| R16 | settings | Choose or set up the agent. "Ask for software" uses an environment variable; "Let the agent fix it" uses Recipe's own `llm.toml`. The two can differ, and neither is checked | exists (`--llm-profile`, `--autofix-attempts`) | `client.cpp:16` | **partly**: Settings chooses the helper's agent; "Let the agent fix it" still uses recipe's own llm.toml, unchecked |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| R17 | action | Report a program that installed but does not work. The hand-over exists only for failed runs | UI only | `pages.cpp:598` | **open**: no "Tell the agent it does not work" |
| R18 | feedback | Give the agent the run's output. The prompt says "N lines" and gives neither the lines nor the log folder | exists (the invocation's log folder) | `pages.cpp:624` | **done**: the agent is given the last 20 lines of the run's output and where Recipe keeps its log and record (8a-agent-facts); real `recipe` not run |
| R19 | action | Ask why the sandbox refused something | exists (`recipe explain`) | `recipe explain --help` | **open**: `recipe explain` unused |
| R20 | feedback | See the limits that will really apply, and what the dependencies may use. The page shows `NEEDS` as written | exists (`inspect --json`) | `pages.cpp:337` | **open**: NEEDS shown as written; `inspect --json` unused |
| R21 | state | See before installing what else will be installed. Only build dependencies appear, inside a sentence | exists (`deps`) | `pages.cpp:85` | **done**: Before page lists needed recipes not yet installed, build-only apart; program page "Needs"; 2b checks ncurses |
| R22 | state | Tell libraries from programs on Installed, see what needs a library, or be warned before removing one others need | partly (no reverse query) | `pages.cpp:687` | **open**: no library line, no "needed by" |
| R23 | state | See both recipes when two sources hold one name. The first is shown, the other hidden | exists (`SOURCE:NAME`) | `client.cpp:73` | **open**: first of two same-named recipes shown, other hidden (client.cpp:129) |
| R24 | state | Act on a source with a problem: no hand-over, no hint; a folder source has no "Read again" | partly (`problem`) | `sources.cpp:57` | **partly**: a source's problem shown in its state line; no hand-over, folder has no "Read again" |
| R25 | feedback | Be told what adding a source means: trusting its keeper. No preview of its recipes, no branch, no check before root changes anything | exists (`source add --branch`) | `sources.cpp:28` | **open**: Add a source is a plain input dialog: no trust warning, preview or branch |
| R26 | action | Switch the shipped source off, or one that was switched off back on | exists | `sources.cpp:75` | **open**: no Switch off / on (Remove switches a shipped source off, nothing turns it back on) |
| R27 | action | Install or open a `.rhai` file from the file manager or a dialog | exists (`install ./file.rhai`) | `main.cpp`, the menu entry | **open**: no .rhai file opening |
| R28 | navigation | Be sent into the app at a place: a recipe, Updates, a source, a failed run | UI only | `main.cpp` | **open**: no deep links |
| R29 | feedback | Be told an update exists, or that a long build finished while the window was hidden | missing in recipe (`updates`, no scheduler) | README "Not done" | **open**: no update notice |
| R30 | page | See the history: what was installed, updated or removed, when, how it ended, and whether an agent did it | partly (the last 32 invocation logs are kept) | no such page | **partly**: Storage "What was done" from `recipe history`; no actor (agent or not), none for a locked computer's system |
| R31 | state | Have a run that was cut off (window closed, crash, restart) noticed at the next start and offered to continue | exists (resumable state) | `window.cpp:164` | **open**: a cut-off run is not noticed at the next start |
| R32 | feedback | Be warned when closing the window during a run. The run is killed, or orphaned under `pkexec` | UI only | no `closeEvent` | **done**: closing the window during an install, update or short change asks first, says what stops, what is kept and that it can be started again (19a, 19b); whether Stop reaches a `pkexec` process is still unverified (R13) |
| R33 | action | Queue a second install. "Update all" keeps a hidden queue, empties it at the first failure, and gives no summary | UI only | `window.cpp:312`, `:333` | **partly**: Update all asks about each update in turn; queue still emptied at the first failure, no summary, no second install queued |
| R34 | feedback | Update all with one password. It asks once per update | partly | `window.cpp:156` | **open**: one pkexec per update |
| R35 | state | Trust "Update to": without `updates`, any differing version counts, including an older one | missing in recipe | `client.cpp:125` | **done**: `updates --json` (real) decides what is newer; held flag read |
| R36 | action | Repair or reinstall a program, or check that it is intact | exists (`isinstalled`, `--force-install`) | `recipe isinstalled --help` | **open**: no repair/reinstall/check (`isinstalled` never asked) |
| R37 | page | See what Recipe itself keeps on disk (build caches, kept generations, fetched sources) and clear it | partly (`cleanup` per recipe) | `pages.cpp:663` | **partly**: Storage: earlier versions per store and Clear earlier versions…; no build caches or fetched sources (recipe reports none) |
| R38 | action | Go back to an earlier version. The button calls a command that does not exist and shows only for a field recipe never sends | missing in recipe (`go-back`, `kept`) | `client.cpp:195` | **done**: Go back to X on Installed and program page runs `recipe go-back`; 7b-gone-back |
| R39 | action | Remove a program's settings and data in the home folder with it | missing in recipe | `pages.cpp:907` | **open**: remove leaves home data and says so |
| R40 | action | Open a program properly. The recipe page offers "Open" for command-line tools, started with no terminal, and always starts the first file in `bin/`, not the menu entry | partly | `pages.cpp:294`, `window.cpp:468` | **done**: Open starts the program from the profile of the place it is in, by its menu entry's `Exec` line, else the first `bin/` file; a `Terminal=true` entry is not started and the status line says so; software only the next system holds has no Open; p8, p9. `DBusActivatable`, `TryExec` and `Path` are ignored |
| R41 | feedback | Be told an install succeeded, and told again when a restart or a new login is needed | UI only | `window.cpp:341` | **partly**: after an install the program page shows it installed; no explicit success notice; restart/login only as recipe text |
| R42 | navigation | Go back to where one came from. "‹ All software" always goes to Browse | UI only | `pages.cpp:284` | **open**: "‹ All software" always goes to Browse |
| R43 | feedback | Know the computer is offline. A failed fetch is a message box and the kept list is used silently | partly | `window.cpp:446` | **open**: offline is a failed fetch message box |
| R44 | state | Find a way forward with no sources or no recipes: "Nothing here." | UI only | `pages.cpp:238`, `sources.cpp:83` | **partly**: empty Browse says "Nothing here." but offers "Ask for software…"; no source → no lead |
| R45 | state | Be led from an empty Installed page to Browse, or see when updates were last checked | partly (`checked`) | `pages.cpp:678`, `:787` | **partly**: Updates says when lists were last fetched; empty Installed does not lead to Browse |
| R46 | action | Answer the agent during "Let the agent fix it" | missing in recipe | README "Not done" | **open**: recipe cannot pass the autofix agent's question on |
| R47 | state | See that a recipe was written by the agent and not yet read by a person, or ask the agent to change or delete it | UI only | `window.cpp:112` | **open**: an agent-written recipe is opened but not marked unread; no "change or delete it" |
| R48 | feedback | Be told before handing over that the agent program is missing | UI only | `client.cpp:226` | **partly**: status line and first page say whether the agent is installed/signed in; hand-over not stopped beforehand |
| R49 | action | Copy a recipe into one's own folder and change it, or publish one to a source | partly (the own folder; `index`) | README "Not done" | **open**: no copy to own recipes, no publish |
| R50 | action | Report a broken recipe to its source's keeper | missing in recipe | `docs/recipe-sources.md` | **open**: no report to a source's keeper |
| R51 | feedback | See how much will be downloaded and how much disk an install needs. Only upper limits are shown | missing in recipe | `pages.cpp:339` | **open**: recipe reports no download or disk need |
| R52 | feedback | See real progress. `fetch`, `progress`, `plan` and `say` are read but never sent, so the bar has no end and the steps are guessed | missing in recipe | `pages.cpp:475` | **partly**: plan/fetch events read (real ones compiled, not run); no progress inside a step |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| R53 | action | Search by what a program does. Search reads name, title and description only | exists (`about`) | `pages.cpp:199` | **open**: search reads name, title, description only |
| R54 | action | Filter Browse to installed or not, or by licence; sort it | UI only | `pages.cpp:194` | **partly**: kind box and source box; no installed/licence filter, no sort |
| R55 | action | Sort or search the Installed and Updates tables | UI only | `pages.cpp:130` | **open**: Installed and Updates tables have no search or sort |
| R56 | settings | Set preferences: agent, store, how often sources are fetched, limits (`--no-network`, `--memory-mib`, `--jobs`) | exists (global options) | none in `window.cpp` | **partly**: Settings holds the agent only; no store, fetch interval or limits |
| R57 | navigation | Use a menu, Help or About, or reach the manual | UI only | `main.cpp` | **partly**: Help page; no menu, About or manual |
| R58 | navigation | Use Ctrl+F, Escape or Alt+Left, Ctrl+Q | UI only | `kit/refresh.cpp:12` | **partly**: Escape on Before; no Ctrl+F, Alt+Left, Ctrl+Q |
| R59 | settings | Find the window as it was left: size, page, chosen source | UI only | no `QSettings` | **partly**: window size remembered; page and chosen source not |
| R60 | feedback | Use it with a screen reader: every Install button reads "Install", step marks are drawn with no text, Browse is a grid of labels, failure is shown only in red | UI only | `pages.cpp:232` | **open**: buttons all read "Install", step marks drawn, colour carries failure |
| R61 | state | See a library as a library when opened by name: it shows an "Install" page like an app | UI only | `pages.cpp:302` | **open**: a library opened by name looks like an app |
| R62 | feedback | Tell "Read again" (local) from "Check for updates" and "Fetch" (network). The difference is only in tooltips | UI only | `window.cpp:130` | **open**: Read again vs Check for updates/Fetch still told apart only by tooltip (and Help) |
| R63 | feedback | See a program's own icon, or the app in another language | missing in recipe (icons) | README "Not done" | **open**: letter tiles, no icons, no translations |
| R64 | action | See the licences of the dependencies | partly | `pages.cpp:372` | **open**: dependencies' licences not shown |
| R65 | action | Hold a program at its version | missing in recipe | `recipe --help` | **done**: Keep at this version / Let it update again (`recipe hold/unhold`); 16a-16d (real hold read on a hand-made store) |

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

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| S1 | feedback | Read why a long run failed. The "It stopped" page is replaced by the main page as soon as the status is read again | exists (recab's error message and class) | `kit/work.cpp` `onFinished`; `window.cpp:57` | **done**: a failed recab command stays on "It stopped" with recab's own reason and output, and Back leads on; the story reaches a failed command (stand-in) |
| S2 | state | See that the running system is a trial that is not kept yet. It is shown as a normal locked system and offers "Make a change", which recab refuses | exists (`committed`, `scheduled_trial`) | `model.cpp` never reads `committed` | **done**: Overview "You are trying the new system" from lifecycle on_trial; story r4a |
| S3 | action | Keep the running system by hand. The terminal app can | exists (`recab commit`) | `recab commit --help` | **done**: "Keep it now" (Overview) / "Keep it" (Next, Getting back) -> recab commit; story trial kept by hand |
| S4 | state | Recover when "Restart into it" set the next start but the restart itself failed or its password was refused. The button is greyed out with "There is nothing to do", and the warning says "The change was not made", which is false | partly (`trial_pending`) | `window.cpp:280`, `:213` | **done**: restart_set is its own state with Restart now / Cancel the set restart (r3b, rp6 to rp8); a failed restart says it did not happen and names the step made before it, and the story reaches it |
| S5 | action | Continue the right thing after an interruption. "Continue" always runs `recab prepare`; a change cut off in the middle needs `recab change recover`, and recab refuses other work until then | exists (`recab change recover`) | `client.cpp:84` | **done**: Continue follows recab's own *_recover operation; the story recovers a cut-off change with the real-shaped stand-in |
| S6 | state | Tell "recab is working right now" (the agent, a terminal, the Recipe app) from "it was interrupted". Every phase but two is shown as interrupted, with Discard on offer | missing in recab (no "a command is running" in status) | `model.cpp:161` | **done**: lifecycle.working: Busy view, rereads every 2 s; story r2b, rp4 |
| S7 | page | List the next system's changes. Only the last one is shown, and the agent is told "the next system already holds: X" when it may hold more | exists (`recab change list`) | `model.cpp:148` | **done**: The next system lists every preparation.changes row with who and when; Ask facts list all; story rp2 |
| S8 | action | Update the system to a new release. The row is always off with a fixed sentence | exists (`recab stage --from-media`, `recab release import`) | `model.cpp:217` | **partly**: the Release page drives update_check, download, apply and media, and recab's merge operations (resolve, reproduce a package, adopt a baseline, replace a failed system, recover, cancel) with recab's own labels, and says so when recab has no update source; a real check needs a configured origin and keys, and everything ran against the stand-in only |
| S9 | feedback | Be told that a new release exists | missing in recab (discovery and download are not built) | recab README | **partly**: Overview names a known release only after "Check for a new release"; no notice, no automatic check |
| S10 | action | Review what the agent did before finishing. After an agent's run on the real recab the window goes back to the main page; the expected result is never checked | missing in recab (no proposal); partly (`recab change check`) | `window.cpp:239` | **partly**: Activity shows what the agent ran and changed; window still returns to the page; expected result never checked |
| S11 | feedback | See what a successful run did: the output, the agent's summary, the synchronization report are dropped | exists (`preparation.synchronization`, `last_change.report`, `finalization`) | `window.cpp:57` | **partly**: agent runs kept in Activity; recab's own output and reports dropped after success |
| S12 | action | See exactly what will be different after the restart: neither the boot selection before and after, nor the files or packages | exists for the boot selection (`ActionView.after`, ignored); partly for content | `model.cpp:193` | **partly**: "What will differ after the restart" (slot, device, release, software, files); no ActionView.after, no file/package content |
| S13 | state | Know with what rights the agent runs. It is started as the desktop user, and every recab change needs root | the app's decision | `client.cpp:100` | **done**: Help "The agent" and Settings say: runs as you, asks before admin commands; story h-help, s1-s2 |
| S14 | page | Learn how to go back when the desktop does not come up at all: the boot menu entry for the other system, `recab rollback` from a console | exists (rollback, boot entries) | recab README:315 | **done**: Getting back "If the desktop does not start" with steps and "Save as a file…"; story rp1, p02, rp-steps.txt |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| S15 | state | Be told plainly "this computer has one system only", "recab is not set up", "the installation is unfinished", "no UEFI". All end on "The system could not be read" with raw text | exists (the error's class; `init --mode mutable`) | `client.cpp:43` | **partly**: only "permission" (Read it as administrator) and recab's recovery_reason told apart; single system not built, rest raw text |
| S16 | state | See recab's own reason for a needed recovery | exists (`recovery_reason`) | never read | **done**: a system that needs recovery has its own heading with recab's reason and the steps recab offers, each with its own verdict; the story reaches it (real-shaped stand-in) |
| S17 | feedback | Read the system without a password prompt each time. After "Read it as administrator" every read asks again, every change asks again, a restart costs two prompts, and the restart is not covered by the polkit action | partly | `window.cpp:77`; the policy file | **open**: every read/change asks again after "Read it as administrator"; policy auth_admin (no _keep), reboot not covered |
| S18 | state | Know that once the next system is ready, any restart starts it, not only the button | exists (`trial_pending`, `next_boot`) | `pages.cpp:129` | **done**: ready/restart_set pages say any restart starts it, whoever makes it; story r3, r3b |
| S19 | action | Cancel a set restart into the next system and keep starting the running one | exists (`recab rollback` with a trial pending) | `pages.cpp:251` | **done**: "Cancel the set restart" -> recab commit (Next, Getting back); story rp7, rp8, br04 |
| S20 | action | Set the next system and restart later; go back without restarting now | exists (`set-next`, `rollback` without the restart) | `window.cpp:280` | **partly**: "Start it at the next restart" (set-next alone) done; going back without restarting is not offered |
| S21 | action | Make the other partition an exact copy when there is no next system. Reset shows only with a next system | exists (`recab prepare`) | `pages.cpp:297` | **open**: no "make the other a copy" when there is no next system; prepare is only in the recovery list (replace_failed is also on the failed system's page and the release page) |
| S22 | state | Trust the reset. It leaves an open next system that cannot be started, not the ready second copy there was before | partly (no "make a ready copy" command) | `window.cpp` `reset` | **partly**: reset is discard + prepare: leaves an open empty next system, not the ready copy (recab has no single command) |
| S23 | feedback | Be told after a restart whether it was kept or came back, or that the agent asks something while the window is behind others | exists (`last_trial`); no push | no notification code | **partly**: helper notices for agent questions/ends (Settings "Notices"); nothing after a restart kept or fallen back |
| S24 | page | See why a start failed: the failed units, that start's journal | partly (`last_trial.detail`) | `pages.cpp:161` | **partly**: failed-start page shows recab's last_trial.detail (failed units); no journal of that start |
| S25 | state | Rely on the failed-start page appearing. It needs the preparation to still be there, ready, after the fallback; whether the real recab leaves it was never checked | partly | `model.cpp:184` | **done**: page follows recab's own lifecycle trial_failed (with or without preparation); story r13 (stand-in only, not on A/B) |
| S26 | page | See the history of changes and restarts, and who or which agent made them | partly (`/var/lib/recab/audit.log`, no actor) | no such page; `recab-tui` has one | **partly**: History page from `recab history --json` (r15, r15b); who only for changes, recab records no actor otherwise |
| S27 | page | Inspect or export logs: the agent's transcript, recab's output, the audit log, the trial record | partly | `kit/work.cpp:62` | **partly**: History "Save as a file…", Activity transcript, trial detail; recab command output and audit log not exportable as such |
| S28 | feedback | See what a change cost in disk space, or the free space on either slot | partly (the reports' byte counts; no free space) | status types | **open**: no slot size or free space; recab reports none |
| S29 | feedback | See real progress of a preparation | partly (`completed_steps`, the counters; no live events) | `kit/work.cpp:53` | **open**: moving bar only; real recab's --machine-events give start/finish, no progress |
| S30 | state | Know the two systems by more than the running system's name: generation, kernel, readiness, device. After a staged release the names would be wrong | exists (generation, `seal.kernel_release`, `readiness`, `device`) | `model.cpp:111` | **partly**: slot letter, device, entry files shown (Next, Getting back); no generation, kernel, readiness |
| S31 | action | Know what Stop does. On a recab command through `pkexec` it probably cannot stop recab; on the agent it stops the agent, not a recab it started. Not verified | partly | `kit/work.cpp:38` | **partly**: agent Stop is levitate-agent --stop (livestory); Stop of a recab command under pkexec still unverified |
| S32 | action | Be asked before closing the window ends an agent in the middle of a change | UI only | no `closeEvent` | **done**: agent runs outlive the window (levitate-agent lasting runs; frame letGoOfRuns); livestory a-live-* |
| S33 | feedback | Have the window notice a change made elsewhere while it is in front | exists (`/var/lib/recab/state.json` could be watched) | `kit/refresh.cpp` | **partly**: rereads every 2 s only while recab says working; otherwise only on F5 / coming back |
| S34 | action | Be warned about unsaved work in other programs before a restart, or restart in a few minutes | UI only | `window.cpp:280` | **partly**: Before page says open programs are closed; no warning of unsaved work, no delayed restart |
| S35 | action | Install or remove software for the system directly, or get to the Recipe app | exists (`recab change install|update|remove`) | `pages.cpp:281` | **done**: "Install software…" on The next system starts Recipe (for this computer -> recipe --system -> next system); helpstory app starts |
| S36 | navigation | Get to the Recipe app and the Disk app. Recipe can also add changes to the same next system without this window showing more than the last | UI only | none in `gui/recab/src` | **done**: rail app buttons and Help "others" start Recipe and Disk; helpstory |
| S37 | settings | Choose the agent, what it may run, the restart command. The status bar always says "Agent: codex" | UI only | `client.cpp:7` | **done**: Settings chooses agent and "Run commands"; status line names agent; restart command env only |
| S38 | action | Approve or refuse the agent's single steps, or see which commands it ran | missing | README | **done**: helper asks before admin/outside-sandbox commands; Activity Approve/Refuse and commands run; livestory -live-approval |
| S39 | feedback | Find an agent's question again after the run ended or the window closed | UI only | `kit/work.cpp:195` | **done**: Activity lists waiting runs, answerable from any window; livestory -live-question |
| S40 | action | Hand an interrupted preparation, a refused command or a failed run to the agent. Only a failed start and a clash build a prompt | exists (the error's class and message) | `pages.cpp:241` | **open**: only failed start and clash build a prompt; interrupted, refused and failed runs do not hand over |
| S41 | feedback | Understand a refusal. It is a box with recab's raw message, with no guidance by kind of error | exists (error classes) | `window.cpp:218` | **partly**: a refused or failed command stays on a page with recab's own message and output, not a box; still no guidance by kind of error |
| S42 | feedback | Know what going back does not undo: data in `/var` and what programs changed in their own data | exists (in recab's requirements) | `window.cpp:288` | **done**: Getting back "What going back does not turn back" (/var, home); story rp* (the go-back Before page does not repeat it) |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| S43 | feedback | Stop seeing "You are on the new system" some day. It stays until the next trial, with no date | exists (`last_trial.unix_time`) | `pages.cpp:143` | **partly**: card says "Kept: it started properly on <time>"; heading "You are on the new system" stays until the next change |
| S44 | feedback | Trust "Finish and restart": it only finishes | UI only | `pages.cpp:268` | **done**: Overview row "Finish and restart" says two steps; Before page "The computer does not restart"; b02 |
| S45 | action | Edit a configuration file oneself | exists (`recab edit PATH --file`) | only told to the agent | **open**: `edit` operation only in the recovery list; no "Edit a file…" |
| S46 | page | See how the running system's start-up ended | exists (`recab health --json`) | unused | **open**: recab health unused |
| S47 | action | Change the flavour (desktop or base), or finish with another kernel | exists (`--change-flavour`, `finalize --kernel`) | unused | **open**: --change-flavour, finalize --kernel unused |
| S48 | action | Have an inserted release medium noticed and offered | partly | `docs/recab-gui.md` | **open**: no medium noticed; only "Choose a file…" (update_media --manifest) |
| S49 | feedback | Be told that "Make a change" on a ready next system throws its finishing away and cancels the restart | exists (the verdict's `after`) | `pages.cpp:255` | **open**: "Make a change…" on a ready next system gives no warning that finishing is undone / restart cancelled |
| S50 | feedback | Tell the agent everything recab offers: the prompt leaves out `change list`, `check`, `recover` and `recipe --system` | exists | `pages.cpp:339` | **partly**: Ask facts name prepare, change install/update/remove, edit, finalize, status; not change list/check/recover, recipe --system |
| S51 | settings | Use a menu, Help, About, or read what A and B are | UI only | `window.cpp` | **partly**: Help (ideas, how, what went wrong); no menu, no About |
| S52 | settings | Use keys for the main actions, or Esc for Back | UI only | `kit/refresh.cpp` | **partly**: F5/Ctrl+R, Escape on the Before page only; no keys for main actions, no Esc for Back |
| S53 | settings | Use it with a screen reader or by keyboard: cards and pictures carry the state by drawing and colour, "‹ Back" may not take focus | UI only | `pages.cpp:52` | **open**: three setAccessibleName in recab; cards and step marks carry state by drawing |
| S54 | settings | Find the window as it was left; use another language | UI only | `main.cpp:33` | **partly**: window size remembered (rememberWindowSize); page not, no translations |
| S55 | state | Reach the clash and proposal pages with the real recab. They exist only for the stand-in | missing in recab | `model.cpp:41` | **partly**: the clash page is reached with real recab's merge conflicts, with the sentence for each kind recab reports, the commands recab offers and recab's own facts for the agent (r10 to r22); the proposal page exists only for the stand-in, since recab has no place for a proposal |
| S56 | state | Decide several clashes safely: one box per file, never run, and cancelling midway still applies the earlier answers | missing in recab | `window.cpp` `decideByHand` | **done**: real recab: one resolve operation at a time, status read between; r11 (old stand-in still loops boxes) |
| S57 | feedback | See what the next system holds when a restart is set and no preparation is left | partly | `model.cpp:169` | **partly**: restart set with no preparation: "recab lists no finished change in it" |
| S58 | navigation | Read another installed system, or the full inventory (for repair) | exists (`status --root`, `inventory --root`) | unused | **open**: status --root / inventory --root unused |

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

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| D1 | state | See the partition that really fills up: the state partition, which holds `/var` and `/home`. The app reads the home folder and `/var/lib/recipe`, so it can say "No mess" while `/var` is the cause | possible today (read rights or a root helper) | `window.cpp:41`, `scan.cpp:55` | **partly**: The disk lists every partition with statvfs fill; the scan and rules still read only home and /var/lib/recipe, not /var |
| D2 | page | See the partitions: EFI, the two systems, state, swap; each with size, used, free | possible today; the other system's fill needs recab | `window.cpp:194` | **partly**: The disk "Every partition of this disk" (lsblk); the other system is "not mounted, so not read" |
| D3 | state | Be told which folders could not be read. They are collected and thrown away, so totals are too low without a word | possible today | `scan.cpp` `unreadable()` is never used | **partly**: The disk says how many folders could not be read, not which |
| D4 | state | Have the places that grow looked at: logs and the journal, `/var/tmp`, core dumps, container and virtual machine images, the swap file | possible today (read rights) | no rule in `mess.cpp` | **open**: no rule for logs, journal, /var/tmp, core dumps, images, swap file (README: cannot be measured as the person) |
| D5 | state | Stop a scan, or close the window during one without a crash. There is no Stop, and a running scan thread is destroyed with the window | possible today | `pages.cpp:46`, `window.cpp:177` | **partly**: closing waits for an interrupted scan (~Window); no Stop for a scan; no story closes during a scan |
| D6 | state | Be warned that the disk is nearly or wholly full, with what to do first | possible today | `pages.cpp:90` | **done**: The disk heading "nearly full" from statvfs, with "What can be freed" rows (1-disk); in-window only |
| D7 | state | See what the agent did. After it succeeds the window looks again and jumps to the map; its output, answers and summary are gone | possible today (keep the run) | `window.cpp:421` | **done**: page "What was set aside" (a row per place, read back from the holding folder) and Activity; the stories run through the built helper with the stand-in Codex; a real Codex or Claude Code has never written the lines the helper reads |
| D8 | state | Know before handing over that the agent program is missing or cannot speak to the app. It ends as "It stopped" | needs the agent (a launcher) | `window.cpp:30` | **partly**: status line shows `levitate-agent --status`; the hand-over page asks the helper which agent is chosen (`--settings`) and words itself for it, but does not stop a hand-over to an agent that is missing |
| D9 | state | Close the window safely while the agent works. It is killed partway through removing, without a question | possible today | no `closeEvent` | **done**: runs outlive the window (frame lets go, SIGHUP); livestory a3-* checks it |
| D10 | state | Be told "this computer has one system only" as a fact. It is shown as an error with "Try again"; with the real recab in single-system mode the other is even named "The second copy" and Compare is offered | needs recab (the mode in status) | `systems.cpp:583` | **open**: recab status has no single-system mode; the stand-in's "none" is an error with Try again; an "unknown" other system is no longer called "The second copy" |
| D11 | state | Compare at all with the real recab. Compare always ends in a refusal, and "Try again" leads nowhere | needs recab (`recab compare`) | `systems.cpp:619` | **partly**: real `recab compare` exists and the page reads its shape; two-slot form never run on A/B; story uses schema-3 stand-in |
| D12 | action | Get one answer to "my disk is full, what do I do now": which partition, which three things free the most, go | possible today, with D1 | `pages.cpp:75` | **partly**: The disk: heading + what can be freed with buttons; for the home partition only, not /var or the systems |
| D13 | navigation | Zoom into a folder on the map. The path line only moves the selection; below a depth or size, or inside a finding, the map draws one block | possible today | `map.cpp:61` | **done**: Browse "Go in" / path: the map follows the folder, a finding opens when entered (2b-browse-in) |
| D14 | navigation | Go up, go back, double-click to go in, or move by wheel or keyboard on the map | possible today | `map.cpp:217` | **partly**: Up, clickable path, Enter/double-click on a row; no back history, no wheel or keys on the map |
| D15 | page | Browse a folder tree with sizes beside the map, the core of every such tool. The list shows only what the rules named | possible today | `pages.cpp:126` | **done**: Browse: folder rows with size and share beside the map (2a-browse) |
| D16 | feedback | Be told whether it is one's own files, software, or the system that takes the room | possible today for home and Recipe; needs recab for the systems | `pages.cpp:90` | **partly**: The disk "Whose room it is" (files, programs, leftovers, set aside, other); systems not split (needs recab) |
| D17 | action | See, path by path, what the agent removed, kept, or asked about and was refused | needs the agent (an event per path) | README, the agent's events | **done**: a row per place: set aside, kept (and what was asked), could not (and why, "Try again"); the helper reports each place from the agent's SET-ASIDE, KEEP and CANNOT lines and moves the places itself; a real agent never run |
| D18 | action | Get anything back. The prompt never asks for the trash, and there is no undo | needs the agent to honour it; possible today in the prompt | `pages.cpp:351` | **done**: set aside / Bring back / Let it all go: the helper moves the places into a holding folder (never replacing, never following a link out of home); story 4-after to 4e through the built helper; no trash; a real agent never run |
| D19 | action | Act on a difference between the two systems: make them the same again. The agent is told to change nothing, and "Open System…" passes nothing along | needs recab; possible today as a hand-over to the System app | `systems.cpp:431` | **open**: "Open System…" passes nothing; agent told to change nothing |

### Hurts

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| D20 | feedback | Tell how much of the disk was looked at. "X of Y looks like mess" uses the bytes scanned, not the disk | possible today | `pages.cpp:207` | **partly**: The disk "The disk counts X as used; Y outside the folders read", only without DISK_GUI_APPARENT, so no story shows it; issue still uses scanned bytes |
| D21 | state | See other people's home folders and root's | possible today with a root helper | `window.cpp:41` | **open**: no root helper for other homes |
| D22 | state | See the EFI partition, where a full disk blocks the next system change | possible today | recpart README | **done**: The disk names "Start-up files / The EFI system partition" with size and use when mounted (1-disk) |
| D23 | state | See how full each system partition is and whether the next change will fit | needs recab (slot size and use in status) | `systems.cpp:506` | **partly**: size from lsblk; the other system's use is not read; no "will the next change fit" (recab lacks slot use) |
| D24 | action | Point the scan at another folder or a removable drive | possible today | `window.cpp:41` | **open**: places only through DISK_GUI_ROOTS |
| D25 | state | Know which of Recipe's kept versions is in use and which are old. The rule goes by a folder's name | needs Recipe (a listing with in-use marks; a prune command) | `mess.cpp:86` | **partly**: "Kept by Recipe" uses `recipe storage --json` earlier_bytes; Recipe's caches/state in Mess still named by folder |
| D26 | feedback | Trust "At least X is free again": it counts Recipe's folders, which are to be cleared "through Recipe" by a command nobody checked exists | needs Recipe | `pages.cpp:351` | **done**: Recipe's own folders are listed but never handed over, so "up to X free" does not count them; "Kept by Recipe" is Recipe's own number; Mess says Disk leaves them alone |
| D27 | state | See how far a scan is, and which folder is being read | possible today | `pages.cpp:58` | **partly**: Mess and The disk say bytes and files so far; not which folder or how far |
| D28 | state | Open the app without waiting: every start scans from nothing | possible today | `window.cpp:135` | **open**: every start scans from nothing |
| D29 | state | Learn where the room went when no mess is found: "No mess worth naming" and nothing more | possible today | `pages.cpp:207` | **done**: Browse, Biggest files and "Whose room it is" show where the room went (2a, 2c, 1-disk) |
| D30 | feedback | Trust "freed": it is scanned bytes before minus after, not the change in free space | possible today | `window.cpp:191` | **done**: after page: Free before / Free now measured by statvfs, Free once you let it go (4-after) |
| D31 | feedback | Trust "Nothing has been removed" after an agent's run that freed nothing net | possible today | `pages.cpp:90` | **done**: sentence gone; after page measures (4-after) |
| D32 | feedback | Trust "The running system was not touched" after a failed clearing run. The sentence is the System app's and untrue here | possible today | `kit/work.cpp:216` | **partly**: the work page no longer claims the running system was untouched (fixed 2026-11-11); no story fails a clearing run |
| D33 | feedback | Trust "What is done is kept; you can continue later" on Stop: every tick and pick is then forgotten | possible today | `kit/work.cpp:44` | **partly**: Stop now says what was set aside stays and the next page shows it (fixed 2026-11-11); no story stops a clearing run |
| D34 | state | Notice the agent's question when the window is behind others | possible today | `kit/work.cpp:174` | **partly**: helper notifier (gdbus) when the agent waits; never seen on a desktop, off in stories |
| D35 | state | See the map while the agent clears, to check a path it asks about | possible today | `window.cpp:162` | **partly**: Browse can be chosen while the agent clears (only Mess shows the work); no story checks it |
| D36 | state | Tell a kept comparison is old when recab cannot be read: it is drawn as if current | possible today | `window.cpp:246` | **open**: stale() needs state_.known; with recab unreadable the kept comparison is drawn as current |
| D37 | state | Look at the disk while a comparison runs for minutes. The other tab is switched off | possible today | `window.cpp:275` | **partly**: the other entries can be chosen while comparing (README); no story checks it |
| D38 | state | Find the agent's explanation of the differences after a restart of the app. The comparison is kept, the explanation is not | possible today | `window.cpp:327` | **open**: keep() saves the answer, not what the agent said |
| D39 | feedback | Trust "% the same": it is by bytes, so thousands of small changed files still read 99.9%. That works against the goal of seeing that they look the same | possible today | `systems.cpp:694` | **partly**: headline still by bytes; "N files differ" in the line under it |
| D40 | state | Reach the differences recab left out when there are thousands. The agent is given only the 25 biggest | needs recab (paging) | `systems.cpp:412` | **partly**: list shows what recab gave (up to 5000, `more` counted); agent gets folder groups + 25 biggest |
| D41 | page | List the biggest files | possible today | `scan.cpp:17` | **done**: Browse "Biggest files" (2c-browse-biggest) |
| D42 | page | See old files, or colour by age | possible today (`lastUsed` is read) | `scan.cpp:85` | **done**: Browse "Not used for a year" (2d-browse-unused); no colour by age |
| D43 | page | See by file type | possible today | `scan.cpp:106` | **open**: scanner records no file kinds (README) |
| D44 | action | Search for a file, a name or a type | possible today | no search | **partly**: Browse search filters the current folder only (2b) |
| D45 | action | Sort the lists | possible today | `pages.cpp:127`, `systems.cpp:790` | **open**: no list sorts (no setSortingEnabled in disk/src) |
| D46 | action | Leave a folder out of future scans | possible today | no setting | **open**: no "do not look here" list (README) |
| D47 | action | Mark a finding "never suggest this again" | possible today | no ignore list | **done**: "Never suggest this" in Browse/Mess, kept in config, "Suggest it again" (2f, 3c) |
| D48 | action | Add a rule of one's own. The rules are C++ | possible today (rules as data) | `mess.cpp:59` | **open**: rules are C++; "never suggest" is whole places only |
| D49 | action | Take a hand-picked item off the list. Every new look and every agent's run forgets the picks | possible today | `pages.cpp:274` | **partly**: "Take off the list" exists (Browse/Mess); picks are still forgotten at every look |
| D50 | feedback | Trust the issue: it calls hand-picked, unknown things "leftovers" | possible today | `pages.cpp:348` | **open**: issue still says "taken by things that look like leftovers" for picks too (pages.cpp:439) |
| D51 | page | See the history of past clean-ups | possible today | nothing is written | **partly**: Activity lists past agent runs; no record of what was freed per clean-up |
| D52 | action | Be warned before the disk fills, or have a check run on a schedule | possible today, with something running outside the window | no timer | **open**: nothing runs outside the window |
| D53 | page | See what changed inside a file whose content differs | needs recab (one path from both systems) | README "Not done" | **open**: needs recab (one path from both systems) |
| D54 | action | Search or filter the differences | possible today | `systems.cpp:790` | **open**: no search or filter of differences (README) |
| D55 | navigation | Zoom both system maps into one folder together | possible today | `systems.cpp:162` | **open**: a click marks a block on both sides; no zoom of both maps |
| D56 | action | Have differences explained against what was changed: the agent is asked which change each belongs to and is given no list of changes | needs recab (the change list) | `systems.cpp:429` | **open**: agent asked which change each difference belongs to and given no change list (real recab has preparation.changes and `history`) |
| D57 | settings | Set anything: the agent, the thresholds (20 MiB; 256 MiB and a year; 90 days), the places, what is left out, the rules | possible today | `mess.cpp:15`, `window.cpp:29` | **partly**: agent chosen in shared Settings (1b-settings); thresholds, places, rules not settable |
| D58 | navigation | Use a menu, Help, About, or keys for zoom, up, search, copy | possible today | `kit/refresh.cpp:12` | **partly**: Help (h-help*), F5/Ctrl+R, Escape on the hand-over; no menu, no keys for zoom/up/search/copy |
| D59 | action | Right-click a block or a line: open, copy the path, add, leave out | possible today | no context menu | **open**: no context menu |
| D60 | feedback | Read the map: no legend but the list's colours, no highlight under the pointer, everything unnamed one grey | possible today | `map.cpp:123` | **partly**: legends on The disk, Browse, the two systems; no hover highlight; unnamed still grey |
| D61 | navigation | Get from Recipe's own findings to the Recipe app | possible today | `mess.cpp:79` | **partly**: "Kept by Recipe" row has "Open Recipe…" (story checks); Mess's Recipe findings have no way to Recipe |
| D62 | feedback | Use it with a screen reader or keyboard: the maps cannot take focus, kinds are told apart by colour alone, "Picked by you" is nearly invisible on a light look | possible today | `map.cpp:13`, `mess.cpp:166` | **open**: maps take no focus; kinds by colour; light-look "Picked by you" not checked |
| D63 | action | Keep ticks and picks through "Look again", "Read again" and F5. They are dropped without a question | possible today | `window.cpp:117` | **open**: Look again / F5 recompute the list; ticks and picks dropped (window.cpp scanned()) |

### Polish

| # | Kind | What a person cannot do or see | Backend | Evidence | Now (2026-11-11) |
|---|---|---|---|---|---|
| D64 | state | Use anything while a scan runs | possible today | `pages.cpp:48` | **partly**: The disk, partitions and The two systems usable during a scan; Browse and Mess wait |
| D65 | state | Be sure a kept comparison is of this computer's disks | needs recab | README "Not done" | **partly**: state key now holds slots, generations, revision; nothing ties it to this computer's disks |
| D66 | state | Be told a comparison was stopped. An older kept one quietly reappears | possible today | `window.cpp:299` | **open**: a stopped comparison says nothing and the older kept one reappears (window.cpp compared()) |
| D67 | state | Read "identical" with its exception in the headline: "except N files that name their own partition" | possible today | `systems.cpp:840` | **partly**: "differ by design" is a foot note, not in the headline |
| D68 | feedback | Be told the password was refused or cancelled | possible today | `systems.cpp:649` | **partly**: the root helper's problem text is shown; no plain "password refused or cancelled" |
| D69 | action | Find duplicate files | possible in the app, with hashing the scanner does not do | `scan.cpp` | **open**: no hashing |
| D70 | action | Open the file itself, copy its path, open a terminal there | possible today | `window.cpp:149` | **partly**: "Open in Files" (the folder) and "Copy the path" (2e); no open the file, no terminal |
| D71 | action | Change the issue or a fact, or add an instruction ("use the trash") | possible today | `kit/parts.h:86` | **open**: "Only the expected result is yours to change."; the set-aside instruction is fixed in the issue |
| D72 | action | Export or copy a report | possible today | README "Not done" | **open**: no export or copy of a report |
| D73 | feedback | Know that a small difference is drawn bigger than it is. Nothing on the screen says so | possible today | `systems.cpp:197` | **open**: nothing on screen says a small difference is drawn bigger |
| D74 | page | See earlier comparisons, before and after each change | possible today | `window.cpp:321` | **open**: one kept comparison only |
| D75 | feedback | Read more in a block's tip: files, last used, the rule that named it | possible today | `map.cpp:138` | **partly**: map tip is path and size only (map.cpp:150); Browse panel has files, last used, rule |
| D76 | navigation | Get from the Mess tab to the System app when the systems or EFI are the problem | possible today | `pages.cpp` | **partly**: strip "Finish and restart…" and "Open System…" on the two systems; nothing from Mess |
| D77 | feedback | Notice that the System app failed to start: six seconds in the status bar | possible today | `window.cpp:97` | **open**: still a status-line flash when the System app fails to start |
| D78 | settings | Find the window as it was left | possible today | `main.cpp:34` | **partly**: window size remembered (rememberWindowSize); page not; picture mode fixes 1100x700, no story |
| D79 | feedback | Trust the status bar: "This window only reads the disk", also while the agent it started removes files | possible today | `window.cpp:76` | **done**: status line "only “Let it all go” deletes anything from here", story checks it |
| D80 | feedback | See when each item was last used and why it is listed, without hovering | possible today | `pages.cpp:127` | **done**: Mess columns "Last used" and "The agent", panel says why (3a-mess) |
| D81 | action | Tick in bulk: only what clears without asking, none, only above a size | possible today | `pages.cpp:192` | **partly**: only "Tick only what clears without asking"; no none / above a size |

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
