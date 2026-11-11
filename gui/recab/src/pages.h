// The pages of the window. Each one is told what to show and rebuilds itself;
// it asks for actions with signals and never runs anything itself.
#pragma once

#include "model.h"
#include "page.h"
#include "work.h"

#include <QDialog>
#include <QMap>
#include <QWidget>

class PromptPanel;
class QLabel;
class QPushButton;
class QVBoxLayout;

// The one page a person sees most: the running system, what is beside it, and
// what can be done. Its shape follows recab's state.
class SystemPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
    // `asAdministrator`: the read failed for lack of rights, so offer to
    // read with the administrator's password.
    void showProblem(const QString &problem, bool asAdministrator = false);
signals:
    void operation(const QJsonObject &operation);
    void ask();
    void addUpdate();
    void goBack();
    void keep();
    void restartLater();
    void cancelRestart();
    void finish();
    void restart();
    void discard();
    // Give up every change in the next system and make it an exact copy of
    // the running system again.
    void reset();
    void recover();
    void settle();
    void fix(const QString &prompt);
    void retry();
    void readAgain();
    void readAsAdministrator();
    // The ways to the two pages that hold the rest.
    void openNext();
    void openBack();
};

// Everything about the next system in one place: what it holds, what will
// differ after the restart, and the three steps to get it started.
class NextPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
signals:
    void operation(const QJsonObject &operation);
    void ask();
    void installSoftware();
    void reset();
    void settle();
    void finish();
    void restart();
    void restartLater();
    void cancelRestart();
    void recover();
    void discard();
    void keep();
    void goBack();
    void openDisk();
    // What to do about a next system that did not start is on Overview.
    void openOverview();
};

// What can be turned back from here, and what to do when the desktop does not
// start. The steps for the second are also given as plain text, to be saved.
class BackPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Status &status);
signals:
    void goBack();
    void keep();
    void cancelRestart();
    void saveSteps();
};

// The steps for a desktop that does not start, as plain text with one step to
// a paragraph; only what recab's own source and status confirm.
QString backStepsText(const Status &status);

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
