// The one window: the pages and the flows between them.
#pragma once

#include "before.h"
#include "client.h"
#include "model.h"

#include <QMainWindow>

#include <functional>

class AskPage;
class ClashPage;
class HistoryPage;
class ReleasePage;
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
    // After a long run: the section, unless the run failed and its page says why (it stays,
    // and its Back leads to the section).
    void sectionUnlessStopped(bool ok);
    void showAsk(const QString &request = QString());
    void showClash();

    // Each of these that changes something shows the Before page first, which
    // says what will happen; "askFirst = false" does it at once (the picture
    // mode's direct calls). Handing a request to the agent has its own page.
    void handOver(const QString &prompt, bool askFirst = true);
    void addUpdate(bool askFirst = true);
    void goBack(bool askFirst = true);
    void keep(bool askFirst = true);
    // Sets the next system for the next restart without restarting; and takes that back.
    void restartLater(bool askFirst = true);
    void cancelRestart(bool askFirst = true);
    void discard(bool askFirst = true);
    // Discards the next system, then makes it a copy of the running one again.
    void reset(bool askFirst = true);
    void finish(bool askFirst = true);
    void restart(bool askFirst = true);
    void recover(bool askFirst = true);
    void fixWithAgent(const QString &prompt, const QString &title);
    void acceptProposal();
    void stopRun();
    // Starts Recipe, where software is installed into this same next system.
    void installSoftware();
    // Writes the steps for a desktop that does not start, as plain text.
    bool saveSteps(const QString &path);

    // The Before page, for the checks.
    BeforePage *beforePage() const { return before_; }

    // Reads the system again, as F5 does.
    void refresh();
    // `extra` follows the operation's own arguments; `then` runs when it has ended
    // (default: back to the page the person came from).
    // An operation that changes the system shows the Before page first, unless
    // `askFirst` is false or it asks for values (those dialogs come first).
    void runOperation(const QJsonObject &operation, const QStringList &extra = QStringList(),
                      std::function<void(bool)> then = nullptr, bool askFirst = true);
    // One of recab's operations by its id, then the release page again.
    void runReleaseOperation(const QString &id, bool askFirst = true);
    // A release manifest from a file the person chooses (`update_media`);
    // `path` names it without asking, for the picture mode.
    void addFromFile(const QString &path = QString());
    // What a merge blocked on waits for, each through the Before page. The
    // original release's baseline from a file the person chooses (`path` names
    // it without asking, for the picture mode); a package reproduced through
    // Recipe for the release being added; a failed system that recab keeps no
    // preparation record for, replaced by the running one.
    void adoptBaseline(const QString &path = QString(), bool askFirst = true);
    void reproduce(const QString &package, bool askFirst = true);
    void replaceFailed(bool askFirst = true);
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
    // `made` says what the steps before the restart did, `restartLast` that the
    // last of `runs` is the restart: if it fails, the person is told the
    // computer did not restart and what was made.
    void chain(QList<Run *> runs, const QString &made = QString(), bool restartLast = false, bool first = true);
    void decideByHand();
    // Shows the Before page for `content` (drawn again when the system is read
    // again) and does `go` only when its button is pressed.
    void askBefore(std::function<BeforePage::Content()> content, std::function<void()> go);
    // The Before page of an operation, empty when it has none.
    BeforePage::Content beforeOperation(const QJsonObject &op) const;
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
    BeforePage *before_;
    std::function<BeforePage::Content()> remake_;
    std::function<void()> go_;
};
