#include "shots.h"

#include "activity.h"
#include "frame.h"
#include "look.h"
#include "parts.h"
#include "settings.h"
#include "nextsystem.h"
#include "pages.h"
#include "window.h"

#include <QCoreApplication>
#include <QDir>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QElapsedTimer>
#include <QFileInfo>
#include <QProcess>
#include <QSettings>
#include <QPushButton>
#include <QTextStream>

#include <csignal>
#include <functional>

namespace {

void waitUntilIdle(Window &window, int seconds = 30)
{
    QElapsedTimer clock;
    clock.start();
    while (window.busy() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(150);
}

const char *nvidia = "My graphics card is an NVIDIA RTX 4070. I want NVIDIA's own driver instead of the built-in one.";
const char *sshd = "Turn off password logins over ssh; I only use keys.";

QString asked(const char *request)
{
    return QStringLiteral("The request:\n%1\n\nThe expected result:\nAfter a restart the change is in effect and the "
                          "system starts as before. Nothing else changes.\n")
        .arg(QLatin1String(request));
}



// ---------------------------------------------------------------- The next system and Getting back
//
// What a page says, for the checks: every label as written, every button as
// "[Words]" or "[Words](off)" when recab's verdict leaves it off.

QString wordsOn(QWidget *page)
{
    QStringList words;
    for (QWidget *child : page->findChildren<QWidget *>()) {
        if (!child->isVisibleTo(page))
            continue;
        if (auto *button = qobject_cast<QPushButton *>(child))
            words << QStringLiteral("[%1]%2").arg(button->text(), button->isEnabled() ? QString() : QStringLiteral("(off)"));
        else if (auto *label = qobject_cast<QLabel *>(child))
            words << label->text();
    }
    return words.join(QLatin1Char('\n'));
}

// Shows one of the three pages and says what it holds; Overview again after.
QString showWords(Window &window, int which)
{
    // As a click on the rail does: leaves Activity or Settings if they were showing.
    window.frame()->select(which == 0 ? QStringLiteral("overview") : which == 1 ? QStringLiteral("next")
                                                                                : QStringLiteral("back"));
    if (which == 0)
        window.showMain();
    else if (which == 1)
        window.showNext();
    else
        window.showBack();
    settle(200);
    switch (which) {
    case 0:
        return wordsOn(window.findChild<SystemPage *>());
    case 1:
        return wordsOn(window.findChild<NextPage *>());
    }
    return wordsOn(window.findChild<BackPage *>());
}

QString overviewWords(Window &window) { return showWords(window, 0); }
QString nextWords(Window &window) { return showWords(window, 1); }
QString backWords(Window &window) { return showWords(window, 2); }

// ---------------------------------------------------------------- Activity
//
// The pages of the frame that are not recab's: Activity, and the strip about
// the next system. They need runs to show, which `gui/agent/fixtures/records`
// makes by driving the built levitate-agent with the stand-in Codex. Both
// stories call this while the next system holds changes.

using Save = std::function<void(QWidget &, const QString &)>;
using Expect = std::function<void(const QString &, bool)>;

void waitFor(const std::function<bool()> &holds, int seconds = 20)
{
    QElapsedTimer clock;
    clock.start();
    while (!holds() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(150);
}

void clickButton(QWidget *inside, const QString &words)
{
    for (QPushButton *button : inside->findChildren<QPushButton *>())
        if (button->text() == words && button->isVisible())
            button->click();
}

// `holds`: how many changes the next system holds now, as the window knows.
void activityShots(Window &window, const QString &folder, const QString &prefix, int holds, const Save &save,
                   const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        out << "not as expected: Activity needs RECAB_GUI_RECORDS (gui/agent/fixtures/records) and "
               "LEVITATE_ACTIVITY_AGENT (the built levitate-agent), see README.md\\n";
        expect(QStringLiteral("the records for Activity could be made"), false);
        return;
    }
    // The runs, in a folder of their own; the page reads them through levitate-agent.
    const QString where = QDir(folder).filePath(prefix + "-records");
    QProcess make;
    make.start(script, {where, agent});
    make.waitForFinished(60000);
    const QString said = QString::fromUtf8(make.readAllStandardOutput()).trimmed();
    expect(QStringLiteral("the records for Activity were made (%1)").arg(QString::fromUtf8(make.readAllStandardError()).trimmed()),
           make.exitStatus() == QProcess::NormalExit && make.exitCode() == 0 && said.startsWith("state="));
    if (!said.startsWith("state="))
        return;
    const QString state = said.section(' ', 0, 0).mid(6);
    const qint64 waitingPid = said.section("waiting_pid=", 1).toLongLong();
    qputenv("XDG_STATE_HOME", state.toUtf8());
    ActivityPage *page = frame->activity();

    // What the rail shows before Activity is opened: the badge is true from the start.
    frame->readAgain();
    waitFor([&] { return page->loaded() && frame->badge(Frame::activityKey()) > 0; });
    expect(QStringLiteral("the Activity entry counts the one run that waits"),
           frame->badge(Frame::activityKey()) == 2 && frame->badgeWarns(Frame::activityKey()));

    frame->choose(Frame::activityKey());
    waitFor([&] { return page->listedTitles().size() == 5 && !page->detailText().isEmpty(); });
    expect(QStringLiteral("Activity is showing and marked"), frame->activityShown() && frame->selected() == Frame::activityKey());
    expect(QStringLiteral("Activity lists the five runs, newest first"),
           page->listedTitles()
               == QStringList({"Install the NVIDIA driver", "Update Element", "Turn off password logins over ssh",
                               "Install htop for this computer", "Install Krita for this computer"}));
    expect(QStringLiteral("the newest run is chosen and shows the administrator step the person refused"),
           page->detailText().contains("you refused, so it was not run") && page->detailText().contains("id -u")
               && page->detailText().contains("The whole prompt"));
    expect(QStringLiteral("a run that is over is not offered as answerable"), !page->offersGoTo());
    save(window, prefix + "-activity-list");

    // A run that is done: what it ran and changed, and where.
    page->chooseTitle("Install htop for this computer");
    waitFor([&] { return page->detailText().contains("ls -l"); });
    expect(QStringLiteral("a done run shows its commands, its files and the agent's last word"),
           page->detailText().contains("ls -l") && page->detailText().contains("/home/x/a.txt")
               && page->detailText().contains("in the agent's sandbox")
               && page->detailText().contains("I made the change. Everything checks out."));
    expect(QStringLiteral("a done run has no way to be answered"), !page->offersGoTo());
    save(window, prefix + "-a-done-run");

    page->chooseTitle("Turn off password logins over ssh");
    waitFor([&] { return page->detailText().contains("It asks"); });
    expect(QStringLiteral("a run that waits says that only its own window can answer it"),
           page->detailText().contains("It asks") && page->detailText().contains("Which colour?")
               && page->detailText().contains("Only the window that started it, in System, can answer")
               && !page->offersGoTo());
    save(window, prefix + "-a-run-that-waits");

    // A run that is going, not waiting: counted as running, listed as such, and badged.
    QStringList chips;
    for (QLabel *label : page->findChildren<QLabel *>())
        if (dynamic_cast<Chip *>(label))
            chips << label->text();
    page->chooseTitle("Install Krita for this computer");
    waitFor([&] { return page->detailText().contains("is working on it"); });
    expect(QStringLiteral("the page counts one run that waits and one that runs (%1)").arg(chips.join(", ")),
           chips == QStringList({"1 waits for an answer", "1 running"}));
    expect(QStringLiteral("a running run is described as working, with what it ran so far, and cannot be answered"),
           page->detailText().contains("Codex is working on it") && page->detailText().contains("sleep 300")
               && !page->detailText().contains("It asks") && !page->offersGoTo());
    expect(QStringLiteral("the badge counts the waiting and the running run"),
           frame->badge(Frame::activityKey()) == 2 && frame->badgeWarns(Frame::activityKey()));
    expect(QStringLiteral("the list says it is running (%1)").arg(page->listedStates().join(" | ")),
           page->listedStates().value(4) == "Running now");
    save(window, prefix + "-a-run-that-runs");
    page->chooseTitle("Turn off password logins over ssh");
    waitFor([&] { return page->detailText().contains("It asks"); });

    // The same run is this window's own live run: it can be gone to.
    page->setOwnRun(waitingPid);
    waitFor([&] { return page->offersGoTo(); });
    expect(QStringLiteral("this window's own live run offers Go to it"),
           page->offersGoTo() && page->detailText().contains("It waits in this window"));
    save(window, prefix + "-this-windows-own-run");
    clickButton(page, QStringLiteral("Go to it"));
    settle(200);
    expect(QStringLiteral("Go to it leaves Activity for the app's own pages"),
           !frame->activityShown() && frame->selected() == "overview");
    frame->choose(Frame::activityKey());
    page->setOwnRun(0);
    waitFor([&] { return page->loaded() && !page->offersGoTo(); });

    // The filter.
    page->showOnlyThisApp(true);
    expect(QStringLiteral("only this app's runs are listed"),
           page->listedTitles() == QStringList({"Install the NVIDIA driver", "Turn off password logins over ssh"}));
    page->showOnlyThisApp(false);
    expect(QStringLiteral("all apps again"), page->listedTitles().size() == 5);

    // The strip, here, while the next system holds changes.
    expect(QStringLiteral("the strip shows on Activity while the next system holds changes"),
           frame->stripShown()
               && frame->stripText() == (holds == 1 ? QStringLiteral("The next system holds 1 change.")
                                                    : QStringLiteral("The next system holds %1 changes.").arg(holds)));
    save(window, prefix + "-activity-with-the-strip");

    // And on the pages that are not Overview, but not on Overview.
    frame->choose(QStringLiteral("overview"));
    settle(200);
    expect(QStringLiteral("Overview is marked and carries no strip"),
           !frame->activityShown() && frame->selected() == "overview" && !frame->stripShown());
    window.showAsk();
    settle(100);
    expect(QStringLiteral("the strip shows on the page where a change is asked for"), frame->stripShown());
    window.showMain();
    settle(100);
    expect(QStringLiteral("and goes again on Overview"), !frame->stripShown());

    // The agent in the status line, asked of levitate-agent with the stand-in Codex.
    const QString codex = QDir(QFileInfo(script).absolutePath()).filePath("codex");
    qputenv("LEVITATE_AGENT_CODEX", codex.toUtf8());
    const QString stand = qEnvironmentVariable("RECAB_GUI_AGENT");
    auto agentSays = [&](const QString &words) {
        waitFor([&] { return frame->agentText() == words; }, 10);
        return frame->agentText() == words;
    };
    frame->setAgent(agent);
    expect(QStringLiteral("the status line names the agent and that it is signed in (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, signed in")));
    qputenv("LEVITATE_STANDIN_SIGNED_OUT", "1");
    frame->readAgain();
    expect(QStringLiteral("the status line says when it is not signed in (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, not signed in")));
    qunsetenv("LEVITATE_STANDIN_SIGNED_OUT");
    qputenv("LEVITATE_AGENT_CODEX", "/nonexistent/codex");
    frame->readAgain();
    expect(QStringLiteral("the status line says when it is not installed (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex is not installed")));
    save(window, prefix + "-agent-not-installed");
    qunsetenv("LEVITATE_AGENT_CODEX");
    frame->setAgent(stand);
    expect(QStringLiteral("a stand-in agent is shown by its file name"),
           frame->agentText() == QStringLiteral("Agent: %1").arg(QFileInfo(stand).fileName()));

    // Over: the waiting agent is stopped, and the page forgets the records.
    QProcess::execute(script, {"--stop", where});
    qunsetenv("XDG_STATE_HOME");
    page->setOwnRun(0);
    window.showMain();
    settle(100);
}

// ---------------------------------------------------------------- Settings
//
// The page the frame owns after Activity: the agent, what it may do without
// asking, and the windows. Everything runs through the built levitate-agent
// with the stand-in Codex, and HOME and the XDG folders point into a folder of
// the story's own, so nothing of the person's is read or written.

QString fileText(const QString &path)
{
    QFile file(path);
    return file.open(QIODevice::ReadOnly) ? QString::fromUtf8(file.readAll()) : QString();
}

void settingsShots(Window &window, const QString &folder, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        out << "not as expected: Settings needs RECAB_GUI_RECORDS (gui/agent/fixtures/records) and "
               "LEVITATE_ACTIVITY_AGENT (the built levitate-agent), see README.md\n";
        expect(QStringLiteral("the helper for Settings could be named"), false);
        return;
    }
    const QString fixtures = QFileInfo(script).absolutePath();
    const QString other = QFileInfo(QDir(fixtures).filePath("../../disk/fixtures/agent")).canonicalFilePath();
    const QString home = QDir(folder).filePath("settings-home");
    QDir(home).removeRecursively();
    QDir().mkpath(home);
    const QString file = home + "/config/levitate/agent.json";
    const QByteArrayList names{"HOME", "XDG_CONFIG_HOME", "XDG_STATE_HOME", "LEVITATE_AGENT_CODEX", "LEVITATE_AGENT_ROOT_HELPER",
                               "LEVITATE_STANDIN_LOG", "LEVITATE_STANDIN_LOGIN_SECONDS", "LEVITATE_STANDIN_STORY"};
    QMap<QByteArray, QByteArray> before;
    for (const QByteArray &name : names)
        if (qEnvironmentVariableIsSet(name.constData()))
            before.insert(name, qgetenv(name.constData()));
    // Where the size a window had is kept, here and not in the person's own folder.
    QSettings::setPath(QSettings::NativeFormat, QSettings::UserScope, home + "/config");
    qputenv("HOME", home.toUtf8());
    qputenv("XDG_CONFIG_HOME", (home + "/config").toUtf8());
    qputenv("XDG_STATE_HOME", (home + "/state").toUtf8());
    qputenv("LEVITATE_AGENT_CODEX", QDir(fixtures).filePath("codex").toUtf8());
    qputenv("LEVITATE_AGENT_ROOT_HELPER", "");
    qputenv("LEVITATE_STANDIN_LOG", (home + "/standin.log").toUtf8());
    qputenv("LEVITATE_STANDIN_STORY", "plain");
    // Long enough to look at the page while the sign in waits.
    qputenv("LEVITATE_STANDIN_LOGIN_SECONDS", "2");

    SettingsPage *page = frame->settings();
    auto idle = [&] {
        waitFor([&] { return !page->busy(); }, 30);
    };
    auto agentSays = [&](const QString &words) {
        waitFor([&] { return frame->agentText() == words; }, 10);
        return frame->agentText() == words;
    };
    auto shows = [&](const QString &words) { return page->pageText().contains(words); };
    // The status line asks levitate-agent only when the window's agent is that program.
    const QString stand = qEnvironmentVariable("RECAB_GUI_AGENT");
    frame->setAgent(agent);
    expect(QStringLiteral("the status line starts as the stand-in Codex signed in (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, signed in")));

    // The rail: Settings under Activity, added by the frame.
    const QStringList titles = frame->entryTitles();
    expect(QStringLiteral("the rail has Settings under Activity (%1)").arg(titles.join(", ")),
           titles.size() >= 2 && titles.mid(titles.size() - 2) == QStringList({"Activity", "Settings"}));

    // Before the helper has answered: it says so.
    frame->choose(Frame::settingsKey());
    expect(QStringLiteral("Settings is showing and marked, and no app page was asked for"),
           frame->settingsShown() && frame->selected() == Frame::settingsKey() && !frame->activityShown());
    expect(QStringLiteral("it says it is asking while it waits"), !page->loaded() && shows("Asking levitate-agent"));
    QCoreApplication::processEvents(); // the new page is shown once the loop has run
    window.grab().save(QDir(folder).filePath("s0-reading.png"));
    waitFor([&] { return page->loaded(); });

    // As first read.
    expect(QStringLiteral("the page was read (%1)").arg(page->problem()), page->loaded() && page->problem().isEmpty());
    for (const char *words : {"Settings", "One set for Recipe, System and Disk. What is set here holds in all three.",
                              "The agent that does the work", "Codex", "Installed, and signed in", "Sign out…",
                              "Another program", "Any program that takes a request and reports back the same way", "Choose…",
                              "Try it", "Asks the chosen agent to say hello.", "What the agent may do without asking",
                              "Read files and look things up", "Always", "Run commands as you, in your own folders",
                              "Ask each time", "Run a command as administrator", "Always asks. It cannot be switched off.",
                              "The windows", "Each window opens at the size it had. Language follows the desktop.",
                              "Settings are kept for you, not for everyone on this computer.",
                              "With Allowed, the agent's own sandbox lets it write in your home folder."})
        expect(QStringLiteral("the page says: %1").arg(words), shows(QString::fromUtf8(words)));
    expect(QStringLiteral("Claude Code is not shown and nothing is offered that has nothing behind it"),
           !shows("Claude") && !shows("Write a recipe") && !shows("Notices") && !shows("update"));
    expect(QStringLiteral("Codex is the chosen one and commands are asked each time"),
           page->checkedAgent() == "codex" && page->commandsInForce() == "ask" && page->agentInForce() == "codex");
    save(window, "s1-as-first-read");

    // "Allowed": the helper's file says so, and so does the page.
    page->pickCommands(QStringLiteral("Allowed"));
    idle();
    expect(QStringLiteral("the helper's file says allowed (%1)").arg(fileText(file).simplified()),
           QJsonDocument::fromJson(fileText(file).toUtf8()).object()["run_commands"] == "allowed");
    expect(QStringLiteral("the page shows what the helper reports: Allowed"),
           page->commandsInForce() == "allowed" && page->pageText().endsWith("Allowed"));
    save(window, "s2-allowed");

    // Signing out asks first; nothing has changed until it is said yes.
    expect(QStringLiteral("Sign out… is offered"), page->press(QStringLiteral("Sign out…")));
    expect(QStringLiteral("it asks first, in place"),
           shows("Sign out of Codex? The apps cannot hand anything to it until you sign in again."));
    save(window, "s3-sign-out-asks");
    expect(QStringLiteral("nobody is signed out yet (%1)").arg(frame->agentText()), frame->agentText() == "Agent: Codex, signed in");
    expect(QStringLiteral("Keep it leaves it as it was"), page->press(QStringLiteral("Keep it")) && !shows("Sign out of Codex?"));
    page->press(QStringLiteral("Sign out…"));
    expect(QStringLiteral("saying yes signs out"), page->press(QStringLiteral("Sign out")));
    idle();
    expect(QStringLiteral("the row says not signed in, and offers to sign in"),
           shows("Installed, not signed in") && shows("Sign in…") && !shows("Sign out…"));
    expect(QStringLiteral("the status line follows at once (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, not signed in")));
    expect(QStringLiteral("the stand-in's own state says so"), fileText(home + "/.standin-codex-signed") == "out");
    save(window, "s4-signed-out");

    // A sign in that fails says why, in the row.
    qputenv("LEVITATE_STANDIN_LOGIN_FAILS", "1");
    page->press(QStringLiteral("Sign in…"));
    idle();
    qunsetenv("LEVITATE_STANDIN_LOGIN_FAILS");
    expect(QStringLiteral("a failed sign in shows the helper's sentence in place (%1)").arg(page->notice()),
           page->notice().contains("was not completed") && shows("was not completed") && shows("Installed, not signed in"));
    save(window, "s5-sign-in-failed");

    // Sign in again: what Codex prints (an address) is shown while it waits.
    expect(QStringLiteral("Sign in… is offered"), page->press(QStringLiteral("Sign in…")));
    waitFor([&] { return shows("https://auth.standin.invalid"); }, 15);
    expect(QStringLiteral("the address Codex printed is shown while it waits"),
           shows("https://auth.standin.invalid") && shows("Signing in") && page->busy());
    save(window, "s6-signing-in");
    idle();
    expect(QStringLiteral("signed in again: the row says so"), shows("Installed, and signed in") && shows("Sign out…")
                                                                   && !shows("Signing in"));
    expect(QStringLiteral("and so does the status line (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, signed in")));
    save(window, "s7-signed-in-again");

    // Another program, by its path; then back to Codex.
    page->chooseProgram(other);
    idle();
    expect(QStringLiteral("the other program is chosen, as the helper reports it (%1)").arg(page->agentInForce()),
           page->agentInForce() == other && page->checkedAgent() == "other" && shows(other) && shows("Found, and it can be run"));
    {
        const QJsonObject kept = QJsonDocument::fromJson(fileText(file).toUtf8()).object();
        expect(QStringLiteral("the file has the path and kept the other setting"),
               kept["agent"] == other && kept["run_commands"] == "allowed");
    }
    expect(QStringLiteral("the status line names it (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: agent")));
    expect(QStringLiteral("the page says that only Codex reads the setting"), shows("Only Codex reads this setting."));
    save(window, "s8-another-program");
    expect(QStringLiteral("choosing Codex again"), page->press(QStringLiteral("Codex")));
    idle();
    expect(QStringLiteral("Codex is back, signed in (%1)").arg(page->agentInForce()),
           page->agentInForce() == "codex" && page->checkedAgent() == "codex" && shows("Installed, and signed in"));
    expect(QStringLiteral("the status line follows (%1)").arg(frame->agentText()),
           agentSays(QStringLiteral("Agent: Codex, signed in")));
    save(window, "s9-back-to-codex");

    // A refused value is shown in place; nothing changes.
    const QString kept = fileText(file);
    page->chooseProgram(QStringLiteral("/nonexistent/agent"));
    idle();
    expect(QStringLiteral("the refusal is in the page (%1)").arg(page->notice()),
           page->notice() == "/nonexistent/agent does not exist." && shows("/nonexistent/agent does not exist."));
    expect(QStringLiteral("and nothing was written or changed"),
           fileText(file) == kept && page->agentInForce() == "codex" && page->checkedAgent() == "codex");
    save(window, "s10-refused");

    // "Try it": the stand-in answers; the run is in Activity.
    expect(QStringLiteral("Try it is offered"), page->press(QStringLiteral("Try it")));
    expect(QStringLiteral("while it runs the button says so and cannot be pressed"),
           shows("Asking…") && !page->press(QStringLiteral("Asking…")) && page->busy());
    // Straight away: the stand-in answers within the time a picture takes.
    QCoreApplication::processEvents();
    window.grab().save(QDir(folder).filePath("s11-asking.png"));
    idle();
    expect(QStringLiteral("what came back is shown (%1)").arg(page->pageText().section("It answered", 1, 1).left(80)),
           shows("It answered: I made the change. Everything checks out.") && shows("Try it"));
    save(window, "s12-try-it-answered");
    frame->choose(Frame::activityKey());
    waitFor([&] { return frame->activity()->listedTitles().contains("Say hello"); }, 15);
    frame->activity()->chooseTitle("Say hello");
    waitFor([&] { return frame->activity()->detailText().contains("Say hello in one sentence"); }, 15);
    expect(QStringLiteral("the run is listed in Activity under System (%1)").arg(frame->activity()->listedTitles().join(", ")),
           frame->activity()->listedTitles() == QStringList({"Say hello"})
               && frame->activity()->detailText().contains("Asked in System")
               && frame->activity()->detailText().contains("Do not run any command and do not change any file."));
    save(window, "s13-try-it-in-activity");

    // Coming back to Settings reads it again, without the page being rebuilt from nothing.
    frame->choose(Frame::settingsKey());
    waitFor([&] { return page->loaded() && !page->busy(); });
    expect(QStringLiteral("Settings shows again, with the last answer"), frame->settingsShown() && shows("It answered:"));

    // A helper that cannot be asked: the page says so, with a way to try again.
    qputenv("LEVITATE_ACTIVITY_AGENT", "/nonexistent/levitate-agent");
    {
        Window lonely;
        lonely.show();
        lonely.frame()->choose(Frame::settingsKey());
        waitFor([&] { return !lonely.frame()->settings()->problem().isEmpty(); }, 10);
        expect(QStringLiteral("a helper that does not exist is said so (%1)").arg(lonely.frame()->settings()->problem()),
               lonely.frame()->settings()->problem().contains("levitate-agent")
                   && lonely.frame()->settings()->pageText().contains("Settings could not be read")
                   && lonely.frame()->settings()->pageText().contains("Try again"));
        save(lonely, "s14-not-read");
    }
    qputenv("LEVITATE_ACTIVITY_AGENT", agent.toUtf8());

    // The size a window had: kept when it closes, read when it opens, and
    // neither in the picture modes.
    {
        const QString key = QStringLiteral("story-window");
        QWidget first;
        rememberWindowSize(&first, key, {1100, 700}, true);
        expect(QStringLiteral("a window that was never closed opens at the first size"), first.size() == QSize(1100, 700));
        first.resize(900, 600);
        first.show();
        first.close();
        QSettings kept(QStringLiteral("LevitateOS"), QStringLiteral("windows"));
        expect(QStringLiteral("closing keeps its size, under one key (%1)").arg(kept.value(key).toString()),
               kept.value(key).toSize() == QSize(900, 600) && kept.allKeys() == QStringList({key}));
        QWidget second;
        rememberWindowSize(&second, key, {1100, 700}, true);
        expect(QStringLiteral("the next one opens at the size it had"), second.size() == QSize(900, 600));
        QWidget picture;
        rememberWindowSize(&picture, key, {1100, 700}, false);
        picture.resize(800, 500);
        picture.show();
        picture.close();
        kept.sync();
        expect(QStringLiteral("in picture mode nothing is read and nothing is written"),
               picture.size() == QSize(800, 500) && kept.value(key).toSize() == QSize(900, 600));
        QWidget third;
        rememberWindowSize(&third, key, {1100, 700}, false);
        expect(QStringLiteral("and the first size is used"), third.size() == QSize(1100, 700));
    }

    frame->setAgent(stand);
    for (const QByteArray &name : names) {
        if (before.contains(name))
            qputenv(name.constData(), before.value(name));
        else
            qunsetenv(name.constData());
    }
    window.showMain();
    settle(100);
}

} // namespace

// The story needs the stand-in started with RECAB_STANDIN_FAIL_TRIALS=1, so
// the first restart into the next system fails and the second is kept.
int takeShots(Window &window, const QString &folder)
{
    QDir().mkpath(folder);
    QTextStream out(stdout);
    bool ok = true;
    auto save = [&](QWidget &widget, const QString &name) {
        settle(200);
        ok &= widget.grab().save(QDir(folder).filePath(name + ".png"));
    };
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << " (state " << window.status().state << ")\n";
            ok = false;
        }
    };
    window.show();
    // Before recab has answered: what a person sees while the state is read.
    ok &= window.grab().save(QDir(folder).filePath("00-reading.png"));
    waitUntilIdle(window);

    expect("starts stable", window.status().state == "stable");
    save(window, "01-stable");

    // The two pages beside Overview, with no next system.
    {
        const QString overview = overviewWords(window);
        expect("Overview names the one thing to do, and sends to the two pages",
               overview.contains("What to do next") && overview.contains("[Make a change…]")
                   && overview.contains("The rest is on:") && overview.contains("Getting back")
                   && !overview.contains("[Go back…]"));
        const QString next = nextWords(window);
        expect("the next system's page says there is none, and offers to make one",
               next.contains("There is no next system yet") && next.contains("[Make a change…]")
                   && next.contains("[Install software…]") && window.frame()->badge("next") == 0
                   && window.frame()->selected() == "next");
        save(window, "p01-next-none");
        const QString back = backWords(window);
        expect("Getting back offers the way back, and says no restart is set",
               back.contains("[Go back…]") && back.contains("None is set.") && back.contains("[Cancel it](off)")
                   && back.contains("[Keep it](off)") && back.contains("Kept already.")
                   && back.contains("Pick the running system, the one that is kept.")
                   && !back.contains("Slot A") && !back.contains("recab commit(off)"));
        expect("the steps for a desktop that does not start say how to make the choice stay, without names",
               back.contains("recab commit") && back.contains("pick the system from before instead"));
        save(window, "p02-back");
        expect("the strip is not on Overview, nor on the next system's page", !window.frame()->stripShown());
        window.showMain();
        settle(100);
    }

    window.showAsk(nvidia);
    save(window, "02-ask");

    {
        QScopedPointer<ConfirmDialog> question(window.reuseQuestion());
        question->show();
        save(*question, "03-goes-into-the-next-system");
    }

    // The change is stopped half way, which is also how an interruption looks.
    qputenv("RECAB_STANDIN_STOP_AT", "change");
    window.handOver(asked(nvidia), false);
    settle(2500);
    save(window, "04-preparing");
    window.stopRun();
    waitUntilIdle(window);
    qunsetenv("RECAB_STANDIN_STOP_AT");
    expect("a stopped change is reported as interrupted", window.status().state == "interrupted");
    save(window, "05-interrupted");
    {
        const QString next = nextWords(window);
        expect("the next system's page says it was interrupted, and offers to continue or discard",
               next.contains("Preparing the next system was interrupted") && next.contains("[Continue where it stopped]")
                   && next.contains("[Discard…]") && !next.contains("[Finish…]"));
        save(window, "p03-next-interrupted");
        const QString back = backWords(window);
        expect("Getting back says the system from before is gone while the next one holds its place",
               back.contains("There is none now: its place holds the next system.") && back.contains("[Go back…](off)"));
        window.showMain();
        settle(100);
    }

    window.recover();
    waitUntilIdle(window);
    expect("recovering finishes the change", window.status().changes.size() == 1);

    window.handOver(asked(sshd), false);
    // The agent asks something on the way, and waits.
    for (int waited = 0; !window.agentAsks() && window.busy() && waited < 200; ++waited)
        settle(50);
    expect("the agent asks about root logins", window.agentAsks());
    save(window, "05b-the-agent-asks");
    window.answerAgent(QStringLiteral("Leave it"));
    waitUntilIdle(window);
    expect("a second change joins the same next system", window.status().changes.size() == 2);
    save(window, "06-next-system");
    {
        const QStringList titles = window.status().changeTitles();
        const QString next = nextWords(window);
        expect("the badge on the rail is the number of changes",
               window.frame()->badge("next") == window.status().changes.size() && window.status().changes.size() == 2);
        expect("the next system's page says it holds two changes, open",
               next.contains("The next system holds 2 changes") && next.contains("It is open: not finished"));
        expect("it lists every change by its title and no more than recab says",
               next.contains(titles.value(0)) && next.contains(titles.value(1)) && !next.contains("Made by")
                   && !next.contains("Take out") && !next.contains("Edit a file") && !next.contains(" MiB"));
        expect("it offers what recab offers",
               next.contains("[Make a change…]") && next.contains("[Install software…]") && next.contains("[Finish…]")
                   && next.contains("[Reset to the running system…]") && next.contains("2. Choose when"));
        expect("it says what will differ, from the changes",
               next.contains("What will differ after the restart") && next.contains("Starts")
                   && next.contains("the next system, when you restart into it") && next.contains("Your files")
                   && next.contains("the same: home is shared by both systems")
                   && next.contains("comes back to the running system by itself")
                   && next.contains("See the two systems as blocks in Disk"));
        save(window, "p04-next-open");
        expect("the strip is not on the next system's page", !window.frame()->stripShown());
        backWords(window);
        expect("the strip is on Getting back", window.frame()->stripShown());
        save(window, "p05-back-with-a-next-system");
        const QString overview = overviewWords(window);
        expect("Overview offers finishing as the one thing and not the rest of the next system's page",
               overview.contains("[Finish…]") && !overview.contains("[Make a change…]")
                   && !overview.contains("[Reset") && !window.frame()->stripShown());
        save(window, "p06-overview-open");
        window.showMain();
        settle(100);
    }
    activityShots(window, folder, "a", int(window.status().changes.size()), save, expect, out);
    settingsShots(window, folder, save, expect, out);

    window.addUpdate(false);
    waitUntilIdle(window);
    expect("the update clashes with the edited file", !window.status().clashes.isEmpty());
    save(window, "07-clash");
    {
        const QString next = nextWords(window);
        expect("the next system's page offers to settle the clash and not to finish",
               next.contains("The update clashes with one of your changes") && next.contains("[Settle the clash…]")
                   && !next.contains("[Finish…]") && next.contains("Release"));
        window.showMain();
        settle(100);
    }

    window.fixWithAgent(QStringLiteral("The issue:\nThe update and a local change both altered /etc/ssh/sshd_config.\n\n"
                                       "The expected result:\nOne sshd_config that sshd accepts. Password logins stay "
                                       "off, as I set it. The update's own changes are kept.\n"),
                        QStringLiteral("The agent is settling the clash"));
    waitUntilIdle(window);
    expect("the agent leaves its result to be accepted", !window.status().proposalPath.isEmpty());
    save(window, "08-what-the-agent-did");

    window.acceptProposal();
    waitUntilIdle(window);
    expect("accepting settles the clash", window.status().clashes.isEmpty());
    window.finish();
    waitUntilIdle(window);
    expect("finishing makes it ready", window.status().state == "ready");
    save(window, "09-ready");
    {
        const QString next = nextWords(window);
        expect("the next system's page says it is ready, and offers the restart now or later",
               next.contains("The next system is ready") && next.contains("[Restart now]")
                   && next.contains("[Start it at the next restart]") && !next.contains("[Finish…]"));
        save(window, "p07-next-ready");
        const QString back = backWords(window);
        expect("Getting back has nothing to cancel", back.contains("None is set.") && back.contains("[Cancel it](off)"));
        const QString overview = overviewWords(window);
        expect("Overview offers the restart as the one thing", overview.contains("[Restart into it]"));
        window.showMain();
        settle(100);
    }

    window.restart();
    waitUntilIdle(window);
    expect("the first restart fails and falls back", window.status().state == "failed");
    save(window, "10-did-not-start");
    {
        const QString next = nextWords(window);
        expect("the next system's page says it did not start, and offers to try again or hand it to the agent",
               next.contains("The next system did not start, so the computer came back")
                   && next.contains("[Try the same again]") && next.contains("[Hand it to the agent…]"));
        save(window, "p08-next-came-back");
        const QString back = backWords(window);
        expect("Getting back says the computer came back, with the time recab gave",
               back.contains("Kept already: the computer came back to it on") && back.contains("did not start properly")
                   && back.contains("recab says"));
        save(window, "p09-back-came-back");
        const QString overview = overviewWords(window);
        expect("Overview hands it to the agent and does not offer the reset",
               overview.contains("[Let the agent fix it]") && !overview.contains("[Reset"));
        window.showMain();
        settle(100);
    }

    window.fixWithAgent(QStringLiteral("The issue:\nThe next system stopped during start-up.\n\nThe expected result:\n"
                                       "The next system starts.\n"),
                        QStringLiteral("The agent is changing the next system"));
    waitUntilIdle(window);
    expect("the agent's fix opens the next system again", window.status().state == "preparing");
    window.finish();
    waitUntilIdle(window);
    window.restart();
    waitUntilIdle(window);
    expect("the second restart is kept", window.status().state == "stable" && window.status().trialOutcome == "kept");
    save(window, "11-on-the-new-system");
    {
        const QString back = backWords(window);
        expect("Getting back says the new system is kept, and when it started properly",
               back.contains("Kept already: it started properly on") && back.contains("startup ended with nothing failed")
                   && back.contains("[Go back…]") && !back.contains("[Go back…](off)"));
        save(window, "p10-back-kept");
        const QString path = QDir(folder).filePath("p-steps-old.txt");
        QFile::remove(path);
        expect("the steps are written as a file", window.saveSteps(path));
        QFile file(path);
        const QString text = file.open(QIODevice::ReadOnly) ? QString::fromUtf8(file.readAll()) : QString();
        expect("the file names the steps, and no system by a name recab did not give",
               text.contains("1. Restart the computer") && text.contains("2. Pick the running system")
                   && text.contains("3. To make that choice stay, run \"recab commit\"") && !text.contains("Slot "));
        window.showMain();
        settle(100);
    }

    // Reading again by hand, as F5 does, leaves the page where it was.
    window.refresh();
    waitUntilIdle(window);
    expect("reading again keeps the state", window.status().state == "stable");

    // A change, then the reset: the next system is the running one again.
    window.handOver(asked(sshd), false);
    for (int waited = 0; !window.agentAsks() && window.busy() && waited < 200; ++waited)
        settle(50);
    window.answerAgent(QStringLiteral("Leave it"));
    waitUntilIdle(window);
    expect("a change is there to be reset", window.status().changes.size() == 1);
    window.reset(false);
    waitUntilIdle(window);
    expect("the reset leaves a next system without changes",
           window.status().otherRole == "next" && window.status().changes.isEmpty()
               && window.status().state == "preparing");
    save(window, "12-reset");
    {
        const QString next = nextWords(window);
        expect("after the reset the next system holds nothing",
               next.contains("The next system holds no changes yet") && next.contains("Nothing yet")
                   && window.frame()->badge("next") == 0);
        save(window, "p11-next-reset");
        window.showMain();
        settle(100);
    }

    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}

// The story the real recab offers today, against the stand-in that speaks its
// shape (`fixtures/recab-real`): two copies, a change by the agent, finishing,
// the restart, going back, and discarding. No update, clash or proposal:
// the real recab has none of those.
int takeRealShots(Window &window, const QString &folder)
{
    QDir().mkpath(folder);
    QTextStream out(stdout);
    bool ok = true;
    auto save = [&](QWidget &widget, const QString &name) {
        settle(200);
        ok &= widget.grab().save(QDir(folder).filePath(name + ".png"));
    };
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << " (state " << window.status().state << ", other "
                << window.status().otherRole << ")\n";
            ok = false;
        }
    };
    window.show();
    waitUntilIdle(window);
    expect("the status is read as the real recab's", window.status().real);
    if (!window.status().real) {
        // Nothing was read: show why, and go no further. The story below
        // hands requests to the agent, which must not happen on a system
        // that is not the stand-in's.
        save(window, "r0-not-read");
        out << "the system could not be read; stopped before anything was asked of the agent\n";
        return 1;
    }
    expect("it starts with two copies of one system",
           window.status().state == "stable" && window.status().otherRole == "copy");
    expect("the running system is named from os-release", window.status().name(window.status().runningRelease) == "LevitateOS 2026.11");
    save(window, "r1-two-copies");
    {
        const QString next = nextWords(window);
        expect("the next system's page says there is none, and that the other place holds a copy",
               next.contains("There is no next system yet") && next.contains("holds a copy of it"));
        const QString back = backWords(window);
        expect("Getting back says why there is nothing to go back to",
               back.contains("There is none: the other place holds a copy of the running system.")
                   && back.contains("[Go back…](off)") && back.contains("recab holds no record of one"));
        expect("the steps name the two systems as recab titles them, and the running one",
               back.contains("Pick \"LevitateOS (Slot A)\". That is the running system today, and it is kept.")
                   && back.contains("\"LevitateOS (Slot B)\" holds a copy of it.")
                   && back.contains("loader/entries/levitate-a.conf") && back.contains("loader/entries/levitate-b.conf"));
        save(window, "rp1-back-two-copies");
        const QString path = QDir(folder).filePath("rp-steps.txt");
        QFile::remove(path);
        expect("the steps are written as a file", window.saveSteps(path));
        QFile file(path);
        const QString text = file.open(QIODevice::ReadOnly) ? QString::fromUtf8(file.readAll()) : QString();
        expect("the file names the steps and the systems",
               text.contains("1. Restart the computer") && text.contains("2. Pick \"LevitateOS (Slot A)\"")
                   && text.contains("3. To make that choice stay, run \"recab commit\" as administrator")
                   && text.contains("\"LevitateOS (Slot B)\" holds a copy of it."));
        window.showMain();
        settle(100);
    }

    window.handOver(asked(sshd), false);
    waitUntilIdle(window);
    expect("the agent's change opens the next system",
           window.status().state == "preparing" && !window.status().preparationId.isEmpty()
               && window.status().changes.size() == 1);
    expect("the change says the agent made it", window.status().changeTitles().value(0).contains("by the agent"));
    save(window, "r2-the-next-system-is-open");
    {
        const QString next = nextWords(window);
        expect("the next system's page shows who made the change, and when",
               next.contains("Made by") && next.contains("The agent") && next.contains("When")
                   && next.contains("edited: /etc/ssh/sshd_config") && window.frame()->badge("next") == 1);
        window.showMain();
        settle(100);
    }
    activityShots(window, folder, "r2c", int(window.status().changes.size()), save, expect, out);
    {
        // The reader the other two apps use for their strip, against the same recab.
        NextSystem next(qEnvironmentVariable("RECAB_GUI_RECAB"));
        bool answered = false;
        QObject::connect(&next, &NextSystem::known, [&] { answered = true; });
        next.read();
        waitFor([&] { return answered; });
        expect("the next-system reader finds the one change the window knows",
               answered && next.titles().size() == window.status().changes.size()
                   && next.titles() == window.status().changeTitles());
    }

    // A change made by the person, as Recipe would make it, with the stand-in's own command.
    {
        const QString recab = qEnvironmentVariable("RECAB_GUI_RECAB");
        QProcess person;
        person.start(recab, {"--json", "change", "install", "htop", "--preparation", window.status().preparationId,
                             "--revision", QString::number(window.status().preparationRevision), "--actor", "person"});
        person.waitForFinished(20000);
        expect("the stand-in took a change from the person",
               person.exitStatus() == QProcess::NormalExit && person.exitCode() == 0);
        window.refresh();
        waitUntilIdle(window);
        const QString next = nextWords(window);
        expect("the badge counts both changes", window.frame()->badge("next") == 2 && window.status().changes.size() == 2);
        expect("the page says who made each, and when: the agent, and you",
               next.contains("The next system holds 2 changes") && next.contains("The agent") && next.contains("You")
                   && next.contains("htop, installed") && next.contains("added: htop") && next.contains("edited: /etc/ssh/sshd_config")
                   && !next.contains("Take out") && !next.contains(" MiB"));
        save(window, "rp2-next-open-with-who-and-when");
        const QString overview = overviewWords(window);
        expect("Overview offers finishing as the one thing, and sends to the two pages",
               overview.contains("[Finish…]") && !overview.contains("[Make a change…]") && !overview.contains("[Reset")
                   && overview.contains("The rest is on:"));
        save(window, "rp3-overview-open");
        window.showMain();
        settle(100);
    }

    // A command started in a terminal is running: nothing can be started here.
    qputenv("RECAB_STANDIN_WORKING", "1");
    window.refresh();
    waitUntilIdle(window);
    expect("a command running elsewhere is reported as working", window.status().state == "working");
    save(window, "r2b-a-command-runs-elsewhere");
    {
        const QString next = nextWords(window);
        expect("while a command runs elsewhere the next system's page offers nothing, and says so once",
               next.contains("The next system is being worked on") && !next.contains("[Make a change…]")
                   && !next.contains("[Finish…]") && !next.contains("[Reset")
                   && next.count("Nothing can be started while that command runs.") == 1);
        save(window, "rp4-next-working");
        const QString back = backWords(window);
        expect("Getting back offers nothing then", back.contains("[Go back…](off)") && back.contains("[Cancel it](off)"));
        window.showMain();
        settle(100);
    }
    qunsetenv("RECAB_STANDIN_WORKING");
    window.refresh();
    waitUntilIdle(window);
    expect("when it has finished the next system is open again", window.status().state == "preparing");

    window.finish();
    waitUntilIdle(window);
    expect("finishing makes it ready", window.status().state == "ready");
    save(window, "r3-ready");
    {
        const QString next = nextWords(window);
        expect("the next system's page says it is ready, and offers the restart now or later",
               next.contains("The next system is ready") && next.contains("[Restart now]")
                   && next.contains("[Start it at the next restart]") && next.contains("The agent")
                   && next.contains("You"));
        save(window, "rp5-next-ready");
        window.showMain();
        settle(100);
    }

    // Set for the next restart without restarting, and taken back.
    window.restartLater();
    waitUntilIdle(window);
    expect("the restart is set and nothing restarted",
           window.status().state == "trial_pending" && window.status().runningSlot == "A"
               && window.status().can("restart").ready);
    save(window, "r3b-starts-at-the-next-restart");
    {
        const QString next = nextWords(window);
        expect("the next system's page says the restart is set, and offers to restart or cancel it",
               next.contains("The next system starts at the next restart") && next.contains("[Restart now]")
                   && next.contains("[Cancel the set restart]") && next.contains("that restart is set"));
        save(window, "rp6-next-restart-set");
        const QString back = backWords(window);
        expect("Getting back offers to cancel the set restart",
               back.contains("A restart into the next system is set") && back.contains("[Cancel it]")
                   && !back.contains("[Cancel it](off)") && back.contains("[Go back…](off)"));
        save(window, "rp7-back-restart-set");
        window.showMain();
        settle(100);
    }
    window.cancelRestart();
    waitUntilIdle(window);
    expect("cancelling leaves the next system ready", window.status().state == "ready");
    {
        const QString back = backWords(window);
        expect("after cancelling, Getting back says none is set", back.contains("None is set.") && back.contains("[Cancel it](off)"));
        save(window, "rp8-back-restart-cancelled");
        window.showMain();
        settle(100);
    }
    window.restartLater();
    waitUntilIdle(window);

    // Start-up has not been judged yet when the window opens after the restart.
    qputenv("RECAB_STANDIN_KEEP", "hand");
    window.restart();
    waitUntilIdle(window);
    qunsetenv("RECAB_STANDIN_KEEP");
    expect("the restart lands on the other slot, on trial",
           window.status().state == "on_trial" && window.status().runningSlot == "B"
               && window.status().can("keep").ready);
    save(window, "r4-on-trial");
    {
        const QString next = nextWords(window);
        expect("the next system's page says this is the trial, and offers to keep it or go back",
               next.contains("You are on the next system, on trial") && next.contains("[Keep it]")
                   && next.contains("[Go back…]") && !next.contains("Everything in it"));
        save(window, "rp9-next-on-trial");
        const QString back = backWords(window);
        expect("Getting back offers keeping it, and names the system to pick if it does not start",
               back.contains("It started, and is not kept yet.") && back.contains("[Keep it]") && !back.contains("[Keep it](off)")
                   && back.contains("Pick \"LevitateOS (Slot A)\". That is the system from before. The running system, "
                                    "\"LevitateOS (Slot B)\", is on trial and is not kept."));
        save(window, "rp10-back-on-trial");
        window.showMain();
        settle(100);
    }
    window.keep();
    waitUntilIdle(window);
    expect("keeping it makes it the kept system",
           window.status().state == "stable" && window.status().runningSlot == "B"
               && window.status().trialOutcome == "kept" && window.status().otherRole == "previous");
    save(window, "r4b-on-the-new-system");
    {
        const QString back = backWords(window);
        expect("Getting back says it is kept, and when it started properly, in recab's words",
               back.contains("Kept already: it started properly on") && back.contains("kept by hand")
                   && back.contains("[Keep it](off)") && back.contains("[Go back…]") && !back.contains("[Go back…](off)"));
        save(window, "rp11-back-kept");
        window.showMain();
        settle(100);
    }

    window.goBack(false);
    waitUntilIdle(window);
    expect("going back lands on the first slot", window.status().runningSlot == "A");
    save(window, "r5-back");
    {
        const QString back = backWords(window);
        expect("after going back, the system kept is the one before, and the trial was of the other",
               back.contains("Kept already.") && back.contains("kept, but it was the other system that was tried")
                   && back.contains("Pick \"LevitateOS (Slot A)\". That is the running system today"));
        save(window, "rp12-back-after-going-back");
        window.showMain();
        settle(100);
    }

    window.handOver(asked(nvidia), false);
    waitUntilIdle(window);
    expect("a second change opens a next system again",
           window.status().state == "preparing" && window.status().changes.size() == 1);
    const QString before = window.status().preparationId;
    window.reset(false);
    waitUntilIdle(window);
    expect("the reset leaves a new, empty next system",
           window.status().state == "preparing" && window.status().changes.isEmpty()
               && !window.status().preparationId.isEmpty() && window.status().preparationId != before);
    save(window, "r6-reset");
    {
        const QString next = nextWords(window);
        expect("after the reset the next system holds nothing", next.contains("The next system holds no changes yet")
                                                                    && next.contains("Nothing yet") && window.frame()->badge("next") == 0);
        save(window, "rp13-next-reset");
        window.showMain();
        settle(100);
    }
    window.discard(false);
    waitUntilIdle(window);
    expect("discarding leaves no next system",
           window.status().state == "stable" && window.status().preparationId.isEmpty());
    save(window, "r7-discarded");

    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
