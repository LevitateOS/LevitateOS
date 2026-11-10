#include "passthrough.h"

#include "child.h"

Passthrough::Passthrough(Session *session, const QString &program)
    : Adapter(session)
    , program_(program)
{
}

void Passthrough::begin(const Request &request)
{
    child_ = new Child(this);
    child_->setProgram(program_);
    // What it writes on standard output is its answer, and belongs to the app
    // as it would without us.
    child_->setProcessChannelMode(QProcess::ForwardedOutputChannel);
    connect(child_, &QProcess::readyReadStandardError, this, [this] {
        buffer_ += child_->readAllStandardError();
        qsizetype end;
        while ((end = buffer_.indexOf('\n')) >= 0) {
            const QByteArray line = buffer_.left(end);
            buffer_.remove(0, end + 1);
            session_->forward(line);
        }
    });
    connect(child_, &QProcess::finished, this, [this](int code, QProcess::ExitStatus status) {
        if (!buffer_.isEmpty())
            session_->forward(buffer_);
        buffer_.clear();
        const bool ok = status == QProcess::NormalExit && code == 0;
        // Its last event is its own account; the exit code is what the app reads.
        session_->finish(ok ? QStringLiteral("done") : QStringLiteral("failed"), ok ? 0 : (code ? code : 1));
    });
    connect(child_, &QProcess::errorOccurred, this, [this](QProcess::ProcessError error) {
        if (error != QProcess::FailedToStart)
            return;
        session_->send({{"event", "failed"}, {"reason", QStringLiteral("%1 could not be started").arg(program_)}});
        session_->finish(QStringLiteral("failed"), 1);
    });
    child_->startInGroup();
    child_->write(request.line + '\n');
}

void Passthrough::answered(const QString &, const QString &, const QByteArray &line)
{
    if (child_)
        child_->write(line + '\n');
}

void Passthrough::inputClosed()
{
    if (child_)
        child_->closeWriteChannel();
}

void Passthrough::stop()
{
    if (child_)
        child_->endGroup();
}
