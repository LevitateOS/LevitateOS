// The window every app of the three shares: a rail on the left (the three
// apps, then the app's own entries, then Activity), the app's pages in the
// middle, a strip under them about the next system, and a status line. An
// app's window uses it as its central widget and keeps its own pages in one
// widget it hands over with `setContent`. See README.md for an example.
#pragma once

#include <QMap>
#include <QStringList>
#include <QWidget>

class ActivityPage;
class ElidedLabel;
class QHBoxLayout;
class QLabel;
class QPushButton;
class QStackedWidget;
class QTimer;
class RailButton;
class Run;
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

    // ---- Activity, which the frame owns.
    ActivityPage *activity() const { return activity_; }
    // The agent process this window started and waits for, or 0. Activity
    // offers "Go to it" for that run; setting 0 (the run ended) reads again.
    void setOwnRun(qint64 pid);

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

private:
    void mark(const QString &key);
    void step(RailButton *from, int by);
    void showAgent(const QString &words);
    void readAgentStatus();

    App app_;
    QString selected_, followed_;
    QStackedWidget *stack_;
    ActivityPage *activity_;
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
    bool stripAllowed_ = true;
};
