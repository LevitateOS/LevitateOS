#include "helpstory.h"

#include "firstrun.h"
#include "frame.h"
#include "help.h"
#include "look.h"

#include <QDir>
#include <QFile>
#include <QMap>
#include <QSettings>

#include <optional>

namespace helpstory {

namespace {

// Environment a check changes, given back as it was.
class Env {
public:
    void set(const char *name, const QString &value)
    {
        keep(name);
        qputenv(name, value.toUtf8());
    }
    ~Env()
    {
        for (auto it = was_.begin(); it != was_.end(); ++it) {
            if (it.value())
                qputenv(it.key().constData(), *it.value());
            else
                qunsetenv(it.key().constData());
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

QString appWord(Frame::App app)
{
    return app == Frame::Recipe ? QStringLiteral("Recipe") : app == Frame::System ? QStringLiteral("System")
                                                                                 : QStringLiteral("Disk");
}

QString appVariable(Frame::App app)
{
    return app == Frame::Recipe ? QStringLiteral("LEVITATE_APP_RECIPE")
           : app == Frame::System ? QStringLiteral("LEVITATE_APP_SYSTEM")
                                   : QStringLiteral("LEVITATE_APP_DISK");
}

} // namespace

void run(const Plan &plan, const Save &save, const Expect &expect, const QString &prefix)
{
    Frame *frame = plan.frame;
    const Frame::App app = plan.app;
    const HelpText text = helpText(app);
    const QString name = text.name;

    // ---- The rail
    const QStringList titles = frame->entryTitles();
    expect(QStringLiteral("the rail has Help under Settings, under Activity (%1)").arg(titles.join(QStringLiteral(", "))),
           titles.size() >= 3 && titles.mid(titles.size() - 3) == QStringList({"Activity", "Settings", "Help"}));

    // ---- The page
    frame->choose(Frame::helpKey());
    plan.idle();
    expect(QStringLiteral("Help is showing and marked, and no other page of the frame is"),
           frame->helpShown() && frame->selected() == Frame::helpKey() && !frame->settingsShown() && !frame->activityShown()
               && !frame->firstRunShown());
    HelpPage *help = frame->help();
    const QString shown = help->pageText();
    for (const QString &words :
         {QStringLiteral("Help"), QStringLiteral("What %1 is for").arg(name), text.purpose, text.never,
          QStringLiteral("The ideas you need"), QStringLiteral("How do I…"), QStringLiteral("The agent"),
          QStringLiteral("What it may do without asking"), QStringLiteral("What it asks about"), text.agentInThisApp,
          QStringLiteral("When something is wrong"), QStringLiteral("The other two apps"),
          QStringLiteral("Show the first page again"), QStringLiteral("Change this in Settings"),
          QStringLiteral("See what the agent did in Activity")})
        expect(QStringLiteral("the Help page says: %1").arg(words.left(60)), shown.contains(words));
    for (const HelpIdea &idea : text.ideas)
        expect(QStringLiteral("the Help page has the idea \"%1\"").arg(idea.title),
               shown.contains(idea.title) && shown.contains(idea.first) && shown.contains(idea.more.left(40)));
    for (const HelpRow &row : text.how)
        expect(QStringLiteral("the Help page has \"%1\" with its link").arg(row.title),
               shown.contains(row.title) && shown.contains(row.words) && help->links().contains(QStringLiteral("Go to %1").arg(row.page)));
    for (const HelpRow &row : text.wrong)
        expect(QStringLiteral("the Help page has \"%1\"").arg(row.title), shown.contains(row.title) && shown.contains(row.words));
    expect(QStringLiteral("it offers to open the other two apps and not this one"),
           shown.contains(QStringLiteral("Open %1").arg(appWord(text.others[0].first)))
               && shown.contains(QStringLiteral("Open %1").arg(appWord(text.others[1].first)))
               && !shown.contains(QStringLiteral("Open %1").arg(name)));
    // No thing the app has not got: the words below are what the README lists as not done or not true.
    for (const char *untrue : {"Search this page", "Ctrl+K", "recipe-gui --show", "disk-gui ~/", "Go to anything", "cleared through Recipe"})
        expect(QStringLiteral("the page does not say: %1").arg(QLatin1String(untrue)), !shown.contains(QLatin1String(untrue)));
    plan.idle();
    save(*plan.window, prefix + "-help");
    // The page scrolls: a picture of each screen of it.
    for (int screen = 2; screen <= 8 && help->scrollDown(); ++screen)
        save(*plan.window, QStringLiteral("%1-help-%2").arg(prefix).arg(screen));

    // ---- Every link goes where it says
    int links = 0;
    QStringList words;
    for (const HelpRow &row : text.how + text.wrong)
        if (!row.target.isEmpty())
            words << QStringLiteral("Go to %1").arg(row.page);
    expect(QStringLiteral("the page has one link for each row that has a page (%1)").arg(words.join(QStringLiteral(", "))),
           help->links().size() == words.size() + 2);
    auto landed = [&](const QString &target) {
        if (target == Frame::activityKey())
            return frame->activityShown();
        if (target == Frame::settingsKey())
            return frame->settingsShown();
        return frame->selected() == target && !frame->helpShown() && plan.pageShown(target);
    };
    QList<QPair<QString, QString>> goes; // link words, target
    for (const HelpRow &row : text.how + text.wrong)
        if (!row.target.isEmpty())
            goes << qMakePair(QStringLiteral("Go to %1").arg(row.page), row.target);
    goes << qMakePair(QStringLiteral("Change this in Settings"), Frame::settingsKey())
         << qMakePair(QStringLiteral("See what the agent did in Activity"), Frame::activityKey());
    for (const auto &go : goes) {
        frame->choose(Frame::helpKey());
        plan.idle();
        const bool pressed = frame->help()->press(go.first);
        plan.idle();
        // A page the app offered is not a rail entry: the frame has left Help for the app's own page.
        const bool there = go.second == QLatin1String("ask") ? (!frame->helpShown() && plan.pageShown(go.second)) : landed(go.second);
        expect(QStringLiteral("\"%1\" goes to %2 (pressed %3, marked %4)").arg(go.first, go.second).arg(pressed).arg(frame->selected()),
               pressed && there);
        ++links;
        if (links == 2)
            save(*plan.window, prefix + "-help-went");
    }

    // ---- The other apps' buttons, and the first page again
    {
        Frame::App other = text.others[0].first;
        frame->choose(Frame::helpKey());
        frame->flash(QString());
        const bool pressed = frame->help()->press(QStringLiteral("Open %1").arg(appWord(other)));
        // Named in the environment it is started (a stand-in); not named, a picture run says it does not start it.
        const bool named = qEnvironmentVariableIsSet(appVariable(other).toLatin1().constData());
        expect(QStringLiteral("\"Open %1\" asks the frame to start it (%2)").arg(appWord(other), frame->statusText()),
               pressed && (named ? !frame->statusText().contains(QStringLiteral("could not be started"))
                                 : frame->statusText().contains(QStringLiteral("%1 is not started in a picture run").arg(appWord(other)))));
    }
    frame->choose(Frame::helpKey());
    expect(QStringLiteral("\"Show the first page again\" shows the first page"),
           frame->help()->press(QStringLiteral("Show the first page again")) && frame->firstRunShown());
    // It asked the helper as it was shown; what follows must be asked afresh, so that one has ended.
    waitFor([&] { return !frame->firstRun()->busy(); }, 30);
    // Going on from the first page returns to the page the app is on: here, the one it opens on.
    frame->choose(plan.firstEntry);
    plan.idle();
    frame->flash(QString());

    // ---- The first page, in the three states of the agent
    const QString home = QDir(plan.folder).filePath(prefix + "-firstrun-home");
    QDir(home).removeRecursively();
    QDir().mkpath(home);
    Env env;
    env.set("HOME", home);
    env.set("XDG_CONFIG_HOME", home + "/config");
    env.set("XDG_STATE_HOME", home + "/state");
    env.set("LEVITATE_AGENT_CODEX", QDir(plan.fixtures).filePath("codex"));
    env.set("LEVITATE_AGENT_CLAUDE", QDir(plan.fixtures).filePath("claude"));
    env.set("LEVITATE_STANDIN_LOG", home + "/standin.log");
    auto signedAs = [&](const char *state) {
        QFile file(home + "/.standin-codex-signed");
        if (file.open(QIODevice::WriteOnly | QIODevice::Truncate))
            file.write(state);
    };
    auto asked = [&](const QString &picture) {
        frame->showFirstRun();
        waitFor([&] { return !frame->firstRun()->busy() && frame->firstRun()->agent() != FirstRunPage::Asking; }, 30);
        plan.idle();
        save(*plan.window, picture);
        return frame->firstRun()->pageText();
    };

    FirstRunPage *page = frame->firstRun();
    signedAs("in");
    QString said = asked(prefix + "-firstrun-ready");
    expect(QStringLiteral("the first page is showing and the rail marks nothing (%1)").arg(frame->selected()),
           frame->firstRunShown() && !frame->helpShown());
    for (const QString &words : {QStringLiteral("Welcome to %1").arg(name), text.purpose, text.never,
                                 QStringLiteral("Worth knowing first"), QStringLiteral("The agent"),
                                 QStringLiteral("Go on to %1").arg(name), QStringLiteral("Read Help first")})
        expect(QStringLiteral("the first page says: %1").arg(words.left(60)), said.contains(words));
    for (const HelpIdea &idea : text.ideas)
        expect(QStringLiteral("the first page %1 the idea \"%2\" (its first sentence only)")
                   .arg(idea.forFirstRun ? "has" : "leaves out").arg(idea.title),
               idea.forFirstRun ? said.contains(idea.title) && said.contains(idea.first) && !said.contains(idea.more.left(40))
                                : !said.contains(idea.title));
    expect(QStringLiteral("a ready agent is said so (%1)").arg(said.section(QLatin1Char('\n'), 0, 0)),
           page->agent() == FirstRunPage::Ready && said.contains(QStringLiteral("An agent is ready: Codex is installed and signed in."))
               && !said.contains(QStringLiteral("in Settings")));

    signedAs("out");
    said = asked(prefix + "-firstrun-not-signed-in");
    expect(QStringLiteral("an agent that is not signed in is said so, with the way to sign in"),
           page->agent() == FirstRunPage::NotSignedIn
               && said.contains(QStringLiteral("No agent is ready yet: Codex is installed, but not signed in."))
               && said.contains(QStringLiteral("Sign in in Settings")));
    expect(QStringLiteral("its link goes to Settings"), page->press(QStringLiteral("Sign in in Settings")) && frame->settingsShown());

    signedAs("in");
    env.set("LEVITATE_AGENT_CODEX", QStringLiteral("/nonexistent/codex"));
    said = asked(prefix + "-firstrun-not-installed");
    expect(QStringLiteral("an agent that is not installed is said so, with the way to choose another"),
           page->agent() == FirstRunPage::NotInstalled && said.contains(QStringLiteral("No agent is ready yet: Codex is not installed."))
               && said.contains(QStringLiteral("Choose an agent in Settings")));
    env.set("LEVITATE_AGENT_CODEX", QDir(plan.fixtures).filePath("codex"));

    // ---- Help from the first page, and going on to the app
    expect(QStringLiteral("\"Read Help first\" shows Help"), page->press(QStringLiteral("Read Help first")) && frame->helpShown());
    asked(prefix + "-firstrun-again");
    const bool went = page->press(QStringLiteral("Go on to %1").arg(name));
    plan.idle();
    expect(QStringLiteral("the one button goes on to the app's own page (%1)").arg(frame->selected()),
           went && !frame->firstRunShown() && frame->selected() == plan.firstEntry && plan.pageShown(plan.firstEntry));
    save(*plan.window, prefix + "-firstrun-went-on");

    // ---- Picture runs keep nothing about it
    QSettings kept(QStringLiteral("LevitateOS"), QStringLiteral("firstrun"));
    expect(QStringLiteral("a picture run neither wants the page nor wrote that it was seen (%1)")
               .arg(kept.allKeys().join(QStringLiteral(", "))),
           !FirstRunPage::wanted(app) && kept.allKeys().isEmpty());
}

} // namespace helpstory
