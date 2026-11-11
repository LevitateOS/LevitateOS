// The one window: the pages and the flows between them.
#pragma once

#include "client.h"
#include "model.h"

#include <QMainWindow>

#include <functional>

class AskPage;
class ClashPage;
class HistoryPage;
class ReleasePage;
class ConfirmDialog;
class Frame;
class NextPage;
class BackPage;
class ProposedPage;
class Refresher;
class QStackedWidget;
class SystemPage;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    // Overview; the next system's page; Getting back.
    void showMain();
    void showNext();
    void showBack();
    // What happened to the systems (read from recab each time), and a new release.
    void showHistory();
    void showRelease();
    // The page the person was on before a flow took them elsewhere.
    void showSection();
    void showAsk(const QString &request = QString());
    void showClash();

    // Each of these asks first unless told not to (the picture mode does not).
    void handOver(const QString &prompt, bool askFirst = true);
    void addUpdate(bool askFirst = true);
    void goBack(bool askFirst = true);
    void keep();
    // Sets the next system for the next restart without restarting; and takes that back.
    void restartLater();
    void cancelRestart();
    void discard(bool askFirst = true);
    // Discards the next system, then makes it a copy of the running one again.
    void reset(bool askFirst = true);
    void finish();
    void restart();
    void recover();
    void fixWithAgent(const QString &prompt, const QString &title);
    void acceptProposal();
    void stopRun();
    // Starts Recipe, where software is installed into this same next system.
    void installSoftware();
    // Writes the steps for a desktop that does not start, as plain text.
    bool saveSteps(const QString &path);

    // The question shown before the place of the system from before is reused.
    ConfirmDialog *reuseQuestion();

    // Reads the system again, as F5 does.
    void refresh();
    // `extra` follows the operation's own arguments; `then` runs when it has ended
    // (default: back to the page the person came from).
    void runOperation(const QJsonObject &operation, const QStringList &extra = QStringList(),
                      std::function<void(bool)> then = nullptr);
    // One of recab's operations by its id, then the release page again.
    void runReleaseOperation(const QString &id);
    // A release manifest from a file the person chooses (`update_media`);
    // `path` names it without asking, for the picture mode.
    void addFromFile(const QString &path = QString());
    // Writes the history as plain text.
    bool saveHistory(const QString &path);
    // Whether the agent has asked something; and answering it.
    bool agentAsks() const;
    void answerAgent(const QString &words);

    const Status &status() const { return status_; }
    Frame *frame() const { return frame_; }
    bool busy() const { return run_ != nullptr || quiet_ > 0 || reading_ != nullptr || readingHistory_ != nullptr; }

private:
    // Reads the system without holding the window up, then does `then`. When
    // it cannot be read, the page says why and `then` is not done.
    void reload(std::function<void()> then);
    void work(Run *run, const QString &title, const QString &subtitle, std::function<void(bool)> then);
    void chain(QList<Run *> runs);
    void decideByHand();
    // Shows one of the pages. The rail marks Overview for all of them; when
    // Activity is showing it stays, and the page changes beneath it.
    void display(QWidget *page);
    // The strip about the next system, on every page but Overview.
    void updateStrip();

    RecabClient client_;
    Status status_;
    Run *run_ = nullptr, *reading_ = nullptr, *readingHistory_ = nullptr;
    History history_;
    bool read_ = false; // the system was read at least once
    bool asRoot_ = false; // reading needs the administrator's rights here
    Refresher *refresher_;
    int quiet_ = 0;
    std::function<void(bool)> then_;

    Frame *frame_;
    QStackedWidget *stack_;
    SystemPage *system_;
    NextPage *next_;
    BackPage *back_;
    HistoryPage *historyPage_;
    ReleasePage *release_;
    // The rail entry the pages in between (ask, work, clash) belong to.
    QString section_;
    AskPage *ask_;
    WorkPage *work_;
    ClashPage *clash_;
    ProposedPage *proposed_;
};
