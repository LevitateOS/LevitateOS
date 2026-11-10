#include "record.h"

#include <QDateTime>
#include <QDir>
#include <QJsonDocument>
#include <QRandomGenerator>

#include <algorithm>
#include <cerrno>
#include <csignal>
#include <unistd.h>

QString now()
{
    return QDateTime::currentDateTimeUtc().toString(Qt::ISODateWithMs);
}

void Record::open(const QString &app, const QString &title, const QString &prompt)
{
    id_ = QString::number(QRandomGenerator::global()->generate(), 16).rightJustified(8, QLatin1Char('0'));
    QDir().mkpath(settings_.runsDir);
    const QString stamp = QDateTime::currentDateTimeUtc().toString(QStringLiteral("yyyyMMdd'T'HHmmss'Z'"));
    file_.setFileName(settings_.runsDir + "/" + stamp + "-" + id_ + ".jsonl");
    // A record that cannot be written must not stop the work: the person
    // asked for the work, not for the record.
    if (!file_.open(QIODevice::WriteOnly | QIODevice::Append))
        qWarning("levitate-agent: cannot write the record %s", qPrintable(file_.fileName()));
    // The pid lets a later --list tell a run that is still going from one
    // that was killed before it could write its last line.
    line({{"record", "request"}, {"id", id_}, {"t", now()}, {"app", app}, {"title", title}, {"prompt", prompt},
          {"agent", settings_.agent}, {"pid", qint64(getpid())}});
}

void Record::event(const QJsonObject &event)
{
    QJsonObject stamped = event;
    stamped["t"] = now();
    line(stamped);
}

void Record::end(const QString &state)
{
    if (ended_)
        return;
    ended_ = true;
    line({{"record", "end"}, {"t", now()}, {"state", state}});
}

void Record::line(const QJsonObject &object)
{
    if (!file_.isOpen())
        return;
    file_.write(QJsonDocument(object).toJson(QJsonDocument::Compact) + '\n');
    file_.flush();
}

static QList<QJsonObject> readLines(const QString &path)
{
    QList<QJsonObject> lines;
    QFile file(path);
    if (!file.open(QIODevice::ReadOnly))
        return lines;
    for (const QByteArray &raw : file.readAll().split('\n')) {
        const QJsonObject object = QJsonDocument::fromJson(raw).object();
        if (!object.isEmpty())
            lines << object;
    }
    return lines;
}

static bool alive(qint64 pid)
{
    return pid > 0 && (::kill(pid_t(pid), 0) == 0 || errno == EPERM);
}

static QJsonObject summarise(const QList<QJsonObject> &lines)
{
    const QJsonObject request = lines.value(0);
    QString state = QStringLiteral("running");
    QString summary, ended;
    QString last;
    for (const QJsonObject &line : lines) {
        if (line["record"] == QLatin1String("end")) {
            state = line["state"].toString();
            ended = line["t"].toString();
        } else if (line.contains("event")) {
            last = line["event"].toString();
            if (last == QLatin1String("agent") && !line["summary"].toString().isEmpty())
                summary = line["summary"].toString();
            else if (last == QLatin1String("failed"))
                summary = line["reason"].toString();
        }
    }
    if (ended.isEmpty()) {
        if (!alive(request["pid"].toInteger())) {
            // Killed without a last line: it is over, and not by finishing.
            state = QStringLiteral("stopped");
            ended = lines.last()["t"].toString();
        } else if (last == QLatin1String("question")) {
            state = QStringLiteral("waiting");
        }
    }
    return {{"id", request["id"]},   {"app", request["app"]},         {"title", request["title"]},
            {"state", state},        {"started", request["t"]},       {"ended", ended.isEmpty() ? QJsonValue() : ended},
            {"summary", summary}};
}

QJsonObject listRuns(const Settings &settings)
{
    // The file name starts with the time to the second; the request line has
    // it to the millisecond, which orders runs that began together.
    QList<QJsonObject> runs;
    const QStringList names = QDir(settings.runsDir).entryList({"*.jsonl"}, QDir::Files, QDir::Name | QDir::Reversed);
    for (const QString &name : names.mid(0, 200)) {
        const QList<QJsonObject> lines = readLines(settings.runsDir + "/" + name);
        if (!lines.isEmpty() && lines.first()["record"] == QLatin1String("request"))
            runs << summarise(lines);
    }
    std::stable_sort(runs.begin(), runs.end(),
                     [](const QJsonObject &a, const QJsonObject &b) { return a["started"].toString() > b["started"].toString(); });
    QJsonArray sorted;
    for (const QJsonObject &run : runs)
        sorted << run;
    return {{"runs", sorted}};
}

QJsonArray showRun(const Settings &settings, const QString &id, bool *found)
{
    *found = false;
    QJsonArray all;
    // The id is only ever compared with file names, never put into a path.
    const QStringList names = QDir(settings.runsDir).entryList({"*.jsonl"}, QDir::Files);
    for (const QString &name : names) {
        if (id.isEmpty() || !name.endsWith("-" + id + ".jsonl"))
            continue;
        *found = true;
        for (const QJsonObject &line : readLines(settings.runsDir + "/" + name))
            all << line;
    }
    return all;
}
