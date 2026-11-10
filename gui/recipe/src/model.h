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

struct InstalledItem {
    // `kept`: the newest earlier version Recipe still holds, which "Go back" returns to.
    QString name, version, kept;
    QDate installedOn;
    qint64 sizeBytes = 0;
    QStringList programs, menu;
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
    QList<Change> changes;
    // False when `recipe` cannot say what changed in what the recipe may use:
    // nothing was recorded when the installed version was installed.
    bool changesKnown = true;
    // The version is the same and the source publishes another recipe file.
    bool sameVersion = false;
    bool asksMore() const;
};

struct Catalog {
    QList<Origin> origins;
    int unreadable = 0; // recipes in the collection that `recipe` could not read
    QList<Recipe> recipes;
    QList<InstalledItem> installed;
    QList<UpdateItem> updates;
    // Installed software `recipe updates` could not look up, each as "name: why".
    QStringList updateProblems;
    // What the store takes: what is installed, and the earlier versions kept
    // beside it (with what only they hold).
    qint64 activeBytes = 0, earlierBytes = 0;
    int earlierGenerations = 0;

    const Recipe *recipe(const QString &name) const;
    const Origin *origin(const QString &name) const;
    // The name `recipe` is given: SOURCE:NAME, so two sources with one name cannot be confused.
    QString qualified(const QString &name) const;
    // Where the recipes are from, in a few words.
    QString originsWords() const;
    int countFrom(const QString &origin) const;
    const InstalledItem *installedItem(const QString &name) const;
    const UpdateItem *update(const QString &name) const;
    QStringList categories() const;
};

QString categoryTitle(const QString &id);
QString hookTitle(const QString &hook);

