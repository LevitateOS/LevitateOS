// What a coding agent has to do for the apps. Codex is the first; another
// agent is another class beside it, picked in main.cpp.
#pragma once

#include "session.h"

#include <QObject>

class Adapter : public QObject {
public:
    explicit Adapter(Session *session) : QObject(session), session_(session) {}
    // The apps' request; the first thing the person asks.
    virtual void begin(const Request &request) = 0;
    // The person's answer to a `question` event.
    virtual void answered(const QString &id, const QString &answer, const QByteArray &line) = 0;
    // Nobody will read or answer any more.
    virtual void inputClosed() {}
    // End everything this started, now, and wait until it has ended.
    virtual void stop() = 0;

protected:
    Session *session_;
};
