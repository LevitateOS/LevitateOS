// The page for what the agent set aside: what it did with each place it was
// given, how much room is free before and now (measured on the disk), and the
// two ways out: bring places back, or let everything go.
#pragma once

#include "page.h"

#include <QList>
#include <QString>
#include <QStringList>

struct AfterRow {
    enum Kind {
        SetAside,    // in the holding folder
        Kept,        // the agent did not move it, and said why
        Left,        // left for Recipe to clear
        CouldNot,    // the agent tried and could not
        NoWord,      // the agent said nothing about it and it is not held
        Unrequested, // held, but not a place the person handed over
        Missing,     // the manifest names it and it is not in the holding folder
        BroughtBack, // it was set aside, and is where it was again
        LetGo,       // it was set aside, and the holding folder was let go
    };
    Kind kind = SetAside;
    QString path;
    qint64 bytes = 0;
    QString words; // what to say about it
};

struct AfterData {
    bool ranHere = false;         // an agent run of this window came before
    QList<AfterRow> rows;
    qint64 heldBytes = 0;         // what letting go frees
    int heldPlaces = 0;
    bool sameDisk = true;
    qint64 freeBefore = -1;       // -1: not measured in this window
    qint64 freeNow = -1;          // -1: could not be measured
    qint64 unlisted = 0;
    QStringList notes;            // what the last bring-back or let-go did
    QStringList problems;         // what could not be read
    bool letGone = false;
    QString home;                 // paths under it are written with ~
};

class AfterPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void showReading();
    void show(const AfterData &data);
    // What the page was last drawn from.
    const AfterData &shown() const { return shown_; }
signals:
    void back();
    void bringBack(const QString &path);
    void bringAllBack();
    void letGo();
    void neverSuggest(const QString &path);
    void tryAgain(const QString &path);
    void openActivity();
private:
    AfterData shown_;
};
