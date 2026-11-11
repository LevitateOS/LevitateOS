// What the person told the app: places it must never suggest. One file in the
// person's config folder, so it follows them and not the window.
#pragma once

#include <QString>
#include <QStringList>

// <config>/levitate/disk/never-suggest.json, {"paths": ["/home/me/vm-images"]}
QString neverFile();
QStringList readNever();
// False when the file could not be written.
bool writeNever(const QStringList &paths);
