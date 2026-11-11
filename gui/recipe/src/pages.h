// The pages of the window. Each one is told what to show and rebuilds itself;
// it asks for actions with signals and never runs `recipe` itself.
#pragma once

#include "model.h"
#include "page.h"

#include <QDialog>
#include <QMap>
#include <QWidget>

class QCheckBox;
class QComboBox;
class QGridLayout;
class QLabel;
class QLineEdit;
class QPlainTextEdit;
class QProgressBar;
class QPushButton;
class QTreeWidget;
class QVBoxLayout;
class Run;
class StepMark;

// The list of recipes under its own header, search and two boxes. Those stay
// while the list is drawn again, so typing is not interrupted.
class BrowsePage : public Page {
    Q_OBJECT
public:
    explicit BrowsePage(QWidget *parent = nullptr);
    // `origin`: only the recipes of this source; empty for all.
    void show(const Catalog &catalog, const QString &category, const QString &search, const QString &origin);
    void showProblem(const QString &problem);
    // Reading or failing: the header and the boxes are not shown, the words are.
    void showLoading(const QString &title, const QString &words);
    QLineEdit *searchBox() const { return search_; }
    QComboBox *fromBox() const { return from_; }
    QComboBox *categoryBox() const { return category_; }
    // The recipes the list shows now, in order, for a check.
    QStringList listed() const { return listed_; }
signals:
    void openRecipe(const QString &name);
    // Install asks where; update acts where the software is.
    void install(const QString &name);
    void update(const QString &name, Place place);
    void retry();
    // Ask the agent to write a recipe for this.
    void askFor(const QString &what);
private:
    QWidget *top_;
    QLabel *title_, *subtitle_;
    QLineEdit *search_;
    QComboBox *from_, *category_;
    QStringList listed_;
};

class RecipePage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &name);
signals:
    void back();
    // From the page of software whose recipe is gone: back to what is installed.
    void backToInstalled();
    void install(const QString &name);
    void update(const QString &name, Place place);
    void remove(const QString &name, Place place);
    void open(const QString &name, Place place);
    void goBack(const QString &name, Place place);
    // Keep it at its version (`on`), or let it update again.
    void hold(const QString &name, Place place, bool on);
    // Open the recipe's file.
    void readRecipe(const QString &name);
private:
    // Where the software is, in each place that holds it.
    void whereItIs(QVBoxLayout *body, const Catalog &catalog, const QString &name);
    // What can be done to what is installed: keep it, go back, remove. Nothing here needs a recipe.
    void actionRows(QVBoxLayout *body, const Catalog &catalog, const QString &name,
                    const QList<const InstalledItem *> &held);
    // Installed software whose recipe is gone: what its receipt says, and what still works.
    void showReceipt(const Catalog &catalog, const QString &name);
};

class RunPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void begin(const Catalog &catalog, const QString &name, Run *run, Place place = Place::Computer);
signals:
    void back();
    void retry(const QString &name);
    // Hand the failure to the agent: the issue, the facts and the expected result.
    void fix(const QString &name, const QString &prompt);
    void note(const QString &words);
    void succeeded(const QString &name);
    void failed(const QString &name);
    void status(const QString &words);
private:
    void onEvent(const QJsonObject &event);
    void onFinished(bool ok);
    StepMark *mark(const QString &recipe, const QString &hook);
    void addPlanRow(const QString &recipe, const QStringList &hooks);

    Catalog catalog_;
    QString name_, updatingFrom_;
    Place place_ = Place::Computer;
    Run *run_ = nullptr;
    QVBoxLayout *column_ = nullptr;
    QGridLayout *plan_ = nullptr;
    QWidget *planBox_ = nullptr;
    QMap<QString, StepMark *> marks_;
    QLabel *title_ = nullptr, *state_ = nullptr, *outputTitle_ = nullptr;
    QString agentNote_;
    QProgressBar *bar_ = nullptr;
    QPlainTextEdit *output_ = nullptr;
    QPushButton *stop_ = nullptr;
    QLabel *stopNote_ = nullptr;
    int stepsTotal_ = 0, stepsStarted_ = 0;
    // The run's output as the agent can be given it, and where `recipe` said it keeps its log.
    QStringList lines_;
    QString log_;
};

// A short change that has no steps of its own: removing, going back, keeping at a
// version, adding or removing a source, fetching the lists, clearing earlier versions.
// It shows what is being done, lets it be stopped, shows the command's own output,
// and when it did not finish says why and offers "Try again". The window drives it.
class ChangePage : public Page {
    Q_OBJECT
public:
    struct Words {
        // "Removing GIMP", what it does, and the line under the bar.
        QString title, subtitle, doing;
        // What stopping it means (also asked when the window is closed meanwhile).
        QString stopNote;
        // When it did not finish: the headline, and what is true then.
        QString failed, left;
        // Said in the status line when it worked.
        QString done;
    };
    using Page::Page;
    void begin(const Words &words, Run *run);
signals:
    void retry();
    void back();
private:
    void onFinished(bool ok);
    Words words_;
    Run *run_ = nullptr;
    QVBoxLayout *column_ = nullptr;
    QWidget *progress_ = nullptr;
    QPlainTextEdit *output_ = nullptr;
    QLabel *outputTitle_ = nullptr;
    QPushButton *stop_ = nullptr;
    QLabel *stopNote_ = nullptr;
};

class InstalledPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // `selectedPlace`: which place's row when a name is in two; -1 for the first.
    void show(const Catalog &catalog, const QString &selected, int selectedPlace = -1);
signals:
    void open(const QString &name, Place place);
    void goBack(const QString &name, Place place);
    void remove(const QString &name, Place place);
    void update(const QString &name, Place place);
    void hold(const QString &name, Place place, bool on);
    void openStorage();
    void selected(const QString &name, Place place);
    // The page of software whose recipe is gone: what its receipt says.
    void openPage(const QString &name);
};

class UpdatesPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &selected, int selectedPlace = -1);
signals:
    void update(const QString &name, Place place);
    void hold(const QString &name, Place place, bool on);
    void updateAll();
    void selected(const QString &name, Place place);
    // Fetch every source's list again, to see whether anything is newer.
    void check();
};

class RemoveDialog : public QDialog {
public:
    RemoveDialog(const Recipe &recipe, const InstalledItem &item, QWidget *parent = nullptr);
};
