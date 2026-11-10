#include "model.h"

#include <QJsonArray>

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
