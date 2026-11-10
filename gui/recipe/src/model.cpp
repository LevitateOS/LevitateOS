#include "model.h"

#include "words.h"

#include <QDateTime>
#include <QMap>
#include <QLocale>

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

    r.origin = object["source"].toString();
    r.deps = strings(object["deps"]);
    r.buildDeps = strings(object["build_deps"]);
    r.hooks = strings(object["hooks"]);
    return r;
}

QString Change::title() const
{
    static const QMap<QString, QString> titles{
        {"network", QObject::tr("Network")},          {"network_max_bytes", QObject::tr("Download")},
        {"inputs", QObject::tr("Reads")},             {"outputs", QObject::tr("Writes")},
        {"memory_mib", QObject::tr("Memory")},        {"storage_max_objects", QObject::tr("Files while it runs")},
        {"storage_max_bytes", QObject::tr("Disk while it runs")}, {"timeout_secs", QObject::tr("Time")}};
    return titles.value(field, field);
}

bool Change::isList() const
{
    return field == QLatin1String("network") || field == QLatin1String("inputs") || field == QLatin1String("outputs");
}

static QStringList listOf(const QString &value)
{
    return value == QLatin1String("none") ? QStringList() : value.split(QStringLiteral(", "), Qt::SkipEmptyParts);
}

QStringList Change::added() const
{
    QStringList found;
    const QStringList was = listOf(before);
    for (const QString &name : listOf(after))
        if (!was.contains(name))
            found << name;
    return found;
}

QStringList Change::removed() const
{
    QStringList found;
    const QStringList is = listOf(after);
    for (const QString &name : listOf(before))
        if (!is.contains(name))
            found << name;
    return found;
}

QStringList Change::stays() const
{
    QStringList found;
    const QStringList is = listOf(after);
    for (const QString &name : listOf(before))
        if (is.contains(name))
            found << name;
    return found;
}

QString Change::amount(const QString &value)
{
    const qsizetype space = value.indexOf(QLatin1Char(' '));
    bool isNumber = false;
    const qint64 number = value.left(space).toLongLong(&isNumber);
    if (space < 0 || !isNumber)
        return value;
    const QString unit = value.mid(space + 1);
    if (unit == QLatin1String("bytes"))
        return bytesText(number);
    if (unit == QLatin1String("MiB"))
        return bytesText(number * 1048576);
    if (unit == QLatin1String("s"))
        return number >= 120 ? counted(int(number / 60), QObject::tr("%1 minute"), QObject::tr("%1 minutes"))
                             : counted(int(number), QObject::tr("%1 second"), QObject::tr("%1 seconds"));
    if (unit == QLatin1String("objects"))
        return counted(int(number), QObject::tr("%1 file"), QObject::tr("%1 files"));
    return value;
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

const Origin *Catalog::origin(const QString &name) const
{
    for (const Origin &o : origins)
        if (o.name == name)
            return &o;
    return nullptr;
}

QString Catalog::qualified(const QString &name) const
{
    const Recipe *found = recipe(name);
    return found && !found->origin.isEmpty() ? found->origin + QLatin1Char(':') + name : name;
}

int Catalog::countFrom(const QString &origin) const
{
    int count = 0;
    for (const Recipe &r : recipes)
        count += r.listed() && r.origin == origin;
    return count;
}

QString Catalog::originsWords() const
{
    QStringList named;
    for (const Origin &o : origins)
        if (countFrom(o.name) > 0)
            named << o.shown();
    if (named.isEmpty())
        return QObject::tr("No source has recipes");
    if (named.size() == 1)
        return QObject::tr("Recipes from %1").arg(named.first());
    return QObject::tr("Recipes from %1 sources").arg(named.size());
}

QString Origin::shown() const
{
    if (!title.isEmpty())
        return title;
    if (name == QLatin1String("local"))
        return QObject::tr("your own recipes");
    if (name == QLatin1String("image"))
        return QObject::tr("the recipes shipped with the system");
    return name;
}

QString Origin::state() const
{
    if (!problem.isEmpty())
        return problem;
    if (!remote())
        return QObject::tr("A folder on this computer, read as it is");
    if (checked <= 0)
        return QObject::tr("Its list was never fetched");
    const QDateTime when = QDateTime::fromSecsSinceEpoch(checked);
    const QString time = QLocale().toString(when.time(), QLocale::ShortFormat);
    if (when.date() == QDate::currentDate())
        return QObject::tr("Its list was fetched today at %1").arg(time);
    return QObject::tr("Its list was fetched on %1").arg(QLocale().toString(when.date(), QLocale::ShortFormat));
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
