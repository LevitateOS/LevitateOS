// Signals turned into something the event loop can read. A handler may do
// very little; this one writes the number of the signal into a pipe.
#pragma once

#include <QObject>

#include <initializer_list>

class QSocketNotifier;

class SigWatch : public QObject {
    Q_OBJECT
public:
    // Only one may exist at a time in a process.
    SigWatch(std::initializer_list<int> numbers, QObject *parent = nullptr);

signals:
    void raised(int number);

private:
    QSocketNotifier *notifier_ = nullptr;
};
