#include "shots.h"

#include "activity.h"
#include "frame.h"
#include "helpstory.h"
#include "livestory.h"
#include "look.h"
#include "settings.h"
#include "window.h"
#include "words.h"

#include <QAbstractButton>
#include <QApplication>
#include <QClipboard>
#include <QDir>
#include <QElapsedTimer>
#include <QFile>
#include <QFileInfo>
#include <QLabel>
#include <QPlainTextEdit>
#include <QProcess>
#include <QPushButton>
#include <QTabBar>
#include <QTextStream>
#include <QTreeWidget>
#include <QTreeWidgetItemIterator>

#include "after.h"
#include "browse.h"
#include "disk.h"
#include "pages.h"
#include "rules.h"
#include "setaside.h"
#include "systems.h"

#include <functional>
#include <limits>

#include <sys/statvfs.h>

static void waitUntilIdle(Window &window, int seconds)
{
    QElapsedTimer clock;
    clock.start();
    settle(100);
    while (window.busy() && clock.elapsed() < seconds * 1000)
        settle(50);
    settle(200);
}

namespace {

using Save = std::function<void(const QString &)>;
using Expect = std::function<void(const QString &, bool)>;

QPushButton *buttonNamed(QWidget *inside, const QString &words)
{
    for (QPushButton *button : inside->findChildren<QPushButton *>())
        if (button->text() == words && button->isVisible())
            return button;
    return nullptr;
}

// The strip: it is read with a recab of the real shape, named by
// DISK_STANDIN_STRIP_RECAB (gui/recab/fixtures/recab-real, with its own state,
// after one `prepare` and one change). The comparison keeps using the
// window's own recab, the Disk stand-in, which answers `status` in an older
// shape the strip cannot read.
void stripShots(Window &window, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    const QString real = qEnvironmentVariable("DISK_STANDIN_STRIP_RECAB");
    const QString mark = qEnvironmentVariable("DISK_STANDIN_SYSTEM_MARK");
    if (real.isEmpty() || mark.isEmpty()) {
        out << "not as expected: the strip needs DISK_STANDIN_STRIP_RECAB (gui/recab/fixtures/recab-real after one "
               "prepare and one change) and DISK_STANDIN_SYSTEM_MARK (a file the stand-in System app makes), see README.md\n";
        expect(QStringLiteral("the strip could be checked"), false);
        return;
    }
    frame->choose(QStringLiteral("mess"));
    window.readStripFrom(real);
    waitFor([&] { return frame->stripShown(); }, 10);
    expect(QStringLiteral("the strip says what the next system holds (%1)").arg(frame->stripText()),
           frame->stripShown() && frame->stripText() == QStringLiteral("The next system holds 1 change."));
    save("s1-strip-mess");
    out << "the strip: " << frame->stripText() << "\n";

    // On every page of the window.
    frame->choose(QStringLiteral("systems"));
    settle(100);
    expect(QStringLiteral("the strip shows on the two systems too"), frame->stripShown());
    save("s2-strip-systems");

    // "Finish and restart…" and "Open System…" start the System app the same way.
    QFile::remove(mark);
    if (QPushButton *finish = buttonNamed(frame, QStringLiteral("Finish and restart…")))
        finish->click();
    waitFor([&] { return QFileInfo::exists(mark); }, 10);
    expect(QStringLiteral("the strip's button starts the System app"), QFileInfo::exists(mark));
    QFile::remove(mark);
    if (QPushButton *open = buttonNamed(&window, QStringLiteral("Open System…")))
        open->click();
    waitFor([&] { return QFileInfo::exists(mark); }, 10);
    expect(QStringLiteral("\"Open System…\" starts it the same way"), QFileInfo::exists(mark));
    QFile::remove(mark);
}

// Where the strip must not be: a recab that cannot be read, and the older
// shape of the Disk stand-in. Neither says anything about it.
void noStripShots(Window &window, const QString &own, const Expect &expect)
{
    Frame *frame = window.frame();
    window.readStripFrom(QStringLiteral("/nonexistent/recab"));
    settle(600);
    expect(QStringLiteral("no strip when recab cannot be started"), !frame->stripShown());
    // Put the real one back first, so that going away is seen.
    const QString real = qEnvironmentVariable("DISK_STANDIN_STRIP_RECAB");
    window.readStripFrom(real);
    waitFor([&] { return frame->stripShown(); }, 10);
    window.readStripFrom(own);
    settle(600);
    expect(QStringLiteral("no strip when recab answers in a shape the strip cannot read"), !frame->stripShown());
}

// The pages the frame owns after Settings: Help for Disk, and the page a person
// sees the first time. The checks are `../kit/helpstory.*`; this says which page
// of Disk each target is.
void helpShots(Window &window, const QString &folder, const Save &save, const Expect &expect)
{
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    if (script.isEmpty()) {
        expect(QStringLiteral("Help needs RECAB_GUI_RECORDS (gui/agent/fixtures/records) to find the stand-in Codex"), false);
        return;
    }
    helpstory::Plan plan;
    plan.app = Frame::Disk;
    plan.window = &window;
    plan.frame = window.frame();
    plan.firstEntry = QStringLiteral("disk");
    plan.idle = [&] { waitUntilIdle(window, 300); };
    plan.folder = folder;
    plan.fixtures = QFileInfo(script).absolutePath();
    plan.pageShown = [&](const QString &key) {
        QWidget *page = key == QLatin1String("disk")      ? static_cast<QWidget *>(window.diskPage())
                        : key == QLatin1String("browse")  ? static_cast<QWidget *>(window.browsePage())
                        : key == QLatin1String("mess")    ? static_cast<QWidget *>(window.messPage())
                        : key == QLatin1String("systems") ? static_cast<QWidget *>(window.findChild<SystemsPage *>())
                                                          : nullptr;
        return page && page->isVisible();
    };
    helpstory::run(plan, [&](QWidget &, const QString &name) { save(name); }, expect, "h");
    window.frame()->choose(QStringLiteral("disk"));
    waitUntilIdle(window, 300);
}

// The frame's Settings page: the entry under Activity, and the page opening.
// It reads through the built levitate-agent with the stand-in Codex and a
// settings folder of the story's own.
void settingsShots(Window &window, const QString &folder, const Save &save, const Expect &expect)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    if (script.isEmpty() || qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT").isEmpty()) {
        expect(QStringLiteral("Settings needs RECAB_GUI_RECORDS and LEVITATE_ACTIVITY_AGENT, see README.md"), false);
        return;
    }
    const QString home = QDir(folder).filePath("settings-home");
    QDir(home).removeRecursively();
    QDir().mkpath(home);
    const QByteArray config = qgetenv("XDG_CONFIG_HOME"), codex = qgetenv("LEVITATE_AGENT_CODEX");
    qputenv("XDG_CONFIG_HOME", (home + "/config").toUtf8());
    qputenv("LEVITATE_AGENT_CODEX", QDir(QFileInfo(script).absolutePath()).filePath("codex").toUtf8());
    SettingsPage *page = frame->settings();
    frame->choose(Frame::settingsKey());
    waitFor([&] { return page->loaded() || !page->problem().isEmpty(); });
    expect(QStringLiteral("Settings opens and is marked (%1)").arg(page->problem()),
           frame->settingsShown() && frame->selected() == Frame::settingsKey() && page->loaded());
    expect(QStringLiteral("it is the same page: the agent, what it may do, the windows"),
           page->pageText().contains("The agent that does the work") && page->pageText().contains("Installed, and signed in")
               && page->pageText().contains("What the agent may do without asking")
               && page->pageText().contains("Each window opens at the size it had."));
    save("1b-settings");
    frame->choose(QStringLiteral("mess"));
    expect(QStringLiteral("choosing Mess leaves Settings"),
           !frame->settingsShown() && frame->selected() == "mess" && window.showingMess());
    for (auto [name, value] : {std::pair{"XDG_CONFIG_HOME", config}, std::pair{"LEVITATE_AGENT_CODEX", codex}}) {
        if (value.isEmpty())
            qunsetenv(name);
        else
            qputenv(name, value);
    }
}

// Activity with records of the agent's runs (gui/agent/fixtures/records, a
// stand-in Codex, never a real agent), read through the built levitate-agent.
// Disk has started none of these runs: they are Recipe's and System's.
// `own`: the work page shows the live run, which is then this window's own.
void activityShots(Window &window, const QString &folder, bool own, const Save &save, const Expect &expect,
                   QTextStream &out, QString *where)
{
    Frame *frame = window.frame();
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        out << "not as expected: Activity needs RECAB_GUI_RECORDS (gui/agent/fixtures/records) and "
               "LEVITATE_ACTIVITY_AGENT (the built levitate-agent), see README.md\n";
        expect(QStringLiteral("the records for Activity could be made"), false);
        return;
    }
    *where = QDir(folder).filePath(own ? QStringLiteral("records-own") : QStringLiteral("records"));
    QProcess make;
    make.start(script, {*where, agent});
    make.waitForFinished(300000);
    const QString said = QString::fromUtf8(make.readAllStandardOutput()).trimmed();
    expect(QStringLiteral("the records for Activity were made (%1)").arg(QString::fromUtf8(make.readAllStandardError()).trimmed()),
           make.exitStatus() == QProcess::NormalExit && make.exitCode() == 0 && said.startsWith("state="));
    if (!said.startsWith("state="))
        return;
    qputenv("XDG_STATE_HOME", said.section(' ', 0, 0).mid(6).toUtf8());
    const QString waitingId = said.section("waiting_id=", 1).section(' ', 0, 0);
    // The sockets of the runs it left going: attaching needs them.
    qputenv("XDG_RUNTIME_DIR", said.section("runtime=", 1).section(' ', 0, 0).toUtf8());
    ActivityPage *page = frame->activity();

    frame->readAgain();
    waitFor([&] { return page->loaded() && frame->badge(Frame::activityKey()) > 0; });
    expect(QStringLiteral("the Activity entry counts the one run that waits"),
           frame->badge(Frame::activityKey()) == 2 && frame->badgeWarns(Frame::activityKey()));
    frame->choose(Frame::activityKey());
    waitFor([&] { return page->listedTitles().size() == 5 && !page->detailText().isEmpty(); });
    expect(QStringLiteral("Activity is showing and marked"),
           frame->activityShown() && frame->selected() == Frame::activityKey());
    expect(QStringLiteral("Activity lists the five runs"), page->listedTitles().size() == 5);
    expect(QStringLiteral("the strip shows on Activity"), frame->stripShown());
    save(own ? QStringLiteral("a2-activity-while-the-agent-works") : QStringLiteral("a1-activity"));

    page->showOnlyThisApp(true);
    expect(QStringLiteral("Disk started none of the runs, so its own filter lists none"), page->listedTitles().isEmpty());
    page->showOnlyThisApp(false);

    if (!own)
        return;
    // The agent this window waits for: Activity offers to go to it.
    page->chooseTitle("Turn off password logins over ssh");
    waitFor([&] { return page->detailText().contains("It asks"); });
    frame->setOwnRun(waitingId);
    expect(QStringLiteral("the frame knows the id of this window's own run"), frame->ownRun() == waitingId && !waitingId.isEmpty());
    waitFor([&] { return page->offersGoTo(); });
    expect(QStringLiteral("this window's own live run offers Go to it, and its question can be answered here too"),
           page->offersGoTo() && waitFor([&] { return page->answerButtons() == QStringList({"Red", "Blue"}); })
               && !page->detailText().contains("Only the window that started it"));
    if (QPushButton *go = buttonNamed(page, QStringLiteral("Go to it")))
        go->click();
    settle(200);
    expect(QStringLiteral("Go to it leaves Activity for the work page, under the Mess entry"),
           !frame->activityShown() && frame->selected() == "mess" && window.showingWork());
    frame->setOwnRun(QString());
}

} // namespace

namespace {

bool near(qint64 a, qint64 b)
{
    // The disk is shared with other programs while the check runs, so what was
    // measured a moment ago may differ from what statvfs says now.
    return qAbs(a - b) <= (256ll << 20);
}

// Everything a person can read on a page: its labels, buttons and list rows.
QString pageText(QWidget *page)
{
    QStringList words;
    for (QLabel *label : page->findChildren<QLabel *>())
        if (label->isVisible())
            words << label->text();
    for (QAbstractButton *button : page->findChildren<QAbstractButton *>())
        if (button->isVisible())
            words << button->text();
    for (QTabBar *tabs : page->findChildren<QTabBar *>())
        for (int tab = 0; tab < tabs->count(); ++tab)
            words << tabs->tabText(tab);
    for (QTreeWidget *tree : page->findChildren<QTreeWidget *>()) {
        for (int column = 0; column < tree->columnCount(); ++column)
            words << tree->headerItem()->text(column);
        for (QTreeWidgetItemIterator it(tree); *it; ++it)
            for (int column = 0; column < tree->columnCount(); ++column)
                words << (*it)->text(column);
    }
    return words.join(QLatin1Char('\n'));
}

qint64 statFree(const QString &path)
{
    struct statvfs st;
    return ::statvfs(path.toLocal8Bit().constData(), &st) == 0 ? qint64(st.f_bavail) * qint64(st.f_frsize) : -1;
}

qint64 statTotal(const QString &path)
{
    struct statvfs st;
    return ::statvfs(path.toLocal8Bit().constData(), &st) == 0 ? qint64(st.f_blocks) * qint64(st.f_frsize) : -1;
}

qint64 sumOf(const QList<Finding> &found)
{
    qint64 total = 0;
    for (const Finding &finding : found)
        total += finding.bytes;
    return total;
}

// A file made for a check, with the folders it needs.
void plant(const QString &path)
{
    QDir().mkpath(QFileInfo(path).absolutePath());
    QFile file(path);
    if (file.open(QIODevice::WriteOnly))
        file.write("x");
}

// Takes a file made by `plant` out again, and the folders above it that are empty, up to `base`.
void removeForged(const QString &base, const QString &relative)
{
    QFile::remove(base + QLatin1Char('/') + relative);
    for (QString up = QFileInfo(base + QLatin1Char('/') + relative).absolutePath(); up.size() > base.size(); up = QFileInfo(up).absolutePath())
        if (!QDir().rmdir(up))
            break;
}

bool listsPath(const Window &window, const QString &path)
{
    for (const Finding &finding : window.found())
        if (finding.path == path)
            return true;
    return false;
}

Finding findingNamed(const Window &window, const QString &tail)
{
    for (const Finding &finding : window.found())
        if (finding.path.endsWith(tail))
            return finding;
    return {};
}

Node *nodeAt(Node *top, const QString &path)
{
    if (top->path() == path)
        return top;
    for (const auto &child : top->children)
        if (Node *found = nodeAt(child.get(), path))
            return found;
    return nullptr;
}

// The first page: the disk, measured. Leftovers, Recipe's share, the
// partitions from the lsblk stand-in, whose room it is.
void diskShots(Window &window, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    DiskPage *page = window.diskPage();
    const QString home = window.home();
    expect(QStringLiteral("the window opens on The disk"), frame->selected() == "disk" && window.showingDisk());
    const QString words = pageText(page);
    // The page's figures are what it measured when it drew; statvfs now may differ a little (the disk is shared).
    const DiskInfo &drawn = page->shown();
    const qint64 free = drawn.home.free, total = drawn.home.total;
    const bool nearly = free < (5ll << 30) || free * 10 < total;
    out << "the disk: " << bytesText(free) << " free of " << bytesText(total) << (nearly ? " (nearly full)" : " (has room)") << "\n";
    expect(QStringLiteral("the figures are statvfs's (free %1 and %2, total %3 and %4)").arg(free).arg(statFree(home)).arg(total).arg(statTotal(home)),
           drawn.home.known && near(free, statFree(home)) && total == statTotal(home));
    expect(QStringLiteral("the heading is true of the measure (%1)").arg(page->headingText()),
           page->headingText() == (nearly ? QStringLiteral("The place for your files is nearly full")
                                          : QStringLiteral("The place for your files has room")));
    expect(QStringLiteral("it says how much is left of how much (%1 of %2)").arg(bytesText(free), bytesText(total)),
           words.contains(QStringLiteral("%1 is left of %2.").arg(bytesText(free), bytesText(total))));
    expect(QStringLiteral("the window's own measure is statvfs's"),
           window.space().known && near(window.space().free, statFree(home)) && window.space().total == statTotal(home));

    // The leftovers row is the Mess list: the same figure in the same number of places.
    const qint64 mess = sumOf(window.found());
    expect(QStringLiteral("the Known leftovers row equals the Mess list (%1 in %2 places)").arg(bytesText(mess)).arg(window.found().size()),
           words.contains(QStringLiteral("Known leftovers")) && words.contains(bytesText(mess))
               && words.contains(counted(window.found().size(), QStringLiteral("in %1 place"), QStringLiteral("in %1 places"))));
    // Recipe's earlier versions, asked of the stand-in `recipe storage --json`.
    const qint64 earlier = qEnvironmentVariable("DISK_STANDIN_EARLIER", QString::number(6ll << 30)).toLongLong();
    expect(QStringLiteral("Kept by Recipe says what Recipe says (%1)").arg(bytesText(earlier)),
           earlier > 0 && words.contains(QStringLiteral("Kept by Recipe")) && words.contains(bytesText(earlier)));
    const QString mark = qEnvironmentVariable("DISK_STANDIN_RECIPE_MARK");
    if (mark.isEmpty() || qEnvironmentVariable("LEVITATE_APP_RECIPE").isEmpty()) {
        expect(QStringLiteral("the Recipe app could be started: LEVITATE_APP_RECIPE and DISK_STANDIN_RECIPE_MARK, see README.md"), false);
    } else {
        QFile::remove(mark);
        if (QPushButton *open = buttonNamed(page, QStringLiteral("Open Recipe…")))
            open->click();
        waitFor([&] { return QFileInfo::exists(mark); }, 10);
        expect(QStringLiteral("\"Open Recipe…\" starts the Recipe app"), QFileInfo::exists(mark));
        QFile::remove(mark);
    }
    expect(QStringLiteral("there is no row for what cannot be measured (logs and crash records)"),
           !words.contains(QStringLiteral("Logs and crash records")));

    // Every partition of the disk the home folder is on, from lsblk's answer.
    const DiskLayout &layout = window.layout();
    expect(QStringLiteral("the disk was found in lsblk's answer (%1: %2)").arg(layout.disk, layout.problem),
           layout.found && layout.disk == QStringLiteral("/dev/vda") && layout.parts.size() == 6);
    expect(QStringLiteral("the other disk's partition is not listed"), !words.contains(QStringLiteral("backup")));
    for (const QString &phrase :
         {QStringLiteral("The running system"), QStringLiteral("The system from before"),
          QStringLiteral("Your files and what programs keep"), QStringLiteral("Start-up files"),
          QStringLiteral("Partition “scratch”"), QStringLiteral("Swap space")})
        expect(QStringLiteral("a partition row says \"%1\"").arg(phrase), words.contains(phrase));
    expect(QStringLiteral("sizes are lsblk's (20 GiB, 512 MiB)"), words.contains(QStringLiteral("20 GiB")) && words.contains(QStringLiteral("512 MiB")));
    // The home folder's partition is measured where it is mounted.
    const Partition *homePart = nullptr;
    for (const Partition &part : layout.parts)
        if (part.path == QStringLiteral("/dev/vda4"))
            homePart = &part;
    expect(QStringLiteral("the home partition's used and free are statvfs's (%1 free)").arg(bytesText(homePart ? homePart->free : 0)),
           homePart && near(homePart->free, statFree(home)) && homePart->used == homePart->total - homePart->free
               && homePart->total == total && words.contains(bytesText(homePart->free)));
    // The other system sits on a partition recab names; it is not mounted, so its use is not read.
    const Partition *other = nullptr;
    for (const Partition &part : layout.parts)
        if (part.path == QStringLiteral("/dev/vda3"))
            other = &part;
    expect(QStringLiteral("the other system's use is not read (not mounted)"),
           other && other->used < 0 && words.contains(QStringLiteral("not mounted, so not read")));

    // Whose room it is: the kinds the scan tells apart add up to what it read.
    qint64 sliced = 0;
    for (const Slice &slice : composition(window.tree().get(), window.found(), home, window.held().bytes))
        sliced += slice.bytes;
    expect(QStringLiteral("whose room it is adds up to what the scan read (%1 and %2)").arg(bytesText(sliced), bytesText(window.scannedBytes())),
           sliced == window.scannedBytes() && words.contains(QStringLiteral("Whose room it is"))
               && words.contains(QStringLiteral("Your files,")));
    save("1-disk");
}

// Browse: going in and up, search, the two extra views, and what can be done with a place.
void browseShots(Window &window, const QString &folder, const Save &save, const Expect &expect, QTextStream &out)
{
    Frame *frame = window.frame();
    BrowsePage *page = window.browsePage();
    frame->choose(QStringLiteral("browse"));
    settle(200);
    expect(QStringLiteral("Browse is marked and shows"), frame->selected() == "browse" && window.showingBrowse());
    const QString home = window.home();
    expect(QStringLiteral("Browse starts in the home folder (%1)").arg(page->folder() ? page->folder()->path() : QString()),
           page->folder() && page->folder()->path() == home && page->view() == BrowsePage::View::Folders);
    const QStringList top = page->listed();
    out << "browse, in the home folder: " << top.join(", ") << "\n";
    for (const QString &name : {QStringLiteral("Videos"), QStringLiteral("Projects"), QStringLiteral(".cache"), QStringLiteral("Downloads")})
        expect(QStringLiteral("the home folder lists %1").arg(name), top.contains(name));
    save("2a-browse");

    // Going in, and up.
    Node *projects = nodeAt(page->folder(), home + QStringLiteral("/Projects"));
    page->pick(projects);
    expect(QStringLiteral("choosing Projects points at it and allows going in"),
           page->pointed() == projects && buttonNamed(page, QStringLiteral("Go in")) && buttonNamed(page, QStringLiteral("Go in"))->isEnabled());
    buttonNamed(page, QStringLiteral("Go in"))->click();
    settle(200);
    expect(QStringLiteral("Go in shows Projects (%1)").arg(page->folder()->path()), page->folder() == projects);
    expect(QStringLiteral("its folders are listed"), page->listed().contains(QStringLiteral("shop")) && page->listed().contains(QStringLiteral("site"))
                                                        && page->listed().contains(QStringLiteral("viewer")));
    expect(QStringLiteral("the path can be clicked: Projects is named"), pageText(page).contains(QStringLiteral("Projects")));
    save("2b-browse-in");
    // Search filters this folder only.
    page->setSearch(QStringLiteral("sh"));
    settle(100);
    expect(QStringLiteral("search keeps what matches (%1)").arg(page->listed().join(", ")), page->listed() == QStringList{QStringLiteral("shop")});
    page->setSearch(QStringLiteral("nothing-is-called-this"));
    settle(100);
    expect(QStringLiteral("search says when nothing matches"), page->listed().isEmpty() && pageText(page).contains(QStringLiteral("No folder here has")));
    page->setSearch(QString());
    settle(100);
    expect(QStringLiteral("clearing the search lists the folders again"), page->listed().size() >= 3);
    buttonNamed(page, QStringLiteral("Up"))->click();
    settle(100);
    expect(QStringLiteral("Up goes back to the home folder"), page->folder() && page->folder()->path() == home);

    // The two other views, over the home folder.
    page->setView(BrowsePage::View::Biggest);
    settle(100);
    const QStringList biggest = page->listed();
    expect(QStringLiteral("Biggest files starts with the biggest file (%1)").arg(biggest.value(0)),
           biggest.value(0) == QStringLiteral("Videos/wedding.mkv") && biggest.contains(QStringLiteral("Music/album.flac")));
    qint64 last = std::numeric_limits<qint64>::max();
    bool sorted = true;
    for (const QString &name : biggest) {
        Node *node = nodeAt(page->folder(), home + QLatin1Char('/') + name);
        sorted = sorted && node && node->bytes <= last;
        last = node ? node->bytes : 0;
    }
    expect(QStringLiteral("and they run from big to small"), sorted && biggest.size() > 5);
    save("2c-browse-biggest");
    page->setView(BrowsePage::View::Unused);
    settle(100);
    expect(QStringLiteral("Not used for a year lists only what is that old (%1)").arg(page->listed().join(", ")),
           page->listed() == QStringList{QStringLiteral("Videos/screen-recording-2024.mkv")});
    save("2d-browse-unused");
    page->setView(BrowsePage::View::Folders);

    // What can be done with a place. Copy, and Open in Files through a stand-in.
    Node *shop = nodeAt(page->folder(), home + QStringLiteral("/Projects/shop"));
    page->goTo(projects);
    page->pick(shop);
    buttonNamed(page, QStringLiteral("Copy the path"))->click();
    expect(QStringLiteral("Copy the path puts the path on the clipboard"), QApplication::clipboard()->text() == shop->path());
    const QString filesMark = qEnvironmentVariable("DISK_STANDIN_FILES_MARK");
    if (filesMark.isEmpty() || qEnvironmentVariable("DISK_GUI_FILES").isEmpty()) {
        expect(QStringLiteral("Open in Files could be checked: DISK_GUI_FILES and DISK_STANDIN_FILES_MARK, see README.md"), false);
    } else {
        QFile::remove(filesMark);
        buttonNamed(page, QStringLiteral("Open in Files"))->click();
        waitFor([&] { return QFileInfo::exists(filesMark); }, 10);
        QFile mark(filesMark);
        const QString asked = mark.open(QIODevice::ReadOnly) ? QString::fromUtf8(mark.readAll()) : QString();
        expect(QStringLiteral("Open in Files hands the desktop's file manager the folder (%1)").arg(asked), asked == shop->path());
        QFile::remove(filesMark);
    }
    Node *target = nodeAt(page->folder(), home + QStringLiteral("/Projects/shop/target"));
    page->goTo(shop);
    page->pick(target);
    expect(QStringLiteral("a place on the list says what it is and offers to take it off or never suggest it"),
           pageText(page).contains(QStringLiteral("Made by the Rust compiler")) && buttonNamed(page, QStringLiteral("Never suggest this"))
               && buttonNamed(page, QStringLiteral("Take off the list")) && !buttonNamed(page, QStringLiteral("Add to the list")));
    save("2e-browse-place");
    page->pick(nodeAt(page->folder(), home + QStringLiteral("/Projects/shop/src")));
    expect(QStringLiteral("a place that is not on it offers to add it, and not the others"),
           buttonNamed(page, QStringLiteral("Add to the list")) && !buttonNamed(page, QStringLiteral("Never suggest this")));

    // Add to the list, then take it off again.
    const int before = int(window.found().size());
    buttonNamed(page, QStringLiteral("Add to the list"))->click();
    settle(100);
    expect(QStringLiteral("Add to the list puts it on the list and moves the badge (%1 and %2)").arg(before + 1).arg(frame->badge("mess")),
           window.found().size() == before + 1 && frame->badge("mess") == before + 1 && buttonNamed(page, QStringLiteral("Take off the list")));
    buttonNamed(page, QStringLiteral("Take off the list"))->click();
    settle(100);
    expect(QStringLiteral("Take off the list takes it off again"), window.found().size() == before && frame->badge("mess") == before);

    // Never suggest: the place leaves the list, and stays off it at the next look.
    const QString never = home + QStringLiteral("/.cache/mozilla");
    expect(QStringLiteral("the Firefox cache is on the list to begin with"), listsPath(window, never));
    page->goTo(nodeAt(window.tree().get(), home + QStringLiteral("/.cache")));
    page->pick(nodeAt(window.tree().get(), never));
    buttonNamed(page, QStringLiteral("Never suggest this"))->click();
    settle(200);
    expect(QStringLiteral("Never suggest this takes it off the list at once"), !listsPath(window, never) && window.found().size() == before - 1
                                                                                && frame->badge("mess") == before - 1);
    expect(QStringLiteral("and keeps it in the person's config folder (%1)").arg(neverFile()),
           window.never().contains(never) && readNever().contains(never) && QFileInfo::exists(neverFile()));
    save("2f-browse-never");
    window.lookAgain();
    waitUntilIdle(window, 300);
    expect(QStringLiteral("a second look does not suggest it again"), !listsPath(window, never) && window.found().size() == before - 1
                                                                       && frame->badge("mess") == before - 1);
    frame->choose(QStringLiteral("mess"));
    settle(150);
    expect(QStringLiteral("Mess says what you told it"), pageText(window.messPage()).contains(QStringLiteral("Never suggest: 1 place"))
                                                           && pageText(window.messPage()).contains(QStringLiteral("Suggest it again")));
    save("3c-mess-told");
    frame->choose(QStringLiteral("browse"));
    // It is put back at the end of the story: the config folder is the person's.
    window.suggestAgain(never);
    waitUntilIdle(window, 300);
    expect(QStringLiteral("Suggest it again puts it back, and the config file no longer names it"),
           listsPath(window, never) && !readNever().contains(never) && window.found().size() == before);
    Q_UNUSED(folder)
}

// Mess: the list in the mock-up's words, the ticks, the panel for a place.
void messShots(Window &window, const Save &save, const Expect &expect)
{
    Frame *frame = window.frame();
    MessPage *page = window.messPage();
    frame->choose(QStringLiteral("mess"));
    settle(200);
    expect(QStringLiteral("Mess is marked and shows"), frame->selected() == "mess" && window.showingMess());
    const QString words = pageText(page);
    expect(QStringLiteral("the heading counts the places and their size"),
           words.contains(QStringLiteral("%1 look like mess, %2")
                              .arg(counted(window.found().size(), QStringLiteral("%1 place"), QStringLiteral("%1 places")),
                                   bytesText(sumOf(window.found())))));
    expect(QStringLiteral("it says what each kind is, and the columns are the mock-up's"),
           words.contains(QStringLiteral("made again by the next build")) && words.contains(QStringLiteral("What and where"))
               && words.contains(QStringLiteral("Last used")) && words.contains(QStringLiteral("The agent")));
    expect(QStringLiteral("it no longer says this window removes nothing"), !words.contains(QStringLiteral("removes nothing itself")));
    const Finding shop = findingNamed(window, QStringLiteral("/Projects/shop/target"));
    page->pick(shop.node);
    const QString panel = pageText(page);
    expect(QStringLiteral("choosing a place says what it is and every action is there"),
           panel.contains(QStringLiteral("Made by the Rust compiler")) && buttonNamed(page, QStringLiteral("Show in Browse"))
               && buttonNamed(page, QStringLiteral("Open in Files")) && buttonNamed(page, QStringLiteral("Take off the list"))
               && buttonNamed(page, QStringLiteral("Never suggest this")) && buttonNamed(page, QStringLiteral("Hand it to the agent…")));
    expect(QStringLiteral("the foot counts what is ticked and what goes without asking"),
           panel.contains(QStringLiteral("ticked: ")) && panel.contains(QStringLiteral("goes without asking")));
    save("3a-mess");
    // "Show in Browse" goes to the place's folder, with the place pointed at.
    buttonNamed(page, QStringLiteral("Show in Browse"))->click();
    settle(150);
    expect(QStringLiteral("Show in Browse opens the folder and points at the place"),
           window.showingBrowse() && window.browsePage()->pointed() == shop.node && window.browsePage()->folder() == shop.node->parent);
    frame->choose(QStringLiteral("mess"));
    // Ticking only what clears without asking leaves nothing that asks.
    page->tickOnlySure();
    bool allSure = !page->chosen().isEmpty();
    for (const Finding &finding : page->chosen())
        allSure = allSure && finding.sure && finding.kind != kindPicked;
    expect(QStringLiteral("\"Tick only what clears without asking\" leaves only places that clear"), allSure);
    window.lookAgain(); // a second look ticks everything again
    waitUntilIdle(window, 300);
    expect(QStringLiteral("a second look ticks everything"), page->chosen().size() == window.found().size());
}

// After the agent ran: what it set aside, what was kept or could not be done,
// the room measured before and now; then one place brought back, a refusal, the
// confinement, and letting everything go.
void afterShots(Window &window, const QString &folder, const Save &save, const Expect &expect, QTextStream &out,
                qint64 freeBefore, const QString &sentinel, const QString &outside)
{
    Frame *frame = window.frame();
    AfterPage *page = window.afterPage();
    const QString home = window.home();
    waitUntilIdle(window, 300);
    expect(QStringLiteral("after the run the page for what was set aside shows, under Mess"),
           window.showingAfter() && frame->selected() == "mess");
    const SetAside held = window.held();
    out << "set aside: " << held.places.size() << " places, " << bytesText(held.bytes) << " in " << held.root << "\n";
    expect(QStringLiteral("the agent set places aside (%1)").arg(held.places.size()), held.places.size() > 0 && !held.empty());
    bool inside = true, gone = true;
    for (const HeldPlace &place : held.places) {
        inside = inside && place.present && QFileInfo::exists(held.root + QLatin1Char('/') + place.batch + QLatin1Char('/') + place.held);
        gone = gone && !QFileInfo(place.from).exists() && !QFileInfo(place.from).isSymLink();
    }
    expect(QStringLiteral("every one is in the holding folder under its own path, and no longer where it was"), inside && gone);
    // Every place handed over is either held or still where it was, never both and never neither.
    bool oneOrOther = !window.requested().isEmpty();
    for (const Finding &finding : window.requested()) {
        bool isHeld = false;
        for (const HeldPlace &place : held.places)
            isHeld = isHeld || place.from == finding.path;
        const bool there = QFileInfo::exists(finding.path);
        oneOrOther = oneOrOther && (isHeld != there);
    }
    expect(QStringLiteral("every place handed over is held or still where it was, not both"), oneOrOther);
    const QString words = pageText(page);
    expect(QStringLiteral("the page says what was set aside and that nothing is gone yet"),
           words.contains(QStringLiteral("%1 is set aside").arg(bytesText(held.bytes))) && words.contains(QStringLiteral("Nothing is gone yet"))
               && words.contains(QStringLiteral("Bring everything back")));
    expect(QStringLiteral("it does not say anything is let go by itself"), !words.contains(QStringLiteral("30 days")) && !words.contains(QStringLiteral("by itself")));
    // Free before and now are measured on the disk, not counted.
    const AfterData &drawn = page->shown();
    expect(QStringLiteral("Free before is what statvfs said when it was handed over (%1 and %2)").arg(bytesText(freeBefore), bytesText(drawn.freeBefore)),
           drawn.freeBefore == freeBefore && words.contains(QStringLiteral("Free before")) && words.contains(bytesText(freeBefore)));
    expect(QStringLiteral("Free now is what statvfs says (%1 and %2)").arg(drawn.freeNow).arg(statFree(home)),
           near(drawn.freeNow, statFree(home)) && words.contains(QStringLiteral("Free now")) && words.contains(bytesText(drawn.freeNow)));
    expect(QStringLiteral("Free once you let it go is free now plus what is held (%1)").arg(bytesText(drawn.freeNow + held.bytes)),
           words.contains(QStringLiteral("Free once you let it go")) && words.contains(bytesText(drawn.freeNow + held.bytes)));
    // Per place: set aside, kept (and why, when asked), could not.
    expect(QStringLiteral("a place is listed as set aside, with Bring back"), words.contains(QStringLiteral("Set aside")) && buttonNamed(page, QStringLiteral("Bring back")));
    expect(QStringLiteral("a place the person kept says so and why"), words.contains(QStringLiteral("Kept: you said to keep it")) && words.contains(QStringLiteral("it asked, you said")));
    expect(QStringLiteral("a place that could not be done says so and why, and offers to try again"),
           words.contains(QStringLiteral("Could not: a program had files open in it")) && buttonNamed(page, QStringLiteral("Try again")));
    expect(QStringLiteral("the kept place offers \"Never suggest\""), buttonNamed(page, QStringLiteral("Never suggest")) != nullptr);
    expect(QStringLiteral("what Recipe keeps is said to be left to Recipe"), words.contains(QStringLiteral("Left alone: it belongs to Recipe")));
    save("4-after");
    // The disk lists the holding folder while it holds anything.
    frame->choose(QStringLiteral("disk"));
    waitUntilIdle(window, 60);
    const QString disk = pageText(window.diskPage());
    expect(QStringLiteral("The disk lists \"Set aside by the agent\" with its size (%1)").arg(bytesText(held.bytes)),
           disk.contains(QStringLiteral("Set aside by the agent")) && disk.contains(bytesText(held.bytes)) && buttonNamed(window.diskPage(), QStringLiteral("See what is set aside")));
    save("4a-disk-with-set-aside");
    buttonNamed(window.diskPage(), QStringLiteral("See what is set aside"))->click();
    waitUntilIdle(window, 60);
    expect(QStringLiteral("its button goes to the page for what was set aside"), window.showingAfter());

    // Confinement: a record that points anywhere it should not is refused, and nothing outside is touched.
    const HeldPlace real = held.places.first();
    const QStringList zones{home};
    HeldPlace forged = real;
    forged.from = outside + QStringLiteral("/evil");
    forged.held = forged.from.mid(1);
    plant(held.root + QLatin1Char('/') + real.batch + QLatin1Char('/') + forged.held);
    Outcome refused = bringBack(home, zones, forged);
    expect(QStringLiteral("a record from outside the folders looked at is refused (%1)").arg(refused.why),
           !refused.ok && !QFileInfo::exists(forged.from) && refused.why.contains(QStringLiteral("outside the folders")));
    forged = real;
    forged.held = QStringLiteral("../../outside");
    refused = bringBack(home, zones, forged);
    expect(QStringLiteral("a held path that climbs out of the holding folder is refused (%1)").arg(refused.why), !refused.ok && refused.why.contains(QStringLiteral("plain path")));
    forged = real;
    forged.from = home + QStringLiteral("/Documents/thesis.odt");
    refused = bringBack(home, zones, forged);
    expect(QStringLiteral("a record whose held path is not under the path it came from is refused (%1)").arg(refused.why),
           !refused.ok && refused.why.contains(QStringLiteral("somewhere other")) && QFileInfo::exists(home + QStringLiteral("/Documents/thesis.odt")));
    // A folder on the way that is a link: nothing lands behind it.
    const QString link = home + QStringLiteral("/linked");
    QFile::link(outside, link);
    forged = real;
    forged.from = link + QStringLiteral("/placed");
    forged.held = forged.from.mid(1);
    plant(held.root + QLatin1Char('/') + real.batch + QLatin1Char('/') + forged.held);
    refused = bringBack(home, zones, forged);
    expect(QStringLiteral("a folder on the way that is a link is refused (%1)").arg(refused.why),
           !refused.ok && refused.why.contains(QStringLiteral("link")) && !QFileInfo::exists(outside + QStringLiteral("/placed")));
    QFile::remove(link);
    // The records made for this check are taken out of the holding folder again, so what stays there is the agent's alone.
    removeForged(held.root + QLatin1Char('/') + real.batch, forged.held);
    removeForged(held.root + QLatin1Char('/') + real.batch, outside.mid(1) + QStringLiteral("/evil"));

    // Something is there now: bringing back refuses, and keeps what is there.
    const HeldPlace blocked = window.held().places.last();
    const bool blockedDir = blocked.dir;
    QDir().mkpath(QFileInfo(blocked.from).absolutePath());
    if (blockedDir) {
        QDir().mkpath(blocked.from);
    } else {
        QFile mine(blocked.from);
        if (mine.open(QIODevice::WriteOnly))
            mine.write("mine");
    }
    window.bringBack(blocked.from);
    waitUntilIdle(window, 300);
    expect(QStringLiteral("bringing back refuses when something is there now, and says so"),
           pageText(page).contains(QStringLiteral("something is there now")) && QFileInfo::exists(blocked.from)
               && window.held().places.size() == held.places.size());
    expect(QStringLiteral("and what was there is untouched"), blockedDir ? QDir(blocked.from).isEmpty() : QFile(blocked.from).size() == 4);
    save("4b-after-refused");
    if (blockedDir) QDir().rmdir(blocked.from); else QFile::remove(blocked.from);

    // Bring one place back: it is there again, and the others stay held.
    const HeldPlace one = window.held().places.first();
    window.bringBack(one.from);
    waitUntilIdle(window, 300);
    expect(QStringLiteral("a place brought back is there again (%1)").arg(one.from), QFileInfo::exists(one.from));
    expect(QStringLiteral("and no longer held, the rest still held"),
           window.held().places.size() == held.places.size() - 1 && !QFileInfo::exists(held.root + QLatin1Char('/') + one.batch + QLatin1Char('/') + one.held));
    expect(QStringLiteral("the page says it is back, in the note and in the place's row"),
           pageText(page).contains(QStringLiteral("is back where it was")) && pageText(page).contains(QStringLiteral("Brought back: it is where it was"))
               && pageText(page).contains(QStringLiteral("Let it all go")));
    save("4c-after-one-back");

    // Bring everything back says first what it moves; Cancel moves nothing.
    const qint64 heldNow = window.held().bytes;
    const int heldPlaces = window.held().places.size();
    buttonNamed(page, QStringLiteral("Bring everything back"))->click();
    settle(250);
    QString said = pageText(window.beforePage());
    expect(QStringLiteral("bringing everything back asks first, says how much moves and that nothing is deleted"),
           window.showingBefore() && frame->selected() == "mess" && said.contains(QStringLiteral("Bring everything back?"))
               && said.contains(bytesText(heldNow)) && said.contains(QStringLiteral("Nothing is deleted"))
               && said.contains(QStringLiteral("something is at its old place")));
    save("4c2-before-bring-back");
    buttonNamed(window.beforePage(), QStringLiteral("Cancel"))->click();
    settle(200);
    expect(QStringLiteral("Cancel brings nothing back: the page shows, the holding folder is as it was"),
           window.showingAfter() && QFileInfo::exists(held.root) && window.held().bytes == heldNow && window.held().places.size() == heldPlaces);

    // Let it all go: asked first on the Before page, says how much it frees and
    // that it cannot be undone; Cancel keeps everything.
    const qint64 freeThen = statFree(home);
    buttonNamed(page, QStringLiteral("Let it all go, free %1").arg(bytesText(heldNow)))->click();
    settle(250);
    said = pageText(window.beforePage());
    expect(QStringLiteral("letting go asks first and says how much it deletes, that it cannot be undone and what it frees"),
           window.showingBefore() && frame->selected() == "mess" && said.contains(QStringLiteral("Let it all go, and free %1?").arg(bytesText(heldNow)))
               && said.contains(QStringLiteral("cannot be undone")) && said.contains(QStringLiteral("is deleted, with everything in it"))
               && said.contains(QStringLiteral("instead of")));
    save("4d0-before-let-go");
    buttonNamed(window.beforePage(), QStringLiteral("Cancel"))->click();
    settle(200);
    expect(QStringLiteral("Cancel deletes nothing"), window.showingAfter() && QFileInfo::exists(held.root) && !window.held().empty());
    buttonNamed(page, QStringLiteral("Let it all go, free %1").arg(bytesText(heldNow)))->click();
    settle(250);
    if (QPushButton *go = buttonNamed(window.beforePage(), QStringLiteral("Let it all go")))
        go->click();
    else
        expect(QStringLiteral("the Before page has a \"Let it all go\" button"), false);
    waitUntilIdle(window, 300);
    expect(QStringLiteral("the holding folder is gone"), !QFileInfo::exists(held.root) && window.held().empty());
    expect(QStringLiteral("what is beside it was not touched: the sentinel and a file of the home folder"),
           QFileInfo::exists(sentinel) && QFileInfo::exists(home + QStringLiteral("/Documents/thesis.odt")) && QFileInfo::exists(outside + QStringLiteral("/keep.txt")));
    expect(QStringLiteral("the place brought back earlier is still there"), QFileInfo::exists(one.from));
    const QString after = pageText(page);
    expect(QStringLiteral("the page says what was let go, and the room measured (%1 free now, %2 a moment before)")
               .arg(bytesText(statFree(home)), bytesText(freeThen)),
           after.contains(QStringLiteral("Nothing is set aside")) && after.contains(QStringLiteral("Let go: %1 was deleted").arg(bytesText(heldNow)))
               && after.contains(QStringLiteral("Let go: deleted from the holding folder")) && !after.contains(QStringLiteral("The manifest names it"))
               && after.contains(QStringLiteral("Brought back: it is where it was")));
    save("4d-after-let-go");
    frame->choose(QStringLiteral("disk"));
    waitUntilIdle(window, 60);
    expect(QStringLiteral("The disk lists no holding folder any more"),
           !pageText(window.diskPage()).contains(QStringLiteral("Set aside by the agent")) && !pageText(window.diskPage()).contains(QStringLiteral("See what is set aside")));
    save("4e-disk-after-let-go");
    Q_UNUSED(folder)
}


// The hand-over page says what will happen before the agent is given anything,
// shows the request and lets the person edit its expected result, and Cancel
// leaves everything as it was.
void handOverShots(Window &window, const Expect &expect)
{
    HandPage *page = window.handPage();
    const QString words = pageText(page);
    const int places = window.messPage()->chosen().size();
    expect(QStringLiteral("the hand-over is a question that counts the places (%1)").arg(places),
           words.contains(QStringLiteral("Hand these %1 places to the agent?").arg(places)) && words.contains(QStringLiteral("Nothing has started.")));
    expect(QStringLiteral("it says what happens: what is asked, that nothing is deleted before the person lets go, who it works as, the room"),
           words.contains(QStringLiteral("What happens")) && words.contains(QStringLiteral("The agent is asked to clear"))
               && words.contains(QStringLiteral("Nothing is gone until you let it go")) && words.contains(QStringLiteral("without administrator rights"))
               && words.contains(QStringLiteral("is free")));
    expect(QStringLiteral("it says which places it asks about first"), words.contains(QStringLiteral("It asks you first about")));
    expect(QStringLiteral("the request is still there: the issue, the facts and the expected result"),
           words.contains(QStringLiteral("The issue")) && words.contains(QStringLiteral("The facts")) && words.contains(QStringLiteral("The expected result")));
    expect(QStringLiteral("it has Cancel and \"Hand it over\""), buttonNamed(page, QStringLiteral("Cancel")) && buttonNamed(page, QStringLiteral("Hand it over")));
    // The expected result is the person's to change, and the change goes into the request.
    QPlainTextEdit *expected = page->findChild<QPlainTextEdit *>();
    expect(QStringLiteral("the expected result can be edited, and the request carries the edit"), expected && [&] {
        expected->appendPlainText(QStringLiteral("The person's own line."));
        return page->prompt().contains(QStringLiteral("The person's own line."));
    }());
    if (expected)
        expected->undo();
    // Cancel hands nothing over and moves nothing.
    QStringList there;
    for (const Finding &finding : window.messPage()->chosen())
        there << finding.path;
    const QString holdingBase = window.home() + QStringLiteral("/.local/share/levitate/set-aside");
    QPushButton *cancel = buttonNamed(page, QStringLiteral("Cancel"));
    if (cancel)
        cancel->click();
    settle(200);
    bool allThere = true;
    for (const QString &path : there)
        allThere = allThere && QFileInfo::exists(path);
    expect(QStringLiteral("Cancel hands nothing over: the mess page shows, no agent runs, nothing moved, no holding folder"),
           window.showingMess() && !window.busy() && allThere && !QFileInfo::exists(holdingBase));
    window.showHandOver();
    settle(200);
}


// Comparing says what it does first; Cancel changes nothing; the button starts it.
void compareShots(Window &window, const Save &save, const Expect &expect)
{
    const bool had = window.comparison() != nullptr;
    window.askCompare();
    settle(250);
    const QString words = pageText(window.beforePage());
    expect(QStringLiteral("comparing asks first, on the Before page, under The two systems"),
           window.showingBefore() && window.frame()->selected() == "systems" && words.contains(QStringLiteral("Nothing has started."))
               && !window.comparing());
    expect(QStringLiteral("it says both systems are read from start to end, neither is changed, it takes a few minutes, and the command that runs"),
           words.contains(QStringLiteral("read from start to end")) && words.contains(QStringLiteral("Neither system is changed"))
               && words.contains(QStringLiteral("a few minutes")) && words.contains(QStringLiteral("compare --json --machine-events")));
    // The stories run as the person, so no password is asked, and the page says so.
    expect(QStringLiteral("it says the truth about the password (%1)").arg(qEnvironmentVariable("DISK_GUI_ROOT_HELPER").isEmpty() ? "none asked" : "asked"),
           qEnvironmentVariable("DISK_GUI_ROOT_HELPER").isEmpty() ? words.contains(QStringLiteral("no password is asked"))
                                                                  : words.contains(QStringLiteral("administrator password")));
    save("5a-before-compare");
    if (QPushButton *cancel = buttonNamed(window.beforePage(), QStringLiteral("Cancel")))
        cancel->click();
    settle(200);
    expect(QStringLiteral("Cancel compares nothing: The two systems shows, nothing runs, the kept comparison is as it was"),
           window.showingSystems() && !window.comparing() && !window.busy() && (window.comparison() != nullptr) == had);
    window.askCompare();
    settle(250);
    if (QPushButton *go = buttonNamed(window.beforePage(), QStringLiteral("Compare")))
        go->click();
    else
        expect(QStringLiteral("the Before page has a \"Compare\" button"), false);
}

// What the agent is asked about the two systems: said first, Cancel asks nothing.
void explainShots(Window &window, const Save &save, const Expect &expect)
{
    window.showExplain();
    settle(250);
    const QString words = pageText(window.handPage());
    expect(QStringLiteral("asking the agent what differs says first what it is given and that it changes nothing"),
           words.contains(QStringLiteral("Ask the agent what differs?")) && words.contains(QStringLiteral("Nothing has started."))
               && words.contains(QStringLiteral("told to change nothing")) && words.contains(QStringLiteral("The expected result"))
               && words.contains(QStringLiteral("Ask the agent")));
    save("7-ask");
    if (QPushButton *cancel = buttonNamed(window.handPage(), QStringLiteral("Cancel")))
        cancel->click();
    settle(200);
    expect(QStringLiteral("Cancel asks the agent nothing: The two systems shows, nothing runs, no answer"),
           window.showingSystems() && !window.busy() && window.said().isEmpty());
    window.showExplain();
    settle(200);
}

} // namespace

namespace {

// Runs that outlive their window: found, followed, answered and stopped from
// Activity. Each one is started by a window of Disk's own (the hand-over of the
// mess, with the helper as its agent) that then closes.
void liveShots(Window &window, const QString &folder, const Save &save, const Expect &expect)
{
    const QString script = qEnvironmentVariable("RECAB_GUI_RECORDS");
    const QString agent = qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT");
    if (script.isEmpty() || agent.isEmpty()) {
        expect(QStringLiteral("the live checks need RECAB_GUI_RECORDS and LEVITATE_ACTIVITY_AGENT, see README.md"), false);
        return;
    }
    livestory::prepare(QDir(folder).filePath("live-disk"), QFileInfo(script).absolutePath());
    auto start = [&](const QString &story, const QString &request) {
        livestory::useStory(story);
        const QByteArray before = qgetenv("DISK_GUI_AGENT");
        qputenv("DISK_GUI_AGENT", agent.toUtf8());
        auto *started = new Window;
        qputenv("DISK_GUI_AGENT", before);
        started->show();
        waitUntilIdle(*started, 600);
        started->send(QStringLiteral("The issue:\n%1\n").arg(request));
        return livestory::Started{started, started->frame()};
    };
    livestory::run(*window.frame(), window, start, [&](QWidget &, const QString &name) { save(name); }, expect, "a3");
    livestory::finish();
    window.frame()->choose(QStringLiteral("mess"));
    settle(100);
}

} // namespace

int takeShots(Window &window, const QString &folder, bool handOver)
{
    QDir().mkpath(folder);
    QTextStream out(stdout);
    bool ok = true;
    auto expect = [&](const QString &what, bool holds) {
        if (!holds) {
            out << "not as expected: " << what << "\n";
            ok = false;
        }
    };
    // Every picture is of the window at its size: no page may make it wider or taller.
    auto save = [&](const QString &name) {
        settle(200);
        expect(QStringLiteral("%1 fits the window (%2 x %3)").arg(name).arg(window.width()).arg(window.height()),
               window.width() == 1100 && window.height() == 700);
        ok &= window.grab().save(QDir(folder).filePath(name + ".png"));
    };
    const QString ownRecab = qEnvironmentVariable("DISK_GUI_RECAB", "recab");
    QString records;
    // The runs the records fixture leaves going are stopped when this ends, whichever way it ends.
    struct Stopper {
        const QString &records;
        ~Stopper()
        {
            if (!records.isEmpty())
                QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        }
    } stopper{records};
    window.show();
    // A picture run that is not told of its stand-ins refuses to start.
    livestory::guardRefuses(expect, folder, "DISK_GUI_RECAB");
    waitUntilIdle(window, 600);
    qint64 mess = 0;
    for (const Finding &finding : window.found())
        mess += finding.bytes;
    out << "looked at " << bytesText(window.scannedBytes()) << ", " << window.found().size() << " places of mess, "
        << bytesText(mess) << "\n";
    for (const Finding &finding : window.found())
        out << "  " << bytesText(finding.bytes) << "  " << finding.what << "  " << finding.path << "\n";
    Frame *frame = window.frame();
    expect(QStringLiteral("the window is 1100 x 700"), window.width() == 1100 && window.height() == 700);
    expect(QStringLiteral("the rail offers The disk, Browse, Mess, The two systems, Activity, Settings and Help, nothing else (%1)")
               .arg(frame->entryTitles().join(QStringLiteral(", "))),
           frame->entryTitles() == QStringList({"The disk", "Browse", "Mess", "The two systems", "Activity", "Settings", "Help"}));
    expect(QStringLiteral("the Mess badge is the number of places listed (%1 and %2)")
               .arg(frame->badge("mess")).arg(window.found().size()),
           frame->badge("mess") == window.found().size() && window.found().size() > 0);
    expect(QStringLiteral("the status line says when the disk was looked at, and what deletes (%1)").arg(frame->statusText()),
           frame->statusText().startsWith(QStringLiteral("Looked at the disk at ")) && frame->statusText().contains(QStringLiteral("Let it all go"))
               && !frame->statusText().contains(QStringLiteral("only reads")));
    expect(QStringLiteral("the status line names the agent (%1)").arg(frame->agentText()),
           frame->agentText().contains(QFileInfo(qEnvironmentVariable("DISK_GUI_AGENT")).fileName()));
    expect(QStringLiteral("no strip while recab's status cannot be read as the real one"), !frame->stripShown());
    diskShots(window, save, expect, out);
    // "Look again" reads everything again, and the page is as true after it.
    if (QPushButton *again = buttonNamed(window.diskPage(), QStringLiteral("Look again")))
        again->click();
    settle(100);
    waitUntilIdle(window, 300);
    expect(QStringLiteral("Look again reads again and the page shows the same leftovers"),
           pageText(window.diskPage()).contains(bytesText(sumOf(window.found()))) && window.layout().found);
    settingsShots(window, folder, save, expect);
    helpShots(window, folder, save, expect);
    browseShots(window, folder, save, expect, out);
    messShots(window, save, expect);

    const QString picked = window.pickBiggestUnnamed();
    if (!picked.isEmpty())
        out << "picked by hand: " << picked << "\n";
    frame->choose(QStringLiteral("mess"));
    expect(QStringLiteral("what was picked by hand is in the Mess list"),
           pageText(window.messPage()).contains(QStringLiteral("Picked by you")) && listsPath(window, picked));
    save("3-picked");

    window.showHandOver();
    expect(QStringLiteral("the hand-over belongs to Mess"), frame->selected() == "mess");
    const QString holding = window.held().root;
    save("3-hand-over");
    handOverShots(window, expect);
    stripShots(window, save, expect, out);
    window.showHandOver();
    settle(100);
    expect(QStringLiteral("the strip shows on the hand-over too"), frame->stripShown());
    if (!handOver) {
        // What the entry opens with: the kept comparison when there is one,
        // else what recab says is there.
        frame->choose(QStringLiteral("systems"));
        waitUntilIdle(window, 30);
        expect(QStringLiteral("The two systems is marked and shows"),
               frame->selected() == "systems" && window.showingSystems());
        save("4-systems");
        out << "the two systems: " << (window.comparison() ? "a kept comparison is shown" : "not compared")
            << (window.stale() ? ", marked as out of date" : "") << "\n";
        activityShots(window, folder, false, save, expect, out, &records);
        frame->choose(QStringLiteral("mess"));
        expect(QStringLiteral("Mess is marked and shows again"), frame->selected() == "mess" && window.showingMess());
        noStripShots(window, ownRecab, expect);
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        liveShots(window, folder, save, expect);
        out << (ok ? "all as expected\n" : "some things were not as expected\n");
        return ok ? 0 : 1;
    }

    // Two things the story puts beside the holding folder, to see that nothing outside it is touched: a
    // file of the sample's own, in the folder that holds the holding folder, and a folder outside the sample.
    const QString sentinel = QFileInfo(holding).absolutePath() + QStringLiteral("/sentinel.txt");
    QDir().mkpath(QFileInfo(sentinel).absolutePath());
    { QFile f(sentinel); if (f.open(QIODevice::WriteOnly)) f.write("beside the holding folder"); }
    const QString outside = QDir(folder).filePath(QStringLiteral("outside"));
    QDir().mkpath(outside);
    { QFile f(outside + QStringLiteral("/keep.txt")); if (f.open(QIODevice::WriteOnly)) f.write("outside the sample"); }
    const qint64 freeBefore = statFree(window.home());

    window.sendShown();
    // The agent asks before it moves what is marked for that: yes to the
    // first, no to the rest.
    int asked = 0;
    for (QElapsedTimer clock = [] { QElapsedTimer t; t.start(); return t; }(); window.busy() && clock.elapsed() < 300000;) {
        settle(50);
        if (!window.agentAsks())
            continue;
        if (!asked) {
            save("3b-the-agent-asks");
            expect(QStringLiteral("while the agent works the Mess entry is marked"), frame->selected() == "mess");
            activityShots(window, folder, true, save, expect, out, &records);
            expect(QStringLiteral("the agent still asks on the work page"), window.agentAsks());
        }
        window.answerAgent(asked++ ? QStringLiteral("Keep it") : QStringLiteral("Remove it"));
    }
    settle(200);
    out << "the agent asked " << asked << " times\n";
    ok &= asked > 0;
    waitUntilIdle(window, 300); // the second look, and what was set aside
    expect(QStringLiteral("the badge follows the second look (%1 and %2)").arg(frame->badge("mess")).arg(window.found().size()),
           frame->badge("mess") == window.found().size());
    afterShots(window, folder, save, expect, out, freeBefore, sentinel, outside);

    // The two systems: compare, then ask the agent what differs.
    frame->choose(QStringLiteral("systems"));
    waitUntilIdle(window, 30);
    ok &= window.state().known;
    expect(QStringLiteral("The two systems is marked and shows"), frame->selected() == "systems" && window.showingSystems());
    save("5-systems");
    // Slowly once, to see the two sides fill while they are read.
    const QByteArray speed = qgetenv("DISK_STANDIN_SPEED");
    qputenv("DISK_STANDIN_SPEED", "1");
    compareShots(window, save, expect);
    settle(3200);
    ok &= window.comparing();
    expect(QStringLiteral("while comparing, the agent's place says so (%1)").arg(frame->agentText()),
           frame->agentText().contains(QStringLiteral("Comparing")));
    save("5b-comparing");
    qputenv("DISK_STANDIN_SPEED", speed);
    waitUntilIdle(window, 300);
    const Comparison *compared = window.comparison();
    if (!compared) {
        out << "the two systems could not be compared\n";
        save("6-compared");
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        return 1;
    }
    out << "the two systems: " << compared->count() << " differences, " << bytesText(compared->differingBytes())
        << "; " << compared->sameFiles << " files the same\n";
    expect(QStringLiteral("the two maps and the list fit the window without making it wider (%1 x %2)")
               .arg(window.width()).arg(window.height()),
           window.width() == 1100 && window.height() == 700);
    save("6-compared");
    // The partition of the other system now says what its files took.
    frame->choose(QStringLiteral("disk"));
    waitUntilIdle(window, 60);
    expect(QStringLiteral("The disk says what the other system's files took when compared"),
           pageText(window.diskPage()).contains(QStringLiteral("its files took %1 when compared").arg(bytesText(compared->other.bytes))));
    save("6b-disk-compared");
    if (compared->identical()) {
        if (!records.isEmpty())
            QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
        return ok ? 0 : 1;
    }
    frame->choose(QStringLiteral("systems"));
    explainShots(window, save, expect);
    window.sendShown();
    waitUntilIdle(window, 300);
    out << "the agent said: " << window.said() << "\n";
    ok &= !window.said().isEmpty();
    expect(QStringLiteral("after the answer The two systems shows"), frame->selected() == "systems" && window.showingSystems());
    save("8-explained");
    noStripShots(window, ownRecab, expect);
    if (!records.isEmpty())
        QProcess::execute(qEnvironmentVariable("RECAB_GUI_RECORDS"), {"--stop", records});
    out << (ok ? "all as expected\n" : "some things were not as expected\n");
    return ok ? 0 : 1;
}
