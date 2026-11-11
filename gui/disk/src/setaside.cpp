#include "setaside.h"

#include <QDateTime>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QObject>
#include <QRegularExpression>
#include <QSaveFile>

#include <algorithm>
#include <cerrno>
#include <cstdio>
#include <cstring>
#include <set>
#include <utility>
#include <vector>

#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace {

bool isBatchName(const QString &name)
{
    static const QRegularExpression pattern(QStringLiteral("^\\d{8}-\\d{6}$"));
    return pattern.match(name).hasMatch();
}

bool lstatOf(const QString &path, struct stat *st)
{
    return ::lstat(QFile::encodeName(path).constData(), st) == 0;
}

// The room a tree takes, as the scan counts it: each file once, its length
// when `apparent`, else the room it takes on disk. Does not follow links and
// does not leave the disk it started on.
qint64 treeBytes(int parentFd, const char *name, bool apparent, dev_t device, std::set<std::pair<dev_t, ino_t>> &seen)
{
    struct stat st;
    if (fstatat(parentFd, name, &st, AT_SYMLINK_NOFOLLOW) != 0)
        return 0;
    if (!S_ISDIR(st.st_mode)) {
        if (st.st_nlink > 1 && !seen.insert({st.st_dev, st.st_ino}).second)
            return 0;
        return apparent || !S_ISREG(st.st_mode) ? qint64(st.st_size) : qint64(st.st_blocks) * 512;
    }
    if (st.st_dev != device)
        return 0;
    const int fd = openat(parentFd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (fd < 0)
        return 0;
    DIR *dir = fdopendir(fd);
    if (!dir) {
        close(fd);
        return 0;
    }
    qint64 total = 0;
    while (dirent *entry = readdir(dir)) {
        const char *child = entry->d_name;
        if (child[0] == '.' && (!child[1] || (child[1] == '.' && !child[2])))
            continue;
        total += treeBytes(dirfd(dir), child, apparent, device, seen);
    }
    closedir(dir);
    return total;
}

qint64 pathBytes(const QString &path, bool apparent)
{
    struct stat st;
    if (!lstatOf(path, &st))
        return 0;
    const int top = open("/", O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (top < 0)
        return 0;
    const QByteArray local = QFile::encodeName(path);
    // Relative to "/", so the walk opens every part itself and follows no link
    // below the start; the start may be a link, and is then measured as one.
    std::set<std::pair<dev_t, ino_t>> seen;
    const qint64 total = treeBytes(top, local.constData() + (local.startsWith('/') ? 1 : 0), apparent, st.st_dev, seen);
    close(top);
    return total;
}

// Whether a record can be acted on at all: it must agree with itself, and the
// path beneath the holding folder must be plain folders from there down.
QString recordProblem(const QString &root, const HeldPlace &place)
{
    const QString &from = place.from, &held = place.held;
    if (!isBatchName(place.batch))
        return QObject::tr("its holding folder has a name this window does not know");
    if (from.isEmpty() || !from.startsWith(QLatin1Char('/')) || QDir::cleanPath(from) != from || from == QLatin1String("/"))
        return QObject::tr("the place it came from is not written as a plain full path");
    if (held.isEmpty() || held.startsWith(QLatin1Char('/')) || QDir::cleanPath(held) != held || held == QLatin1String("..")
        || held.startsWith(QLatin1String("../")))
        return QObject::tr("where it is held is not written as a plain path inside the holding folder");
    if (held != from.mid(1))
        return QObject::tr("the record says it is held somewhere other than under the path it came from");
    // Every folder from the holding folder down to the place must be a real folder.
    QString at = root + QLatin1Char('/') + place.batch;
    struct stat st;
    if (!lstatOf(at, &st) || !S_ISDIR(st.st_mode))
        return QObject::tr("the holding folder is not a folder");
    const QStringList parts = held.split(QLatin1Char('/'));
    for (int index = 0; index + 1 < parts.size(); ++index) {
        at += QLatin1Char('/') + parts[index];
        if (!lstatOf(at, &st))
            return QString(); // not there: nothing to be wrong with; presence is checked by the caller
        if (!S_ISDIR(st.st_mode))
            return QObject::tr("a part of the way to it inside the holding folder is a link or a file");
    }
    return QString();
}

std::vector<HeldPlace> readManifest(const QString &root, const QString &batch, QString *problem)
{
    std::vector<HeldPlace> places;
    QFile file(root + QLatin1Char('/') + batch + QStringLiteral("/manifest.json"));
    if (!file.open(QIODevice::ReadOnly)) {
        *problem = QObject::tr("%1 has no manifest.json that can be read").arg(batch);
        return places;
    }
    QJsonParseError error;
    const QJsonDocument document = QJsonDocument::fromJson(file.readAll(), &error);
    if (error.error != QJsonParseError::NoError || !document.isObject() || document.object()["version"].toInt() != 1) {
        *problem = QObject::tr("the manifest of %1 is not one this window reads (version 1)").arg(batch);
        return places;
    }
    for (const QJsonValue &value : document.object()["entries"].toArray()) {
        const QJsonObject entry = value.toObject();
        HeldPlace place;
        place.batch = batch;
        place.from = entry["from"].toString();
        place.held = entry["held"].toString();
        place.recorded = entry["bytes"].toInteger();
        places.push_back(place);
    }
    return places;
}

bool writeManifest(const QString &root, const QString &batch, const std::vector<HeldPlace> &places)
{
    QJsonArray entries;
    for (const HeldPlace &place : places)
        entries.append(QJsonObject{{"from", place.from}, {"held", place.held}, {"bytes", place.recorded}});
    QSaveFile file(root + QLatin1Char('/') + batch + QStringLiteral("/manifest.json"));
    if (!file.open(QIODevice::WriteOnly))
        return false;
    file.write(QJsonDocument(QJsonObject{{"version", 1}, {"entries", entries}}).toJson());
    return file.commit();
}

// Deletes what `name` is inside `parentFd`, following no link and staying on
// one disk. A folder that cannot be entered is made enterable first, since a
// build can leave read-only folders and this one is ours to empty.
bool removeTree(int parentFd, const char *name, dev_t device, QString *why)
{
    struct stat st;
    if (fstatat(parentFd, name, &st, AT_SYMLINK_NOFOLLOW) != 0)
        return true; // already gone
    if (!S_ISDIR(st.st_mode)) {
        if (unlinkat(parentFd, name, 0) != 0) {
            *why = QObject::tr("%1 could not be deleted: %2").arg(QFile::decodeName(name), QString::fromLocal8Bit(strerror(errno)));
            return false;
        }
        return true;
    }
    if (st.st_dev != device) {
        *why = QObject::tr("%1 is another disk mounted inside the holding folder; it was left alone").arg(QFile::decodeName(name));
        return false;
    }
    const int fd = openat(parentFd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (fd < 0) {
        *why = QObject::tr("%1 could not be opened: %2").arg(QFile::decodeName(name), QString::fromLocal8Bit(strerror(errno)));
        return false;
    }
    if (st.st_uid == geteuid() && (st.st_mode & 0700) != 0700)
        fchmod(fd, st.st_mode | 0700);
    DIR *dir = fdopendir(fd);
    if (!dir) {
        close(fd);
        return false;
    }
    std::vector<QByteArray> names;
    while (dirent *entry = readdir(dir)) {
        const char *child = entry->d_name;
        if (child[0] == '.' && (!child[1] || (child[1] == '.' && !child[2])))
            continue;
        names.push_back(child);
    }
    bool ok = true;
    for (const QByteArray &child : names)
        ok = removeTree(dirfd(dir), child.constData(), device, why) && ok;
    closedir(dir);
    if (!ok)
        return false;
    if (unlinkat(parentFd, name, AT_REMOVEDIR) != 0) {
        *why = QObject::tr("%1 could not be deleted: %2").arg(QFile::decodeName(name), QString::fromLocal8Bit(strerror(errno)));
        return false;
    }
    return true;
}

} // namespace

QString holdingRoot(const QString &home)
{
    return home + QStringLiteral("/.local/share/levitate/set-aside");
}

QString newHolding(const QString &home)
{
    return holdingRoot(home) + QLatin1Char('/') + QDateTime::currentDateTime().toString(QStringLiteral("yyyyMMdd-HHmmss"));
}

SetAside readSetAside(const QString &home, bool apparent)
{
    SetAside found;
    found.root = holdingRoot(home);
    struct stat st, homeStat;
    if (!lstatOf(found.root, &st) || !S_ISDIR(st.st_mode))
        return found;
    if (::stat(QFile::encodeName(home).constData(), &homeStat) == 0)
        found.sameDisk = st.st_dev == homeStat.st_dev;

    QStringList names = QDir(found.root).entryList(QDir::Dirs | QDir::NoDotAndDotDot | QDir::NoSymLinks, QDir::Name);
    qint64 listed = 0;
    for (const QString &name : std::as_const(names)) {
        if (!isBatchName(name))
            continue;
        found.batches << name;
        found.bytes += pathBytes(found.root + QLatin1Char('/') + name, apparent);
        QString problem;
        for (HeldPlace place : readManifest(found.root, name, &problem)) {
            place.trouble = recordProblem(found.root, place);
            const QString at = found.root + QLatin1Char('/') + name + QLatin1Char('/') + place.held;
            struct stat held;
            place.present = place.trouble.isEmpty() && lstatOf(at, &held);
            if (place.present) {
                place.dir = S_ISDIR(held.st_mode);
                place.bytes = pathBytes(at, apparent);
                listed += place.bytes;
            } else if (place.trouble.isEmpty()) {
                place.trouble = QObject::tr("it is not in the holding folder any more");
            }
            found.places << place;
        }
        if (!problem.isEmpty())
            found.problems << problem;
    }
    found.unlisted = std::max<qint64>(0, found.bytes - listed);
    return found;
}

Outcome bringBack(const QString &home, const QStringList &zones, const HeldPlace &place)
{
    const QString root = holdingRoot(home);
    auto refuse = [](const QString &why) { return Outcome{false, why, 0}; };
    if (const QString problem = recordProblem(root, place); !problem.isEmpty())
        return refuse(problem);
    // The place must belong to somewhere this window looks, and not to the holding folder itself.
    QString zone;
    for (const QString &candidate : zones)
        if (!candidate.isEmpty() && place.from.startsWith(candidate + QLatin1Char('/')) && candidate.size() > zone.size())
            zone = candidate;
    if (zone.isEmpty())
        return refuse(QObject::tr("it came from outside the folders this window looks at, so it will not move it there"));
    if (place.from == root || place.from.startsWith(root + QLatin1Char('/')))
        return refuse(QObject::tr("it points into the holding folder itself"));

    const QString heldAt = root + QLatin1Char('/') + place.batch + QLatin1Char('/') + place.held;
    struct stat st;
    if (!lstatOf(heldAt, &st))
        return refuse(QObject::tr("it is not in the holding folder any more"));

    // The way there must be plain folders, so nothing lands behind a link.
    const QStringList parts = place.from.mid(zone.size() + 1).split(QLatin1Char('/'));
    QString at = zone;
    for (int index = 0; index < parts.size(); ++index) {
        at += QLatin1Char('/') + parts[index];
        if (!lstatOf(at, &st))
            break;
        if (index + 1 == parts.size())
            return refuse(QObject::tr("something is there now: %1").arg(place.from));
        if (!S_ISDIR(st.st_mode))
            return refuse(QObject::tr("a folder on the way is a link or a file: %1").arg(at));
    }
    if (!QDir().mkpath(QFileInfo(place.from).absolutePath()))
        return refuse(QObject::tr("the folder it belongs in could not be made"));
    // Never replaces anything, whatever appeared since the check above.
    if (renameat2(AT_FDCWD, QFile::encodeName(heldAt).constData(), AT_FDCWD, QFile::encodeName(place.from).constData(),
                  RENAME_NOREPLACE) != 0) {
        const int error = errno;
        if (error == EEXIST)
            return refuse(QObject::tr("something is there now: %1").arg(place.from));
        if (error == EXDEV)
            return refuse(QObject::tr("the holding folder is on another disk than where it belongs"));
        return refuse(QObject::tr("it could not be moved: %1").arg(QString::fromLocal8Bit(strerror(error))));
    }

    // Take the record off the manifest, and tidy what is empty in the holding folder.
    QString problem;
    std::vector<HeldPlace> left;
    for (const HeldPlace &other : readManifest(root, place.batch, &problem))
        if (other.from != place.from)
            left.push_back(other);
    const QString batch = root + QLatin1Char('/') + place.batch;
    for (QString up = QFileInfo(heldAt).absolutePath(); up.size() > batch.size() && up.startsWith(batch + QLatin1Char('/'));
         up = QFileInfo(up).absolutePath())
        if (::rmdir(QFile::encodeName(up).constData()) != 0)
            break;
    if (left.empty()) {
        // Nothing else listed: the holding folder goes when it holds nothing but its manifest.
        QFile::remove(batch + QStringLiteral("/manifest.json"));
        if (::rmdir(QFile::encodeName(batch).constData()) != 0)
            writeManifest(root, place.batch, left);
        else
            ::rmdir(QFile::encodeName(root).constData());
    } else {
        writeManifest(root, place.batch, left);
    }
    return Outcome{true, QString(), place.bytes};
}

Outcome letGo(const QString &home, bool apparent)
{
    const QString root = holdingRoot(home);
    struct stat st;
    if (!lstatOf(root, &st))
        return Outcome{true, QObject::tr("there was nothing to let go"), 0};
    if (!S_ISDIR(st.st_mode))
        return Outcome{false, QObject::tr("the holding folder is a link or a file, so nothing was deleted"), 0};
    const int rootFd = open(QFile::encodeName(root).constData(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (rootFd < 0)
        return Outcome{false, QObject::tr("the holding folder could not be opened"), 0};
    Outcome outcome{true, QString(), 0};
    const QStringList names = QDir(root).entryList(QDir::Dirs | QDir::NoDotAndDotDot | QDir::NoSymLinks, QDir::Name);
    for (const QString &name : names) {
        if (!isBatchName(name))
            continue; // not a holding folder of ours: left alone
        const QByteArray local = QFile::encodeName(name);
        struct stat batch;
        if (fstatat(rootFd, local.constData(), &batch, AT_SYMLINK_NOFOLLOW) != 0 || !S_ISDIR(batch.st_mode))
            continue;
        std::set<std::pair<dev_t, ino_t>> seen;
        const qint64 size = treeBytes(rootFd, local.constData(), apparent, batch.st_dev, seen);
        QString why;
        if (removeTree(rootFd, local.constData(), batch.st_dev, &why))
            outcome.bytes += size;
        else if (outcome.ok) {
            outcome.ok = false;
            outcome.why = why;
        }
    }
    close(rootFd);
    ::rmdir(QFile::encodeName(root).constData()); // only when empty
    return outcome;
}
