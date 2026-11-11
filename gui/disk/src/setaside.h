// What the agent sets aside instead of deleting, and the two things this
// window may do with it: bring a place back, or let everything go.
//
// The convention (it is written into the request the agent is handed):
//   ~/.local/share/levitate/set-aside/<yyyyMMdd-HHmmss>/        one holding folder per request
//     manifest.json    {"version": 1, "entries": [{"from": "/home/me/.cache/x", "held": "home/me/.cache/x", "bytes": 123}]}
//     home/me/.cache/x the place itself, moved here, under its own full path
//
// Everything that moves or deletes here is confined: only inside a holding
// folder, and only to or from the paths a manifest records, and only when a
// record agrees with the place it names. The functions below say why when
// they refuse.
#pragma once

#include <QList>
#include <QString>
#include <QStringList>

struct HeldPlace {
    QString batch;       // the holding folder's name: 20261111-140500
    QString from;        // where it was
    QString held;        // its path beneath the holding folder
    qint64 recorded = 0; // its size when it was set aside, as the agent wrote it
    qint64 bytes = 0;    // its size now, measured
    bool dir = false;
    bool present = false; // it is in the holding folder
    QString trouble;      // why this record cannot be acted on, if so
};

struct SetAside {
    QString root;
    QStringList batches;
    QList<HeldPlace> places;
    qint64 bytes = 0;     // everything in the holding folders, measured: what letting go frees
    qint64 unlisted = 0;  // of that, what no manifest records
    bool sameDisk = true; // the holding folders are on the disk the home folder is on
    QStringList problems; // what could not be read, in words
    bool empty() const { return batches.isEmpty(); }
};

QString holdingRoot(const QString &home);
// The holding folder for a request made now.
QString newHolding(const QString &home);

// Reads every manifest and measures what is held. It only reads; it may take
// a while on a big holding folder, so run it off the window's thread. `apparent`
// counts lengths, as the scan does.
SetAside readSetAside(const QString &home, bool apparent);

struct Outcome {
    bool ok = false;
    QString why; // when not ok, or a note when ok
    qint64 bytes = 0;
};

// Moves one place back to where it was. `zones` are the folders a place may
// belong to (the home folder and the other folders the window reads). Refuses,
// and says why, when something is there now, when the record does not agree
// with itself, or when the way there is not plain folders.
Outcome bringBack(const QString &home, const QStringList &zones, const HeldPlace &place);

// Deletes the holding folders and everything in them. Nothing else. `bytes`
// is what was deleted, counted as `readSetAside` counts it.
Outcome letGo(const QString &home, bool apparent);
