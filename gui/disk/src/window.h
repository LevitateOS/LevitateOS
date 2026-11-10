// The one window: look, show, hand over, look again.
#pragma once

#include "mess.h"
#include "scan.h"

#include <QMainWindow>

#include <memory>

class HandPage;
class MessPage;
class QLabel;
class QStackedWidget;
class QTimer;
class Run;
class WorkPage;

class Window : public QMainWindow {
    Q_OBJECT
public:
    explicit Window(QWidget *parent = nullptr);

    void scan();
    void showHandOver();
    void send(const QString &prompt);
    // Adds the biggest folder the rules did not name to the list, as a person
    // would by clicking it. Says which, or nothing when there is none.
    QString pickBiggestUnnamed();
    // Send what the hand-over page shows now.
    void sendShown();

    bool busy() const { return scanner_ != nullptr || run_ != nullptr; }
    const QList<Finding> &found() const { return found_; }
    qint64 scannedBytes() const { return tree_ ? tree_->bytes : 0; }
    qint64 freed() const { return freed_; }

private:
    void scanned();

    QStringList roots_;
    QString home_, agent_;
    bool apparent_;

    Scanner *scanner_ = nullptr;
    Run *run_ = nullptr;
    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    qint64 before_ = 0, freed_ = 0;

    QStackedWidget *stack_;
    MessPage *mess_;
    HandPage *hand_;
    WorkPage *work_;
    QLabel *activity_;
    QTimer *tick_;
};
