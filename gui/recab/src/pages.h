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
    void readAgain();
    void readAsAdministrator();
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
