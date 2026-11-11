// The window every app of the three shares: a rail on the left (the three
// apps, then the app's own entries, then Activity), the app's pages in the
// middle, a strip under them about the next system, and a status line. An
// app's window uses it as its central widget and keeps its own pages in one
// widget it hands over with `setContent`. See README.md for an example.
#pragma once

#include <QMap>
#include <QPointer>
#include <QStringList>
#include <QWidget>

class ActivityPage;
class ElidedLabel;
class FirstRunPage;
class HelpPage;
class QHBoxLayout;
class QLabel;
class QPushButton;
class QStackedWidget;
class QTimer;
class RailButton;
class Run;
class SettingsPage;
class Strip;

class Frame : public QWidget {
    Q_OBJECT
public:
    enum App { Recipe, System, Disk };

    // `app` is the app this window is: its button in the rail is the marked one.
    explicit Frame(App app, QWidget *parent = nullptr);

    // ---- The middle. The widget (usually a QStackedWidget of the app's pages)
    // is shown whenever Activity is not.
    void setContent(QWidget *content);

    // ---- The rail. Entries are the app's own: they come after the three apps
    // and before Activity, in the order added. Only Activity is shared.
    void addEntry(const QString &key, const QString &title);
    // The number on an entry (0 shows none); `warn` makes it amber, for
    // something that waits for the person.
    void setBadge(const QString &key, int count, bool warn = false);
    // The person's choice, made in code: marks the entry and shows the
    // content. Does not emit `chosen`.
    void select(const QString &key);
    // The app's page changed by itself (an agent finished, a flow moved on):
    // marks the entry of that page, and leaves Activity showing when it is
    // showing. Does not emit `chosen`.
    void follow(const QString &key);
    // As a click on the entry would: shows it, marks it and emits `chosen`.
    void choose(const QString &key);
    // The number and colour on an entry now.
    int badge(const QString &key) const;
    bool badgeWarns(const QString &key) const;
    // The key of the marked entry (`activityKey()` while Activity shows).
    QString selected() const { return selected_; }
    bool activityShown() const;
    static QString activityKey() { return QStringLiteral("activity"); }
    // Settings is the frame's too: the rail shows it under Activity, and choosing it
    // does not emit `chosen`, since there is nothing for the app to switch to.
    bool settingsShown() const;
    static QString settingsKey() { return QStringLiteral("settings"); }
    // Help is the frame's as well, under Settings. One page for each app (the
    // frame knows which it is). Its links go to the app's rail entries, to
    // Activity and Settings through `choose`, and to pages the app offered with
    // `addHelpTarget` through `helpLink`.
    bool helpShown() const;
    static QString helpKey() { return QStringLiteral("help"); }
    // A page of the app that is not on the rail and that Help may lead to
    // ("ask"). Help has no entry for a page that is not a rail entry, Activity,
    // Settings or one of these. When it is followed, the frame has already left
    // Help for the app's content and says `helpLink(key)`: the app shows the page.
    void addHelpTarget(const QString &key);

    // ---- The first time. The page before the app's own, shown once per person
    // and per app (QSettings, never in a picture run, where a story asks for it
    // with `showFirstRun`). The rail marks nothing while it shows. Its button
    // goes on to the app's own page; choosing anything in the rail leaves it too.
    void showFirstRun();
    bool firstRunShown() const;
    FirstRunPage *firstRun() const { return firstRun_; }
    HelpPage *help() const { return help_; }

    // ---- The strip: "The next system holds N changes." with a button that
    // finishes it. The titles are the changes' own ("htop, installed"); an
    // empty list hides the strip.
    void setNextSystem(const QStringList &titles);
    // The app may keep the strip away from the page that already says it.
    void setStripAllowed(bool allowed);
    bool stripShown() const;
    QString stripText() const;

    // ---- The status line. Left, the app's own words; right, the agent.
    void setStatus(const QString &words);
    // Something shown beside the words ("Read again"), in the order added.
    void addStatusWidget(QWidget *widget);
    // Said in the status line for a few seconds, in place of the app's words.
    // An empty text ends a flash at once.
    void flash(const QString &words);
    // The agent the app starts. `levitate-agent` is asked for its status
    // (`--status`) without holding anything up; any other program is shown by
    // its file name, as a stand-in is.
    void setAgent(const QString &program);
    // While the agent works, words in place of its name ("The agent asks...");
    // empty puts the name back.
    void setAgentNote(const QString &words);
    QString agentText() const;
    QString statusText() const;

    // Starts an app's program detached, exactly as choosing it in the rail
    // does (the same variables); the status line says when it cannot be started.
    void startApp(App app);
    // The titles of the rail's entries in order, the app's own and then Activity,
    // Settings and Help, without the three apps.
    QStringList entryTitles() const;

    // ---- Activity, which the frame owns.
    ActivityPage *activity() const { return activity_; }
    // ---- Settings, which the frame owns as well.
    SettingsPage *settings() const { return settings_; }
    // The id of the agent run this window started and follows, or empty.
    // Activity offers "Go to it" for that run; setting empty (the run ended)
    // reads Activity again. A run that is not an agent's is ignored. The
    // helper names the run's id a moment after the start, so this waits for
    // it.
    void setOwnRun(Run *run);
    void setOwnRun(const QString &id);
    QString ownRun() const { return ownRun_; }

public slots:
    // Reads Activity and the agent's status again. Connect the app's
    // `Refresher::asked` to it: F5, Ctrl+R and coming back to the window.
    void readAgain();

signals:
    // The person chose an app entry or Activity (`activityKey()`).
    void chosen(const QString &key);
    // The strip's button: finish the next system. The app decides where that goes.
    void finishRequested();
    // "Go to it" on this window's own live run: show the app's work page.
    void goToRun();
    // A link of Help to a page the app offered (`addHelpTarget`). Help has been left.
    void helpLink(const QString &key);

protected:
    bool eventFilter(QObject *watched, QEvent *event) override;
    bool event(QEvent *event) override;
    void showEvent(QShowEvent *event) override;
    void closeEvent(QCloseEvent *event) override;

private:
    // A window that closes lets go of the agent runs it follows; they go on.
    void watchWindow();
    void letGoOfRuns();
    void mark(const QString &key);
    void step(RailButton *from, int by);
    void showAgent(const QString &words);
    void readAgentStatus();
    // The shared pages the frame shows itself, and how a key finds its place in the stack.
    int indexOf(const QString &key) const;
    void helpGo(const QString &target);
    void leaveShared();

    App app_;
    QString selected_, followed_;
    QStackedWidget *stack_;
    ActivityPage *activity_;
    SettingsPage *settings_;
    HelpPage *help_;
    FirstRunPage *firstRun_;
    QStringList helpTargets_;
    bool firstRunChecked_ = false;
    QWidget *appsRow_;
    QMap<QString, RailButton *> entries_;
    QList<RailButton *> order_;
    QWidget *appEntries_;
    Strip *strip_;
    ElidedLabel *status_;
    QHBoxLayout *statusExtras_;
    QLabel *agent_;
    QTimer *flashTimer_;
    QString statusWords_, flashWords_, agentName_, agentNote_, agentProgram_;
    Run *asking_ = nullptr;
    bool stripAllowed_ = true, askAgain_ = false;
    QString ownRun_;
    bool ownNoted_ = false; // "carries on" was said for this run
    QList<QPointer<Run>> ownRuns_;
    QPointer<QWidget> watched_;
};
