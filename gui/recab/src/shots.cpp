#include "shots.h"

#include "look.h"
#include "pages.h"
#include "window.h"

#include <QDir>
#include <QElapsedTimer>
#include <QTextStream>

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
// shape (`fixtures/recab6`): two copies, a change by the agent, finishing,
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
    save(window, "r2-the-next-system-is-open");

    window.finish();
    waitUntilIdle(window);
    expect("finishing makes it ready", window.status().state == "ready");
    save(window, "r3-ready");

    window.restart();
    waitUntilIdle(window);
    expect("the restart lands on the other slot and is kept",
           window.status().state == "stable" && window.status().runningSlot == "B"
               && window.status().trialOutcome == "kept" && window.status().otherRole == "previous");
    save(window, "r4-on-the-new-system");

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
