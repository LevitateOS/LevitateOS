// levitate-agent: what the desktop apps talk to instead of a coding agent.
// The apps speak one small protocol (see README.md); each agent speaks its own.
#include "codex.h"
#include "passthrough.h"
#include "record.h"
#include "session.h"
#include "settings.h"
#include "status.h"

#include <QCoreApplication>
#include <QJsonDocument>

#include <cstdio>

static void print(const QJsonDocument &document)
{
    fputs(document.toJson(QJsonDocument::Indented).constData(), stdout);
}

int main(int argc, char **argv)
{
    QCoreApplication app(argc, argv);
    const QStringList args = app.arguments().mid(1);
    const Settings settings = Settings::load();

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
    if (!args.isEmpty()) {
        fputs("usage: levitate-agent             (reads a request on standard input)\n"
              "       levitate-agent --list | --status | --show ID\n",
              stderr);
        return args.value(0) == QLatin1String("--help") ? 0 : 2;
    }

    Session session(settings);
    // The agent is named in one place. Another agent is another branch here.
    if (settings.agent == QLatin1String("codex"))
        session.setAdapter(new Codex(&session));
    else
        session.setAdapter(new Passthrough(&session, settings.agent));
    session.run();
    return app.exec();
}
