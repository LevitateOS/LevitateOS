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
    if (schema == 3) {
        // The stand-in: it reports the lifecycle directly.
        *into = Status::fromJson(object);
    } else if (schema == 7 || schema == 8 || schema == 9) {
        // The real recab: two slots, a preparation and the state in one
        // word. It names no release.
        *into = Status::fromReal(object, osField(osRelease_, "NAME"), osField(osRelease_, "VERSION_ID"));
        if (!into->unusable.isEmpty()) {
            *error = tr("recab says the system needs recovery before anything else: %1").arg(into->unusable);
            return false;
        }
    } else {
        *error = tr("`%1 status --json` answered with schema %2. This window reads schemas 7, 8 and 9, recab's own, and "
                    "schema 3, its stand-in's.")
                     .arg(program_)
                     .arg(schema);
        return false;
    }
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
    const QStringList asked{"--json", "--actor", "person"};
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
