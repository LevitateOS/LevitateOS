#include "window.h"

#include "frame.h"
#include "nextsystem.h"
#include "pages.h"
#include "refresh.h"
#include "run.h"
#include "work.h"

#include <QDateTime>
#include <QDesktopServices>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QProcess>
#include <QStandardPaths>
#include <QStackedWidget>
#include <QStorageInfo>
#include <QTimer>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>

// What was asked, in one short line, for Activity: the first line of the issue.
static QString requestTitle(const QString &prompt)
{
    const QString heading = QStringLiteral("The issue:");
    const qsizetype at = prompt.indexOf(heading);
    const QString line = (at < 0 ? prompt : prompt.mid(at + heading.size())).trimmed().section(QLatin1Char('\n'), 0, 0).trimmed();
    if (line.isEmpty())
        return QObject::tr("A request from Disk");
    return line.size() > 90 ? line.left(89).trimmed() + QStringLiteral("…") : line;
}

Window::Window(QWidget *parent)
    : QMainWindow(parent)
    , home_(qEnvironmentVariable("DISK_GUI_HOME", QDir::homePath()))
    , agent_(qEnvironmentVariable("DISK_GUI_AGENT", "levitate-agent"))
    , recab_(qEnvironmentVariable("DISK_GUI_RECAB", "recab"))
    , rootHelper_(qEnvironmentVariable("DISK_GUI_ROOT_HELPER", "pkexec"))
    , osRelease_(qEnvironmentVariable("DISK_GUI_OS_RELEASE", "/etc/os-release"))
    , kept_(qEnvironmentVariable("DISK_GUI_KEPT",
                                 QStandardPaths::writableLocation(QStandardPaths::CacheLocation) + "/comparison.json"))
    , apparent_(qEnvironmentVariable("DISK_GUI_APPARENT") == QLatin1String("1"))
{
    setWindowTitle(tr("Disk"));
    // What is the person's to clear: their home, and what Recipe keeps for the system.
    roots_ = qEnvironmentVariable("DISK_GUI_ROOTS").split(QLatin1Char(':'), Qt::SkipEmptyParts);
    if (roots_.isEmpty()) {
        roots_ << home_;
        if (QFileInfo(QStringLiteral("/var/lib/recipe")).isReadable())
            roots_ << QStringLiteral("/var/lib/recipe");
    }

    stack_ = new QStackedWidget;
    mess_ = new MessPage;
    systems_ = new SystemsPage;
    hand_ = new HandPage;
    work_ = new WorkPage;
    stack_->addWidget(mess_);
    stack_->addWidget(systems_);
    stack_->addWidget(hand_);
    stack_->addWidget(work_);

    // The frame is the window: the rail, the pages, the strip, the status line.
    // Only the two things to look at have entries; a hand-over and work in
    // progress are steps of one of them.
    frame_ = new Frame(Frame::Disk);
    frame_->setContent(stack_);
    frame_->addEntry(QStringLiteral("mess"), tr("Mess"));
    frame_->addEntry(QStringLiteral("systems"), tr("The two systems"));
    frame_->select(QStringLiteral("mess"));
    frame_->setAgent(agent_);
    frame_->setStatus(tr("This window only reads the disk"));
    setCentralWidget(frame_);
    next_ = new NextSystem(recab_, this);
    connect(next_, &NextSystem::known, this, [this] { frame_->setNextSystem(next_->titles()); });

    tick_ = new QTimer(this);
    tick_->setInterval(200);
    connect(tick_, &QTimer::timeout, this, [this] {
        if (scanner_)
            mess_->showCount(scanner_->files(), scanner_->bytes());
    });

    connect(frame_, &Frame::chosen, this, [this](const QString &key) {
        if (key == Frame::activityKey())
            return;
        // While the agent works on what this entry is about, the entry shows
        // the work, not a page that is about to change.
        const bool working = run_ && job_ != Job::Comparing && job_ != Job::None;
        const bool itsWork = (key == QLatin1String("mess")) == (job_ == Job::Clearing);
        if (working && itsWork)
            go(work_);
        else if (key == QLatin1String("mess"))
            go(mess_);
        else
            showSystems();
    });
    connect(frame_, &Frame::finishRequested, this, &Window::openSystem);
    connect(frame_, &Frame::goToRun, this, [this] {
        if (!run_)
            return;
        frame_->select(entryOf(work_));
        go(work_);
    });
    connect(mess_, &MessPage::openFolder, this, &Window::openFolder);
    connect(systems_, &SystemsPage::stop, this, &Window::stopComparing);
    connect(systems_, &SystemsPage::readAgain, this, &Window::readState);
    connect(systems_, &SystemsPage::openSystem, this, &Window::openSystem);

    // F5 reads again what the tab shows. Coming back to the window reads the
    // state of the two systems again, which is quick; the disk is only looked
    // at again when asked, since that takes a while and drops what was ticked.
    refresher_ = new Refresher(this, 20);
    connect(refresher_, &Refresher::asked, this, [this](bool byHand) {
        if (byHand)
            refresh();
        else if (!busy())
            readState();
        next_->read();
    });
    connect(refresher_, &Refresher::asked, frame_, &Frame::readAgain);
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("On the mess: look at the disk again. On the two systems: ask recab again. (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    frame_->addStatusWidget(again);
    connect(mess_, &MessPage::rescan, this, [this] {
        before_ = 0;
        scan();
    });
    connect(mess_, &MessPage::handOver, this, &Window::showHandOver);
    connect(systems_, &SystemsPage::compare, this, &Window::compare);
    connect(systems_, &SystemsPage::handOver, this, &Window::showExplain);
    connect(hand_, &HandPage::back, this, [this] { go(handingSystems_ ? static_cast<QWidget *>(systems_) : mess_); });
    connect(hand_, &HandPage::send, this, &Window::send);
    connect(work_, &WorkPage::back, this, &Window::leaveWork);
    connect(work_, &WorkPage::activity, this, [this](const QString &words) { frame_->setAgentNote(words); });
    connect(work_, &WorkPage::done, this, &Window::finished);

    recall();
    systems_->showLoading(tr("Reading the two systems"), tr("Asking recab which system is running and what the other one is."));
    go(mess_);
    scan();
    readState();
    next_->read();
}

void Window::refresh()
{
    if (stack_->currentWidget() == mess_ && !scanner_ && !run_) {
        before_ = 0;
        scan();
    } else if (stack_->currentWidget() == systems_ && !busy()) {
        readState();
    }
}

void Window::openFolder(const QString &path)
{
    const QFileInfo info(path);
    const QString folder = info.isDir() ? path : info.absolutePath();
    if (!QDesktopServices::openUrl(QUrl::fromLocalFile(folder)))
        frame_->flash(tr("%1 could not be opened").arg(folder));
}

// The strip's button and "Open System…" are one thing: start the System app,
// as the rail does.
void Window::openSystem()
{
    frame_->startApp(Frame::System);
}

void Window::readStripFrom(const QString &recab)
{
    delete next_;
    next_ = new NextSystem(recab, this);
    connect(next_, &NextSystem::known, this, [this] { frame_->setNextSystem(next_->titles()); });
    next_->read();
}

bool Window::showingWork() const { return stack_->currentWidget() == work_; }
bool Window::showingMess() const { return stack_->currentWidget() == mess_; }
bool Window::showingSystems() const { return stack_->currentWidget() == systems_; }

QString Window::entryOf(QWidget *page) const
{
    if (page == mess_)
        return QStringLiteral("mess");
    if (page == systems_)
        return QStringLiteral("systems");
    // A hand-over and the work page: the flow they belong to.
    const bool ofSystems = page == hand_ ? handingSystems_ : (job_ == Job::Comparing || job_ == Job::Explaining);
    return ofSystems ? QStringLiteral("systems") : QStringLiteral("mess");
}

// Shows a page and marks the entry it belongs to.
void Window::go(QWidget *page)
{
    stack_->setCurrentWidget(page);
    frame_->follow(entryOf(page));
}

// ---------------------------------------------------------------- The mess

void Window::scan()
{
    if (scanner_)
        return;
    freed_ = 0;
    scanner_ = new Scanner(roots_, apparent_, this);
    connect(scanner_, &QThread::finished, this, &Window::scanned);
    mess_->showScanning(roots_);
    tick_->start();
    scanner_->start();
}

void Window::scanned()
{
    tick_->stop();
    tree_ = scanner_->tree();
    scanner_->deleteLater();
    scanner_ = nullptr;
    found_ = findMess(tree_.get(), home_, QDateTime::currentSecsSinceEpoch());
    if (before_ > 0)
        freed_ = qMax<qint64>(0, before_ - tree_->bytes);
    before_ = 0;
    mess_->show(tree_, found_, roots_, QStorageInfo(roots_.first()).bytesAvailable(), freed_, home_);
    frame_->setBadge(QStringLiteral("mess"), int(found_.size()));
}

void Window::showHandOver()
{
    handingSystems_ = false;
    hand_->show(mess_->chosen(), scannedBytes(), roots_);
    go(hand_);
}

QString Window::pickBiggestUnnamed()
{
    Node *node = mess_->biggestUnnamed();
    if (!node)
        return {};
    mess_->pick(node);
    mess_->addPicked();
    return node->path();
}

// ---------------------------------------------------------------- The two systems

void Window::showSystems()
{
    go(systems_);
}

void Window::readState()
{
    if (reading_ || job_ == Job::Comparing)
        return;
    reading_ = new Run(recab_, {"status", "--json"}, this);
    connect(reading_, &Run::finished, this, [this](bool ok) {
        Run *run = reading_;
        reading_ = nullptr;
        run->deleteLater();
        if (ok) {
            state_ = SystemsState::fromJson(QJsonDocument::fromJson(run->answer()).object(), osRelease_);
        } else {
            state_ = SystemsState();
            state_.problem = tr("`%1 status --json` failed: %2").arg(recab_, run->problem());
        }
        stateRead_ = true;
        refresher_->fresh();
        // A page that is being worked on is left alone.
        if (job_ == Job::None && stack_->currentWidget() != hand_)
            drawSystems();
    });
    reading_->start();
}

// The comparison is out of date when either system has changed since.
bool Window::stale() const
{
    return compared_ && state_.known && !comparedKey_.isEmpty() && state_.key != comparedKey_;
}

// The systems page as things stand: the kept comparison when there is one,
// else what recab says is there.
void Window::drawSystems(const QString &why)
{
    if (compared_ && why.isEmpty())
        systems_->show(comparison_, said_, comparedWhen_, stale());
    else
        systems_->showState(state_, why);
}

void Window::compare()
{
    if (run_ || reading_)
        return;
    // recab reads both systems; the other one can only be read as root.
    const QStringList arguments{"compare", "--json", "--machine-events"};
    run_ = rootHelper_.isEmpty() ? new Run(recab_, arguments, this)
                                 : new Run(rootHelper_, QStringList{recab_} + arguments, this);
    job_ = Job::Comparing;
    stopped_ = false;
    said_.clear();
    // It stays on the tab: the two sides fill as they are read.
    systems_->showComparing(state_);
    go(systems_);
    frame_->setAgentNote(tr("Comparing the two systems"));
    connect(run_, &Run::event, systems_, &SystemsPage::progress);
    connect(run_, &Run::finished, this, &Window::compared);
    run_->start();
}

void Window::stopComparing()
{
    if (run_ && job_ == Job::Comparing) {
        stopped_ = true;
        run_->stop();
    }
}

void Window::compared(bool ok)
{
    Run *run = run_;
    run_ = nullptr;
    job_ = Job::None;
    run->deleteLater();
    frame_->setAgentNote(QString());
    QString why;
    if (stopped_) {
        // Nothing to say: the person stopped it.
    } else if (!ok) {
        why = run->problem().isEmpty() ? tr("recab stopped without saying why.") : run->problem();
    } else {
        const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
        if (answer.isEmpty())
            why = tr("recab finished without an answer this window can read.");
        const Comparison comparison = why.isEmpty() ? Comparison::fromJson(answer, &why) : Comparison();
        if (why.isEmpty()) {
            comparison_ = comparison;
            compared_ = true;
            comparedWhen_ = QDateTime::currentDateTime();
            comparedKey_ = state_.known ? state_.key : QString();
            comparedAnswer_ = answer;
            keep();
        }
    }
    drawSystems(why);
}

// The last comparison is kept, so the window opens with it next time.
void Window::keep()
{
    QDir().mkpath(QFileInfo(kept_).absolutePath());
    QFile file(kept_);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate))
        return;
    file.write(QJsonDocument(QJsonObject{{"when", comparedWhen_.toSecsSinceEpoch()},
                                         {"state", comparedKey_},
                                         {"answer", comparedAnswer_}})
                   .toJson(QJsonDocument::Compact));
}

void Window::recall()
{
    QFile file(kept_);
    if (!file.open(QIODevice::ReadOnly))
        return;
    const QJsonObject kept = QJsonDocument::fromJson(file.readAll()).object();
    QString error;
    const Comparison comparison = Comparison::fromJson(kept["answer"].toObject(), &error);
    if (!error.isEmpty())
        return;
    comparison_ = comparison;
    compared_ = true;
    comparedAnswer_ = kept["answer"].toObject();
    comparedWhen_ = QDateTime::fromSecsSinceEpoch(kept["when"].toInteger());
    comparedKey_ = kept["state"].toString();
}

void Window::showExplain()
{
    if (!compared_)
        return;
    handingSystems_ = true;
    hand_->show(tr("Ask the agent what differs"),
                tr("The agent looks into the differences and says what they are. It changes nothing."),
                comparison_.issue(), comparison_.facts(), comparison_.expected());
    go(hand_);
}

bool Window::agentAsks() const
{
    return run_ && stack_->currentWidget() == work_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

// ---------------------------------------------------------------- The agent

void Window::sendShown()
{
    send(hand_->prompt());
}

void Window::send(const QString &prompt)
{
    if (run_)
        return;
    run_ = new Run(agent_, {}, this);
    // Its questions are answered on the same input; Activity lists the run
    // under what was asked.
    run_->setPrompt(prompt, QStringLiteral("Disk"), requestTitle(prompt));
    if (handingSystems_) {
        job_ = Job::Explaining;
        said_.clear();
        // What the agent answers is the result; keep it for the page.
        connect(run_, &Run::event, this, [this](const QJsonObject &event) {
            if (event["event"] == QLatin1String("agent"))
                said_ += (said_.isEmpty() ? QString() : QStringLiteral("\n")) + event["summary"].toString();
        });
        work_->begin(tr("The agent is looking into the differences"), tr("It reads both systems. It changes nothing."),
                     run_, tr("Nothing is changed either way"));
    } else {
        job_ = Job::Clearing;
        before_ = scannedBytes();
        work_->begin(tr("The agent is clearing the mess"),
                     tr("It removes only what is on the list, and asks before anything marked for that."), run_);
    }
    go(work_);
    run_->start();
    frame_->setOwnRun(run_->pid());
}

void Window::finished(bool ok)
{
    const Job job = job_;
    run_ = nullptr;
    frame_->setOwnRun(0); // Activity reads the run's record again
    next_->read();        // the agent may have changed the next system
    job_ = Job::None;
    if (!ok) {
        // A failure stays on the work page, which says why and offers the way
        // back. A run the person stopped has nothing more to say.
        if (work_->stopped()) {
            job_ = job;
            leaveWork();
        } else {
            job_ = job; // for the way back
        }
        return;
    }
    switch (job) {
    case Job::Clearing:
        scan(); // see what is really free now, not what the agent says
        go(mess_);
        break;
    case Job::Comparing:
        break; // has its own end, on the systems page
    case Job::Explaining:
        if (said_.isEmpty())
            said_ = tr("The agent finished without saying anything.");
        drawSystems();
        go(systems_);
        break;
    case Job::None:
        break;
    }
}

// The way back from work that stopped or failed.
void Window::leaveWork()
{
    const Job job = job_;
    job_ = Job::None;
    if (job == Job::Clearing) {
        scan(); // part of it may have been cleared
        go(mess_);
    } else {
        drawSystems();
        go(systems_);
    }
}
