#include "shots.h"

#include "activity.h"
#include "settings.h"
#include "before.h"
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
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLabel>
#include <QLineEdit>
#include <QMap>
#include <QProcess>
#include <QPushButton>
#include <QPalette>
#include <QStyleFactory>
#include <QTextStream>
#include <QTimer>
#include <QTreeWidget>

#include <optional>

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

// The Before step: press its one button ("Install", "Update"), as a click would.
void goOn(Window &window, const QString &button)
{
    settle(100); // a page drawn just now is shown once the event loop has run
    clickButton(window.beforePage(), button);
}

// What `recipe` was asked to change, each as the arguments it got, from the
// stand-in's log (RECIPE_STANDIN_LOG, else invocations.log in its state
// folder). Reads (list, installed, updates, storage) are left out.
struct Asked {
    QString route; // "system", "store" or "me", as the stand-in told them apart
    QStringList arguments;
};

QList<Asked> changesAsked()
{
    QString path = qEnvironmentVariable("RECIPE_STANDIN_LOG");
    if (path.isEmpty())
        path = QDir(qEnvironmentVariable("RECIPE_STANDIN_STATE")).filePath("invocations.log");
    QFile file(path);
    QList<Asked> found;
    if (!file.open(QIODevice::ReadOnly))
        return found;
    for (const QByteArray &line : file.readAll().split('\n')) {
        const QJsonObject object = QJsonDocument::fromJson(line).object();
        Asked asked{object["route"].toString(), {}};
        for (const QJsonValue &argument : object["args"].toArray())
            asked.arguments << argument.toString();
        for (const QString &argument : asked.arguments) {
            if (argument.startsWith("--"))
                continue;
            if (argument == "install" || argument == "update" || argument == "remove" || argument == "go-back")
                found << asked;
            break;
        }
    }
    return found;
}

// Everything a person can read in a window now: labels, buttons and the rows of lists.
QString wordsOn(QWidget &window)
{
    QStringList words;
    for (const QLabel *label : window.findChildren<QLabel *>())
        if (label->isVisible())
            words << label->text();
    for (const QAbstractButton *button : window.findChildren<QAbstractButton *>())
        if (button->isVisible())
            words << button->text();
    for (const QTreeWidget *tree : window.findChildren<QTreeWidget *>()) {
        if (!tree->isVisible())
            continue;
        for (int row = 0; row < tree->topLevelItemCount(); ++row)
            for (int column = 0; column < tree->columnCount(); ++column)
                if (!tree->isColumnHidden(column))
                    words << tree->topLevelItem(row)->text(column);
    }
    return words.join(QLatin1Char('\n'));
}

// Environment a pass changes and puts back afterwards.
class Environment {
public:
    void set(const char *name, const QString &value)
    {
        keep(name);
        qputenv(name, value.toUtf8());
    }
    void unset(const char *name)
    {
        keep(name);
        qunsetenv(name);
    }
    ~Environment()
    {
        for (auto it = was_.begin(); it != was_.end(); ++it) {
            if (!it.value())
                qunsetenv(it.key().constData());
            else
                qputenv(it.key().constData(), *it.value());
        }
    }
private:
    void keep(const char *name)
    {
        if (!was_.contains(name))
            was_.insert(name, qEnvironmentVariableIsSet(name) ? std::optional<QByteArray>(qgetenv(name)) : std::nullopt);
    }
    QMap<QByteArray, std::optional<QByteArray>> was_;
};

// What the window is as a frame: the rail, the pages in it and what the
// Browse page's search and kinds still do. Pictures `f*`.
void frameShots(Window &window, const Save &save, const Expect &expect)
{
    Frame *frame = window.frame();
    for (const char *title : {"Browse", "Installed", "Updates", "Sources", "Activity", "Settings"})
        expect(QStringLiteral("the rail has %1").arg(title), frame->entryTitles().contains(QString::fromLatin1(title)));
    expect(QStringLiteral("Settings is under Activity, the frame's own"),
           frame->entryTitles().mid(frame->entryTitles().size() - 2) == QStringList({"Activity", "Settings"}));
    for (const char *title : {"Storage", "Help"})
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

// The frame's Settings page: the entry under Activity, and the page opening.
// It reads through the built levitate-agent with the stand-in Codex and a
// settings folder of the story's own.
void settingsShots(Window &window, const QString &folder, const Save &save, const Expect &expect)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECIPE_GUI_RECORDS");
    if (script.isEmpty() || qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT").isEmpty()) {
        expect(QStringLiteral("Settings needs RECIPE_GUI_RECORDS and LEVITATE_ACTIVITY_AGENT, see README.md"), false);
        return;
    }
    const QString home = QDir(folder).filePath("settings-home");
    QDir(home).removeRecursively();
    QDir().mkpath(home);
    const QByteArray config = qgetenv("XDG_CONFIG_HOME"), codex = qgetenv("LEVITATE_AGENT_CODEX");
    qputenv("XDG_CONFIG_HOME", (home + "/config").toUtf8());
    qputenv("LEVITATE_AGENT_CODEX", QDir(QFileInfo(script).absolutePath()).filePath("codex").toUtf8());
    SettingsPage *page = frame->settings();
    frame->choose(Frame::settingsKey());
    waitFor([&] { return page->loaded() || !page->problem().isEmpty(); });
    expect(QStringLiteral("Settings opens and is marked (%1)").arg(page->problem()),
           frame->settingsShown() && frame->selected() == Frame::settingsKey() && page->loaded());
    expect(QStringLiteral("it is the same page: the agent, what it may do, the windows"),
           page->pageText().contains("The agent that does the work") && page->pageText().contains("Installed, and signed in")
               && page->pageText().contains("What the agent may do without asking")
               && page->pageText().contains("Each window opens at the size it had."));
    save(window, "f-settings");
    frame->choose(QStringLiteral("browse"));
    expect(QStringLiteral("choosing Browse leaves Settings"), !frame->settingsShown() && frame->selected() == "browse");
    for (auto [name, value] : {std::pair{"XDG_CONFIG_HOME", config}, std::pair{"LEVITATE_AGENT_CODEX", codex}}) {
        if (value.isEmpty())
            qunsetenv(name);
        else
            qputenv(name, value);
    }
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


// The two places software can go, in two windows of their own: a locked
// computer (`recab.immutable` on a scratch command line), where "for this
// computer" is the next system, and a computer that is not locked, where it is
// the system's store (a scratch folder). Each has its own stand-in state, and
// the stand-in logs every command's arguments, which are checked here.
void placesShots(const QString &folder, const Save &save, const Expect &expect)
{
    const QString recab = qEnvironmentVariable("RECIPE_GUI_RECAB");
    const QString recipe = qEnvironmentVariable("RECIPE_GUI_RECIPE");
    if (recab.isEmpty() || recipe.isEmpty()) {
        expect(QStringLiteral("the two places need RECIPE_GUI_RECAB and RECIPE_GUI_RECIPE, see README.md"), false);
        return;
    }
    const QString scratch = QDir(folder).filePath("places");
    QDir(scratch).removeRecursively();
    QDir().mkpath(scratch);
    auto write = [](const QString &path, const QByteArray &bytes) {
        QFile file(path);
        return file.open(QIODevice::WriteOnly) && file.write(bytes) >= 0;
    };
    write(scratch + "/locked-cmdline", "BOOT_IMAGE=/vmlinuz root=/dev/vda1 ro recab.immutable quiet\n");
    write(scratch + "/open-cmdline", "BOOT_IMAGE=/vmlinuz root=/dev/vda1 ro quiet\n");
    auto kinds = [](const QStringList &arguments, const QStringList &want) { return arguments == want; };
    auto asked = [](qsizetype from) { return changesAsked().mid(from); };

    // ---- A locked computer.
    {
        Environment env;
        env.unset("RECIPE_GUI_PACKAGE_ROOT");
        env.set("RECIPE_GUI_CMDLINE", scratch + "/locked-cmdline");
        env.set("RECIPE_STANDIN_STATE", scratch + "/locked-state");
        env.unset("RECIPE_STANDIN_LOG");
        env.unset("RECIPE_GUI_SYSTEM_ROOT");
        // The stand-in answers `prepared` as it does to root, so the next system's list can be shown.
        env.set("RECIPE_STANDIN_PREPARED", "1");
        env.set("RECIPE_RECAB_BIN", recab);
        env.set("RECAB_STANDIN_STATE", scratch + "/locked-recab");
        // A helper that notes what it was given and runs it: only changes for the computer go through it.
        env.set("RECIPE_GUI_ROOT_HELPER", QFileInfo(recipe).absolutePath() + "/helper-log");
        env.set("HELPER_LOG", scratch + "/locked-helper.log");

        Window window;
        window.resize(1100, 700);
        window.show();
        waitUntilIdle(window, 30);
        const Catalog &catalog = window.catalog();
        const Places places = window.places();
        expect(QStringLiteral("a locked computer is told from its command line"), places.locked && !places.one && catalog.twoPlaces);
        expect(QStringLiteral("it holds the person's own software and the running system's, which says only name and version"),
               catalog.installedItem("tree", Place::Me) && catalog.installedItem("gimp", Place::Computer)
                   && !catalog.installedItem("gimp", Place::Computer)->inStore
                   && catalog.installedItem("gimp", Place::Computer)->version == "3.0.4"
                   && !catalog.installedItem("gimp", Place::Computer)->sizeBytes
                   && catalog.installedItem("gimp", Place::Computer)->kept.isEmpty());
        expect(QStringLiteral("updates: the person's store by `recipe updates`, the system's from the collection's versions"),
               catalog.update("tree", Place::Me) && !catalog.update("tree", Place::Me)->fromCollection
                   && catalog.update("gimp", Place::Computer) && catalog.update("gimp", Place::Computer)->fromCollection);
        expect(QStringLiteral("only the person's store has storage figures"),
               catalog.stores[int(Place::Me)].has && !catalog.stores[int(Place::Computer)].has);
        qsizetype before = changesAsked().size();

        // Install: where it goes, with the sentences for the place picked.
        window.showBrowse();
        window.askInstall("htop");
        BeforePage *page = window.beforePage();
        expect(QStringLiteral("Before: a question, and nothing has started (%1)").arg(page->pageText().left(60)),
               page->isVisible() && page->pageText().contains("Install htop for this computer?")
                   && page->pageText().contains("Nothing has started. This is what will happen if you go on."));
        expect(QStringLiteral("it offers both places, the computer first and picked"),
               page->choiceNames() == QStringList({"For this computer", "For me"}) && page->chosen() == placeKey(Place::Computer));
        expect(QStringLiteral("for this computer on a locked one: into the next system, after a restart, for everyone"),
               page->pageText().contains("Into the next system. It works after a restart, for everyone.")
                   && page->pageText().contains("Asks for the administrator password.")
                   && page->pageText().contains("The running system is not touched."));
        expect(QStringLiteral("it says what the catalog says: where it is fetched from and the sandbox"),
               page->pageText().contains("is fetched from") && page->pageText().contains("inside the sandbox")
                   && page->pageText().contains("The sandbox refuses anything else."));
        expect(QStringLiteral("it does not say how much is downloaded or used; recipe does not report that"),
               !page->pageText().contains("is downloaded") && !page->pageText().contains("is used"));
        expect(QStringLiteral("the foot names the source and offers the recipe"),
               page->pageText().contains("From ") && page->pageText().contains("Read the recipe"));
        save(window, "p1-before-install-choice");
        page->choose(placeKey(Place::Me));
        expect(QStringLiteral("choosing For me changes the sentences: your own folder, at once, and the system is not mentioned"),
               page->chosen() == placeKey(Place::Me) && page->pageText().contains("Install htop for me?")
                   && page->pageText().contains("Into your own folder. It works at once, for you only.")
                   && !page->pageText().contains("The running system is not touched."));
        save(window, "p2-before-install-for-me");
        page->choose(placeKey(Place::Computer));
        goOn(window, "Install");
        waitUntilIdle(window, 30);
        waitFor([&] { return window.frame()->stripShown(); });
        QList<Asked> now = asked(before);
        expect(QStringLiteral("for this computer, locked: `recipe --system install NAME` through the root helper"),
               now.size() == 1 && now[0].route == "system"
                   && kinds(now[0].arguments, {"--system", "install", "levitate:htop", "--machine-events"}));
        expect(QStringLiteral("the root helper was given that command"),
               QFile(scratch + "/locked-helper.log").exists());
        expect(QStringLiteral("it shows as in the next system, after a restart, not in the running system"),
               window.catalog().installedItem("htop", Place::Next) && !window.catalog().installedItem("htop", Place::Computer));
        expect(QStringLiteral("the strip was read again after the change (%1)").arg(window.frame()->stripText()),
               window.frame()->stripShown() && window.frame()->stripText() == QStringLiteral("The next system holds 1 change."));
        before = changesAsked().size();

        // For me: the person's own store, no root helper, no --system, no --package-root.
        auto helperLines = [&] {
            QFile helper(scratch + "/locked-helper.log");
            return helper.open(QIODevice::ReadOnly) ? helper.readAll().count('\n') : 0;
        };
        const qsizetype helperBefore = helperLines();
        window.askInstall("krita");
        page->choose(placeKey(Place::Me));
        goOn(window, "Install");
        waitUntilIdle(window, 30);
        now = asked(before);
        const qsizetype helperAfter = helperLines();
        expect(QStringLiteral("for me: `recipe install NAME` with no store named and no --system"),
               now.size() == 1 && now[0].route == "me" && kinds(now[0].arguments, {"install", "levitate:krita", "--machine-events"}));
        expect(QStringLiteral("for me goes through no root helper"), helperAfter == helperBefore);
        expect(QStringLiteral("it is there at once, in the person's store"),
               window.catalog().installedItem("krita", Place::Me) && window.catalog().installedItem("krita", Place::Me)->inStore);
        before = changesAsked().size();

        // Update where the software is: no choice, only the statement.
        window.showUpdates("gimp", int(Place::Computer));
        window.askUpdate("gimp", Place::Computer);
        expect(QStringLiteral("an update has no choice, only the statement of where it is"),
               page->choiceNames() == QStringList({"For this computer"}) && page->pageText().contains("Where it is")
                   && page->pageText().contains("Update GIMP to 3.0.6?")
                   && page->pageText().contains("GIMP goes from version 3.0.4 to 3.0.6."));
        expect(QStringLiteral("what the system's version was allowed to use is not known, and the page says so"),
               page->pageText().contains("is not known here, so the two cannot be compared"));
        save(window, "p3-before-update-locked");
        goOn(window, "Update");
        waitUntilIdle(window, 30);
        now = asked(before);
        expect(QStringLiteral("updating system software: `recipe --system update NAME`"),
               now.size() == 1 && now[0].route == "system"
                   && kinds(now[0].arguments, {"--system", "update", "levitate:gimp", "--machine-events"}));
        expect(QStringLiteral("the next system holds the new version beside the running one"),
               window.catalog().installedItem("gimp", Place::Next) && window.catalog().installedItem("gimp", Place::Next)->version == "3.0.6"
                   && window.catalog().installedItem("gimp", Place::Computer)->version == "3.0.4");
        before = changesAsked().size();
        // The recipe's page for software in the running system and the next one: buttons that say where.
        window.showRecipe("gimp");
        save(window, "p3b-recipe-two-places");

        // The person's own update, with what `recipe updates` said changed in the recipe.
        window.askUpdate("tree", Place::Me);
        expect(QStringLiteral("an update for me names where and compares nothing it cannot: tree was recorded without needs"),
               page->pageText().contains("Update tree to 2.3.2?") && page->pageText().contains("For me"));
        goOn(window, "Update");
        waitUntilIdle(window, 30);
        now = asked(before);
        expect(QStringLiteral("updating for me: `recipe update NAME`, nothing else"),
               now.size() == 1 && now[0].route == "me" && kinds(now[0].arguments, {"update", "local:tree", "--machine-events"}));
        before = changesAsked().size();

        // What is installed, from both places; what a store cannot do is not offered for the system's.
        window.showInstalled("gimp", int(Place::Computer));
        settle(100);
        QStringList buttons;
        for (QPushButton *button : window.findChildren<QPushButton *>())
            if (button->isVisible())
                buttons << button->text();
        expect(QStringLiteral("a running-system row offers Remove but not Go back (%1)").arg(buttons.join("|")),
               buttons.contains("Remove…") && !buttons.filter("Go back").size());
        expect(QStringLiteral("no store stands behind the system's software, so nothing to clear for it"),
               !buttons.filter("Clear earlier versions, for this computer").size());
        const QString installed = wordsOn(window);
        for (const char *where : {"For me", "For this computer", "In the next system, after a restart"})
            expect(QStringLiteral("Installed says %1").arg(where), installed.contains(QString::fromLatin1(where)));
        expect(QStringLiteral("no page made the window wider than it was (%1)").arg(window.width()), window.width() == 1100);
        save(window, "p4-installed-both-places");
        window.showUpdates();
        save(window, "p5-updates-both-places");
        expect(QStringLiteral("what the next system holds already is not offered as an update again (%1)").arg(window.catalog().updates.size()),
               !window.catalog().update("gimp", Place::Computer) && window.catalog().update("element", Place::Computer));
        window.showBrowse();
        settle(100);
        expect(QStringLiteral("Browse says software that waits for the restart waits for it"), wordsOn(window).contains("After a restart"));
        save(window, "p7-browse-both-places");

        // Remove for me, and from the system.
        window.removeNow("krita", Place::Me);
        waitUntilIdle(window, 30);
        now = asked(before);
        expect(QStringLiteral("removing for me: `recipe remove NAME` and no store, no --system"),
               now.size() == 1 && now[0].route == "me" && kinds(now[0].arguments, {"remove", "krita", "--machine-events"}));
        before = changesAsked().size();
        window.removeNow("element", Place::Computer);
        waitUntilIdle(window, 30);
        now = asked(before);
        expect(QStringLiteral("removing system software: `recipe --system remove NAME`"),
               now.size() == 1 && now[0].route == "system"
                   && kinds(now[0].arguments, {"--system", "remove", "element", "--machine-events"}));
        expect(QStringLiteral("the running system still holds it, and says it goes in the next system"),
               window.catalog().installedItem("element", Place::Computer) && window.catalog().installedItem("element", Place::Computer)->leavesNext);
    }

    // ---- A computer that is not locked.
    {
        Environment env;
        env.unset("RECIPE_GUI_PACKAGE_ROOT");
        env.set("RECIPE_GUI_CMDLINE", scratch + "/open-cmdline");
        env.set("RECIPE_STANDIN_STATE", scratch + "/open-state");
        env.unset("RECIPE_STANDIN_LOG");
        env.set("RECIPE_GUI_SYSTEM_ROOT", scratch + "/system-store");
        env.unset("RECIPE_STANDIN_PREPARED");
        env.set("RECIPE_GUI_ROOT_HELPER", QFileInfo(recipe).absolutePath() + "/helper-log");
        env.set("HELPER_LOG", scratch + "/open-helper.log");
        env.set("RECAB_STANDIN_STATE", scratch + "/open-recab");

        Window window;
        window.resize(1100, 700);
        window.show();
        waitUntilIdle(window, 30);
        expect(QStringLiteral("a computer that is not locked holds the person's store and the system's store"),
               !window.places().locked && window.catalog().twoPlaces && window.catalog().installedItem("gimp", Place::Computer)
                   && window.catalog().installedItem("gimp", Place::Computer)->inStore
                   && window.catalog().stores[int(Place::Computer)].has && window.catalog().installedItem("tree", Place::Me));
        qsizetype before = changesAsked().size();
        window.askInstall("htop");
        BeforePage *page = window.beforePage();
        expect(QStringLiteral("for this computer, not locked: into the system's folder, at once, for everyone, and no restart"),
               page->pageText().contains("Into this computer's software folder, " + scratch + "/system-store. It works at once, for everyone.")
                   && !page->pageText().contains("The running system is not touched.")
                   && !page->pageText().contains("next system"));
        goOn(window, "Install");
        waitUntilIdle(window, 30);
        QList<Asked> now = changesAsked().mid(before);
        expect(QStringLiteral("not locked: `recipe install NAME --package-root` the system's store, through the helper"),
               now.size() == 1 && now[0].route == "store"
                   && kinds(now[0].arguments, {"install", "levitate:htop", "--machine-events", "--package-root", scratch + "/system-store"})
                   && QFile(scratch + "/open-helper.log").exists());
        expect(QStringLiteral("it is in effect at once for everyone: it shows as for this computer"),
               window.catalog().installedItem("htop", Place::Computer) && !window.catalog().installedItem("htop", Place::Next));
        expect(QStringLiteral("there is no next system to read: no strip"), !window.frame()->stripShown());
        before = changesAsked().size();
        window.goBack("gimp", Place::Computer);
        waitUntilIdle(window, 30);
        now = changesAsked().mid(before);
        expect(QStringLiteral("going back, where a store stands behind it: through the helper with that store"),
               now.size() == 1 && kinds(now[0].arguments, {"go-back", "gimp", "--machine-events", "--package-root", scratch + "/system-store"}));
        window.showInstalled("htop", int(Place::Computer));
        save(window, "p6-installed-not-locked");
    }
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
    // The Before step reads nothing and starts nothing; it is shown in both modes.
    window.askInstall(name);
    ok &= save(window, "real-2b-before");
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

    goOn(window, QStringLiteral("Install"));
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

    QTextStream said(stdout);
    // This window is on one explicit store (RECIPE_GUI_PACKAGE_ROOT): one place, no choice.
    if (!window.places().one) {
        said << "not as expected: this pass needs RECIPE_GUI_PACKAGE_ROOT, see README.md\n";
        ok = false;
    }

    window.showBrowse();
    ok &= save(window, "1-browse");

    window.showRecipe("htop");
    ok &= save(window, "2-recipe");

    // Before an install: what will happen, with no choice because there is one place.
    window.askInstall("htop");
    {
        BeforePage *page = window.beforePage();
        auto hold = [&](const QString &what, bool holds) {
            if (!holds) {
                said << "not as expected: " << what << "\n";
                said.flush();
                ok = false;
            }
        };
        hold("the Before page says nothing has started", page->isVisible()
                 && page->pageText().contains("Nothing has started. This is what will happen if you go on."));
        hold("one explicit store is one place, so no choice: a statement", page->choiceNames().size() == 1
                 && !page->pageText().contains("For me") && page->pageText().contains("It is the only place this window changes."));
        hold("it says what happens from the catalog: fetched from, built, in the sandbox, what it needs",
             page->pageText().contains("htop 3.5.3 is fetched from github.com and release-assets.githubusercontent.com and built on this computer, inside the sandbox.")
                 && page->pageText().contains("Needed only to build it: ncurses."));
        hold("it lists what the recipe may use and that the sandbox refuses the rest",
             page->pageText().contains("github.com (port 443), release-assets.githubusercontent.com (port 443). Nothing else.")
                 && page->pageText().contains("Memory") && page->pageText().contains("Time")
                 && page->pageText().contains("The sandbox refuses anything else."));
        hold("it does not say how much is downloaded or how much disk is used: recipe does not report either",
             !page->pageText().contains("is downloaded") && !page->pageText().contains("is used"));
        hold("no running system to mention: this is not a locked computer", !page->pageText().contains("The running system is not touched."));
        ok &= save(window, "2b-before-install");
        // Cancel comes back to where the person was; nothing was run.
        const qsizetype asked = changesAsked().size();
        clickButton(page, "Cancel");
        hold("Cancel starts nothing and comes back to the recipe", changesAsked().size() == asked && !page->isVisible()
                 && window.frame()->selected() == "browse");
    }
    qputenv("RECIPE_STANDIN_SPEED", "0");
    qputenv("RECIPE_STANDIN_STOP_AT", "htop:build");
    const qsizetype beforeInstall = changesAsked().size();
    window.askInstall("htop");
    goOn(window, QStringLiteral("Install"));
    settle(1500);
    {
        const QList<Asked> now = changesAsked().mid(beforeInstall);
        QStringList want{"install", "levitate:htop", "--machine-events", "--package-root", qEnvironmentVariable("RECIPE_GUI_PACKAGE_ROOT")};
        if (now.size() != 1 || now[0].arguments != want || now[0].route != "store") {
            said << "not as expected: the install asked recipe for " << (now.isEmpty() ? QString() : now[0].arguments.join(' ')) << "\n";
            said.flush();
            ok = false;
        }
    }
    ok &= save(window, "3-installing");
    window.stopRun();
    waitUntilIdle(window);

    qunsetenv("RECIPE_STANDIN_STOP_AT");
    // Before an update: where the software is, and what the newer recipe may use that the old one could not.
    window.askUpdate("element", Place::Computer);
    ok &= save(window, "4b-before-update");
    {
        BeforePage *page = window.beforePage();
        const bool holds = page->pageText().contains("Update Element to 1.12.4?")
                           && page->pageText().contains("Element goes from version 1.12.3 to 1.12.4.")
                           && page->pageText().contains("New: static.element.io (port 443).")
                           && page->pageText().contains("Download") && page->choiceNames().size() == 1;
        if (!holds) {
            said << "not as expected: the update's Before page names the version change and what the recipe may use now\n";
            ok = false;
        }
    }
    goOn(window, QStringLiteral("Update"));
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
                said.flush();
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

    window.startInstall("element", Place::Computer, "The issue:\nThe update stopped.\n\nThe expected result:\nElement updates.\n");
    waitUntilIdle(window);
    ok &= save(window, "8-agent-fixed");

    // Sources: the page, a second source added, its list fetched again, only its recipes shown.
    QTextStream out(stdout);
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << "\n";
            out.flush();
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
    window.clearEarlier(Place::Computer, false);
    waitUntilIdle(window);
    expect("clearing leaves no earlier version and everything installed",
           window.catalog().earlierGenerations == 0 && window.catalog().installed.size() == installedBefore
               && window.catalog().installedItem("gimp")->kept.isEmpty());
    ok &= save(window, "14-earlier-versions-cleared");

    // The frame: the rail, the strip about the next system, Activity.
    auto saved = [&](QWidget &widget, const QString &name) { ok &= save(widget, name); };
    auto held = [&](const QString &what, bool holds) { expect(what, holds); };
    frameShots(window, saved, held);
    settingsShots(window, folder, saved, held);
    stripShots(window, folder, saved, held, out);
    placesShots(folder, saved, held);
    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
