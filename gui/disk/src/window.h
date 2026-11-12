// The one window. Four things to look at (the disk, Browse, the mess, the two
// systems) and one way out of them: hand what was found to the agent, which
// sets it aside; what is set aside is brought back or let go here.
#pragma once

#include "after.h"
#include "before.h"
#include "disk.h"
#include "mess.h"
#include "pages.h"
#include "scan.h"
#include "setaside.h"
#include "systems.h"

#include <QDateTime>
#include <QJsonObject>
#include <QMainWindow>
#include <QMap>
#include <QPointer>

#include <functional>
#include <memory>

class AfterPage;
class BrowsePage;
class DiskPage;
class Frame;
class HandPage;
class HeldReader;
class MessPage;
class NextSystem;
class QStackedWidget;
class QTimer;
class Refresher;
class Run;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);
    ~Window() override;

    void scan();
    void showHandOver();
    // Which agent the person chose, as the helper last said it, and whether it is being asked now
    // (the hand-over page is drawn when the answer is in).
    const AgentChoice &agentChoice() const { return choice_; }
    bool askingAgent() const { return choiceRun_ != nullptr; }
    void send(const QString &prompt);
    // Adds the biggest folder the rules did not name to the list, as a person
    // would by clicking it. Says which, or nothing when there is none.
    QString pickBiggestUnnamed();
    // Send what the hand-over page shows now.
    void sendShown();
    // The list of mess, changed by the person: the places Browse and Mess offer.
    void addToList(Node *node);
    void takeOffList(Node *node);
    void neverSuggest(const QString &path);
    void suggestAgain(const QString &path);
    // Everything again: the scan, the partitions, Recipe's storage, what is set aside.
    void lookAgain();

    // What was set aside: the page, bringing places back, letting it all go.
    void showAfter();
    void bringBack(const QString &path);
    // Do it at once (what the Before page's button does).
    void bringAllBack();
    void letGoNow();
    // Say first what each will do, on the Before page; the buttons of the page
    // for what was set aside come here, and the page's button does it.
    void askBringAllBack();
    void askLetGo();
    void tryAgain(const QString &path);
    // Asks the helper which agent is chosen without holding the window up, then draws the hand-over page.
    void showHandPage();
    // The page that says what will happen before it does.
    BeforePage *beforePage() const { return before_; }
    // The hand-over page: the same layout, with the request in it.
    HandPage *handPage() const { return hand_; }
    SystemsPage *systemsPage() const { return systems_; }
    bool showingBefore() const;

    // The two systems: the page, the comparison, the question to the agent.
    void showSystems();
    // Asks recab what the two systems are; quick, and changes nothing.
    void readState();
    void compare();
    // Say first what comparing does (the Before page), or do it at once.
    void askCompare();
    void stopComparing();
    void showExplain();
    // What F5 does: on the mess, look again; on the systems, read their state again.
    void refresh();
    // Whether the agent has asked something; and answering it.
    bool agentAsks() const;
    void answerAgent(const QString &words);

    bool busy() const;
    bool comparing() const { return job_ == Job::Comparing; }
    const SystemsState &state() const { return state_; }
    bool stale() const;
    const QList<Finding> &found() const { return found_; }
    qint64 scannedBytes() const { return tree_ ? tree_->bytes : 0; }
    // Null until the two systems were compared.
    const Comparison *comparison() const { return compared_ ? &comparison_ : nullptr; }
    QString said() const { return said_; }
    const SetAside &held() const { return held_; }
    const std::shared_ptr<Node> &tree() const { return tree_; }
    // What the last request to the agent listed.
    const QList<Finding> &requested() const { return requested_; }
    // The program that is the agent from now on (the picture runs use the helper for the hand-over
    // and a stand-in program for the check of another one).
    void useAgent(const QString &program);
    const DiskLayout &layout() const { return layout_; }
    const QStringList &never() const { return never_; }
    // The room on the disk the home folder is on, measured now.
    Space space() const { return spaceAt(home_); }
    QString home() const { return home_; }
    DiskPage *diskPage() const { return disk_; }
    BrowsePage *browsePage() const { return browse_; }
    MessPage *messPage() const { return mess_; }
    AfterPage *afterPage() const { return after_; }
    Frame *frame() const { return frame_; }
    // Which page shows, for a check.
    bool showingWork() const;
    bool showingDisk() const;
    bool showingBrowse() const;
    bool showingAfter() const;
    bool showingMess() const;
    bool showingSystems() const;
    // The strip is read with the recab program the window has. For a check
    // that must feed it a recab of another shape (the comparison stand-in
    // answers `status` in an older shape), this reads it from another program.
    void readStripFrom(const QString &recab);

private:
    enum class Job { None, Clearing, Comparing, Explaining };
    void scanned();
    void go(QWidget *page);
    void finished(bool ok);
    void leaveWork();
    void compared(bool ok);
    // Shows the Before page: its button runs `go`, Cancel shows `back`, and
    // `entry` is the rail entry it belongs to.
    void openBefore(const QString &entry, QWidget *back, const BeforePage::Content &content, std::function<void()> go);
    void drawSystems(const QString &why = QString());
    void drawDisk();
    void drawAfter();
    void readHeld();
    void readLayout();
    void readEarlier();
    void listChanged();
    DiskInfo diskInfo() const;
    QStringList zones() const;
    void keep();
    void recall();
    void openFolder(const QString &path);
    // The Recab app: from the strip's button and from "Open Recab…".
    void openSystem();
    // The rail entry the page belongs to: a hand-over or work page belongs to
    // the flow it came from.
    QString entryOf(QWidget *page) const;

    QStringList roots_;
    QString home_, agent_, recab_, rootHelper_, osRelease_, kept_, lsblk_, recipe_, files_;
    bool apparent_;

    Scanner *scanner_ = nullptr;
    Run *run_ = nullptr, *reading_ = nullptr;
    Job job_ = Job::None;
    HeldReader *heldReader_ = nullptr;
    Run *layoutRun_ = nullptr, *recipeRun_ = nullptr;
    bool heldAgain_ = false, pendingAfter_ = false, heldRead_ = false, layoutRead_ = false;
    SetAside held_;
    DiskLayout layout_;
    QStringList never_;
    qint64 earlierBytes_ = -1, earlierGenerations_ = 0;
    int unreadable_ = 0;
    qint64 scanFiles_ = 0, scanBytes_ = 0;
    // The last request to the agent: what was handed over, what it said of each place.
    QList<Finding> requested_;
    QMap<QString, QJsonObject> placed_;
    QString holding_;
    AgentChoice choice_;
    Run *choiceRun_ = nullptr;
    qint64 freeBefore_ = -1;
    bool ranHere_ = false, letGone_ = false;
    QStringList notes_;
    // The Before page: what its button does, where Cancel and the rail entry go.
    std::function<void()> beforeGo_;
    QWidget *beforeBack_ = nullptr;
    QString beforeEntry_;
    SystemsState state_;
    bool stateRead_ = false, stopped_ = false;
    QDateTime comparedWhen_;
    QString comparedKey_; // recab's state of the two systems when they were compared
    QJsonObject comparedAnswer_;
    Refresher *refresher_;
    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    Comparison comparison_;
    bool compared_ = false;
    QString said_; // what the agent answered about the differences
    bool handingSystems_ = false;

    Frame *frame_;
    NextSystem *next_;
    QStackedWidget *stack_;
    DiskPage *disk_;
    BrowsePage *browse_;
    AfterPage *after_;
    BeforePage *before_;
    MessPage *mess_;
    SystemsPage *systems_;
    HandPage *hand_;
    WorkPage *work_;
    QTimer *tick_;
};
