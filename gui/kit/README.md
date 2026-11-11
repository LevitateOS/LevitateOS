# gui/kit

What `gui/recipe`, `gui/recab` and `gui/disk` share. Each app lists the kit
sources it uses in its own `CMakeLists.txt` (`../kit/frame.cpp` and so on) and
adds `../kit` to its include path. Colours come from the desktop's palette
only: no stylesheet.

| File | What |
|---|---|
| `look.*` | a stand-in look from a checkout (`--look`), and `settle` for the picture modes |
| `parts.*` | small widgets: text, `Card`, `Panel`, `Chip`, `ElidedLabel`, `ClickLabel`, `Mood` colours |
| `page.*` | `Page`, the base of every page (it rebuilds itself; says "loading") |
| `run.*` | `Run`, one command and its JSON events; an agent run takes `setPrompt(prompt, app, title, mayWrite)`, knows its id, is stopped with `--stop` and let go of with `release()` (below) |
| `work.*` | `WorkPage`, a long run with its steps, output and the agent's questions |
| `refresh.*` | `Refresher`: F5, Ctrl+R and coming back to the window |
| `frame.*` | the window frame (below) |
| `activity.*` | the Activity page, owned by the frame |
| `settings.*` | the Settings page, owned by the frame, and `rememberWindowSize` |
| `nextsystem.*` | `NextSystem`: reads `recab status --json` for the strip |
| `before.*` | `BeforePage`: what will happen, shown before anything starts (below) |
| `livestory.*` | the checks of the stories about runs that outlive their window (below); only the picture modes use it |

## The frame

One widget that an app's window sets as its central widget. It is the rail on
the left (the three apps, then the app's own entries, then Activity), the app's
pages in the middle, a strip under them about the next system, and a status
line. Do not add entries that lead nowhere: only Activity and Settings are
shared, and the frame adds both under the app's own entries.

```cpp
Frame(Frame::App app)                    // Frame::Recipe, Frame::System, Frame::Disk
void setContent(QWidget *content);       // your page stack; shown whenever Activity is not
void addEntry(key, title);               // the app's own rail entries, in order
void setBadge(key, count, warn = false); // 0 shows none; warn = amber (something waits for the person)
void select(key);                        // mark an entry and show the content (no signal)
void follow(key);                        // your pages moved on by themselves: mark that entry,
                                         //   but stay on Activity when it is showing
void choose(key);                        // as a click would (marks, shows, emits chosen)
void setNextSystem(QStringList titles);  // the strip; empty hides it
void setStripAllowed(bool);              // keep the strip away from the page that says it itself
void setStatus(QString);                 // left of the status line: what was read and when
void addStatusWidget(QWidget *);         // beside it, e.g. "Read again"
void flash(QString);                     // said for a few seconds in place of the status; empty ends it at once
void startApp(App);                      // starts another app as the rail does; says so when it cannot
QStringList entryTitles() const;         // the rail's entries in order, without the three apps
QString ownRun() const;                  // the id setOwnRun was given
void setAgent(QString program);          // right of the status line (see below)
void setAgentNote(QString);              // while the agent works; empty restores its name
void setOwnRun(Run *run);                // the agent run this window follows: its id is known a moment after the start
void setOwnRun(QString id);              // empty when it ended
ActivityPage *activity() const;
SettingsPage *settings() const;
static QString settingsKey();            // "settings"
bool settingsShown() const;
slot readAgain();                        // connect Refresher::asked to it
signals: chosen(key)  finishRequested()  goToRun()
```

- `chosen(key)` comes from the person's click or keyboard, for the app's entries
  and for Activity (`Frame::activityKey()`, "activity"). Switch your stack to
  the page of that key. For Activity the frame has shown its page already.
- Settings (`Frame::settingsKey()`) is chosen by the person like Activity, but
  the frame does not emit `chosen` for it: it is the frame's own page and the
  app has nothing to switch to. `follow(key)` leaves it showing, as it does
  Activity; the person leaving it for one of the app's entries does emit `chosen`.
- `finishRequested()` is the strip's button, "Finish and restart…". Show the
  page where the next system is finished.
- `goToRun()` is "Go to it" on Activity for this window's own live run: show
  the page of that run (the work page).
- **A run outlives its window.** The window knows its run by the id the helper
  names in its first line (`Run::idKnown`), not by a process id. When the person
  leaves for another entry of the rail while the run works, the status line says
  once that the agent carries on and is in Activity. When the window closes, the
  frame lets go of the runs it follows (and the ones its pages and Activity
  started or attached to): the input is closed and the helper hung up (SIGHUP),
  never SIGTERM, which would stop the run; the status line says so, and the run
  goes on. Stopping is `Run::stop()`: for a run whose id is known it is
  `levitate-agent --stop ID`, otherwise (a stand-in) SIGTERM to the program.
  The helper's exit code 75 ("the run still waits or works without me") is
  `Run::carriesOn`, not a failure and not an end.
- Choosing another app in the rail starts its program detached
  (`recipe-gui`, `recab-gui`, `disk-gui`; override with `LEVITATE_APP_RECIPE`,
  `LEVITATE_APP_SYSTEM`, `LEVITATE_APP_DISK`). If that fails the status line
  says so.
- `setAgent(program)`: when the file name is `levitate-agent` the frame runs
  `levitate-agent --status` and shows "Agent: Codex, signed in" (or "not
  signed in", "is not installed"); any other program, a stand-in, is shown by
  its file name.
- The rail is reachable by Tab; Up and Down move between entries; Space or
  Return chooses. Every entry has an accessible name, with its badge.

The strip says "The next system holds N changes." with the changes in a few
words and the button. Recipe and Disk do not hold the status, so they ask:

```cpp
auto *next = new NextSystem(recabProgram, this);   // runs `recab status --json`
connect(next, &NextSystem::known, this, [this, next] { frame_->setNextSystem(next->titles()); });
next->read();   // again after what changes it, and when Refresher asks
```

It reports nothing when there is no next system or the status cannot be read, and
nothing while the running system is on trial: recab still calls the other system
"next" then (the preparation that made the running system is kept until the trial
is kept), but it is the system from before, so there is nothing to finish and the
strip would say what is not so (System's own model reads it the same way).

### An app adopting the frame

```cpp
// window.cpp
frame_ = new Frame(Frame::Disk);
frame_->setContent(stack_);                       // the pages the window already had
frame_->addEntry("map", tr("Mess"));
frame_->addEntry("compare", tr("Compare"));
frame_->select("map");
frame_->setAgent(agentProgram);                   // the program the window starts as its agent
frame_->setStatus(tr("Read at %1").arg(time));
setCentralWidget(frame_);                         // and drop the QStatusBar

connect(frame_, &Frame::chosen, this, [this](const QString &key) {
    if (key != Frame::activityKey())
        stack_->setCurrentWidget(pageFor(key));
});
connect(frame_, &Frame::finishRequested, this, [this] { /* open Overview of System, or show a note */ });
connect(frame_, &Frame::goToRun, this, [this] { stack_->setCurrentWidget(work_); });
connect(refresher_, &Refresher::asked, frame_, &Frame::readAgain);

// when an agent run starts:  run->setPrompt(prompt, "Disk", title);  run->start();
//                            frame_->setOwnRun(run);
// when it ends:              frame_->setOwnRun(QString());   // Activity reads the record again
```

Titles of runs should be what the person asked, in a short line: Activity lists
a run under it. Pass `app` as the name Activity shows ("Recipe", "System",
"Disk").

## Before

`BeforePage` is the page an app shows before it starts something that changes
the computer. It says nothing about recipes, disks or systems: the app puts the
words in. From the top: the title as a question, the line "Nothing has started.
This is what will happen if you go on.", then the sections that were filled
(left empty, a section is not shown), then the foot with "Cancel" and one named
button. It is a `Page`: add it to your stack and give it content when you need it.

```cpp
before_ = new BeforePage;                       // once, into the stack
BeforePage::Content c;
c.title = tr("Install Krita?");
c.whereTitle = tr("Where it goes");             // a choice between places: two or more are
c.choices = {{"pc", tr("For this computer"), tr("Into the next system."), tr("Asks for the password.")},
             {"me", tr("For me"), tr("Into your own folder."), tr("No password.")}};
c.chosen = "pc";                                // ...one is a statement, none is no section
c.happensTitle = tr("What happens");            // plain sentences, one line each
c.happens = {tr("Krita 5.3.1 is fetched and built.")};
c.tableTitle = tr("What it may use");           // two columns, then a line under them
c.table = {{tr("Network"), tr("download.kde.org")}, {tr("Memory"), tr("up to 4 GiB")}};
c.tableNote = tr("Anything else is refused.");
c.foot = tr("From the main source."); c.linkWords = tr("Read the recipe");   // link only with words
c.go = tr("Install");
before_->show(c);                               // draws it; call again to draw it anew
stack_->setCurrentWidget(before_);

connect(before_, &BeforePage::cancelled, ...);          // Cancel, and Escape
connect(before_, &BeforePage::confirmed, this, [](const QString &choice) { /* start it */ });
connect(before_, &BeforePage::choiceChanged, this, [](const QString &choice) { /* show(...) again if the words depend on it */ });
connect(before_, &BeforePage::linkClicked, ...);
```

- Picking a card draws the page again with the new choice at once and emits
  `choiceChanged(key)`; an app whose sentences depend on the choice answers with
  `show()` of new content (the scroll position stays).
- `confirmed(key)` carries the key picked (the only choice's key when there is
  one, empty when there are none).
- The sections scroll; Cancel and the button do not. The cards use the
  palette's highlight for the one picked; nothing else is coloured.
- For a check: `pageText()` (every label and button of the current page),
  `choiceNames()`, `chosen()`, `choose(key)`. A page drawn just now is shown
  once the event loop has run, so a check settles before clicking its button.
- The Recipe app is the first user (`gui/recipe/src/beforewords.*` has its
  words); System and Disk list `before.cpp` in their `CMakeLists.txt` to use it.

## Activity

The agent's runs from all three apps, read from `levitate-agent --list` and
`--show ID` (program: `LEVITATE_ACTIVITY_AGENT`, default `levitate-agent`),
without holding the window up. The frame owns the page and gives its rail entry a
badge (runs waiting or running; amber while one waits). It reads on first show, on
`readAgain()`, after `setOwnRun("")`, and **by itself every two seconds while the
page is on screen** (a timer that runs only between its show and hide events; the
list is drawn again only when it changed, so what the person is typing stays). The
badge and the counts therefore follow runs of other windows without anyone pressing
anything while Activity is shown; while it is not shown they are as of the last
reading.

A run that is going and has its socket (`attachable`) is **followed live**: the page
runs `levitate-agent --attach ID`, takes the replay of what the run has done, and then
what it does as it does it. The detail shows the request, "What it does now" (the
step, and the last lines the run printed), "What it did" (a row for each command, file
and search), and, when the run waits, the question:

- a question with choices is answered with a button for each choice, as the work page
  does; one without is a line to type in and "Answer";
- an administrator step or a command outside the sandbox shows the command, why, and
  "Approve" and "Refuse";
- a click is sent through the attached client; the card says the answer was sent until
  the helper reports it, and then the page shows "Answered: ..." for everyone, so two
  windows that show the same run both see that it was answered and by what. The helper
  counts an answer once;
- "Stop" for a running or waiting run is `levitate-agent --stop ID`; what the run did
  stays done, and the page then shows how it ended.

The page lets go of the run (the input closed and a hang-up, never SIGTERM) when
another run is chosen or the page is hidden; the run goes on. A run that has ended
shows its record, as before. "Go to it" stays for this window's own live run, and the
question can be answered on either page: the work page learns that it was answered
from Activity when the agent says anything after it. Only agent runs are kept:
installs and restarts are not recorded anywhere yet.

## Settings

One page, the same in all three apps, owned by the frame and listed under
Activity in the rail. It reads `levitate-agent --status` and `--settings`
(program: `LEVITATE_ACTIVITY_AGENT`, default `levitate-agent`, as Activity does)
without holding the window up: it says it is asking, and a failed reading is
said with "Try again". It reads when first chosen, and again on `readAgain()`
once it has been read.

Sections: the agent that does the work (one row per agent the helper lists, today
Codex and Claude Code, and "Another program"), "Try it", what the agent may do without
asking (read and look things up: always; run commands as you: "Ask each time" or
"Allowed"; run a command as administrator: always asks), Notices (one row: "Tell me
when the agent needs an answer or has finished, and no window is watching", Yes or
No), and the windows.

- The page changes nothing itself. A choice runs `levitate-agent --set agent=...`,
  `--set run_commands=...` or `--set notices=...`; "Sign in…" runs `--sign-in` and
  shows what it prints (an address to open) in the row until it ends; "Sign out…"
  asks in the row first, then runs `--sign-out`. The helper signs in and out the agent
  that is chosen, so only that row has the buttons; the other says "Choose it to sign
  in or out here.". Afterwards the page reads again and shows what the
  helper reports, not what was clicked. A refusal or a failure is the helper's
  own sentence, shown under the row it belongs to, never a dialog.
- "Choose…" opens a file dialog; the path goes through `--set agent=PATH`.
- Claude Code is a row like Codex (installed or not, signed in or not, chosen, sign in
  and out through the helper), with one line that says what is true of it: this side
  has not been tried with a real Claude Code yet (the helper's adapter was written
  from the documentation and has run only against a stand-in).
- "Try it" runs `levitate-agent` with "Say hello in one sentence. Do not run any
  command and do not change any file.", app = this window's app, title "Say hello",
  and shows "It answered: ..." or why it failed. The run is Activity's like any
  other: the frame reads Activity when it ends.
- After every change the frame asks `--status` again, so "Agent: Codex, signed in"
  follows at once (only when the window's agent is `levitate-agent`).
- The windows: `rememberWindowSize(window, "recab", {1100, 700}, remember)`
  gives a window the size it had when it was closed (QSettings, organisation
  "LevitateOS", file `windows`, one key per app), 1100 by 700 the first time. Each
  app's `main.cpp` calls it instead of `resize`. With `remember` false (the
  picture modes) nothing is read or written.

Checks use `SettingsPage::pageText()`, `press(words)`, `pickCommands(words)`,
`pickNotices(on)`, `chooseProgram(path)`, `notice()`, `checkedAgent()` (the agent's
id, or "other"), `noticesInForce()` and `busy()`; see
`gui/recab/src/shots.cpp` (`settingsShots`) for the whole story, and the other
two apps' shots for the entry and the page opening.

### Not done

Each of these is in the design and not shown, because nothing is behind it:

- **Notices have never been shown on a desktop.** The helper shows them (the row
  above switches them) through `gdbus call`; Plasma's and LXQt's notification servers
  have not been seen showing one, and a click on it does nothing.
- **Per-app settings** (Recipe's default for "this computer", System's "ask when to
  restart", Disk's places and rules): no app keeps such a setting yet, and the helper's file holds only the agent's.
- **Anything about updates** (checking daily, notices for them): there is no
  update check to schedule.
- **"Write a recipe into your own recipes" without asking**: only partly. Recipe's
  "Ask for software" lets the agent write in the person's own recipe folder (`may_write`)
  and the page says so; there is no setting to go further.
- **Claude Code has never run against a real one** (see the row above).

## Checks

There are no unit tests. Each app's picture mode (`--shots`) is the check; the
frame, Activity and the strip are checked in `gui/recab/src/shots.cpp`, and runs for
Activity to show come from `gui/agent/fixtures/records`.

**The guard.** A picture run starts the app's backend programs, the agent, the
helper's own agents and a root helper, and writes settings and records: run bare it
would do that to the person's real computer. `guardPictureMode` (`look.*`, called by
each app's `main.cpp` before the window is made, since the window starts reading at
once) makes `--shots` refuse to start, with a sentence that names what is missing and
exit code 2, unless these are named in the environment: the app's own programs (see
its README), `LEVITATE_ACTIVITY_AGENT`, `LEVITATE_AGENT_CODEX`,
`LEVITATE_AGENT_CLAUDE`, `HOME` (not the person's own home folder),
`XDG_CONFIG_HOME`, `XDG_STATE_HOME`, `XDG_DATA_HOME`, `XDG_RUNTIME_DIR`, and the root
helpers (`LEVITATE_AGENT_ROOT_HELPER` and the app's; set and empty counts as named,
and runs the command directly). The notifier defaults to off in a picture run
(`LEVITATE_AGENT_NOTIFIER` set to a stand-in is respected), and the rail does not start
an app that `LEVITATE_APP_*` does not name. Each app's story runs itself bare and checks
that it refuses.

**Live runs.** `livestory.*` has the checks that the three stories share about runs
that outlive their window; each app only says how its window hands a request to the
agent (`Starter`). With the helper and the stand-in Codex, in a state folder of the
story's own, it checks: a window learns the id of its run; Activity of another window
lists it and counts it without a press; leaving a page says once that the run carries
on; closing the window says so and the run goes on waiting; another window follows it
and shows its choices as buttons; a second window shows the same run; an answer from
one window is seen by the other (and counted once in the record); an own window's page
learns that its question was answered from Activity; an administrator step is approved
from Activity; a working run is followed live and stopped from Activity (the agent and
what it started are gone); nothing is left. `waitFor` (`look.*`) is what the stories
wait with: for the thing itself, never a number of seconds; its limit is three times
what the thing normally takes and never less than a minute, because a busy machine
answers slowly.
