#include "session.h"

#include "adapter.h"
#include "link.h"
#include "notify.h"
#include "sigwatch.h"

#include <QCoreApplication>
#include <QDir>
#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QSocketNotifier>
#include <QTimer>

#include <cerrno>
#include <csignal>
#include <unistd.h>

namespace {

QByteArray compact(const QJsonObject &object)
{
    return QJsonDocument(object).toJson(QJsonDocument::Compact);
}

// Folders the agent may write in. They must be full paths of folders that
// exist inside the person's home folder; a request for anything else is
// refused whole, so that no part of it is quietly dropped.
bool checkFolders(const QStringList &given, const QString &home, QStringList *checked, QString *problem)
{
    const QString inside = QFileInfo(home).canonicalFilePath();
    for (const QString &folder : given) {
        if (!QDir::isAbsolutePath(folder)) {
            *problem = QStringLiteral("The folder \"%1\" is not a full path, so the agent is not allowed to write there.").arg(folder);
            return false;
        }
        const QFileInfo info(folder);
        if (!info.exists() || !info.isDir()) {
            *problem = QStringLiteral("The folder %1 does not exist, so the agent cannot be allowed to write there.").arg(folder);
            return false;
        }
        // Symbolic links are followed first: a link inside the home folder
        // that leads out of it is outside.
        const QString real = info.canonicalFilePath();
        if (inside.isEmpty() || (real != inside && !real.startsWith(inside + QLatin1Char('/')))) {
            *problem = QStringLiteral("The folder %1 is outside your home folder, so the agent is not allowed to write there.").arg(folder);
            return false;
        }
        *checked << real;
    }
    return true;
}

} // namespace

Session::Session(const Settings &settings, const QString &id, qint64 starter, QObject *parent)
    : QObject(parent)
    , settings_(settings)
    , record_(settings)
    , id_(id)
    , starter_(starter)
{
    age_.start();
}

void Session::run()
{
    // A window may go away while we write to it, and nobody is at a terminal
    // that could hang up: neither may end a run.
    ::signal(SIGPIPE, SIG_IGN);
    ::signal(SIGHUP, SIG_IGN);
    // Nothing of ours may keep a folder busy that the person removes later.
    QDir::setCurrent(settings_.home);
    signals_ = new SigWatch({SIGTERM, SIGINT}, this);
    connect(signals_, &SigWatch::raised, this, [this](int) { stopRequested(QStringLiteral("it was asked to stop")); });
    input_ = new QSocketNotifier(0, QSocketNotifier::Read, this);
    connect(input_, &QSocketNotifier::activated, this, &Session::onInput);
}

void Session::onInput()
{
    char buffer[4096];
    const ssize_t count = ::read(0, buffer, sizeof buffer);
    if (count > 0) {
        pending_.append(buffer, count);
    } else if (count < 0 && (errno == EINTR || errno == EAGAIN)) {
        return;
    }
    const qsizetype end = pending_.indexOf('\n');
    if (end >= 0 || count <= 0) {
        input_->setEnabled(false);
        const QByteArray line = end >= 0 ? pending_.left(end) : pending_;
        pending_.clear();
        if (line.trimmed().isEmpty()) {
            // The helper that starts us always sends the request first.
            QCoreApplication::exit(1);
            return;
        }
        begin(line);
    }
}

void Session::begin(const QByteArray &line)
{
    const QJsonObject object = QJsonDocument::fromJson(line).object();
    started_ = true;
    request_ = {object["prompt"].toString(), object["app"].toString(), object["title"].toString(), line, {}, {}};

    QJsonObject extra;
    QStringList given;
    const bool listed = !object.contains("may_write") || object["may_write"].isArray();
    if (object.contains("may_write")) {
        extra["may_write"] = object["may_write"];
        for (const QJsonValue &folder : object["may_write"].toArray())
            given << folder.toString();
    }
    if (object.contains("set_aside"))
        extra["set_aside"] = object["set_aside"];
    record_.open(id_, request_.app, request_.title, request_.prompt, starter_, extra);

    // Windows may connect from here on; the record is there for them to read.
    QString problem;
    const int fd = sockets::listenAt(settings_.socketsDir, socketName(id_), &problem);
    if (fd < 0) {
        send({{"event", "failed"}, {"reason", problem}});
        finish(QStringLiteral("failed"), 1);
        return;
    }
    listener_ = new Listener(fd, this);
    connect(listener_, &Listener::accepted, this, &Session::onAccepted);

    if (!listed) {
        send({{"event", "failed"}, {"reason", QStringLiteral("may_write must be a list of folders.")}});
        finish(QStringLiteral("failed"), 1);
        return;
    }
    if (!given.isEmpty() && !checkFolders(given, settings_.home, &request_.mayWrite, &problem)) {
        send({{"event", "failed"}, {"reason", problem}});
        finish(QStringLiteral("failed"), 1);
        return;
    }
    if (object.contains("set_aside") && !readSetAside(object["set_aside"], settings_.home, &request_.setAside, &problem)) {
        send({{"event", "failed"}, {"reason", problem}});
        finish(QStringLiteral("failed"), 1);
        return;
    }
    adapter_->begin(request_);
}

void Session::onAccepted(int fd)
{
    auto *link = new Link(fd, this);
    greeting_ << link;
    connect(link, &Link::line, this, [this, link](const QByteArray &line) { onClientLine(link, line); });
    connect(link, &Link::closed, this, [this, link] { onClientClosed(link); });
}

void Session::onClientLine(Link *link, const QByteArray &line)
{
    const QJsonObject object = QJsonDocument::fromJson(line).object();
    if (greeting_.contains(link)) {
        // Something that connects and says nothing (a check that the socket
        // is there) is not a window, and is not counted as one.
        const QString hello = object["hello"].toString();
        if (hello != QLatin1String("relay") && hello != QLatin1String("attach"))
            return;
        greeting_.removeOne(link);
        const Client client{link, hello == QLatin1String("attach")};
        clients_ << client;
        everAttached_ = true;
        link->send(compact({{"control", "hello"}, {"id", id_}, {"host", qint64(getpid())}}));
        // The record is read and the client added in one step, so it misses
        // nothing and sees nothing twice.
        replay(client);
        if (asking_)
            link->send(compact({{"control", "waiting"}, {"id", question_}}));
        if (finished_) {
            link->send(compact({{"control", "end"}, {"state", state_}, {"code", exitCode_}}));
            link->closeWhenSent();
        }
        return;
    }
    if (object["stop"].toBool()) {
        stopRequested(QStringLiteral("it was stopped from a window"));
        return;
    }
    if (object.contains("question") && object.contains("answer"))
        answer(line);
}

void Session::onClientClosed(Link *link)
{
    greeting_.removeOne(link);
    for (int i = 0; i < clients_.size(); ++i)
        if (clients_[i].link == link)
            clients_.removeAt(i--);
    link->deleteLater();
    if (finished_) {
        if (clients_.isEmpty() && greeting_.isEmpty())
            QCoreApplication::exit(exitCode_);
        return;
    }
    // The last window left while the run waits for it: say so once.
    if (clients_.isEmpty() && asking_ && !asked_) {
        asked_ = true;
        tell(QStringLiteral("waits for an answer"));
    }
}

void Session::replay(const Client &client)
{
    for (const QJsonObject &line : readRecord(record_.path())) {
        if (!line.contains("event"))
            continue;
        if (client.attach) {
            client.link->send(compact(line));
        } else if (line["event"] != QLatin1String("answer")) {
            // What an app reads has never had the time in it, nor an answer.
            QJsonObject plain = line;
            plain.remove("t");
            client.link->send(compact({{"stderr", QString::fromUtf8(compact(plain))}}));
        }
    }
}

void Session::note(const QJsonObject &event)
{
    const QString kind = event["event"].toString();
    if (kind == QLatin1String("question")) {
        asking_ = true;
        asked_ = false;
        question_ = event["id"].toVariant().toString();
    } else if (kind == QLatin1String("agent") && !event["summary"].toString().isEmpty()) {
        summary_ = event["summary"].toString();
    } else if (kind == QLatin1String("failed")) {
        failure_ = event["reason"].toString();
    }
}

void Session::broadcast(const QByteArray &app, const QJsonObject &stamped)
{
    const QByteArray relay = compact({{"stderr", QString::fromUtf8(app)}});
    const QByteArray attach = compact(stamped);
    for (const Client &client : std::as_const(clients_))
        client.link->send(client.attach ? attach : relay);
}

void Session::control(const QJsonObject &message)
{
    const QByteArray bytes = compact(message);
    for (const Client &client : std::as_const(clients_))
        client.link->send(bytes);
}

void Session::send(const QJsonObject &event)
{
    if (finished_)
        return;
    note(event);
    const QJsonObject stamped = record_.event(event);
    broadcast(compact(event), stamped);
    if (event["event"] == QLatin1String("question")) {
        control({{"control", "waiting"}, {"id", question_}});
        if (nobodyWatching()) {
            asked_ = true;
            tell(QStringLiteral("waits for an answer"));
        }
    }
}

void Session::forward(const QByteArray &line)
{
    if (finished_)
        return;
    const QJsonObject object = line.startsWith('{') ? QJsonDocument::fromJson(line).object() : QJsonObject();
    const QJsonObject event = object.contains("event") ? object : QJsonObject{{"event", "output"}, {"line", QString::fromUtf8(line)}};
    note(event);
    const QJsonObject stamped = record_.event(event);
    broadcast(line, stamped);
    if (event["event"] == QLatin1String("question")) {
        control({{"control", "waiting"}, {"id", question_}});
        if (nobodyWatching()) {
            asked_ = true;
            tell(QStringLiteral("waits for an answer"));
        }
    }
}

void Session::forwardOutput(const QByteArray &bytes)
{
    const QByteArray message = compact({{"stdout", QString::fromLatin1(bytes.toBase64())}});
    for (const Client &client : std::as_const(clients_))
        if (!client.attach)
            client.link->send(message);
}

void Session::answer(const QByteArray &line)
{
    const QJsonObject object = QJsonDocument::fromJson(line).object();
    const QString id = object["question"].toVariant().toString();
    // One question, one answer: whoever was first is heard, and a window that
    // answers late (or answers what is already answered) changes nothing.
    if (!asking_ || id != question_)
        return;
    asking_ = false;
    const QString text = object["answer"].toString();
    // Written before the agent hears it, so that the last line of a run that
    // was answered is no longer a question.
    const QJsonObject stamped = record_.event({{"event", "answer"}, {"id", id}, {"answer", text}});
    const QByteArray bytes = compact(stamped);
    const QByteArray done = compact({{"control", "answered"}, {"id", id}});
    for (const Client &client : std::as_const(clients_)) {
        if (client.attach)
            client.link->send(bytes);
        client.link->send(done);
    }
    adapter_->answered(id, text, line);
}

void Session::stopRequested(const QString &reason)
{
    if (finished_)
        return;
    // Children first: the record is then written about something that has
    // really ended.
    if (adapter_ && started_)
        adapter_->stop();
    finish(QStringLiteral("stopped"), 143, reason);
}

bool Session::nobodyWatching() const
{
    // The window that started the run connects a moment after we begin; until
    // it has (or has had five seconds to), the run is not unwatched.
    return clients_.isEmpty() && (everAttached_ || age_.elapsed() > 5000);
}

QString Session::noticeTitle() const
{
    if (!request_.title.isEmpty())
        return request_.title;
    return request_.app.isEmpty() ? QStringLiteral("Agent") : request_.app;
}

void Session::tell(const QString &text)
{
    notify(settings_, noticeTitle(), text);
}

void Session::finish(const QString &state, int exitCode, const QString &reason)
{
    if (finished_)
        return;
    finished_ = true;
    exitCode_ = exitCode;
    state_ = state;
    // Nobody may connect to a run that has ended; the record tells the rest.
    delete listener_;
    listener_ = nullptr;
    sockets::remove(settings_.socketsDir, socketName(id_));
    record_.end(state);

    const bool alone = nobodyWatching();
    // Whoever connected and said nothing has nothing to wait for.
    const QList<Link *> silent = greeting_;
    for (Link *link : silent)
        link->close();
    for (const Client &client : std::as_const(clients_)) {
        client.link->send(compact({{"control", "end"}, {"state", state}, {"code", exitCode}}));
        client.link->closeWhenSent();
    }
    if (alone) {
        auto brief = [](QString text) { return text.size() > 200 ? text.left(200) + QStringLiteral("...") : text; };
        if (state == QLatin1String("done"))
            tell(summary_.isEmpty() ? QStringLiteral("is done") : QStringLiteral("is done: ") + brief(summary_));
        else if (state == QLatin1String("failed"))
            tell(QStringLiteral("failed: ") + brief(failure_.isEmpty() ? QStringLiteral("it did not finish") : failure_));
        else
            tell(QStringLiteral("stopped: ") + brief(reason.isEmpty() ? QStringLiteral("it was stopped") : reason));
    }
    if (clients_.isEmpty()) {
        QCoreApplication::exit(exitCode);
        return;
    }
    // A window that does not read must not keep the process around.
    QTimer::singleShot(2000, this, [exitCode] { QCoreApplication::exit(exitCode); });
}
