#include "claude.h"

#include "child.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QTimer>

namespace {

// What the agent may always use: it reads, searches and looks things up, and
// changes nothing.
const char *readOnlyTools = "Read,Glob,Grep,WebSearch,WebFetch";
// What changes files. Where the person has not allowed that, these are taken
// away, so the agent sees it cannot write and asks instead.
const char *writingTools = "Edit,Write,NotebookEdit";

// What is true of this agent's tools, beside the contract every agent gets.
const char *addendum = R"(

Your tools
- You may read files, search, and look things up on the web. Writing a file or running a command is refused unless the person's settings allow it for the folder in question. When a tool is refused, you are told so: do not try another way to get the same result. Use the RUN, RUN-AS-ADMINISTRATOR or ASK lines above instead, or say what is left undone.
- Do not ask the person anything through a tool made for asking. The ASK line is the only way.)";

// The text of a tool_result's content: a string, or blocks with text.
QString textOf(const QJsonValue &content)
{
    if (content.isString())
        return content.toString();
    QStringList parts;
    for (const QJsonValue &block : content.toArray())
        if (block["type"].toString() == QLatin1String("text"))
            parts << block["text"].toString();
    return parts.join('\n');
}

} // namespace

Claude::Claude(Session *session)
    : Conversation(session)
{
}

QString Claude::firstPrompt(const QString &who, const QString &prompt) const
{
    // The standing instructions go in as the system prompt (below); only the
    // request is the person's turn.
    return QStringLiteral("The request, from %1:\n\n").arg(who) + prompt;
}

void Claude::launch(const QString &prompt, bool first)
{
    const Settings &settings = session_->settings();
    if (!first && sessionId_.isEmpty()) {
        fail(QStringLiteral("Claude Code did not say which session to continue"));
        return;
    }
    // The same rule as for Codex: "allowed" means the home folder; otherwise
    // folders named in the request, the first as the working folder; with
    // none, nothing may be written.
    const bool folders = !settings.commandsAllowed && !request_.mayWrite.isEmpty();
    const bool writes = settings.commandsAllowed || folders;
    const QString root = folders ? request_.mayWrite.first() : settings.home;

    // `-p` is print mode; stream-json needs `--verbose`; the prompt is read
    // from standard input. A mode that writes inside the working folders
    // (acceptEdits) or one that denies whatever would ask (dontAsk): there is
    // nobody in print mode to answer a prompt, so what is not allowed is
    // refused and Claude Code tells the agent so.
    QStringList arguments = {"-p", "--output-format", "stream-json", "--verbose"};
    if (!first)
        arguments << "--resume" << sessionId_;
    arguments << "--permission-mode" << (writes ? "acceptEdits" : "dontAsk");
    arguments << "--allowedTools" << readOnlyTools;
    if (!writes)
        arguments << "--disallowedTools" << writingTools;
    if (folders)
        for (const QString &folder : request_.mayWrite.mid(1))
            arguments << "--add-dir" << folder;
    // Given again on a resume: nothing says it is kept with the session.
    arguments << "--append-system-prompt" << contract() + QString::fromUtf8(addendum);

    buffer_.clear();
    errors_.clear();
    lastText_.clear();
    resultText_.clear();
    assistantError_.clear();
    completed_ = false;
    tools_.clear();
    denied_.clear();
    finished_.clear();

    if (child_)
        child_->deleteLater();
    Child *child = child_ = new Child(this);
    child->setWorkingDirectory(root);
    child->setProgram(settings.claudeProgram);
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
            fail(QStringLiteral("Claude Code could not be started (%1). Is it installed?").arg(child->program()));
    });
    child->startInGroup();
    child->write(prompt.toUtf8());
    child->closeWriteChannel();
}

void Claude::endAgent()
{
    if (grace_)
        grace_->stop();
    if (child_)
        child_->endGroup();
}

void Claude::readOutput()
{
    buffer_ += child_->readAllStandardOutput();
    qsizetype end;
    while ((end = buffer_.indexOf('\n')) >= 0 && !over_) {
        const QByteArray line = buffer_.left(end);
        buffer_.remove(0, end + 1);
        onLine(line);
    }
}

void Claude::onLine(const QByteArray &line)
{
    const QJsonObject message = QJsonDocument::fromJson(line).object();
    if (message.isEmpty()) {
        // Not a message: keep it, it may be the reason it stopped.
        errors_ += line + '\n';
        return;
    }
    if (sessionId_.isEmpty() && !message["session_id"].toString().isEmpty())
        sessionId_ = message["session_id"].toString();
    const QString type = message["type"].toString();
    if (type == QLatin1String("system")) {
        const QString subtype = message["subtype"].toString();
        if (subtype == QLatin1String("init")) {
            sessionId_ = message["session_id"].toString(sessionId_);
        } else if (subtype == QLatin1String("permission_denied")) {
            // The call was refused; what the agent asked for did not happen.
            const QString id = message["tool_use_id"].toString();
            denied_ << id;
            const Tool tool = tools_.value(id, {message["tool_name"].toString(), {}});
            QString what = tool.name;
            if (tool.input.contains("command"))
                what = tool.input["command"].toString();
            else if (tool.input.contains("file_path"))
                what += QStringLiteral(" ") + tool.input["file_path"].toString();
            output(QStringLiteral("not allowed: %1").arg(what));
        }
        // api_retry and the other system messages say nothing the person needs.
    } else if (type == QLatin1String("assistant")) {
        onAssistant(message);
    } else if (type == QLatin1String("user")) {
        onUser(message);
    } else if (type == QLatin1String("result")) {
        onResult(message);
    }
}

void Claude::onAssistant(const QJsonObject &message)
{
    // What a subagent says is its own; the person is told what the agent did.
    if (!message["parent_tool_use_id"].isNull() && !message["parent_tool_use_id"].toString().isEmpty())
        return;
    if (!message["error"].toString().isEmpty())
        assistantError_ = message["error"].toString();
    for (const QJsonValue &block : message["message"]["content"].toArray()) {
        const QString kind = block["type"].toString();
        if (kind == QLatin1String("text")) {
            lastText_ = block["text"].toString();
            say(readWish(lastText_).rest);
        } else if (kind == QLatin1String("tool_use")) {
            onToolUse(block.toObject());
        }
        // `thinking` is not for the person.
    }
}

void Claude::onUser(const QJsonObject &message)
{
    if (!message["parent_tool_use_id"].isNull() && !message["parent_tool_use_id"].toString().isEmpty())
        return;
    const QJsonValue content = message["message"]["content"];
    if (!content.isArray())
        return;
    for (const QJsonValue &block : content.toArray())
        if (block["type"].toString() == QLatin1String("tool_result"))
            onToolResult(block.toObject());
}

void Claude::onToolUse(const QJsonObject &block)
{
    const QString id = block["id"].toString();
    const QString name = block["name"].toString();
    const QJsonObject input = block["input"].toObject();
    tools_[id] = {name, input};
    if (name == QLatin1String("Bash")) {
        output(QStringLiteral("$ ") + input["command"].toString());
    } else if (name == QLatin1String("TodoWrite")) {
        QStringList texts, states;
        for (const QJsonValue &todo : input["todos"].toArray()) {
            texts << todo["content"].toString();
            const QString status = todo["status"].toString();
            states << (status == QLatin1String("completed") ? QStringLiteral("done")
                                                              : status == QLatin1String("in_progress") ? QStringLiteral("running") : QString());
        }
        plan(texts, states);
    }
}

void Claude::onToolResult(const QJsonObject &block)
{
    const QString id = block["tool_use_id"].toString();
    if (finished_.contains(id) || !tools_.contains(id))
        return;
    finished_ << id;
    // A refused call has been said already, and nothing was done.
    if (denied_.contains(id))
        return;
    const Tool tool = tools_.value(id);
    const bool failed = block["is_error"].toBool();
    const QString name = tool.name;
    if (name == QLatin1String("Bash")) {
        for (const QString &line : cappedLines(textOf(block["content"]), 30))
            output(line);
        // The exit code is not in the message; a command that worked is 0.
        did(QStringLiteral("command"), tool.input["command"].toString(), failed ? QJsonValue() : QJsonValue(0), false, false,
            QJsonValue());
    } else if (name == QLatin1String("Edit") || name == QLatin1String("Write") || name == QLatin1String("NotebookEdit")) {
        if (failed)
            return;
        const QString path = tool.input.contains("notebook_path") ? tool.input["notebook_path"].toString() : tool.input["file_path"].toString();
        output(QStringLiteral("%1 %2").arg(name == QLatin1String("Write") ? QStringLiteral("write") : QStringLiteral("update"), path));
        did(QStringLiteral("write"), path, QJsonValue(), false, false, QJsonValue());
    } else if (name == QLatin1String("WebSearch") || name == QLatin1String("WebFetch")) {
        if (!failed)
            did(QStringLiteral("search"), name == QLatin1String("WebSearch") ? tool.input["query"].toString() : tool.input["url"].toString(),
                QJsonValue(), false, false, QJsonValue());
    }
    // Reading and searching files say nothing the person needs.
}

void Claude::onResult(const QJsonObject &message)
{
    const bool success = message["subtype"].toString() == QLatin1String("success");
    if (!success || message["is_error"].toBool()) {
        QString reason = message["result"].toString();
        if (reason.isEmpty()) {
            QStringList errors;
            for (const QJsonValue &error : message["errors"].toArray())
                errors << error.toString();
            reason = errors.join('\n');
        }
        if (reason.isEmpty())
            reason = assistantError_.isEmpty() ? message["subtype"].toString() : assistantError_;
        if (reason.isEmpty())
            reason = QStringLiteral("Claude Code could not finish");
        // The assistant message carries the kind of error when it is a sign-in one.
        if (assistantError_ == QLatin1String("authentication_failed") && !reason.contains(QLatin1String("log"), Qt::CaseInsensitive))
            reason += QStringLiteral(" (authentication failed)");
        fail(plainReason(reason, QStringLiteral("Claude Code"),
                         QStringLiteral("Run `claude` once in a terminal to sign in, then try again.")));
        return;
    }
    completed_ = true;
    resultText_ = message["result"].toString();
    // The turn is over, but a run of `claude -p` may stay open for background
    // work it was told about. Waiting for that would hold the person's
    // window; it is given a few seconds to end by itself.
    if (!grace_) {
        grace_ = new QTimer(this);
        grace_->setSingleShot(true);
        connect(grace_, &QTimer::timeout, this, [this] {
            if (child_)
                child_->endGroup();
        });
    }
    grace_->start(10000);
}

void Claude::onExit()
{
    if (over_)
        return;
    if (grace_)
        grace_->stop();
    readOutput();
    if (completed_) {
        turnEnded(resultText_.isEmpty() ? lastText_ : resultText_);
        return;
    }
    QStringList lines = QString::fromUtf8(errors_).trimmed().split('\n', Qt::SkipEmptyParts);
    QString reason = lines.mid(qMax<qsizetype>(0, lines.size() - 8)).join('\n').trimmed();
    if (reason.isEmpty())
        reason = assistantError_.isEmpty() ? QStringLiteral("Claude Code ended without finishing and said nothing") : assistantError_;
    fail(plainReason(reason, QStringLiteral("Claude Code"), QStringLiteral("Run `claude` once in a terminal to sign in, then try again.")));
}
