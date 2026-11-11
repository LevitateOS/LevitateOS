// What the stories of the three apps check about runs that outlive their
// window: a run the helper hosts is started by one window, the window closes,
// and another window finds the run in Activity, follows it, answers it,
// approves its administrator step, and stops it. The checks are the same in
// every app; the app only says how its own window hands a request to the agent
// (`Starter`). Everything runs through the built levitate-agent with the
// stand-in Codex, in a state folder of its own.
#pragma once

#include <QString>
#include <QStringList>

#include <functional>

class Frame;
class QWidget;

namespace livestory {

using Save = std::function<void(QWidget &, const QString &)>;
using Expect = std::function<void(const QString &, bool)>;

// A window of the app that handed a request to the agent, and its frame.
struct Started {
    QWidget *window = nullptr;
    Frame *frame = nullptr;
};
// Makes a window whose agent is levitate-agent, with the stand-in Codex telling
// `story` (`LEVITATE_STANDIN_STORY`), has it hand `request` to the agent the way
// the app does, and returns it. The run is listed in Activity under `request`.
using Starter = std::function<Started(const QString &story, const QString &request)>;

// Points the helper's state, runtime and settings folders and the stand-in
// Codex into `folder`/live, for what follows, and keeps what they were.
// `fixtures` is gui/agent/fixtures (where `codex` is).
void prepare(const QString &folder, const QString &fixtures);
// What the stand-in Codex tells next (`LEVITATE_STANDIN_STORY`); given back by `finish`.
void useStory(const QString &story);
// The checks, from the window `mine` (its Activity is the one that follows).
// `prefix` names the pictures (`PREFIX-live-...`).
void run(Frame &mine, QWidget &mineWindow, const Starter &start, const Save &save, const Expect &expect,
         const QString &prefix);
// Ends every run that is still going (also when the checks failed halfway),
// and gives the environment back as it was.
void finish();

// The stand-in Codex's log of the last `prepare`: the calls it was given.
QString codexLog();

// A picture run that is not told of its stand-ins must refuse to start: runs this
// very program with `--shots` and nothing named, and checks the exit code, the
// sentence, and that it did not start anything.
void guardRefuses(const Expect &expect, const QString &folder, const QString &mustName);

} // namespace livestory
