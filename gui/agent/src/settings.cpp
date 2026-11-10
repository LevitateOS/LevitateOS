#include "settings.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QSaveFile>
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

    settings.configFile = xdgDir("XDG_CONFIG_HOME", settings.home + "/.config") + "/levitate/agent.json";
    QFile file(settings.configFile);
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

QString Settings::set(const QString &key, const QString &value)
{
    if (key == QLatin1String("agent")) {
        if (value != QLatin1String("codex")) {
            if (!QDir::isAbsolutePath(value))
                return QStringLiteral("The agent is codex, or the full path of a program.");
            const QFileInfo info(value);
            if (!info.exists())
                return QStringLiteral("%1 does not exist.").arg(value);
            if (!info.isFile() || !info.isExecutable())
                return QStringLiteral("%1 is not a program that can be run.").arg(value);
        }
    } else if (key == QLatin1String("run_commands")) {
        if (value != QLatin1String("ask") && value != QLatin1String("allowed"))
            return QStringLiteral("Running commands is either ask or allowed.");
    } else {
        return QStringLiteral("There is no setting called %1.").arg(key);
    }

    const QString path = load().configFile;
    QJsonObject object;
    QFile old(path);
    // Whatever else is in the file stays; a file that is not JSON is replaced.
    if (old.open(QIODevice::ReadOnly))
        object = QJsonDocument::fromJson(old.readAll()).object();
    old.close();
    object[key] = value;
    if (!QDir().mkpath(QFileInfo(path).absolutePath()))
        return QStringLiteral("The folder for %1 could not be made.").arg(path);
    QSaveFile out(path);
    if (!out.open(QIODevice::WriteOnly) || out.write(QJsonDocument(object).toJson(QJsonDocument::Indented)) < 0
        || !out.commit())
        return QStringLiteral("%1 could not be written.").arg(path);
    return QString();
}
