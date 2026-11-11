#include "window.h"

#include "pages.h"
#include "sources.h"

#include "before.h"
#include "frame.h"
#include "nextsystem.h"
#include "parts.h"
#include "refresh.h"
#include "work.h"

#include <QComboBox>
#include <QDesktopServices>
#include <QLineEdit>
#include <QMessageBox>
#include <QProcess>
#include <QStackedWidget>
#include <QTime>
#include <QToolButton>
#include <QUrl>

namespace {

// What Activity calls a run: the first line under "The request:", else the
// first line of the prompt.
QString requestTitle(const QString &prompt)
{
    const QString heading = QStringLiteral("The request:");
    const qsizetype at = prompt.indexOf(heading);
    const QString line = (at < 0 ? prompt : prompt.mid(at + heading.size())).trimmed().section(QLatin1Char('\n'), 0, 0).trimmed();
    return line.size() > 90 ? line.left(89).trimmed() + QStringLiteral("…") : line;
}

} // namespace

Window::Window(QWidget *parent)
    : QMainWindow(parent)
{
    setWindowTitle(tr("Recipe"));

    stack_ = new QStackedWidget;
    browse_ = new BrowsePage;
    recipe_ = new RecipePage;
    before_ = new BeforePage;
    runPage_ = new RunPage;
    installed_ = new InstalledPage;
    updates_ = new UpdatesPage;
    sources_ = new SourcesPage;
    wish_ = new WishPage;
    work_ = new WorkPage;
    for (QWidget *page : {static_cast<QWidget *>(browse_), static_cast<QWidget *>(recipe_), static_cast<QWidget *>(before_),
                          static_cast<QWidget *>(runPage_), static_cast<QWidget *>(installed_),
                          static_cast<QWidget *>(updates_), static_cast<QWidget *>(sources_),
                          static_cast<QWidget *>(wish_), static_cast<QWidget *>(work_)})
        stack_->addWidget(page);

    // The frame is the window: the rail, the pages in it, the strip about the
    // next system, and the status line.
    frame_ = new Frame(Frame::Recipe);
    frame_->setContent(stack_);
    frame_->addEntry(QStringLiteral("browse"), tr("Browse"));
    frame_->addEntry(QStringLiteral("installed"), tr("Installed"));
    frame_->addEntry(QStringLiteral("updates"), tr("Updates"));
    frame_->addEntry(QStringLiteral("sources"), tr("Sources"));
    frame_->select(QStringLiteral("browse"));
    frame_->setAgent(client_.agentProgram());
    updateStatus();
    setCentralWidget(frame_);

    // Only recab knows whether a next system holds changes; Recipe asks it.
    next_ = new NextSystem(qEnvironmentVariable("RECIPE_GUI_RECAB", "recab"), this);
    connect(next_, &NextSystem::known, this, [this] { frame_->setNextSystem(next_->titles()); });

    connect(frame_, &Frame::chosen, this, [this](const QString &key) {
        if (key == QLatin1String("browse"))
            showBrowse(category_);
        else if (key == QLatin1String("installed"))
            showInstalled(installedChoice_, installedPlace_);
        else if (key == QLatin1String("updates"))
            showUpdates(updateChoice_, updatePlace_);
        else if (key == QLatin1String("sources"))
            showSources();
    });
    // The next system is finished in System, the way its button in the rail starts it.
    connect(frame_, &Frame::finishRequested, this, [this] { frame_->startApp(Frame::System); });
    connect(frame_, &Frame::goToRun, this, [this] {
        frame_->select(QStringLiteral("browse"));
        stack_->setCurrentWidget(work_);
    });

    connect(browse_->searchBox(), &QLineEdit::textChanged, this, [this] { showBrowse(category_); });
    connect(browse_->categoryBox(), &QComboBox::activated, this,
            [this](int index) { showBrowse(browse_->categoryBox()->itemData(index).toString()); });
    connect(browse_->fromBox(), &QComboBox::activated, this,
            [this](int index) { chooseSource(browse_->fromBox()->itemData(index).toString()); });

    connect(browse_, &BrowsePage::openRecipe, this, &Window::showRecipe);
    connect(browse_, &BrowsePage::install, this, [this](const QString &name) { askInstall(name); });
    connect(browse_, &BrowsePage::update, this, [this](const QString &name, Place place) { askUpdate(name, place); });
    connect(browse_, &BrowsePage::retry, this, [this] { reload([this] { showBrowse(); }); });
    connect(browse_, &BrowsePage::askFor, this, &Window::showWish);
    connect(updates_, &UpdatesPage::check, this, [this] { fetchSources(); });
    connect(sources_, &SourcesPage::refresh, this, &Window::fetchSources);
    connect(sources_, &SourcesPage::add, this, &Window::addSource);
    connect(sources_, &SourcesPage::remove, this, [this](const QString &name) { removeSource(name); });
    connect(sources_, &SourcesPage::browse, this, [this](const QString &name) {
        chooseSource(name);
        showBrowse();
    });
    connect(wish_, &WishPage::back, this, [this] { showBrowse(category_); });
    connect(wish_, &WishPage::handOver, this, &Window::askAgent);
    connect(work_, &WorkPage::back, this, [this] { reload([this] { showBrowse(category_); }); });
    connect(work_, &WorkPage::activity, this, [this](const QString &words) { frame_->setAgentNote(words); });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
        frame_->setOwnRun(0); // Activity reads the run's record again
        if (!ok && !work_->stopped())
            return; // the work page says why, and offers the way back
        // The recipe it wrote is there to be read; nothing was installed.
        const QString written = written_;
        reload([this, written] {
            if (!written.isEmpty() && catalog_.recipe(written))
                showRecipe(written);
            else
                showBrowse(category_);
        });
    });

    // F5, and coming back to the window after a while: an agent or a terminal
    // may have installed or written something in the meantime.
    refresher_ = new Refresher(this, 20);
    connect(refresher_, &Refresher::asked, this, [this](bool) { refresh(); });
    connect(refresher_, &Refresher::asked, frame_, &Frame::readAgain);
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("Read the recipes and what is installed again (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    frame_->addStatusWidget(again);
    connect(recipe_, &RecipePage::back, this, [this] { showBrowse(); });
    connect(recipe_, &RecipePage::install, this, [this](const QString &name) { askInstall(name); });
    connect(recipe_, &RecipePage::update, this, [this](const QString &name, Place place) { askUpdate(name, place); });
    connect(recipe_, &RecipePage::remove, this, &Window::askRemove);
    connect(recipe_, &RecipePage::open, this, &Window::open);

    // The Before step: what will happen, then Cancel or go on.
    connect(before_, &BeforePage::cancelled, this, [this] {
        queue_.clear();
        if (const auto draw = again_)
            draw();
        else
            showBrowse(category_);
    });
    connect(before_, &BeforePage::choiceChanged, this, [this](const QString &key) {
        // The sentences depend on the place: say them again for the one picked.
        before_->show(beforeContent(catalog_, client_.places(), ask_, placeFromKey(key)));
    });
    connect(before_, &BeforePage::confirmed, this, [this](const QString &key) {
        startInstall(ask_.name, ask_.update ? ask_.place : placeFromKey(key));
    });
    connect(before_, &BeforePage::linkClicked, this, [this] {
        const Recipe *recipe = catalog_.recipe(ask_.name);
        if (recipe && !recipe->path.isEmpty())
            QDesktopServices::openUrl(QUrl::fromLocalFile(recipe->path));
    });
    connect(runPage_, &RunPage::back, this, [this] { showBrowse(); });
    connect(runPage_, &RunPage::retry, this, [this](const QString &name) { startInstall(name, runPlace_); });
    connect(runPage_, &RunPage::fix, this,
            [this](const QString &name, const QString &prompt) { startInstall(name, runPlace_, prompt); });
    connect(runPage_, &RunPage::note, this, [this](const QString &words) { frame_->flash(words); });
    connect(runPage_, &RunPage::succeeded, this, [this](const QString &name) { runEnded(name, true); });
    connect(runPage_, &RunPage::failed, this, [this](const QString &name) { runEnded(name, false); });
    connect(runPage_, &RunPage::status, this, [this](const QString &words) {
        buildingWords_ = words;
        updateStatus();
    });
    connect(installed_, &InstalledPage::open, this, &Window::open);
    connect(installed_, &InstalledPage::goBack, this, &Window::goBack);
    connect(installed_, &InstalledPage::clearEarlier, this, [this](Place place) { clearEarlier(place); });
    connect(installed_, &InstalledPage::remove, this, &Window::askRemove);
    connect(installed_, &InstalledPage::update, this, [this](const QString &name, Place place) { askUpdate(name, place); });
    connect(installed_, &InstalledPage::selected, this, [this](const QString &name, Place place) {
        installedChoice_ = name;
        installedPlace_ = int(place);
    });
    connect(updates_, &UpdatesPage::update, this, [this](const QString &name, Place place) { askUpdate(name, place); });
    connect(updates_, &UpdatesPage::selected, this, [this](const QString &name, Place place) {
        updateChoice_ = name;
        updatePlace_ = int(place);
    });
    // Each update is asked about in turn, with its own Before page.
    connect(updates_, &UpdatesPage::updateAll, this, [this] {
        queue_.clear();
        for (const UpdateItem &item : catalog_.updates)
            if (item.place != Place::Next)
                queue_ << qMakePair(item.name, item.place);
        startQueued();
    });

    reload([this] { showBrowse(); });
}

void Window::updateStatus()
{
    frame_->setStatus(buildingWords_.isEmpty() ? readWords_ : tr("%1 · %2").arg(buildingWords_, readWords_));
}

void Window::clearFlash()
{
    frame_->flash(QString());
    updateStatus();
}

void Window::reload(std::function<void()> then)
{
    if (client_.loading())
        return;
    if (!read_) {
        browse_->showLoading(tr("Reading the recipes"),
                             tr("Asking recipe what there is to install and what is installed already."));
        stack_->setCurrentWidget(browse_);
    }
    readWords_ = tr("Reading the recipes…");
    updateStatus();
    client_.load([this, then](bool ok, const Catalog &catalog, const QString &problem) {
        if (!ok) {
            readWords_ = tr("The recipes could not be read");
            updateStatus();
            browse_->showProblem(problem);
            stack_->setCurrentWidget(browse_);
            return;
        }
        catalog_ = catalog;
        read_ = true;
        refresher_->fresh();
        readWords_ = tr("%1, read at %2")
                         .arg(catalog_.originsWords(), QLocale().toString(QTime::currentTime(), QLocale::ShortFormat));
        if (catalog_.unreadable)
            readWords_ += tr(" · %1").arg(counted(catalog_.unreadable, tr("%1 recipe could not be read"),
                                                  tr("%1 recipes could not be read")));
        updateStatus();
        if (!origin_.isEmpty() && !catalog_.origin(origin_))
            origin_.clear(); // the chosen source is gone
        fillSources();
        fillCategories();
        fillRail();
        // The strip under every page: what the next system holds may have changed.
        next_->read();
        if (then)
            then();
    });
}

// Reading again leaves the person where they were: the page is drawn again
// from what was read, except a page that is being worked on.
void Window::refresh()
{
    if (busy())
        return;
    QWidget *shown = stack_->currentWidget();
    const bool drawn = shown != runPage_ && shown != work_ && shown != wish_ && shown != before_;
    reload([this, drawn] {
        // A copy: drawing the page sets `again_` anew, and the one running must outlive that.
        if (const auto draw = again_; drawn && draw)
            draw();
    });
}

void Window::fillSources()
{
    QComboBox *from = browse_->fromBox();
    const QSignalBlocker quiet(from);
    from->clear();
    from->addItem(tr("From all sources"), QString());
    for (const Origin &origin : catalog_.origins)
        if (catalog_.countFrom(origin.name) > 0)
            from->addItem(tr("From %1").arg(origin.shown()), origin.name);
    from->setCurrentIndex(qMax(0, from->findData(origin_)));
    // One source: nothing to choose between.
    from->setVisible(from->count() > 2);
}

void Window::chooseSource(const QString &name)
{
    origin_ = name;
    QComboBox *from = browse_->fromBox();
    const QSignalBlocker quiet(from);
    from->setCurrentIndex(qMax(0, from->findData(origin_)));
    if (stack_->currentWidget() == browse_)
        showBrowse(category_);
}

void Window::fillCategories()
{
    QComboBox *box = browse_->categoryBox();
    const QSignalBlocker quiet(box);
    box->clear();
    box->addItem(tr("All kinds"), QString());
    for (const QString &category : catalog_.categories())
        box->addItem(categoryTitle(category), category);
    box->setCurrentIndex(qMax(0, box->findData(category_)));
}

void Window::fillRail()
{
    frame_->setBadge(QStringLiteral("updates"), int(catalog_.updates.size()));
}

void Window::showBrowse(const QString &category)
{
    category_ = category;
    again_ = [this, category] { showBrowse(category); };
    {
        QComboBox *box = browse_->categoryBox();
        const QSignalBlocker quiet(box);
        box->setCurrentIndex(qMax(0, box->findData(category)));
    }
    frame_->follow(QStringLiteral("browse"));
    browse_->show(catalog_, category, browse_->searchBox()->text().trimmed(), origin_);
    stack_->setCurrentWidget(browse_);
}

void Window::showRecipe(const QString &name)
{
    again_ = [this, name] { showRecipe(name); };
    recipe_->show(catalog_, name);
    stack_->setCurrentWidget(recipe_);
}

void Window::showInstalled(const QString &selected, int place)
{
    again_ = [this] { showInstalled(installedChoice_, installedPlace_); };
    frame_->follow(QStringLiteral("installed"));
    installed_->show(catalog_, selected, place);
    stack_->setCurrentWidget(installed_);
}

void Window::showUpdates(const QString &selected, int place)
{
    again_ = [this] { showUpdates(updateChoice_, updatePlace_); };
    frame_->follow(QStringLiteral("updates"));
    updates_->show(catalog_, selected, place);
    stack_->setCurrentWidget(updates_);
}

// What will happen, before it does. An install is asked where; the places
// offered are the ones that do not hold the software yet.
void Window::askInstall(const QString &name)
{
    QList<Place> choices = catalog_.installPlaces(name);
    if (choices.isEmpty())
        return;
    ask_ = Ask{name, false, choices.first(), choices};
    showBefore();
}

void Window::askUpdate(const QString &name, Place place)
{
    ask_ = Ask{name, true, place, {place}};
    showBefore();
}

void Window::showBefore()
{
    if (run_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        stack_->setCurrentWidget(runPage_);
        return;
    }
    // The Cancel button comes back to the page the person was on: `again_` is still its.
    before_->show(beforeContent(catalog_, client_.places(), ask_, ask_.update ? ask_.place : ask_.choices.first()));
    stack_->setCurrentWidget(before_);
}

void Window::startQueued()
{
    if (!queue_.isEmpty()) {
        const auto next = queue_.takeFirst();
        askUpdate(next.first, next.second);
    }
}

void Window::startInstall(const QString &name, Place place, const QString &agentPrompt)
{
    if (run_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        stack_->setCurrentWidget(runPage_);
        return;
    }
    const bool installed = catalog_.installedItem(name, place) != nullptr;
    // `recipe` is given SOURCE:NAME, so two sources with one name cannot be confused.
    const QString named = catalog_.qualified(name);
    runPlace_ = place;
    run_ = !agentPrompt.isEmpty() ? client_.fixWithAgent(named, agentPrompt, installed, place)
           : installed ? client_.update(named, place) : client_.install(named, place);
    runPage_->begin(catalog_, name, run_, place);
    stack_->setCurrentWidget(runPage_);
    run_->start();
}

void Window::stopRun()
{
    if (run_)
        run_->stop();
}

void Window::runEnded(const QString &name, bool ok)
{
    run_ = nullptr;
    reload([this, name, ok] {
        if (!ok) {
            queue_.clear(); // the run page stays, with what went wrong
            return;
        }
        if (!queue_.isEmpty())
            startQueued();
        else
            showRecipe(name);
    });
}

// ---------------------------------------------------------------- Sources

void Window::showSources()
{
    again_ = [this] { showSources(); };
    frame_->follow(QStringLiteral("sources"));
    sources_->show(catalog_);
    stack_->setCurrentWidget(sources_);
}

void Window::fetchSources(const QString &name)
{
    frame_->flash(name.isEmpty() ? tr("Fetching every source's list…") : tr("Fetching the list of %1…").arg(name));
    quietChange(client_.refresh(name));
}

void Window::addSource(const QString &addressOrFolder)
{
    frame_->flash(tr("Adding the source…"));
    quietChange(client_.addSource(addressOrFolder));
}

void Window::removeSource(const QString &name, bool askFirst)
{
    const Origin *origin = catalog_.origin(name);
    if (!origin)
        return;
    if (askFirst
        && QMessageBox::question(this, tr("Remove a source"),
                                 tr("Remove the source %1?\n\nIts recipes are no longer offered. What was installed "
                                    "from it stays installed.")
                                     .arg(origin->shown()),
                                 QMessageBox::Yes | QMessageBox::Cancel)
               != QMessageBox::Yes)
        return;
    quietChange(client_.removeSource(name));
}

// ---------------------------------------------------------------- Asking for a recipe

void Window::showWish(const QString &wish)
{
    again_ = nullptr;
    wish_->show(catalog_, wish);
    stack_->setCurrentWidget(wish_);
}

void Window::askAgent(const QString &prompt)
{
    if (run_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        return;
    }
    written_.clear();
    run_ = client_.agent(prompt, requestTitle(prompt));
    // The agent names the recipe it wrote, so the window can open it.
    connect(run_, &Run::event, this, [this](const QJsonObject &event) {
        if (event["event"] == QLatin1String("agent") && !event["recipe"].toString().isEmpty())
            written_ = event["recipe"].toString();
    });
    work_->begin(tr("The agent is writing a recipe"), tr("It only writes the recipe. Nothing is installed."), run_,
                 tr("A half-written recipe is left in the folder"));
    stack_->setCurrentWidget(work_);
    run_->start();
    frame_->setOwnRun(run_->pid());
}

bool Window::agentAsks() const
{
    return run_ && stack_->currentWidget() == work_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

void Window::askRemove(const QString &name, Place place)
{
    const Recipe *recipe = catalog_.recipe(name);
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (!recipe || !item)
        return;
    RemoveDialog dialog(*recipe, *item, this);
    if (dialog.exec() == QDialog::Accepted)
        quietChange(client_.remove(name, place));
}

void Window::removeNow(const QString &name, Place place)
{
    quietChange(client_.remove(name, place));
}

void Window::goBack(const QString &name, Place place)
{
    quietChange(client_.goBack(name, place));
}

void Window::clearEarlier(Place place, bool askFirst)
{
    if (askFirst
        && QMessageBox::question(this, tr("Clear earlier versions"),
                                 tr("Delete every earlier version Recipe keeps%1 (%2)?\n\nNothing that is installed "
                                    "changes. Afterwards no program there can be taken back to an earlier version "
                                    "until it is updated again.")
                                     .arg(catalog_.twoPlaces ? (place == Place::Me ? tr(" for you") : tr(" for this computer")) : QString(),
                                          bytesText(catalog_.stores[int(place)].earlierBytes)),
                                 QMessageBox::Yes | QMessageBox::Cancel)
               != QMessageBox::Yes)
        return;
    quietChange(client_.clearEarlier(place));
}

// A short change with no page of its own: run it, then show the page again.
void Window::quietChange(Run *run)
{
    setEnabled(false);
    ++quiet_;
    connect(run, &Run::finished, this, [this, run](bool ok) {
        setEnabled(true);
        --quiet_;
        clearFlash();
        if (!ok) {
            const QString reason = run->problem();
            QMessageBox::warning(this, tr("Recipe"),
                                 reason.isEmpty() ? tr("The change was not made.") : tr("The change was not made: %1.").arg(reason));
        }
        run->deleteLater();
        reload([this] {
            if (const auto draw = again_)
                draw();
        });
    });
    run->start();
}

void Window::open(const QString &name, Place place)
{
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (!item || item->programs.isEmpty())
        return;
    if (!QProcess::startDetached(item->programs.first(), {}))
        frame_->flash(tr("%1 could not be started").arg(item->programs.first()));
}
