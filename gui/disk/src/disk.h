// "The disk": how full the place for the person's files is, what could be
// freed and where it is, every partition of the disk and what it holds, and
// whose room it is. Every number is measured (statvfs, lsblk) or read from
// the scan; nothing here is counted from a list.
#pragma once

#include "mess.h"
#include "page.h"
#include "scan.h"
#include "setaside.h"

#include <QColor>
#include <QDateTime>
#include <QJsonObject>
#include <QList>
#include <QString>

// One block device below the disk, as lsblk names it.
struct Partition {
    QString path, type, fstype, label, partlabel, parttype, mount;
    qint64 size = 0;
    qint64 total = 0;            // what the filesystem says it holds, at the mount point
    qint64 used = -1, free = -1; // measured at the mount point; -1: not read (not mounted)
};

struct DiskLayout {
    bool found = false;
    QString problem; // why the disk could not be named, when it could not
    QString disk;    // /dev/vda
    qint64 size = 0;
    QList<Partition> parts;
};

// `answer` is `lsblk --json --bytes` output. The disk is the one that holds
// `homeDevice` or has something mounted at `homeMount`. Mounted partitions
// are measured with statvfs here.
DiskLayout layoutFromLsblk(const QJsonObject &answer, const QString &homeDevice, const QString &homeMount);

// The room on a disk, measured: `free` is what a person can still use.
struct Space {
    bool known = false;
    qint64 total = 0, free = 0;
    QString problem;
};
Space spaceAt(const QString &path);

struct Slice {
    QString label;
    qint64 bytes = 0;
    QColor colour;
};

// What the page needs, all gathered by the window.
struct DiskInfo {
    Space home;
    QString homeDevice, homeMount;

    // The scan
    bool scanning = true;
    qint64 scanFiles = 0, scanBytes = 0;
    qint64 scannedBytes = 0;
    int unreadable = 0;
    bool apparent = false;
    QList<Finding> found;
    QList<Slice> slices;

    // What could be freed
    qint64 earlierBytes = -1; // Recipe's earlier versions; -1: Recipe could not be asked
    qint64 earlierGenerations = 0;
    bool heldRead = false;
    SetAside held;

    // The partitions
    bool layoutRead = false;
    DiskLayout layout;
    QString runningDevice, otherDevice, otherRole;
    bool compared = false;
    qint64 otherFilesBytes = 0;
    QString otherDeviceCompared;
    QDateTime comparedWhen;
};

// The scan split into kinds it can tell apart: files, what programs keep in
// hidden folders, known leftovers, what the agent set aside, other folders.
QList<Slice> composition(Node *tree, const QList<Finding> &found, const QString &home, qint64 heldBytes);

class DiskPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const DiskInfo &info);
    // The heading as shown, for a check.
    QString headingText() const { return heading_; }
    // What the page was last drawn from.
    const DiskInfo &shown() const { return shown_; }
signals:
    void lookAgain();
    void showMess();
    void showHeld();
    void showBrowse();
    void showSystems();
    void openRecipe();
private:
    QString heading_;
    DiskInfo shown_;
};
