// The only place that runs `recipe`. Reads go through as the desktop user;
// changes go through the root helper (pkexec), which shows the desktop's own
// password prompt.
#pragma once

#include "model.h"
#include "run.h"

#include <QObject>
#include <QProcess>

#include <functional>
#include <memory>

class RecipeClient : public QObject {
    Q_OBJECT
public:
    explicit RecipeClient(QObject *parent = nullptr);

    // Reads the catalog from `list`, `installed`, `updates` and `storage`, one after the
    // other, without holding the window up. `done` gets the catalog, or the
    // problem when it could not be read.
    using Loaded = std::function<void(bool ok, const Catalog &catalog, const QString &problem)>;
    void load(Loaded done);
    bool loading() const { return loading_ > 0; }

    Run *install(const QString &name);
    Run *update(const QString &name);
    // The same install or update with Recipe's agent repair on, and the
    // prompt (issue, facts, expected result) handed to it.
    Run *fixWithAgent(const QString &name, const QString &prompt, bool installed);
    Run *remove(const QString &name);
    // Activates the newest earlier version Recipe still holds; no recipe runs.
    Run *goBack(const QString &name);
    // Deletes every earlier version Recipe keeps. Nothing installed changes.
    Run *clearEarlier();

    // Sources. Fetching a source's list again is a person's own business;
    // adding and removing one changes the system's list and needs root.
    Run *refresh(const QString &source = QString());
    Run *addSource(const QString &addressOrFolder);
    Run *removeSource(const QString &name);

    // The agent, asked to write a recipe: the prompt on its input as one JSON
    // line, its questions answered on the same input.
    Run *agent(const QString &prompt);
    QString agentName() const;

private:
    using Answered = std::function<void(const QJsonObject &answer, const QString &problem)>;
    void query(const QStringList &arguments, Answered then);
    Run *change(const QStringList &arguments);
    QStringList collection() const;
    QStringList store() const;
    QString program_, rootHelper_, recipesPath_, packageRoot_, agent_;
    int loading_ = 0;
};
