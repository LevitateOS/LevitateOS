#include "words.h"

#include <QLocale>
#include <QObject>

QString bytesText(qint64 bytes)
{
    const double mib = bytes / 1048576.0;
    if (mib >= 1024) {
        const double gib = mib / 1024;
        const bool whole = qAbs(gib - qRound(gib)) < 0.05;
        return QObject::tr("%1 GiB").arg(QLocale().toString(gib, 'f', whole ? 0 : 1));
    }
    // Something smaller than half a MiB is not "0 MiB".
    if (bytes > 0 && mib < 0.5)
        return QObject::tr("%1 KiB").arg(QLocale().toString(qMax(1, qRound(bytes / 1024.0))));
    return QObject::tr("%1 MiB").arg(QLocale().toString(qRound(mib)));
}

QString counted(qsizetype count, const QString &one, const QString &many)
{
    return (count == 1 ? one : many).arg(count);
}
