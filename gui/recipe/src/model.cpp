#include "model.h"

#include <QJsonArray>
#include <QLocale>
#include <QUrl>

static QStringList strings(const QJsonValue &value)
{
    QStringList out;
    for (const QJsonValue &item : value.toArray())
        out << item.toString();
    return out;
}

QString Source::host() const
{
    return QUrl(git.isEmpty() ? url : git).host();
}

QString Source::pin() const
{
    if (!commit.isEmpty())
        return QObject::tr("pinned to commit %1").arg(commit.left(10));
    if (checksumFrom == QLatin1String("publisher"))
        return QObject::tr("checked against the checksum the project publishes");
    if (checksumFrom == QLatin1String("signature"))
        return QObject::tr("checked against the project's signature");
    return QObject::tr("checked against a checksum the recipe's author took at first download");
}

Recipe Recipe::fromJson(const QJsonObject &object)
{
    Recipe r;
    r.name = object["name"].toString();
    r.version = object["version"].toString();
    r.description = object["description"].toString();
    r.path = object["path"].toString();

    const QJsonObject about = object["about"].toObject();
    r.kind = about["kind"].toString();
    r.category = about["category"].toString();
    r.title = about["title"].toString();
    r.details = about["details"].toString();
    r.homepage = about["homepage"].toString();
    r.license = about["license"].toString();
    r.afterInstall = about["after_install"].toString();
    r.programs = strings(about["adds"].toObject()["programs"]);
    r.menu = strings(about["adds"].toObject()["menu"]);
    r.keeps = strings(about["keeps"]);
    const QJsonObject notes = about["steps"].toObject();
    for (auto it = notes.begin(); it != notes.end(); ++it)
        r.notes.insert(it.key(), it.value().toString());

    const QJsonObject sources = object["sources"].toObject();
    for (auto it = sources.begin(); it != sources.end(); ++it) {
        const QJsonObject s = it.value().toObject();
        Source source{it.key(), s["url"].toString(), s["git"].toString(), s["commit"].toString(),
                      s["sha256"].toString(), s["checksum_from"].toString()};
        source.url.replace("{version}", r.version).replace("{name}", r.name);
        r.sources << source;
    }

    const QJsonObject needs = object["needs"].toObject();
    r.needs.network = strings(needs["network"]);
    r.needs.memoryMib = needs["memory_mib"].toInteger();
    r.needs.storageMaxBytes = needs["storage_max_bytes"].toInteger();
    r.needs.timeoutSecs = needs["timeout_secs"].toInteger();
    r.needs.networkMaxBytes = needs["network_max_bytes"].toInteger();

    r.deps = strings(object["deps"]);
    r.buildDeps = strings(object["build_deps"]);
    r.hooks = strings(object["hooks"]);
    return r;
}

bool UpdateItem::asksMore() const
{
    for (const Change &change : changes)
        if (change.kind == QLatin1String("added") || change.kind == QLatin1String("raised"))
            return true;
    return false;
}

const Recipe *Catalog::recipe(const QString &name) const
{
    for (const Recipe &r : recipes)
        if (r.name == name)
            return &r;
    return nullptr;
}

const InstalledItem *Catalog::installedItem(const QString &name) const
{
    for (const InstalledItem &i : installed)
        if (i.name == name)
            return &i;
    return nullptr;
}

const UpdateItem *Catalog::update(const QString &name) const
{
    for (const UpdateItem &u : updates)
        if (u.name == name)
            return &u;
    return nullptr;
}

static const QList<QPair<QString, const char *>> &categoryTable()
{
    static const QList<QPair<QString, const char *>> table = {
        {"office", QT_TRANSLATE_NOOP("QObject", "Office")},
        {"mail", QT_TRANSLATE_NOOP("QObject", "Mail")},
        {"chat", QT_TRANSLATE_NOOP("QObject", "Chat")},
        {"web", QT_TRANSLATE_NOOP("QObject", "Web")},
        {"media", QT_TRANSLATE_NOOP("QObject", "Media")},
        {"graphics", QT_TRANSLATE_NOOP("QObject", "Graphics")},
        {"development", QT_TRANSLATE_NOOP("QObject", "Developer tools")},
        {"games", QT_TRANSLATE_NOOP("QObject", "Games")},
        {"utilities", QT_TRANSLATE_NOOP("QObject", "Utilities")},
        {"drivers-codecs", QT_TRANSLATE_NOOP("QObject", "Drivers and codecs")},
    };
    return table;
}

QStringList Catalog::categories() const
{
    QStringList used;
    for (const auto &entry : categoryTable())
        for (const Recipe &r : recipes)
            if (r.listed() && r.category == entry.first) {
                used << entry.first;
                break;
            }
    return used;
}

QString categoryTitle(const QString &id)
{
    for (const auto &entry : categoryTable())
        if (entry.first == id)
            return QObject::tr(entry.second);
    return id;
}

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

QString hookTitle(const QString &hook)
{
    if (hook == QLatin1String("acquire"))
        return QObject::tr("Fetch");
    if (hook == QLatin1String("build"))
        return QObject::tr("Build");
    if (hook == QLatin1String("install"))
        return QObject::tr("Install");
    if (hook == QLatin1String("remove"))
        return QObject::tr("Remove");
    return hook;
}
