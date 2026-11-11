// One desktop notice, for a run that needs the person or has ended while no
// window is watching it.
#pragma once

#include "settings.h"

// Shows the notice and waits (a few seconds at most) until the program that
// shows it has ended, so that a run about to exit does not take it with it.
// Does nothing when the person turned notices off. A notice that cannot be
// shown is not an error: the run is the work, the notice is a courtesy.
void notify(const Settings &settings, const QString &title, const QString &text);
