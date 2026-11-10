// For running from a checkout only: a mode that walks the pages and saves a
// picture of each. The stand-in looks are in gui/kit/look.h.
#pragma once

#include <QString>

class Window;

int takeShots(Window &window, const QString &folder);

// Against the real `recipe`: show the recipe, install it, show what happened
// and what is installed, then remove it again. A picture at each point.
// With `readOnly`, nothing is installed: only the pages that read.
int tryRecipe(Window &window, const QString &name, const QString &folder, bool keep, bool readOnly);
