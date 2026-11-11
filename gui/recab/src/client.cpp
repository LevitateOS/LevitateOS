#include "client.h"

#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>

RecabClient::RecabClient(QObject *parent)
    : QObject(parent)
    , program_(qEnvironmentVariable("RECAB_GUI_RECAB", "recab"))
    , rootHelper_(qEnvironmentVariable("RECAB_GUI_ROOT_HELPER", "pkexec"))
    , agent_(qEnvironmentVariable("RECAB_GUI_AGENT", "levitate-agent"))
    , reboot_(qEnvironmentVariable("RECAB_GUI_REBOOT", "reboot"))
    , osRelease_(qEnvironmentVariable("RECAB_GUI_OS_RELEASE", "/etc/os-release"))
{
}

// One field of the running system's os-release, without its quotes.
static QString osField(const QString &path, const QString &field)
{
    QFile file(path);
    if (!file.open(QIODevice::ReadOnly))
        return {};
    for (const QByteArray &raw : file.readAll().split('\n')) {
        const QString line = QString::fromUtf8(raw).trimmed();
        if (line.startsWith(field + QLatin1Char('=')))
            return line.mid(field.size() + 1).remove(QLatin1Char('"'));
    }
    return {};
}

Run *RecabClient::read(bool asRoot)
{
    const QStringList arguments{"status", "--json"};
    if (asRoot && !rootHelper_.isEmpty())
        return new Run(rootHelper_, QStringList{program_} + arguments, this);
    return new Run(program_, arguments, this);
}

Run *RecabClient::readHistory()
{
    return new Run(program_, QStringList{"history", "--json"}, this);
}

bool RecabClient::understandHistory(Run *finished, bool ok, History *into, const QString &openPreparation)
{
    *into = History();
    if (!ok) {
        const QString said = finished->problem();
        // A recab from before the command: clap's own words for an unknown
        // one, or the older stand-in's.
        into->unknown = said.contains(QLatin1String("unrecognized subcommand"))
                        || said.contains(QLatin1String("unknown command"));
        into->problem = said;
        return false;
    }
    const QJsonObject object = QJsonDocument::fromJson(finished->answer()).object();
    if (object["schema_version"].toInt() != 1) {
        into->problem = tr("`%1 history --json` answered with schema %2; this window reads schema 1.")
                            .arg(program_)
                            .arg(object["schema_version"].toInt());
        return false;
    }
    *into = History::fromJson(object, QString(), openPreparation);
    return true;
}

bool RecabClient::understand(Run *finished, bool ok, Status *into, QString *error, bool *permission)
{
    if (permission)
        *permission = false;
    if (!ok) {
        *error = tr("`%1 status --json` failed: %2").arg(program_, finished->problem());
        if (permission)
            *permission = finished->failure()["class"] == QLatin1String("permission")
                          || finished->problem().contains(QLatin1String("Permission denied"));
        return false;
    }
    const QJsonObject object = QJsonDocument::fromJson(finished->answer()).object();
    const int schema = object["schema_version"].toInt();
    if(schema==10 && object["updates"].toObject()["phase"]=="unavailable" && object["updates"].toObject()["last_error"].toString().contains("Permission denied")){
        *error=tr("Administrator access is needed to read the prepared release and update records.");if(permission)*permission=true;return false;
    }
    if (schema == 3) {
        // The stand-in: it reports the lifecycle directly.
        *into = Status::fromJson(object);
    } else if (schema == 7 || schema == 8 || schema == 9 || schema == 10) {
        // The real recab: two slots, a preparation and the state in one
        // word. It names no release.
        *into = Status::fromReal(object, osField(osRelease_, "NAME"), osField(osRelease_, "VERSION_ID"));
        if (!into->unusable.isEmpty() && into->operations.isEmpty()) {
            *error = tr("recab says the system needs recovery before anything else: %1").arg(into->unusable);
            return false;
        }
    } else {
        *error = tr("`%1 status --json` answered with schema %2. This window reads schemas 7 through 10, recab's own, and "
                    "schema 3, its stand-in's.")
                     .arg(program_)
                     .arg(schema);
        return false;
    }
    operations_ = into->operations;
    real_ = into->real;
    preparationId_ = into->preparationId;
    preparationRevision_ = into->preparationRevision;
    otherSlot_ = into->otherSlot;
    return true;
}

// This window's name for a change, as the real recab's command.
QStringList RecabClient::realArguments(const QStringList &arguments) const
{
    const QString verb = arguments.value(0);
    // What this window asks for is asked by the person in front of it.
    QStringList asked{"--json", "--actor", "person"};
    if(!operations_.isEmpty())asked<<"--machine-events";
    if(verb=="catalog")return asked+arguments.mid(1);
    if(!operations_.isEmpty()){
        QString id=verb=="set-next"?QStringLiteral("set_next"):verb;
        if(verb=="recover"){for(const auto &candidate:{"trial_recover","change_recover","update_recover","finalize_recover","sync_recover"})for(const auto &v:operations_){auto op=v.toObject();if(op["id"]==candidate && op["verdict"]=="ready"){QStringList args;for(auto a:op["argv"].toArray())args<<a.toString();return asked+args;}}}
        for(const auto &v:operations_){auto op=v.toObject();if(op["id"]==id){QStringList args;for(auto a:op["argv"].toArray())args<<a.toString();return asked+args;}}
    }
    QStringList which;
    if (!preparationId_.isEmpty() && preparationRevision_ >= 0)
        which = QStringList{"--preparation", preparationId_, "--revision", QString::number(preparationRevision_)};
    if (verb == QLatin1String("finalize"))
        return asked + QStringList{"finalize"} + which;
    if (verb == QLatin1String("discard"))
        return asked + QStringList{"discard"} + which;
    // Continuing what stopped is preparing again: recab looks at what is really there.
    if (verb == QLatin1String("recover"))
        return asked + QStringList{"prepare"} + which;
    if (verb == QLatin1String("set-next"))
        return asked + QStringList{"set-next", otherSlot_};
    // `--json` makes a failure one JSON object with recab's message.
    return asked + arguments;
}

Run *RecabClient::change(const QStringList &arguments)
{
    const QStringList all = real_ ? realArguments(arguments) : arguments + QStringList{"--machine-events"};
    if (rootHelper_.isEmpty())
        return new Run(program_, all, this);
    return new Run(rootHelper_, QStringList{program_} + all, this);
}

Run *RecabClient::agent(const QString &prompt, const QString &title)
{
    auto *run = new Run(agent_, {}, this);
    run->setPrompt(prompt, QStringLiteral("System"), title);
    return run;
}

Run *RecabClient::reboot()
{
    if (rootHelper_.isEmpty())
        return new Run(reboot_, {}, this);
    return new Run(rootHelper_, {reboot_}, this);
}

QString RecabClient::agentName() const
{
    return QFileInfo(agent_).fileName();
}
