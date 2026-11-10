// The agent's runs from all three apps, as `levitate-agent` kept them: a list,
// and for the chosen run what was asked, what it did and how it ended. The
// frame owns one; an app does not build it.
#pragma once

#include "page.h"

#include <QJsonArray>

class QListWidget;
class QScrollArea;
class Run;

class ActivityPage : public Page {
    Q_OBJECT
public:
    // `app`: this window's app as the runs name it ("System"); "This app"
    // in the filter means the runs of that app.
    explicit ActivityPage(const QString &app, QWidget *parent = nullptr);

    // Reads the list (and the chosen run) again without holding the window up.
    // The first time the page says so while it waits.
    void read();
    // The agent process this window started and waits for, or 0.
    void setOwnRun(qint64 pid);

    // What the last reading found, over all apps.
    int waiting() const { return waiting_; }
    int running() const { return running_; }
    bool loaded() const { return loaded_; }
    // What the page shows, for a check: the runs listed (after the filter),
    // the id of the chosen one, and the words of the detail.
    QStringList listedTitles() const;
    // The line of state under each title, in the same order.
    QStringList listedStates() const;
    QString chosenId() const { return chosen_; }
    QString detailText() const;
    QString problem() const { return problem_; }
    // Filter and choice, as a click would.
    void showOnlyThisApp(bool only);
    void choose(const QString &id);
    void chooseTitle(const QString &title);
    bool offersGoTo() const;

signals:
    // Runs that wait for an answer or run, over all apps, for the rail's badge.
    void counts(int active, bool waits);
    void goToRun();

private:
    void render();
    void fetchDetail();
    void showDetail();
    QJsonArray filtered() const;

    QString app_, program_;
    Run *listing_ = nullptr, *showing_ = nullptr;
    QJsonArray runs_, lines_;
    QString problem_, chosen_, detailId_;
    bool loaded_ = false, onlyThis_ = false, detailBroken_ = false;
    int waiting_ = 0, running_ = 0;
    qint64 own_ = 0;
    int listScroll_ = 0;
    QListWidget *list_ = nullptr;
    QScrollArea *detail_ = nullptr;
};
