// For running from a checkout only: a stand-in look for each desktop, and a
// mode that walks the pages and saves a picture of each.
#pragma once

#include <QString>

class Window;

// "levitate": Breeze-like dark with the LevitateOS accent. "acorn": light
// paper with the AcornOS accent. On a real desktop the app takes the
// desktop's own style and colours and this is never called.
void applyLook(const QString &look);

int takeShots(Window &window, const QString &folder);

// Against the real `recipe`: show the recipe, install it, show what happened
// and what is installed, then remove it again. A picture at each point.
// With `readOnly`, nothing is installed: only the pages that read.
int tryRecipe(Window &window, const QString &name, const QString &folder, bool keep, bool readOnly);
