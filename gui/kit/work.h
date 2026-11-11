#pragma once

#include "page.h"

#include <QMap>

class QHBoxLayout;
class QLabel;
class QPlainTextEdit;
class QProgressBar;
class QPushButton;
class Run;
class StepMark;

// Something long is being done: its steps, how far it is, its output. It reads
// the events of one Run (plan, step, progress, say, agent, question). When
// the agent asks something, the question is shown with its choices and the
// answer goes back to the agent.
class WorkPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // `stopNote` says what stopping means; by default, that what is done is kept.
    void begin(const QString &title, const QString &subtitle, Run *run, const QString &stopNote = QString());
    // Whether the person pressed Stop on the run shown.
    bool stopped() const;
    // Whether the run ended and the page says why (it stopped, or it carries on elsewhere).
    bool saysWhy() const { return saysWhy_; }
    // Whether the agent has asked something and waits for the answer.
    bool asking() const { return !question_.isEmpty(); }
    QStringList choices() const { return choices_; }
    // Answers what the agent asked, as a click on a choice would.
    void answer(const QString &words);
signals:
    void done(bool ok);
    void back();
    void activity(const QString &words);
private:
    void onEvent(const QJsonObject &event);
    void ask(const QJsonObject &event);
    void onFinished(bool ok);
    void onCarriesOn();
    // The question was answered from somewhere else (Activity): nothing is waiting any more.
    void answeredElsewhere();
    void showVerdict(const QString &title, const QString &words);

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
    QString question_; // the id of what the agent asked, while it waits
    QStringList choices_;
    QWidget *asked_ = nullptr;
    bool stoppedElsewhere_ = false; // the run ended with the helper's "stopped", not by our Stop
    bool saysWhy_ = false;
};
