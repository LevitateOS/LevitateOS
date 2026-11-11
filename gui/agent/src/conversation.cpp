#include "conversation.h"

#include "child.h"

#include <QJsonArray>
#include <QRegularExpression>

#include <cstring>

namespace {

// The contract with the agent. It is the only place the agent learns that it
// works for a window, and how to ask the person for anything.
const char *preamble = R"(You are working for the person at this computer, a LevitateOS or AcornOS desktop. A window on their screen (Recipe, System or Disk) handed you the request below and shows them what you do, so what you write is read by a person who is not a programmer.

Where you are
- You run as that person, without administrator rights. Never use sudo, su, doas or pkexec, and do not try to get around the sandbox you are in.
- The running system is locked: its system folders cannot be written. The system is changed only through two tools made for that, and you use them instead of editing system files: `recab` for the A and B systems (prepare a copy, change it, switch to it, go back) and `recipe` for software (install, update, remove). Ask either one for `--help` when you are unsure.
- Do what the request asks, and no more. If something is unclear, or could lose the person's data, ask before you do it.

How you ask the person for something
You cannot ask in the middle of your work. When you need an answer or a command run, end your turn: your last message holds the line below on a line of its own, and you stop. The window shows it, and you are started again with the answer. Use at most one of these in a message.

ASK: question | choice | choice
  A question for the person, with the choices to offer. Leave the choices out for a question they must answer in words.

RUN-AS-ADMINISTRATOR: command
BECAUSE: one sentence the person can understand, saying why it is needed
  A command that only an administrator can run. The person sees the command and your reason and may refuse. You are told the exit code and the output, or that they refused.

RUN: command
  A command you want run as the person, which your own sandbox does not let you run. They may refuse.

A command named in RUN or RUN-AS-ADMINISTRATOR is one program and its arguments, on one line, and is run without a shell: no pipes, no redirects, no &&, no `;`, no backticks, no $(...). Quote an argument that holds a space. If you need several steps, run one and ask for the next after you see the result.

When you are finished
Your final message is shown to the person as it is. Write two sentences at most, in plain words, saying what you did (or why you could not). Leave out file lists and technical detail, and put none of the lines above in it.)";

QString tail(const QString &text, int characters)
{
    return text.size() <= characters ? text : QStringLiteral("...") + text.right(characters);
}

} // namespace

Conversation::Conversation(Session *session)
    : Adapter(session)
{
}

QString Conversation::contract()
{
    return QString::fromUtf8(preamble);
}

Conversation::Wish Conversation::readWish(const QString &message)
{
    Wish wish;
    QStringList rest;
    for (const QString &raw : message.split('\n')) {
        const QString line = raw.trimmed();
        auto after = [&](const char *key) { return line.mid(int(strlen(key))).trimmed(); };
        if (line.startsWith(QLatin1String("ASK:"))) {
            QStringList parts = after("ASK:").split('|');
            wish = {};
            wish.kind = Wish::Ask;
            wish.text = parts.takeFirst().trimmed();
            for (const QString &part : parts)
                if (!part.trimmed().isEmpty())
                    wish.choices << part.trimmed();
        } else if (line.startsWith(QLatin1String("RUN-AS-ADMINISTRATOR:"))) {
            wish = {};
            wish.kind = Wish::Administrator;
            wish.text = after("RUN-AS-ADMINISTRATOR:");
        } else if (line.startsWith(QLatin1String("RUN:"))) {
            wish = {};
            wish.kind = Wish::Run;
            wish.text = after("RUN:");
        } else if (line.startsWith(QLatin1String("BECAUSE:"))) {
            wish.because = after("BECAUSE:");
        } else {
            rest << raw;
        }
    }
    // An empty request is no request: a bare `ASK:` would show a blank question.
    if (wish.text.isEmpty())
        wish.kind = Wish::Nothing;
    wish.rest = rest.join('\n').trimmed();
    return wish;
}

// The last lines of something, for the output pane or for the agent.
QStringList Conversation::cappedLines(const QString &text, int most)
{
    QStringList lines = text.trimmed().split('\n');
    if (lines.size() == 1 && lines.first().isEmpty())
        return {};
    for (QString &line : lines)
        if (line.size() > 400)
            line = line.left(400) + QStringLiteral("...");
    if (lines.size() > most) {
        const int more = int(lines.size()) - most;
        lines = lines.mid(0, most);
        lines << QStringLiteral("... %1 more lines").arg(more);
    }
    return lines;
}

QString Conversation::plainReason(const QString &reason, const QString &agentName, const QString &signInWords)
{
    // A person who is not signed in needs to read that, not a stack of text.
    static const QRegularExpression signIn(QStringLiteral("log(ged)?[ -]?in|sign(ed)?[ -]?in|unauthori[sz]ed|\\b401\\b|api key|authentication"),
                                           QRegularExpression::CaseInsensitiveOption);
    if (signIn.match(reason).hasMatch())
        return QStringLiteral("%1 is not signed in. %2 (%3)").arg(agentName, signInWords, reason);
    return reason;
}

void Conversation::begin(const Request &request)
{
    request_ = request;
    QString who = request.app.isEmpty() ? QStringLiteral("a window") : QStringLiteral("the %1 window").arg(request.app);
    if (!request.title.isEmpty())
        who += QStringLiteral(" (%1)").arg(request.title);
    turn(firstPrompt(who, request.prompt));
}

void Conversation::turn(const QString &prompt)
{
    if (over_)
        return;
    // A loop of questions and refusals must end somewhere.
    if (++turns_ > 30) {
        fail(QStringLiteral("The agent kept going back and forth without finishing, so it was stopped"));
        return;
    }
    launch(prompt, turns_ == 1);
}

void Conversation::turnEnded(const QString &message)
{
    if (over_)
        return;
    const Wish wish = readWish(message);
    if (wish.kind == Wish::Nothing) {
        session_->send({{"event", "agent"},
                        {"summary", wish.rest.isEmpty() ? QStringLiteral("The agent finished without a message.") : wish.rest}});
        session_->send({{"event", "done"}});
        over_ = true;
        session_->finish(QStringLiteral("done"), 0);
        return;
    }
    if (wish.kind == Wish::Ask) {
        ask(wish, {});
        return;
    }
    // From here the agent wants a program run. The person is not asked
    // about what could not be run safely anyway.
    const Words words = splitCommand(wish.text);
    if (!words.ok) {
        output(QStringLiteral("agent asked for a command that was not run: %1").arg(wish.text));
        turn(QStringLiteral("Not run: the command `%1` cannot be run because %2. Commands are run without a shell. "
                            "Name one plain command, a program and its arguments, and end your turn with the line again.")
                 .arg(wish.text, words.problem));
        return;
    }
    // Always asked, whatever run_commands says: that setting widens what the
    // agent may do inside its own sandbox. A RUN request is for something
    // outside it, run without a sandbox as the person.
    ask(wish, words.list);
}

// The plan is sent again only when the wording of the list changes; a tick on
// an item is a `step`.
void Conversation::plan(const QStringList &texts, const QStringList &states)
{
    if (texts.isEmpty())
        return;
    if (texts != planned_) {
        planned_ = texts;
        stepState_.clear();
        QJsonArray steps;
        for (int i = 0; i < texts.size(); ++i)
            steps << QJsonArray{QString::number(i + 1), texts[i]};
        session_->send({{"event", "plan"}, {"steps", steps}});
    }
    for (int i = 0; i < texts.size(); ++i) {
        const QString state = states.value(i);
        if (state.isEmpty() || stepState_.value(i) == state)
            continue;
        stepState_[i] = state;
        session_->send({{"event", "step"}, {"step", QString::number(i + 1)}, {"status", state}});
    }
}

void Conversation::ask(const Wish &wish, const QStringList &words)
{
    pending_ = {QString::number(++questions_), wish, words};
    waiting_ = true;
    QJsonObject question{{"event", "question"}, {"id", pending_.id}};
    if (wish.kind == Wish::Ask) {
        question["text"] = wish.text;
        question["choices"] = QJsonArray::fromStringList(wish.choices);
    } else {
        const bool admin = wish.kind == Wish::Administrator;
        QString text = admin ? QStringLiteral("The agent wants to run this command as administrator:\n\n%1")
                             : QStringLiteral("The agent wants to run this command:\n\n%1");
        text = text.arg(wish.text);
        if (!wish.because.isEmpty())
            text += QStringLiteral("\n\nWhy: ") + wish.because;
        question["text"] = text;
        question["choices"] = QJsonArray{"Approve", "Refuse"};
        question["kind"] = admin ? QStringLiteral("administrator") : QStringLiteral("command");
        question["command"] = wish.text;
    }
    session_->send(question);
}

void Conversation::answered(const QString &id, const QString &answer, const QByteArray &)
{
    if (!waiting_ || id != pending_.id)
        return;
    waiting_ = false;
    const Pending pending = pending_;
    if (pending.wish.kind == Wish::Ask) {
        turn(answer);
        return;
    }
    const bool admin = pending.wish.kind == Wish::Administrator;
    if (answer.trimmed().compare(QLatin1String("Approve"), Qt::CaseInsensitive) == 0) {
        execute(pending);
        return;
    }
    did(QStringLiteral("command"), pending.wish.text, QJsonValue(), admin, true, false);
    turn(QStringLiteral("The person refused to let `%1` run. Do not try another way to get the same result. "
                        "Carry on without it, or finish and say what is left undone.")
             .arg(pending.wish.text));
}

void Conversation::execute(const Pending &pending)
{
    const bool admin = pending.wish.kind == Wish::Administrator;
    QStringList words = pending.words;
    if (admin) {
        const Words helper = splitCommand(session_->settings().rootHelper);
        if (helper.ok)
            words = helper.list + words;
    }
    output(QStringLiteral("$ %1%2").arg(pending.wish.text, admin ? QStringLiteral("   (as administrator)") : QString()));

    Child *child = helper_ = new Child(this);
    child->setProcessChannelMode(QProcess::MergedChannels);
    child->setWorkingDirectory(session_->settings().home);
    child->setProgram(words.takeFirst());
    child->setArguments(words);
    const QString command = pending.wish.text;
    auto conclude = [this, child, command, admin](const QString &problem) {
        if (child != helper_ || over_)
            return;
        helper_ = nullptr;
        const QString text = QString::fromUtf8(child->readAllStandardOutput());
        const bool ran = problem.isEmpty();
        const QJsonValue code = ran && child->exitStatus() == QProcess::NormalExit ? QJsonValue(child->exitCode()) : QJsonValue();
        for (const QString &line : cappedLines(text, 30))
            output(line);
        if (!ran)
            output(problem);
        did(QStringLiteral("command"), command, code, admin, true, true);
        QString result = ran ? QStringLiteral("Exit code: %1").arg(child->exitCode()) : problem;
        turn(QStringLiteral("The person approved `%1`%2 and it was run. %3\nOutput:\n%4")
                 .arg(command, admin ? QStringLiteral(" as administrator") : QString(), result,
                      text.trimmed().isEmpty() ? QStringLiteral("(none)") : tail(text.trimmed(), 8000)));
        child->deleteLater();
    };
    connect(child, &QProcess::finished, this, [conclude] { conclude(QString()); });
    connect(child, &QProcess::errorOccurred, this, [conclude, command](QProcess::ProcessError error) {
        if (error == QProcess::FailedToStart)
            conclude(QStringLiteral("It could not be started."));
    });
    child->startInGroup();
    child->closeWriteChannel();
}

void Conversation::stop()
{
    over_ = true;
    endAgent();
    if (helper_)
        helper_->endGroup();
}

void Conversation::fail(const QString &reason)
{
    if (over_)
        return;
    over_ = true;
    endAgent();
    session_->send({{"event", "failed"}, {"reason", reason}});
    session_->finish(QStringLiteral("failed"), 1);
}

void Conversation::say(const QString &text)
{
    if (!text.isEmpty())
        session_->send({{"event", "say"}, {"text", text}});
}

void Conversation::output(const QString &text)
{
    session_->send({{"event", "output"}, {"line", text}});
}

void Conversation::did(const QString &kind, const QString &what, const QJsonValue &exitCode, bool administrator, bool asked,
                const QJsonValue &approved)
{
    session_->send({{"event", "did"},
                    {"kind", kind},
                    {"what", what},
                    {"exit_code", exitCode},
                    {"as_administrator", administrator},
                    {"asked", asked},
                    {"approved", approved}});
}
