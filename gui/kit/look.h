// For running from a checkout only: a stand-in look for each desktop, and a
// pause for the picture modes.
#pragma once

#include <QString>
#include <QStringList>

#include <functional>

// "levitate": Breeze-like dark with the LevitateOS accent. "acorn": light
// paper with the AcornOS accent. On a real desktop an app takes the desktop's
// own style and colours and this is never called.
void applyLook(const QString &look);

// Let the event loop run for a while.
void settle(int milliseconds);
// Lets the event loop run until `holds()` is true, and a moment more. For the
// stories, which wait for the thing itself (an event, a file, a state) and
// never for a number of seconds: `seconds` is what it normally takes, and the
// limit is three times that and never less than a minute, because a busy
// machine answers slowly. Returns whether it held.
bool waitFor(const std::function<bool()> &holds, int seconds = 20);

// The guard of the picture modes (`--shots`). A picture run starts the backend
// programs of its app, the agent and the helper's own agents, and a root
// helper, and writes settings and records: run bare it would do that to the
// person's real computer. So it refuses to start unless each of them is named
// in the environment: `programs` (the app's own variables, each a stand-in
// that must be named and non-empty), `mayBeEmpty` (the root helper: set and
// empty counts as named, it then runs the command directly), and the ones every
// app shares (the helper, its Codex and Claude Code, its root helper, HOME, the
// XDG folders; HOME must not be the person's own home folder). Returns 0 when
// all are named, else says on standard error what is missing, in one sentence,
// and returns 2: the program's exit code. Also turns the notifier off unless
// the check named one, and marks the process as a picture run (`pictureMode`).
// Call it before the window is made: the window starts reading at once.
int guardPictureMode(const QString &program, const QStringList &programs, const QStringList &mayBeEmpty);
bool pictureMode();
