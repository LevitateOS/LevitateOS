// The helper a window starts, and the one a window attaches with. It does not
// run anything: it relays between the window (its standard input and error)
// and a run, which lives in a process of its own (see session.h) and goes on
// when this one ends.
#pragma once

#include "settings.h"

#include <QObject>

class Link;
class QSocketNotifier;
class QTimer;
class SigWatch;

class Client : public QObject {
    Q_OBJECT
public:
    // Relay: the first input line is the request; the run is started for it.
    // Attach: follows the run that has this id.
    enum Mode { Relay, Attach };
    Client(const Settings &settings, Mode mode, const QString &id, QObject *parent = nullptr);
    void run();

private:
    void onInput();
    void onSignal(int number);
    void startRun(const QByteArray &requestLine);
    void tryConnect();
    void connected(int fd);
    void onHostLine(const QByteArray &line);
    void onClosed();
    void fromRecord();
    void toWindow(int fd, const QByteArray &bytes);
    void failed(const QString &reason);
    void leave(int code);

    const Settings &settings_;
    const Mode mode_;
    QString id_;
    SigWatch *signals_ = nullptr;
    QSocketNotifier *input_ = nullptr;
    QTimer *retry_ = nullptr, *patience_ = nullptr;
    Link *link_ = nullptr;
    QByteArray pending_;
    QList<QByteArray> queued_; // answers that came before the run could be reached
    qint64 host_ = 0;
    QString question_;
    int attempts_ = 0;
    bool started_ = false, stdinClosed_ = false, stopping_ = false, leaving_ = false;
};

// `--stop ID`: ends a run that is going. Returns the exit code.
int stopRun(const Settings &settings, const QString &id);
