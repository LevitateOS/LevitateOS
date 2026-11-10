#include "client.h"

#include <QFileInfo>
#include <QJsonDocument>

RecabClient::RecabClient(QObject *parent)
    : QObject(parent)
    , program_(qEnvironmentVariable("RECAB_GUI_RECAB", "recab"))
    , rootHelper_(qEnvironmentVariable("RECAB_GUI_ROOT_HELPER", "pkexec"))
    , agent_(qEnvironmentVariable("RECAB_GUI_AGENT", "codex"))
    , reboot_(qEnvironmentVariable("RECAB_GUI_REBOOT", "reboot"))
{
}

Run *RecabClient::read()
{
    return new Run(program_, {"status", "--json"}, this);
}

bool RecabClient::understand(Run *finished, bool ok, Status *into, QString *error) const
{
    if (!ok) {
        *error = tr("`%1 status --json` failed: %2").arg(program_, finished->problem());
        return false;
    }
    const QJsonObject object = QJsonDocument::fromJson(finished->answer()).object();
    // The lifecycle this front end shows arrives with schema 3; an older recab
    // describes slots and staging, which this window cannot draw truthfully.
    if (object["schema_version"].toInt() != 3) {
        *error = tr("`%1 status --json` answered with schema %2. This window needs schema 3, the one that "
                    "reports the next system, its changes and what may be done.")
                     .arg(program_)
                     .arg(object["schema_version"].toInt());
        return false;
    }
    *into = Status::fromJson(object);
    return true;
}

Run *RecabClient::change(const QStringList &arguments)
{
    const QStringList all = arguments + QStringList{"--machine-events"};
    if (rootHelper_.isEmpty())
        return new Run(program_, all, this);
    return new Run(rootHelper_, QStringList{program_} + all, this);
}

Run *RecabClient::agent(const QString &prompt)
{
    auto *run = new Run(agent_, {}, this);
    run->setPrompt(prompt);
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
