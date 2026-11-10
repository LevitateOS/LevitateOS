// What the person and the environment chose. Read once, at start.
#pragma once

#include <QString>

struct Settings {
    // `codex`, or the path of a program that already speaks the apps' protocol.
    QString agent = QStringLiteral("codex");
    // Whether the agent may run commands itself, without asking each time.
    bool commandsAllowed = false;
    QString home;
    // `agent.json`, whether or not it exists yet.
    QString configFile;
    QString runsDir;
    QString codexProgram = QStringLiteral("codex");
    // Runs what the person approved as administrator; empty runs it directly.
    QString rootHelper = QStringLiteral("pkexec");

    static Settings load();

    // `--set key=value`: checks the value, then writes it into `agent.json`
    // and keeps every other key that is there. Returns an empty string, or one
    // plain sentence saying why the value was refused (and nothing is written).
    static QString set(const QString &key, const QString &value);
};
