#include "rules.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QSaveFile>
#include <QStandardPaths>

QString neverFile()
{
    return QStandardPaths::writableLocation(QStandardPaths::GenericConfigLocation)
           + QStringLiteral("/levitate/disk/never-suggest.json");
}

QStringList readNever()
{
    QFile file(neverFile());
    if (!file.open(QIODevice::ReadOnly))
        return {};
    QStringList paths;
    for (const QJsonValue &value : QJsonDocument::fromJson(file.readAll()).object()["paths"].toArray())
        if (value.toString().startsWith(QLatin1Char('/')))
            paths << value.toString();
    return paths;
}

bool writeNever(const QStringList &paths)
{
    QDir().mkpath(QFileInfo(neverFile()).absolutePath());
    QSaveFile file(neverFile());
    if (!file.open(QIODevice::WriteOnly))
        return false;
    file.write(QJsonDocument(QJsonObject{{"paths", QJsonArray::fromStringList(paths)}}).toJson());
    return file.commit();
}
