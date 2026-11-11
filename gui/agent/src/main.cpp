// levitate-agent: what the desktop apps talk to instead of a coding agent.
// The apps speak one small protocol (see README.md); each agent speaks its own.
//
// Started the usual way (the request on standard input) it is only a relay:
// the run itself lives in a process of its own, `levitate-agent --host`, that
// the relay starts and that goes on when the relay ends.
#include "account.h"
#include "claude.h"
#include "client.h"
#include "codex.h"
#include "passthrough.h"
#include "record.h"
#include "session.h"
#include "settings.h"
#include "status.h"

#include <QCoreApplication>
#include <QJsonDocument>
#include <QJsonObject>

#include <cstdio>

static void print(const QJsonDocument &document)
{
    fputs(document.toJson(QJsonDocument::Indented).constData(), stdout);
}

// What is in force, defaults and the environment included.
static QJsonObject settingsInForce(const Settings &settings)
{
    return {{"agent", settings.agent},
            {"run_commands", settings.commandsAllowed ? "allowed" : "ask"},
            {"notices", settings.notices},
            {"file", settings.configFile}};
}

int main(int argc, char **argv)
{
    QCoreApplication app(argc, argv);
    const QStringList args = app.arguments().mid(1);
    const Settings settings = Settings::load();

    if (args.size() == 1 && args[0] == QLatin1String("--version")) {
        printf("levitate-agent %s\n", LEVITATE_AGENT_VERSION);
        return 0;
    }
    if (args.value(0) == QLatin1String("--list")) {
        print(QJsonDocument(listRuns(settings)));
        return 0;
    }
    if (args.value(0) == QLatin1String("--show") && args.size() == 2) {
        bool found;
        const QJsonArray lines = showRun(settings, args[1], &found);
        if (!found) {
            fprintf(stderr, "levitate-agent: no run %s\n", qPrintable(args[1]));
            return 1;
        }
        print(QJsonDocument(lines));
        return 0;
    }
    if (args.value(0) == QLatin1String("--status")) {
        print(QJsonDocument(agentStatus(settings)));
        return 0;
    }
    if (args.value(0) == QLatin1String("--settings") && args.size() == 1) {
        print(QJsonDocument(settingsInForce(settings)));
        return 0;
    }
    if (args.value(0) == QLatin1String("--set") && (args.size() != 2 || !args[1].contains('='))) {
        fputs("Say what to set as KEY=VALUE.\n", stderr);
        return 1;
    }
    if (args.value(0) == QLatin1String("--set")) {
        const QString refused = Settings::set(args[1].section('=', 0, 0), args[1].section('=', 1));
        if (!refused.isEmpty()) {
            fprintf(stderr, "%s\n", qPrintable(refused));
            return 1;
        }
        print(QJsonDocument(settingsInForce(Settings::load())));
        return 0;
    }
    if (args.size() == 1 && (args[0] == QLatin1String("--sign-in") || args[0] == QLatin1String("--sign-out")))
        return account(settings, args[0] == QLatin1String("--sign-in"));
    if (args.size() == 2 && args[0] == QLatin1String("--stop"))
        return stopRun(settings, args[1]);

    if (args.size() == 2 && args[0] == QLatin1String("--attach")) {
        Client client(settings, Client::Attach, args[1]);
        client.run();
        return app.exec();
    }

    // The process a run lives in. Only the relay starts it, with the request
    // on its standard input; it is not for people to run.
    if (args.size() == 4 && args[0] == QLatin1String("--host") && args[2] == QLatin1String("--starter")) {
        Session session(settings, args[1], args[3].toLongLong());
        // The agent is named in one place. Another agent is another branch here.
        if (settings.agent == QLatin1String("codex"))
            session.setAdapter(new Codex(&session));
        else if (settings.agent == QLatin1String("claude"))
            session.setAdapter(new Claude(&session));
        else
            session.setAdapter(new Passthrough(&session, settings.agent));
        session.run();
        return app.exec();
    }

    if (!args.isEmpty()) {
        fputs("usage: levitate-agent             (reads a request on standard input)\n"
              "       levitate-agent --list | --status | --show ID | --version\n"
              "       levitate-agent --attach ID | --stop ID\n"
              "       levitate-agent --settings | --set KEY=VALUE | --sign-in | --sign-out\n",
              stderr);
        return args.value(0) == QLatin1String("--help") ? 0 : 2;
    }

    Client client(settings, Client::Relay, Record::newId());
    client.run();
    return app.exec();
}
