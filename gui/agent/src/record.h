// The record of every run, kept as a file so that a later page can show what
// the agent did, and so that a run nobody watched can still be read.
#pragma once

#include "settings.h"

#include <QFile>
#include <QJsonArray>
#include <QJsonObject>
#include <QList>

class Record {
public:
    explicit Record(const Settings &settings) : settings_(settings) {}
    // A new run's id: eight hex digits, also the end of the file's name.
    static QString newId();
    // Creates the file and writes the request as its first line. `starter` is
    // the process that asked for the run (the window's helper): the apps match
    // it against their own process to know which run is theirs. `extra` is
    // added to the line (what the request asked for beyond the prompt).
    void open(const QString &id, const QString &app, const QString &title, const QString &prompt, qint64 starter,
              const QJsonObject &extra);
    // One event, with the time it was sent; returns it as written.
    QJsonObject event(const QJsonObject &event);
    // The last line: `done`, `failed` or `stopped`. Only the first call counts.
    void end(const QString &state);
    QString id() const { return id_; }
    QString path() const { return file_.fileName(); }

private:
    void line(const QJsonObject &object);
    const Settings &settings_;
    QFile file_;
    QString id_;
    bool ended_ = false;
};

QString now();
// `--list`: every run, newest first.
QJsonObject listRuns(const Settings &settings);
// `--show ID`: one run's lines, or an empty array when there is none.
QJsonArray showRun(const Settings &settings, const QString &id, bool *found);
// The record's lines, one object each (no object for a line that is not one).
QList<QJsonObject> readRecord(const QString &path);
// The file of the run with this id, or an empty string.
QString recordPath(const Settings &settings, const QString &id);
// The process of a run that is going: the one that hosts it. 0 when the run
// has ended or nobody is there any more.
qint64 liveHost(const Settings &settings, const QString &id);
// The name of a run's socket in `Settings::socketsDir`.
QString socketName(const QString &id);
