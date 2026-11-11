// What the front end knows about the two systems: plain data read from
// `recab status --json`. No lifecycle rules live here; recab's verdicts say
// what may be done.
#pragma once

#include "words.h"

#include <QJsonObject>
#include <QJsonArray>
#include <QList>
#include <QMap>
#include <QString>
#include <QStringList>

struct ChangeItem {
    QString id, kind, title; // kind: software, file, update, other
    // What the next system's page lists. Only the real recab says more than
    // a title: `what` is the title without "by the agent" (the page has a
    // column for who), `action` and `package` are recab's own words for it
    // (install, update, remove, edit; the package or the edited path), `actor`
    // is the label the caller gave (empty when none) and `when` the unix time
    // (0 when recab gives none).
    QString what, action, package, actor;
    qint64 when = 0;
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
    QJsonArray operations;
    QString updatePhase, updateError, downloadProgress;
    bool rebootNeeded = false;
    // Read from the real recab, which speaks of two slots and a preparation,
    // not from the stand-in, which speaks of the lifecycle directly.
    bool real = false;
    QString runningSlot, otherSlot;            // real: "A" or "B"
    QString preparationId;                     // real: the open preparation, if any
    qint64 preparationRevision = -1;

    QString distro;
    // stable, preparing, working (a command runs now), interrupted, ready,
    // trial_pending (a restart is set), on_trial (running, not kept yet),
    // failed (the trial came back)
    QString state;
    // recab says the system needs recovery before anything else: its reason.
    QString unusable;

    QString runningRelease;
    QStringList runningHolds; // changes in effect in the running system

    // The place beside the running system: "previous" (the system from
    // before), "next" (being prepared), "copy" (the same system, kept in
    // step) or "none".
    QString otherRole, otherRelease, otherState;
    QList<ChangeItem> changes;

    QString updateRelease; // empty: no newer release is known
    qint64 updateBytes = 0;

    QList<Clash> clashes;
    QString proposalPath, proposalSummary, proposalScope; // what an agent did about a clash
    QList<Check> checks;

    // The last restart into a next system. Outcome: "kept" (the running
    // system is the one tried), "failed" (the computer came back), "other"
    // (kept, but it was the other system that was tried).
    QString trialOutcome, trialDetail, trialWhen;
    int trialLogLines = 0;
    QString interruptedDuring;

    QMap<QString, ActionVerdict> actions;

    // What recab says of the two places (real recab only): the entry file
    // each one starts from, relative to the ESP, and its block device.
    QString entryPrefix, runningEntry, otherEntry, runningDevice, otherDevice;
    // How the start-up menu lists a system, as recab writes it. Empty when
    // recab does not say.
    QString menuName(const QString &slot) const;

    QString name(const QString &release) const { return (distro + QLatin1Char(' ') + release).trimmed(); }
    ActionVerdict can(const QString &action) const { return actions.value(action); }
    // The changes a person made, without the update (the release name says that).
    QStringList changeTitles() const;
    // The stand-in's status (schema 3).
    static Status fromJson(const QJsonObject &object);
    // The real recab's status (schema 6). It names no release, so the running
    // system's name comes from its os-release.
    static Status fromReal(const QJsonObject &object, const QString &distro, const QString &release);
};
