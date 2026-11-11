#include "record.h"

#include "link.h"

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

QString Record::newId()
{
    return QString::number(QRandomGenerator::global()->generate(), 16).rightJustified(8, QLatin1Char('0'));
}

QString socketName(const QString &id)
{
    return id + QStringLiteral(".sock");
}

void Record::open(const QString &id, const QString &app, const QString &title, const QString &prompt, qint64 starter,
                  const QJsonObject &extra)
{
    id_ = id;
    QDir().mkpath(settings_.runsDir);
    const QString stamp = QDateTime::currentDateTimeUtc().toString(QStringLiteral("yyyyMMdd'T'HHmmss'Z'"));
    file_.setFileName(settings_.runsDir + "/" + stamp + "-" + id_ + ".jsonl");
    // A record that cannot be written must not stop the work: the person
    // asked for the work, not for the record.
    if (!file_.open(QIODevice::WriteOnly | QIODevice::Append))
        qWarning("levitate-agent: cannot write the record %s", qPrintable(file_.fileName()));
    // `pid` is the process that asked, as it always was; `host` is the process
    // the run lives in, which lets a later --list tell a run that is still
    // going from one that was killed before it could write its last line.
    QJsonObject request{{"record", "request"}, {"id", id_}, {"t", now()}, {"app", app}, {"title", title},
                        {"prompt", prompt}, {"agent", settings_.agent}, {"pid", starter}, {"host", qint64(getpid())}};
    for (auto field = extra.begin(); field != extra.end(); ++field)
        request[field.key()] = field.value();
    line(request);
}

QJsonObject Record::event(const QJsonObject &event)
{
    QJsonObject stamped = event;
    stamped["t"] = now();
    line(stamped);
    return stamped;
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

QList<QJsonObject> readRecord(const QString &path)
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
    if (pid <= 0 || (::kill(pid_t(pid), 0) != 0 && errno != EPERM))
        return false;
    // A process that has ended but not been collected is not there any more.
    QFile stat(QStringLiteral("/proc/%1/stat").arg(pid));
    if (stat.open(QIODevice::ReadOnly)) {
        const QByteArray text = stat.readAll();
        const qsizetype close = text.lastIndexOf(')');
        if (close > 0 && text.mid(close + 2, 1) == "Z")
            return false;
    }
    return true;
}

// The pid of a run's own process, if that is what it still is: the number may
// have been given to some other program since.
static bool hostIsAlive(const QJsonObject &request)
{
    // Records from before runs had a process of their own name only `pid`.
    const qint64 pid = request.contains("host") ? request["host"].toInteger() : request["pid"].toInteger();
    if (!alive(pid))
        return false;
    if (!request.contains("host"))
        return true;
    QFile cmdline(QStringLiteral("/proc/%1/cmdline").arg(pid));
    if (!cmdline.open(QIODevice::ReadOnly))
        return true; // cannot be looked at (not ours): trust the pid
    return cmdline.readAll().contains(request["id"].toString().toUtf8());
}

static QJsonObject summarise(const Settings &settings, const QList<QJsonObject> &lines)
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
        if (!hostIsAlive(request)) {
            // Killed without a last line: it is over, and not by finishing.
            state = QStringLiteral("stopped");
            ended = lines.last()["t"].toString();
        } else if (last == QLatin1String("question")) {
            state = QStringLiteral("waiting");
        }
    }
    // A window can attach only to a run that is going and has its socket.
    const bool attachable = ended.isEmpty() && sockets::exists(settings.socketsDir, socketName(request["id"].toString()));
    return {{"id", request["id"]},   {"app", request["app"]},         {"title", request["title"]},
            {"state", state},        {"started", request["t"]},       {"ended", ended.isEmpty() ? QJsonValue() : ended},
            {"summary", summary},    {"attachable", attachable}};
}

QJsonObject listRuns(const Settings &settings)
{
    // The file name starts with the time to the second; the request line has
    // it to the millisecond, which orders runs that began together.
    QList<QJsonObject> runs;
    const QStringList names = QDir(settings.runsDir).entryList({"*.jsonl"}, QDir::Files, QDir::Name | QDir::Reversed);
    for (const QString &name : names.mid(0, 200)) {
        const QList<QJsonObject> lines = readRecord(settings.runsDir + "/" + name);
        if (!lines.isEmpty() && lines.first()["record"] == QLatin1String("request"))
            runs << summarise(settings, lines);
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
        for (const QJsonObject &line : readRecord(settings.runsDir + "/" + name))
            all << line;
    }
    return all;
}

QString recordPath(const Settings &settings, const QString &id)
{
    if (id.isEmpty())
        return QString();
    const QStringList names = QDir(settings.runsDir).entryList({"*.jsonl"}, QDir::Files);
    for (const QString &name : names)
        if (name.endsWith("-" + id + ".jsonl"))
            return settings.runsDir + "/" + name;
    return QString();
}

qint64 liveHost(const Settings &settings, const QString &id)
{
    const QList<QJsonObject> lines = readRecord(recordPath(settings, id));
    if (lines.isEmpty() || lines.first()["record"] != QLatin1String("request"))
        return 0;
    for (const QJsonObject &line : lines)
        if (line["record"] == QLatin1String("end"))
            return 0;
    return hostIsAlive(lines.first()) ? lines.first()["host"].toInteger() : 0;
}
