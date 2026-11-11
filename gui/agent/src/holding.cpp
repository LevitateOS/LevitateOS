#include "holding.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QObject>

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <vector>

#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace {

bool strictlyInside(const QString &real, const QString &home)
{
    return !home.isEmpty() && real.startsWith(home + QLatin1Char('/'));
}

bool sameOrInside(const QString &real, const QString &folder)
{
    return real == folder || real.startsWith(folder + QLatin1Char('/'));
}

QString joined(const QString &folder, const QString &name)
{
    return folder == QLatin1String("/") ? QLatin1Char('/') + name : folder + QLatin1Char('/') + name;
}

// A path with the links of the folders it is in resolved; the last part is
// kept as it is, link or not. Empty when the folder is not there.
QString resolveLeaf(const QString &path)
{
    if (!QDir::isAbsolutePath(path) || path == QLatin1String("/"))
        return {};
    const QFileInfo info(path);
    const QString parent = QFileInfo(info.absolutePath()).canonicalFilePath();
    if (parent.isEmpty())
        return {};
    return joined(parent, info.fileName());
}

// Where a folder is or would be, with links resolved as far as it exists.
// Empty when a link on the way leads nowhere.
QString resolveMaybeMissing(const QString &path)
{
    QStringList below;
    QString at = path;
    struct stat st;
    while (::lstat(QFile::encodeName(at).constData(), &st) != 0) {
        if (at == QLatin1String("/"))
            return {};
        below.prepend(QFileInfo(at).fileName());
        at = QFileInfo(at).absolutePath();
    }
    const QString real = QFileInfo(at).canonicalFilePath();
    if (real.isEmpty())
        return {};
    QString result = real;
    for (const QString &part : std::as_const(below))
        result = joined(result, part);
    return result;
}

QString fdPath(int fd)
{
    char buffer[4096];
    const ssize_t length = ::readlink(QFile::encodeName(QStringLiteral("/proc/self/fd/%1").arg(fd)).constData(), buffer, sizeof buffer - 1);
    return length > 0 ? QFile::decodeName(QByteArray(buffer, length)) : QString();
}

// The length of everything under a path, links not followed.
qint64 lengthOf(int parentFd, const char *name, int depth = 0)
{
    struct stat st;
    if (fstatat(parentFd, name, &st, AT_SYMLINK_NOFOLLOW) != 0)
        return 0;
    if (!S_ISDIR(st.st_mode) || depth > 200)
        return st.st_size;
    const int fd = openat(parentFd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (fd < 0)
        return 0;
    DIR *dir = fdopendir(fd);
    if (!dir) {
        ::close(fd);
        return 0;
    }
    qint64 total = 0;
    while (dirent *entry = readdir(dir)) {
        const char *child = entry->d_name;
        if (child[0] == '.' && (!child[1] || (child[1] == '.' && !child[2])))
            continue;
        total += lengthOf(dirfd(dir), child, depth + 1);
    }
    closedir(dir);
    return total;
}

// Opens each folder of an absolute path from "/" down, following no link,
// making the ones that are missing. Returns the last folder's descriptor, or
// -1 and why.
int openFolderMaking(const QString &path, QString *why)
{
    int fd = ::open("/", O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    const QStringList parts = path.split(QLatin1Char('/'), Qt::SkipEmptyParts);
    for (int index = 0; index < parts.size() && fd >= 0; ++index) {
        const QByteArray name = QFile::encodeName(parts[index]);
        int next = openat(fd, name.constData(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
        if (next < 0 && errno == ENOENT) {
            // The holding folder itself is for the person alone.
            if (mkdirat(fd, name.constData(), index + 1 == parts.size() ? 0700 : 0755) == 0 || errno == EEXIST)
                next = openat(fd, name.constData(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
        }
        const int error = errno;
        ::close(fd);
        if (next < 0) {
            *why = error == ELOOP || error == ENOTDIR
                       ? QObject::tr("a folder on the way to the holding folder is a link or a file")
                       : QObject::tr("the holding folder could not be made: %1").arg(QString::fromLocal8Bit(strerror(error)));
            return -1;
        }
        fd = next;
    }
    return fd;
}

// One folder below `fd`, made when missing, without following a link.
int openBelowMaking(int fd, const QByteArray &name, QString *why)
{
    int next = openat(fd, name.constData(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (next < 0 && errno == ENOENT) {
        if (mkdirat(fd, name.constData(), 0700) == 0 || errno == EEXIST)
            next = openat(fd, name.constData(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    }
    if (next < 0) {
        const int error = errno;
        *why = error == ELOOP || error == ENOTDIR
                   ? QObject::tr("a part of the way to it inside the holding folder is a link or a file")
                   : QObject::tr("a folder for it in the holding folder could not be made: %1").arg(QString::fromLocal8Bit(strerror(error)));
    }
    return next;
}

QByteArray readSmall(int dirFd, const char *name, bool *present)
{
    const int fd = openat(dirFd, name, O_RDONLY | O_NOFOLLOW | O_CLOEXEC);
    *present = fd >= 0;
    if (fd < 0)
        return {};
    QByteArray all;
    char buffer[8192];
    for (ssize_t count; (count = ::read(fd, buffer, sizeof buffer)) > 0;)
        all.append(buffer, count);
    ::close(fd);
    return all;
}

bool writeSmall(int dirFd, const char *name, const QByteArray &bytes)
{
    const char *part = "manifest.json.part";
    unlinkat(dirFd, part, 0);
    const int fd = openat(dirFd, part, O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, 0600);
    if (fd < 0)
        return false;
    qsizetype done = 0;
    while (done < bytes.size()) {
        const ssize_t count = ::write(fd, bytes.constData() + done, bytes.size() - done);
        if (count <= 0) {
            ::close(fd);
            unlinkat(dirFd, part, 0);
            return false;
        }
        done += count;
    }
    ::fsync(fd);
    ::close(fd);
    if (renameat(dirFd, part, dirFd, name) != 0) {
        unlinkat(dirFd, part, 0);
        return false;
    }
    return true;
}

} // namespace

bool readSetAside(const QJsonValue &value, const QString &home, SetAsidePlan *plan, QString *problem)
{
    plan->homeReal = QFileInfo(home).canonicalFilePath();
    if (!value.isObject() || !value.toObject()["holding"].isString() || !value.toObject()["places"].isArray()
        || (value.toObject().contains("ask_first") && !value.toObject()["ask_first"].isArray())) {
        *problem = QObject::tr("set_aside must name the holding folder and give the list of places.");
        return false;
    }
    const QJsonObject object = value.toObject();
    auto strings = [&](const QJsonArray &array, QStringList *out, const QString &what) {
        for (const QJsonValue &item : array) {
            if (!item.isString()) {
                *problem = QObject::tr("%1 must be a list of full paths.").arg(what);
                return false;
            }
            *out << item.toString();
        }
        return true;
    };
    QStringList places, askFirst;
    if (!strings(object["places"].toArray(), &places, QStringLiteral("The places")) || !strings(object["ask_first"].toArray(), &askFirst, QStringLiteral("ask_first")))
        return false;

    const QString holding = object["holding"].toString();
    if (!QDir::isAbsolutePath(holding)) {
        *problem = QObject::tr("The holding folder \"%1\" is not a full path, so nothing may be set aside there.").arg(holding);
        return false;
    }
    plan->holding = QDir::cleanPath(holding);
    plan->holdingReal = resolveMaybeMissing(plan->holding);
    if (plan->holdingReal.isEmpty() || !strictlyInside(plan->holdingReal, plan->homeReal)) {
        *problem = QObject::tr("The holding folder %1 is outside your home folder, so nothing may be set aside there.").arg(holding);
        return false;
    }
    for (const QString &given : std::as_const(places)) {
        if (!QDir::isAbsolutePath(given)) {
            *problem = QObject::tr("The place \"%1\" is not a full path, so it may not be set aside.").arg(given);
            return false;
        }
        const QString clean = QDir::cleanPath(given);
        const QString key = resolveLeaf(clean);
        if (key.isEmpty() || !strictlyInside(key, plan->homeReal)) {
            *problem = QObject::tr("The place %1 is outside your home folder, so it may not be set aside.").arg(given);
            return false;
        }
        if (sameOrInside(key, plan->holdingReal) || sameOrInside(plan->holdingReal, key)) {
            *problem = QObject::tr("The place %1 and the holding folder lie one inside the other, so it may not be set aside.").arg(given);
            return false;
        }
        if (!plan->keyOf.contains(clean)) {
            plan->places << clean;
            plan->keyOf.insert(clean, key);
        }
    }
    for (const QString &given : std::as_const(askFirst)) {
        const QString clean = QDir::cleanPath(given);
        if (!plan->keyOf.contains(clean)) {
            *problem = QObject::tr("The place %1 is marked to ask first but is not on the list of places.").arg(given);
            return false;
        }
        if (!plan->askFirst.contains(clean))
            plan->askFirst << clean;
    }
    plan->given = true;
    return true;
}

QString approvedPlace(const SetAsidePlan &plan, const QString &path)
{
    if (!QDir::isAbsolutePath(path))
        return {};
    const QString key = resolveLeaf(QDir::cleanPath(path));
    if (key.isEmpty())
        return {};
    for (auto it = plan.keyOf.cbegin(); it != plan.keyOf.cend(); ++it)
        if (it.value() == key)
            return it.key();
    return {};
}

Moved moveIntoHolding(const SetAsidePlan &plan, const QString &given)
{
    auto refuse = [](const QString &why) { return Moved{false, why, 0}; };
    const QString key = plan.keyOf.value(given);
    if (key.isEmpty())
        return refuse(QObject::tr("it is not on the list"));

    // What is there now. A link is moved as a link, never followed, and one
    // that leads out of the home folder is left alone.
    struct stat st;
    if (::lstat(QFile::encodeName(key).constData(), &st) != 0)
        return refuse(QObject::tr("it is not there any more"));
    if (S_ISLNK(st.st_mode)) {
        const QString target = QFileInfo(key).canonicalFilePath();
        if (target.isEmpty())
            return refuse(QObject::tr("it is a link to nothing, so it is left alone"));
        if (!strictlyInside(target, plan.homeReal))
            return refuse(QObject::tr("it is a link that leads out of your home folder, so it is left alone"));
    }

    QString why;
    const int holdingFd = openFolderMaking(plan.holdingReal, &why);
    if (holdingFd < 0)
        return refuse(why);
    // The folder the descriptor really is: nothing may have been swapped for a link.
    if (fdPath(holdingFd) != plan.holdingReal) {
        ::close(holdingFd);
        return refuse(QObject::tr("the holding folder is not where it was a moment ago"));
    }

    // The manifest first, so that a manifest that cannot be added to stops it before anything moves.
    QJsonArray entries;
    bool present = false;
    const QByteArray old = readSmall(holdingFd, "manifest.json", &present);
    if (present) {
        QJsonParseError error;
        const QJsonDocument document = QJsonDocument::fromJson(old, &error);
        if (error.error != QJsonParseError::NoError || !document.isObject() || document.object()["version"].toInt() != 1) {
            ::close(holdingFd);
            return refuse(QObject::tr("the manifest.json in the holding folder is not one that can be added to"));
        }
        entries = document.object()["entries"].toArray();
    }
    for (const QJsonValue &entry : std::as_const(entries))
        if (entry.toObject()["from"].toString() == given) {
            ::close(holdingFd);
            return refuse(QObject::tr("it is already in the holding folder"));
        }

    // Its place beneath the holding folder: its own full path.
    const QString held = given.mid(1);
    const QStringList parts = held.split(QLatin1Char('/'), Qt::SkipEmptyParts);
    int destination = ::dup(holdingFd);
    for (int index = 0; index + 1 < parts.size(); ++index) {
        const int next = openBelowMaking(destination, QFile::encodeName(parts[index]), &why);
        ::close(destination);
        destination = next;
        if (next < 0) {
            ::close(holdingFd);
            return refuse(why);
        }
    }

    const QString parent = QFileInfo(key).absolutePath();
    const int parentFd = ::open(QFile::encodeName(parent).constData(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    auto closeAll = [&] {
        if (parentFd >= 0)
            ::close(parentFd);
        ::close(destination);
        ::close(holdingFd);
    };
    if (parentFd < 0 || fdPath(parentFd) != parent) {
        closeAll();
        return refuse(QObject::tr("the folder it is in changed while it was being set aside"));
    }
    const QByteArray name = QFile::encodeName(QFileInfo(key).fileName());
    const QByteArray leaf = QFile::encodeName(parts.last());
    const qint64 bytes = lengthOf(parentFd, name.constData());

    // Never replaces anything, whatever appeared since the checks above.
    if (renameat2(parentFd, name.constData(), destination, leaf.constData(), RENAME_NOREPLACE) != 0) {
        const int error = errno;
        closeAll();
        if (error == EEXIST)
            return refuse(QObject::tr("something is already at its place in the holding folder"));
        if (error == EXDEV)
            return refuse(QObject::tr("it is on another disk than the holding folder, so it cannot be set aside by moving"));
        if (error == ENOENT)
            return refuse(QObject::tr("it is not there any more"));
        return refuse(QObject::tr("it could not be moved: %1").arg(QString::fromLocal8Bit(strerror(error))));
    }

    entries.append(QJsonObject{{"from", given}, {"held", held}, {"bytes", bytes}});
    const QByteArray manifest = QJsonDocument(QJsonObject{{"version", 1}, {"entries", entries}}).toJson();
    if (!writeSmall(holdingFd, "manifest.json", manifest)) {
        // A move nothing records would be lost to the person: put it back.
        renameat2(destination, leaf.constData(), parentFd, name.constData(), RENAME_NOREPLACE);
        closeAll();
        return refuse(QObject::tr("the record of what is set aside could not be written, so it was put back"));
    }
    closeAll();
    return Moved{true, QString(), bytes};
}
