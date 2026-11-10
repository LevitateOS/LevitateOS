#include "shots.h"

#include "activity.h"
#include "frame.h"
#include "look.h"
#include "nextsystem.h"
#include "pages.h"
#include "window.h"

#include <QDir>
#include <QElapsedTimer>
#include <QFileInfo>
#include <QProcess>
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
           frame->badge(Frame::activityKey()) == 1 && frame->badgeWarns(Frame::activityKey()));

    frame->choose(Frame::activityKey());
    waitFor([&] { return page->listedTitles().size() == 4 && !page->detailText().isEmpty(); });
    expect(QStringLiteral("Activity is showing and marked"), frame->activityShown() && frame->selected() == Frame::activityKey());
    expect(QStringLiteral("Activity lists the four runs, newest first"),
           page->listedTitles()
               == QStringList({"Install the NVIDIA driver", "Update Element", "Turn off password logins over ssh",
                               "Install htop for this computer"}));
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
    expect(QStringLiteral("all apps again"), page->listedTitles().size() == 4);

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
    activityShots(window, folder, "a", int(window.status().changes.size()), save, expect, out);

    window.addUpdate(false);
    waitUntilIdle(window);
    expect("the update clashes with the edited file", !window.status().clashes.isEmpty());
    save(window, "07-clash");

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

    window.restart();
    waitUntilIdle(window);
    expect("the first restart fails and falls back", window.status().state == "failed");
    save(window, "10-did-not-start");

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

    window.handOver(asked(sshd), false);
    waitUntilIdle(window);
    expect("the agent's change opens the next system",
           window.status().state == "preparing" && !window.status().preparationId.isEmpty()
               && window.status().changes.size() == 1);
    expect("the change says the agent made it", window.status().changeTitles().value(0).contains("by the agent"));
    save(window, "r2-the-next-system-is-open");
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

    // A command started in a terminal is running: nothing can be started here.
    qputenv("RECAB_STANDIN_WORKING", "1");
    window.refresh();
    waitUntilIdle(window);
    expect("a command running elsewhere is reported as working", window.status().state == "working");
    save(window, "r2b-a-command-runs-elsewhere");
    qunsetenv("RECAB_STANDIN_WORKING");
    window.refresh();
    waitUntilIdle(window);
    expect("when it has finished the next system is open again", window.status().state == "preparing");

    window.finish();
    waitUntilIdle(window);
    expect("finishing makes it ready", window.status().state == "ready");
    save(window, "r3-ready");

    // Set for the next restart without restarting, and taken back.
    window.restartLater();
    waitUntilIdle(window);
    expect("the restart is set and nothing restarted",
           window.status().state == "trial_pending" && window.status().runningSlot == "A"
               && window.status().can("restart").ready);
    save(window, "r3b-starts-at-the-next-restart");
    window.cancelRestart();
    waitUntilIdle(window);
    expect("cancelling leaves the next system ready", window.status().state == "ready");
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
    window.keep();
    waitUntilIdle(window);
    expect("keeping it makes it the kept system",
           window.status().state == "stable" && window.status().runningSlot == "B"
               && window.status().trialOutcome == "kept" && window.status().otherRole == "previous");
    save(window, "r4b-on-the-new-system");

    window.goBack(false);
    waitUntilIdle(window);
    expect("going back lands on the first slot", window.status().runningSlot == "A");
    save(window, "r5-back");

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
    window.discard(false);
    waitUntilIdle(window);
    expect("discarding leaves no next system",
           window.status().state == "stable" && window.status().preparationId.isEmpty());
    save(window, "r7-discarded");

    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
