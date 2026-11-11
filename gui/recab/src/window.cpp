#include "window.h"

#include "pages.h"

#include "beforewords.h"

#include "activity.h"
#include "frame.h"
#include "refresh.h"

#include <QFile>
#include <QInputDialog>
#include <QLineEdit>
#include <QJsonArray>
#include <QFileDialog>
#include <QTimer>
#include <QLocale>
#include <QMessageBox>
#include <QPushButton>
#include <QStackedWidget>
#include <QTime>
#include <QToolButton>

namespace {

// What Activity calls a run: the first line under "The request:", else under
// "The issue:", else the title of the page that shows it.
QString requestTitle(const QString &prompt, const QString &fallback)
{
    for (const QString &heading : {QStringLiteral("The request:"), QStringLiteral("The issue:")}) {
        const qsizetype at = prompt.indexOf(heading);
        if (at < 0)
            continue;
        const QString line = prompt.mid(at + heading.size()).trimmed().section(QLatin1Char('\n'), 0, 0).trimmed();
        if (!line.isEmpty())
            return line.size() > 90 ? line.left(89).trimmed() + QStringLiteral("…") : line;
    }
    return fallback;
}

// What the work page is titled for one of recab's operations: its label is
// written for a list of buttons, and is long for a heading.
QString workTitle(const QJsonObject &op)
{
    const QString id = op["id"].toString();
    if (id == QLatin1String("update_check"))
        return QObject::tr("Looking for a new release");
    if (id == QLatin1String("update_download"))
        return QObject::tr("Downloading the release");
    if (id == QLatin1String("update_apply") || id == QLatin1String("update_media"))
        return QObject::tr("Adding the release to the next system");
    if (id == QLatin1String("update_recover"))
        return QObject::tr("Carrying on adding the release");
    if (id == QLatin1String("update_cancel"))
        return QObject::tr("Taking the release out of the next system");
    return op["label"].toString();
}

} // namespace

Window::Window(QWidget *parent)
    : QMainWindow(parent)
{
    setWindowTitle(tr("System"));

    stack_ = new QStackedWidget;
    system_ = new SystemPage;
    next_ = new NextPage;
    back_ = new BackPage;
    historyPage_ = new HistoryPage;
    release_ = new ReleasePage;
    ask_ = new AskPage;
    work_ = new WorkPage;
    clash_ = new ClashPage;
    proposed_ = new ProposedPage;
    before_ = new BeforePage;
    for (QWidget *page : {static_cast<QWidget *>(system_), static_cast<QWidget *>(next_), static_cast<QWidget *>(back_),
                          static_cast<QWidget *>(historyPage_), static_cast<QWidget *>(release_),
                          static_cast<QWidget *>(ask_), static_cast<QWidget *>(work_),
                          static_cast<QWidget *>(clash_), static_cast<QWidget *>(proposed_), static_cast<QWidget *>(before_)})
        stack_->addWidget(page);

    // The frame is the window: rail, the pages above, the strip, the status line.
    frame_ = new Frame(Frame::System);
    frame_->setContent(stack_);
    frame_->addEntry(QStringLiteral("overview"), tr("Overview"));
    frame_->addEntry(QStringLiteral("next"), tr("The next system"));
    frame_->addEntry(QStringLiteral("history"), tr("History"));
    frame_->addEntry(QStringLiteral("back"), tr("Getting back"));
    frame_->select(QStringLiteral("overview"));
    section_ = QStringLiteral("overview");
    frame_->setAgent(client_.agentProgram());
    frame_->setStatus(tr("Read from recab on this machine"));
    setCentralWidget(frame_);
    connect(frame_, &Frame::chosen, this, [this](const QString &key) {
        if (key == Frame::activityKey())
            updateStrip();
        else if (key == QLatin1String("next"))
            showNext();
        else if (key == QLatin1String("history"))
            showHistory();
        else if (key == QLatin1String("back"))
            showBack();
        else
            showMain();
    });
    // The strip's button goes where the next system is finished.
    connect(frame_, &Frame::finishRequested, this, [this] {
        frame_->select(QStringLiteral("next"));
        showNext();
    });
    connect(frame_, &Frame::goToRun, this, [this] {
        frame_->select(section_);
        display(work_);
    });

    connect(system_, &SystemPage::ask, this, [this] { showAsk(); });
    connect(system_, &SystemPage::operation, this, [this](const QJsonObject &op) { runOperation(op); });
    connect(next_, &NextPage::operation, this, [this](const QJsonObject &op) { runOperation(op); });
    connect(system_, &SystemPage::addUpdate, this, [this] { addUpdate(); });
    connect(system_, &SystemPage::goBack, this, [this] { goBack(); });
    connect(system_, &SystemPage::keep, this, [this] { keep(); });
    connect(system_, &SystemPage::restartLater, this, [this] { restartLater(); });
    connect(system_, &SystemPage::cancelRestart, this, [this] { cancelRestart(); });
    connect(system_, &SystemPage::discard, this, [this] { discard(); });
    connect(system_, &SystemPage::reset, this, [this] { reset(); });
    connect(system_, &SystemPage::finish, this, [this] { finish(); });
    connect(system_, &SystemPage::restart, this, [this] { restart(); });
    connect(system_, &SystemPage::retry, this, [this] { restart(); });
    connect(system_, &SystemPage::recover, this, [this] { recover(); });
    connect(system_, &SystemPage::settle, this, &Window::showClash);
    connect(system_, &SystemPage::fix, this,
            [this](const QString &prompt) { fixWithAgent(prompt, tr("The agent is changing the next system")); });

    connect(system_, &SystemPage::openNext, this, [this] {
        frame_->select(QStringLiteral("next"));
        showNext();
    });
    connect(system_, &SystemPage::openBack, this, [this] {
        frame_->select(QStringLiteral("back"));
        showBack();
    });

    connect(system_, &SystemPage::openRelease, this, &Window::showRelease);
    connect(next_, &NextPage::openRelease, this, &Window::showRelease);
    connect(release_, &ReleasePage::back, this, &Window::showSection);
    connect(release_, &ReleasePage::operation, this, [this](const QString &id) { runReleaseOperation(id); });
    connect(release_, &ReleasePage::add, this, [this] { addUpdate(); });
    connect(release_, &ReleasePage::addFromFile, this, [this] { addFromFile(); });
    connect(release_, &ReleasePage::settle, this, &Window::showClash);
    connect(release_, &ReleasePage::adoptBaseline, this, [this] { adoptBaseline(); });
    connect(release_, &ReleasePage::reproduce, this, [this](const QString &package) { reproduce(package); });
    connect(release_, &ReleasePage::replaceFailed, this, [this] { replaceFailed(); });
    connect(system_, &SystemPage::replaceFailed, this, [this] { replaceFailed(); });
    connect(clash_, &ClashPage::adoptBaseline, this, [this] { adoptBaseline(); });
    connect(clash_, &ClashPage::reproduce, this, [this](const QString &package) { reproduce(package); });
    connect(release_, &ReleasePage::finish, this, [this] {
        frame_->select(QStringLiteral("next"));
        showNext();
    });
    connect(historyPage_, &HistoryPage::readAgain, this, &Window::showHistory);
    connect(historyPage_, &HistoryPage::saveAsFile, this, [this] {
        const QString path = QFileDialog::getSaveFileName(this, tr("Save the history"), QStringLiteral("history.txt"),
                                                          tr("Text (*.txt)"));
        if (!path.isEmpty() && !saveHistory(path))
            QMessageBox::warning(this, tr("System"), tr("The history could not be saved to %1.").arg(path));
    });

    connect(next_, &NextPage::ask, this, [this] { showAsk(); });
    connect(next_, &NextPage::installSoftware, this, &Window::installSoftware);
    connect(next_, &NextPage::reset, this, [this] { reset(); });
    connect(next_, &NextPage::settle, this, &Window::showClash);
    connect(next_, &NextPage::finish, this, [this] { finish(); });
    connect(next_, &NextPage::restart, this, [this] { restart(); });
    connect(next_, &NextPage::restartLater, this, [this] { restartLater(); });
    connect(next_, &NextPage::cancelRestart, this, [this] { cancelRestart(); });
    connect(next_, &NextPage::recover, this, [this] { recover(); });
    connect(next_, &NextPage::discard, this, [this] { discard(); });
    connect(next_, &NextPage::keep, this, [this] { keep(); });
    connect(next_, &NextPage::goBack, this, [this] { goBack(); });
    connect(next_, &NextPage::openDisk, this, [this] { frame_->startApp(Frame::Disk); });
    connect(next_, &NextPage::openOverview, this, [this] {
        frame_->select(QStringLiteral("overview"));
        showMain();
    });

    connect(back_, &BackPage::goBack, this, [this] { goBack(); });
    connect(back_, &BackPage::keep, this, [this] { keep(); });
    connect(back_, &BackPage::cancelRestart, this, [this] { cancelRestart(); });
    connect(back_, &BackPage::saveSteps, this, [this] {
        const QString path = QFileDialog::getSaveFileName(this, tr("Save the steps"), QStringLiteral("start-up-steps.txt"),
                                                          tr("Text (*.txt)"));
        if (!path.isEmpty() && !saveSteps(path))
            QMessageBox::warning(this, tr("System"), tr("The steps could not be saved to %1.").arg(path));
    });

    connect(ask_, &AskPage::back, this, &Window::showSection);
    connect(ask_, &AskPage::handOver, this, [this](const QString &prompt) { handOver(prompt); });

    connect(work_, &WorkPage::back, this, &Window::showSection);
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        frame_->setAgentNote(words);
    });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
        frame_->setOwnRun(QString()); // Activity reads the run's record again
        auto then = then_;
        then_ = nullptr;
        reload([then, ok] {
            if (then)
                then(ok);
        });
    });

    connect(clash_, &ClashPage::later, this, &Window::showSection);
    connect(clash_, &ClashPage::byHand, this, &Window::decideByHand);
    connect(clash_, &ClashPage::fix, this,
            [this](const QString &prompt) { fixWithAgent(prompt, tr("The agent is settling the clash")); });

    // The Before page: Cancel leaves everything as it was; the button does it.
    connect(before_, &BeforePage::cancelled, this, [this] {
        remake_ = nullptr;
        go_ = nullptr;
        showSection();
    });
    connect(before_, &BeforePage::confirmed, this, [this] {
        const auto go = go_;
        remake_ = nullptr;
        go_ = nullptr;
        if (go)
            go();
    });

    connect(proposed_, &ProposedPage::accept, this, &Window::acceptProposal);
    connect(proposed_, &ProposedPage::askAgain, this, &Window::showClash);
    connect(proposed_, &ProposedPage::byHand, this, &Window::decideByHand);

    connect(system_, &SystemPage::readAgain, this, [this] { reload([this] { showSection(); }); });
    connect(system_, &SystemPage::readAsAdministrator, this, [this] {
        asRoot_ = true;
        reload([this] { showSection(); });
    });

    // F5, and coming back to the window after a while: an agent or a terminal
    // may have changed the system in the meantime.
    refresher_ = new Refresher(this, 20);
    connect(refresher_, &Refresher::asked, this, [this](bool byHand) {
        if (stack_->currentWidget() == system_ || stack_->currentWidget() == next_ || stack_->currentWidget() == back_
            || stack_->currentWidget() == historyPage_ || stack_->currentWidget() == release_ || byHand)
            refresh();
    });
    connect(refresher_, &Refresher::asked, frame_, &Frame::readAgain);
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("Read the system again (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    frame_->addStatusWidget(again);

    reload([this] { showSection(); });
}

void Window::refresh()
{
    if (busy())
        return;
    // Only the pages drawn from the state alone are drawn again; a page being
    // filled in is left as it is. The page is the one shown when the answer
    // comes, not when it was asked: the person may have gone on meanwhile.
    reload([this] {
        QWidget *shown = stack_->currentWidget();
        const bool main = shown == system_ || shown == next_ || shown == back_ || shown == historyPage_;
        const bool release = shown == release_;
        const bool before = shown == before_;
        if (main)
            showSection();
        else if (release)
            showRelease();
        else if (before && remake_)
            before_->show(remake_()); // the words are for the state as read now
    });
}

void Window::reload(std::function<void()> then)
{
    if (reading_)
        return;
    reading_ = client_.read(asRoot_);
    if (!read_) {
        system_->showLoading(tr("Reading the system"),
                             tr("Asking recab what is running, what is beside it and what can be done."));
        display(system_);
    }
    frame_->setStatus(tr("Reading the system…"));
    connect(reading_, &Run::finished, this, [this, then](bool ok) {
        Run *run = reading_;
        reading_ = nullptr;
        run->deleteLater();
        QString problem;
        Status status;
        bool permission = false;
        if (!client_.understand(run, ok, &status, &problem, &permission)) {
            frame_->setStatus(tr("The system could not be read"));
            system_->showProblem(problem, permission && !asRoot_);
            display(system_);
            asRoot_ = false; // a refused password is not asked for again unasked
            return;
        }
        status_ = status;
        read_ = true;
        // The number of changes the next system holds, when it exists.
        frame_->setBadge(QStringLiteral("next"),
                         status_.otherRole == QLatin1String("next") ? int(status_.changes.size()) : 0);
        updateStrip();
        refresher_->fresh();
        const QString at = QLocale().toString(QTime::currentTime(), QLocale::ShortFormat);
        frame_->setStatus(status_.state == QLatin1String("working")
                              ? tr("Read from recab at %1, and again every two seconds while a command runs").arg(at)
                              : tr("Read from recab at %1").arg(at));
        if (then)
            then();
        // A command started elsewhere is working: look again until it is done.
        if (status_.state == QLatin1String("working"))
            QTimer::singleShot(2000, this, &Window::refresh);
    });
    reading_->start();
}

bool Window::agentAsks() const
{
    return run_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

void Window::display(QWidget *page)
{
    stack_->setCurrentWidget(page);
    // The three pages are their own entries; the pages in between belong to
    // the one the person came from.
    if (page == system_)
        section_ = QStringLiteral("overview");
    else if (page == next_)
        section_ = QStringLiteral("next");
    else if (page == back_)
        section_ = QStringLiteral("back");
    else if (page == historyPage_)
        section_ = QStringLiteral("history");
    frame_->follow(section_);
    updateStrip();
}

void Window::updateStrip()
{
    // The next system's page and Overview say it themselves; every other
    // page, and Activity, gets the strip.
    frame_->setStripAllowed(frame_->activityShown()
                            || (stack_->currentWidget() != system_ && stack_->currentWidget() != next_));
    frame_->setNextSystem(status_.otherRole == QLatin1String("next") ? status_.changeTitles() : QStringList());
}

void Window::showMain()
{
    system_->show(status_);
    display(system_);
}

void Window::showNext()
{
    next_->show(status_);
    display(next_);
}

void Window::showBack()
{
    back_->show(status_);
    display(back_);
}

void Window::showHistory()
{
    if (readingHistory_)
        return;
    historyPage_->showLoading(tr("Reading the history"), tr("Asking recab what it has recorded."));
    display(historyPage_);
    readingHistory_ = client_.readHistory();
    connect(readingHistory_, &Run::finished, this, [this](bool ok) {
        Run *run = readingHistory_;
        History history;
        const bool read = client_.understandHistory(run, ok, &history, status_.preparationId);
        history_ = history;
        readingHistory_ = nullptr;
        run->deleteLater();
        if (read)
            historyPage_->show(history);
        else
            historyPage_->showUnavailable(history);
    });
    readingHistory_->start();
}

void Window::showRelease()
{
    release_->show(status_);
    display(release_);
}

bool Window::saveHistory(const QString &path)
{
    QFile file(path);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate))
        return false;
    return file.write(history_.text().toUtf8()) >= 0;
}

void Window::showSection()
{
    if (section_ == QLatin1String("next"))
        showNext();
    else if (section_ == QLatin1String("history"))
        showHistory();
    else if (section_ == QLatin1String("back"))
        showBack();
    else
        showMain();
}

void Window::installSoftware()
{
    frame_->startApp(Frame::Recipe);
}

bool Window::saveSteps(const QString &path)
{
    QFile file(path);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate))
        return false;
    return file.write(backStepsText(status_).toUtf8()) >= 0;
}

void Window::showAsk(const QString &request)
{
    ask_->show(status_, request);
    display(ask_);
}

void Window::showClash()
{
    clash_->show(status_);
    display(clash_);
}

void Window::askBefore(std::function<BeforePage::Content()> content, std::function<void()> go)
{
    remake_ = content;
    go_ = std::move(go);
    before_->show(content());
    display(before_);
}

void Window::sectionUnlessStopped(bool ok)
{
    if (ok || !work_->saysWhy())
        showSection();
}

// Run one long thing on the work page, then read the system again.
void Window::work(Run *run, const QString &title, const QString &subtitle, std::function<void(bool)> then)
{
    run_ = run;
    then_ = std::move(then);
    work_->begin(title, subtitle, run);
    display(work_);
    run->start();
    frame_->setOwnRun(run);
}

// Short changes with no page of their own, one after another.
void Window::chain(QList<Run *> runs, const QString &made, bool restartLast, bool first)
{
    if (runs.isEmpty()) {
        reload([this] { showSection(); });
        return;
    }
    Run *run = runs.takeFirst();
    setEnabled(false);
    ++quiet_;
    connect(run, &Run::finished, this, [this, run, runs, first, made, restartLast](bool ok) {
        setEnabled(true);
        --quiet_;
        const QString reason = run->failure()["reason"].toString();
        const QString said = run->problem();
        run->deleteLater();
        if (!ok) {
            for (Run *left : runs)
                left->deleteLater();
            // A later step is the restart (no JSON, no reason): what came before it was done.
            const bool restart = restartLast && runs.isEmpty();
            QString words;
            if (restart)
                words = tr("The computer did not restart. %1 The page shows where things stand now.")
                            .arg(made.isEmpty() ? tr("The step before the restart was made.") : made)
                        + (said.isEmpty() ? QString() : tr(" It said: %1").arg(said));
            else
                words = !reason.isEmpty() ? reason : first ? tr("The change was not made.")
                                                           : tr("A step after the first was not made. The page shows "
                                                                "where things stand now.");
            QMessageBox box(QMessageBox::Warning, tr("System"), words, QMessageBox::Ok, this);
            box.exec();
            reload([this] { showSection(); });
            return;
        }
        chain(runs, made, restartLast, false);
    });
    run->start();
}

void Window::handOver(const QString &prompt, bool askFirst)
{
    // Only the reuse of the system from before is asked: the request itself
    // was read on the Ask page, and the agent's page says what it is given.
    if (askFirst && status_.otherRole == QLatin1String("previous")) {
        askBefore([this] { return beforewords::reuse(client_, status_); }, [this, prompt] { handOver(prompt, false); });
        return;
    }
    work(client_.agent(prompt, requestTitle(prompt, tr("Preparing the next system"))), tr("Preparing the next system"),
         tr("The running system is not touched. You can keep working."), [this](bool ok) { sectionUnlessStopped(ok); });
}

void Window::fixWithAgent(const QString &prompt, const QString &title)
{
    work(client_.agent(prompt, requestTitle(prompt, title)), title, tr("It works in the next system only. The running system is not touched."),
         [this](bool ok) {
             if (ok && !status_.proposalPath.isEmpty()) {
                 proposed_->show(status_);
                 display(proposed_);
             } else {
                 sectionUnlessStopped(ok);
             }
         });
}

// What an operation says on the Before page, by the operation's id; the pages
// above cover what the window names itself, the rest is recab's own label.
BeforePage::Content Window::beforeOperation(const QJsonObject &op) const
{
    const QString id = op["id"].toString();
    if (!op["inputs"].toArray().isEmpty())
        return {}; // asks for values first; those dialogs are the question
    if (id == QLatin1String("reboot"))
        return beforewords::restartComputer(client_, status_);
    if (id == QLatin1String("discard"))
        return beforewords::discard(client_, status_);
    if (id == QLatin1String("finalize"))
        return beforewords::finish(client_, status_);
    if (id == QLatin1String("set_next"))
        return beforewords::restartLater(client_, status_);
    if (id == QLatin1String("commit"))
        return status_.state == QLatin1String("trial_pending") ? beforewords::cancelRestart(client_, status_)
                                                               : beforewords::keep(client_, status_);
    if (id == QLatin1String("update_apply"))
        return beforewords::addRelease(client_, status_, false);
    if (id == QLatin1String("update_cancel") || id == QLatin1String("update_recover"))
        return beforewords::releaseStep(client_, status_, id);
    if (id == QLatin1String("prepare") || id == QLatin1String("sync_recover"))
        return beforewords::recover(client_, status_);
    if (id == QLatin1String("replace_failed"))
        return beforewords::replaceFailed(client_, status_, op);
    // update_check and update_download only look and fetch.
    if (id == QLatin1String("update_check") || id == QLatin1String("update_download"))
        return {};
    return beforewords::operation(client_, status_, op);
}

void Window::runOperation(const QJsonObject &op, const QStringList &extra, std::function<void(bool)> after, bool askFirst) {
    if(busy() || op["verdict"]!="ready")return;
    // The recovery list's own buttons: the two that ask for values go the way the pages go.
    if (askFirst && op["id"] == "adopt_baseline") {
        adoptBaseline();
        return;
    }
    if (askFirst && op["id"] == "update_reproduce") {
        QStringList packages;
        for (const MergeConflict &c : status_.update.open())
            if (c.isPackageRebuild())
                packages << c.path;
        QString package = packages.value(0);
        if (packages.size() > 1) {
            bool chosen = false;
            package = QInputDialog::getItem(this, op["label"].toString(), tr("Which package?"), packages, 0, false, &chosen);
            if (!chosen)
                return;
        }
        reproduce(package);
        return;
    }
    if (askFirst && !beforeOperation(op).title.isEmpty()) {
        const QString id = op["id"].toString();
        askBefore(
            [this, op, id] {
                const QJsonObject now = status_.operationObject(id);
                return beforeOperation(now.isEmpty() ? op : now);
            },
            [this, op, extra, after] { runOperation(op, extra, after, false); });
        return;
    }
    if(op["id"]=="reboot"){chain({client_.reboot()}, tr("Nothing else was changed."), true);return;}
    QStringList args{"catalog"};for(auto v:op["argv"].toArray())args<<v.toString();
    args<<extra;
    QString choice;
    for(const auto &v:op["inputs"].toArray()){
        const auto field=v.toObject();if(field["name"]=="file" && !choice.isEmpty() && choice!="edited")continue;
        QString value;bool ok=false;QStringList choices;for(auto c:field["choices"].toArray())choices<<c.toString();
        if(choices.isEmpty())value=QInputDialog::getText(this,op["label"].toString(),field["label"].toString(),QLineEdit::Normal,QString(),&ok);
        else {
            // recab's own words for a choice are local, release and edited; a person is told what each does.
            auto words=[](const QString &choice){
                return choice=="local"?tr("Keep the next system's version (local)"):choice=="release"?tr("Take the release's version (release)")
                       :choice=="edited"?tr("Use a file I give (edited)"):choice;};
            QStringList shown;for(const QString &c:choices)shown<<words(c);
            value=QInputDialog::getItem(this,op["label"].toString(),field["label"].toString(),shown,0,false,&ok);
            if(ok)value=choices.value(shown.indexOf(value));
        }
        if(!ok)return;
        value=value.trimmed();
        if((field["required"].toBool() || (field["name"]=="file" && choice=="edited")) && value.isEmpty()){QMessageBox::warning(this,tr("System"),tr("A value is required."));return;}
        if(field["name"]=="choice")choice=value;
        if(!value.isEmpty()){if(!field["flag"].isNull())args<<field["flag"].toString();args<<value;}
    }
    work(client_.change(args),workTitle(op),tr("Working on the inactive system. The running system is preserved."),
         after ? after : [this](bool ok) { sectionUnlessStopped(ok); });
}

// The release page's own steps: when one ends well the page is shown again
// with what recab says now; when it fails the work page keeps recab's reason.
void Window::runReleaseOperation(const QString &id, bool askFirst)
{
    runOperation(status_.operationObject(id), QStringList(), [this](bool ok) {
        if (ok)
            showRelease();
    }, askFirst);
}

void Window::addFromFile(const QString &named)
{
    const QJsonObject op = status_.operationObject(QStringLiteral("update_media"));
    if (busy() || op["verdict"] != "ready")
        return;
    const QString path = !named.isEmpty() ? named
                                          : QFileDialog::getOpenFileName(this, tr("Choose a release manifest"), QString(),
                                                                         tr("Release manifest (release.json);;All files (*)"));
    if (path.isEmpty())
        return;
    const QStringList extra{QStringLiteral("--manifest"), path};
    const auto after = [this](bool ok) {
        if (ok)
            showRelease();
    };
    // A file the person chose is asked about too; the picture mode names one
    // and goes on at once.
    if (named.isEmpty()) {
        askBefore([this] { return beforewords::addRelease(client_, status_, true); },
                  [this, op, extra, after] { runOperation(op, extra, after, false); });
        return;
    }
    runOperation(op, extra, after, false);
}

// One of recab's operations run with the values it asks for already in hand,
// so no question comes before the Before page.
void Window::adoptBaseline(const QString &named, bool askFirst)
{
    const QJsonObject op = status_.operationObject(QStringLiteral("adopt_baseline"));
    if (busy() || op["verdict"] != "ready")
        return;
    const QString path = !named.isEmpty() ? named
                                          : QFileDialog::getOpenFileName(this, tr("Choose the original release's release.json"),
                                                                         QString(), tr("Release manifest (release.json);;All files (*)"));
    if (path.isEmpty())
        return;
    if (askFirst) {
        askBefore([this, path] { return beforewords::adoptBaseline(client_, status_, path); },
                  [this, path] { adoptBaseline(path, false); });
        return;
    }
    QJsonObject bare = op;
    bare.remove(QStringLiteral("inputs"));
    runOperation(bare, QStringList{QStringLiteral("--manifest"), path}, [this](bool ok) {
        if (ok)
            showRelease();
    }, false);
}

void Window::reproduce(const QString &package, bool askFirst)
{
    const QJsonObject op = status_.operationObject(QStringLiteral("update_reproduce"));
    if (busy() || op["verdict"] != "ready" || package.isEmpty())
        return;
    if (askFirst) {
        askBefore([this, package] { return beforewords::reproduce(client_, status_, package); },
                  [this, package] { reproduce(package, false); });
        return;
    }
    QJsonObject bare = op;
    bare.remove(QStringLiteral("inputs"));
    runOperation(bare, QStringList{package}, [this](bool ok) {
        if (ok)
            showSection();
    }, false);
}

void Window::replaceFailed(bool askFirst)
{
    const QJsonObject op = status_.operationObject(QStringLiteral("replace_failed"));
    if (busy() || op["verdict"] != "ready")
        return;
    runOperation(op, QStringList(), nullptr, askFirst);
}

void Window::addUpdate(bool askFirst)
{
    if (!status_.operations.isEmpty()) {
        // recab's own merge of the release into the next system; the release
        // page is where it came from, and where it returns.
        const QJsonObject op = status_.operationObject(QStringLiteral("update_apply"));
        if (op["verdict"] != "ready")
            return;
        if (askFirst) {
            askBefore([this] { return beforewords::addRelease(client_, status_, false); },
                      [this] { addUpdate(false); });
            return;
        }
        runOperation(op, QStringList(), [this](bool ok) {
            if (ok)
                showRelease();
        }, false);
        return;
    }
    if (askFirst) {
        askBefore([this] { return beforewords::addRelease(client_, status_, false); }, [this] { addUpdate(false); });
        return;
    }
    work(client_.change({"update", "apply"}), tr("Adding the update to the next system"),
         tr("The running system is not touched. You can keep working."), [this](bool ok) {
             if (!status_.clashes.isEmpty())
                 showClash();
             else
                 sectionUnlessStopped(ok);
         });
}

void Window::finish(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::finish(client_, status_); }, [this] { finish(false); });
        return;
    }
    work(client_.change({"finalize"}), tr("Finishing the next system"),
         tr("It is checked and closed, so it can be started."), [this](bool ok) { sectionUnlessStopped(ok); });
}

void Window::recover(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::recover(client_, status_); }, [this] { recover(false); });
        return;
    }
    work(client_.change({"recover"}), tr("Continuing where it stopped"),
         tr("recab looks at what is really there and carries on."), [this](bool ok) { sectionUnlessStopped(ok); });
}

void Window::restart(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::restartNow(client_, status_); }, [this] { restart(false); });
        return;
    }
    // When the restart is set already, only the restart is left.
    if (status_.state == QLatin1String("trial_pending"))
        chain({client_.reboot()}, tr("A restart into the next system was already set; nothing was changed."), true);
    else
        chain({client_.change({"set-next"}), client_.reboot()},
              tr("The step before it was made: recab set the next system to start at the next restart."), true);
}

void Window::restartLater(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::restartLater(client_, status_); }, [this] { restartLater(false); });
        return;
    }
    chain({client_.change({"set-next"})});
}

// recab's commit of the running system takes a set restart back.
void Window::cancelRestart(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::cancelRestart(client_, status_); }, [this] { cancelRestart(false); });
        return;
    }
    chain({client_.change({"commit"})});
}

void Window::keep(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::keep(client_, status_); }, [this] { keep(false); });
        return;
    }
    work(client_.change({"commit"}), tr("Keeping this system"),
         tr("It becomes the system that every restart starts."), [this](bool ok) { sectionUnlessStopped(ok); });
}

void Window::goBack(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::goBack(client_, status_); }, [this] { goBack(false); });
        return;
    }
    chain({client_.change({"rollback"}), client_.reboot()}, tr("The step before it was made: recab ran `recab rollback`."),
          true);
}

void Window::discard(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::discard(client_, status_); }, [this] { discard(false); });
        return;
    }
    chain({client_.change({"discard"})});
}

// recab has no single command for this: discarding forgets the next system
// but leaves its files as they are; preparing again copies the running system
// over them. Both together are the reset.
void Window::reset(bool askFirst)
{
    if (askFirst) {
        askBefore([this] { return beforewords::reset(client_, status_); }, [this] { reset(false); });
        return;
    }
    Run *drop = client_.change({"discard"});
    setEnabled(false);
    ++quiet_;
    connect(drop, &Run::finished, this, [this, drop](bool ok) {
        setEnabled(true);
        --quiet_;
        const QString reason = drop->problem();
        drop->deleteLater();
        if (!ok) {
            QMessageBox::warning(this, tr("System"), reason.isEmpty() ? tr("The next system was not reset.") : reason);
            reload([this] { showSection(); });
            return;
        }
        // Read first, so the copy is asked for as a new preparation. The
        // copy is the long part and gets the work page.
        reload([this] {
            work(client_.change({"prepare"}), tr("Making the next system a copy of the running one"),
                 tr("Only what differs is copied. The running system is not touched."), [this](bool ok) { sectionUnlessStopped(ok); });
        });
    });
    drop->start();
}

void Window::acceptProposal()
{
    chain({client_.change({"resolve", status_.proposalPath, "--proposal"})});
}

// One question for each file that clashes, then all the answers at once.
void Window::decideByHand()
{
    if(!status_.operations.isEmpty()){for(auto v:status_.operations){auto op=v.toObject();if(op["id"].toString().startsWith("resolve:") && op["verdict"]=="ready"){
        runOperation(op, QStringList(), [this](bool ok){
            if(!ok)return;
            if(!status_.clashes.isEmpty())showClash();else showRelease();
        });return;}}return;}
    QList<Run *> runs;
    int number = 0;
    for (const Clash &clash : status_.clashes) {
        QMessageBox question(this);
        question.setWindowTitle(status_.clashes.size() == 1
                                    ? tr("Decide by hand")
                                    : tr("Decide by hand (%1 of %2)").arg(++number).arg(status_.clashes.size()));
        question.setText(tr("Which %1 goes into the next system?").arg(clash.path));
        question.setInformativeText(tr("Mine: %1.\nThe update's: %2.").arg(clash.local, clash.release));
        QPushButton *mine = question.addButton(tr("Keep mine"), QMessageBox::AcceptRole);
        QPushButton *theirs = question.addButton(tr("Take the update's"), QMessageBox::AcceptRole);
        question.addButton(QMessageBox::Cancel);
        question.exec();
        if (question.clickedButton() == mine)
            runs << client_.change({"resolve", clash.path, "--local"});
        else if (question.clickedButton() == theirs)
            runs << client_.change({"resolve", clash.path, "--release"});
        else
            break; // what was decided so far is still carried out
    }
    if (!runs.isEmpty())
        chain(runs);
}

void Window::stopRun()
{
    if (run_)
        run_->stop();
}
