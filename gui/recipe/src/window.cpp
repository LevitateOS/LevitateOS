#include "window.h"

#include "pages.h"
#include "parts.h"

#include <QHBoxLayout>
#include <QLineEdit>
#include <QListWidget>
#include <QMessageBox>
#include <QStackedWidget>
#include <QStatusBar>

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
    for (QWidget *page : {static_cast<QWidget *>(browse_), static_cast<QWidget *>(recipe_),
                          static_cast<QWidget *>(runPage_), static_cast<QWidget *>(installed_),
                          static_cast<QWidget *>(updates_)})
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
        else
            showBrowse(key.mid(7));
    });
    connect(search_, &QLineEdit::textChanged, this, [this] { showBrowse(); });

    connect(browse_, &BrowsePage::openRecipe, this, &Window::showRecipe);
    connect(browse_, &BrowsePage::install, this, [this](const QString &name) { startInstall(name); });
    connect(browse_, &BrowsePage::retry, this, [this] {
        if (reload())
            showBrowse();
    });
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

    if (reload())
        showBrowse();
}

bool Window::reload()
{
    QString problem;
    if (!client_.load(&catalog_, &problem)) {
        browse_->showProblem(problem);
        stack_->setCurrentWidget(browse_);
        return false;
    }
    QString source = catalog_.checked.isEmpty() ? catalog_.sourceTitle
                                                : tr("%1, checked %2").arg(catalog_.sourceTitle, catalog_.checked);
    if (catalog_.unreadable)
        source += tr(" · %1").arg(counted(catalog_.unreadable, tr("%1 recipe could not be read"),
                                           tr("%1 recipes could not be read")));
    source_->setText(source);
    fillSidebar();
    return true;
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
    again_ = [this, category] { showBrowse(category); };
    mark("browse:" + (search.isEmpty() ? category : QString()));
    browse_->show(catalog_, search.isEmpty() ? category : QString(), search);
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
    run_ = !agentPrompt.isEmpty() ? client_.fixWithAgent(name, agentPrompt, installed)
           : installed ? client_.update(name) : client_.install(name);
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
    reload();
    if (!ok) {
        queue_.clear();
        return;
    }
    if (!queue_.isEmpty())
        startInstall(queue_.takeFirst());
    else
        showRecipe(name);
}

void Window::askRemove(const QString &name)
{
    const Recipe *recipe = catalog_.recipe(name);
    const InstalledItem *item = catalog_.installedItem(name);
    if (!recipe || !item)
        return;
    RemoveDialog dialog(*recipe, *item, this);
    if (dialog.exec() == QDialog::Accepted)
        quietChange(client_.remove(name, dialog.clearKept()));
}

void Window::removeNow(const QString &name)
{
    quietChange(client_.remove(name, false));
}

void Window::goBack(const QString &name)
{
    quietChange(client_.goBack(name));
}

// A short change with no page of its own: run it, then show the page again.
void Window::quietChange(Run *run)
{
    setEnabled(false);
    ++quiet_;
    connect(run, &Run::finished, this, [this, run](bool ok) {
        setEnabled(true);
        --quiet_;
        if (!ok) {
            const QString reason = run->failure()["reason"].toString();
            QMessageBox::warning(this, tr("Recipe"),
                                 reason.isEmpty() ? tr("The change was not made.") : tr("The change was not made: %1.").arg(reason));
        }
        run->deleteLater();
        if (reload() && again_)
            again_();
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
