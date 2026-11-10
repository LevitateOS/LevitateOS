#include "window.h"

#include "pages.h"
#include "parts.h"
#include "refresh.h"
#include "sources.h"
#include "work.h"

#include <QComboBox>
#include <QHBoxLayout>
#include <QLineEdit>
#include <QListWidget>
#include <QMessageBox>
#include <QStackedWidget>
#include <QStatusBar>
#include <QTime>
#include <QToolButton>

Window::Window(QWidget *parent)
    : QMainWindow(parent)
{
    setWindowTitle(tr("Recipe"));

    auto *centre = new QWidget;
    auto *row = new QHBoxLayout(centre);
    row->setContentsMargins(0, 0, 0, 0);
    row->setSpacing(0);

    auto *sidePane = new QWidget;
    sidePane->setFixedWidth(216);
    auto *sideColumn = new QVBoxLayout(sidePane);
    sideColumn->setContentsMargins(8, 12, 8, 8);
    sideColumn->setSpacing(8);
    search_ = new QLineEdit;
    search_->setPlaceholderText(tr("Search"));
    search_->setClearButtonEnabled(true);
    search_->setAccessibleName(tr("Search software"));
    sideColumn->addWidget(search_);
    // Which source's recipes are shown: all of them, or one.
    from_ = new QComboBox;
    from_->setAccessibleName(tr("Recipes from"));
    sideColumn->addWidget(from_);
    side_ = new QListWidget;
    side_->setFrameShape(QFrame::NoFrame);
    side_->setSpacing(1);
    side_->viewport()->setAutoFillBackground(false);
    sideColumn->addWidget(side_, 1);
    row->addWidget(sidePane);

    auto *edge = new QFrame;
    edge->setFrameShape(QFrame::VLine);
    edge->setFrameShadow(QFrame::Sunken);
    row->addWidget(edge);

    stack_ = new QStackedWidget;
    browse_ = new BrowsePage;
    recipe_ = new RecipePage;
    runPage_ = new RunPage;
    installed_ = new InstalledPage;
    updates_ = new UpdatesPage;
    sources_ = new SourcesPage;
    wish_ = new WishPage;
    work_ = new WorkPage;
    for (QWidget *page : {static_cast<QWidget *>(browse_), static_cast<QWidget *>(recipe_),
                          static_cast<QWidget *>(runPage_), static_cast<QWidget *>(installed_),
                          static_cast<QWidget *>(updates_), static_cast<QWidget *>(sources_),
                          static_cast<QWidget *>(wish_), static_cast<QWidget *>(work_)})
        stack_->addWidget(page);
    row->addWidget(stack_, 1);
    setCentralWidget(centre);

    source_ = new QLabel;
    activity_ = new QLabel(tr("Nothing is being built"));
    statusBar()->addWidget(source_);
    statusBar()->addPermanentWidget(activity_);
    statusBar()->setSizeGripEnabled(false);

    connect(side_, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *item) {
        if (!item)
            return;
        const QString key = item->data(Qt::UserRole).toString();
        if (key == QLatin1String("installed"))
            showInstalled(installedChoice_);
        else if (key == QLatin1String("updates"))
            showUpdates(updateChoice_);
        else if (key == QLatin1String("sources"))
            showSources();
        else
            showBrowse(key.mid(7));
    });
    connect(search_, &QLineEdit::textChanged, this, [this] { showBrowse(); });

    connect(browse_, &BrowsePage::openRecipe, this, &Window::showRecipe);
    connect(browse_, &BrowsePage::install, this, [this](const QString &name) { startInstall(name); });
    connect(browse_, &BrowsePage::retry, this, [this] { reload([this] { showBrowse(); }); });
    connect(browse_, &BrowsePage::askFor, this, &Window::showWish);
    connect(from_, &QComboBox::activated, this, [this](int index) { chooseSource(from_->itemData(index).toString()); });
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
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        activity_->setText(words.isEmpty() ? tr("Nothing is being built") : words);
    });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
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
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("Read the recipes and what is installed again (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    statusBar()->addWidget(again);
    connect(recipe_, &RecipePage::back, this, [this] { showBrowse(); });
    connect(recipe_, &RecipePage::install, this, [this](const QString &name) { startInstall(name); });
    connect(recipe_, &RecipePage::remove, this, &Window::askRemove);
    connect(recipe_, &RecipePage::open, this, &Window::open);
    connect(runPage_, &RunPage::back, this, [this] { showBrowse(); });
    connect(runPage_, &RunPage::retry, this, [this](const QString &name) { startInstall(name); });
    connect(runPage_, &RunPage::fix, this, &Window::startInstall);
    connect(runPage_, &RunPage::note, this, [this](const QString &words) { statusBar()->showMessage(words, 20000); });
    connect(runPage_, &RunPage::succeeded, this, [this](const QString &name) { runEnded(name, true); });
    connect(runPage_, &RunPage::failed, this, [this](const QString &name) { runEnded(name, false); });
    connect(runPage_, &RunPage::status, this, [this](const QString &words) {
        activity_->setText(words.isEmpty() ? tr("Nothing is being built") : words);
    });
    connect(installed_, &InstalledPage::open, this, &Window::open);
    connect(installed_, &InstalledPage::goBack, this, &Window::goBack);
    connect(installed_, &InstalledPage::clearEarlier, this, [this] { clearEarlier(); });
    connect(installed_, &InstalledPage::remove, this, &Window::askRemove);
    connect(installed_, &InstalledPage::update, this, [this](const QString &name) { startInstall(name); });
    connect(installed_, &InstalledPage::selected, this, [this](const QString &name) { installedChoice_ = name; });
    connect(updates_, &UpdatesPage::update, this, [this](const QString &name) { startInstall(name); });
    connect(updates_, &UpdatesPage::selected, this, [this](const QString &name) { updateChoice_ = name; });
    connect(updates_, &UpdatesPage::updateAll, this, [this] {
        queue_.clear();
        for (const UpdateItem &item : catalog_.updates)
            queue_ << item.name;
        if (!queue_.isEmpty())
            startInstall(queue_.takeFirst());
    });

    reload([this] { showBrowse(); });
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
    source_->setText(tr("Reading the recipes…"));
    client_.load([this, then](bool ok, const Catalog &catalog, const QString &problem) {
        if (!ok) {
            source_->setText(tr("The recipes could not be read"));
            browse_->showProblem(problem);
            stack_->setCurrentWidget(browse_);
            return;
        }
        catalog_ = catalog;
        read_ = true;
        refresher_->fresh();
        QString source = tr("%1, read at %2")
                             .arg(catalog_.originsWords(), QLocale().toString(QTime::currentTime(), QLocale::ShortFormat));
        if (catalog_.unreadable)
            source += tr(" · %1").arg(counted(catalog_.unreadable, tr("%1 recipe could not be read"),
                                               tr("%1 recipes could not be read")));
        source_->setText(source);
        if (!origin_.isEmpty() && !catalog_.origin(origin_))
            origin_.clear(); // the chosen source is gone
        fillSources();
        fillSidebar();
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
    const bool drawn = shown != runPage_ && shown != work_ && shown != wish_;
    reload([this, drawn] {
        if (drawn && again_)
            again_();
    });
}

void Window::fillSources()
{
    const QSignalBlocker quiet(from_);
    from_->clear();
    from_->addItem(tr("From all sources"), QString());
    for (const Origin &origin : catalog_.origins)
        if (catalog_.countFrom(origin.name) > 0)
            from_->addItem(tr("From %1").arg(origin.shown()), origin.name);
    from_->setCurrentIndex(qMax(0, from_->findData(origin_)));
    // One source: nothing to choose between.
    from_->setVisible(from_->count() > 2);
}

void Window::chooseSource(const QString &name)
{
    origin_ = name;
    const QSignalBlocker quiet(from_);
    from_->setCurrentIndex(qMax(0, from_->findData(origin_)));
    if (stack_->currentWidget() == browse_)
        showBrowse(category_);
}

void Window::fillSidebar()
{
    const QString kept = side_->currentItem() ? side_->currentItem()->data(Qt::UserRole).toString() : QString();
    QSignalBlocker quiet(side_);
    side_->clear();
    auto section = [this](const QString &title) {
        auto *item = new QListWidgetItem(title, side_);
        item->setFlags(Qt::NoItemFlags);
        QFont small = item->font();
        small.setPointSizeF(small.pointSizeF() * 0.9);
        small.setBold(true);
        item->setFont(small);
    };
    auto entry = [this](const QString &title, const QString &key) {
        auto *item = new QListWidgetItem(title, side_);
        item->setData(Qt::UserRole, key);
        item->setSizeHint(QSize(0, 30));
    };
    section(tr("Browse"));
    entry(tr("All software"), "browse:");
    for (const QString &category : catalog_.categories())
        entry(categoryTitle(category), "browse:" + category);
    section(tr("On this computer"));
    entry(tr("Installed (%1)").arg(catalog_.installed.size()), "installed");
    entry(catalog_.updates.isEmpty() ? tr("Updates") : tr("Updates (%1)").arg(catalog_.updates.size()), "updates");
    entry(tr("Sources (%1)").arg(catalog_.origins.size()), "sources");
    mark(kept.isEmpty() ? QStringLiteral("browse:") : kept);
}

void Window::mark(const QString &key)
{
    QSignalBlocker quiet(side_);
    for (int index = 0; index < side_->count(); ++index)
        if (side_->item(index)->data(Qt::UserRole).toString() == key) {
            side_->setCurrentRow(index);
            return;
        }
}

void Window::showBrowse(const QString &category)
{
    const QString search = search_->text().trimmed();
    category_ = category;
    again_ = [this, category] { showBrowse(category); };
    mark("browse:" + (search.isEmpty() ? category : QString()));
    browse_->show(catalog_, search.isEmpty() ? category : QString(), search, origin_);
    stack_->setCurrentWidget(browse_);
}

void Window::showRecipe(const QString &name)
{
    again_ = [this, name] { showRecipe(name); };
    recipe_->show(catalog_, name);
    stack_->setCurrentWidget(recipe_);
}

void Window::showInstalled(const QString &selected)
{
    again_ = [this] { showInstalled(installedChoice_); };
    mark("installed");
    installed_->show(catalog_, selected);
    stack_->setCurrentWidget(installed_);
}

void Window::showUpdates(const QString &selected)
{
    again_ = [this] { showUpdates(updateChoice_); };
    mark("updates");
    updates_->show(catalog_, selected);
    stack_->setCurrentWidget(updates_);
}

void Window::startInstall(const QString &name, const QString &agentPrompt)
{
    if (run_) {
        statusBar()->showMessage(tr("One thing at a time: wait for the running one, or stop it."), 6000);
        stack_->setCurrentWidget(runPage_);
        return;
    }
    const bool installed = catalog_.installedItem(name) != nullptr;
    // `recipe` is given SOURCE:NAME, so two sources with one name cannot be confused.
    const QString named = catalog_.qualified(name);
    run_ = !agentPrompt.isEmpty() ? client_.fixWithAgent(named, agentPrompt, installed)
           : installed ? client_.update(named) : client_.install(named);
    runPage_->begin(catalog_, name, run_);
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
            startInstall(queue_.takeFirst());
        else
            showRecipe(name);
    });
}

// ---------------------------------------------------------------- Sources

void Window::showSources()
{
    again_ = [this] { showSources(); };
    mark("sources");
    sources_->show(catalog_);
    stack_->setCurrentWidget(sources_);
}

void Window::fetchSources(const QString &name)
{
    statusBar()->showMessage(name.isEmpty() ? tr("Fetching every source's list…") : tr("Fetching the list of %1…").arg(name));
    quietChange(client_.refresh(name));
}

void Window::addSource(const QString &addressOrFolder)
{
    statusBar()->showMessage(tr("Adding the source…"));
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
        statusBar()->showMessage(tr("One thing at a time: wait for the running one, or stop it."), 6000);
        return;
    }
    written_.clear();
    run_ = client_.agent(prompt);
    // The agent names the recipe it wrote, so the window can open it.
    connect(run_, &Run::event, this, [this](const QJsonObject &event) {
        if (event["event"] == QLatin1String("agent") && !event["recipe"].toString().isEmpty())
            written_ = event["recipe"].toString();
    });
    work_->begin(tr("The agent is writing a recipe"), tr("It only writes the recipe. Nothing is installed."), run_,
                 tr("A half-written recipe is left in the folder"));
    stack_->setCurrentWidget(work_);
    run_->start();
}

bool Window::agentAsks() const
{
    return run_ && stack_->currentWidget() == work_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

void Window::askRemove(const QString &name)
{
    const Recipe *recipe = catalog_.recipe(name);
    const InstalledItem *item = catalog_.installedItem(name);
    if (!recipe || !item)
        return;
    RemoveDialog dialog(*recipe, *item, this);
    if (dialog.exec() == QDialog::Accepted)
        quietChange(client_.remove(name));
}

void Window::removeNow(const QString &name)
{
    quietChange(client_.remove(name));
}

void Window::goBack(const QString &name)
{
    quietChange(client_.goBack(name));
}

void Window::clearEarlier(bool askFirst)
{
    if (askFirst
        && QMessageBox::question(this, tr("Clear earlier versions"),
                                 tr("Delete every earlier version Recipe keeps (%1)?\n\nNothing that is installed "
                                    "changes. Afterwards no program can be taken back to an earlier version until "
                                    "it is updated again.")
                                     .arg(bytesText(catalog_.earlierBytes)),
                                 QMessageBox::Yes | QMessageBox::Cancel)
               != QMessageBox::Yes)
        return;
    quietChange(client_.clearEarlier());
}

// A short change with no page of its own: run it, then show the page again.
void Window::quietChange(Run *run)
{
    setEnabled(false);
    ++quiet_;
    connect(run, &Run::finished, this, [this, run](bool ok) {
        setEnabled(true);
        --quiet_;
        statusBar()->clearMessage();
        if (!ok) {
            const QString reason = run->problem();
            QMessageBox::warning(this, tr("Recipe"),
                                 reason.isEmpty() ? tr("The change was not made.") : tr("The change was not made: %1.").arg(reason));
        }
        run->deleteLater();
        reload([this] {
            if (again_)
                again_();
        });
    });
    run->start();
}

void Window::open(const QString &name)
{
    const InstalledItem *item = catalog_.installedItem(name);
    if (!item || item->programs.isEmpty())
        return;
    if (!QProcess::startDetached(item->programs.first(), {}))
        statusBar()->showMessage(tr("%1 could not be started").arg(item->programs.first()), 6000);
}
