#include "status.h"

#include <QFileInfo>
#include <QJsonArray>
#include <QProcess>
#include <QRegularExpression>
#include <QStandardPaths>

namespace {

struct Ran {
    bool started = false;
    int code = -1;
    QString text;
};

// A short question to a program. It must never wait for a person: the
// input is closed, and a program that hangs is given up on.
Ran ask(const QString &program, const QStringList &arguments)
{
    Ran ran;
    QProcess process;
    process.setProcessChannelMode(QProcess::MergedChannels);
    process.start(program, arguments);
    process.closeWriteChannel();
    if (!process.waitForStarted(3000))
        return ran;
    ran.started = true;
    if (!process.waitForFinished(10000)) {
        process.kill();
        process.waitForFinished(1000);
        return ran;
    }
    ran.code = process.exitStatus() == QProcess::NormalExit ? process.exitCode() : -1;
    ran.text = QString::fromUtf8(process.readAll()).trimmed();
    return ran;
}

} // namespace

QJsonObject agentStatus(const Settings &settings)
{
    QJsonArray agents;

    const QString found = QStandardPaths::findExecutable(settings.codexProgram);
    const bool installed = !found.isEmpty() || (settings.codexProgram.contains('/') && QFileInfo(settings.codexProgram).isExecutable());
    QString version;
    QJsonValue signedIn;
    if (installed) {
        const QString program = found.isEmpty() ? settings.codexProgram : found;
        const Ran asked = ask(program, {"--version"});
        // "codex-cli 0.167.0": the number is what matters.
        static const QRegularExpression number(QStringLiteral("\\d+(\\.\\d+)+\\S*"));
        const QRegularExpressionMatch match = number.match(asked.text);
        version = match.hasMatch() ? match.captured() : asked.text;
        // Prints "Logged in using ..." and exits 0 when signed in; anything
        // else is not signed in. A program that could not be asked says nothing.
        const Ran login = ask(program, {"login", "status"});
        if (login.started)
            signedIn = login.code == 0;
    }
    agents << QJsonObject{{"id", "codex"}, {"name", "Codex"}, {"installed", installed}, {"version", version}, {"signed_in", signedIn}};

    if (settings.agent != QLatin1String("codex")) {
        const bool usable = QFileInfo(settings.agent).isExecutable() || !QStandardPaths::findExecutable(settings.agent).isEmpty();
        agents << QJsonObject{{"id", settings.agent}, {"name", QFileInfo(settings.agent).fileName()}, {"installed", usable},
                              {"version", ""}, {"signed_in", QJsonValue()}};
    }
    return {{"chosen", settings.agent}, {"agents", agents}};
}
