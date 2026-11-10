// The pages of the window. Each one is told what to show and rebuilds itself;
// it asks for actions with signals and never runs `recipe` itself.
#pragma once

#include "model.h"

#include <QDialog>
#include <QMap>
#include <QWidget>

class QCheckBox;
class QGridLayout;
class QLabel;
class QPlainTextEdit;
class QProgressBar;
class QPushButton;
class QTreeWidget;
class QVBoxLayout;
class Run;
class StepMark;

class Page : public QWidget {
protected:
    explicit Page(QWidget *parent = nullptr);
    // Drops what the page showed and returns an empty column to fill.
    QVBoxLayout *fresh();
private:
    QVBoxLayout *outer_;
    QWidget *body_ = nullptr;
};

class BrowsePage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &category, const QString &search);
    void showProblem(const QString &problem);
signals:
    void openRecipe(const QString &name);
    void install(const QString &name);
    void retry();
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
};

class RemoveDialog : public QDialog {
public:
    RemoveDialog(const Recipe &recipe, const InstalledItem &item, QWidget *parent = nullptr);
    bool clearKept() const;
private:
    QCheckBox *clear_ = nullptr;
};
