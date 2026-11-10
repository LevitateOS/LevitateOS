// The only place that runs `recipe`. Reads go through as the desktop user;
// changes go through the root helper (pkexec), which shows the desktop's own
// password prompt.
#pragma once

#include "model.h"
#include "run.h"

#include <QObject>
#include <QProcess>

class RecipeClient : public QObject {
    Q_OBJECT
public:
    explicit RecipeClient(QObject *parent = nullptr);

    // Fills the catalog from `list`, `installed` and `updates`.
    bool load(Catalog *into, QString *error);

    Run *install(const QString &name);
    Run *update(const QString &name);
    // The same install or update with Recipe's agent repair on, and the
    // prompt (issue, facts, expected result) handed to it.
    Run *fixWithAgent(const QString &name, const QString &prompt, bool installed);
    Run *remove(const QString &name, bool clearKept);
    Run *goBack(const QString &name);

private:
    QJsonObject query(const QStringList &arguments, QString *error);
    Run *change(const QStringList &arguments);
    QStringList collection() const;
    QStringList store() const;
    QString program_, rootHelper_, recipesPath_, packageRoot_;
};
