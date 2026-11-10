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
    void install(const QString &name);
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
    void install(const QString &name);
    void remove(const QString &name);
    void open(const QString &name);
};

class RunPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void begin(const Catalog &catalog, const QString &name, Run *run);
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
    Run *run_ = nullptr;
    QVBoxLayout *column_ = nullptr;
    QGridLayout *plan_ = nullptr;
    QMap<QString, StepMark *> marks_;
    QLabel *title_ = nullptr, *state_ = nullptr, *outputTitle_ = nullptr;
    QString agentNote_;
    QProgressBar *bar_ = nullptr;
    QPlainTextEdit *output_ = nullptr;
    QPushButton *stop_ = nullptr;
    QLabel *stopNote_ = nullptr;
    int stepsTotal_ = 0, stepsStarted_ = 0;
};

class InstalledPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &selected);
signals:
    void open(const QString &name);
    void goBack(const QString &name);
    // Delete every earlier version Recipe keeps.
    void clearEarlier();
    void remove(const QString &name);
    void update(const QString &name);
    void selected(const QString &name);
};

class UpdatesPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &selected);
signals:
    void update(const QString &name);
    void updateAll();
    void selected(const QString &name);
    // Fetch every source's list again, to see whether anything is newer.
    void check();
};

class RemoveDialog : public QDialog {
public:
    RemoveDialog(const Recipe &recipe, const InstalledItem &item, QWidget *parent = nullptr);
};
