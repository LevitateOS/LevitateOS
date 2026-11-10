#include "run.h"

#include <QJsonDocument>
#include <QRegularExpression>

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
        emit finished(status == QProcess::NormalExit && code == 0);
    });
    connect(&process_, &QProcess::errorOccurred, this, [this](QProcess::ProcessError error) {
        if (error != QProcess::FailedToStart)
            return;
        failure_ = QJsonObject{{"reason", tr("%1 could not be started").arg(process_.program())}};
        emit finished(false);
    });
}

void Run::setPrompt(const QString &prompt)
{
    input_ = QJsonDocument(QJsonObject{{"prompt", prompt}}).toJson(QJsonDocument::Compact) + '\n';
    open_ = true;
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
    process_.terminate();
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
        if (object["event"] == QLatin1String("failed"))
            failure_ = object;
        if (object["event"] == QLatin1String("recipe-hook") && object["status"] == QLatin1String("failed")) {
            failure_["recipe"] = object["recipe"];
            failure_["hook"] = object["hook"];
        }
        if (object["event"] == QLatin1String("output"))
            emit output(object["line"].toString());
        else
            emit event(object);
    }
}
