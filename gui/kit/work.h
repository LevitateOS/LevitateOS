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
// the events of one Run (plan, step, progress, say, agent).
class WorkPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // `stopNote` says what stopping means; by default, that what is done is kept.
    void begin(const QString &title, const QString &subtitle, Run *run, const QString &stopNote = QString());
    // Whether the person pressed Stop on the run shown.
    bool stopped() const;
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
