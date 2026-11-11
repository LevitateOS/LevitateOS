// The rules that recognise mess in a scanned tree: things a program can make
// again, things already thrown away, and big things nobody has touched. A
// rule only names what it found and why; nothing is removed here.
#pragma once

#include "scan.h"

#include <QColor>
#include <QList>
#include <QString>
#include <QStringList>

// The kind of a place the person picked by hand (the last of messKinds()).
inline constexpr int kindPicked = 6;
// Recipe's own folders: listed, never handed over (Recipe has no command to clear them yet).
inline constexpr int kindRecipe = 2;

struct MessKind {
    QString title;
    QColor colour;
};

struct Finding {
    int kind = 0;
    Node *node = nullptr;
    QString path;
    qint64 bytes = 0;
    QString what; // "Build output of recab"
    QString why;  // one sentence: why this can go
    QString how;  // how it should be cleared, when not simply by removing it
    bool sure = true; // false: the agent asks before it removes this
};

const QList<MessKind> &messKinds();

// Looks through the tree, marks each found node with its kind, and returns
// the findings, biggest first. `home` is the person's home folder; `now` is
// the present time in seconds.
// `never` holds the places the person said never to suggest: a finding at or
// inside one of them is not made.
QList<Finding> findMess(Node *tree, const QString &home, qint64 now, const QStringList &never = {});

// The finding that is this node or holds it, or null.
const Finding *findingAround(const QList<Finding> &found, Node *node);

// What a kind of mess is, in a few plain words, for the line over its places.
QString kindWords(int kind);
// When something was last used, as a person says it: "today", "3 weeks ago",
// "a year ago". 0 (not recorded) says so.
QString lastUsedText(qint64 when, qint64 now);

// Something the person pointed at on the map themselves. The rules do not
// know what it is, so the agent is told to find out and ask. Findings that
// lie inside it are taken out of `found`: the picked thing stands for them.
Finding pickedByHand(Node *node, const QString &home, QList<Finding> &found);

// A path as a person reads it: the home folder is written as ~.
QString shortPath(const QString &path, const QString &home);
