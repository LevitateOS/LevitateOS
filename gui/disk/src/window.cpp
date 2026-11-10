#include "window.h"

#include "pages.h"
#include "run.h"
#include "work.h"

#include <QDateTime>
#include <QDir>
#include <QFileInfo>
#include <QLabel>
#include <QStackedWidget>
#include <QStatusBar>
#include <QStorageInfo>
#include <QTimer>

Window::Window(QWidget *parent)
    : QMainWindow(parent)
    , home_(qEnvironmentVariable("DISK_GUI_HOME", QDir::homePath()))
    , agent_(qEnvironmentVariable("DISK_GUI_AGENT", "codex"))
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
    hand_ = new HandPage;
    work_ = new WorkPage;
    stack_->addWidget(mess_);
    stack_->addWidget(hand_);
    stack_->addWidget(work_);
    setCentralWidget(stack_);

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

    connect(mess_, &MessPage::rescan, this, [this] {
        before_ = 0;
        scan();
    });
    connect(mess_, &MessPage::handOver, this, &Window::showHandOver);
    connect(hand_, &HandPage::back, this, [this] { stack_->setCurrentWidget(mess_); });
    connect(hand_, &HandPage::send, this, &Window::send);
    connect(work_, &WorkPage::back, this, [this] { scan(); });
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        activity_->setText(words.isEmpty() ? tr("Agent: %1").arg(QFileInfo(agent_).fileName()) : words);
    });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
        if (ok)
            scan(); // see what is really free now, not what the agent says
    });

    scan();
}

void Window::scan()
{
    if (scanner_)
        return;
    freed_ = 0;
    scanner_ = new Scanner(roots_, apparent_, this);
    connect(scanner_, &QThread::finished, this, &Window::scanned);
    mess_->showScanning(roots_);
    stack_->setCurrentWidget(mess_);
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
    stack_->setCurrentWidget(mess_);
}

void Window::showHandOver()
{
    hand_->show(mess_->chosen(), scannedBytes(), roots_);
    stack_->setCurrentWidget(hand_);
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

void Window::sendShown()
{
    send(hand_->prompt());
}

void Window::send(const QString &prompt)
{
    before_ = scannedBytes();
    run_ = new Run(agent_, {}, this);
    run_->setInput(prompt.toUtf8());
    work_->begin(tr("The agent is clearing the mess"),
                 tr("It removes only what is on the list, and asks before anything marked for that."), run_);
    stack_->setCurrentWidget(work_);
    run_->start();
}
