#include "livestory.h"

#include "activity.h"
#include "frame.h"
#include "look.h"
#include "work.h"

#include <QApplication>
#include <QGuiApplication>
#include <QDir>
#include <QFile>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QMap>
#include <QPlainTextEdit>
#include <QProcess>
#include <QSet>

#include <csignal>
#include <sys/types.h>

namespace livestory {

namespace {

QMap<QByteArray, QByteArray> kept; // what the environment held before `prepare`
QSet<QByteArray> unset;            // and what it did not hold at all
QString logFile, firstLog, liveFolder;
int started = 0;
bool quitKept = true;

void setEnv(const char *name, const QString &value)
{
    if (!kept.contains(name) && !unset.contains(name)) {
        if (qEnvironmentVariableIsSet(name))
            kept.insert(name, qgetenv(name));
        else
            unset.insert(name);
    }
    qputenv(name, value.toUtf8());
}

QString helper()
{
    return qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
}

// One call of the helper, waited for (it is quick; a busy machine is not).
QByteArray call(const QStringList &arguments, int *code = nullptr)
{
    QProcess process;
    process.start(helper(), arguments);
    process.waitForFinished(180000);
    if (code)
        *code = process.exitStatus() == QProcess::NormalExit ? process.exitCode() : -1;
    return process.readAllStandardOutput();
}

QJsonArray runs()
{
    return QJsonDocument::fromJson(call({"--list"})).object()["runs"].toArray();
}

QString idOf(const QString &title)
{
    for (const QJsonValue &run : runs())
        if (run["title"].toString() == title)
            return run["id"].toString();
    return QString();
}

QString stateIn(Frame &frame, const QString &title)
{
    ActivityPage *page = frame.activity();
    const int at = page->listedTitles().indexOf(title);
    return at < 0 ? QString() : page->listedStates().value(at);
}

// The events of a run's record, as the helper keeps them.
QJsonArray record(const QString &id)
{
    return QJsonDocument::fromJson(call({"--show", id})).array();
}

int count(const QJsonArray &lines, const QString &event)
{
    int found = 0;
    for (const QJsonValue &line : lines)
        found += line["event"].toString() == event;
    return found;
}

QString endState(const QJsonArray &lines)
{
    for (const QJsonValue &line : lines)
        if (line["record"].toString() == QLatin1String("end"))
            return line["state"].toString();
    return QString();
}

// Every word on a work page: its labels and its output.
QString workWords(QWidget *window)
{
    QStringList all;
    if (auto *page = window->findChild<WorkPage *>()) {
        for (QLabel *label : page->findChildren<QLabel *>())
            all << label->text();
        for (QPlainTextEdit *output : page->findChildren<QPlainTextEdit *>())
            all << output->toPlainText();
    }
    return all.join(QLatin1Char('\n'));
}

bool alive(qint64 pid)
{
    return pid > 0 && ::kill(pid_t(pid), 0) == 0;
}

// The window that handed the request over closes: the run must go on.
void closeWindow(Started &started)
{
    started.window->close();
    started.window->deleteLater();
    started.window = nullptr;
    settle(300);
}

} // namespace

QString codexLog()
{
    return firstLog;
}

void prepare(const QString &folder, const QString &fixtures)
{
    const QString live = QDir(folder).filePath("live");
    QDir(live).removeRecursively();
    for (const char *sub : {"state", "run", "config"})
        QDir().mkpath(live + "/" + sub);
    setEnv("XDG_STATE_HOME", live + "/state");
    setEnv("XDG_RUNTIME_DIR", live + "/run");
    setEnv("XDG_CONFIG_HOME", live + "/config");
    setEnv("LEVITATE_AGENT_CODEX", QDir(fixtures).filePath("codex"));
    setEnv("LEVITATE_AGENT_ROOT_HELPER", QString());
    setEnv("LEVITATE_AGENT_NOTIFIER", QString());
    liveFolder = live;
    started = 0;
    firstLog.clear();
    // A window of ours closing is not the end of the story.
    quitKept = QGuiApplication::quitOnLastWindowClosed();
    QGuiApplication::setQuitOnLastWindowClosed(false);
}

void useStory(const QString &story)
{
    setEnv("LEVITATE_STANDIN_STORY", story);
    // The stand-in counts turns for the whole of a log: each run has a log of its own,
    // or the second run would take up where the first stopped.
    logFile = liveFolder + QStringLiteral("/codex-%1.log").arg(++started);
    if (firstLog.isEmpty())
        firstLog = logFile;
    setEnv("LEVITATE_STANDIN_LOG", logFile);
}

void finish()
{
    // Whatever is still going is stopped, with the helper's own `--stop`.
    if (!helper().isEmpty()) {
        for (const QJsonValue &run : runs()) {
            const QString state = run["state"].toString();
            if (state == QLatin1String("running") || state == QLatin1String("waiting"))
                call({"--stop", run["id"].toString()});
        }
    }
    for (auto it = kept.begin(); it != kept.end(); ++it)
        qputenv(it.key().constData(), it.value());
    for (const QByteArray &name : std::as_const(unset))
        qunsetenv(name.constData());
    kept.clear();
    unset.clear();
    QGuiApplication::setQuitOnLastWindowClosed(quitKept);
}

void run(Frame &mine, QWidget &mineWindow, const Starter &start, const Save &save, const Expect &expect,
         const QString &prefix)
{
    ActivityPage *page = mine.activity();
    const QString key = Frame::activityKey();
    mine.choose(key);
    waitFor([&] { return page->loaded() && mine.activityShown(); });
    expect(QStringLiteral("live: there is no run yet, and the rail does not count one"),
           page->listedTitles().isEmpty() && mine.badge(key) == 0);

    // ---- One run asks; its window leaves and then closes; another window finds it.
    const QString asking = QStringLiteral("Pick a colour for the check");
    Started first = start(QStringLiteral("asks"), asking);
    expect(QStringLiteral("live: the window learns the id of its run from the helper"),
           waitFor([&] { return first.frame && !first.frame->ownRun().isEmpty(); }));
    const QString id = first.frame->ownRun();
    // Nobody pressed anything here: the page reads the list again by itself.
    expect(QStringLiteral("live: Activity of another window lists a run that window did not start, by itself (%1)")
               .arg(page->listedStates().join(" | ")),
           waitFor([&] { return stateIn(mine, asking) == QStringLiteral("Waits for an answer"); }));
    expect(QStringLiteral("live: and its badge counts it, amber, with nobody pressing anything"),
           waitFor([&] { return mine.badge(key) == 1 && mine.badgeWarns(key); }));
    expect(QStringLiteral("live: the id the window holds is the helper's id of that run"), id == idOf(asking) && !id.isEmpty());

    // Leaving a page of the window: the run carries on, said once.
    first.frame->choose(Frame::settingsKey());
    expect(QStringLiteral("live: leaving the page says the run carries on and is in Activity (%1)").arg(first.frame->statusText()),
           first.frame->statusText().contains(QStringLiteral("carries on with its run"))
               && first.frame->statusText().contains(QStringLiteral("Activity")));
    first.frame->flash(QString());
    first.frame->choose(Frame::settingsKey());
    expect(QStringLiteral("live: and it is not said again for that run"),
           !first.frame->statusText().contains(QStringLiteral("carries on")));

    // Closing the window: let go of the helper, not stop it.
    Frame *firstFrame = first.frame;
    first.window->close();
    expect(QStringLiteral("live: closing the window says the run carries on (%1)").arg(firstFrame->statusText()),
           firstFrame->statusText().contains(QStringLiteral("carries on without this window")));
    first.window->deleteLater();
    first.window = nullptr;
    settle(500);
    int code = -1;
    const QJsonArray listed = QJsonDocument::fromJson(call({"--list"}, &code)).object()["runs"].toArray();
    bool stillAsking = false, attachable = false;
    for (const QJsonValue &run : listed) {
        if (run["id"].toString() == id) {
            stillAsking = run["state"].toString() == QLatin1String("waiting");
            attachable = run["attachable"].toBool();
        }
    }
    expect(QStringLiteral("live: the run is still waiting for its answer after its window is gone"), stillAsking && attachable);

    page->chooseTitle(asking);
    expect(QStringLiteral("live: the other window follows it and offers its choices as buttons (%1)")
               .arg(page->answerButtons().join(", ")),
           waitFor([&] { return page->following() && page->answerButtons() == QStringList({"Red", "Blue"}); }));
    expect(QStringLiteral("live: the question is shown and nothing says only one window can answer"),
           page->detailText().contains(QStringLiteral("Which colour?")) && !page->detailText().contains(QStringLiteral("Only the window"))
               && page->offersStop());
    save(mineWindow, prefix + "-live-question");

    // A second window shows the same run.
    Frame other(Frame::Disk);
    other.setContent(new QWidget);
    other.resize(1000, 640);
    other.show();
    other.choose(key);
    ActivityPage *there = other.activity();
    waitFor([&] { return there->listedTitles().contains(asking); });
    there->chooseTitle(asking);
    expect(QStringLiteral("live: a second window follows the same run and shows the same buttons"),
           waitFor([&] { return there->following() && there->answerButtons() == QStringList({"Red", "Blue"}); }));
    expect(QStringLiteral("live: the answer from the second window is taken"), there->answer(QStringLiteral("Blue")));
    expect(QStringLiteral("live: the first window sees that it was answered, with the answer, and offers no buttons"),
           waitFor([&] {
               return page->detailText().contains(QStringLiteral("Answered: Blue.")) || stateIn(mine, asking).startsWith("Done");
           }));
    expect(QStringLiteral("live: the run then ends as done with what the agent made of the answer (%1)").arg(stateIn(mine, asking)),
           waitFor([&] { return stateIn(mine, asking).startsWith(QStringLiteral("Done: I used the colour you chose: Blue.")); }));
    expect(QStringLiteral("live: the second window shows it as done too"),
           waitFor([&] { return stateIn(other, asking).startsWith(QStringLiteral("Done")) && there->answerButtons().isEmpty(); }));
    const QJsonArray firstRecord = record(id);
    expect(QStringLiteral("live: the record holds one answer and the run ended done"),
           count(firstRecord, QStringLiteral("answer")) == 1 && endState(firstRecord) == QLatin1String("done"));
    expect(QStringLiteral("live: a run that is over is not followed and offers no Stop"),
           waitFor([&] { return !page->following() && !page->offersStop() && page->detailText().contains(QStringLiteral("Done")); }));

    // ---- A run still in its own window is answered from Activity: the window learns of it.
    const QString waiting = QStringLiteral("Pick another colour for the check");
    Started second = start(QStringLiteral("asks"), waiting);
    waitFor([&] { return stateIn(mine, waiting) == QStringLiteral("Waits for an answer"); });
    page->chooseTitle(waiting);
    expect(QStringLiteral("live: a run whose own window is open can be answered from Activity"),
           waitFor([&] { return page->following() && page->answerButtons() == QStringList({"Red", "Blue"}); })
               && page->answer(QStringLiteral("Red")));
    expect(QStringLiteral("live: that window's page says the question was answered from Activity (%1)")
               .arg(workWords(second.window).section(QLatin1Char('\n'), 0, 3)),
           waitFor([&] { return workWords(second.window).contains(QStringLiteral("answered from Activity")); }));
    waitFor([&] { return stateIn(mine, waiting).startsWith(QStringLiteral("Done")); });
    expect(QStringLiteral("live: and its run ended done"), stateIn(mine, waiting).startsWith(QStringLiteral("Done")));
    closeWindow(second);

    // ---- An administrator step is approved from Activity.
    const QString admin = QStringLiteral("Show which user the agent runs as");
    Started third = start(QStringLiteral("administrator"), admin);
    waitFor([&] { return !third.frame->ownRun().isEmpty(); });
    waitFor([&] { return stateIn(mine, admin) == QStringLiteral("Waits for an answer"); });
    closeWindow(third);
    page->chooseTitle(admin);
    expect(QStringLiteral("live: an administrator step shows the command and Approve and Refuse (%1)").arg(page->answerButtons().join(", ")),
           waitFor([&] { return page->following() && page->answerButtons() == QStringList({"Approve", "Refuse"}); })
               && page->detailText().contains(QStringLiteral("It wants to run this as administrator"))
               && page->detailText().contains(QStringLiteral("id -u")));
    save(mineWindow, prefix + "-live-approval");
    expect(QStringLiteral("live: Approve is given from Activity"), page->answer(QStringLiteral("Approve")));
    expect(QStringLiteral("live: the step ran, and the record says the person approved it as administrator"),
           waitFor([&] { return page->detailText().contains(QStringLiteral("as administrator, you approved")); }));
    waitFor([&] { return stateIn(mine, admin).startsWith(QStringLiteral("Done")); });
    const QJsonArray adminRecord = record(idOf(admin));
    expect(QStringLiteral("live: the record holds the answer Approve once and the run ended done"),
           count(adminRecord, QStringLiteral("answer")) == 1 && endState(adminRecord) == QLatin1String("done"));

    // ---- A run that works is followed live and stopped from Activity.
    const QString hang = QStringLiteral("Wait for the long job");
    Started fourth = start(QStringLiteral("hang"), hang);
    waitFor([&] { return stateIn(mine, hang) == QStringLiteral("Running now"); });
    closeWindow(fourth);
    page->chooseTitle(hang);
    expect(QStringLiteral("live: a working run is followed: what it does now, and what it did so far"),
           waitFor([&] {
               return page->following() && page->detailText().contains(QStringLiteral("What it does now"))
                      && page->detailText().contains(QStringLiteral("sleep 300"));
           }));
    expect(QStringLiteral("live: a working run offers Stop and no answer"), page->offersStop() && page->answerButtons().isEmpty());
    save(mineWindow, prefix + "-live-follow");
    QStringList pids;
    {
        QFile file(logFile + ".pids");
        if (file.open(QIODevice::ReadOnly))
            pids = QString::fromUtf8(file.readAll()).split(QLatin1Char(' '), Qt::SkipEmptyParts);
    }
    expect(QStringLiteral("live: the agent and its command are running before Stop"),
           pids.size() == 2 && alive(pids[0].toLongLong()) && alive(pids[1].toLongLong()));
    expect(QStringLiteral("live: Stop is pressed in Activity"), page->stopChosen());
    expect(QStringLiteral("live: the run ends as stopped, which the page says (%1)").arg(stateIn(mine, hang)),
           waitFor([&] { return stateIn(mine, hang) == QStringLiteral("Stopped before it finished"); }, 30));
    expect(QStringLiteral("live: a stopped run says how it ended and offers no Stop"),
           waitFor([&] { return page->detailText().contains(QStringLiteral("How it ended")) && !page->offersStop(); }));
    expect(QStringLiteral("live: the agent and what it started are gone"),
           waitFor([&] { return pids.size() == 2 && !alive(pids[0].toLongLong()) && !alive(pids[1].toLongLong()); }, 30));
    expect(QStringLiteral("live: nothing waits or runs any more, and the badge says so"),
           waitFor([&] { return mine.badge(key) == 0; }));
}

void guardRefuses(const Expect &expect, const QString &folder, const QString &mustName)
{
    // Bare: no stand-in named, nothing but what a shell needs.
    QProcessEnvironment bare;
    bare.insert(QStringLiteral("PATH"), qEnvironmentVariable("PATH"));
    bare.insert(QStringLiteral("QT_QPA_PLATFORM"), QStringLiteral("offscreen"));
    const QString pictures = QDir(folder).filePath("refused-pictures");
    QDir(pictures).removeRecursively();
    QProcess process;
    process.setProcessEnvironment(bare);
    process.start(QCoreApplication::applicationFilePath(), {"--shots", pictures});
    process.waitForFinished(120000);
    const QString said = QString::fromUtf8(process.readAllStandardError());
    expect(QStringLiteral("guard: a bare --shots refuses to start, with exit code 2 (%1)").arg(process.exitCode()),
           process.exitStatus() == QProcess::NormalExit && process.exitCode() == 2);
    expect(QStringLiteral("guard: it says what is missing in a sentence (%1)").arg(said.trimmed().left(120)),
           said.contains(QStringLiteral("would reach this computer for real")) && said.contains(mustName)
               && said.contains(QStringLiteral("LEVITATE_ACTIVITY_AGENT")) && said.contains(QStringLiteral("LEVITATE_AGENT_CODEX")));
    expect(QStringLiteral("guard: and it started nothing (no folder for pictures)"), !QDir(pictures).exists());
}

} // namespace livestory
