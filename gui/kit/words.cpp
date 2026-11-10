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
    return QObject::tr("%1 MiB").arg(QLocale().toString(qRound(mib)));
}

QString counted(qsizetype count, const QString &one, const QString &many)
{
    return (count == 1 ? one : many).arg(count);
}
