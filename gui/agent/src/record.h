// The record of every run, kept as a file so that a later page can show what
// the agent did, and so that a run nobody watched can still be read.
#pragma once

#include "settings.h"

#include <QFile>
#include <QJsonArray>
#include <QJsonObject>

class Record {
public:
    explicit Record(const Settings &settings) : settings_(settings) {}
    // Creates the file and writes the request as its first line.
    void open(const QString &app, const QString &title, const QString &prompt);
    // One event, with the time it was sent.
    void event(const QJsonObject &event);
    // The last line: `done`, `failed` or `stopped`. Only the first call counts.
    void end(const QString &state);
    QString id() const { return id_; }

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
