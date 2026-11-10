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
    // `asRoot`: through the root helper, when the person cannot read the
    // boot entries themselves.
    Run *read(bool asRoot = false);
    // Reads the answer: the stand-in's lifecycle (schema 3) or the real
    // recab's slots and preparation (schema 6). `permission` is set when the
    // read failed for lack of rights.
    bool understand(Run *finished, bool ok, Status *into, QString *error, bool *permission = nullptr);

    // One change, named as this window names it (`finalize`, `discard`,
    // `recover`, `prepare`, `set-next`, `rollback`, and the stand-in's
    // `update apply` and `resolve`). For the stand-in that is the command
    // itself, with `--machine-events`; for the real recab it becomes recab's
    // own command, with the open preparation's id and revision.
    Run *change(const QStringList &arguments);
    // The agent, with the prompt (issue, facts, expected result) on its input
    // as one JSON line; its questions are answered on the same input.
    Run *agent(const QString &prompt);
    Run *reboot();

    QString agentName() const;

private:
    QStringList realArguments(const QStringList &arguments) const;
    QString program_, rootHelper_, agent_, reboot_, osRelease_;
    // From the last status understood: which recab answers, and what its
    // commands need to be told.
    bool real_ = false;
    QString preparationId_, otherSlot_;
    qint64 preparationRevision_ = -1;
};
