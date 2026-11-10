// The rules that recognise mess in a scanned tree: things a program can make
// again, things already thrown away, and big things nobody has touched. A
// rule only names what it found and why; nothing is removed here.
#pragma once

#include "scan.h"

#include <QColor>
#include <QList>
#include <QString>

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
QList<Finding> findMess(Node *tree, const QString &home, qint64 now);

// Something the person pointed at on the map themselves. The rules do not
// know what it is, so the agent is told to find out and ask. Findings that
// lie inside it are taken out of `found`: the picked thing stands for them.
Finding pickedByHand(Node *node, const QString &home, QList<Finding> &found);

// A path as a person reads it: the home folder is written as ~.
QString shortPath(const QString &path, const QString &home);
