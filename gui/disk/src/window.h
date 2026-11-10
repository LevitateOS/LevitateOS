// The one window. Two things to look at, the mess and the two systems, and
// one way out of both: hand what was found to the agent.
#pragma once

#include "mess.h"
#include "scan.h"
#include "systems.h"

#include <QDateTime>
#include <QJsonObject>
#include <QMainWindow>

#include <memory>

class HandPage;
class MessPage;
class QLabel;
class QStackedWidget;
class QTabBar;
class QTimer;
class Refresher;
class Run;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    void scan();
    void showHandOver();
    void send(const QString &prompt);
    // Adds the biggest folder the rules did not name to the list, as a person
    // would by clicking it. Says which, or nothing when there is none.
    QString pickBiggestUnnamed();
    // Send what the hand-over page shows now.
    void sendShown();

    // The two systems: the page, the comparison, the question to the agent.
    void showSystems();
    // Asks recab what the two systems are; quick, and changes nothing.
    void readState();
    void compare();
    void stopComparing();
    void showExplain();
    // What F5 does: on the mess, look again; on the systems, read their state again.
    void refresh();
    // Whether the agent has asked something; and answering it.
    bool agentAsks() const;
    void answerAgent(const QString &words);

    bool busy() const { return scanner_ != nullptr || run_ != nullptr || reading_ != nullptr; }
    bool comparing() const { return job_ == Job::Comparing; }
    const SystemsState &state() const { return state_; }
    bool stale() const;
    const QList<Finding> &found() const { return found_; }
    qint64 scannedBytes() const { return tree_ ? tree_->bytes : 0; }
    qint64 freed() const { return freed_; }
    // Null until the two systems were compared.
    const Comparison *comparison() const { return compared_ ? &comparison_ : nullptr; }
    QString said() const { return said_; }

private:
    enum class Job { None, Clearing, Comparing, Explaining };
    void scanned();
    void go(QWidget *page);
    void finished(bool ok);
    void leaveWork();
    void compared(bool ok);
    void drawSystems(const QString &why = QString());
    void keep();
    void recall();
    void openFolder(const QString &path);

    QStringList roots_;
    QString home_, agent_, recab_, rootHelper_, systemApp_, kept_;
    bool apparent_;

    Scanner *scanner_ = nullptr;
    Run *run_ = nullptr, *reading_ = nullptr;
    Job job_ = Job::None;
    SystemsState state_;
    bool stateRead_ = false, stopped_ = false;
    QDateTime comparedWhen_;
    int comparedRevision_ = -1;
    QJsonObject comparedAnswer_;
    Refresher *refresher_;
    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    qint64 before_ = 0, freed_ = 0;
    Comparison comparison_;
    bool compared_ = false;
    QString said_; // what the agent answered about the differences
    bool handingSystems_ = false;

    QTabBar *tabs_;
    QStackedWidget *stack_;
    MessPage *mess_;
    SystemsPage *systems_;
    HandPage *hand_;
    WorkPage *work_;
    QLabel *activity_;
    QTimer *tick_;
};
