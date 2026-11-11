// The one window: the shared frame, the pages in it, and the flows between them.
#pragma once

#include "beforewords.h"
#include "client.h"
#include "model.h"
#include "pages.h"

#include <QMainWindow>
#include <QMessageBox>
#include <QPointer>

#include <functional>



class BeforePage;
class BrowsePage;
class Frame;
class InstalledPage;
class NextSystem;
class QStackedWidget;
class RecipePage;
class Refresher;
class RunPage;
class SourcesPage;
class StoragePage;
class UpdatesPage;
class WishPage;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    void showBrowse(const QString &category = QString());
    void showRecipe(const QString &name);
    void showInstalled(const QString &selected = QString(), int place = -1);
    void showUpdates(const QString &selected = QString(), int place = -1);
    // The Before step: what will happen, shown before anything starts. An
    // install asks where; an update acts where the software is. The page's
    // button goes on to `startInstall`.
    void askInstall(const QString &name);
    void askUpdate(const QString &name, Place place);
    // Starts the install or update at once, with no Before step: for the run
    // the person has just agreed to, and for "Try again".
    void startInstall(const QString &name, Place place = Place::Computer, const QString &agentPrompt = QString());
    void askRemove(const QString &name, Place place = Place::Computer);
    void stopRun();
    void showSources();
    void showStorage();
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
    Frame *frame() const { return frame_; }
    BrowsePage *browsePage() const { return browse_; }
    BeforePage *beforePage() const { return before_; }
    Places places() const { return client_.places(); }
    bool busy() const { return run_ != nullptr || changeRun_ != nullptr || client_.loading(); }
    // From a checkout: remove without the question.
    void removeNow(const QString &name, Place place = Place::Computer);
    // Activates the newest earlier version Recipe keeps of an installed program.
    void goBack(const QString &name, Place place = Place::Computer);
    // Deletes every earlier version the store of a place keeps, after asking unless told not to.
    void clearEarlier(Place place = Place::Computer, bool askFirst = true);
    // Keeps an installed program at its version, or lets it update again.
    void hold(const QString &name, Place place, bool on);
    StoragePage *storagePage() const { return storage_; }
    InstalledPage *installedPage() const { return installed_; }
    UpdatesPage *updatesPage() const { return updates_; }
    RecipePage *recipePage() const { return recipe_; }
    RunPage *runPage() const { return runPage_; }
    ChangePage *changePage() const { return change_; }
    // Starts the program of an installed piece of software, from the profile of the place it is in.
    void open(const QString &name, Place place);
    // The question asked when the window is closed while something runs, and the one that asks
    // whether to remove: open until answered, else null. For a check.
    QMessageBox *closeQuestion() const { return closeBox_; }
    RemoveDialog *removeQuestion() const { return removeBox_; }

protected:
    void closeEvent(QCloseEvent *event) override;

private:
    // Reads the catalog without holding the window up, then does `then`.
    // When it cannot be read, the page says why and `then` is not done.
    void reload(std::function<void()> then);
    // The boxes of the Browse page, and the number on Updates.
    void fillSources();
    void fillCategories();
    void fillRail();
    // The status line: where the recipes are from, and what is being built.
    void updateStatus();
    // Takes a said-for-a-moment line away at once.
    void clearFlash();
    void showBefore();
    void startQueued();
    // A short change has a page of its own: what is being done, Stop, the command's output, and
    // when it did not finish, why. `make` starts the same command again for "Try again".
    void change(const ChangePage::Words &words, std::function<Run *()> make);
    void leaveChange();
    void changeEnded(bool ok);
    void runEnded(const QString &name, bool ok);
    // Closing while something runs: the question, what it says, and ending the run first.
    void askClose();
    void stopAndClose();
    Run *runningRun() const;
    QString closeWords() const;
    ChangePage::Words removeWords(const QString &name, Place place) const;
    QString shownName(const QString &name) const;

    RecipeClient client_;
    Catalog catalog_;
    Run *run_ = nullptr;
    // The short change that is running (remove, go back, hold, a source, clearing), if any.
    Run *changeRun_ = nullptr;
    std::function<Run *()> remake_;
    ChangePage::Words changeWords_;
    // What the install or update that runs is, for the question when the window closes.
    QString runName_;
    bool runUpdate_ = false;
    QPointer<QMessageBox> closeBox_;
    QPointer<RemoveDialog> removeBox_;
    bool closing_ = false, closeAfterStop_ = false;
    // A read asked for while another is going: done as soon as that one ends.
    bool reloadQueued_ = false;
    std::function<void()> queuedThen_;
    // Updates still to be asked about and made, one after another.
    QList<QPair<QString, Place>> queue_;
    Ask ask_;           // what the Before page is about
    Place runPlace_ = Place::Computer; // where the running (or last) run acts
    std::function<void()> again_;
    QString installedChoice_, updateChoice_;
    int installedPlace_ = -1, updatePlace_ = -1;
    QString origin_;    // the source chosen on the Browse page; empty: all
    QString category_;  // the category last browsed
    bool read_ = false; // the catalog was read at least once
    QString written_;   // the recipe the agent said it wrote
    Refresher *refresher_;
    QString readWords_, buildingWords_;

    Frame *frame_;
    NextSystem *next_;
    QStackedWidget *stack_;
    BrowsePage *browse_;
    RecipePage *recipe_;
    BeforePage *before_;
    RunPage *runPage_;
    ChangePage *change_;
    InstalledPage *installed_;
    UpdatesPage *updates_;
    SourcesPage *sources_;
    StoragePage *storage_;
    WishPage *wish_;
    WorkPage *work_;
};
