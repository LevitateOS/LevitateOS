// What a coding agent has to do for the apps. Codex and Claude Code each have
// a class beside this one; an agent that already speaks the apps' protocol is
// passed through. Which one is used is picked in main.cpp.
#pragma once

#include "session.h"

#include <QObject>

class Adapter : public QObject {
public:
    explicit Adapter(Session *session) : QObject(session), session_(session) {}
    // The apps' request; the first thing the person asks.
    virtual void begin(const Request &request) = 0;
    // The person's answer to a `question` event. The session has already
    // checked that the question is the one that waits.
    virtual void answered(const QString &id, const QString &answer, const QByteArray &line) = 0;
    // End everything this started, now, and wait until it has ended.
    virtual void stop() = 0;

protected:
    Session *session_;
};
