#include "window.h"

#include "after.h"
#include "browse.h"
#include "disk.h"
#include "frame.h"
#include "nextsystem.h"
#include "pages.h"
#include "refresh.h"
#include "rules.h"
#include "run.h"
#include "words.h"
#include "work.h"

#include <QDateTime>
#include <QDesktopServices>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QMessageBox>
#include <QProcess>
#include <QSet>
#include <QPushButton>
#include <QStandardPaths>
#include <QStackedWidget>
#include <QStorageInfo>
#include <QThread>
#include <algorithm>
#include <QTimer>
#include <QToolButton>
#include <QUrl>
#include <QVBoxLayout>

// Reads what is set aside off the window's thread: it measures whole folders.
class HeldReader : public QThread {
public:
    HeldReader(const QString &home, bool apparent, QObject *parent)
        : QThread(parent)
        , home_(home)
        , apparent_(apparent)
    {
    }
    SetAside result;

protected:
    void run() override { result = readSetAside(home_, apparent_); }

private:
    QString home_;
    bool apparent_;
};

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
    , lsblk_(qEnvironmentVariable("DISK_GUI_LSBLK", "lsblk"))
    , recipe_(qEnvironmentVariable("DISK_GUI_RECIPE", "recipe"))
    , files_(qEnvironmentVariable("DISK_GUI_FILES"))
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
    never_ = readNever();

    stack_ = new QStackedWidget;
    disk_ = new DiskPage;
    browse_ = new BrowsePage;
    mess_ = new MessPage;
    systems_ = new SystemsPage;
    hand_ = new HandPage;
    work_ = new WorkPage;
    after_ = new AfterPage;
    for (QWidget *page : std::initializer_list<QWidget *>{disk_, browse_, mess_, systems_, hand_, work_, after_})
        stack_->addWidget(page);

    // The frame is the window: the rail, the pages, the strip, the status line.
    // Only the four things to look at have entries; a hand-over, work in
    // progress and what the agent set aside are steps of the mess.
    frame_ = new Frame(Frame::Disk);
    frame_->setContent(stack_);
    frame_->addEntry(QStringLiteral("disk"), tr("The disk"));
    frame_->addEntry(QStringLiteral("browse"), tr("Browse"));
    frame_->addEntry(QStringLiteral("mess"), tr("Mess"));
    frame_->addEntry(QStringLiteral("systems"), tr("The two systems"));
    frame_->select(QStringLiteral("disk"));
    frame_->setAgent(agent_);
    frame_->setStatus(tr("Looking at the disk…"));
    setCentralWidget(frame_);
    next_ = new NextSystem(recab_, this);
    connect(next_, &NextSystem::known, this, [this] { frame_->setNextSystem(next_->titles()); });

    tick_ = new QTimer(this);
    tick_->setInterval(200);
    connect(tick_, &QTimer::timeout, this, [this] {
        if (!scanner_)
            return;
        scanFiles_ = scanner_->files();
        scanBytes_ = scanner_->bytes();
        mess_->showCount(scanFiles_, scanBytes_);
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
        else if (key == QLatin1String("disk"))
            go(disk_);
        else if (key == QLatin1String("browse"))
            go(browse_);
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
    connect(browse_, &BrowsePage::openFolder, this, &Window::openFolder);
    connect(browse_, &BrowsePage::said, this, [this](const QString &words) { frame_->flash(words); });
    connect(systems_, &SystemsPage::stop, this, &Window::stopComparing);
    connect(systems_, &SystemsPage::readAgain, this, &Window::readState);
    connect(systems_, &SystemsPage::openSystem, this, &Window::openSystem);

    // The list of mess is one list, changed here and shown on three pages.
    connect(browse_, &BrowsePage::addRequested, this, &Window::addToList);
    connect(browse_, &BrowsePage::takeOffRequested, this, &Window::takeOffList);
    connect(browse_, &BrowsePage::neverRequested, this, [this](Node *node) { neverSuggest(node->path()); });
    connect(browse_, &BrowsePage::showMess, this, [this] { frame_->choose(QStringLiteral("mess")); });
    connect(browse_, &BrowsePage::handOver, this, &Window::showHandOver);
    connect(mess_, &MessPage::takeOffRequested, this, &Window::takeOffList);
    connect(mess_, &MessPage::neverRequested, this, [this](Node *node) { neverSuggest(node->path()); });
    connect(mess_, &MessPage::suggestAgain, this, &Window::suggestAgain);
    connect(mess_, &MessPage::showInBrowse, this, [this](Node *node) {
        browse_->goTo(node->parent);
        browse_->pick(node);
        frame_->choose(QStringLiteral("browse"));
    });

    connect(disk_, &DiskPage::lookAgain, this, &Window::lookAgain);
    connect(disk_, &DiskPage::showMess, this, [this] { frame_->choose(QStringLiteral("mess")); });
    connect(disk_, &DiskPage::showBrowse, this, [this] { frame_->choose(QStringLiteral("browse")); });
    connect(disk_, &DiskPage::showSystems, this, [this] { frame_->choose(QStringLiteral("systems")); });
    connect(disk_, &DiskPage::showHeld, this, &Window::showAfter);
    connect(disk_, &DiskPage::openRecipe, this, [this] { frame_->startApp(Frame::Recipe); });

    connect(after_, &AfterPage::back, this, [this] { frame_->choose(QStringLiteral("mess")); });
    connect(after_, &AfterPage::bringBack, this, &Window::bringBack);
    connect(after_, &AfterPage::bringAllBack, this, &Window::bringAllBack);
    connect(after_, &AfterPage::letGo, this, &Window::askLetGo);
    connect(after_, &AfterPage::neverSuggest, this, &Window::neverSuggest);
    connect(after_, &AfterPage::tryAgain, this, &Window::tryAgain);
    connect(after_, &AfterPage::openActivity, this, [this] { frame_->choose(Frame::activityKey()); });

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
    again->setToolTip(tr("On the disk, Browse and Mess: look at the disk again. On the two systems: ask recab again. (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    frame_->addStatusWidget(again);
    connect(mess_, &MessPage::rescan, this, &Window::lookAgain);
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
    after_->showReading();
    browse_->showLoading(tr("Looking at the disk"), tr("Reading the folders. Nothing is changed."));
    go(disk_);
    scan();
    readState();
    readLayout();
    readEarlier();
    readHeld();
    next_->read();
}

Window::~Window()
{
    // Threads that read the disk must end before the window does.
    if (scanner_) {
        scanner_->requestInterruption();
        scanner_->wait();
    }
    if (heldReader_)
        heldReader_->wait();
}

bool Window::busy() const
{
    return scanner_ != nullptr || run_ != nullptr || reading_ != nullptr || heldReader_ != nullptr || layoutRun_ != nullptr
           || recipeRun_ != nullptr;
}

void Window::refresh()
{
    if (busy())
        return;
    if (stack_->currentWidget() == systems_)
        readState();
    else if (stack_->currentWidget() == after_)
        showAfter();
    else
        lookAgain();
}

void Window::lookAgain()
{
    if (scanner_ || run_)
        return;
    scan();
    readLayout();
    readEarlier();
    readHeld();
}

// "Open in Files": the desktop's file manager. DISK_GUI_FILES names a stand-in
// for a check, so that no real program is started from a story.
void Window::openFolder(const QString &path)
{
    const QFileInfo info(path);
    const QString folder = info.isDir() ? path : info.absolutePath();
    if (!files_.isEmpty()) {
        if (!QProcess::startDetached(files_, {folder}))
            frame_->flash(tr("%1 could not be opened").arg(folder));
        return;
    }
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
bool Window::showingDisk() const { return stack_->currentWidget() == disk_; }
bool Window::showingBrowse() const { return stack_->currentWidget() == browse_; }
bool Window::showingAfter() const { return stack_->currentWidget() == after_; }
bool Window::showingMess() const { return stack_->currentWidget() == mess_; }
bool Window::showingSystems() const { return stack_->currentWidget() == systems_; }

QString Window::entryOf(QWidget *page) const
{
    if (page == disk_)
        return QStringLiteral("disk");
    if (page == browse_)
        return QStringLiteral("browse");
    if (page == mess_ || page == after_)
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

QStringList Window::zones() const
{
    QStringList all{home_};
    for (const QString &root : roots_)
        if (!all.contains(root))
            all << root;
    return all;
}

// ---------------------------------------------------------------- The scan, the mess, Browse

void Window::scan()
{
    if (scanner_)
        return;
    scanFiles_ = scanBytes_ = 0;
    scanner_ = new Scanner(roots_, apparent_, this);
    connect(scanner_, &QThread::finished, this, &Window::scanned);
    mess_->showScanning(roots_);
    browse_->showLoading(tr("Looking at the disk"), tr("Reading %1. Nothing is changed.").arg(roots_.join(tr(", "))));
    frame_->setStatus(tr("Looking at the disk…"));
    tick_->start();
    scanner_->start();
    drawDisk();
}

void Window::scanned()
{
    tick_->stop();
    tree_ = scanner_->tree();
    unreadable_ = int(scanner_->unreadable().size());
    scanner_->deleteLater();
    scanner_ = nullptr;
    const qint64 now = QDateTime::currentSecsSinceEpoch();
    found_ = findMess(tree_.get(), home_, now, never_);
    mess_->show(tree_, found_, roots_, spaceAt(home_), home_, never_, now);
    browse_->show(tree_, found_, home_, now);
    frame_->setBadge(QStringLiteral("mess"), int(found_.size()));
    frame_->setStatus(tr("Looked at the disk at %1 · only “Let it all go” deletes anything from here")
                          .arg(QLocale().toString(QTime::currentTime(), QLocale::ShortFormat)));
    drawDisk();
}

void Window::showHandOver()
{
    handingSystems_ = false;
    holding_ = newHolding(home_);
    requested_ = mess_->chosen();
    // The list is read again later; only what was handed over is kept.
    for (Finding &finding : requested_)
        finding.node = nullptr;
    hand_->show(requested_, scannedBytes(), roots_, holding_);
    go(hand_);
}

QString Window::pickBiggestUnnamed()
{
    // Among the folders directly inside each root: the biggest that holds no finding.
    Node *best = nullptr;
    if (!tree_)
        return {};
    for (const auto &root : tree_->children)
        for (const auto &child : root->children) {
            if (!child->dir || child->kind >= 0)
                continue;
            bool holds = false;
            for (const Finding &finding : found_)
                for (Node *up = finding.node; up && !holds; up = up->parent)
                    holds = up == child.get();
            if (!holds && (!best || child->bytes > best->bytes))
                best = child.get();
        }
    if (!best)
        return {};
    addToList(best);
    return best->path();
}

void Window::listChanged()
{
    std::sort(found_.begin(), found_.end(), [](const Finding &a, const Finding &b) { return a.bytes > b.bytes; });
    frame_->setBadge(QStringLiteral("mess"), int(found_.size()));
    mess_->setFound(found_, never_);
    browse_->setFound(found_);
    drawDisk();
}

void Window::addToList(Node *node)
{
    if (!node || findingAround(found_, node))
        return;
    found_ << pickedByHand(node, home_, found_);
    listChanged();
}

void Window::takeOffList(Node *node)
{
    for (int index = 0; index < found_.size(); ++index)
        if (found_[index].node == node) {
            const bool picked = found_[index].kind == kindPicked;
            found_.removeAt(index);
            node->kind = -1;
            listChanged();
            frame_->flash(picked ? tr("Taken off the list.")
                                 : tr("Taken off the list. It is found again when you look again, unless you say never to suggest it."));
            return;
        }
}

void Window::neverSuggest(const QString &path)
{
    if (path.isEmpty())
        return;
    if (!never_.contains(path))
        never_ << path;
    if (!writeNever(never_))
        frame_->flash(tr("The list of places never to suggest could not be saved in %1").arg(neverFile()));
    // What is at or inside that place leaves the list now; the rules keep it away at every later look.
    for (int index = found_.size() - 1; index >= 0; --index)
        if (found_[index].path == path || found_[index].path.startsWith(path + QLatin1Char('/'))) {
            found_[index].node->kind = -1;
            found_.removeAt(index);
        }
    listChanged();
}

void Window::suggestAgain(const QString &path)
{
    never_.removeAll(path);
    if (!writeNever(never_))
        frame_->flash(tr("The list of places never to suggest could not be saved in %1").arg(neverFile()));
    // Whether it is mess is for the rules to say again.
    lookAgain();
}

// ---------------------------------------------------------------- The disk

DiskInfo Window::diskInfo() const
{
    DiskInfo info;
    info.home = spaceAt(home_);
    const QStorageInfo storage(home_);
    info.homeDevice = QString::fromLocal8Bit(storage.device());
    info.homeMount = storage.rootPath();
    info.scanning = scanner_ != nullptr;
    info.scanFiles = scanFiles_;
    info.scanBytes = scanBytes_;
    info.scannedBytes = tree_ ? tree_->bytes : 0;
    info.unreadable = unreadable_;
    info.apparent = apparent_;
    info.found = found_;
    if (tree_ && !scanner_)
        info.slices = composition(tree_.get(), found_, home_, heldRead_ ? held_.bytes : 0);
    info.earlierBytes = earlierBytes_;
    info.earlierGenerations = earlierGenerations_;
    info.heldRead = heldRead_;
    info.held = held_;
    info.layoutRead = layoutRead_;
    info.layout = layout_;
    info.runningDevice = state_.runningDevice;
    info.otherDevice = state_.otherDevice;
    info.otherRole = state_.otherRole;
    if (compared_) {
        info.compared = true;
        info.otherFilesBytes = comparison_.other.bytes;
        info.otherDeviceCompared = comparison_.other.device;
        info.comparedWhen = comparedWhen_;
    }
    return info;
}

void Window::drawDisk()
{
    disk_->show(diskInfo());
}

void Window::readLayout()
{
    if (layoutRun_)
        return;
    layoutRun_ = new Run(lsblk_,
                         {"--json", "--bytes", "-o", "NAME,PATH,TYPE,SIZE,FSTYPE,LABEL,PARTLABEL,PARTTYPE,MOUNTPOINTS"}, this);
    connect(layoutRun_, &Run::finished, this, [this](bool ok) {
        Run *run = layoutRun_;
        layoutRun_ = nullptr;
        run->deleteLater();
        const QStorageInfo storage(home_);
        if (ok) {
            layout_ = layoutFromLsblk(QJsonDocument::fromJson(run->answer()).object(), QString::fromLocal8Bit(storage.device()),
                                      storage.rootPath());
        } else {
            layout_ = DiskLayout();
            layout_.problem = tr("`%1` could not be run: %2").arg(lsblk_, run->problem());
        }
        layoutRead_ = true;
        drawDisk();
    });
    layoutRun_->start();
}

void Window::readEarlier()
{
    if (recipeRun_)
        return;
    recipeRun_ = new Run(recipe_, {"storage", "--json"}, this);
    connect(recipeRun_, &Run::finished, this, [this](bool ok) {
        Run *run = recipeRun_;
        recipeRun_ = nullptr;
        run->deleteLater();
        earlierBytes_ = -1;
        earlierGenerations_ = 0;
        if (ok) {
            const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
            if (answer.contains("earlier_bytes")) {
                earlierBytes_ = answer["earlier_bytes"].toInteger();
                earlierGenerations_ = answer["earlier_generations"].toInteger();
            }
        }
        drawDisk();
    });
    recipeRun_->start();
}

// ---------------------------------------------------------------- What was set aside

void Window::readHeld()
{
    if (heldReader_) {
        heldAgain_ = true;
        return;
    }
    heldReader_ = new HeldReader(home_, apparent_, this);
    connect(heldReader_, &QThread::finished, this, [this] {
        HeldReader *reader = heldReader_;
        heldReader_ = nullptr;
        held_ = reader->result;
        reader->deleteLater();
        if (heldAgain_) {
            heldAgain_ = false;
            readHeld();
            return;
        }
        heldRead_ = true;
        drawDisk();
        if (pendingAfter_ || stack_->currentWidget() == after_)
            drawAfter();
        pendingAfter_ = false;
    });
    heldReader_->start();
}

void Window::showAfter()
{
    pendingAfter_ = true;
    notes_.clear();
    after_->showReading();
    go(after_);
    readHeld();
}

// The page as things stand: what was handed over, what the agent said of each
// place, what is really in the holding folder, and the room measured now.
void Window::drawAfter()
{
    AfterData data;
    data.ranHere = ranHere_;
    data.heldBytes = held_.bytes;
    for (const HeldPlace &place : held_.places)
        data.heldPlaces += place.present;
    data.sameDisk = held_.sameDisk;
    data.freeBefore = freeBefore_;
    const Space space = spaceAt(home_);
    data.freeNow = space.known ? space.free : -1;
    data.unlisted = held_.unlisted;
    data.notes = notes_;
    data.problems = held_.problems;
    data.letGone = letGone_;
    data.home = home_;

    auto heldFor = [&](const QString &path) -> const HeldPlace * {
        for (const HeldPlace &place : held_.places)
            if (place.from == path)
                return &place;
        return nullptr;
    };
    QSet<QString> listed;
    if (ranHere_) {
        for (const Finding &finding : requested_) {
            listed << finding.path;
            AfterRow row;
            row.path = finding.path;
            row.bytes = finding.bytes;
            const QJsonObject said = placed_.value(finding.path);
            const QString reason = said["reason"].toString();
            const HeldPlace *held = heldFor(finding.path);
            if (held && held->present) {
                row.kind = AfterRow::SetAside;
                row.bytes = held->bytes;
                if (said["asked"].toBool())
                    row.words = tr("it asked, you said “%1”").arg(said["answer"].toString());
            } else if (held) {
                row.kind = AfterRow::Missing;
                row.words = held->trouble;
            } else if (said["result"] == QLatin1String("kept")) {
                row.kind = AfterRow::Kept;
                row.words = reason.isEmpty() ? tr("the agent did not say why") : reason;
                if (said["asked"].toBool())
                    row.words += tr(". It asked, you said “%1”").arg(said["answer"].toString());
            } else if (said["result"] == QLatin1String("left")) {
                row.kind = AfterRow::Left;
                row.words = reason.isEmpty() ? tr("the agent left it") : reason;
            } else if (said["result"] == QLatin1String("could-not")) {
                row.kind = AfterRow::CouldNot;
                row.words = reason.isEmpty() ? tr("the agent did not say why") : reason;
            } else if (said["result"] == QLatin1String("set-aside")) {
                // It was set aside; what became of it is on the disk.
                if (QFileInfo::exists(finding.path) || QFileInfo(finding.path).isSymLink()) {
                    row.kind = AfterRow::BroughtBack;
                } else if (letGone_) {
                    row.kind = AfterRow::LetGo;
                } else {
                    row.kind = AfterRow::Missing;
                    row.words = tr("the agent said it set this aside");
                }
            } else {
                row.kind = AfterRow::NoWord;
            }
            data.rows << row;
        }
    }
    for (const HeldPlace &place : held_.places) {
        if (listed.contains(place.from))
            continue;
        AfterRow row;
        row.path = place.from;
        row.bytes = place.bytes;
        if (!place.present) {
            row.kind = AfterRow::Missing;
            row.words = place.trouble;
        } else if (ranHere_) {
            row.kind = AfterRow::Unrequested;
        } else {
            row.kind = AfterRow::SetAside;
            const QDateTime when = QDateTime::fromString(place.batch, QStringLiteral("yyyyMMdd-HHmmss"));
            if (when.isValid())
                row.words = tr("on %1").arg(QLocale().toString(when, QLocale::ShortFormat));
        }
        data.rows << row;
    }
    after_->show(data);
}

void Window::bringBack(const QString &path)
{
    notes_.clear();
    const HeldPlace *found = nullptr;
    for (const HeldPlace &place : held_.places)
        if (place.from == path && place.present)
            found = &place;
    if (!found) {
        notes_ << tr("%1 is not in the holding folder, so there is nothing to bring back.").arg(shortPath(path, home_));
    } else {
        const Outcome outcome = ::bringBack(home_, zones(), *found);
        notes_ << (outcome.ok ? tr("%1 is back where it was.").arg(shortPath(path, home_))
                              : tr("%1 was not brought back: %2.").arg(shortPath(path, home_), outcome.why));
    }
    pendingAfter_ = true;
    readHeld();
    scan(); // what is in the home folder changed
}

void Window::bringAllBack()
{
    notes_.clear();
    int back = 0;
    QStringList refused;
    for (const HeldPlace &place : held_.places) {
        if (!place.present)
            continue;
        const Outcome outcome = ::bringBack(home_, zones(), place);
        if (outcome.ok)
            ++back;
        else
            refused << tr("%1 was not brought back: %2.").arg(shortPath(place.from, home_), outcome.why);
    }
    notes_ << tr("%1 brought back.").arg(counted(back, tr("%1 place was"), tr("%1 places were"))) << refused;
    pendingAfter_ = true;
    readHeld();
    scan();
}

QMessageBox *Window::letGoQuestion() const
{
    return asking_;
}

void Window::askLetGo()
{
    if (asking_ || held_.empty())
        return;
    auto *box = new QMessageBox(this);
    asking_ = box;
    box->setAttribute(Qt::WA_DeleteOnClose);
    box->setIcon(QMessageBox::Warning);
    box->setWindowTitle(tr("Let it all go?"));
    box->setText(tr("Let everything the agent set aside go, and free %1?").arg(bytesText(held_.bytes)));
    int places = 0;
    for (const HeldPlace &place : held_.places)
        places += place.present;
    box->setInformativeText(tr("This deletes the holding folder %1 and everything in it: %2. It cannot be undone, and the places "
                               "cannot be brought back afterwards.")
                                .arg(shortPath(held_.root, home_), counted(places, tr("%1 place"), tr("%1 places"))));
    QPushButton *go = box->addButton(tr("Let it all go"), QMessageBox::YesRole);
    QPushButton *keep = box->addButton(tr("Keep it"), QMessageBox::NoRole);
    box->setDefaultButton(keep);
    connect(box, &QMessageBox::buttonClicked, this, [this, go](QAbstractButton *button) {
        if (button == go)
            letGoNow();
    });
    box->open();
}

void Window::letGoNow()
{
    const qint64 before = spaceAt(home_).free;
    const Outcome outcome = ::letGo(home_, apparent_);
    notes_.clear();
    const Space after = spaceAt(home_);
    if (outcome.ok) {
        letGone_ = true;
        notes_ << tr("Let go: %1 was deleted from the holding folder. The disk reports %2 free now; it was %3 a moment ago.")
                      .arg(bytesText(outcome.bytes), bytesText(after.free), bytesText(before));
    } else {
        notes_ << tr("Not everything was let go: %1. %2 was deleted.").arg(outcome.why, bytesText(outcome.bytes));
    }
    pendingAfter_ = true;
    readHeld();
    scan();
}

void Window::tryAgain(const QString &path)
{
    for (const Finding &finding : requested_)
        if (finding.path == path) {
            handingSystems_ = false;
            holding_ = newHolding(home_);
            requested_ = {finding};
            hand_->show(requested_, scannedBytes(), roots_, holding_);
            go(hand_);
            return;
        }
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
        drawDisk(); // which partitions are the two systems comes from here
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
    drawDisk();
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
        // What the agent says of each place, kept for the page after it.
        placed_.clear();
        notes_.clear();
        ranHere_ = true;
        letGone_ = false;
        const Space space = spaceAt(home_);
        freeBefore_ = space.known ? space.free : -1;
        connect(run_, &Run::event, this, [this](const QJsonObject &event) {
            if (event["event"] == QLatin1String("place"))
                placed_.insert(event["path"].toString(), event);
        });
        work_->begin(tr("The agent is setting the mess aside"),
                     tr("It moves only what is on the list into the holding folder, and asks before anything marked for that."), run_);
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
        scan(); // see what the disk looks like now, not what the agent says
        pendingAfter_ = true;
        after_->showReading();
        go(after_);
        readHeld();
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
        scan(); // part of it may have been set aside
        pendingAfter_ = true;
        after_->showReading();
        go(after_);
        readHeld();
    } else {
        drawSystems();
        go(systems_);
    }
}
