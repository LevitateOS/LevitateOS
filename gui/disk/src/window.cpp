#include "window.h"

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
#include <QLabel>
#include <QProcess>
#include <QSignalBlocker>
#include <QStandardPaths>
#include <QStackedWidget>
#include <QStatusBar>
#include <QStorageInfo>
#include <QTabBar>
#include <QTimer>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>

Window::Window(QWidget *parent)
    : QMainWindow(parent)
    , home_(qEnvironmentVariable("DISK_GUI_HOME", QDir::homePath()))
    , agent_(qEnvironmentVariable("DISK_GUI_AGENT", "codex"))
    , recab_(qEnvironmentVariable("DISK_GUI_RECAB", "recab"))
    , rootHelper_(qEnvironmentVariable("DISK_GUI_ROOT_HELPER", "pkexec"))
    , systemApp_(qEnvironmentVariable("DISK_GUI_SYSTEM_APP", "recab-gui"))
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

    tabs_ = new QTabBar;
    tabs_->setDocumentMode(true);
    tabs_->setExpanding(false);
    tabs_->setDrawBase(false);
    tabs_->addTab(tr("Mess"));
    tabs_->addTab(tr("The two systems"));
    stack_ = new QStackedWidget;
    mess_ = new MessPage;
    systems_ = new SystemsPage;
    hand_ = new HandPage;
    work_ = new WorkPage;
    stack_->addWidget(mess_);
    stack_->addWidget(systems_);
    stack_->addWidget(hand_);
    stack_->addWidget(work_);
    auto *middle = new QWidget;
    auto *column = new QVBoxLayout(middle);
    column->setContentsMargins(0, 0, 0, 0);
    column->setSpacing(0);
    auto *top = new QHBoxLayout;
    top->setContentsMargins(22, 8, 22, 0);
    top->addWidget(tabs_);
    top->addStretch();
    column->addLayout(top);
    column->addWidget(stack_, 1);
    setCentralWidget(middle);

    activity_ = new QLabel(tr("Agent: %1").arg(QFileInfo(agent_).fileName()));
    statusBar()->addWidget(new QLabel(tr("This window only reads the disk")));
    statusBar()->addPermanentWidget(activity_);
    statusBar()->setSizeGripEnabled(false);

    tick_ = new QTimer(this);
    tick_->setInterval(200);
    connect(tick_, &QTimer::timeout, this, [this] {
        if (scanner_)
            mess_->showCount(scanner_->files(), scanner_->bytes());
    });

    connect(tabs_, &QTabBar::currentChanged, this, [this](int index) {
        if (index == 0)
            go(mess_);
        else
            showSystems();
    });
    connect(mess_, &MessPage::openFolder, this, &Window::openFolder);
    connect(systems_, &SystemsPage::stop, this, &Window::stopComparing);
    connect(systems_, &SystemsPage::readAgain, this, &Window::readState);
    connect(systems_, &SystemsPage::openSystem, this, [this] {
        if (!QProcess::startDetached(systemApp_, {}))
            statusBar()->showMessage(tr("%1 could not be started").arg(systemApp_), 6000);
    });

    // F5 reads again what the tab shows. Coming back to the window reads the
    // state of the two systems again, which is quick; the disk is only looked
    // at again when asked, since that takes a while and drops what was ticked.
    refresher_ = new Refresher(this, 20);
    connect(refresher_, &Refresher::asked, this, [this](bool byHand) {
        if (byHand)
            refresh();
        else if (!busy())
            readState();
    });
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("On the mess: look at the disk again. On the two systems: ask recab again. (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    statusBar()->addWidget(again);
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
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        activity_->setText(words.isEmpty() ? tr("Agent: %1").arg(QFileInfo(agent_).fileName()) : words);
    });
    connect(work_, &WorkPage::done, this, &Window::finished);

    recall();
    systems_->showLoading(tr("Reading the two systems"), tr("Asking recab which system is running and what the other one is."));
    go(mess_);
    scan();
    readState();
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
        statusBar()->showMessage(tr("%1 could not be opened").arg(folder), 6000);
}

// Shows a page. The two things to look at have tabs; a hand-over and work
// in progress are steps of one of them and hide the tabs.
void Window::go(QWidget *page)
{
    stack_->setCurrentWidget(page);
    const bool top = page == mess_ || page == systems_;
    tabs_->setVisible(top);
    if (top) {
        const QSignalBlocker quiet(tabs_);
        tabs_->setCurrentIndex(page == mess_ ? 0 : 1);
    }
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
    tabs_->setEnabled(false);
    activity_->setText(tr("Comparing the two systems"));
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
    tabs_->setEnabled(true);
    activity_->setText(tr("Agent: %1").arg(QFileInfo(agent_).fileName()));
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
    run_->setPrompt(prompt); // its questions are answered on the same input
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
}

void Window::finished(bool ok)
{
    const Job job = job_;
    run_ = nullptr;
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
