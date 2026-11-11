#include "shots.h"

#include "activity.h"
#include "settings.h"
#include "before.h"
#include "frame.h"
#include "helpstory.h"
#include "livestory.h"
#include "look.h"
#include "nextsystem.h"

#include "pages.h"
#include "parts.h"
#include "sources.h"
#include "storage.h"
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
#include <QRegularExpression>
#include <QPushButton>
#include <QPalette>
#include <QStyleFactory>
#include <QTextStream>
#include <QTimer>
#include <QTreeWidget>

#include <algorithm>
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

void clickButton(QWidget *inside, const QString &words)
{
    for (QPushButton *button : inside->findChildren<QPushButton *>())
        if (QString(button->text()).remove(QLatin1Char('&')) == words && button->isVisible())
            button->click();
}

// The buttons a person can press in a window now.
QStringList buttonTexts(QWidget &inside)
{
    QStringList buttons;
    for (const QPushButton *button : inside.findChildren<QPushButton *>())
        if (button->isVisible())
            buttons << QString(button->text()).remove(QLatin1Char('&'));
    return buttons;
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

// What `recipe` answers to a read, asked of the same stand-in and store the window uses,
// for numbers the pages must equal.
QJsonObject answerOf(const QStringList &arguments)
{
    QProcess run;
    run.start(qEnvironmentVariable("RECIPE_GUI_RECIPE"),
              arguments + QStringList{"--package-root", qEnvironmentVariable("RECIPE_GUI_PACKAGE_ROOT")});
    run.waitForFinished(120000);
    return QJsonDocument::fromJson(run.readAllStandardOutput()).object();
}

// Every command that changes a store, from the stand-in's log: hold and unhold too.
QList<Asked> loggedCommands()
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
        found << asked;
    }
    return found;
}

// How many times `recipe` was run with all of these words among its arguments, from the stand-in's log.
int timesAsked(const QStringList &words)
{
    int found = 0;
    for (const Asked &each : loggedCommands()) {
        bool all = true;
        for (const QString &word : words)
            all = all && each.arguments.contains(word);
        found += all;
    }
    return found;
}

// How many processes run a command line with these words (the stand-in recipe, stopped in the middle).
int processesRunning(const QString &pattern)
{
    QProcess pgrep;
    pgrep.start(QStringLiteral("pgrep"), {QStringLiteral("-f"), pattern});
    pgrep.waitForFinished(10000);
    return pgrep.exitCode() == 0 ? 1 : 0;
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
    for (const char *title : {"Browse", "Installed", "Updates", "Sources", "Storage", "Activity", "Settings", "Help"})
        expect(QStringLiteral("the rail has %1").arg(title), frame->entryTitles().contains(QString::fromLatin1(title)));
    expect(QStringLiteral("Storage follows Sources in the rail"),
           frame->entryTitles().mid(0, 5) == QStringList({"Browse", "Installed", "Updates", "Sources", "Storage"}));
    expect(QStringLiteral("Settings is under Activity and Help under Settings, the frame's own"),
           frame->entryTitles().mid(frame->entryTitles().size() - 3) == QStringList({"Activity", "Settings", "Help"}));
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

    // A change just made says so for a few seconds; the line under it is where the recipes are from.
    frame->flash(QString());
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
    // The runs it leaves going are stopped when this ends, whichever way it ends.
    struct Stopper {
        QString script, where;
        ~Stopper() { QProcess::execute(script, {"--stop", where}); }
    } stopper{script, where};
    QProcess make;
    make.start(script, {where, agent});
    make.waitForFinished(300000);
    const QString said = QString::fromUtf8(make.readAllStandardOutput()).trimmed();
    expect(QStringLiteral("the records for Activity were made (%1)").arg(QString::fromUtf8(make.readAllStandardError()).trimmed()),
           make.exitStatus() == QProcess::NormalExit && make.exitCode() == 0 && said.startsWith("state="));
    if (!said.startsWith("state="))
        return;
    const QString state = said.section(' ', 0, 0).mid(6);
    const QString waitingId = said.section("waiting_id=", 1).section(' ', 0, 0);
    const QString runtime = said.section("runtime=", 1).section(' ', 0, 0);
    const QByteArray stateBefore = qgetenv("XDG_STATE_HOME"), runtimeBefore = qgetenv("XDG_RUNTIME_DIR");
    qputenv("XDG_STATE_HOME", state.toUtf8());
    // The sockets of the runs it left going: attaching needs them.
    qputenv("XDG_RUNTIME_DIR", runtime.toUtf8());
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
    frame->setOwnRun(waitingId);
    expect(QStringLiteral("the frame knows the id of this window's own run"), frame->ownRun() == waitingId && !waitingId.isEmpty());
    waitFor([&] { return page->offersGoTo(); });
    expect(QStringLiteral("this window's own live run offers Go to it, and its question can be answered here too (%1)")
               .arg(page->answerButtons().join(", ")),
           waitFor([&] { return page->answerButtons() == QStringList({"Red", "Blue"}); }) && page->offersGoTo()
               && !page->detailText().contains("It waits in this window")
               && !page->detailText().contains("Only the window that started it"));
    save(window, "a-this-windows-own-run");
    clickButton(page, QStringLiteral("Go to it"));
    settle(200);
    expect(QStringLiteral("Go to it leaves Activity for Recipe's own pages"),
           !frame->activityShown() && frame->selected() == "browse");
    frame->choose(Frame::activityKey());
    frame->setOwnRun(QString());
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
    waitFor([&] { return frame->agentText() == "Agent: Codex, signed in"; }, 20);
    expect(QStringLiteral("the status line names the agent and that it is signed in (%1)").arg(frame->agentText()),
           frame->agentText() == "Agent: Codex, signed in");
    qunsetenv("LEVITATE_AGENT_CODEX");
    frame->setAgent(stand);

    QProcess::execute(script, {"--stop", where});
    if (stateBefore.isEmpty())
        qunsetenv("XDG_STATE_HOME");
    else
        qputenv("XDG_STATE_HOME", stateBefore);
    if (runtimeBefore.isEmpty())
        qunsetenv("XDG_RUNTIME_DIR");
    else
        qputenv("XDG_RUNTIME_DIR", runtimeBefore);
    frame->setOwnRun(QString());
    window.showBrowse();
    settle(100);

    // Runs that outlive their window: found, followed, answered and stopped from Activity.
    // Each one is started by a window of its own, through "Ask for software", that then closes.
    livestory::prepare(QDir(folder).filePath("a"), QFileInfo(script).absolutePath());
    auto start = [&](const QString &story, const QString &request) {
        livestory::useStory(story);
        const QByteArray before = qgetenv("RECIPE_GUI_AGENT");
        qputenv("RECIPE_GUI_AGENT", agent.toUtf8());
        auto *started = new Window;
        qputenv("RECIPE_GUI_AGENT", before);
        started->show();
        waitFor([&] { return !started->busy(); });
        started->askAgent(QStringLiteral("The request:\n%1\n").arg(request));
        return livestory::Started{started, started->frame()};
    };
    livestory::run(*frame, window, start, save, expect, "a");
    // What the helper was given for the first of them: it may write in the person's own recipe folder.
    {
        QFile log(livestory::codexLog());
        QString first;
        if (log.open(QIODevice::ReadOnly))
            first = QString::fromUtf8(log.readAll()).section('\n', 0, 0);
        const QJsonObject call = QJsonDocument::fromJson(first.toUtf8()).object();
        QStringList args;
        for (const QJsonValue &arg : call["args"].toArray())
            args << arg.toString();
        const QString own = QDir::homePath() + "/.local/share/recipe/recipes";
        expect(QStringLiteral("the agent was started with a sandbox that may write, in the person's own recipe folder (%1)")
                   .arg(args.join(' ')),
               args.contains("workspace-write") && args.contains("-C") && args.value(args.indexOf("-C") + 1) == own
                   && call["cwd"].toString() == own && QDir(own).exists());
    }
    livestory::finish();
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
// The pages the frame owns after Settings: Help for Recipe, and the page a person
// sees the first time. The checks are `../kit/helpstory.*`; this says which page
// of Recipe each target is.
void helpShots(Window &window, const QString &folder, const Save &save, const Expect &expect)
{
    const QString script = qEnvironmentVariable("RECIPE_GUI_RECORDS");
    if (script.isEmpty()) {
        expect(QStringLiteral("Help needs RECIPE_GUI_RECORDS (gui/agent/fixtures/records) to find the stand-in Codex"), false);
        return;
    }
    helpstory::Plan plan;
    plan.app = Frame::Recipe;
    plan.window = &window;
    plan.frame = window.frame();
    plan.firstEntry = QStringLiteral("browse");
    plan.idle = [&] { waitUntilIdle(window, 30); };
    plan.folder = folder;
    plan.fixtures = QFileInfo(script).absolutePath();
    plan.pageShown = [&](const QString &key) {
        QWidget *page = key == QLatin1String("browse")      ? static_cast<QWidget *>(window.findChild<BrowsePage *>())
                        : key == QLatin1String("installed") ? static_cast<QWidget *>(window.findChild<InstalledPage *>())
                        : key == QLatin1String("updates")   ? static_cast<QWidget *>(window.findChild<UpdatesPage *>())
                        : key == QLatin1String("sources")   ? static_cast<QWidget *>(window.findChild<SourcesPage *>())
                        : key == QLatin1String("storage")   ? static_cast<QWidget *>(window.findChild<StoragePage *>())
                        : key == QLatin1String("ask")       ? static_cast<QWidget *>(window.findChild<WishPage *>())
                                                            : nullptr;
        return page && page->isVisible();
    };
    helpstory::run(plan, save, expect, "h");
    window.showBrowse();
    settle(100);
}

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
        run.waitForFinished(120000);
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

    // The running system on trial: recab still calls the other system "next" (the preparation
    // that made the running system is kept until the trial is kept), but it is the system from
    // before, so there is no next system to finish and the strip goes.
    {
        const QString trial = QDir(folder).filePath("trial-recab");
        QDir(trial).removeRecursively();
        QDir().mkpath(trial);
        const QByteArray stateBefore = qgetenv("RECAB_STANDIN_STATE");
        const QString reboot = QFileInfo(recab).absolutePath() + "/reboot-real";
        QProcessEnvironment trialEnv = QProcessEnvironment::systemEnvironment();
        trialEnv.insert("RECAB_STANDIN_STATE", trial);
        trialEnv.insert("RECAB_STANDIN_KEEP", "hand"); // the restarted system stays on trial until it is kept
        auto standIn = [&](const QString &program, const QStringList &arguments) {
            QProcess run;
            run.setProcessEnvironment(trialEnv);
            run.start(program, arguments);
            run.waitForFinished(60000);
            return run.exitStatus() == QProcess::NormalExit && run.exitCode() == 0 ? QString::fromUtf8(run.readAllStandardOutput())
                                                                                    : QString();
        };
        const bool made = !standIn(recab, {"--json", "prepare"}).isEmpty()
                          && !standIn(recab, {"--json", "change", "install", "htop", "--preparation", "p2", "--revision", "1"}).isEmpty()
                          && !standIn(recab, {"--json", "finalize", "--preparation", "p2", "--revision", "2"}).isEmpty()
                          && !standIn(recab, {"--json", "set-next", "B"}).isEmpty();
        standIn(reboot, {}); // plays the restart: the computer comes up on the next system, not yet kept
        const QByteArray status = standIn(recab, {"status", "--json"}).toUtf8();
        const QJsonObject lifecycle = QJsonDocument::fromJson(status).object()["lifecycle"].toObject();
        expect(QStringLiteral("the stand-in recab has the running system on trial and still calls the other one next (%1)")
                   .arg(QString::fromUtf8(QJsonDocument(lifecycle).toJson(QJsonDocument::Compact))),
               made && lifecycle["state"] == "on_trial" && lifecycle["other"] == "next"
                   && QJsonDocument::fromJson(status).object()["preparation"].toObject()["changes"].toArray().size() == 1);
        QStringList titles;
        expect(QStringLiteral("the reader of the strip finds no change to finish then"),
               NextSystem::parse(status, &titles) && titles.isEmpty());
        qputenv("RECAB_STANDIN_STATE", trial.toUtf8());
        window.refresh();
        waitUntilIdle(window);
        settle(300);
        expect(QStringLiteral("no strip while the running system is on trial (%1)").arg(frame->stripText()),
               !frame->stripShown() && frame->stripText().isEmpty());
        window.showUpdates();
        expect(QStringLiteral("not on any other page either"), !frame->stripShown());
        save(window, "s4-no-strip-on-trial");
        if (stateBefore.isEmpty())
            qunsetenv("RECAB_STANDIN_STATE");
        else
            qputenv("RECAB_STANDIN_STATE", stateBefore);
        window.showBrowse();
    }

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
        // "Open" starts the program from the profile of the store it is in (the person's own: its bin is not
        // on PATH), and the one its menu entry names, not the first program the package put in bin.
        {
            const QString meProfile = scratch + "/locked-state/me/profile";
            const QString startedLog = scratch + "/locked-state/started.log";
            auto started = [&] {
                QFile file(startedLog);
                return file.open(QIODevice::ReadOnly) ? QString::fromUtf8(file.readAll()) : QString();
            };
            QFile::remove(startedLog);
            window.showRecipe("krita");
            settle(150);
            const InstalledItem *mine = window.catalog().installedItem("krita", Place::Me);
            expect(QStringLiteral("recipe named the person's own profile for it, and the programs and menu entry it put there"),
                   mine && mine->profile == meProfile && mine->programs == QStringList({"kritarunner", "krita"})
                       && mine->menu == QStringList({"krita"}));
            expect(QStringLiteral("the page offers Open"), buttonTexts(window).contains("Open"));
            clickButton(window.recipePage(), "Open");
            waitFor([&] { return started().contains("krita started from"); }, 10);
            expect(QStringLiteral("Open started krita (the menu entry's program, not kritarunner) from the person's profile, with its bin first on PATH (%1)").arg(started().trimmed()),
                   started().contains("krita started from " + meProfile + "/bin/krita, PATH begins with " + meProfile + "/bin")
                       && !started().contains("kritarunner started"));
            expect(QStringLiteral("the status line says it was started (%1)").arg(window.frame()->statusText()),
                   window.frame()->statusText() == "Started Krita");
            save(window, "p8-open-for-me");
            // The program is not there any more: the window says what is true.
            QFile::remove(meProfile + "/bin/krita");
            QFile::remove(meProfile + "/bin/kritarunner");
            clickButton(window.recipePage(), "Open");
            settle(200);
            expect(QStringLiteral("when it cannot be started the status line says which program is missing and where (%1)").arg(window.frame()->statusText()),
                   window.frame()->statusText().contains("Krita could not be started: krita, which the menu entry krita starts, in " + meProfile + "/bin is not there."));
            save(window, "p9-open-failed");
        }
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
        settle(100);
        {
            const QString there = wordsOn(window);
            expect(QStringLiteral("what only the next system holds, and the running system's list, offer no Open and say why"),
                   !buttonTexts(window).contains("Open") && there.contains("It cannot be opened until then.")
                       && there.contains("This window cannot open it: the running system's list names the software and its version, not its programs."));
            window.open("gimp", Place::Next);
            expect(QStringLiteral("asking anyway says why (%1)").arg(window.frame()->statusText()),
                   window.frame()->statusText().contains("only in the next system, so it cannot be opened until after the restart"));
        }
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
        // Software whose recipe is gone is taken back to the earlier version its store keeps: no recipe is needed.
        before = changesAsked().size();
        window.showInstalled("fd", int(Place::Computer));
        settle(100);
        expect(QStringLiteral("a program with no recipe, in a store that keeps an earlier version, offers going back (%1)").arg(buttonTexts(window).join("|")),
               !window.catalog().recipe("fd") && buttonTexts(window).contains("Go back to 10.1.0") && buttonTexts(window).contains("Its receipt"));
        clickButton(window.installedPage(), "Its receipt");
        settle(150);
        expect(QStringLiteral("its page offers it too"), window.recipePage()->isVisible() && buttonTexts(window).contains("Go back to 10.1.0"));
        save(window, "18d-program-no-recipe-kept");
        clickButton(window.recipePage(), "Go back to 10.1.0");
        waitUntilIdle(window, 30);
        now = changesAsked().mid(before);
        expect(QStringLiteral("going back with no recipe: `recipe go-back fd` with that store, through the helper, and 10.1.0 is active"),
               now.size() == 1 && kinds(now[0].arguments, {"go-back", "fd", "--machine-events", "--package-root", scratch + "/system-store"})
                   && window.catalog().installedItem("fd", Place::Computer)->version == "10.1.0"
                   && window.catalog().installedItem("fd", Place::Computer)->kept == "10.2.0");
        window.showInstalled("htop", int(Place::Computer));
        save(window, "p6-installed-not-locked");
    }
}


// The program page in three states: not installed, installed, and installed with
// an update (and with an earlier version kept, so it can be gone back to).
void programPages(Window &window, const Save &save, const Expect &expect)
{
    auto buttonsOn = [&](QWidget &inside) {
        QStringList buttons;
        for (const QPushButton *button : inside.findChildren<QPushButton *>())
            if (button->isVisible())
                buttons << button->text();
        return buttons;
    };
    const Catalog &catalog = window.catalog();

    // Not installed: what it is, what it may use, where it is from, and Install.
    expect(QStringLiteral("htop is not installed here (its install was stopped)"), !catalog.installedItem("htop"));
    window.showRecipe("htop");
    settle(100);
    QString words = wordsOn(window);
    QStringList buttons = buttonsOn(window);
    expect(QStringLiteral("not installed: it says so and offers Install, nothing to remove or hold (%1)").arg(buttons.join("|")),
           words.contains("Not installed") && buttons.contains("Install") && !buttons.contains("Remove…")
               && !buttons.contains("Keep at this version"));
    expect(QStringLiteral("it says what it adds, what it may use, where it is from and offers the recipe"),
           words.contains("It adds") && words.contains("the program htop") && words.contains("What it may use")
               && words.contains("github.com (port 443)") && words.contains("The recipe") && buttons.contains("Read it")
               && words.contains("from "));
    save(window, "15a-program-not-installed");

    // Installed, nothing newer (Element, updated earlier in the story).
    const InstalledItem *mail = catalog.installedItem("element");
    expect(QStringLiteral("element is installed and has no update"), mail && !catalog.update("element"));
    window.showRecipe("element");
    settle(100);
    words = wordsOn(window);
    buttons = buttonsOn(window);
    expect(QStringLiteral("installed: its version, Open, Keep at this version and Remove; nothing to install again (%1)").arg(buttons.join("|")),
           mail && words.contains("Version " + mail->version) && buttons.contains("Open") && buttons.contains("Keep at this version")
               && buttons.contains("Remove…") && !buttons.contains("Install") && words.contains("Where it is"));
    save(window, "15b-program-installed");

    // Installed with an update, and an earlier version kept.
    const InstalledItem *gimp = catalog.installedItem("gimp");
    const UpdateItem *newer = catalog.update("gimp");
    expect(QStringLiteral("gimp has an update and an earlier version kept"), gimp && newer && !gimp->kept.isEmpty());
    window.showRecipe("gimp");
    settle(100);
    words = wordsOn(window);
    buttons = buttonsOn(window);
    expect(QStringLiteral("installed with an update: Update to the new version, Go back to the kept one, Keep, Remove (%1)").arg(buttons.join("|")),
           gimp && newer && buttons.contains("Update to " + newer->to) && buttons.contains("Go back to " + gimp->kept)
               && buttons.contains("Keep at this version") && buttons.contains("Remove…")
               && words.contains(newer->to + " is ready."));
    save(window, "15c-program-installed-with-update");
}

// Storage before anything is cleared: what the stand-in says, and what was done so far.
void storageBefore(Window &window, const Save &save, const Expect &expect)
{
    const QJsonObject said = answerOf({"storage", "--json"});
    const qint64 active = said["active_bytes"].toInteger(), earlier = said["earlier_bytes"].toInteger();
    const int generations = said["earlier_generations"].toInt();
    expect(QStringLiteral("the stand-in reports earlier versions (%1 bytes in %2)").arg(earlier).arg(generations), earlier > 0 && generations > 0);
    window.showStorage();
    settle(100);
    const QString words = wordsOn(window);
    expect(QStringLiteral("Storage is marked in the rail"), window.frame()->selected() == "storage");
    expect(QStringLiteral("Storage says what the earlier versions take, as recipe reports it (%1)").arg(bytesText(earlier)),
           words.contains(QStringLiteral("Earlier versions take %1").arg(bytesText(earlier)))
               && words.contains(counted(generations, QStringLiteral("%1 earlier generation"), QStringLiteral("%1 earlier generations"))));
    expect(QStringLiteral("and what is installed takes (%1)").arg(bytesText(active)), words.contains(bytesText(active)));
    expect(QStringLiteral("it offers to clear them, and says what recipe does not report"),
           words.contains("Clear earlier versions…") && words.contains("Recipe reports no sizes for downloads or builds"));
    // The history: what the story did to this store so far, newest first.
    const QJsonArray history = answerOf({"history", "--json"})["history"].toArray();
    expect(QStringLiteral("recipe's history has the story's changes (%1)").arg(history.size()), history.size() >= 3);
    const QStringList lines = window.storagePage()->historyLines();
    expect(QStringLiteral("the page lists as many as recipe reports (%1)").arg(lines.size()), lines.size() == history.size());
    expect(QStringLiteral("newest first: the update of Element, the agent's install of Krita, then the two goings back (%1)").arg(lines.join(" | ")),
           lines.size() >= 4 && lines[0] == "Updated Element from 1.12.3 to 1.12.4, from levitate"
               && lines[1] == "Installed Krita 5.2.13, from levitate"
               && lines[2] == "Went back to GIMP 3.0.4, from 3.0.2" && lines[3] == "Went back to GIMP 3.0.2, from 3.0.4");
    save(window, "15d-storage");
    window.showInstalled("gimp");
    settle(100);
    const QString installed = wordsOn(window);
    expect(QStringLiteral("Installed keeps a pointer to Storage and no clear button of its own"),
           installed.contains("They are in Storage.") && !installed.contains("Clear earlier versions"));
}

// After clearing: no earlier versions, and the clearing is the newest line of the history.
void storageAfter(Window &window, const Save &save, const Expect &expect)
{
    const QJsonObject said = answerOf({"storage", "--json"});
    expect(QStringLiteral("the stand-in holds no earlier version now"), said["earlier_generations"].toInt() == 0 && said["earlier_bytes"].toInteger() == 0);
    const int before = int(window.storagePage()->historyLines().size());
    window.showStorage();
    settle(100);
    const QString words = wordsOn(window);
    const QStringList lines = window.storagePage()->historyLines();
    expect(QStringLiteral("Storage says no earlier versions are kept, and offers nothing to clear"),
           words.contains("No earlier versions are kept") && !words.contains("Clear earlier versions…") && words.contains("None are kept."));
    expect(QStringLiteral("clearing added a line, on top (%1)").arg(lines.value(0)),
           lines.size() == before + 1 && lines.value(0) == "Cleared the earlier versions");
    save(window, "15e-storage-cleared");
}

// Hold: a program kept at its version is marked, not counted, left by "Update all", and
// released again.
void holdShots(Window &window, const Save &save, const Expect &expect)
{
    const int counting = window.catalog().updatesToCount();
    expect(QStringLiteral("gimp has an update to count, with others (%1)").arg(counting),
           window.catalog().update("gimp") && !window.catalog().update("gimp")->held && counting >= 2);
    const QString root = qEnvironmentVariable("RECIPE_GUI_PACKAGE_ROOT");
    window.hold("gimp", Place::Computer, true);
    waitUntilIdle(window, 30);
    const InstalledItem *item = window.catalog().installedItem("gimp");
    expect(QStringLiteral("holding: `recipe hold gimp` for that store"),
           [&] {
               for (const Asked &each : loggedCommands())
                   if (each.arguments == QStringList({"hold", "gimp", "--machine-events", "--package-root", root}))
                       return true;
               return false;
           }());
    expect(QStringLiteral("gimp is held, and its update is listed as held"),
           item && item->held && window.catalog().update("gimp") && window.catalog().update("gimp")->held);
    expect(QStringLiteral("the Updates badge does not count it (%1 of %2)").arg(window.frame()->badge("updates")).arg(counting),
           window.catalog().updatesToCount() == counting - 1 && window.frame()->badge("updates") == counting - 1
               && window.catalog().updates.size() == counting);

    window.showInstalled("gimp");
    settle(100);
    QString words = wordsOn(window);
    QStringList buttons;
    for (const QPushButton *button : window.findChildren<QPushButton *>())
        if (button->isVisible())
            buttons << button->text();
    expect(QStringLiteral("Installed marks it as kept and offers Let it update again, not its update (%1)").arg(buttons.join("|")),
           words.contains("kept at this version") && buttons.contains("Let it update again") && !buttons.filter("Update to").size()
               && !buttons.contains("Keep at this version"));
    save(window, "16a-installed-held");

    window.showUpdates("gimp");
    settle(100);
    words = wordsOn(window);
    expect(QStringLiteral("Updates lists it as kept at this version, and says Update all leaves it"),
           words.contains("Kept at this version") && words.contains("Update all leaves GIMP (kept at 3.0.4) alone")
               && words.contains("Let it update again"));
    save(window, "16b-updates-held");

    window.askUpdate("gimp", Place::Computer);
    settle(100);
    expect(QStringLiteral("its Before step is not offered while it is kept"), !window.beforePage()->isVisible());

    window.showRecipe("gimp");
    settle(100);
    words = wordsOn(window);
    expect(QStringLiteral("the program page says it is kept and offers to release it"),
           words.contains("Kept at 3.0.4") && words.contains("Let it update again") && !words.contains("Update to 3.0.6")
               && words.contains("is ready, and is not offered while it is kept"));
    save(window, "16c-program-held");

    // "Update all" asks about each update that is not kept, in turn, and never about gimp.
    window.showUpdates();
    settle(100);
    clickButton(window.updatesPage(), "Update all");
    QStringList asked;
    for (int turn = 0; turn < 10; ++turn) {
        settle(150);
        if (!window.beforePage()->isVisible())
            break;
        asked << window.beforePage()->pageText().section('\n', 0, 0);
        goOn(window, "Update");
        waitUntilIdle(window, 30);
    }
    expect(QStringLiteral("Update all asked about the others and not about GIMP (%1)").arg(asked.join(" | ")),
           asked.size() == counting - 1 && !asked.join(" ").contains("GIMP"));
    expect(QStringLiteral("what is kept is still waiting, and counts for nothing"),
           window.catalog().update("gimp") && window.catalog().update("gimp")->held && window.catalog().updatesToCount() == 0
               && window.frame()->badge("updates") == 0);

    // A refused update of a held program shows recipe's own sentence, and offers no repair.
    window.startInstall("gimp", Place::Computer);
    waitUntilIdle(window, 30);
    words = wordsOn(window);
    expect(QStringLiteral("a refused update of a held program: recipe's own sentence, no agent to fix it"),
           words.contains("GIMP is kept at version 3.0.4")
               && words.contains("gimp is held, so it is not updated. Release it with: recipe unhold gimp")
               && !words.contains("Let the agent fix it") && !words.contains("What the agent is given"));
    save(window, "16d-held-refused");

    // Released again: its update counts and is offered.
    window.hold("gimp", Place::Computer, false);
    waitUntilIdle(window, 30);
    expect(QStringLiteral("released: `recipe unhold gimp`, no longer held, counted again"),
           [&] {
               for (const Asked &each : loggedCommands())
                   if (each.arguments == QStringList({"unhold", "gimp", "--machine-events", "--package-root", root}))
                       return true;
               return false;
           }() && window.catalog().installedItem("gimp") && !window.catalog().installedItem("gimp")->held
               && window.catalog().updatesToCount() == 1 && window.frame()->badge("updates") == 1);
    window.showInstalled("gimp");
    settle(100);
    buttons.clear();
    for (const QPushButton *button : window.findChildren<QPushButton *>())
        if (button->isVisible())
            buttons << button->text();
    expect(QStringLiteral("and Installed offers Keep at this version and the update again"),
           buttons.contains("Keep at this version") && !buttons.filter("Update to").isEmpty());
    window.showStorage();
    settle(100);
    const QStringList lines = window.storagePage()->historyLines();
    expect(QStringLiteral("the history says it was kept and let go (%1)").arg(lines.mid(0, 4).join(" | ")),
           lines.value(0) == "Let GIMP update again" && lines.contains("Kept GIMP at 3.0.4")
               && lines.indexOf("Kept GIMP at 3.0.4") > 0);
    window.showBrowse();
}


// What the audit of the window found and the stories did not reach: software whose recipe is gone,
// the page for a name nothing carries, the remove question, a short change with a page of its own
// (running, stopped, not finished, tried again), closing the window while something runs. Pictures
// `17*` and `18*`, `19*`; "Open" for the person's own store is in the places pass (`p8`, `p9`).
void gapShots(Window &window, const Save &save, const Expect &expect)
{
    // ---- Software whose recipe is gone: what its receipt says, and what still works without one.
    expect(QStringLiteral("fd is installed and no recipe for it is in the collection or the sources"),
           window.catalog().installedItem("fd") && !window.catalog().recipe("fd"));
    window.showInstalled("fd");
    settle(100);
    QString words = wordsOn(window);
    QStringList buttons = buttonTexts(window);
    expect(QStringLiteral("Installed says its recipe is gone, what the receipt lists and where it came from, and offers what works (%1)").arg(buttons.join("|")),
           words.contains("Its recipe is gone: no recipe for it is in the collection or the sources any more.")
               && words.contains("Its receipt lists 2 files.")
               && words.contains("It came from the source oldmirror, which is not among the sources any more.")
               && buttons.contains("Its receipt") && buttons.contains("Remove…")
               && buttons.contains("Keep at this version") && !buttons.contains("Open") && !buttons.contains("Install")
               && !buttons.contains("Go back to 10.1.0")); // the story cleared the earlier versions before this
    save(window, "18a-installed-no-recipe");
    clickButton(window.installedPage(), "Its receipt");
    settle(150);
    words = wordsOn(window);
    buttons = buttonTexts(window);
    expect(QStringLiteral("its page shows the receipt: name, version, files, source and commit, the recipe file that is not there (%1)").arg(buttons.join("|")),
           window.recipePage()->isVisible() && words.contains("Its recipe is gone") && words.contains("What its receipt says")
               && words.contains("bin/fd") && words.contains("share/man/man1/fd.1") && words.contains("2 files")
               && words.contains("The source oldmirror, which is not among the sources any more, at commit e41c7a0")
               && words.contains("/usr/share/recipe/recipes/fd.rhai") && words.contains("The file is not there any more.")
               && words.contains("Kept at ") == false);
    expect(QStringLiteral("and offers remove and keeping at the version, never install or a recipe to read"),
           buttons.contains("Remove…") && buttons.contains("Keep at this version")
               && !buttons.contains("Install") && !buttons.contains("Read it"));
    save(window, "18b-program-no-recipe");
    // Keeping it at its version works from the receipt alone, through a page of its own, and the page is drawn again.
    const int kept = timesAsked({"hold", "fd"});
    clickButton(window.recipePage(), "Keep at this version");
    waitUntilIdle(window, 30);
    expect(QStringLiteral("keeping it ran `recipe hold fd`, fd is held, and the receipt's page is back with the way to let it go"),
           timesAsked({"hold", "fd"}) == kept + 1 && window.catalog().installedItem("fd") && window.catalog().installedItem("fd")->held
               && window.recipePage()->isVisible() && wordsOn(window).contains("Kept at 10.2.0")
               && buttonTexts(window).contains("Let it update again"));
    clickButton(window.recipePage(), "Let it update again");
    waitUntilIdle(window, 30);
    expect(QStringLiteral("and it can be let go again"),
           timesAsked({"unhold", "fd"}) == 1 && !window.catalog().installedItem("fd")->held && window.recipePage()->isVisible());
    // The back link leads to what is installed.
    for (ClickLabel *label : window.recipePage()->findChildren<ClickLabel *>())
        if (label->text() == QStringLiteral("‹ Installed"))
            emit label->clicked();
    settle(100);
    expect(QStringLiteral("the page's back link returns to Installed"), window.installedPage()->isVisible());
    expect(QStringLiteral("Updates says recipe could not look it up"),
           window.catalog().updateProblems.join("|").contains("fd: no recipe for it is in the collection or the sources any more"));

    // ---- A name no recipe has and nothing installed carries.
    window.showRecipe("no-such-recipe");
    settle(100);
    words = wordsOn(window);
    expect(QStringLiteral("its page says there is no recipe by that name, why it might be so, and leads back"),
           words.contains("No recipe named no-such-recipe") && words.contains("No source offers a recipe by that name, and nothing installed carries it.")
               && words.contains("‹ All software"));
    save(window, "18c-no-recipe-named");

    // ---- The remove question, then the change with a page of its own.
    const int removed = timesAsked({"remove", "fd"});
    window.showInstalled("fd");
    settle(100);
    clickButton(window.installedPage(), "Remove…");
    settle(200);
    QPointer<RemoveDialog> question = window.removeQuestion();
    expect(QStringLiteral("Remove… asks first, in a window of its own; nothing has run"),
           question && question->isVisible() && timesAsked({"remove", "fd"}) == removed);
    if (question) {
        const QString asked = wordsOn(*question);
        expect(QStringLiteral("for software with no recipe it still says what happens (%1)").arg(asked.left(80)),
               asked.contains("Remove fd 10.2.0?") && asked.contains("It is no longer installed.")
                   && asked.contains("Recipe keeps its files among the earlier versions until those are cleared, in Storage.")
                   && asked.contains("What it wrote in your home folder stays."));
        save(*question, "17a-remove-question");
        clickButton(question, "Cancel");
    }
    settle(200);
    expect(QStringLiteral("Cancel removes nothing and closes the question"),
           timesAsked({"remove", "fd"}) == removed && window.catalog().installedItem("fd") && (!question || !question->isVisible()));

    qputenv("RECIPE_STANDIN_STOP_AT", "fd:remove");
    clickButton(window.installedPage(), "Remove…");
    settle(200);
    clickButton(window.removeQuestion(), "Remove");
    waitFor([&] { return timesAsked({"remove", "fd"}) > removed; }, 30);
    settle(300);
    expect(QStringLiteral("Remove ran `recipe remove fd`, and the change has a page: what it does, Stop, and the status line says what runs (%1; %2)")
               .arg(buttonTexts(window).join("|"), window.frame()->statusText()),
           timesAsked({"remove", "fd"}) == removed + 1 && window.changePage()->isVisible() && window.busy()
               && wordsOn(window).contains("Removing fd") && buttonTexts(window).contains("Stop")
               && wordsOn(window).contains("Stopping it before it ends leaves fd installed. You can remove it again from Installed.")
               && window.frame()->statusText().contains("Removing fd"));
    save(window, "17b-change-running");
    clickButton(window.changePage(), "Stop");
    waitUntilIdle(window, 30);
    words = wordsOn(window);
    buttons = buttonTexts(window);
    expect(QStringLiteral("stopped: the page says so, that fd 10.2.0 is still installed, and offers Try again and Back (%1)").arg(buttons.join("|")),
           words.contains("You stopped it") && words.contains("fd 10.2.0 is still installed.") && buttons.contains("Try again")
               && buttons.contains("Back") && !buttons.contains("Stop") && window.catalog().installedItem("fd") && !window.busy());
    save(window, "17c-change-stopped");

    qunsetenv("RECIPE_STANDIN_STOP_AT");
    qputenv("RECIPE_STANDIN_FAIL", "remove:fd");
    clickButton(window.changePage(), "Try again");
    waitUntilIdle(window, 30);
    words = wordsOn(window);
    buttons = buttonTexts(window);
    expect(QStringLiteral("not finished: the headline, recipe's own message, the command's own output, what is true, and the way on (%1)").arg(buttons.join("|")),
           timesAsked({"remove", "fd"}) == removed + 2 && window.changePage()->isVisible()
               && words.contains("Removing fd did not finish")
               && words.contains("cannot switch the active profile of") && words.contains("Permission denied")
               && words.contains("fd 10.2.0 is still installed.")
               && buttons.contains("Try again") && buttons.contains("Back") && window.catalog().installedItem("fd") && !window.busy());
    save(window, "17d-change-failed");
    qunsetenv("RECIPE_STANDIN_FAIL");
    clickButton(window.changePage(), "Try again");
    waitUntilIdle(window, 30);
    expect(QStringLiteral("Try again ran the same command once more and it worked: fd is gone, Installed is drawn again, the status line says so"),
           timesAsked({"remove", "fd"}) == removed + 3 && !window.catalog().installedItem("fd") && window.installedPage()->isVisible()
               && window.frame()->statusText().contains("Removed fd"));
    window.showStorage();
    settle(100);
    expect(QStringLiteral("and the history has it, on top"), window.storagePage()->historyLines().value(0) == "Removed fd 10.2.0");

    // A failed change that is not a package's: the lists could not be fetched.
    qputenv("RECIPE_STANDIN_FAIL", "refresh");
    window.showUpdates();
    settle(100);
    clickButton(window.updatesPage(), "Check for updates");
    waitUntilIdle(window, 30);
    words = wordsOn(window);
    expect(QStringLiteral("fetching that did not finish says so, with recipe's own words (%1)").arg(words.left(60)),
           window.changePage()->isVisible() && words.contains("Fetching the lists did not finish")
               && words.contains("could not fetch the list of levitate: connection timed out")
               && words.contains("The recipes are as they were when they were last read."));
    save(window, "17e-fetch-failed");
    qunsetenv("RECIPE_STANDIN_FAIL");
    clickButton(window.changePage(), "Back");
    waitUntilIdle(window, 30);
    expect(QStringLiteral("Back returns to the page the person was on"), window.updatesPage()->isVisible());

    // ---- Closing the window while something runs: asked first, with what is true.
    {
        qputenv("RECIPE_STANDIN_STOP_AT", "htop:build");
        const int installs = timesAsked({"install", "levitate:htop"});
        Window other;
        other.show();
        waitUntilIdle(other, 30);
        other.startInstall("htop", Place::Computer);
        waitFor([&] { return timesAsked({"install", "levitate:htop"}) > installs; }, 30);
        settle(500);
        expect(QStringLiteral("an install runs, stopped in the middle of its build"),
               processesRunning("install levitate:htop --machine-events") == 1 && other.busy());
        other.close();
        settle(300);
        QPointer<QMessageBox> box = other.closeQuestion();
        expect(QStringLiteral("closing the window asks first, and the window stays"), box && box->isVisible() && other.isVisible());
        if (box) {
            const QString asked = wordsOn(*box);
            expect(QStringLiteral("it says what stops, what is kept, that nothing was changed and that it can be started again (%1)").arg(asked.left(60)),
                   asked.contains("Installing htop is still running.") && asked.contains("If you close the window now it is stopped.")
                       && asked.contains("What is already fetched and built is kept.") && asked.contains("Nothing on this computer is changed.")
                       && asked.contains("You can start it again from htop's page whenever you like.")
                       && buttonTexts(*box).contains("Stop it and close") && buttonTexts(*box).contains("Keep the window open"));
            save(*box, "19a-close-install");
            clickButton(box, "Keep the window open");
        }
        settle(300);
        expect(QStringLiteral("keeping the window open leaves the install running"),
               other.isVisible() && other.busy() && processesRunning("install levitate:htop --machine-events") == 1);
        other.close();
        settle(300);
        box = other.closeQuestion();
        expect(QStringLiteral("closing again asks again"), box && box->isVisible());
        if (box)
            clickButton(box, "Stop it and close");
        waitFor([&] { return !other.isVisible(); }, 20);
        settle(200);
        expect(QStringLiteral("stopping it ended the install, and the window closed"),
               !other.isVisible() && processesRunning("install levitate:htop --machine-events") == 0);
    }
    {
        qputenv("RECIPE_STANDIN_STOP_AT", "thunderbird:remove");
        const int removes = timesAsked({"remove", "thunderbird"});
        Window other;
        other.show();
        waitUntilIdle(other, 30);
        const QString shown = other.catalog().recipe("thunderbird") ? other.catalog().recipe("thunderbird")->shown() : QStringLiteral("thunderbird");
        other.removeNow("thunderbird", Place::Computer);
        waitFor([&] { return timesAsked({"remove", "thunderbird"}) > removes; }, 30);
        settle(400);
        other.close();
        settle(300);
        QPointer<QMessageBox> box = other.closeQuestion();
        expect(QStringLiteral("closing during a remove asks too, and says what stopping it leaves"),
               box && box->isVisible() && wordsOn(*box).contains(QStringLiteral("Removing %1 is still running.").arg(shown))
                   && wordsOn(*box).contains(QStringLiteral("Stopping it before it ends leaves %1 installed. You can remove it again from Installed.").arg(shown)));
        if (box) {
            save(*box, "19b-close-remove");
            clickButton(box, "Stop it and close");
        }
        waitFor([&] { return !other.isVisible(); }, 20);
        settle(200);
        bool still = false;
        for (const QJsonValue &each : answerOf({"installed", "--json"})["packages"].toArray())
            still = still || each.toObject()["name"] == "thunderbird";
        expect(QStringLiteral("it stopped and the window closed; Thunderbird is still installed"),
               !other.isVisible() && still && processesRunning("remove thunderbird --machine-events") == 0);
        qunsetenv("RECIPE_STANDIN_STOP_AT");
    }
    window.showBrowse();
    settle(100);
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
        // Storage and the history, against what `recipe storage --json` and `recipe history --json`
        // print for the same store; and the held flag of every installed package against `installed --json`.
        window.showStorage();
        ok &= save(window, "real-8-storage");
        const QString shown = wordsOn(window);
        const QJsonObject storage = answerOf({"storage", "--json"});
        const QJsonArray history = answerOf({"history", "--json"})["history"].toArray();
        const QStringList lines = window.storagePage()->historyLines();
        auto check = [&](const QString &what, bool holds) {
            out << (holds ? "same: " : "DIFFERENT: ") << what << "\n";
            ok &= holds;
        };
        out << "recipe storage: active " << storage["active_bytes"].toInteger() << ", earlier " << storage["earlier_bytes"].toInteger()
            << " in " << storage["earlier_generations"].toInt() << "\n";
        check(QStringLiteral("Storage shows the earlier versions' size and generations"),
              shown.contains(bytesText(storage["earlier_bytes"].toInteger()))
                  && shown.contains(counted(storage["earlier_generations"].toInt(), QStringLiteral("%1 earlier generation"),
                                            QStringLiteral("%1 earlier generations"))));
        check(QStringLiteral("Storage shows what is installed takes %1").arg(bytesText(storage["active_bytes"].toInteger())),
              shown.contains(bytesText(storage["active_bytes"].toInteger())));
        out << "recipe history: " << history.size() << " entries, newest " << history.first().toObject()["what"].toString() << "\n";
        check(QStringLiteral("the history lists %1 entries, newest first (%2)").arg(history.size()).arg(lines.join(" | ")),
              lines.size() == history.size());
        const QJsonArray packages = answerOf({"installed", "--json"})["packages"].toArray();
        for (const QJsonValue &value : packages) {
            const QJsonObject package = value.toObject();
            const InstalledItem *item = window.catalog().installedItem(package["name"].toString());
            out << "recipe installed: " << package["name"].toString() << " " << package["version"].toString()
                << (package["held"].toBool() ? " held" : "") << "\n";
            check(QStringLiteral("%1 is %2 as `installed --json` says").arg(package["name"].toString(), package["held"].toBool() ? "held" : "not held"),
                  item && item->held == package["held"].toBool() && item->version == package["version"].toString());
        }
        window.showInstalled();
        const QString installedWords = wordsOn(window);
        check(QStringLiteral("Installed marks the held package"),
              !installedWords.isEmpty() && (installedWords.contains("kept at this version")
                                           == std::any_of(packages.begin(), packages.end(), [](const QJsonValue &each) { return each.toObject()["held"].toBool(); })));
        ok &= save(window, "real-9-installed-held");
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
    bool ok_ = true; // the guard's check, before the first picture
    QDir().mkpath(folder);
    auto save = [&folder](QWidget &widget, const QString &name) {
        settle(200);
        return widget.grab().save(QDir(folder).filePath(name + ".png"));
    };
    window.show();
    // A picture run that is not told of its stand-ins refuses to start.
    {
        auto refused = [&](const QString &what, bool holds) {
            if (!holds) {
                QTextStream(stdout) << "not as expected: " << what << "\n";
                ok_ = false;
            }
        };
        livestory::guardRefuses(refused, folder, "RECIPE_GUI_RECIPE");
    }
    // Before recipe has answered: what a person sees while the catalog is read.
    bool ok = ok_ && window.grab().save(QDir(folder).filePath("0-reading.png"));
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
    // Until the stand-in recipe has been asked, and has stopped where it was told to.
    waitFor([&] { return changesAsked().size() > beforeInstall; }, 30);
    settle(300);
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
    {
        // The refused connection: recipe reports a numeric address, and the page words it as one.
        const QString words = wordsOn(window);
        QRegularExpression address(QStringLiteral(R"(the address (\d+\.\d+\.\d+\.\d+), port (\d+), which is not on the recipe's list)"));
        const bool numeric = address.match(words).hasMatch();
        if (!numeric || words.contains("cdn.example.net")) {
            said << "not as expected: the stopped run names the address recipe reports, in words, not a host name\n";
            ok = false;
        }
        if (!words.contains("Recipe reports the address the recipe tried to reach, not a host name.")) {
            said << "not as expected: the facts say that the address is not a host name\n";
            ok = false;
        }
        // `recipe update` takes no `--autofix` (cli.rs): a stopped update offers no repair, and says so.
        if (words.contains("Let the agent fix it")
            || !words.contains("Recipe lets its agent repair a recipe only while installing it")) {
            said << "not as expected: a stopped update offers no agent repair, and says why\n";
            ok = false;
        }
        // Where recipe keeps the log is a fact of the stop; the output itself is under "Show the output".
        if (!words.contains("Recipe keeps the log and the retained record of this run in ")
            || words.contains("of the run's output")) {
            said << "not as expected: a stopped update names the log and keeps the output behind its button\n";
            ok = false;
        }
    }

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

    // Recipe's repair belongs to an install: a first install the sandbox refuses, handed to
    // Recipe's agent with `recipe install --autofix`.
    qputenv("RECIPE_STANDIN_REFUSE_NEW", "krita");
    window.startInstall("krita", Place::Computer);
    waitUntilIdle(window);
    if (!wordsOn(window).contains("Let the agent fix it")) {
        said << "not as expected: a stopped install offers Let the agent fix it\n";
        ok = false;
    }
    {
        // The agent is given the last lines of the run's output and where recipe keeps its log, not a count.
        const QString facts = wordsOn(window);
        const bool given = facts.contains("The last 2 lines of the run's output (2 lines in all):")
                           && facts.contains("recipe: fetching krita 5.2.13 from")
                           && facts.contains("Recipe keeps the log and the retained record of this run in ")
                           && !facts.contains("The output of the run: ");
        if (!given) {
            said << "not as expected: the facts for the agent carry the output's last lines and the log's place\n";
            ok = false;
        }
        ok &= save(window, "8a-agent-facts");
        const qsizetype beforeFix = changesAsked().size();
        clickButton(window.runPage(), "Let the agent fix it");
        waitUntilIdle(window);
        QFile handed(QDir(qEnvironmentVariable("RECIPE_STANDIN_STATE")).filePath("last-autofix-prompt.txt"));
        const QString prompt = handed.open(QIODevice::ReadOnly) ? QString::fromUtf8(handed.readAll()) : QString();
        if (!prompt.contains("The last 2 lines of the run's output (2 lines in all):\n    recipe: diagnostics: ")
            || !prompt.contains("recipe: fetching krita 5.2.13 from")
            || !prompt.contains("Recipe keeps the log and the retained record of this run in " + qEnvironmentVariable("RECIPE_STANDIN_STATE") + "/logs/")) {
            said << "not as expected: the prompt file handed to recipe holds the last lines and the log's place\n";
            ok = false;
        }
        const QList<Asked> now = changesAsked().mid(beforeFix);
        if (now.size() != 1 || now[0].arguments.mid(0, 4) != QStringList{"install", "levitate:krita", "--machine-events", "--autofix"}
            || !window.catalog().installedItem("krita", Place::Computer)) {
            said << "not as expected: the repair is `recipe install NAME --machine-events --autofix ...`, and it installs\n";
            ok = false;
        }
    }
    ok &= save(window, "8-agent-fixed");
    qunsetenv("RECIPE_STANDIN_REFUSE_NEW");
    // Element's source has since published a recipe that stays on its list: the update goes through.
    qputenv("RECIPE_STANDIN_FIXED", "element");
    window.startInstall("element", Place::Computer);
    waitUntilIdle(window);

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
    const Origin *ownFolder = window.catalog().origin("local");
    {
        QStringList words;
        if (auto *page = window.findChild<WishPage *>())
            for (QLabel *label : page->findChildren<QLabel *>())
                words << label->text();
        expect(QStringLiteral("the ask page says the agent may write in the person's own recipe folder only (%1)")
                   .arg(ownFolder ? ownFolder->path : QString()),
               ownFolder && words.join('\n').contains("may write in your own recipe folder only (" + ownFolder->path + ")."));
    }
    window.askAgent(QStringLiteral("The request:\na file manager for the terminal\n\nThe issue:\nNo recipe for it "
                                   "exists.\n\nThe expected result:\nOne recipe that recipe info accepts.\n"));
    for (int waited = 0; !window.agentAsks() && window.busy() && waited < 1800; ++waited)
        settle(50);
    expect("the agent asks which one", window.agentAsks());
    ok &= save(window, "12-the-agent-asks");
    {
        // What Activity will list the run under was handed to the agent.
        QFile sent(QDir(qEnvironmentVariable("RECIPE_STANDIN_STATE")).filePath("last-prompt.json"));
        const QJsonObject said = sent.open(QIODevice::ReadOnly) ? QJsonDocument::fromJson(sent.readAll()).object() : QJsonObject();
        expect("the agent is told this is Recipe, and the first line of the request as the title",
               said["app"].toString() == "Recipe" && said["title"].toString() == "a file manager for the terminal");
        expect("the agent is allowed to write in the person's own recipe folder, which was made first",
               ownFolder && said["may_write"].toArray() == QJsonArray{ownFolder->path} && QDir(ownFolder->path).exists());
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
    {
        auto save_ = [&](QWidget &widget, const QString &name) { ok &= save(widget, name); };
        programPages(window, save_, expect);
        storageBefore(window, save_, expect);
    }
    window.clearEarlier(Place::Computer, false);
    waitUntilIdle(window);
    expect("clearing leaves no earlier version and everything installed",
           window.catalog().earlierGenerations == 0 && window.catalog().installed.size() == installedBefore
               && window.catalog().installedItem("gimp")->kept.isEmpty());
    ok &= save(window, "14-earlier-versions-cleared");
    {
        auto save_ = [&](QWidget &widget, const QString &name) { ok &= save(widget, name); };
        storageAfter(window, save_, expect);
    }

    // The frame: the rail, the strip about the next system, Activity.
    auto saved = [&](QWidget &widget, const QString &name) { ok &= save(widget, name); };
    auto held = [&](const QString &what, bool holds) { expect(what, holds); };
    frameShots(window, saved, held);
    holdShots(window, saved, held);
    gapShots(window, saved, held);
    settingsShots(window, folder, saved, held);
    helpShots(window, folder, saved, held);
    stripShots(window, folder, saved, held, out);
    placesShots(folder, saved, held);
    out << (ok ? "all flows ended where they should\n" : "some flows did not end where they should\n");
    return ok ? 0 : 1;
}
