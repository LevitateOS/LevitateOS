// The only place that runs `recipe`. Reads go through as the desktop user;
// changes go through the root helper (pkexec), which shows the desktop's own
// password prompt.
#pragma once

#include "model.h"

#include <QObject>
#include <QProcess>

// One running `recipe` command that changes something. It reports the JSON
// events `recipe --machine-events` writes to stderr, one object per line; any
// other line is passed on as output.
class Run : public QObject {
    Q_OBJECT
public:
    Run(const QString &program, const QStringList &arguments, QObject *parent);
    void start();
    void stop();
    QJsonObject failure() const { return failure_; }

signals:
    void event(const QJsonObject &event);
    void output(const QString &line);
    void finished(bool ok);

private:
    void drain();
    QProcess process_;
    QByteArray pending_;
    QJsonObject failure_;
};

class RecipeClient : public QObject {
    Q_OBJECT
public:
    explicit RecipeClient(QObject *parent = nullptr);

    // Fills the catalog from `list`, `installed` and `updates`.
    bool load(Catalog *into, QString *error);

    Run *install(const QString &name);
    Run *remove(const QString &name, bool clearKept);
    Run *goBack(const QString &name);

private:
    QJsonObject query(const QStringList &arguments, QString *error);
    Run *change(const QStringList &arguments);
    QString program_, rootHelper_;
};
