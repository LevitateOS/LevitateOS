// The agent's runs from all three apps, as `levitate-agent` kept them: a list,
// and for the chosen run what was asked, what it did and how it ended. A run
// that is still going is followed live (`levitate-agent --attach`): what it did
// so far, what it does now, its question with the choices as buttons, and Stop.
// The list is read again every two seconds while the page is on screen, so the
// counts follow runs of other windows. The frame owns the page; an app does
// not build it.
#pragma once

#include "page.h"

#include <QJsonArray>
#include <QJsonObject>
#include <QMap>
#include <QPointer>

class QLabel;
class QLineEdit;
class QListWidget;
class QPushButton;
class QScrollArea;
class QTimer;
class Run;

class ActivityPage : public Page {
    Q_OBJECT
public:
    // `app`: this window's app as the runs name it ("Recab"); "This app"
    // in the filter means the runs of that app.
    explicit ActivityPage(const QString &app, QWidget *parent = nullptr);

    // Reads the list (and the chosen run) again without holding the window up.
    // The first time the page says so while it waits.
    void read();
    // The id of the run this window started and follows, or empty: "Go to it".
    void setOwnRun(const QString &id);

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
    // Following the chosen run live: attached, and what it has said so far was read.
    bool following() const { return attached_ && live_; }
    // The question of the followed run that waits, empty when none does.
    QString askedId() const { return asked_; }
    // The buttons of the question shown (their words), for a check.
    QStringList answerButtons() const;
    // As a click on the choice (or the typed answer) would; false when no question is shown.
    bool answer(const QString &words);
    // As a click on "Stop"; false when the chosen run cannot be stopped from here.
    bool stopChosen();
    bool offersStop() const;
    // Filter and choice, as a click would.
    void showOnlyThisApp(bool only);
    void choose(const QString &id);
    void chooseTitle(const QString &title);
    bool offersGoTo() const;

protected:
    void showEvent(QShowEvent *event) override;
    void hideEvent(QHideEvent *event) override;

signals:
    // Runs that wait for an answer or run, over all apps, for the rail's badge.
    void counts(int active, bool waits);
    void goToRun();

private:
    void render();
    void fetchDetail();
    void showDetail();
    QJsonArray filtered() const;
    // Following the chosen run: attach when it goes and can be reached, let go
    // when it is not shown or ended.
    void follow(const QJsonObject &summary);
    void detach();
    void onLive(const QJsonObject &event);
    void liveEnded();
    void rememberTyping();

    QString app_, program_;
    Run *listing_ = nullptr, *showing_ = nullptr;
    QJsonArray runs_, lines_;
    QString problem_, chosen_, detailId_;
    bool loaded_ = false, onlyThis_ = false, detailBroken_ = false;
    int waiting_ = 0, running_ = 0;
    QString own_;
    int listScroll_ = 0;
    QListWidget *list_ = nullptr;
    QScrollArea *detail_ = nullptr;
    QTimer *poll_ = nullptr;
    // The followed run: the client attached to it, what it said (the request
    // first, then the events as the record holds them, without `output`), and
    // what it does now.
    Run *attached_ = nullptr, *stopping_ = nullptr;
    QString attachedId_, asked_, sentFor_, stepNow_;
    QJsonArray liveLines_;
    QStringList tail_;
    bool live_ = false;          // liveLines_ holds the request and the events so far
    QPointer<QLabel> stepLabel_, tailLabel_;
    QPointer<QLineEdit> typing_;
    QString typed_;
    bool typingFocused_ = false;
    int detailScroll_ = 0;
    QString answeredWords_, stopProblem_, typedFor_;
    QStringList endedIds_;      // runs whose follow ended: not attached again from a list that is a moment old
    QMap<QString, QString> stepTitles_;
    QTimer *redraw_ = nullptr;
};
