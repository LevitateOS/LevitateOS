#include "shots.h"

#include "look.h"

#include "pages.h"
#include "window.h"

#include <QApplication>
#include <QDir>
#include <QElapsedTimer>
#include <QEventLoop>
#include <QPalette>
#include <QStyleFactory>
#include <QTextStream>
#include <QTimer>

static void waitUntilIdle(Window &window, int seconds = 10)
{
    QElapsedTimer clock;
    clock.start();
    while (window.busy() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(150);
}

int tryRecipe(Window &window, const QString &name, const QString &folder, bool keep, bool readOnly)
{
    QDir().mkpath(folder);
    auto save = [&folder](QWidget &widget, const QString &title) {
        settle(200);
        return widget.grab().save(QDir(folder).filePath(title + ".png"));
    };
    window.show();
    waitUntilIdle(window, 60);
    bool ok = window.catalog().recipe(name) != nullptr;
    window.showBrowse();
    ok &= save(window, "real-1-browse");
    window.showRecipe(name);
    ok &= save(window, "real-2-recipe");
    if (readOnly) {
        window.showInstalled(name);
        ok &= save(window, "real-5-installed");
        // What `recipe updates` and `recipe storage` said, for the one who runs this.
        QTextStream out(stdout);
        for (const UpdateItem &update : window.catalog().updates)
            out << "update: " << update.name << " " << update.from << " -> " << update.to << ", "
                << (update.changesKnown ? QString::number(update.changes.size()) + " changes in what it may use"
                                        : QStringLiteral("not compared"))
                << "\n";
        for (const QString &problem : window.catalog().updateProblems)
            out << "not looked up: " << problem << "\n";
        out << "installed takes " << window.catalog().activeBytes << " bytes; earlier versions "
            << window.catalog().earlierBytes << " bytes in " << window.catalog().earlierGenerations << "\n";
        if (!window.catalog().updates.isEmpty()) {
            window.showUpdates(name);
            ok &= save(window, "real-7-updates");
        }
        return ok ? 0 : 1;
    }

    window.startInstall(name);
    settle(20000);
    if (window.busy())
        ok &= save(window, "real-3-running");
    waitUntilIdle(window, 900);
    ok &= save(window, "real-4-after");
    const bool installed = window.catalog().installedItem(name) != nullptr;
    window.showInstalled(name);
    ok &= save(window, "real-5-installed");
    if (installed && !keep) {
        window.removeNow(name);
        waitUntilIdle(window, 120);
        window.showInstalled();
        ok &= save(window, "real-6-removed");
        ok &= window.catalog().installedItem(name) == nullptr;
    }
    return ok && installed ? 0 : 1;
}

int takeShots(Window &window, const QString &folder)
{
    QDir().mkpath(folder);
    auto save = [&folder](QWidget &widget, const QString &name) {
        settle(200);
        return widget.grab().save(QDir(folder).filePath(name + ".png"));
    };
    window.show();
    // Before recipe has answered: what a person sees while the catalog is read.
    bool ok = window.grab().save(QDir(folder).filePath("0-reading.png"));
    waitUntilIdle(window);
    ok &= !window.catalog().recipes.isEmpty();

    window.showBrowse();
    ok &= save(window, "1-browse");

    window.showRecipe("htop");
    ok &= save(window, "2-recipe");

    qputenv("RECIPE_STANDIN_SPEED", "0");
    qputenv("RECIPE_STANDIN_STOP_AT", "htop:build");
    window.startInstall("htop");
    settle(1500);
    ok &= save(window, "3-installing");
    window.stopRun();
    waitUntilIdle(window);

    qunsetenv("RECIPE_STANDIN_STOP_AT");
    window.startInstall("element");
    waitUntilIdle(window);
    ok &= save(window, "4-stopped");

    window.showInstalled("gimp");
    ok &= save(window, "5-installed");

    const Recipe *recipe = window.catalog().recipe("gimp");
    const InstalledItem *item = window.catalog().installedItem("gimp");
    if (recipe && item) {
        RemoveDialog dialog(*recipe, *item, &window);
        dialog.show();
        ok &= save(dialog, "6-remove");
        dialog.close();
    }

    window.showUpdates("element");
    ok &= save(window, "7-updates");
    {
        QTextStream said(stdout);
        auto hold = [&](const QString &what, bool holds) {
            if (!holds) {
                said << "not as expected: " << what << "\n";
                ok = false;
            }
        };
        const UpdateItem *update = window.catalog().update("element");
        hold("the update of Element says what the newer recipe may use that the installed one may not",
             update && update->changesKnown && update->asksMore() && !update->changes.isEmpty()
                 && update->changes.first().added() == QStringList{"static.element.io:443"});

        // An earlier version: gone back to, and the one left behind is kept instead.
        const QString now = window.catalog().installedItem("gimp")->version;
        const QString earlier = window.catalog().installedItem("gimp")->kept;
        hold("an earlier version of GIMP is kept", !earlier.isEmpty() && window.catalog().earlierGenerations > 0);
        window.showInstalled("gimp");
        window.goBack("gimp");
        waitUntilIdle(window);
        hold("going back activates the earlier version and keeps the one left",
             window.catalog().installedItem("gimp")->version == earlier
                 && window.catalog().installedItem("gimp")->kept == now);
        ok &= save(window, "7b-gone-back");
        window.goBack("gimp");
        waitUntilIdle(window);
        hold("going back again returns", window.catalog().installedItem("gimp")->version == now);
    }

    window.startInstall("element", "The issue:\nThe update stopped.\n\nThe expected result:\nElement updates.\n");
    waitUntilIdle(window);
    ok &= save(window, "8-agent-fixed");

    // Sources: the page, a second source added, its list fetched again, only its recipes shown.
    QTextStream out(stdout);
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << "\n";
            ok = false;
        }
    };
    window.addSource("https://github.com/acme/recipes");
    waitUntilIdle(window);
    expect("an added source is listed", window.catalog().origin("acme") != nullptr);
    expect("its recipes are offered", window.catalog().recipe("zellij") != nullptr);
    window.fetchSources("acme");
    waitUntilIdle(window);
    window.showSources();
    ok &= save(window, "9-sources");
    window.chooseSource("acme");
    window.showBrowse();
    ok &= save(window, "10-one-source");
    window.chooseSource(QString());

    // Asking for software no source has: the agent asks back, then writes the recipe.
    window.showWish("a file manager for the terminal");
    ok &= save(window, "11-ask-for-software");
    window.askAgent(QStringLiteral("The request:\na file manager for the terminal\n\nThe issue:\nNo recipe for it "
                                   "exists.\n\nThe expected result:\nOne recipe that recipe info accepts.\n"));
    for (int waited = 0; !window.agentAsks() && window.busy() && waited < 200; ++waited)
        settle(50);
    expect("the agent asks which one", window.agentAsks());
    ok &= save(window, "12-the-agent-asks");
    window.answerAgent(QStringLiteral("Yazi"));
    waitUntilIdle(window);
    expect("the recipe the agent wrote is listed", window.catalog().recipe("yazi") != nullptr);
    expect("it is not installed", window.catalog().installedItem("yazi") == nullptr);
    ok &= save(window, "13-the-recipe-it-wrote");

    window.removeSource("acme", false);
    waitUntilIdle(window);
    expect("a removed source is gone", window.catalog().origin("acme") == nullptr);
    window.refresh();
    waitUntilIdle(window);

    // The earlier versions are cleared; what is installed stays.
    const qsizetype installedBefore = window.catalog().installed.size();
    window.showInstalled("gimp");
    expect("earlier versions take room before they are cleared", window.catalog().earlierBytes > 0);
    window.clearEarlier(false);
    waitUntilIdle(window);
    expect("clearing leaves no earlier version and everything installed",
           window.catalog().earlierGenerations == 0 && window.catalog().installed.size() == installedBefore
               && window.catalog().installedItem("gimp")->kept.isEmpty());
    ok &= save(window, "14-earlier-versions-cleared");
    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
