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

// recab titles each entry "<NAME of the system's os-release> (Slot A)"
// (tools/recab/src/systemd_boot.rs, where it publishes a slot). Status does
// not carry the title, so the name is put together from the same parts.
QString Status::menuName(const QString &slot) const
{
    return real && !slot.isEmpty() && !distro.isEmpty() ? QObject::tr("%1 (Slot %2)").arg(distro, slot) : QString();
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
        ChangeItem item;
        item.id = change["id"].toString();
        item.kind = change["kind"].toString();
        item.title = change["title"].toString();
        item.what = item.title;
        item.package = item.title;
        s.changes << item;
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
    ChangeItem item;
    item.id = change["id"].toString();
    item.kind = action == QLatin1String("edit") ? "file" : "software";
    item.title = title;
    item.what = title;
    item.action = action;
    item.package = package;
    item.actor = change["actor"].toString();
    item.when = change["unix_time"].toInteger();
    return item;
}

Status Status::fromReal(const QJsonObject &o, const QString &distro, const QString &release)
{
    Status s;
    s.real = true;
    s.operations=o["operations"].toArray();
    s.rebootNeeded=o["lifecycle"].toObject()["reboot_needed"].toBool();
    s.distro = distro;
    s.runningRelease = o["running_release"].toString(release);
    s.runningSlot = o["active_slot"].toString();
    s.otherSlot = o["inactive_slot"].toString();

    const QJsonObject trial = o["last_trial"].toObject();
    if (!trial.isEmpty() && (o["schema_version"].toInt()<10 || o["lifecycle"].toObject()["trial_current"].toBool())) {
        s.trialDetail = trial["detail"].toString();
        s.trialWhen = QLocale().toString(QDateTime::fromSecsSinceEpoch(trial["unix_time"].toInteger()), QLocale::ShortFormat);
        // Kept: the running system is the one that was tried.
        if (trial["committed"].toBool() && trial["slot"].toString() == s.runningSlot)
            s.trialOutcome = QStringLiteral("kept");
        else if (!trial["committed"].toBool())
            s.trialOutcome = QStringLiteral("failed");
        else
            s.trialOutcome = QStringLiteral("other");
    }

    // recab says where things stand itself (status schema 7); nothing here
    // works it out again from slots and phases.
    s.entryPrefix = o["entry_prefix"].toString();
    for (const QJsonValue &value : o["slots"].toArray()) {
        const QJsonObject slot = value.toObject();
        const bool running = slot["slot"].toString() == s.runningSlot;
        (running ? s.runningEntry : s.otherEntry) = slot["entry"].toString();
        (running ? s.runningDevice : s.otherDevice) = slot["device"].toString();
    }
    const QJsonObject lifecycle = o["lifecycle"].toObject();
    static const QMap<QString, QString> states{
        {"stable", "stable"},           {"open", "preparing"}, {"working", "working"},
        {"interrupted", "interrupted"}, {"ready", "ready"},    {"restart_set", "trial_pending"},
        {"conflicted", "preparing"}, {"on_trial", "on_trial"},       {"trial_failed", "failed"}};
    const QString said = lifecycle["state"].toString();
    s.state = states.value(said);
    if (s.state.isEmpty()) {
        const QString reason = o["recovery_reason"].toString();
        s.unusable = said == QLatin1String("unusable") && !reason.isEmpty()
                         ? reason
                         : QObject::tr("recab reports the state \"%1\", which this window does not know").arg(said);
    }
    if(said=="unusable"){s.state="interrupted";s.unusable=o["recovery_reason"].toString();s.interruptedDuring=s.unusable;}
    const QString other = lifecycle["other"].toString();
    s.otherRole = other == QLatin1String("unknown") || other.isEmpty() ? QStringLiteral("none") : other;
    // The next system continues the running one, and a copy is it.
    if (s.otherRole == QLatin1String("next") || s.otherRole == QLatin1String("copy"))
        s.otherRelease = o["prepared_release"].toString(release);

    const QJsonObject p = o["preparation"].toObject();
    if (!p.isEmpty()) {
        s.preparationId = p["id"].toString();
        s.preparationRevision = p["revision"].toInteger(-1);
        if(p["slot"].toString()==s.otherSlot)
            s.otherState = p["phase"] == QLatin1String("ready") ? QStringLiteral("ready") : QStringLiteral("open");
        // Every finished change, oldest first, with who asked for it.
        for (const QJsonValue &value : p["changes"].toArray()) {
            ChangeItem item = changeFrom(value.toObject());
            if (item.actor.startsWith(QLatin1String("agent")))
                item.title = QObject::tr("%1, by the agent").arg(item.title);
            s.changes << item;
        }
        // A command that runs between two steps has left no word of what it does.
        const bool between = p["phase"] == QLatin1String("prepared") || p["phase"] == QLatin1String("ready");
        if (s.state == QLatin1String("interrupted") || (s.state == QLatin1String("working") && !between)) {
            s.interruptedDuring = during(p["phase"].toString(), p["operation"].toString());
            const QString failure = p["failure"].toString();
            if (!failure.isEmpty() && s.state == QLatin1String("interrupted"))
                s.interruptedDuring += QObject::tr(": %1").arg(failure);
        }
    } else if (s.otherRole == QLatin1String("next")) {
        // A restart is set and no preparation is left: it is finished and waits.
        s.otherState = QStringLiteral("ready");
    }
    // Tried, and the computer came back: still there, and not the way back.
    if (s.state == QLatin1String("failed"))
        s.otherState = QStringLiteral("failed");

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
    s.actions.insert("keep", verdict("commit"));
    // Once the restart is set, only the restart itself is left to do.
    s.actions.insert("restart", s.state == QLatin1String("trial_pending") ? ActionVerdict{true, QString()}
                                                                          : verdict("set_next"));
    // A change goes into an open next system: the one there is, or one recab can prepare.
    s.actions.insert("change", open ? ActionVerdict{true, QString()} : prepare);
    // Continuing is asking recab to prepare again; it carries on from what is really there.
    s.actions.insert("recover", s.state == QLatin1String("interrupted")
                                    ? ActionVerdict{true, QString()}
                                    : ActionVerdict{false, QObject::tr("Nothing was interrupted.")});
    s.actions.insert("update", ActionVerdict{false, QObject::tr("recab takes a new release from release media; "
                                                                "this window cannot add one yet.")});
    if (!s.operations.isEmpty()) {
        const QJsonObject update=o["updates"].toObject();
        s.updatePhase=update["phase"].toString();s.updateError=update["last_error"].toString();
        s.updateRelease=update["available"].toObject()["version"].toString();
        s.updateBytes=update["available"].toObject()["total_bytes"].toInteger();
        const QJsonObject download=update["download"].toObject();
        if(!download.isEmpty())s.downloadProgress=QObject::tr("%1 / %2 verified bytes%3").arg(download["verified_bytes"].toInteger()).arg(download["total_bytes"].toInteger()).arg(download["downloaded"].toBool()?QObject::tr(" — downloaded"):QString());
        for(const QJsonValue &value:update["merge"].toObject()["conflicts"].toArray()) {const auto c=value.toObject();if(c["resolution"].isNull())s.clashes<<Clash{c["path"].toString()+" ("+c["kind"].toString()+")",QObject::tr("Prepared content (private)"),QObject::tr("Release content (private)")};}
        auto own=[&s](const QString &id){ for(const auto &v:s.operations){const auto op=v.toObject();if(op["id"]==id)return ActionVerdict{op["verdict"]=="ready",op["reason"].toString()};}return ActionVerdict{false,QObject::tr("recab did not offer this operation")};};
        s.actions["finalize"]=own("finalize");s.actions["discard"]=own("discard");s.actions["go_back"]=own("rollback");s.actions["keep"]=own("commit");s.actions["restart"]=s.state=="trial_pending"?ActionVerdict{true,QString()}:own("set_next");
        s.actions["change"]=own("install");s.actions["recover"]=own("update_recover");
        for(const auto &id:{"trial_recover","change_recover","finalize_recover","sync_recover"})if(own(id).ready)s.actions["recover"]=own(id);
        s.actions["update"]=own("update_apply");if(!s.actions["update"].ready)s.actions["update"]=own("update_download");if(!s.actions["update"].ready)s.actions["update"]=own("update_check");
    }
    return s;
}
