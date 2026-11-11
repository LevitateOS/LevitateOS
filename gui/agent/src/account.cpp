#include "account.h"

#include "child.h"

#include <QCoreApplication>
#include <QFileInfo>
#include <QSocketNotifier>

#include <cerrno>
#include <csignal>
#include <fcntl.h>
#include <unistd.h>

namespace {

int stopPipe[2] = {-1, -1};

void onSignalNumber(int)
{
    const char byte = 1;
    [[maybe_unused]] const ssize_t written = ::write(stopPipe[1], &byte, 1);
}

void say(const QByteArray &line)
{
    const QByteArray out = line + '\n';
    [[maybe_unused]] const ssize_t written = ::write(2, out.constData(), size_t(out.size()));
}

} // namespace

int account(const Settings &settings, bool signIn)
{
    const bool claude = settings.agent == QLatin1String("claude");
    if (settings.agent != QLatin1String("codex") && !claude) {
        say(QFileInfo(settings.agent).fileName().toUtf8() + " signs in by itself; there is nothing to do here.");
        return 1;
    }
    Child codex; // the agent's own program, Codex or Claude Code
    // `codex login` and `codex logout`; Claude Code has `claude auth login`
    // and `claude auth logout` (its CLI reference).
    codex.setProgram(claude ? settings.claudeProgram : settings.codexProgram);
    codex.setArguments(claude ? QStringList{"auth", signIn ? "login" : "logout"} : QStringList{signIn ? "login" : "logout"});
    codex.setProcessChannelMode(QProcess::MergedChannels);
    codex.setStandardInputFile(QProcess::nullDevice());
    int code = 1;
    bool started = true;
    QByteArray pending;
    auto drain = [&](bool last) {
        pending += codex.readAll();
        qsizetype end;
        while ((end = pending.indexOf('\n')) >= 0) {
            say(pending.left(end));
            pending.remove(0, end + 1);
        }
        if (last && !pending.isEmpty())
            say(pending);
        if (last)
            pending.clear();
    };
    QObject::connect(&codex, &QProcess::readyRead, [&] { drain(false); });
    QObject::connect(&codex, &QProcess::finished, [&](int exitCode, QProcess::ExitStatus status) {
        drain(true);
        code = status == QProcess::NormalExit ? exitCode : 1;
        QCoreApplication::quit();
    });
    QObject::connect(&codex, &QProcess::errorOccurred, [&](QProcess::ProcessError error) {
        if (error != QProcess::FailedToStart)
            return;
        started = false;
        say(claude ? "Claude Code is not installed." : "Codex is not installed.");
        QCoreApplication::quit();
    });
    // Stopped (the window closed, the person cancelled): Codex goes too.
    ::signal(SIGPIPE, SIG_IGN);
    if (::pipe2(stopPipe, O_CLOEXEC | O_NONBLOCK) == 0) {
        struct sigaction action {};
        action.sa_handler = onSignalNumber;
        sigemptyset(&action.sa_mask);
        for (int number : {SIGTERM, SIGINT, SIGHUP})
            sigaction(number, &action, nullptr);
        auto *notifier = new QSocketNotifier(stopPipe[0], QSocketNotifier::Read, &codex);
        QObject::connect(notifier, &QSocketNotifier::activated, [&] {
            codex.endGroup();
            ::_exit(143);
        });
    }
    codex.startInGroup();
    QCoreApplication::exec();
    return started ? code : 1;
}
