// An agent that already speaks the apps' protocol: its words go through
// unchanged. Only the record is ours.
#pragma once

#include "adapter.h"

class Child;

class Passthrough : public Adapter {
public:
    Passthrough(Session *session, const QString &program);
    void begin(const Request &request) override;
    void answered(const QString &id, const QString &answer, const QByteArray &line) override;
    void inputClosed() override;
    void stop() override;

private:
    QString program_;
    Child *child_ = nullptr;
    QByteArray buffer_;
};
