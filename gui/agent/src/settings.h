// What the person and the environment chose. Read once, at start.
#pragma once

#include <QString>

struct Settings {
    // `codex`, `claude`, or the path of a program that already speaks the apps' protocol.
    QString agent = QStringLiteral("codex");
    // Whether the agent may run commands itself, without asking each time.
    bool commandsAllowed = false;
    // Whether a desktop notice is shown when a run needs the person, or ends,
    // and no window is watching.
    bool notices = true;
    QString home;
    // `agent.json`, whether or not it exists yet.
    QString configFile;
    QString runsDir;
    // One Unix socket for each run that is going, so that a window can attach.
    QString socketsDir;
    QString codexProgram = QStringLiteral("codex");
    QString claudeProgram = QStringLiteral("claude");
    // What shows a notice: it is run with the title and the text as its two
    // arguments. Unset means the session bus is asked through `gdbus`; set but
    // empty means no notice is shown at all (the checks use this).
    QString notifier;
    bool notifierSet = false;
    // Runs what the person approved as administrator; empty runs it directly.
    QString rootHelper = QStringLiteral("pkexec");

    static Settings load();

    // `--set key=value`: checks the value, then writes it into `agent.json`
    // and keeps every other key that is there. Returns an empty string, or one
    // plain sentence saying why the value was refused (and nothing is written).
    static QString set(const QString &key, const QString &value);
};
