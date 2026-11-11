// What the front end knows about recipes: plain data read from `recipe`'s JSON.
// No package rules live here; Recipe stays the only thing that knows them.
#pragma once

#include "words.h"

#include <QDate>
#include <QJsonObject>
#include <QList>
#include <QMap>
#include <QString>
#include <QStringList>

struct Source {
    QString name, url, git, commit, sha256, checksumFrom;
    QString host() const;
    // How the source is pinned, as the end of a sentence.
    QString pin() const;
};

struct Needs {
    QStringList network;
    qint64 memoryMib = 0, storageMaxBytes = 0, timeoutSecs = 0, networkMaxBytes = 0;
};

// Where recipes come from: a repository whose index was fetched, or a folder
// on this computer. Recipe calls both a source.
struct Origin {
    QString name, title, kind, address, path, commit, problem;
    qint64 checked = 0; // when a remote source's index was last fetched, in seconds
    bool remote() const { return kind == QLatin1String("remote"); }
    // Its title; the two folders Recipe always reads have none and are named here.
    QString shown() const;
    // The address or the folder.
    QString where() const { return remote() ? address : path; }
    // "fetched today at 09:12", or why it could not be.
    QString state() const;
};

struct Recipe {
    QString origin; // the name of the source it is from
    QString name, version, description;
    // From ABOUT: the author's words.
    QString kind, category, title, details, homepage, license, afterInstall;
    QStringList programs, menu, keeps;
    QMap<QString, QString> notes;
    // Enforced.
    QList<Source> sources;
    Needs needs;
    QStringList deps, buildDeps, hooks;
    QString path;

    QString shown() const { return title.isEmpty() ? name : title; }
    bool listed() const { return kind != QLatin1String("library"); }
    static Recipe fromJson(const QJsonObject &object);
};

// Where software is. `Me` is the person's own store, in effect at once for
// them only. `Computer` is the system's: its store when the system is not
// locked, the running system's selection (through recab) when it is. `Next`
// is software that only the next system holds, after a restart. The one store
// a check names (RECIPE_GUI_PACKAGE_ROOT) is `Computer`, the only place there is.
enum class Place { Me = 0, Computer = 1, Next = 2 };

struct InstalledItem {
    // `kept`: the newest earlier version Recipe still holds, which "Go back" returns to.
    QString name, version, kept;
    // Every earlier version Recipe still holds, newest first (`kept` is the first).
    QStringList earlier;
    // `recipe hold` was run on it: `recipe update` leaves it as it is.
    bool held = false;
    QDate installedOn;
    qint64 sizeBytes = 0;
    QStringList programs, menu;
    // What the receipt says besides: every file it put in the profile, the
    // source its recipe was fetched from and the commit (both only when
    // recorded), and the recipe file it was installed from.
    QStringList files;
    QString source, commit, receiptRecipe;
    // The profile of the store it is in, as `installed --json` names it: the
    // programs and the menu entries are in its `bin` and `share/applications`.
    // Empty where no store stands behind it.
    QString profile;
    Place place = Place::Computer;
    // A store stands behind it: earlier versions, storage and going back apply.
    // False for what recab answers about a locked system, which says only
    // name and version.
    bool inStore = true;
    // On a locked system: the next system no longer holds it.
    bool leavesNext = false;
};

// One field of what a recipe may use (NEEDS) that the newer recipe changes,
// as `recipe updates` names it.
struct Change {
    QString field, before, after, kind; // kind: added, removed, raised, lowered, changed
    // The field in a person's words.
    QString title() const;
    // For a list (the network, what it reads and writes): what is new, what
    // is gone and what stays.
    QStringList added() const, removed() const, stays() const;
    bool isList() const;
    // An amount in a person's words: "8388608 bytes" as "8 MiB".
    static QString amount(const QString &value);
};

struct UpdateItem {
    QString name, from, to;
    Place place = Place::Computer;
    // Made from the collection's version, not by `recipe updates` (which
    // cannot look at a locked system's software).
    bool fromCollection = false;
    QList<Change> changes;
    // False when `recipe` cannot say what changed in what the recipe may use:
    // nothing was recorded when the installed version was installed.
    bool changesKnown = true;
    // The version is the same and the source publishes another recipe file.
    bool sameVersion = false;
    // `recipe updates` still lists a held package, marked held. It is not
    // counted and "Update all" leaves it.
    bool held = false;
    bool asksMore() const;
    // What it asks for beyond what the installed version had, in a few words:
    // "network, memory". Empty when nothing is more.
    QString asksMoreWords() const;
};

// One line of `recipe history`: what was done to a store. `from` and `to` are
// versions; any field that does not apply is empty.
struct HistoryEntry {
    qint64 at = 0; // seconds since 1970
    QString what;  // install, update, remove, go-back, clear-earlier, hold, unhold
    QString package, from, to, source;
};

// What one store takes, as `recipe storage` says, and what was done to it, as
// `recipe history` says (newest first).
struct Store {
    bool has = false;
    qint64 activeBytes = 0, earlierBytes = 0;
    int earlierGenerations = 0;
    QList<HistoryEntry> history;
    // `recipe history` answered (an empty history is an answer).
    bool historyKnown = false;
};

// A recipe `recipe list` could not read, and why.
struct RecipeProblem {
    QString path, problem;
};

struct Catalog {
    QList<Origin> origins;
    int unreadable = 0; // recipes in the collection that `recipe` could not read
    QList<RecipeProblem> unreadableRecipes;
    QList<Recipe> recipes;
    QList<InstalledItem> installed;
    QList<UpdateItem> updates;
    // Installed software `recipe updates` could not look up, each as "name: why".
    QStringList updateProblems;
    // What the store takes: what is installed, and the earlier versions kept
    // beside it (with what only they hold).
    // Summed over the stores there are; `stores` has each, by `Place`.
    qint64 activeBytes = 0, earlierBytes = 0;
    int earlierGenerations = 0;
    Store stores[2];
    // More than one place holds software (or can): rows say where they are.
    bool twoPlaces = false;
    // The computer's own software could not be read; why. Empty otherwise.
    QString computerProblem;
    // On a locked system, what the next system's changes name (`recab`'s
    // `preparation.changes`, readable by a person): package -> last action.
    QMap<QString, QString> nextChanges;

    const Recipe *recipe(const QString &name) const;
    const Origin *origin(const QString &name) const;
    // The name `recipe` is given: SOURCE:NAME, so two sources with one name cannot be confused.
    QString qualified(const QString &name) const;
    // Where the recipes are from, in a few words.
    QString originsWords() const;
    int countFrom(const QString &origin) const;
    // The first place that holds it (Me before Computer before Next), or the given one.
    const InstalledItem *installedItem(const QString &name) const;
    const InstalledItem *installedItem(const QString &name, Place place) const;
    const UpdateItem *update(const QString &name) const;
    const UpdateItem *update(const QString &name, Place place) const;
    // Updates that count: the held ones are listed but not counted.
    int updatesToCount() const;
    // The next system holds a change that installs or updates it (recab says so).
    bool waitsForRestart(const QString &name) const;
    // The places it could still be installed in, the computer's first.
    QList<Place> installPlaces(const QString &name) const;
    // Every place that holds it.
    QList<const InstalledItem *> installedEverywhere(const QString &name) const;
    QStringList categories() const;
};

QString categoryTitle(const QString &id);
QString hookTitle(const QString &hook);


// "For me", "For this computer", "In the next system, after a restart".
QString placeTitle(Place place);
