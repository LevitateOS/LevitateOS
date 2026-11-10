#include "session.h"

#include "adapter.h"

#include <QCoreApplication>
#include <QJsonDocument>
#include <QSocketNotifier>

#include <cerrno>
#include <csignal>
#include <fcntl.h>
#include <unistd.h>

// A signal handler may do very little; it writes one byte and the event loop
// does the rest.
static int signalPipe[2] = {-1, -1};

static void onSignalNumber(int)
{
    const char byte = 1;
    [[maybe_unused]] const ssize_t written = ::write(signalPipe[1], &byte, 1);
}

Session::Session(const Settings &settings, QObject *parent)
    : QObject(parent)
    , settings_(settings)
    , record_(settings)
{
}

void Session::run()
{
    // The app may go away while we write to it; that must not kill us
    // before the record has its last line.
    ::signal(SIGPIPE, SIG_IGN);
    if (::pipe2(signalPipe, O_CLOEXEC | O_NONBLOCK) == 0) {
        struct sigaction action {};
        action.sa_handler = onSignalNumber;
        sigemptyset(&action.sa_mask);
        for (int number : {SIGTERM, SIGINT, SIGHUP})
            sigaction(number, &action, nullptr);
        signals_ = new QSocketNotifier(signalPipe[0], QSocketNotifier::Read, this);
        connect(signals_, &QSocketNotifier::activated, this, &Session::onSignal);
    }
    input_ = new QSocketNotifier(0, QSocketNotifier::Read, this);
    connect(input_, &QSocketNotifier::activated, this, &Session::onInput);
}

void Session::onSignal()
{
    char byte;
    while (::read(signalPipe[0], &byte, 1) > 0) {
    }
    if (finished_)
        return;
    // Children first: the record is then written about something that has
    // really ended.
    if (adapter_)
        adapter_->stop();
    finish(QStringLiteral("stopped"), 143);
}

void Session::onInput()
{
    char buffer[4096];
    const ssize_t count = ::read(0, buffer, sizeof buffer);
    if (count > 0) {
        pending_.append(buffer, count);
        qsizetype end;
        while ((end = pending_.indexOf('\n')) >= 0 && !finished_) {
            const QByteArray line = pending_.left(end);
            pending_.remove(0, end + 1);
            handle(line);
        }
    } else if (count == 0 || (errno != EINTR && errno != EAGAIN)) {
        input_->setEnabled(false);
        if (!pending_.trimmed().isEmpty()) {
            const QByteArray line = pending_;
            pending_.clear();
            handle(line);
        }
        if (!finished_ && adapter_ && started_)
            adapter_->inputClosed();
        else if (!started_) {
            send({{"event", "failed"}, {"reason", "No request came on the input"}});
            finish(QStringLiteral("failed"), 1);
        }
    }
}

void Session::handle(const QByteArray &line)
{
    const QJsonObject object = QJsonDocument::fromJson(line).object();
    if (line.trimmed().isEmpty())
        return;
    if (!started_) {
        if (object["prompt"].toString().isEmpty()) {
            send({{"event", "failed"}, {"reason", "The first line must be a request with a prompt"}});
            finish(QStringLiteral("failed"), 1);
            return;
        }
        started_ = true;
        const Request request{object["prompt"].toString(), object["app"].toString(), object["title"].toString(), line};
        record_.open(request.app, request.title, request.prompt);
        adapter_->begin(request);
        return;
    }
    if (!object.contains("question") || !object.contains("answer"))
        return;
    const QString id = object["question"].toVariant().toString();
    const QString answer = object["answer"].toString();
    // Written before the agent hears it, so that the last line of a run that
    // was answered is no longer a question.
    record_.event({{"event", "answer"}, {"id", id}, {"answer", answer}});
    adapter_->answered(id, answer, line);
}

void Session::toApp(const QByteArray &bytes)
{
    qsizetype done = 0;
    while (done < bytes.size()) {
        const ssize_t count = ::write(2, bytes.constData() + done, size_t(bytes.size() - done));
        if (count < 0 && errno == EINTR)
            continue;
        if (count <= 0)
            return;
        done += count;
    }
}

void Session::send(const QJsonObject &event)
{
    if (finished_)
        return;
    toApp(QJsonDocument(event).toJson(QJsonDocument::Compact) + '\n');
    record_.event(event);
}

void Session::forward(const QByteArray &line)
{
    if (finished_)
        return;
    toApp(line + '\n');
    const QJsonObject object = line.startsWith('{') ? QJsonDocument::fromJson(line).object() : QJsonObject();
    record_.event(object.contains("event") ? object : QJsonObject{{"event", "output"}, {"line", QString::fromUtf8(line)}});
}

void Session::finish(const QString &state, int exitCode)
{
    if (finished_)
        return;
    finished_ = true;
    record_.end(state);
    QCoreApplication::exit(exitCode);
}
