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

// A thing the selected release and the next system disagree about, as recab's
// merge lists it (`Conflict` in tools/recab/src/merge.rs). `kind` is one of
// content_or_metadata, type, deletion, directory_tree, package_ownership,
// package_rebuild, missing_or_unsafe_baseline. `path` is a path of the system
// without its leading slash, except for package_rebuild, where it names the
// package. recab says nothing about what either side holds. `resolution` is
// empty until it is settled (local, release or edited).
struct MergeConflict {
    QString id, path, kind, resolution;
    // Whether recab settles it by choosing a side (`resolve`). The other two
    // are not settled that way: a package is reproduced, a baseline adopted.
    bool choosesASide() const;
    bool isPackageRebuild() const { return kind == QLatin1String("package_rebuild"); }
    bool isBaseline() const { return kind == QLatin1String("missing_or_unsafe_baseline"); }
    // A few words for the kind, and a sentence of what recab says of it, for
    // the release being added and the release the running system shipped.
    QString kindWords() const;
    QString says(const QString &release, const QString &running) const;
    // How it stands: "Not settled" or what was chosen.
    QString standing() const;
};

// What recab says about a new release: `updates` of its status
// (tools/recab/src/update.rs). Nothing here is worked out by the window.
struct UpdateInfo {
    // recab's own word: idle, checking, up_to_date, available, check_failed,
    // downloading, downloaded, interrupted, awaiting_merge, or the merge's
    // phase (conflicted, validated, applying, complete, blocked).
    QString phase, lastError;
    // The release found by the last check (version empty: none known).
    QString version, channel, id;
    qint64 sequence = 0, totalBytes = 0;
    // What was fetched of it so far.
    bool hasDownload = false, downloaded = false;
    qint64 verifiedBytes = 0, downloadBytes = 0;
    // Chosen to be merged into the next system, and how that merge stands.
    QString pendingRelease;
    bool hasMerge = false, mergeComplete = false;
    QString mergePhase, mergeFailure;
    qint64 unresolved = 0;
    QList<MergeConflict> conflicts;
    bool available() const { return !version.isEmpty(); }
    // The merge stopped on something that needs the person: conflicts left
    // unresolved, or blocked before it began (phase "blocked").
    bool stopped() const { return hasMerge && !mergeComplete && (unresolved > 0 || mergePhase == QLatin1String("blocked")); }
    // The conflicts not settled yet.
    QList<MergeConflict> open() const;
};

struct Status {
    UpdateInfo update;
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
    // recab's own operation by its id (`update_check`, `update_apply`, ...):
    // its verdict and reason, or "not offered" when this recab has none.
    ActionVerdict operation(const QString &id) const;
    QJsonObject operationObject(const QString &id) const;
    // The changes a person made, without the update (the release name says that).
    QStringList changeTitles() const;
    // The release the merge is about, as a name ("LevitateOS 2026.12"); empty
    // when recab names none.
    QString mergeRelease() const { return otherRelease.isEmpty() ? QString() : name(otherRelease); }
    // What the next system's pages say when the merge stopped (the real
    // recab): a heading and a sentence that hold for every kind of conflict.
    QString mergeHeading() const;
    QString mergeSentence() const;
    // The older stand-in's status (schema 3): the lifecycle, spoken directly.
    static Status fromJson(const QJsonObject &object);
    // The real recab's status (schemas 7 to 10; `operations` and `updates` are
    // read when the status has them, which schema 10 does, and the older
    // schemas keep the limited boot and preparation mapping). It names no
    // release for the running system, so its name comes from the os-release
    // the window is told of.
    static Status fromReal(const QJsonObject &object, const QString &distro, const QString &release);
};

// What `recab history --json` lists (tools/recab/src/history.rs), each entry
// put in words. Newest first, as recab sends it.
struct HistoryRow {
    enum Mood { Plain, Warn, Ok, Bad };
    qint64 when = 0;
    QString kind, what, who, ended, detail, slot, preparation, actor;
    Mood mood = Plain;
    bool restart = false;      // a restart, its keeping or return, going back
    bool byAgent = false;
    // Whom recab names for it: only a change has an actor.
    bool carriesActor = false;
};

struct History {
    // False when this recab has no `history` or it could not be read: `problem`
    // says which, and `unknown` is set when the command does not exist.
    bool read = false, unknown = false;
    QString problem;
    QList<HistoryRow> rows;
    qint64 unreadable = 0;
    bool truncated = false, logFound = true;
    // `openPreparation`: the preparation the status holds now, which tells a
    // change that is still in the next system from one that is not.
    static History fromJson(const QJsonObject &object, const QString &distro, const QString &openPreparation);
    // The page as plain text, for saving.
    QString text() const;
};

// "Today 13:52", "9 Nov 18:04".
QString whenWords(qint64 unixTime);
