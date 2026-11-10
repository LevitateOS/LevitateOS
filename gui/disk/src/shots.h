// For running from a checkout only: look at a folder, save a picture of what
// was found and of the hand-over, and with a stand-in agent of the result.
#pragma once

#include <QString>

class Window;

// `handOver` false: only look and picture (safe on a real home folder).
int takeShots(Window &window, const QString &folder, bool handOver);
