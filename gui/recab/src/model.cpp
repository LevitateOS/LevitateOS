#include "model.h"

#include <QDateTime>
#include <QJsonArray>
#include <QLocale>
#include <QObject>

QStringList Status::changeTitles() const
{
    QStringList titles;
    for (const ChangeItem &change : changes)
        if (change.kind != QLatin1String("update"))
            titles << change.title;
    return titles;
}

Status Status::fromJson(const QJsonObject &object)
{
    Status s;
    s.distro = object["distro"].toString();
    s.state = object["state"].toString();

    const QJsonObject running = object["running"].toObject();
    s.runningRelease = running["release"].toString();
    for (const QJsonValue &value : running["holds"].toArray())
        s.runningHolds << value.toString();

    const QJsonObject other = object["other"].toObject();
    s.otherRole = other["role"].toString();
    s.otherRelease = other["release"].toString();
    s.otherState = other["state"].toString();
    for (const QJsonValue &value : other["changes"].toArray()) {
        const QJsonObject change = value.toObject();
        s.changes << ChangeItem{change["id"].toString(), change["kind"].toString(), change["title"].toString()};
    }

    const QJsonObject update = object["update"].toObject();
    s.updateRelease = update["release"].toString();
    s.updateBytes = update["size_bytes"].toInteger();

    for (const QJsonValue &value : object["conflicts"].toArray()) {
        const QJsonObject clash = value.toObject();
        s.clashes << Clash{clash["path"].toString(), clash["local"].toString(), clash["release"].toString()};
    }

    const QJsonObject proposal = object["proposal"].toObject();
    s.proposalPath = proposal["path"].toString();
    s.proposalSummary = proposal["summary"].toString();
    s.proposalScope = proposal["scope"].toString();
    for (const QJsonValue &value : proposal["checks"].toArray()) {
        const QJsonObject check = value.toObject();
        s.checks << Check{check["expected"].toString(), check["verdict"].toString(), check["note"].toString()};
    }

    const QJsonObject trial = object["last_trial"].toObject();
    s.trialOutcome = trial["outcome"].toString();
    s.trialDetail = trial["detail"].toString();
    s.trialWhen = trial["when"].toString();
    s.trialLogLines = trial["log_lines"].toInt();

    s.interruptedDuring = object["interrupted"].toObject()["during"].toString();

    const QJsonObject actions = object["actions"].toObject();
    for (auto it = actions.begin(); it != actions.end(); ++it) {
        const QJsonObject verdict = it.value().toObject();
        s.actions.insert(it.key(), ActionVerdict{verdict["verdict"] == QLatin1String("ready"),
                                                 verdict["reason"].toString()});
    }
    return s;
}

// ---------------------------------------------------------------- The real recab

// What recab was doing, from the preparation's phase and operation.
static QString during(const QString &phase, const QString &operation)
{
    QString what;
    if (phase == QLatin1String("intent") || phase == QLatin1String("awaiting_synchronization"))
        what = QObject::tr("making the next system a copy of the running one");
    else if (phase == QLatin1String("staging"))
        what = QObject::tr("writing a change into the next system");
    else if (phase == QLatin1String("finalizing"))
        what = QObject::tr("finishing the next system");
    else
        what = QObject::tr("working on the next system");
    return operation.isEmpty() ? what : QObject::tr("%1 (%2)").arg(what, operation);
}

static ChangeItem changeFrom(const QJsonObject &change)
{
    const QString action = change["action"].toString(), package = change["package"].toString();
    QString title;
    if (action == QLatin1String("install"))
        title = QObject::tr("%1, installed").arg(package);
    else if (action == QLatin1String("update"))
        title = QObject::tr("%1, updated").arg(package);
    else if (action == QLatin1String("remove"))
        title = QObject::tr("%1, removed").arg(package);
    else if (action == QLatin1String("edit"))
        title = QObject::tr("%1, edited").arg(package);
    else
        title = QObject::tr("%1 (%2)").arg(package, action);
    return ChangeItem{change["id"].toString(), action == QLatin1String("edit") ? "file" : "software", title};
}

Status Status::fromReal(const QJsonObject &o, const QString &distro, const QString &release)
{
    Status s;
    s.real = true;
    s.distro = distro;
    s.runningRelease = release;
    s.runningSlot = o["active_slot"].toString();
    s.otherSlot = o["inactive_slot"].toString();
    const bool trialPending = o["trial_pending"].toBool();

    QString runningGeneration, otherGeneration, otherReadiness;
    for (const QJsonValue &value : o["slots"].toArray()) {
        const QJsonObject slot = value.toObject();
        const QString generation = slot["generation"].toObject()["id"].toString();
        if (slot["slot"].toString() == s.runningSlot) {
            runningGeneration = generation;
        } else {
            otherGeneration = generation;
            otherReadiness = slot["readiness"].toString();
        }
    }

    const QJsonObject trial = o["last_trial"].toObject();
    if (!trial.isEmpty()) {
        s.trialDetail = trial["detail"].toString();
        s.trialWhen = QLocale().toString(QDateTime::fromSecsSinceEpoch(trial["unix_time"].toInteger()), QLocale::ShortFormat);
        // Kept: the running system is the one that was tried.
        if (trial["committed"].toBool() && trial["slot"].toString() == s.runningSlot)
            s.trialOutcome = QStringLiteral("kept");
        else if (!trial["committed"].toBool())
            s.trialOutcome = QStringLiteral("failed");
    }

    const QJsonObject p = o["preparation"].toObject();
    QString phase;
    if (!p.isEmpty()) {
        phase = p["phase"].toString();
        const QString failure = p["failure"].toString();
        s.otherRole = QStringLiteral("next");
        s.otherRelease = release; // the next system continues the running one
        s.preparationId = p["id"].toString();
        s.preparationRevision = p["revision"].toInteger(-1);
        // recab's status names the change in flight and the last one finished, not all of them.
        s.changesKnown = false;
        for (const char *key : {"last_change", "change"}) {
            const QJsonObject change = p[QLatin1String(key)].toObject();
            if (!change.isEmpty())
                s.changes << changeFrom(change);
        }
        if (phase == QLatin1String("ready")) {
            s.otherState = QStringLiteral("ready");
            s.state = trialPending ? QStringLiteral("trial_pending") : QStringLiteral("ready");
        } else if (phase == QLatin1String("prepared") && failure.isEmpty()) {
            s.otherState = QStringLiteral("open");
            s.state = QStringLiteral("preparing");
        } else {
            // Any other phase with no command running is where one stopped.
            s.otherState = QStringLiteral("open");
            s.state = QStringLiteral("interrupted");
            s.interruptedDuring = during(phase, p["operation"].toString());
            if (!failure.isEmpty())
                s.interruptedDuring += QObject::tr(": %1").arg(failure);
        }
    } else if (trialPending) {
        // A restart into the other system is set; it is finished and waits.
        s.otherRole = QStringLiteral("next");
        s.otherRelease = release;
        s.otherState = QStringLiteral("ready");
        s.state = QStringLiteral("trial_pending");
    } else {
        s.state = QStringLiteral("stable");
        if (o["rollback_slot"].toString() == s.otherSlot)
            s.otherRole = QStringLiteral("previous");
        else if (!otherGeneration.isEmpty() && otherGeneration == runningGeneration)
            s.otherRole = QStringLiteral("copy");
        else
            s.otherRole = QStringLiteral("none");
    }
    // The next system was tried and the computer came back: it is still
    // there, finished, and not the way back.
    if (s.trialOutcome == QLatin1String("failed") && trial["slot"].toString() == s.otherSlot
        && s.otherRole == QLatin1String("next") && s.otherState == QLatin1String("ready") && !trialPending) {
        s.state = QStringLiteral("failed");
        s.otherState = QStringLiteral("failed");
    }

    // recab's verdict on each of its actions, under the names this window asks by.
    QMap<QString, QJsonObject> by;
    for (const QJsonValue &value : o["actions"].toArray())
        by.insert(value.toObject()["action"].toString(), value.toObject());
    auto verdict = [&by](const QString &action) {
        const QJsonObject view = by.value(action);
        const QString said = view["verdict"].toString();
        QString reason = view["reason"].toString();
        if (reason.isEmpty())
            reason = said == QLatin1String("no_change") ? QObject::tr("There is nothing to do.")
                                                         : QObject::tr("recab did not say whether this can be done.");
        return ActionVerdict{said == QLatin1String("ready"), reason};
    };
    const bool open = s.otherRole == QLatin1String("next") && s.state == QLatin1String("preparing");
    const ActionVerdict prepare = verdict("prepare");
    s.actions.insert("finalize", verdict("finalize"));
    s.actions.insert("discard", verdict("discard"));
    s.actions.insert("go_back", verdict("rollback"));
    s.actions.insert("restart", verdict("set_next"));
    // A change goes into an open next system: the one there is, or one recab can prepare.
    s.actions.insert("change", open ? ActionVerdict{true, QString()} : prepare);
    // Continuing is asking recab to prepare again; it carries on from what is really there.
    s.actions.insert("recover", s.state == QLatin1String("interrupted")
                                    ? ActionVerdict{true, QString()}
                                    : ActionVerdict{false, QObject::tr("Nothing was interrupted.")});
    s.actions.insert("update", ActionVerdict{false, QObject::tr("recab takes a new release from release media; "
                                                                "this window cannot add one yet.")});
    return s;
}
