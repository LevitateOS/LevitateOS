#include "window.h"

#include "pages.h"
#include "sources.h"
#include "storage.h"

#include "before.h"
#include "frame.h"
#include "nextsystem.h"
#include "parts.h"
#include "refresh.h"
#include "work.h"

#include <QCloseEvent>
#include <QComboBox>
#include <QDir>
#include <QDesktopServices>
#include <QFile>
#include <QFileInfo>
#include <QLineEdit>
#include <QMessageBox>
#include <QProcess>
#include <QProcessEnvironment>
#include <QRegularExpression>
#include <QPushButton>
#include <QStackedWidget>
#include <QStandardPaths>
#include <QTime>
#include <QTimer>
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
    storage_ = new StoragePage;
    wish_ = new WishPage;
    work_ = new WorkPage;
    change_ = new ChangePage;
    for (QWidget *page : {static_cast<QWidget *>(browse_), static_cast<QWidget *>(recipe_), static_cast<QWidget *>(before_),
                          static_cast<QWidget *>(runPage_), static_cast<QWidget *>(installed_),
                          static_cast<QWidget *>(updates_), static_cast<QWidget *>(sources_), static_cast<QWidget *>(storage_),
                          static_cast<QWidget *>(wish_), static_cast<QWidget *>(work_), static_cast<QWidget *>(change_)})
        stack_->addWidget(page);

    // The frame is the window: the rail, the pages in it, the strip about the
    // next system, and the status line.
    frame_ = new Frame(Frame::Recipe);
    frame_->setContent(stack_);
    frame_->addEntry(QStringLiteral("browse"), tr("Browse"));
    frame_->addEntry(QStringLiteral("installed"), tr("Installed"));
    frame_->addEntry(QStringLiteral("updates"), tr("Updates"));
    frame_->addEntry(QStringLiteral("sources"), tr("Sources"));
    frame_->addEntry(QStringLiteral("storage"), tr("Storage"));
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
        else if (key == QLatin1String("storage"))
            showStorage();
    });
    // The next system is finished in Recab, the way its button starts it.
    connect(frame_, &Frame::finishRequested, this, [this] { frame_->startApp(Frame::Recab); });
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
    // Help leads to "Ask for software" too; it is not a rail entry, so the frame is told of it.
    frame_->addHelpTarget(QStringLiteral("ask"));
    connect(frame_, &Frame::helpLink, this, [this](const QString &key) {
        if (key == QLatin1String("ask"))
            showWish();
    });
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
    // The work page is the agent's; only an agent's run speaks for the agent in the status line.
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        if (run_ && run_->isAgent())
            frame_->setAgentNote(words);
    });
    connect(change_, &ChangePage::back, this, [this] { leaveChange(); });
    connect(change_, &ChangePage::retry, this, [this] {
        if (remake_)
            change(changeWords_, remake_);
    });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
        frame_->setOwnRun(QString()); // Activity reads the run's record again
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
    connect(recipe_, &RecipePage::goBack, this, &Window::goBack);
    connect(recipe_, &RecipePage::hold, this, &Window::hold);
    connect(recipe_, &RecipePage::readRecipe, this, [this](const QString &name) {
        const Recipe *recipe = catalog_.recipe(name);
        if (recipe && !recipe->path.isEmpty())
            QDesktopServices::openUrl(QUrl::fromLocalFile(recipe->path));
    });

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
    connect(installed_, &InstalledPage::openPage, this, &Window::showRecipe);
    connect(recipe_, &RecipePage::backToInstalled, this, [this] { showInstalled(installedChoice_, installedPlace_); });
    connect(installed_, &InstalledPage::goBack, this, &Window::goBack);
    connect(installed_, &InstalledPage::hold, this, &Window::hold);
    connect(installed_, &InstalledPage::openStorage, this, [this] { showStorage(); });
    connect(storage_, &StoragePage::clearEarlier, this, [this](Place place) { clearEarlier(place); });
    connect(storage_, &StoragePage::openInstalled, this, [this] { showInstalled(installedChoice_, installedPlace_); });
    connect(updates_, &UpdatesPage::hold, this, &Window::hold);
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
            // A program kept at its version is left alone.
            if (item.place != Place::Next && !item.held)
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
    if (client_.loading()) {
        // What is being read began before whatever asked for this: read once more when it ends,
        // and then do what was asked, so a page that waits for it is not left waiting.
        reloadQueued_ = true;
        if (then)
            queuedThen_ = then;
        return;
    }
    if (!read_) {
        browse_->showLoading(tr("Reading the recipes"),
                             tr("Asking recipe what there is to install and what is installed already."));
        stack_->setCurrentWidget(browse_);
    }
    readWords_ = tr("Reading the recipes…");
    updateStatus();
    client_.load([this, then](bool ok, const Catalog &catalog, const QString &problem) {
        if (reloadQueued_) {
            reloadQueued_ = false;
            const auto queued = queuedThen_;
            queuedThen_ = nullptr;
            reload([then, queued] {
                if (then)
                    then();
                if (queued)
                    queued();
            });
            return;
        }
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
    const bool drawn = shown != runPage_ && shown != work_ && shown != wish_ && shown != before_ && shown != change_;
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
    // A held program's update is listed, not counted.
    frame_->setBadge(QStringLiteral("updates"), catalog_.updatesToCount());
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
    // A held program's Before step is not offered until it is released.
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (item && item->held) {
        queue_.clear();
        frame_->flash(tr("%1 is kept at version %2. Let it update again first.").arg(catalog_.recipe(name) ? catalog_.recipe(name)->shown() : name, item->version));
        return;
    }
    ask_ = Ask{name, true, place, {place}};
    showBefore();
}

void Window::showBefore()
{
    if (run_ || changeRun_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        stack_->setCurrentWidget(run_ ? static_cast<QWidget *>(runPage_) : change_);
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
    if (run_ || changeRun_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        stack_->setCurrentWidget(run_ ? static_cast<QWidget *>(runPage_) : change_);
        return;
    }
    const bool installed = catalog_.installedItem(name, place) != nullptr;
    if (installed && !agentPrompt.isEmpty()) {
        // `recipe update` takes no `--autofix`; the stopped page offers no repair of an update.
        frame_->flash(tr("Recipe's agent repairs a recipe only while installing it, not for an update."));
        return;
    }
    // `recipe` is given SOURCE:NAME, so two sources with one name cannot be confused.
    const QString named = catalog_.qualified(name);
    runPlace_ = place;
    runName_ = name;
    runUpdate_ = installed;
    run_ = !agentPrompt.isEmpty() ? client_.fixWithAgent(named, agentPrompt, place)
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
    if (closeAfterStop_) {
        // The window was closed while this ran, and the person said to stop it: it has stopped.
        closing_ = true;
        close();
        return;
    }
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
    ChangePage::Words words;
    words.title = name.isEmpty() ? tr("Fetching every source's list") : tr("Fetching the list of %1").arg(name);
    words.subtitle = tr("Recipe asks %1 for its list of recipes again, over the network. Nothing is installed.")
                         .arg(name.isEmpty() ? tr("each source") : tr("the source"));
    words.doing = tr("Asking Recipe to fetch again…");
    words.stopNote = tr("Stopping it leaves the lists Recipe already has. Fetch again whenever you like.");
    words.failed = name.isEmpty() ? tr("Fetching the lists did not finish") : tr("Fetching the list of %1 did not finish").arg(name);
    words.left = tr("The recipes are as they were when they were last read. Sources shows when each list was fetched.");
    words.done = name.isEmpty() ? tr("Fetched every source's list") : tr("Fetched the list of %1").arg(name);
    change(words, [this, name] { return client_.refresh(name); });
}

void Window::addSource(const QString &addressOrFolder)
{
    ChangePage::Words words;
    words.title = tr("Adding the source");
    words.subtitle = tr("%1 is added to the list of sources, and its list of recipes is read. Nothing is installed from it.")
                         .arg(addressOrFolder.trimmed());
    words.doing = tr("Asking Recipe to add the source…");
    words.stopNote = tr("Stopping it before it ends: Sources shows whether the source is listed. Add it again if it is not.");
    words.failed = tr("Adding the source did not finish");
    words.left = tr("Sources shows which sources are listed now.");
    words.done = tr("Added the source");
    change(words, [this, addressOrFolder] { return client_.addSource(addressOrFolder); });
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
    ChangePage::Words words;
    words.title = tr("Removing the source %1").arg(origin->shown());
    words.subtitle = tr("Its recipes are no longer offered. What was installed from it stays installed.");
    words.doing = tr("Asking Recipe to remove the source…");
    words.stopNote = tr("Stopping it before it ends: Sources shows whether the source is still listed. Remove it again if it is.");
    words.failed = tr("Removing the source %1 did not finish").arg(origin->shown());
    words.left = tr("Sources shows which sources are listed now.");
    words.done = tr("Removed the source %1").arg(origin->shown());
    change(words, [this, name] { return client_.removeSource(name); });
}

void Window::showStorage()
{
    again_ = [this] { showStorage(); };
    frame_->follow(QStringLiteral("storage"));
    storage_->show(catalog_, client_.places());
    stack_->setCurrentWidget(storage_);
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
    if (run_ || changeRun_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        return;
    }
    written_.clear();
    // The agent writes the recipe into the person's own folder, and may write
    // nowhere else on its own: the folder is made first, so that it can.
    const QString folder = ownRecipesFolder(catalog_);
    if (!QDir().mkpath(folder)) {
        frame_->flash(tr("The folder for your own recipes could not be made (%1), so the agent was not started.").arg(folder));
        return;
    }
    run_ = client_.agent(prompt, requestTitle(prompt), {folder});
    // The agent names the recipe it wrote, so the window can open it.
    connect(run_, &Run::event, this, [this](const QJsonObject &event) {
        if (event["event"] == QLatin1String("agent") && !event["recipe"].toString().isEmpty())
            written_ = event["recipe"].toString();
    });
    work_->begin(tr("The agent is writing a recipe"), tr("It only writes the recipe. Nothing is installed."), run_,
                 tr("A half-written recipe is left in the folder"));
    stack_->setCurrentWidget(work_);
    run_->start();
    frame_->setOwnRun(run_);
}

bool Window::agentAsks() const
{
    return run_ && stack_->currentWidget() == work_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

QString Window::shownName(const QString &name) const
{
    const Recipe *recipe = catalog_.recipe(name);
    return recipe ? recipe->shown() : name;
}

void Window::askRemove(const QString &name, Place place)
{
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (!item)
        return;
    if (run_ || changeRun_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        return;
    }
    // `recipe remove` works from the receipt alone, so a program whose recipe is
    // gone (its source removed, the recipe deleted) can still be removed.
    Recipe gone;
    gone.name = name;
    const Recipe *recipe = catalog_.recipe(name);
    // Asked without waiting: the window stays alive under it, and a check can answer it.
    auto *dialog = new RemoveDialog(recipe ? *recipe : gone, *item, this);
    dialog->setAttribute(Qt::WA_DeleteOnClose);
    removeBox_ = dialog;
    connect(dialog, &QDialog::accepted, this, [this, name, place] { removeNow(name, place); });
    dialog->open();
}

ChangePage::Words Window::removeWords(const QString &name, Place place) const
{
    const InstalledItem *item = catalog_.installedItem(name, place);
    const QString shown = shownName(name);
    const bool store = !item || item->inStore;
    ChangePage::Words words;
    words.title = tr("Removing %1").arg(shown);
    words.subtitle = !store ? (place == Place::Next ? tr("It is taken out of the next system, which is the only one that holds it.")
                                                    : tr("It is taken out of the next system. The running system keeps it until the restart."))
                            : tr("Recipe keeps its files among the earlier versions until those are cleared, in Storage.");
    words.doing = tr("Asking Recipe to remove %1…").arg(shown);
    words.stopNote = store ? tr("Stopping it before it ends leaves %1 installed. You can remove it again from Installed.").arg(shown)
                           : tr("Stopping it before it ends leaves the next system as it was. You can remove it again from Installed.");
    words.failed = tr("Removing %1 did not finish").arg(shown);
    words.left = store && item ? tr("%1 %2 is still installed.").arg(shown, item->version)
                               : tr("The next system was not changed. Installed shows what it holds.");
    words.done = tr("Removed %1").arg(shown);
    return words;
}

void Window::removeNow(const QString &name, Place place)
{
    change(removeWords(name, place), [this, name, place] { return client_.remove(name, place); });
}

void Window::goBack(const QString &name, Place place)
{
    const InstalledItem *item = catalog_.installedItem(name, place);
    const QString shown = shownName(name);
    ChangePage::Words words;
    words.title = item && !item->kept.isEmpty() ? tr("Going back to %1 %2").arg(shown, item->kept) : tr("Going back with %1").arg(shown);
    words.subtitle = tr("Recipe activates the earlier version again from the files it kept. The version left behind stays kept.");
    words.doing = tr("Asking Recipe to go back…");
    words.stopNote = item ? tr("Stopping it before it ends leaves %1 at version %2. You can go back again from Installed.").arg(shown, item->version)
                          : tr("Stopping it before it ends leaves %1 as it was. You can go back again from Installed.").arg(shown);
    words.failed = tr("Going back with %1 did not finish").arg(shown);
    words.left = item ? tr("%1 is still at version %2.").arg(shown, item->version) : tr("Installed shows which version it is at.");
    words.done = item && !item->kept.isEmpty() ? tr("%1 is back at version %2").arg(shown, item->kept) : tr("Went back with %1").arg(shown);
    change(words, [this, name, place] { return client_.goBack(name, place); });
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
    ChangePage::Words words;
    words.title = tr("Clearing earlier versions");
    words.subtitle = tr("Recipe deletes every earlier version it keeps%1. Nothing that is installed changes.")
                         .arg(catalog_.twoPlaces ? (place == Place::Me ? tr(" for you") : tr(" for this computer")) : QString());
    words.doing = tr("Asking Recipe to delete them…");
    words.stopNote = tr("If you stop it, some earlier versions may already be deleted and the rest stay. What is installed does not "
                        "change. Clear again from Storage.");
    words.failed = tr("Clearing the earlier versions did not finish");
    words.left = tr("What is installed did not change. Storage shows which earlier versions are left.");
    words.done = tr("Cleared the earlier versions");
    change(words, [this, place] { return client_.clearEarlier(place); });
}

void Window::hold(const QString &name, Place place, bool on)
{
    // `recipe` holds software only where a store stands behind it.
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (!item || !item->inStore || place == Place::Next)
        return;
    const QString shown = shownName(name);
    ChangePage::Words words;
    words.title = on ? tr("Keeping %1 at version %2").arg(shown, item->version) : tr("Letting %1 update again").arg(shown);
    words.subtitle = on ? tr("Update and Update all leave it alone until you let it update again.")
                        : tr("Its updates are offered again, and Update all includes it.");
    words.doing = on ? tr("Asking Recipe to keep it at its version…") : tr("Asking Recipe to let it update again…");
    words.stopNote = tr("Stopping it before it ends leaves %1 as it was. You can do it again from Installed.").arg(shown);
    words.failed = on ? tr("Keeping %1 at its version did not finish").arg(shown) : tr("Letting %1 update again did not finish").arg(shown);
    words.left = on ? tr("%1 is not kept at a version.").arg(shown) : tr("%1 is still kept at version %2.").arg(shown, item->version);
    words.done = on ? tr("%1 is kept at version %2").arg(shown, item->version) : tr("%1 updates again").arg(shown);
    change(words, [this, name, place, on] { return on ? client_.hold(name, place) : client_.unhold(name, place); });
}

// A short change has its own page: what it does, Stop, what the command printed, and when it did
// not finish, why. The person may go elsewhere meanwhile; the status line says what runs.
void Window::change(const ChangePage::Words &words, std::function<Run *()> make)
{
    if (run_ || changeRun_) {
        frame_->flash(tr("One thing at a time: wait for the running one, or stop it."));
        stack_->setCurrentWidget(run_ ? static_cast<QWidget *>(runPage_) : change_);
        return;
    }
    remake_ = make;
    changeWords_ = words;
    changeRun_ = make();
    Run *run = changeRun_;
    change_->begin(words, run);
    stack_->setCurrentWidget(change_);
    buildingWords_ = words.title;
    frame_->flash(QString()); // what ran before is said; the status line now says what runs
    updateStatus();
    // The page hears the end first (it shows why), then the window moves on.
    connect(run, &Run::finished, this, [this, run](bool ok) {
        if (run != changeRun_)
            return;
        changeEnded(ok);
        run->deleteLater();
    });
    run->start();
}

void Window::changeEnded(bool ok)
{
    changeRun_ = nullptr;
    buildingWords_.clear();
    updateStatus();
    if (closeAfterStop_) {
        closing_ = true;
        close();
        return;
    }
    const bool onThePage = stack_->currentWidget() == change_;
    if (ok) {
        frame_->flash(changeWords_.done);
        // Back where the person was, drawn from what was read after the change.
        reload([this] {
            if (stack_->currentWidget() == change_ || stack_->currentWidget() == nullptr) {
                if (const auto draw = again_)
                    draw();
                else
                    showBrowse(category_);
            } else if (const auto draw = again_; draw && stack_->currentWidget() != runPage_ && stack_->currentWidget() != work_
                                                  && stack_->currentWidget() != wish_ && stack_->currentWidget() != before_) {
                draw();
            }
        });
        return;
    }
    // It did not finish: the page stays with what went wrong, and the read still happens so that
    // the other pages are true. Someone elsewhere is told in the status line.
    if (!onThePage)
        frame_->flash(tr("%1. Its page is in the Recipe window.").arg(changeWords_.failed));
    reload(nullptr);
}

// "Back" on a change that did not finish.
void Window::leaveChange()
{
    reload([this] {
        if (const auto draw = again_)
            draw();
        else
            showBrowse(category_);
    });
}

// ---------------------------------------------------------------- Opening a program

namespace {

// A value of the "Desktop Entry" group of a menu entry's file.
QString desktopValue(const QString &path, const QString &key)
{
    QFile file(path);
    if (!file.open(QIODevice::ReadOnly))
        return {};
    bool inEntry = false;
    for (const QByteArray &raw : file.readAll().split('\n')) {
        const QString line = QString::fromUtf8(raw).trimmed();
        if (line.startsWith(QLatin1Char('['))) {
            inEntry = line == QLatin1String("[Desktop Entry]");
            continue;
        }
        if (inEntry && line.startsWith(key + QLatin1Char('=')))
            return line.mid(key.size() + 1).trimmed();
    }
    return {};
}

// The command a menu entry's Exec line says, without its field codes (%U, %f ...).
QStringList execWords(const QString &exec)
{
    QStringList words;
    static const QRegularExpression field(QStringLiteral("%[a-zA-Z]"));
    for (QString word : QProcess::splitCommand(exec)) {
        if (word.size() == 2 && field.match(word).hasMatch() && word.at(1) != QLatin1Char('%'))
            continue;
        word.replace(QStringLiteral("%%"), QStringLiteral("\x01"));
        word.remove(field);
        word.replace(QLatin1Char('\x01'), QLatin1Char('%'));
        words << word;
    }
    return words;
}

} // namespace

// Starts the program from the profile of the store it is installed in: that profile's `bin` may
// not be on PATH (the person's own store usually is not, and this computer's is another). The
// program a menu entry of the package names comes first, as the applications menu would start it;
// else the first program the package put in `bin`.
void Window::open(const QString &name, Place place)
{
    const InstalledItem *item = catalog_.installedItem(name, place);
    if (!item)
        return;
    const QString shown = shownName(name);
    if (place == Place::Next) {
        frame_->flash(tr("%1 is only in the next system, so it cannot be opened until after the restart.").arg(shown));
        return;
    }
    if (!item->inStore) {
        frame_->flash(tr("%1 cannot be opened from here: the running system's list names the software and its version, "
                         "not its programs.")
                          .arg(shown));
        return;
    }
    if (item->profile.isEmpty()) {
        frame_->flash(tr("%1 could not be started: Recipe did not say where it is installed.").arg(shown));
        return;
    }
    const QString bin = item->profile + QStringLiteral("/bin");
    QStringList command;
    QString terminalCommand, missing;
    for (const QString &entry : item->menu) {
        const QString file = item->profile + QStringLiteral("/share/applications/") + entry + QStringLiteral(".desktop");
        const QStringList words = execWords(desktopValue(file, QStringLiteral("Exec")));
        if (words.isEmpty()) {
            if (missing.isEmpty())
                missing = tr("the menu entry %1 in %2").arg(entry, item->profile + QStringLiteral("/share/applications"));
            continue;
        }
        QString program = words.first();
        if (!program.contains(QLatin1Char('/'))) {
            program = QStandardPaths::findExecutable(program, {bin});
            if (program.isEmpty())
                program = QStandardPaths::findExecutable(words.first());
        }
        if (program.isEmpty() || !QFileInfo(program).isExecutable()) {
            if (missing.isEmpty())
                missing = tr("%1, which the menu entry %2 starts, in %3").arg(words.first(), entry, bin);
            continue;
        }
        if (desktopValue(file, QStringLiteral("Terminal")).compare(QLatin1String("true"), Qt::CaseInsensitive) == 0) {
            terminalCommand = program;
            continue;
        }
        command = QStringList{program} + words.mid(1);
        break;
    }
    if (command.isEmpty()) {
        if (!terminalCommand.isEmpty()) {
            frame_->flash(tr("%1 runs in a terminal: start it there as %2.").arg(shown, terminalCommand));
            return;
        }
        for (const QString &program : item->programs) {
            const QString path = bin + QLatin1Char('/') + program;
            if (QFileInfo(path).isExecutable()) {
                command = {path};
                break;
            }
        }
        if (command.isEmpty()) {
            if (missing.isEmpty() && !item->programs.isEmpty())
                missing = tr("%1 in %2").arg(item->programs.first(), bin);
            frame_->flash(missing.isEmpty() ? tr("%1 could not be started: it put no program in %2.").arg(shown, bin)
                                            : tr("%1 could not be started: %2 is not there.").arg(shown, missing));
            return;
        }
    }
    QProcess process;
    process.setProgram(command.first());
    process.setArguments(command.mid(1));
    process.setWorkingDirectory(QDir::homePath());
    QProcessEnvironment environment = QProcessEnvironment::systemEnvironment();
    environment.insert(QStringLiteral("PATH"), bin + QLatin1Char(':') + environment.value(QStringLiteral("PATH")));
    process.setProcessEnvironment(environment);
    process.setStandardInputFile(QProcess::nullDevice());
    process.setStandardOutputFile(QProcess::nullDevice());
    process.setStandardErrorFile(QProcess::nullDevice());
    if (!process.startDetached())
        frame_->flash(tr("%1 could not be started: %2 would not run (%3).").arg(shown, command.first(), process.errorString()));
    else
        frame_->flash(tr("Started %1").arg(shown));
}

// ---------------------------------------------------------------- Closing the window

Run *Window::runningRun() const
{
    // An agent's run goes on without the window (the frame lets go of it); the others end with it.
    if (changeRun_ && changeRun_->running())
        return changeRun_;
    if (run_ && !run_->isAgent() && run_->running())
        return run_;
    return nullptr;
}

QString Window::closeWords() const
{
    if (Run *run = runningRun(); run && run == changeRun_)
        return tr("%1 is still running.\n\n%2").arg(changeWords_.title, changeWords_.stopNote);
    const QString shown = shownName(runName_);
    const InstalledItem *item = catalog_.installedItem(runName_, runPlace_);
    const QString unchanged = runUpdate_ && item ? tr("%1 %2 stays installed and works as before.").arg(shown, item->version)
                                                 : tr("Nothing on this computer is changed.");
    QString words = tr("%1 is still running.\n\nIf you close the window now it is stopped. What is already fetched and built is kept. "
                       "%2 You can start it again from %3's page whenever you like.")
                        .arg(runUpdate_ ? tr("Updating %1").arg(shown) : tr("Installing %1").arg(shown), unchanged, shown);
    if (!queue_.isEmpty())
        words += tr("\n\nThe %1 still waiting in Update all %2 not started.")
                     .arg(counted(queue_.size(), tr("%1 update"), tr("%1 updates")), queue_.size() == 1 ? tr("is") : tr("are"));
    return words;
}

void Window::closeEvent(QCloseEvent *event)
{
    if (closing_ || !runningRun()) {
        QMainWindow::closeEvent(event);
        return;
    }
    // Closing would end the run without a word: ask first, with what is true.
    event->ignore();
    askClose();
}

void Window::askClose()
{
    if (closeBox_) {
        closeBox_->raise();
        return;
    }
    auto *box = new QMessageBox(this);
    box->setAttribute(Qt::WA_DeleteOnClose);
    box->setIcon(QMessageBox::Question);
    box->setWindowTitle(tr("Close while it runs?"));
    const QString words = closeWords();
    box->setText(words.section(QStringLiteral("\n\n"), 0, 0));
    box->setInformativeText(words.section(QStringLiteral("\n\n"), 1));
    QPushButton *stop = box->addButton(tr("Stop it and close"), QMessageBox::AcceptRole);
    QPushButton *keep = box->addButton(tr("Keep the window open"), QMessageBox::RejectRole);
    box->setDefaultButton(keep);
    box->setEscapeButton(keep);
    closeBox_ = box;
    connect(stop, &QPushButton::clicked, this, [this] { stopAndClose(); });
    box->open();
}

// The person said to stop it: the run is asked to end, and the window closes when it has.
void Window::stopAndClose()
{
    Run *run = runningRun();
    if (!run) {
        closing_ = true;
        close();
        return;
    }
    closeAfterStop_ = true;
    frame_->flash(tr("Stopping it, then closing…"));
    run->stop();
    // A run that does not answer is ended with the window.
    QTimer::singleShot(8000, this, [this] {
        if (closeAfterStop_) {
            closing_ = true;
            close();
        }
    });
}
