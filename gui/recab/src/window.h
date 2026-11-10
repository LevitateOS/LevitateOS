// The one window: the pages and the flows between them.
#pragma once

#include "client.h"
#include "model.h"

#include <QMainWindow>

#include <functional>

class AskPage;
class ClashPage;
class ConfirmDialog;
class ProposedPage;
class Refresher;
class QLabel;
class QStackedWidget;
class SystemPage;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    void showMain();
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

    // The question shown before the place of the system from before is reused.
    ConfirmDialog *reuseQuestion();

    // Reads the system again, as F5 does.
    void refresh();
    // Whether the agent has asked something; and answering it.
    bool agentAsks() const;
    void answerAgent(const QString &words);

    const Status &status() const { return status_; }
    bool busy() const { return run_ != nullptr || quiet_ > 0 || reading_ != nullptr; }

private:
    // Reads the system without holding the window up, then does `then`. When
    // it cannot be read, the page says why and `then` is not done.
    void reload(std::function<void()> then);
    void work(Run *run, const QString &title, const QString &subtitle, std::function<void(bool)> then);
    void chain(QList<Run *> runs);
    void decideByHand();

    RecabClient client_;
    Status status_;
    Run *run_ = nullptr, *reading_ = nullptr;
    bool read_ = false; // the system was read at least once
    bool asRoot_ = false; // reading needs the administrator's rights here
    Refresher *refresher_;
    int quiet_ = 0;
    std::function<void(bool)> then_;

    QStackedWidget *stack_;
    SystemPage *system_;
    AskPage *ask_;
    WorkPage *work_;
    ClashPage *clash_;
    ProposedPage *proposed_;
    QLabel *source_, *activity_;
};
