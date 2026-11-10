#include "settings.h"

#include <QDir>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>

// XDG says an unset or empty variable means the default.
static QString xdgDir(const char *variable, const QString &fallback)
{
    const QString value = qEnvironmentVariable(variable);
    return value.isEmpty() ? fallback : value;
}

Settings Settings::load()
{
    Settings settings;
    settings.home = QDir::homePath();

    const QString config = xdgDir("XDG_CONFIG_HOME", settings.home + "/.config") + "/levitate/agent.json";
    QFile file(config);
    if (file.open(QIODevice::ReadOnly)) {
        // A file we cannot read as JSON is the same as no file: the apps
        // must still work, and the defaults are the careful ones.
        const QJsonObject object = QJsonDocument::fromJson(file.readAll()).object();
        if (!object["agent"].toString().isEmpty())
            settings.agent = object["agent"].toString();
        settings.commandsAllowed = object["run_commands"].toString() == QLatin1String("allowed");
    }
    if (!qEnvironmentVariable("LEVITATE_AGENT").isEmpty())
        settings.agent = qEnvironmentVariable("LEVITATE_AGENT");
    if (!qEnvironmentVariable("LEVITATE_AGENT_CODEX").isEmpty())
        settings.codexProgram = qEnvironmentVariable("LEVITATE_AGENT_CODEX");
    // Set but empty is a choice (checks run the command directly).
    if (qEnvironmentVariableIsSet("LEVITATE_AGENT_ROOT_HELPER"))
        settings.rootHelper = qEnvironmentVariable("LEVITATE_AGENT_ROOT_HELPER");

    settings.runsDir = xdgDir("XDG_STATE_HOME", settings.home + "/.local/state") + "/levitate/agent/runs";
    return settings;
}
