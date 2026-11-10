// When a window should read its state again: when the person asks for it
// (F5 or Ctrl+R), and when they come back to the window after a while, since
// an agent or a terminal may have changed things in the meantime.
#pragma once

#include <QElapsedTimer>
#include <QObject>

class QWidget;

class Refresher : public QObject {
    Q_OBJECT
public:
    // `afterSeconds`: coming back to the window asks for a refresh only when
    // the state is at least this old; 0 never asks on coming back.
    Refresher(QWidget *window, int afterSeconds);
    // The state was just read.
    void fresh() { age_.restart(); }

signals:
    // `byHand` is true when the person pressed the key.
    void asked(bool byHand);

protected:
    bool eventFilter(QObject *watched, QEvent *event) override;

private:
    QWidget *window_;
    int after_;
    QElapsedTimer age_;
};
