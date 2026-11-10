// For running from a checkout only: walk the whole lifecycle against the
// stand-in and save a picture at each point. It is also the check that every
// flow works: it fails when a step does not lead where it should.
#pragma once

#include <QString>

class Window;

int takeShots(Window &window, const QString &folder);
// The same for the story the real recab offers, against `fixtures/recab-real`,
// the stand-in that speaks the real recab's own status and commands.
int takeRealShots(Window &window, const QString &folder);
