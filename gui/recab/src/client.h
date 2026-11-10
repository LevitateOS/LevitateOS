// The only place that runs `recab`, the agent and the restart. Reading goes
// through as the desktop user; changes go through the root helper (pkexec),
// which shows the desktop's own password prompt.
#pragma once

#include "model.h"
#include "run.h"

#include <QObject>

class RecabClient : public QObject {
    Q_OBJECT
public:
    explicit RecabClient(QObject *parent = nullptr);

    // `recab status --json`, as a run that does not hold the window up; its
    // answer is read with `understand`.
    Run *read();
    bool understand(Run *finished, bool ok, Status *into, QString *error) const;

    // `recab <arguments> --machine-events`.
    Run *change(const QStringList &arguments);
    // The agent, with the prompt (issue, facts, expected result) on its input
    // as one JSON line; its questions are answered on the same input.
    Run *agent(const QString &prompt);
    Run *reboot();

    QString agentName() const;

private:
    QString program_, rootHelper_, agent_, reboot_;
};
