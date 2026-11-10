// The pages of the window. Each one is told what to show and rebuilds itself;
// it asks for actions with signals and never runs anything itself.
#pragma once

#include "model.h"

#include <QDialog>
#include <QMap>
#include <QWidget>

class PromptPanel;
class QLabel;
class QHBoxLayout;
class QPlainTextEdit;
class QProgressBar;
class QPushButton;
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

// The one page a person sees most: the running system, what is beside it, and
// what can be done. Its shape follows recab's state.
class SystemPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
    void showProblem(const QString &problem);
signals:
    void ask();
    void addUpdate();
    void goBack();
    void finish();
    void restart();
    void discard();
    void recover();
    void settle();
    void fix(const QString &prompt);
    void retry();
};

// Say the change in words; an agent works out how.
class AskPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status, const QString &request = QString());
signals:
    void back();
    void handOver(const QString &prompt);
};

// Something is being done to the next system: its steps, how far, its output.
class WorkPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void begin(const QString &title, const QString &subtitle, Run *run);
signals:
    void done(bool ok);
    void back();
    void activity(const QString &words);
private:
    void onEvent(const QJsonObject &event);
    void onFinished(bool ok);

    Run *run_ = nullptr;
    QVBoxLayout *column_ = nullptr;
    QHBoxLayout *steps_ = nullptr;
    QMap<QString, StepMark *> marks_;
    QMap<QString, QString> titles_;
    QLabel *state_ = nullptr;
    QProgressBar *bar_ = nullptr;
    QPlainTextEdit *output_ = nullptr;
    QPushButton *stop_ = nullptr;
    QLabel *stopNote_ = nullptr;
};

// An update and a change clash: what the agent is given to settle it.
class ClashPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
signals:
    void later();
    void byHand();
    void fix(const QString &prompt);
};

// What the agent did about the clash, against each thing that was expected.
class ProposedPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
signals:
    void accept();
    void askAgain();
    void byHand();
};

// A question with one sentence of what it costs.
class ConfirmDialog : public QDialog {
public:
    ConfirmDialog(const QString &title, const QString &question, const QStringList &paragraphs, const QString &cost,
                  const QString &yes, QWidget *parent = nullptr);
};
