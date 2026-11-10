// For running from a checkout only: walk the whole lifecycle against the
// stand-in and save a picture at each point. It is also the check that every
// flow works: it fails when a step does not lead where it should.
#pragma once

#include <QString>

class Window;

int takeShots(Window &window, const QString &folder);
