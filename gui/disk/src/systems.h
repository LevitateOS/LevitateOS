// The two systems side by side, each as a map of blocks: how alike the
// running system and the other one are, and what differs. recab reads both
// and compares them; this only draws what recab answered.
#pragma once

#include "page.h"

#include <QJsonObject>
#include <QList>
#include <QStringList>

#include <memory>

class DiskMap;
class QLabel;
class QTreeWidget;
struct Blocks; // both systems as one tree of blocks, so both maps share a layout
struct Node;

struct Side {
    QString slot, device;
    qint64 files = 0, bytes = 0;
};

struct Difference {
    enum Change { OnlyRunning, OnlyOther, Content, Metadata };
    QString path;
    Change change = Content;
    qint64 runningBytes = 0, otherBytes = 0;
    QString detail; // for Metadata: what differs, in recab's words
};

// A folder's whole size in each system.
struct FolderSize {
    QString path;
    qint64 runningBytes = 0, otherBytes = 0;
};

struct Comparison {
    Side running, other;
    // What the other system is to this one: "copy" (made as one), "before"
    // (the system from before the last change) or "next" (holds a change
    // that is not started yet).
    QString role;
    qint64 sameFiles = 0, sameBytes = 0;
    QList<Difference> differences;
    QList<FolderSize> folders; // what the maps are drawn from
    QStringList own;  // files that differ by design: each system names its own partition
    qint64 more = 0;  // differences recab left out of the list

    // Reads recab's answer. An empty `error` means it was understood.
    static Comparison fromJson(const QJsonObject &object, QString *error);
    bool identical() const { return differences.isEmpty() && more == 0; }
    qint64 count() const { return differences.size() + more; }
    // The room taken by what differs: for each difference, the bigger side.
    qint64 differingBytes() const;
    // How much of the two systems is the same, by room taken, 0 to 1.
    double alike() const;
    // One sentence on what the other system is.
    QString roleWords() const;
    // The issue, facts and expected result for an agent asked to explain.
    QString issue() const;
    QStringList facts() const;
    QString expected() const;
};

class SystemsPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // Before anything was read. `why` is why the last try failed, if it did.
    void showStart(const QString &why = QString());
    // `said` is what the agent answered when it was asked to explain, if it was.
    void show(const Comparison &comparison, const QString &said = QString());

signals:
    void compare();
    void handOver();

private:
    // Points at one block: on both maps, in the line under them, in the list.
    void point(Node *node, bool inList = true);

    std::shared_ptr<Blocks> blocks_;
    DiskMap *here_ = nullptr, *there_ = nullptr;
    QLabel *pointed_ = nullptr;
    QTreeWidget *list_ = nullptr;
};
