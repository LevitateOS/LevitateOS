// Setting places aside: what the helper does when a request carries
// `set_aside`. The agent only decides (SET-ASIDE, KEEP, CANNOT lines); the move
// itself is made here, by the helper, so the agent never needs the right to
// write in the home folder for it.
//
// A request's `set_aside` is {"holding": "/abs/folder", "places": ["/abs", ...],
// "ask_first": ["/abs", ...]}. The holding folder and every place must lie
// inside the person's home folder. A place is moved only when it is exactly one
// of `places`, into the holding folder keeping its full path beneath it,
// without a shell and without following a link out of the home folder, and
// the manifest of the holding folder is kept true after every move (the format
// gui/disk reads: {"version": 1, "entries": [{"from", "held", "bytes"}]}).
#pragma once

#include <QHash>
#include <QJsonValue>
#include <QString>
#include <QStringList>

struct SetAsidePlan {
    bool given = false;
    QString holding;      // as given
    QString holdingReal;  // where it is, or will be, with links resolved
    QString homeReal;
    QStringList places;   // as given
    QStringList askFirst; // as given
    QHash<QString, QString> keyOf; // a place as given -> the place with the links of its folders resolved

    bool approved(const QString &given) const { return keyOf.contains(given); }
};

// Reads and checks the request's `set_aside`. On a problem returns false and
// says it in one plain sentence; the run then fails at once.
bool readSetAside(const QJsonValue &value, const QString &home, SetAsidePlan *plan, QString *problem);

// The place as given on the list that `path` is, once its folders are
// resolved; empty when it is not on the list. `path` may be written with a
// trailing slash or `.` parts, and a `~` is not understood.
QString approvedPlace(const SetAsidePlan &plan, const QString &path);

struct Moved {
    bool ok = false;
    QString why;   // when not ok
    qint64 bytes = 0;
};

// Moves one approved place (as given on the list) into the holding folder and
// records it in the manifest. Never replaces anything, never follows a link
// out of the home folder.
Moved moveIntoHolding(const SetAsidePlan &plan, const QString &given);
