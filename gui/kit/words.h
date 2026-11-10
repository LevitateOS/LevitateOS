// Sizes and counts as words. Shared by the front ends.
#pragma once

#include <QString>

QString bytesText(qint64 bytes);
// "1 program" or "4 programs": the words for a count.
QString counted(qsizetype count, const QString &one, const QString &many);
