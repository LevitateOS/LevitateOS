// What the front end knows about recipes: plain data read from `recipe`'s JSON.
// No package rules live here; Recipe stays the only thing that knows them.
#pragma once

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

struct Recipe {
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
    QString name, version, kept;
    QDate installedOn;
    qint64 sizeBytes = 0, keptBytes = 0;
    QStringList programs, menu;
};

struct Change {
    QString field, before, after, kind; // kind: same, added, raised, lowered, changed
};

struct UpdateItem {
    QString name, from, to;
    QList<Change> changes;
    // False when `recipe` cannot say what changed in what the recipe may use.
    bool changesKnown = true;
    bool asksMore() const;
};

struct Catalog {
    QString sourceTitle, checked;
    int unreadable = 0; // recipes in the collection that `recipe` could not read
    QList<Recipe> recipes;
    QList<InstalledItem> installed;
    QList<UpdateItem> updates;

    const Recipe *recipe(const QString &name) const;
    const InstalledItem *installedItem(const QString &name) const;
    const UpdateItem *update(const QString &name) const;
    QStringList categories() const;
};

QString categoryTitle(const QString &id);
QString bytesText(qint64 bytes);
QString hookTitle(const QString &hook);
// "1 program" or "4 programs": the words for a count.
QString counted(qsizetype count, const QString &one, const QString &many);
