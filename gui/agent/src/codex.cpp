#include "codex.h"

#include "child.h"

#include <QJsonArray>
#include <QJsonDocument>

Codex::Codex(Session *session)
    : Conversation(session)
{
}

QString Codex::firstPrompt(const QString &who, const QString &prompt) const
{
    // Codex has no place for standing instructions of its own here, so they
    // go in front of the request.
    return contract() + QStringLiteral("\n\n---\nThe request, from %1:\n\n").arg(who) + prompt;
}

void Codex::launch(const QString &prompt, bool first)
{
    const Settings &settings = session_->settings();
    // run_commands "allowed" means the home folder, whatever else was asked.
    // Otherwise a request that names folders to write in gets Codex's
    // workspace-write sandbox rooted at the first of them; with none, the
    // agent can only read.
    const bool folders = !settings.commandsAllowed && !request_.mayWrite.isEmpty();
    const QString mode = settings.commandsAllowed || folders ? QStringLiteral("workspace-write") : QStringLiteral("read-only");
    const QString root = folders ? request_.mayWrite.first() : settings.home;
    const QStringList more = folders ? request_.mayWrite.mid(1) : QStringList();

    QStringList arguments;
    if (first) {
        arguments = {"exec", "--json", "--skip-git-repo-check", "-s", mode, "-C", root};
        for (const QString &folder : more)
            arguments << "--add-dir" << folder;
        arguments << "-";
    } else {
        if (threadId_.isEmpty()) {
            fail(QStringLiteral("Codex did not say which session to continue"));
            return;
        }
        // `resume` takes no -s, -C or --add-dir; the sandbox goes in as
        // configuration and the folder is the one it is started in.
        arguments = {"exec", "resume", threadId_, "--json", "--skip-git-repo-check", "-c",
                     QStringLiteral("sandbox_mode=\"%1\"").arg(mode)};
        if (!more.isEmpty()) {
            QJsonArray roots;
            for (const QString &folder : more)
                roots << folder;
            arguments << "-c"
                      << QStringLiteral("sandbox_workspace_write.writable_roots=%1")
                             .arg(QString::fromUtf8(QJsonDocument(roots).toJson(QJsonDocument::Compact)));
        }
        arguments << "-";
    }

    buffer_.clear();
    errors_.clear();
    lastMessage_.clear();
    errorMessage_.clear();
    completed_ = false;
    // Item ids count from the start of each run of codex, so a new turn
    // reuses the old ones.
    started_.clear();
    finished_.clear();

    if (child_)
        child_->deleteLater();
    Child *child = child_ = new Child(this);
    child->setWorkingDirectory(root);
    child->setProgram(settings.codexProgram);
    child->setArguments(arguments);
    connect(child, &QProcess::readyReadStandardOutput, this, [this, child] {
        if (child == child_)
            readOutput();
    });
    connect(child, &QProcess::readyReadStandardError, this, [this, child] {
        errors_ += child->readAllStandardError();
        if (errors_.size() > 8000)
            errors_ = errors_.right(4000);
    });
    connect(child, &QProcess::finished, this, [this, child] {
        if (child == child_)
            onExit();
    });
    connect(child, &QProcess::errorOccurred, this, [this, child](QProcess::ProcessError error) {
        if (child == child_ && error == QProcess::FailedToStart)
            fail(QStringLiteral("Codex could not be started (%1). Is it installed?").arg(child->program()));
    });
    child->startInGroup();
    child->write(prompt.toUtf8());
    child->closeWriteChannel();
}

void Codex::endAgent()
{
    if (child_)
        child_->endGroup();
}

void Codex::readOutput()
{
    buffer_ += child_->readAllStandardOutput();
    qsizetype end;
    while ((end = buffer_.indexOf('\n')) >= 0 && !over_) {
        const QByteArray line = buffer_.left(end);
        buffer_.remove(0, end + 1);
        onLine(line);
    }
}

void Codex::onLine(const QByteArray &line)
{
    const QJsonObject event = QJsonDocument::fromJson(line).object();
    if (event.isEmpty()) {
        // Not an event: keep it, it may be the reason it stopped.
        errors_ += line + '\n';
        return;
    }
    const QString type = event["type"].toString();
    if (type == QLatin1String("thread.started")) {
        threadId_ = event["thread_id"].toString();
    } else if (type == QLatin1String("turn.completed")) {
        completed_ = true;
    } else if (type == QLatin1String("turn.failed")) {
        fail(event["error"]["message"].toString(QStringLiteral("The agent could not finish")));
    } else if (type == QLatin1String("error")) {
        // Codex also reports trouble it gets over (a connection it retried).
        // It is a failure only if the turn then ends without completing.
        errorMessage_ = event["message"].toString();
    } else if (type.startsWith(QLatin1String("item."))) {
        onItem(type, event["item"].toObject());
    }
}

void Codex::onItem(const QString &phase, const QJsonObject &item)
{
    const QString kind = item["type"].toString();
    const QString id = item["id"].toString();
    const bool completed = phase == QLatin1String("item.completed");

    if (kind == QLatin1String("command_execution")) {
        const QString command = item["command"].toString();
        if (!started_.contains(id)) {
            started_ << id;
            output(QStringLiteral("$ ") + command);
        }
        if (!completed || finished_.contains(id))
            return;
        finished_ << id;
        for (const QString &line : cappedLines(item["aggregated_output"].toString(), 30))
            output(line);
        const QString status = item["status"].toString();
        // Run by the agent in its own sandbox: nobody was asked, unless
        // Codex itself asked and was refused.
        const bool declined = status == QLatin1String("declined");
        did(QStringLiteral("command"), command, item["exit_code"].isNull() ? QJsonValue() : item["exit_code"], false,
            declined, declined ? QJsonValue(false) : QJsonValue());
    } else if (kind == QLatin1String("file_change")) {
        if (!completed || finished_.contains(id) || item["status"].toString() == QLatin1String("failed"))
            return;
        finished_ << id;
        for (const QJsonValue &change : item["changes"].toArray()) {
            const QString path = change["path"].toString();
            output(QStringLiteral("%1 %2").arg(change["kind"].toString(QStringLiteral("changed")), path));
            did(QStringLiteral("write"), path, QJsonValue(), false, false, QJsonValue());
        }
    } else if (kind == QLatin1String("todo_list")) {
        onTodo(item);
    } else if (kind == QLatin1String("web_search")) {
        if (completed && !finished_.contains(id)) {
            finished_ << id;
            did(QStringLiteral("search"), item["query"].toString(), QJsonValue(), false, false, QJsonValue());
        }
    } else if (kind == QLatin1String("agent_message")) {
        if (completed && !finished_.contains(id)) {
            finished_ << id;
            lastMessage_ = item["text"].toString();
            say(readWish(lastMessage_).rest);
        }
    } else if (kind == QLatin1String("error")) {
        if (completed && !finished_.contains(id)) {
            finished_ << id;
            output(QStringLiteral("agent: ") + item["message"].toString());
        }
    }
    // `reasoning` and `mcp_tool_call` say nothing the person needs.
}

void Codex::onTodo(const QJsonObject &item)
{
    const QJsonArray items = item["items"].toArray();
    QStringList texts, states;
    bool runningGiven = false;
    for (const QJsonValue &entry : items) {
        texts << entry["text"].toString();
        if (entry["completed"].toBool()) {
            states << QStringLiteral("done");
        } else if (!runningGiven) {
            states << QStringLiteral("running");
            runningGiven = true;
        } else {
            states << QString();
        }
    }
    plan(texts, states);
}

void Codex::onExit()
{
    if (over_)
        return;
    readOutput();
    if (completed_) {
        turnEnded(lastMessage_);
        return;
    }
    QString reason = errorMessage_;
    if (reason.isEmpty()) {
        QStringList lines = QString::fromUtf8(errors_).trimmed().split('\n', Qt::SkipEmptyParts);
        reason = lines.mid(qMax<qsizetype>(0, lines.size() - 8)).join('\n').trimmed();
    }
    if (reason.isEmpty())
        reason = QStringLiteral("Codex ended without finishing and said nothing");
    fail(plainReason(reason, QStringLiteral("Codex"), QStringLiteral("Sign in with `codex login` and try again.")));
}
