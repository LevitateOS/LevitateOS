#include "shots.h"

#include "look.h"
#include "window.h"
#include "words.h"

#include <QDir>
#include <QElapsedTimer>
#include <QTextStream>

static void waitUntilIdle(Window &window, int seconds)
{
    QElapsedTimer clock;
    clock.start();
    settle(100);
    while (window.busy() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(200);
}

int takeShots(Window &window, const QString &folder, bool handOver)
{
    QDir().mkpath(folder);
    QTextStream out(stdout);
    bool ok = true;
    auto save = [&](const QString &name) {
        settle(200);
        ok &= window.grab().save(QDir(folder).filePath(name + ".png"));
    };
    window.show();
    waitUntilIdle(window, 600);
    qint64 mess = 0;
    for (const Finding &finding : window.found())
        mess += finding.bytes;
    out << "looked at " << bytesText(window.scannedBytes()) << ", " << window.found().size() << " places of mess, "
        << bytesText(mess) << "\n";
    for (const Finding &finding : window.found())
        out << "  " << bytesText(finding.bytes) << "  " << finding.what << "  " << finding.path << "\n";
    save("1-mess");

    const QString picked = window.pickBiggestUnnamed();
    if (!picked.isEmpty())
        out << "picked by hand: " << picked << "\n";
    save("2-picked");

    window.showHandOver();
    save("3-hand-over");
    if (!handOver) {
        // What the tab opens with: the kept comparison when there is one,
        // else what recab says is there.
        window.showSystems();
        save("4-systems");
        out << "the two systems: " << (window.comparison() ? "a kept comparison is shown" : "not compared")
            << (window.stale() ? ", marked as out of date" : "") << "\n";
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
        if (!asked)
            save("3b-the-agent-asks");
        window.answerAgent(asked++ ? QStringLiteral("Keep it") : QStringLiteral("Remove it"));
    }
    settle(200);
    out << "the agent asked " << asked << " times\n";
    ok &= asked > 0;
    waitUntilIdle(window, 300); // the second look
    out << "the agent freed " << bytesText(window.freed()) << "; " << window.found().size() << " places of mess are left\n";
    ok &= window.freed() > 0;
    save("4-after");

    // The two systems: compare, then ask the agent what differs.
    window.showSystems();
    waitUntilIdle(window, 30);
    ok &= window.state().known;
    save("5-systems");
    // Slowly once, to see the two sides fill while they are read.
    const QByteArray speed = qgetenv("DISK_STANDIN_SPEED");
    qputenv("DISK_STANDIN_SPEED", "1");
    window.compare();
    settle(3200);
    ok &= window.comparing();
    save("5b-comparing");
    qputenv("DISK_STANDIN_SPEED", speed);
    waitUntilIdle(window, 300);
    const Comparison *compared = window.comparison();
    if (!compared) {
        out << "the two systems could not be compared\n";
        save("6-compared");
        return 1;
    }
    out << "the two systems: " << compared->count() << " differences, " << bytesText(compared->differingBytes())
        << "; " << compared->sameFiles << " files the same\n";
    save("6-compared");
    if (compared->identical())
        return ok ? 0 : 1;
    window.showExplain();
    save("7-ask");
    window.sendShown();
    waitUntilIdle(window, 300);
    out << "the agent said: " << window.said() << "\n";
    ok &= !window.said().isEmpty();
    save("8-explained");
    return ok ? 0 : 1;
}
