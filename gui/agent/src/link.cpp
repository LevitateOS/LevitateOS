#include "link.h"

#include <QDir>
#include <QFile>
#include <QSocketNotifier>

#include <cerrno>
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/un.h>
#include <unistd.h>

Link::Link(int fd, QObject *parent)
    : QObject(parent)
    , fd_(fd)
{
    ::fcntl(fd_, F_SETFL, ::fcntl(fd_, F_GETFL) | O_NONBLOCK);
    ::fcntl(fd_, F_SETFD, FD_CLOEXEC);
    reader_ = new QSocketNotifier(fd_, QSocketNotifier::Read, this);
    connect(reader_, &QSocketNotifier::activated, this, &Link::readable);
    writer_ = new QSocketNotifier(fd_, QSocketNotifier::Write, this);
    writer_->setEnabled(false);
    connect(writer_, &QSocketNotifier::activated, this, &Link::writable);
}

Link::~Link()
{
    if (fd_ >= 0)
        ::close(fd_);
}

void Link::readable()
{
    char buffer[8192];
    const ssize_t count = ::read(fd_, buffer, sizeof buffer);
    if (count > 0) {
        in_.append(buffer, count);
        qsizetype end;
        // The slot may close this link; look again each time round.
        while (fd_ >= 0 && (end = in_.indexOf('\n')) >= 0) {
            const QByteArray one = in_.left(end);
            in_.remove(0, end + 1);
            emit line(one);
        }
    } else if (count == 0 || (errno != EINTR && errno != EAGAIN)) {
        close();
    }
}

void Link::send(const QByteArray &text)
{
    if (fd_ < 0)
        return;
    out_ += text + '\n';
    writable();
}

void Link::writable()
{
    while (fd_ >= 0 && !out_.isEmpty()) {
        const ssize_t count = ::send(fd_, out_.constData(), size_t(out_.size()), MSG_NOSIGNAL);
        if (count > 0) {
            out_.remove(0, count);
        } else if (count < 0 && errno == EINTR) {
            continue;
        } else if (count < 0 && (errno == EAGAIN || errno == EWOULDBLOCK)) {
            break;
        } else {
            close();
            return;
        }
    }
    if (fd_ < 0)
        return;
    writer_->setEnabled(!out_.isEmpty());
    if (out_.isEmpty() && closing_)
        close();
}

void Link::closeWhenSent()
{
    closing_ = true;
    if (out_.isEmpty())
        close();
}

void Link::close()
{
    if (fd_ < 0)
        return;
    reader_->setEnabled(false);
    writer_->setEnabled(false);
    ::close(fd_);
    fd_ = -1;
    emit closed();
}

Listener::Listener(int fd, QObject *parent)
    : QObject(parent)
    , fd_(fd)
{
    notifier_ = new QSocketNotifier(fd_, QSocketNotifier::Read, this);
    connect(notifier_, &QSocketNotifier::activated, this, [this] {
        const int client = ::accept4(fd_, nullptr, nullptr, SOCK_CLOEXEC | SOCK_NONBLOCK);
        if (client >= 0)
            emit accepted(client);
    });
}

Listener::~Listener()
{
    ::close(fd_);
}

namespace sockets {
namespace {

// The path of a socket is limited to about 100 bytes, which a long folder
// name breaks. Naming the folder by the descriptor we hold open on it keeps
// the address short whatever the folder is called.
bool address(int folderFd, const QString &name, sockaddr_un *out)
{
    std::memset(out, 0, sizeof *out);
    out->sun_family = AF_UNIX;
    const QByteArray bytes = name.toUtf8();
    const int written = std::snprintf(out->sun_path, sizeof out->sun_path, "/proc/self/fd/%d/%s", folderFd, bytes.constData());
    return written > 0 && size_t(written) < sizeof out->sun_path;
}

int openFolder(const QString &folder)
{
    return ::open(QFile::encodeName(folder).constData(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
}

} // namespace

int listenAt(const QString &folder, const QString &name, QString *problem)
{
    QDir().mkpath(folder);
    ::chmod(QFile::encodeName(folder).constData(), 0700);
    const int dir = openFolder(folder);
    if (dir < 0) {
        *problem = QStringLiteral("The folder %1 for the runs' sockets could not be opened.").arg(folder);
        return -1;
    }
    sockaddr_un addr;
    const int fd = ::socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC | SOCK_NONBLOCK, 0);
    // A socket left by a run that was killed hard has the same name only if
    // the id repeats; the old one is of no use then.
    ::unlinkat(dir, QFile::encodeName(name).constData(), 0);
    if (fd < 0 || !address(dir, name, &addr) || ::bind(fd, reinterpret_cast<sockaddr *>(&addr), sizeof addr) != 0
        || ::listen(fd, 16) != 0) {
        *problem = QStringLiteral("The run's socket could not be made in %1 (%2).").arg(folder, QString::fromLocal8Bit(std::strerror(errno)));
        if (fd >= 0)
            ::close(fd);
        ::close(dir);
        return -1;
    }
    ::fchmodat(dir, QFile::encodeName(name).constData(), 0600, 0);
    ::close(dir);
    return fd;
}

int connectTo(const QString &folder, const QString &name)
{
    const int dir = openFolder(folder);
    if (dir < 0)
        return -1;
    sockaddr_un addr;
    const int fd = ::socket(AF_UNIX, SOCK_STREAM | SOCK_CLOEXEC, 0);
    if (fd < 0 || !address(dir, name, &addr) || ::connect(fd, reinterpret_cast<sockaddr *>(&addr), sizeof addr) != 0) {
        if (fd >= 0)
            ::close(fd);
        ::close(dir);
        return -1;
    }
    ::close(dir);
    return fd;
}

bool exists(const QString &folder, const QString &name)
{
    return QFile::exists(folder + "/" + name);
}

void remove(const QString &folder, const QString &name)
{
    QFile::remove(folder + "/" + name);
}

} // namespace sockets
