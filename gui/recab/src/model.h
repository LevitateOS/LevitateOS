// What the front end knows about the two systems: plain data read from
// `recab status --json`. No lifecycle rules live here; recab's verdicts say
// what may be done.
#pragma once

#include "words.h"

#include <QJsonObject>
#include <QList>
#include <QMap>
#include <QString>
#include <QStringList>

struct ChangeItem {
    QString id, kind, title; // kind: software, file, update, other
};

struct ActionVerdict {
    bool ready = false;
    QString reason; // recab's own words, when it would refuse
};

struct Clash {
    QString path, local, release;
};

struct Check {
    QString expected, verdict, note; // verdict: yes, no, unknown
};

struct Status {
    QString distro;
    // stable, preparing, ready, trial_pending, failed, interrupted
    QString state;

    QString runningRelease;
    QStringList runningHolds; // changes in effect in the running system

    // The place beside the running system: "previous" (the system from
    // before), "next" (being prepared), or "none".
    QString otherRole, otherRelease, otherState;
    QList<ChangeItem> changes;

    QString updateRelease; // empty: no newer release is known
    qint64 updateBytes = 0;

    QList<Clash> clashes;
    QString proposalPath, proposalSummary, proposalScope; // what an agent did about a clash
    QList<Check> checks;

    QString trialOutcome, trialDetail, trialWhen; // the last restart into a next system
    int trialLogLines = 0;
    QString interruptedDuring;

    QMap<QString, ActionVerdict> actions;

    QString name(const QString &release) const { return distro + QLatin1Char(' ') + release; }
    ActionVerdict can(const QString &action) const { return actions.value(action); }
    // The changes a person made, without the update (the release name says that).
    QStringList changeTitles() const;
    static Status fromJson(const QJsonObject &object);
};
