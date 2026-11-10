// For running from a checkout only: a stand-in look for each desktop, and a
// pause for the picture modes.
#pragma once

#include <QString>

// "levitate": Breeze-like dark with the LevitateOS accent. "acorn": light
// paper with the AcornOS accent. On a real desktop an app takes the desktop's
// own style and colours and this is never called.
void applyLook(const QString &look);

// Let the event loop run for a while.
void settle(int milliseconds);
