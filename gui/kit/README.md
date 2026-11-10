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
| `run.*` | `Run`, one command and its JSON events; an agent run takes `setPrompt(prompt, app, title)` |
| `work.*` | `WorkPage`, a long run with its steps, output and the agent's questions |
| `refresh.*` | `Refresher`: F5, Ctrl+R and coming back to the window |
| `frame.*` | the window frame (below) |
| `activity.*` | the Activity page, owned by the frame |
| `nextsystem.*` | `NextSystem`: reads `recab status --json` for the strip |

## The frame

One widget that an app's window sets as its central widget. It is the rail on
the left (the three apps, then the app's own entries, then Activity), the app's
pages in the middle, a strip under them about the next system, and a status
line. Do not add entries that lead nowhere: only Activity is shared.

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
void flash(QString);                     // said for a few seconds in place of the status
void setAgent(QString program);          // right of the status line (see below)
void setAgentNote(QString);              // while the agent works; empty restores its name
void setOwnRun(qint64 pid);              // the agent process this window waits for; 0 when it ended
ActivityPage *activity() const;
slot readAgain();                        // connect Refresher::asked to it
signals: chosen(key)  finishRequested()  goToRun()
```

- `chosen(key)` comes from the person's click or keyboard, for the app's entries
  and for Activity (`Frame::activityKey()`, "activity"). Switch your stack to
  the page of that key. For Activity the frame has shown its page already.
- `finishRequested()` is the strip's button, "Finish and restart…". Show the
  page where the next system is finished.
- `goToRun()` is "Go to it" on Activity for this window's own live run: show
  the page of that run (the work page).
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

It reports nothing when there is no next system or the status cannot be read.

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
//                            frame_->setOwnRun(run->pid());
// when it ends:              frame_->setOwnRun(0);   // Activity reads the record again
```

Titles of runs should be what the person asked, in a short line: Activity lists
a run under it. Pass `app` as the name Activity shows ("Recipe", "System",
"Disk").

## Activity

The agent's runs from all three apps, read from `levitate-agent --list` and
`--show ID` (program: `LEVITATE_ACTIVITY_AGENT`, default `levitate-agent`),
without holding the window up. The frame owns the page, gives its rail entry a
badge (runs waiting or running; amber while one waits), reads it on first show,
on `readAgain()` and after `setOwnRun(0)`. A run waiting for an answer can only
be answered in the window that started it; Activity says so and does not answer.
Only agent runs are kept: installs and restarts are not recorded anywhere yet.

## Checks

There are no unit tests. Each app's picture mode (`--shots`) is the check; the
frame, Activity and the strip are checked in `gui/recab/src/shots.cpp`. Runs for
Activity to show come from `gui/agent/fixtures/records`.
