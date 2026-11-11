#include "client.h"

#include "link.h"
#include "record.h"
#include "sigwatch.h"

#include <QCoreApplication>
#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>
#include <QSocketNotifier>
#include <QThread>
#include <QTimer>

#include <cerrno>
#include <csignal>
#include <cstdio>
#include <fcntl.h>
#include <sys/syscall.h>
#include <unistd.h>

namespace {

// How a run that has ended is told to a window: as the exit code it always was.
int codeOf(const QString &state)
{
    return state == QLatin1String("done") ? 0 : state == QLatin1String("failed") ? 1 : 143;
}

bool alive(qint64 pid)
{
    return pid > 0 && (::kill(pid_t(pid), 0) == 0 || errno == EPERM);
}

// Starts the process that hosts the run: this same program, in a session of
// its own so that nothing that happens to this one (a closed window, a hangup)
// reaches it, with nothing of ours open but the pipe that carries the request.
pid_t spawnHost(const QString &id, const QByteArray &requestLine)
{
    int request[2];
    if (::pipe2(request, O_CLOEXEC) != 0)
        return -1;
    const QByteArray program = QFile::encodeName(QCoreApplication::applicationFilePath());
    const QByteArray ident = id.toUtf8();
    const QByteArray starter = QByteArray::number(qint64(getpid()));
    const char *argv[] = {program.constData(), "--host", ident.constData(), "--starter", starter.constData(), nullptr};
    // The host's own children must be reaped by it as usual; the setting for
    // ours (below) would be inherited across the exec otherwise.
    const pid_t pid = ::fork();
    if (pid == 0) {
        ::setsid();
        ::signal(SIGCHLD, SIG_DFL);
        ::dup2(request[0], 0);
        const int null = ::open("/dev/null", O_RDWR);
        ::dup2(null, 1);
        ::dup2(null, 2);
        // Whatever the window gave us stays with the window.
#ifdef SYS_close_range
        ::syscall(SYS_close_range, 3u, ~0u, 0u);
#else
        for (int fd = 3; fd < 4096; ++fd)
            ::close(fd);
#endif
        ::execv(program.constData(), const_cast<char *const *>(argv));
        ::_exit(127);
    }
    ::close(request[0]);
    if (pid > 0) {
        const QByteArray text = requestLine + '\n';
        qsizetype done = 0;
        while (done < text.size()) {
            const ssize_t count = ::write(request[1], text.constData() + done, size_t(text.size() - done));
            if (count < 0 && errno == EINTR)
                continue;
            if (count <= 0)
                break;
            done += count;
        }
    }
    ::close(request[1]);
    return pid;
}

} // namespace

Client::Client(const Settings &settings, Mode mode, const QString &id, QObject *parent)
    : QObject(parent)
    , settings_(settings)
    , mode_(mode)
    , id_(id)
{
}

void Client::run()
{
    // The window may go away while we write to it; that ends us, not by the
    // signal but by the failed write.
    ::signal(SIGPIPE, SIG_IGN);
    // The host is not waited for: it must not linger as a zombie when it ends
    // while we are still here (a stop would wait on it).
    ::signal(SIGCHLD, SIG_IGN);
    signals_ = new SigWatch({SIGTERM, SIGINT, SIGHUP}, this);
    connect(signals_, &SigWatch::raised, this, &Client::onSignal);
    input_ = new QSocketNotifier(0, QSocketNotifier::Read, this);
    connect(input_, &QSocketNotifier::activated, this, &Client::onInput);

    if (mode_ == Attach) {
        const QString path = recordPath(settings_, id_);
        if (path.isEmpty()) {
            fprintf(stderr, "levitate-agent: no run %s\n", qPrintable(id_));
            leave(1);
            return;
        }
        const int fd = liveHost(settings_, id_) > 0 ? sockets::connectTo(settings_.socketsDir, socketName(id_)) : -1;
        if (fd < 0) {
            fprintf(stderr, "levitate-agent: the run %s has ended or cannot be reached, so there is nothing to attach to. "
                            "Its record is --show %s.\n", qPrintable(id_), qPrintable(id_));
            leave(1);
            return;
        }
        started_ = true;
        connected(fd);
    }
}

void Client::onInput()
{
    char buffer[4096];
    const ssize_t count = ::read(0, buffer, sizeof buffer);
    if (count > 0) {
        pending_.append(buffer, count);
    } else if (count < 0 && (errno == EINTR || errno == EAGAIN)) {
        return;
    }
    qsizetype end;
    while (!leaving_ && (end = pending_.indexOf('\n')) >= 0) {
        const QByteArray line = pending_.left(end);
        pending_.remove(0, end + 1);
        if (line.trimmed().isEmpty())
            continue;
        if (!started_) {
            started_ = true;
            startRun(line);
            continue;
        }
        const QJsonObject object = QJsonDocument::fromJson(line).object();
        if (!object.contains("question") || !object.contains("answer"))
            continue;
        if (link_)
            link_->send(line);
        else
            queued_ << line;
    }
    if (count <= 0) {
        input_->setEnabled(false);
        stdinClosed_ = true;
        if (!started_) {
            failed(QStringLiteral("No request came on the input"));
            return;
        }
        // Nobody can answer through us any more. If the run waits for an
        // answer, we have nothing left to do for it; it waits without us.
        // Otherwise we go on relaying until it ends, as we always did.
        if (!question_.isEmpty())
            leave(75);
    }
}

void Client::startRun(const QByteArray &requestLine)
{
    const QJsonObject object = QJsonDocument::fromJson(requestLine).object();
    if (object["prompt"].toString().isEmpty()) {
        failed(QStringLiteral("The first line must be a request with a prompt"));
        return;
    }
    host_ = spawnHost(id_, requestLine);
    if (host_ < 0) {
        failed(QStringLiteral("The run could not be started"));
        return;
    }
    // The host makes its record and its socket within a moment.
    retry_ = new QTimer(this);
    connect(retry_, &QTimer::timeout, this, &Client::tryConnect);
    retry_->start(20);
}

void Client::tryConnect()
{
    const int fd = sockets::connectTo(settings_.socketsDir, socketName(id_));
    if (fd >= 0) {
        retry_->stop();
        connected(fd);
        return;
    }
    // A run that failed at once (or was over before we looked) is already
    // gone with its socket; its record says how it went.
    if (!alive(host_)) {
        retry_->stop();
        fromRecord();
    } else if (++attempts_ > 500) {
        retry_->stop();
        failed(QStringLiteral("The run did not start in time"));
    }
}

void Client::connected(int fd)
{
    link_ = new Link(fd, this);
    connect(link_, &Link::line, this, &Client::onHostLine);
    connect(link_, &Link::closed, this, &Client::onClosed);
    link_->send(QJsonDocument(QJsonObject{{"hello", mode_ == Attach ? "attach" : "relay"}}).toJson(QJsonDocument::Compact));
    for (const QByteArray &line : std::as_const(queued_))
        link_->send(line);
    queued_.clear();
    // Asked to stop while the run was still starting.
    if (stopping_)
        link_->send("{\"stop\":true}");
}

void Client::onHostLine(const QByteArray &line)
{
    const QJsonObject object = QJsonDocument::fromJson(line).object();
    if (object.contains("stderr")) {
        toWindow(2, object["stderr"].toString().toUtf8() + '\n');
    } else if (object.contains("stdout")) {
        toWindow(1, QByteArray::fromBase64(object["stdout"].toString().toLatin1()));
    } else if (object.contains("control")) {
        const QString control = object["control"].toString();
        if (control == QLatin1String("hello")) {
            host_ = object["host"].toInteger();
        } else if (control == QLatin1String("waiting")) {
            question_ = object["id"].toVariant().toString();
            if (stdinClosed_)
                leave(75);
        } else if (control == QLatin1String("answered")) {
            if (object["id"].toVariant().toString() == question_)
                question_.clear();
        } else if (control == QLatin1String("end")) {
            leave(object["code"].toInt(codeOf(object["state"].toString())));
        }
    } else if (object.contains("event")) {
        // An attached window reads the events as the record holds them.
        toWindow(2, line + '\n');
    }
}

void Client::onClosed()
{
    if (leaving_)
        return;
    if (stopping_) {
        leave(143);
        return;
    }
    // The run's process went without saying how it ended: killed hard.
    failed(QStringLiteral("The run's process ended without saying how"));
}

void Client::fromRecord()
{
    const QList<QJsonObject> lines = readRecord(recordPath(settings_, id_));
    if (lines.isEmpty()) {
        failed(QStringLiteral("The run could not be started"));
        return;
    }
    QString state;
    for (const QJsonObject &line : lines) {
        if (line["record"] == QLatin1String("end")) {
            state = line["state"].toString();
        } else if (line.contains("event") && line["event"] != QLatin1String("answer")) {
            QJsonObject plain = line;
            plain.remove("t");
            toWindow(2, QJsonDocument(plain).toJson(QJsonDocument::Compact) + '\n');
        }
    }
    if (state.isEmpty()) {
        failed(QStringLiteral("The run's process ended without saying how"));
        return;
    }
    leave(codeOf(state));
}

void Client::toWindow(int fd, const QByteArray &bytes)
{
    qsizetype done = 0;
    while (done < bytes.size()) {
        const ssize_t count = ::write(fd, bytes.constData() + done, size_t(bytes.size() - done));
        if (count < 0 && errno == EINTR)
            continue;
        if (count <= 0) {
            // Nobody reads any more. The run is not ours to end; it goes on.
            leave(141);
            return;
        }
        done += count;
    }
}

void Client::failed(const QString &reason)
{
    toWindow(2, QJsonDocument(QJsonObject{{"event", "failed"}, {"reason", reason}}).toJson(QJsonDocument::Compact) + '\n');
    leave(1);
}

void Client::leave(int code)
{
    if (leaving_)
        return;
    leaving_ = true;
    if (retry_)
        retry_->stop();
    if (link_)
        link_->close();
    // Queued: this may be called before the event loop runs (an attach that
    // finds nothing to attach to), when an exit would be lost.
    QMetaObject::invokeMethod(qApp, [code] { QCoreApplication::exit(code); }, Qt::QueuedConnection);
}

void Client::onSignal(int number)
{
    if (number == SIGHUP) {
        // The window's terminal went away. The run goes on without it.
        leave(129);
        return;
    }
    // SIGTERM and SIGINT are the window's Stop: the run ends.
    if (stopping_)
        return;
    stopping_ = true;
    if (link_) {
        link_->send("{\"stop\":true}");
    } else if (host_ > 0 && alive(host_)) {
        ::kill(pid_t(host_), SIGTERM);
    } else if (!started_ || host_ <= 0) {
        leave(143);
        return;
    }
    // The run ends its agent first, which can take a moment; not for ever.
    patience_ = new QTimer(this);
    patience_->setSingleShot(true);
    connect(patience_, &QTimer::timeout, this, [this] { leave(143); });
    patience_->start(8000);
}

int stopRun(const Settings &settings, const QString &id)
{
    const QString path = recordPath(settings, id);
    if (path.isEmpty()) {
        fprintf(stderr, "levitate-agent: no run %s\n", qPrintable(id));
        return 1;
    }
    const qint64 host = liveHost(settings, id);
    auto state = [&] {
        for (const QJsonObject &line : readRecord(path))
            if (line["record"] == QLatin1String("end"))
                return line["state"].toString();
        return QString();
    };
    if (host <= 0) {
        const QString ended = state();
        fprintf(stderr, "levitate-agent: the run %s has already ended (%s).\n", qPrintable(id),
                qPrintable(ended.isEmpty() ? QStringLiteral("stopped") : ended));
        return 0;
    }
    ::kill(pid_t(host), SIGTERM);
    // The run ends its agent and everything the agent started, writes the end
    // of its record, and exits; the record is what says it is over.
    for (int waited = 0; waited < 10000; waited += 50) {
        if (!state().isEmpty() && liveHost(settings, id) == 0)
            break;
        QThread::msleep(50);
    }
    if (state().isEmpty()) {
        ::kill(pid_t(host), SIGKILL);
        fprintf(stderr, "levitate-agent: the run %s did not stop in ten seconds and was killed; what its agent started may still be running.\n",
                qPrintable(id));
        return 1;
    }
    printf("%s\n", QJsonDocument(QJsonObject{{"id", id}, {"state", state()}}).toJson(QJsonDocument::Compact).constData());
    return 0;
}
