// The one window: a sidebar, the pages, and the flows between them.
#pragma once

#include "client.h"
#include "model.h"

#include <QMainWindow>

#include <functional>

class BrowsePage;
class InstalledPage;
class QLabel;
class QLineEdit;
class QListWidget;
class QStackedWidget;
class RecipePage;
class RunPage;
class UpdatesPage;

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

    const Catalog &catalog() const { return catalog_; }
    bool busy() const { return run_ != nullptr || quiet_ > 0; }
    // From a checkout: remove without the question.
    void removeNow(const QString &name);

private:
    bool reload();
    void fillSidebar();
    void mark(const QString &key);
    void open(const QString &name);
    void goBack(const QString &name);
    void quietChange(Run *run);
    void runEnded(const QString &name, bool ok);

    RecipeClient client_;
    Catalog catalog_;
    Run *run_ = nullptr;
    int quiet_ = 0;
    QStringList queue_;
    std::function<void()> again_;
    QString installedChoice_, updateChoice_;

    QLineEdit *search_;
    QListWidget *side_;
    QStackedWidget *stack_;
    BrowsePage *browse_;
    RecipePage *recipe_;
    RunPage *runPage_;
    InstalledPage *installed_;
    UpdatesPage *updates_;
    QLabel *source_, *activity_;
};
