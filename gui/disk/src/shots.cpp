#include "shots.h"

#include "activity.h"
#include "frame.h"
#include "look.h"
#include "window.h"
#include "words.h"

#include <QAbstractButton>
#include <QDir>
#include <QElapsedTimer>
#include <QFile>
#include <QFileInfo>
#include <QProcess>
#include <QPushButton>
#include <QTextStream>

#include <functional>

static void waitUntilIdle(Window &window, int seconds)
{
    QElapsedTimer clock;
    clock.start();
    settle(100);
    while (window.busy() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(200);
}

namespace {

using Save = std::function<void(const QString &)>;
using Expect = std::function<void(const QString &, bool)>;

void waitFor(const std::function<bool()> &holds, int seconds = 20)
{
    QElapsedTimer clock;
    clock.start();
    while (!holds() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(150);
}

// What the rail offers, in order, without the badges: the checkable buttons
// are the app's entries and Activity.
QStringList railEntries(Frame *frame)
{
    QStringList names;
    for (QAbstractButton *button : frame->findChildren<QAbstractButton *>())
        if (button->isCheckable() && button->isVisible())
            names << button->text();
    return names;
}

QPushButton *buttonNamed(QWidget *inside, const QString &words)
{
    for (QPushButton *button : inside->findChildren<QPushButton *>())
        if (button->text() == words && button->isVisible())
            return button;
    return nullptr;
}

// The strip: it is read with a recab of the real shape, named by
// DISK_STANDIN_STRIP_RECAB (gui/recab/fixtures/recab-real, with its own state,
// after one `prepare` and one change). The comparison keeps using the
// window's own recab, the Disk stand-in, which answers `status` in an older
// shape the strip cannot read.
void stripShots(Window &window, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString real = qEnvironmentVariable("DISK_STANDIN_STRIP_RECAB");
    const QString mark = qEnvironmentVariable("DISK_STANDIN_SYSTEM_MARK");
    if (real.isEmpty() || mark.isEmpty()) {
        out << "not as expected: the strip needs DISK_STANDIN_STRIP_RECAB (gui/recab/fixtures/recab-real after one "
               "prepare and one change) and DISK_STANDIN_SYSTEM_MARK (a file the stand-in System app makes), see README.md\n";
        expect(QStringLiteral("the strip could be checked"), false);
        return;
    }
    frame->choose(QStringLiteral("mess"));
    window.readStripFrom(real);
    waitFor([&] { return frame->stripShown(); }, 10);
    expect(QStringLiteral("the strip says what the next system holds (%1)").arg(frame->stripText()),
           frame->stripShown() && frame->stripText() == QStringLiteral("The next system holds 1 change."));
    save("s1-strip-mess");
    out << "the strip: " << frame->stripText() << "\n";

    // On every page of the window.
    frame->choose(QStringLiteral("systems"));
    settle(100);
    expect(QStringLiteral("the strip shows on the two systems too"), frame->stripShown());
    save("s2-strip-systems");

    // "Finish and restart…" and "Open System…" start the System app the same way.
    QFile::remove(mark);
    if (QPushButton *finish = buttonNamed(frame, QStringLiteral("Finish and restart…")))
        finish->click();
    waitFor([&] { return QFileInfo::exists(mark); }, 10);
    expect(QStringLiteral("the strip's button starts the System app"), QFileInfo::exists(mark));
    QFile::remove(mark);
    if (QPushButton *open = buttonNamed(&window, QStringLiteral("Open System…")))
        open->click();
    waitFor([&] { return QFileInfo::exists(mark); }, 10);
    expect(QStringLiteral("\"Open System…\" starts it the same way"), QFileInfo::exists(mark));
    QFile::remove(mark);
}

// Where the strip must not be: a recab that cannot be read, and the older
// shape of the Disk stand-in. Neither says anything about it.
void noStripShots(Window &window, const QString &own, const Expect &expect)
{
    Frame *frame = window.frame();
    window.readStripFrom(QStringLiteral("/nonexistent/recab"));
    settle(600);
    expect(QStringLiteral("no strip when recab cannot be started"), !frame->stripShown());
    // Put the real one back first, so that going away is seen.
    const QString real = qEnvironmentVariable("DISK_STANDIN_STRIP_RECAB");
    window.readStripFrom(real);
    waitFor([&] { return frame->stripShown(); }, 10);
    window.readStripFrom(own);
    settle(600);
    expect(QStringLiteral("no strip when recab answers in a shape the strip cannot read"), !frame->stripShown());
}

// Activity with records of the agent's runs (gui/agent/fixtures/records, a
// stand-in Codex, never a real agent), read through the built levitate-agent.
// Disk has started none of these runs: they are Recipe's and System's.
// `own`: the work page shows the live run, which is then this window's own.
void activityShots(Window &window, const QString &folder, bool own, const Save &save, const Expect &expect,
                   QTextStream &out, QString *where)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        out << "not as expected: Activity needs RECAB_GUI_RECORDS (gui/agent/fixtures/records) and "
               "LEVITATE_ACTIVITY_AGENT (the built levitate-agent), see README.md\n";
        expect(QStringLiteral("the records for Activity could be made"), false);
        return;
    }
    *where = QDir(folder).filePath(own ? QStringLiteral("records-own") : QStringLiteral("records"));
    QProcess make;
    make.start(script, {*where, agent});
    make.waitForFinished(60000);
    const QString said = QString::fromUtf8(make.readAllStandardOutput()).trimmed();
    expect(QStringLiteral("the records for Activity were made (%1)").arg(QString::fromUtf8(make.readAllStandardError()).trimmed()),
           make.exitStatus() == QProcess::NormalExit && make.exitCode() == 0 && said.startsWith("state="));
    if (!said.startsWith("state="))
        return;
    qputenv("XDG_STATE_HOME", said.section(' ', 0, 0).mid(6).toUtf8());
    const qint64 waitingPid = said.section("waiting_pid=", 1).toLongLong();
    ActivityPage *page = frame->activity();

    frame->readAgain();
    waitFor([&] { return page->loaded() && frame->badge(Frame::activityKey()) > 0; });
    expect(QStringLiteral("the Activity entry counts the one run that waits"),
           frame->badge(Frame::activityKey()) == 1 && frame->badgeWarns(Frame::activityKey()));
    frame->choose(Frame::activityKey());
    waitFor([&] { return page->listedTitles().size() == 4 && !page->detailText().isEmpty(); });
    expect(QStringLiteral("Activity is showing and marked"),
           frame->activityShown() && frame->selected() == Frame::activityKey());
    expect(QStringLiteral("Activity lists the four runs"), page->listedTitles().size() == 4);
    expect(QStringLiteral("the strip shows on Activity"), frame->stripShown());
    save(own ? QStringLiteral("a2-activity-while-the-agent-works") : QStringLiteral("a1-activity"));

    page->showOnlyThisApp(true);
    expect(QStringLiteral("Disk started none of the runs, so its own filter lists none"), page->listedTitles().isEmpty());
    page->showOnlyThisApp(false);

    if (!own)
        return;
    // The agent this window waits for: Activity offers to go to it.
    page->chooseTitle("Turn off password logins over ssh");
    waitFor([&] { return page->detailText().contains("It asks"); });
    page->setOwnRun(waitingPid);
    waitFor([&] { return page->offersGoTo(); });
    expect(QStringLiteral("this window's own live run offers Go to it"), page->offersGoTo());
    if (QPushButton *go = buttonNamed(page, QStringLiteral("Go to it")))
        go->click();
    settle(200);
    expect(QStringLiteral("Go to it leaves Activity for the work page, under the Mess entry"),
           !frame->activityShown() && frame->selected() == "mess" && window.showingWork());
    page->setOwnRun(0);
}

} // namespace

int takeShots(Window &window, const QString &folder, bool handOver)
{
    QDir().mkpath(folder);
    QTextStream out(stdout);
    bool ok = true;
    auto save = [&](const QString &name) {
        settle(200);
        ok &= window.grab().save(QDir(folder).filePath(name + ".png"));
    };
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << "\n";
            ok = false;
        }
    };
    const QString ownRecab = qEnvironmentVariable("DISK_GUI_RECAB", "recab");
    QString records;
    window.show();
    waitUntilIdle(window, 600);
    qint64 mess = 0;
    for (const Finding &finding : window.found())
        mess += finding.bytes;
    out << "looked at " << bytesText(window.scannedBytes()) << ", " << window.found().size() << " places of mess, "
        << bytesText(mess) << "\n";
    for (const Finding &finding : window.found())
        out << "  " << bytesText(finding.bytes) << "  " << finding.what << "  " << finding.path << "\n";
    Frame *frame = window.frame();
    expect(QStringLiteral("the window is 1100 x 700"), window.width() == 1100 && window.height() == 700);
    expect(QStringLiteral("the rail offers Mess, The two systems and Activity, nothing else (%1)")
               .arg(railEntries(frame).join(QStringLiteral(", "))),
           railEntries(frame) == QStringList({"Mess", "The two systems", "Activity"}));
    expect(QStringLiteral("the Mess badge is the number of places listed (%1 and %2)")
               .arg(frame->badge("mess")).arg(window.found().size()),
           frame->badge("mess") == window.found().size() && window.found().size() > 0);
    expect(QStringLiteral("Mess is marked and shows"), frame->selected() == "mess" && window.showingMess());
    expect(QStringLiteral("the status line says what the window does"),
           frame->statusText() == QStringLiteral("This window only reads the disk"));
    expect(QStringLiteral("the status line names the agent (%1)").arg(frame->agentText()),
           frame->agentText().contains(QFileInfo(qEnvironmentVariable("DISK_GUI_AGENT")).fileName()));
    expect(QStringLiteral("no strip while recab's status cannot be read as the real one"), !frame->stripShown());
    save("1-mess");

    const QString picked = window.pickBiggestUnnamed();
    if (!picked.isEmpty())
        out << "picked by hand: " << picked << "\n";
    save("2-picked");

    window.showHandOver();
    expect(QStringLiteral("the hand-over belongs to Mess"), frame->selected() == "mess");
    save("3-hand-over");
    stripShots(window, save, expect, out);
    window.showHandOver();
    settle(100);
    expect(QStringLiteral("the strip shows on the hand-over too"), frame->stripShown());
    if (!handOver) {
        // What the entry opens with: the kept comparison when there is one,
        // else what recab says is there.
        frame->choose(QStringLiteral("systems"));
        waitUntilIdle(window, 30);
        expect(QStringLiteral("The two systems is marked and shows"),
               frame->selected() == "systems" && window.showingSystems());
        save("4-systems");
        out << "the two systems: " << (window.comparison() ? "a kept comparison is shown" : "not compared")
            << (window.stale() ? ", marked as out of date" : "") << "\n";
        activityShots(window, folder, false, save, expect, out, &records);
        frame->choose(QStringLiteral("mess"));
        expect(QStringLiteral("Mess is marked and shows again"), frame->selected() == "mess" && window.showingMess());
        noStripShots(window, ownRecab, expect);
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        out << (ok ? "all as expected\n" : "some things were not as expected\n");
        return ok ? 0 : 1;
    }

    window.sendShown();
    // The agent asks before it removes what is marked for that: yes to the
    // first, no to the rest.
    int asked = 0;
    for (QElapsedTimer clock = [] { QElapsedTimer t; t.start(); return t; }(); window.busy() && clock.elapsed() < 300000;) {
        settle(50);
        if (!window.agentAsks())
            continue;
        if (!asked) {
            save("3b-the-agent-asks");
            expect(QStringLiteral("while the agent works the Mess entry is marked"), frame->selected() == "mess");
            activityShots(window, folder, true, save, expect, out, &records);
            expect(QStringLiteral("the agent still asks on the work page"), window.agentAsks());
        }
        window.answerAgent(asked++ ? QStringLiteral("Keep it") : QStringLiteral("Remove it"));
    }
    settle(200);
    out << "the agent asked " << asked << " times\n";
    ok &= asked > 0;
    waitUntilIdle(window, 300); // the second look
    out << "the agent freed " << bytesText(window.freed()) << "; " << window.found().size() << " places of mess are left\n";
    ok &= window.freed() > 0;
    expect(QStringLiteral("the badge follows the second look (%1 and %2)").arg(frame->badge("mess")).arg(window.found().size()),
           frame->badge("mess") == window.found().size());
    expect(QStringLiteral("back on Mess, marked"), frame->selected() == "mess" && window.showingMess());
    save("4-after");

    // The two systems: compare, then ask the agent what differs.
    frame->choose(QStringLiteral("systems"));
    waitUntilIdle(window, 30);
    ok &= window.state().known;
    expect(QStringLiteral("The two systems is marked and shows"), frame->selected() == "systems" && window.showingSystems());
    save("5-systems");
    // Slowly once, to see the two sides fill while they are read.
    const QByteArray speed = qgetenv("DISK_STANDIN_SPEED");
    qputenv("DISK_STANDIN_SPEED", "1");
    window.compare();
    settle(3200);
    ok &= window.comparing();
    expect(QStringLiteral("while comparing, the agent's place says so (%1)").arg(frame->agentText()),
           frame->agentText().contains(QStringLiteral("Comparing")));
    save("5b-comparing");
    qputenv("DISK_STANDIN_SPEED", speed);
    waitUntilIdle(window, 300);
    const Comparison *compared = window.comparison();
    if (!compared) {
        out << "the two systems could not be compared\n";
        save("6-compared");
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        return 1;
    }
    out << "the two systems: " << compared->count() << " differences, " << bytesText(compared->differingBytes())
        << "; " << compared->sameFiles << " files the same\n";
    expect(QStringLiteral("the two maps and the list fit the window without making it wider (%1 x %2)")
               .arg(window.width()).arg(window.height()),
           window.width() == 1100 && window.height() == 700);
    save("6-compared");
    if (compared->identical()) {
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        return ok ? 0 : 1;
    }
    window.showExplain();
    save("7-ask");
    window.sendShown();
    waitUntilIdle(window, 300);
    out << "the agent said: " << window.said() << "\n";
    ok &= !window.said().isEmpty();
    expect(QStringLiteral("after the answer The two systems shows"), frame->selected() == "systems" && window.showingSystems());
    save("8-explained");
    noStripShots(window, ownRecab, expect);
    if (!records.isEmpty())
        QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
    out << (ok ? "all as expected\n" : "some things were not as expected\n");
    return ok ? 0 : 1;
}
