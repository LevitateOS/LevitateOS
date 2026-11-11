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
    s.update.version = s.updateRelease;
    s.update.totalBytes = s.updateBytes;

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
    // On trial, the other place holds the system from before: that is what
    // `rollback` leads to. recab still calls it "next" then, because the
    // preparation that made the running system is kept until it is kept.
    if (s.state == QLatin1String("on_trial"))
        s.otherRole = QStringLiteral("previous");
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
        UpdateInfo &u = s.update;
        u.phase = s.updatePhase;
        u.lastError = s.updateError;
        const QJsonObject available = update["available"].toObject();
        u.version = available["version"].toString();
        u.channel = available["channel"].toString();
        u.id = available["id"].toString();
        u.sequence = available["sequence"].toInteger();
        u.totalBytes = available["total_bytes"].toInteger();
        const QJsonObject fetched = update["download"].toObject();
        u.hasDownload = !fetched.isEmpty();
        u.downloaded = fetched["downloaded"].toBool();
        u.verifiedBytes = fetched["verified_bytes"].toInteger();
        u.downloadBytes = fetched["total_bytes"].toInteger();
        u.pendingRelease = update["pending_release"].toString();
        const QJsonObject merge = update["merge"].toObject();
        u.hasMerge = !merge.isEmpty();
        u.mergePhase = merge["phase"].toString();
        u.mergeComplete = update["merge_complete"].toBool();
        u.mergeFailure = merge["failure"].toString();
        u.unresolved = merge["unresolved"].toInteger();
        for (const QJsonValue &value : merge["conflicts"].toArray()) {
            const QJsonObject c = value.toObject();
            u.conflicts << MergeConflict{c["id"].toString(), c["path"].toString(), c["kind"].toString(), c["resolution"].toString()};
        }
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

ActionVerdict Status::operation(const QString &id) const
{
    const QJsonObject op = operationObject(id);
    if (op.isEmpty())
        return ActionVerdict{false, QObject::tr("This recab does not offer it.")};
    return ActionVerdict{op["verdict"] == QLatin1String("ready"), op["reason"].toString()};
}

QJsonObject Status::operationObject(const QString &id) const
{
    for (const QJsonValue &value : operations)
        if (value.toObject()["id"] == id)
            return value.toObject();
    return {};
}

// ---------------------------------------------------------------- History

QString whenWords(qint64 unixTime)
{
    const QDateTime time = QDateTime::fromSecsSinceEpoch(unixTime);
    const QLocale locale;
    if (time.date() == QDate::currentDate())
        return QObject::tr("Today %1").arg(locale.toString(time.time(), QLocale::ShortFormat));
    return QObject::tr("%1 %2").arg(locale.toString(time.date(), QStringLiteral("d MMM")),
                                    locale.toString(time.time(), QLocale::ShortFormat));
}

namespace {

QString slotWords(const QString &slot)
{
    return slot.isEmpty() ? QString() : QObject::tr(" (Slot %1)").arg(slot);
}

QString changeWords(const QString &action, const QString &package)
{
    if (action == QLatin1String("install"))
        return QObject::tr("%1, installed").arg(package);
    if (action == QLatin1String("update"))
        return QObject::tr("%1, updated").arg(package);
    if (action == QLatin1String("remove"))
        return QObject::tr("%1, removed").arg(package);
    if (action == QLatin1String("edit"))
        return QObject::tr("%1, edited").arg(package);
    return QObject::tr("%1 (%2)").arg(package, action);
}

QString whoWords(const QString &actor)
{
    if (actor.isEmpty())
        return QObject::tr("recab names nobody");
    if (actor == QLatin1String("person"))
        return QObject::tr("You");
    if (actor.startsWith(QLatin1String("agent")))
        return QObject::tr("The agent");
    return actor;
}

} // namespace

History History::fromJson(const QJsonObject &o, const QString &distro, const QString &openPreparation)
{
    History h;
    h.read = true;
    h.unreadable = o["unreadable"].toInteger();
    h.truncated = o["truncated"].toBool();
    h.logFound = o["log_found"].toBool(true);
    Q_UNUSED(distro)
    const QJsonArray entries = o["entries"].toArray();
    auto at = [&entries](int i) { return entries.at(i).toObject(); };
    const int n = entries.size();
    for (int i = 0; i < n; ++i) {
        const QJsonObject e = at(i);
        HistoryRow row;
        row.when = e["time"].toInteger();
        row.kind = e["kind"].toString();
        row.slot = e["slot"].toString();
        row.preparation = e["preparation"].toString();
        row.actor = e["actor"].toString();
        row.detail = e["detail"].toString();
        const QString outcome = e["outcome"].toString();
        const QString kind = row.kind;
        row.restart = kind == QLatin1String("set-next") || kind.startsWith(QLatin1String("trial-"))
                      || kind == QLatin1String("rollback") || kind == QLatin1String("commit");
        if (kind == QLatin1String("change")) {
            row.carriesActor = true;
            row.byAgent = row.actor.startsWith(QLatin1String("agent"));
            row.what = changeWords(e["action"].toString(), e["package"].toString());
            row.who = whoWords(row.actor);
            // How it ended: what happened to the next system it went into,
            // as far as the log shows it.
            const QString slot = row.slot, preparation = row.preparation;
            int finished = -1;
            bool discarded = false;
            for (int j = 0; j < n; ++j) {
                const QJsonObject other = at(j);
                if (other["preparation"].toString() != preparation)
                    continue;
                if (other["kind"] == QLatin1String("discard") && other["outcome"] == QLatin1String("done"))
                    discarded = true;
                if (other["kind"] == QLatin1String("finalize") && other["outcome"] == QLatin1String("done") && finished < 0)
                    finished = j;
            }
            if (discarded) {
                row.ended = QObject::tr("Given up with that next system");
            } else if (finished >= 0) {
                // How the restarts into that place went after it was finished,
                // the last one counting, until it was begun again.
                QString verdict;
                for (int j = finished - 1; j >= 0; --j) {
                    const QJsonObject later = at(j);
                    if (later["slot"].toString() != slot)
                        continue;
                    const QString k = later["kind"].toString();
                    // A commit of that place, once it runs, is the system kept by
                    // hand; one that cancelled a set restart is not.
                    if (k == QLatin1String("commit") && later["outcome"] != QLatin1String("restart_cancelled"))
                        verdict = QStringLiteral("trial-kept");
                    else if (k == QLatin1String("trial-kept") || k == QLatin1String("trial-failed"))
                        verdict = k;
                    else if (k == QLatin1String("prepare") || k == QLatin1String("discard"))
                        break;
                }
                if (verdict == QLatin1String("trial-kept")) {
                    row.ended = QObject::tr("Went into a restart that was kept");
                    row.mood = HistoryRow::Ok;
                } else if (verdict == QLatin1String("trial-failed")) {
                    row.ended = QObject::tr("Went into a restart that came back");
                    row.mood = HistoryRow::Bad;
                } else {
                    row.ended = QObject::tr("In the finished next system, not started");
                    row.mood = HistoryRow::Warn;
                }
            } else if (preparation == openPreparation) {
                row.ended = QObject::tr("In the next system, not finished");
                row.mood = HistoryRow::Warn;
            } else {
                row.ended = QObject::tr("recab records no end for it");
            }
        } else if (kind == QLatin1String("init")) {
            row.what = QObject::tr("recab was set up here%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Done");
        } else if (kind == QLatin1String("prepare")) {
            row.what = QObject::tr("The next system was begun as a copy of the running one%1").arg(slotWords(row.slot));
            row.ended = outcome == QLatin1String("interrupted") ? QObject::tr("Stopped") : QObject::tr("Begun; recab does not log its end");
            row.mood = outcome == QLatin1String("interrupted") ? HistoryRow::Bad : HistoryRow::Plain;
        } else if (kind == QLatin1String("update")) {
            row.what = QObject::tr("A release was merged into the next system%1").arg(slotWords(row.slot));
            row.ended = outcome == QLatin1String("interrupted") ? QObject::tr("Stopped, waiting to be settled") : QObject::tr("Merged");
            row.mood = outcome == QLatin1String("interrupted") ? HistoryRow::Warn : HistoryRow::Plain;
        } else if (kind == QLatin1String("finalize")) {
            row.what = QObject::tr("The next system was finished: checked and closed%1").arg(slotWords(row.slot));
            row.ended = outcome == QLatin1String("done") ? QObject::tr("Done") : QObject::tr("Begun; recab does not log its end");
        } else if (kind == QLatin1String("discard")) {
            row.what = QObject::tr("The next system was given up%1").arg(slotWords(row.slot));
            row.ended = outcome == QLatin1String("done") ? QObject::tr("Done") : QObject::tr("Stopped");
            row.mood = outcome == QLatin1String("done") ? HistoryRow::Plain : HistoryRow::Bad;
        } else if (kind == QLatin1String("set-next")) {
            row.what = QObject::tr("A restart into the next system was set%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Set");
        } else if (kind == QLatin1String("trial-kept")) {
            row.what = QObject::tr("Restart into the next system%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Kept: start-up finished with nothing failed");
            row.mood = HistoryRow::Ok;
        } else if (kind == QLatin1String("trial-failed")) {
            row.what = QObject::tr("Restart into the next system%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Came back: it did not start properly");
            row.mood = HistoryRow::Bad;
        } else if (kind == QLatin1String("trial-missed")) {
            row.what = QObject::tr("A set restart was used up by another start%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Not a trial");
        } else if (kind == QLatin1String("commit")) {
            const bool cancelled = outcome == QLatin1String("restart_cancelled");
            row.what = cancelled ? QObject::tr("The set restart was cancelled by keeping the running system")
                                 : QObject::tr("The running system was kept%1").arg(slotWords(row.slot));
            row.ended = cancelled ? QObject::tr("The next system stays ready") : QObject::tr("Kept");
            row.mood = HistoryRow::Ok;
        } else if (kind == QLatin1String("rollback")) {
            row.what = QObject::tr("Going back to the system from before was set%1").arg(slotWords(row.slot));
            row.ended = QObject::tr("Set");
        } else {
            row.what = QObject::tr("%1%2").arg(kind, slotWords(row.slot));
        }
        h.rows << row;
    }
    return h;
}

QString History::text() const
{
    QString out = QObject::tr("History of this computer's systems, newest first\n\n");
    for (const HistoryRow &row : rows) {
        out += QStringLiteral("%1  %2").arg(QLocale().toString(QDateTime::fromSecsSinceEpoch(row.when), QLocale::ShortFormat), row.what);
        if (row.carriesActor)
            out += QObject::tr("  (%1)").arg(row.who);
        out += QObject::tr("  -  %1").arg(row.ended);
        if (!row.detail.isEmpty())
            out += QObject::tr(": %1").arg(row.detail);
        out += QLatin1Char('\n');
    }
    return out;
}
