#include "shots.h"

#include "activity.h"
#include "frame.h"
#include "look.h"
#include "nextsystem.h"

#include "pages.h"
#include "window.h"

#include <QAbstractButton>
#include <QApplication>
#include <QComboBox>
#include <QDir>
#include <QElapsedTimer>
#include <QEventLoop>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLineEdit>
#include <QProcess>
#include <QPushButton>
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


namespace {

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

// What the window is as a frame: the rail, the pages in it and what the
// Browse page's search and kinds still do. Pictures `f*`.
void frameShots(Window &window, const Save &save, const Expect &expect)
{
    Frame *frame = window.frame();
    for (const char *title : {"Browse", "Installed", "Updates", "Sources", "Activity"})
        expect(QStringLiteral("the rail has %1").arg(title), frame->entryTitles().contains(QString::fromLatin1(title)));
    for (const char *title : {"Storage", "Settings", "Help"})
        expect(QStringLiteral("the rail has no %1: that page does not exist").arg(title),
               !frame->entryTitles().contains(QString::fromLatin1(title)));
    expect(QStringLiteral("the window is 1100 by 700"), window.size() == QSize(1100, 700));

    window.chooseSource(QString());
    window.browsePage()->searchBox()->clear();
    window.showBrowse();
    expect(QStringLiteral("Browse is marked in the rail"), frame->selected() == "browse");
    const qsizetype all = window.browsePage()->listed().size();
    int listed = 0;
    for (const Recipe &recipe : window.catalog().recipes)
        listed += recipe.listed();
    expect(QStringLiteral("Browse lists every listed recipe (%1)").arg(listed), all == listed && all > 3);
    save(window, "f1-browse");

    // Search still filters, and says so in the header.
    window.browsePage()->searchBox()->setText("htop");
    settle(100);
    expect(QStringLiteral("a search lists htop and fewer than all"),
           window.browsePage()->listed().contains("htop") && window.browsePage()->listed().size() < all);
    save(window, "f1b-browse-search");
    window.browsePage()->searchBox()->setText("zzzz-nothing");
    settle(100);
    expect(QStringLiteral("a search that matches nothing lists nothing"), window.browsePage()->listed().isEmpty());
    window.browsePage()->searchBox()->clear();
    settle(100);
    expect(QStringLiteral("clearing the search lists all again"), window.browsePage()->listed().size() == all);

    // A kind still filters.
    const QStringList kinds = window.catalog().categories();
    expect(QStringLiteral("the catalog has kinds"), kinds.size() >= 2);
    if (kinds.size() >= 2) {
        const QString kind = kinds.first();
        QStringList want;
        for (const Recipe &recipe : window.catalog().recipes)
            if (recipe.listed() && recipe.category == kind)
                want << recipe.name;
        window.showBrowse(kind);
        expect(QStringLiteral("a kind lists exactly the recipes of that kind (%1)").arg(kind),
               window.browsePage()->listed() == want && want.size() < all);
        expect(QStringLiteral("the kinds box shows it"),
               window.browsePage()->categoryBox()->currentData().toString() == kind);
        save(window, "f1c-browse-kind");
        window.showBrowse();
        expect(QStringLiteral("all kinds again"), window.browsePage()->listed().size() == all);
    }

    window.showInstalled("gimp");
    expect(QStringLiteral("Installed is marked in the rail"), frame->selected() == "installed");
    save(window, "f2-installed");

    window.showUpdates("element");
    expect(QStringLiteral("Updates is marked in the rail"), frame->selected() == "updates");
    expect(QStringLiteral("the Updates badge is the number of updates (%1)").arg(window.catalog().updates.size()),
           window.catalog().updates.size() > 0 && frame->badge("updates") == window.catalog().updates.size()
               && !frame->badgeWarns("updates"));
    save(window, "f3-updates");

    window.showSources();
    expect(QStringLiteral("Sources is marked in the rail"), frame->selected() == "sources");
    save(window, "f4-sources");

    // The clicks: choosing an entry shows its page.
    frame->choose(QStringLiteral("installed"));
    expect(QStringLiteral("choosing Installed shows it"), frame->selected() == "installed");
    frame->choose(QStringLiteral("browse"));
    expect(QStringLiteral("choosing Browse shows the list"), frame->selected() == "browse" && !window.browsePage()->listed().isEmpty());

    const QString status = frame->statusText();
    expect(QStringLiteral("the status line says where the recipes are from and when (%1)").arg(status),
           status.contains(" read at ") && status.contains("source"));
    expect(QStringLiteral("the status line names the agent (%1)").arg(frame->agentText()),
           frame->agentText() == QStringLiteral("Agent: %1").arg(QFileInfo(qEnvironmentVariable("RECIPE_GUI_AGENT")).fileName()));
}

// The pages of the frame that are not Recipe's: Activity. It needs runs to
// show, which `gui/agent/fixtures/records` makes by driving the built
// levitate-agent with the stand-in Codex. Pictures `a-*`.
void activityShots(Window &window, const QString &folder, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECIPE_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        out << "not as expected: Activity needs RECIPE_GUI_RECORDS (gui/agent/fixtures/records) and "
               "LEVITATE_ACTIVITY_AGENT (the built levitate-agent), see README.md\n";
        expect(QStringLiteral("the records for Activity could be made"), false);
        return;
    }
    const QString where = QDir(folder).filePath("a-records");
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
    save(window, "a-activity-list");

    page->chooseTitle("Install htop for this computer");
    waitFor([&] { return page->detailText().contains("ls -l"); });
    expect(QStringLiteral("a done run shows its commands and the agent's last word"),
           page->detailText().contains("ls -l") && page->detailText().contains("I made the change. Everything checks out."));
    save(window, "a-a-done-run");

    // The run that waits is this window's own: Go to it leaves Activity for the work page.
    page->chooseTitle("Turn off password logins over ssh");
    waitFor([&] { return page->detailText().contains("It asks"); });
    frame->setOwnRun(waitingPid);
    expect(QStringLiteral("the frame knows the pid of this window's own run"), frame->ownRun() == waitingPid && waitingPid > 0);
    waitFor([&] { return page->offersGoTo(); });
    expect(QStringLiteral("this window's own live run offers Go to it"),
           page->offersGoTo() && page->detailText().contains("It waits in this window"));
    save(window, "a-this-windows-own-run");
    clickButton(page, QStringLiteral("Go to it"));
    settle(200);
    expect(QStringLiteral("Go to it leaves Activity for Recipe's own pages"),
           !frame->activityShown() && frame->selected() == "browse");
    frame->choose(Frame::activityKey());
    frame->setOwnRun(0);
    waitFor([&] { return page->loaded() && !page->offersGoTo(); });

    page->showOnlyThisApp(true);
    expect(QStringLiteral("only Recipe's runs are listed"),
           page->listedTitles() == QStringList({"Update Element", "Install htop for this computer", "Install Krita for this computer"}));
    save(window, "a-only-recipe");
    page->showOnlyThisApp(false);

    expect(QStringLiteral("the strip shows on Activity while the next system holds a change"),
           frame->stripShown() && frame->stripText() == QStringLiteral("The next system holds 1 change."));
    save(window, "a-activity-with-the-strip");

    // The agent in the status line, asked of levitate-agent with the stand-in Codex.
    const QString codex = QDir(QFileInfo(script).absolutePath()).filePath("codex");
    qputenv("LEVITATE_AGENT_CODEX", codex.toUtf8());
    const QString stand = qEnvironmentVariable("RECIPE_GUI_AGENT");
    frame->setAgent(agent);
    waitFor([&] { return frame->agentText() == "Agent: Codex, signed in"; }, 10);
    expect(QStringLiteral("the status line names the agent and that it is signed in (%1)").arg(frame->agentText()),
           frame->agentText() == "Agent: Codex, signed in");
    qunsetenv("LEVITATE_AGENT_CODEX");
    frame->setAgent(stand);

    QProcess::execute(script, {"--stop", where});
    qunsetenv("XDG_STATE_HOME");
    frame->setOwnRun(0);
    window.showBrowse();
    settle(100);
}

// The strip about the next system: read with the recab the story names, after
// one prepare and one change in its own scratch state; and no strip when the
// program does not exist.
void stripShots(Window &window, const QString &folder, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString recab = qEnvironmentVariable("RECIPE_GUI_RECAB");
    if (recab.isEmpty()) {
        out << "not as expected: the strip needs RECIPE_GUI_RECAB (gui/recab/fixtures/recab-real), see README.md\n";
        expect(QStringLiteral("a recab to read the next system from"), false);
        return;
    }
    expect(QStringLiteral("no strip while no next system holds changes"), !frame->stripShown());
    auto recabRuns = [&](const QStringList &arguments) {
        QProcess run;
        run.start(recab, arguments);
        run.waitForFinished(20000);
        return run.exitStatus() == QProcess::NormalExit && run.exitCode() == 0;
    };
    expect(QStringLiteral("the next system could be prepared"), recabRuns({"--json", "prepare"}));
    window.refresh();
    waitUntilIdle(window);
    expect(QStringLiteral("a prepared next system with no change shows no strip"), !frame->stripShown());
    expect(QStringLiteral("a change could be made in it"),
           recabRuns({"--json", "change", "install", "htop", "--preparation", "p2", "--revision", "1"}));
    window.refresh();
    waitFor([&] { return frame->stripShown(); }, 20);
    expect(QStringLiteral("the strip says the next system holds the change (%1)").arg(frame->stripText()),
           frame->stripShown() && frame->stripText() == QStringLiteral("The next system holds 1 change."));
    save(window, "s1-strip-browse");
    window.showUpdates();
    expect(QStringLiteral("the strip shows on Updates"), frame->stripShown());
    save(window, "s2-strip-updates");
    window.showInstalled();
    window.showSources();
    window.showRecipe("htop");
    expect(QStringLiteral("and on every other page"), frame->stripShown());
    window.showWish(QString());
    expect(QStringLiteral("including the page that asks for software"), frame->stripShown());
    window.showBrowse();

    // Its button starts System; here no System exists, and the status line says so.
    qputenv("LEVITATE_APP_SYSTEM", "/nonexistent/recab-gui");
    emit frame->finishRequested();
    expect(QStringLiteral("Finish and restart starts System, and says when it cannot (%1)").arg(frame->statusText()),
           frame->statusText().contains("System could not be started"));
    qunsetenv("LEVITATE_APP_SYSTEM");

    activityShots(window, folder, save, expect, out);

    // A recab that does not exist: no strip, and no complaint in the window.
    qputenv("RECIPE_GUI_RECAB", "/nonexistent/recab");
    {
        Window other;
        other.show();
        waitUntilIdle(other);
        settle(300);
        expect(QStringLiteral("no strip when recab cannot be run"), !other.frame()->stripShown());
        expect(QStringLiteral("and no error about it (%1)").arg(other.frame()->statusText()),
               !other.frame()->statusText().contains("recab") && other.catalog().recipes.size() > 3);
        save(other, "s3-no-recab");
    }
    qputenv("RECIPE_GUI_RECAB", recab.toUtf8());
}

} // namespace

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
    {
        // What Activity will list the run under was handed to the agent.
        QFile sent(QDir(qEnvironmentVariable("RECIPE_STANDIN_STATE")).filePath("last-prompt.json"));
        const QJsonObject said = sent.open(QIODevice::ReadOnly) ? QJsonDocument::fromJson(sent.readAll()).object() : QJsonObject();
        expect("the agent is told this is Recipe, and the first line of the request as the title",
               said["app"].toString() == "Recipe" && said["title"].toString() == "a file manager for the terminal");
        // "Go to it" on Activity returns to this run's page.
        window.showBrowse();
        emit window.frame()->goToRun();
        expect("going to the run shows its page again", window.agentAsks());
    }
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

    // The frame: the rail, the strip about the next system, Activity.
    auto saved = [&](QWidget &widget, const QString &name) { ok &= save(widget, name); };
    auto held = [&](const QString &what, bool holds) { expect(what, holds); };
    frameShots(window, saved, held);
    stripShots(window, folder, saved, held, out);
    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
