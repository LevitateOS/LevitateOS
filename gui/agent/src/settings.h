// What the person and the environment chose. Read once, at start.
#pragma once

#include <QString>

struct Settings {
    // `codex`, or the path of a program that already speaks the apps' protocol.
    QString agent = QStringLiteral("codex");
    // Whether the agent may run commands itself, without asking each time.
    bool commandsAllowed = false;
    QString home;
    QString runsDir;
    QString codexProgram = QStringLiteral("codex");
    // Runs what the person approved as administrator; empty runs it directly.
    QString rootHelper = QStringLiteral("pkexec");

    static Settings load();
};
