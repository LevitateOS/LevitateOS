// The one window: a sidebar, the pages, and the flows between them.
#pragma once

#include "client.h"
#include "model.h"

#include <QMainWindow>

#include <functional>

class BrowsePage;
class InstalledPage;
class QComboBox;
class QLabel;
class QLineEdit;
class QListWidget;
class QStackedWidget;
class RecipePage;
class Refresher;
class RunPage;
class SourcesPage;
class UpdatesPage;
class WishPage;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    void showBrowse(const QString &category = QString());
    void showRecipe(const QString &name);
    void showInstalled(const QString &selected = QString());
    void showUpdates(const QString &selected = QString());
    void startInstall(const QString &name, const QString &agentPrompt = QString());
    void askRemove(const QString &name);
    void stopRun();
    void showSources();
    // Ask the agent for a recipe: the page, and handing its prompt over.
    void showWish(const QString &wish = QString());
    void askAgent(const QString &prompt);
    bool agentAsks() const;
    void answerAgent(const QString &words);
    // Reads the catalog again, as F5 does.
    void refresh();
    // Fetches the sources' lists again (one, or all), then reads the catalog.
    void fetchSources(const QString &name = QString());
    void addSource(const QString &addressOrFolder);
    void removeSource(const QString &name, bool askFirst = true);
    // Only the recipes of one source; empty for all.
    void chooseSource(const QString &name);

    const Catalog &catalog() const { return catalog_; }
    bool busy() const { return run_ != nullptr || quiet_ > 0 || client_.loading(); }
    // From a checkout: remove without the question.
    void removeNow(const QString &name);
    // Activates the newest earlier version Recipe keeps of an installed program.
    void goBack(const QString &name);
    // Deletes every earlier version Recipe keeps, after asking unless told not to.
    void clearEarlier(bool askFirst = true);

private:
    // Reads the catalog without holding the window up, then does `then`.
    // When it cannot be read, the page says why and `then` is not done.
    void reload(std::function<void()> then);
    void fillSidebar();
    void fillSources();
    void mark(const QString &key);
    void open(const QString &name);
    void quietChange(Run *run);
    void runEnded(const QString &name, bool ok);

    RecipeClient client_;
    Catalog catalog_;
    Run *run_ = nullptr;
    int quiet_ = 0;
    QStringList queue_;
    std::function<void()> again_;
    QString installedChoice_, updateChoice_;
    QString origin_;    // the source chosen in the sidebar; empty: all
    QString category_;  // the category last browsed
    bool read_ = false; // the catalog was read at least once
    QString written_;   // the recipe the agent said it wrote
    Refresher *refresher_;

    QLineEdit *search_;
    QListWidget *side_;
    QStackedWidget *stack_;
    BrowsePage *browse_;
    RecipePage *recipe_;
    RunPage *runPage_;
    InstalledPage *installed_;
    UpdatesPage *updates_;
    SourcesPage *sources_;
    WishPage *wish_;
    WorkPage *work_;
    QComboBox *from_;
    QLabel *source_, *activity_;
};
