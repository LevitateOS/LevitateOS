#include "run.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QRegularExpression>

#include <csignal>
#include <sys/types.h>

Run::Run(const QString &program, const QStringList &arguments, QObject *parent)
    : QObject(parent)
{
    process_.setProgram(program);
    process_.setArguments(arguments);
    connect(&process_, &QProcess::readyReadStandardError, this, &Run::drain);
    connect(&process_, &QProcess::readyReadStandardOutput, this,
            [this] { answer_ += process_.readAllStandardOutput(); });
    connect(&process_, &QProcess::finished, this, [this](int code, QProcess::ExitStatus status) {
        drain();
        answer_ += process_.readAllStandardOutput();
        exitCode_ = status == QProcess::NormalExit ? code : -1;
        finishedAlready_ = true;
        if (released_) {
            deleteLater(); // nobody listens any more
            return;
        }
        // 75: the helper left a run that goes on without it. Not a failure.
        if (agent_ && exitCode_ == 75) {
            emit carriesOn();
            return;
        }
        emit finished(status == QProcess::NormalExit && code == 0);
    });
    connect(&process_, &QProcess::errorOccurred, this, [this](QProcess::ProcessError error) {
        if (error != QProcess::FailedToStart)
            return;
        failure_ = QJsonObject{{"reason", tr("%1 could not be started").arg(process_.program())}};
        emit finished(false);
    });
}

Run::~Run()
{
    // A command whose object goes is not heard any more: its end must not call
    // into pages that are going too. A client that is still going is ended
    // here (without Qt's warning), which the helper takes: SIGKILL on a client
    // leaves the run going.
    QObject::disconnect(&process_, nullptr, nullptr, nullptr);
    QObject::disconnect(this, nullptr, nullptr, nullptr);
    if (process_.state() != QProcess::NotRunning) {
        process_.kill();
        process_.waitForFinished(500);
    }
}

void Run::setPrompt(const QString &prompt, const QString &app, const QString &title, const QStringList &mayWrite)
{
    QJsonObject line{{"prompt", prompt}};
    if (!app.isEmpty())
        line["app"] = app;
    if (!title.isEmpty())
        line["title"] = title;
    if (!mayWrite.isEmpty())
        line["may_write"] = QJsonArray::fromStringList(mayWrite);
    input_ = QJsonDocument(line).toJson(QJsonDocument::Compact) + '\n';
    open_ = true;
    agent_ = true;
}

Run *Run::attach(const QString &agent, const QString &id, QObject *parent)
{
    auto *run = new Run(agent, {"--attach", id}, parent);
    run->open_ = true;
    run->attach_ = true;
    run->id_ = id;
    return run;
}

void Run::release()
{
    if (released_)
        return;
    released_ = true;
    disconnect(this, nullptr, nullptr, nullptr);
    if (process_.state() == QProcess::NotRunning) {
        deleteLater();
        return;
    }
    process_.closeWriteChannel();
    // The client may be relaying for a run that works on: hang up as well, so
    // that it does not stay for as long as the run does. Neither ends the run.
    if (process_.processId() > 0)
        ::kill(pid_t(process_.processId()), SIGHUP);
}

void Run::reply(const QString &question, const QString &answer)
{
    if (process_.state() == QProcess::NotRunning)
        return;
    process_.write(QJsonDocument(QJsonObject{{"question", question}, {"answer", answer}}).toJson(QJsonDocument::Compact)
                   + '\n');
}

QString Run::problem() const
{
    const QString reason = failure_["reason"].toString();
    if (!reason.isEmpty())
        return reason;
    return said_.join(QLatin1Char('\n')).trimmed();
}

void Run::start()
{
    process_.start();
    if (!input_.isEmpty())
        process_.write(input_);
    if (!open_)
        process_.closeWriteChannel();
}

void Run::stop()
{
    if (!agent_ || id_.isEmpty()) {
        process_.terminate();
        return;
    }
    // The helper ends the run, its agent and what the agent started; this
    // client then ends with the run (143).
    auto *stopper = new QProcess(this);
    stopper->setProgram(process_.program());
    stopper->setArguments({"--stop", id_});
    connect(stopper, &QProcess::finished, this, [this, stopper](int code, QProcess::ExitStatus status) {
        // The helper does not know the run yet (stopped in its first moment),
        // or could not be asked: the client's own end is a stop as well.
        if ((status != QProcess::NormalExit || code != 0) && process_.state() != QProcess::NotRunning)
            process_.terminate();
        stopper->deleteLater();
    });
    connect(stopper, &QProcess::errorOccurred, this, [this, stopper](QProcess::ProcessError error) {
        if (error == QProcess::FailedToStart && process_.state() != QProcess::NotRunning)
            process_.terminate();
        stopper->deleteLater();
    });
    stopper->start();
}

void Run::drain()
{
    pending_ += process_.readAllStandardError();
    int end;
    while ((end = pending_.indexOf('\n')) >= 0) {
        const QByteArray line = pending_.left(end);
        pending_.remove(0, end + 1);
        const QJsonObject object = line.startsWith('{') ? QJsonDocument::fromJson(line).object() : QJsonObject();
        if (object.isEmpty()) {
            // `recipe` colours its lines even into a pipe.
            static const QRegularExpression colour(QStringLiteral("\x1b\\[[0-9;]*[A-Za-z]"));
            const QString words = QString::fromUtf8(line).remove(colour);
            if (words.startsWith(QLatin1String("Error:")) && !failure_.contains("reason"))
                failure_["reason"] = words.mid(6).trimmed();
            if (!words.trimmed().isEmpty()) {
                said_ << words;
                if (said_.size() > 40)
                    said_.removeFirst();
                emit output(words);
            }
            continue;
        }
        // The last word on a failure. What an earlier event said of the recipe
        // and the step stays where this one names none.
        if (object["event"] == QLatin1String("failed")) {
            for (auto field = object.begin(); field != object.end(); ++field)
                if (!field.value().isNull() || !failure_.contains(field.key()))
                    failure_[field.key()] = field.value();
        }
        // recab's own error: one JSON object with a class, a code and a message.
        if (!object.contains("event") && object.contains("message")) {
            failure_ = object;
            failure_["reason"] = object["message"];
            continue;
        }
        if (object["event"] == QLatin1String("recipe-hook") && object["status"] == QLatin1String("failed")) {
            failure_["recipe"] = object["recipe"];
            failure_["hook"] = object["hook"];
        }
        // The helper's first line: which run this is.
        if (object["event"] == QLatin1String("run") && agent_ && id_.isEmpty()) {
            id_ = object["id"].toString();
            emit idKnown(id_);
            continue;
        }
        if (object["event"] == QLatin1String("output"))
            emit output(object["line"].toString());
        else
            emit event(object);
    }
}
