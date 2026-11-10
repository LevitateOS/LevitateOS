// The one window. Two things to look at, the mess and the two systems, and
// one way out of both: hand what was found to the agent.
#pragma once

#include "mess.h"
#include "scan.h"
#include "systems.h"

#include <QMainWindow>

#include <memory>

class HandPage;
class MessPage;
class QLabel;
class QStackedWidget;
class QTabBar;
class QTimer;
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
    void compare();
    void showExplain();

    bool busy() const { return scanner_ != nullptr || run_ != nullptr; }
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

    QStringList roots_;
    QString home_, agent_, recab_, rootHelper_;
    bool apparent_;

    Scanner *scanner_ = nullptr;
    Run *run_ = nullptr;
    Job job_ = Job::None;
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
