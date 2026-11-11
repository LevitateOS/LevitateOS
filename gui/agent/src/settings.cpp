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
        // A boolean in the file; a word is read too, so a hand-written file works.
        const QJsonValue notices = object["notices"];
        if (notices.isBool())
            settings.notices = notices.toBool();
        else if (notices.isString())
            settings.notices = notices.toString() != QLatin1String("false");
    }
    if (!qEnvironmentVariable("LEVITATE_AGENT").isEmpty())
        settings.agent = qEnvironmentVariable("LEVITATE_AGENT");
    if (!qEnvironmentVariable("LEVITATE_AGENT_CODEX").isEmpty())
        settings.codexProgram = qEnvironmentVariable("LEVITATE_AGENT_CODEX");
    if (!qEnvironmentVariable("LEVITATE_AGENT_CLAUDE").isEmpty())
        settings.claudeProgram = qEnvironmentVariable("LEVITATE_AGENT_CLAUDE");
    if (qEnvironmentVariableIsSet("LEVITATE_AGENT_NOTIFIER")) {
        settings.notifierSet = true;
        settings.notifier = qEnvironmentVariable("LEVITATE_AGENT_NOTIFIER");
    }
    // Set but empty is a choice (checks run the command directly).
    if (qEnvironmentVariableIsSet("LEVITATE_AGENT_ROOT_HELPER"))
        settings.rootHelper = qEnvironmentVariable("LEVITATE_AGENT_ROOT_HELPER");

    const QString state = xdgDir("XDG_STATE_HOME", settings.home + "/.local/state") + "/levitate/agent";
    settings.runsDir = state + "/runs";
    // The runtime folder is the right place for sockets: it is the person's
    // own, goes away at logout, and is never backed up. Without one (a bare
    // console, a check) the state folder holds them instead.
    const QString runtime = qEnvironmentVariable("XDG_RUNTIME_DIR");
    settings.socketsDir = runtime.isEmpty() ? state + "/sockets" : runtime + "/levitate-agent";
    return settings;
}

QString Settings::set(const QString &key, const QString &value)
{
    if (key == QLatin1String("agent")) {
        if (value != QLatin1String("codex") && value != QLatin1String("claude")) {
            if (!QDir::isAbsolutePath(value))
                return QStringLiteral("The agent is codex, claude, or the full path of a program.");
            const QFileInfo info(value);
            if (!info.exists())
                return QStringLiteral("%1 does not exist.").arg(value);
            if (!info.isFile() || !info.isExecutable())
                return QStringLiteral("%1 is not a program that can be run.").arg(value);
        }
    } else if (key == QLatin1String("run_commands")) {
        if (value != QLatin1String("ask") && value != QLatin1String("allowed"))
            return QStringLiteral("Running commands is either ask or allowed.");
    } else if (key == QLatin1String("notices")) {
        if (value != QLatin1String("true") && value != QLatin1String("false"))
            return QStringLiteral("Notices are either true or false.");
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
    if (key == QLatin1String("notices"))
        object[key] = value == QLatin1String("true");
    else
        object[key] = value;
    if (!QDir().mkpath(QFileInfo(path).absolutePath()))
        return QStringLiteral("The folder for %1 could not be made.").arg(path);
    QSaveFile out(path);
    if (!out.open(QIODevice::WriteOnly) || out.write(QJsonDocument(object).toJson(QJsonDocument::Indented)) < 0
        || !out.commit())
        return QStringLiteral("%1 could not be written.").arg(path);
    return QString();
}
