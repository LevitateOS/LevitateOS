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

// The places software can go on this computer, as the client knows them.
struct Places {
    // One explicit store (RECIPE_GUI_PACKAGE_ROOT): the only place, no choice.
    bool one = false;
    // The kernel command line says `recab.immutable`: the computer's software
    // goes into the next system and is in effect after a restart.
    bool locked = false;
    // The computer's store when it is not locked, or the explicit store.
    QString systemRoot;
    // The computer's changes go through a root helper that asks for the password.
    bool password = false;
};

class RecipeClient : public QObject {
    Q_OBJECT
public:
    explicit RecipeClient(QObject *parent = nullptr);

    // Read again every time: a check swaps the command line file between passes.
    Places places() const;

    // Reads the catalog from `list`, `installed`, `updates` and `storage`, one after the
    // other, without holding the window up. `done` gets the catalog, or the
    // problem when it could not be read.
    using Loaded = std::function<void(bool ok, const Catalog &catalog, const QString &problem)>;
    void load(Loaded done);
    bool loading() const { return loading_ > 0; }

    // Each change names the place it is made in (see model.h, `Place`).
    Run *install(const QString &name, Place place);
    Run *update(const QString &name, Place place);
    // The same install or update with Recipe's agent repair on, and the
    // prompt (issue, facts, expected result) handed to it.
    Run *fixWithAgent(const QString &name, const QString &prompt, bool installed, Place place);
    Run *remove(const QString &name, Place place);
    // Activates the newest earlier version a store still holds; no recipe
    // runs. Only where a store stands behind the software.
    Run *goBack(const QString &name, Place place);
    // Deletes every earlier version a store keeps. Nothing installed changes.
    Run *clearEarlier(Place place);

    // Sources. Fetching a source's list again is a person's own business;
    // adding and removing one changes the system's list and needs root.
    Run *refresh(const QString &source = QString());
    Run *addSource(const QString &addressOrFolder);
    Run *removeSource(const QString &name);

    // The agent, asked to write a recipe: the prompt on its input as one JSON
    // line, its questions answered on the same input. `title` is what the
    // Activity page lists the run under.
    Run *agent(const QString &prompt, const QString &title);
    // The program the window starts as its agent; the frame shows it.
    QString agentProgram() const { return agent_; }

private:
    using Answered = std::function<void(const QJsonObject &answer, const QString &problem)>;
    void query(const QStringList &arguments, Answered then);
    // `place`: the person's own store runs without the root helper.
    Run *change(const QStringList &arguments, Place place);
    // What goes before the command (`--system` on a locked computer) and
    // after it (the store), for a place.
    QStringList lead(Place place) const;
    QStringList at(Place place) const;
    QStringList collection() const;
    // The store whose sources are listed and fetched: the computer's.
    QStringList store() const;
    bool locked() const;
    QString program_, rootHelper_, recipesPath_, packageRoot_, systemRoot_, agent_;
    bool explicit_ = false;
    int loading_ = 0;
};
