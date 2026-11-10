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
    if (!handOver)
        return ok ? 0 : 1;

    window.sendShown();
    waitUntilIdle(window, 300); // the agent, then the second look
    out << "the agent freed " << bytesText(window.freed()) << "; " << window.found().size() << " places of mess are left\n";
    ok &= window.freed() > 0;
    save("4-after");
    return ok ? 0 : 1;
}
