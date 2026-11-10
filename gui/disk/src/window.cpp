#include "window.h"

#include "pages.h"
#include "run.h"
#include "work.h"

#include <QDateTime>
#include <QDir>
#include <QFileInfo>
#include <QJsonDocument>
#include <QLabel>
#include <QSignalBlocker>
#include <QStackedWidget>
#include <QStatusBar>
#include <QStorageInfo>
#include <QTabBar>
#include <QTimer>
#include <QVBoxLayout>

Window::Window(QWidget *parent)
    : QMainWindow(parent)
    , home_(qEnvironmentVariable("DISK_GUI_HOME", QDir::homePath()))
    , agent_(qEnvironmentVariable("DISK_GUI_AGENT", "codex"))
    , recab_(qEnvironmentVariable("DISK_GUI_RECAB", "recab"))
    , rootHelper_(qEnvironmentVariable("DISK_GUI_ROOT_HELPER", "pkexec"))
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

    systems_->showStart();
    go(mess_);
    scan();
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

void Window::compare()
{
    if (run_)
        return;
    // recab reads both systems; the other one can only be read as root.
    const QStringList arguments{"compare", "--json", "--machine-events"};
    run_ = rootHelper_.isEmpty() ? new Run(recab_, arguments, this)
                                 : new Run(rootHelper_, QStringList{recab_} + arguments, this);
    job_ = Job::Comparing;
    said_.clear();
    work_->begin(tr("Comparing the two systems"), tr("Both are read from start to end. Nothing is changed on either."),
                 run_, tr("Nothing is kept: comparing starts over next time"));
    go(work_);
    run_->start();
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
    run_->setInput(prompt.toUtf8());
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
    Run *run = run_;
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
    case Job::Comparing: {
        QString error;
        const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
        if (answer.isEmpty())
            error = tr("recab finished without an answer this window can read.");
        const Comparison comparison = error.isEmpty() ? Comparison::fromJson(answer, &error) : Comparison();
        if (error.isEmpty()) {
            comparison_ = comparison;
            compared_ = true;
            systems_->show(comparison_);
        } else {
            systems_->showStart(error);
        }
        go(systems_);
        break;
    }
    case Job::Explaining:
        systems_->show(comparison_, said_.isEmpty() ? tr("The agent finished without saying anything.") : said_);
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
        go(systems_);
    }
}
