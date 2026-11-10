// The two systems side by side, each as a map of blocks: how alike the
// running system and the other one are, and what differs. recab reads both
// and compares them; this only draws what recab answered.
#pragma once

#include "page.h"

#include <QDateTime>
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
    // What the other system is to this one, in recab's words: "copy" (made
    // as one), "previous" (the system from before the last change) or "next"
    // (holds a change that is not started yet).
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

// What recab says about the two systems without reading them through: quick
// to ask, and enough to say what is there before anything is compared.
struct SystemsState {
    bool known = false;
    QString problem; // why it is not known
    QString distro, runningRelease, otherRelease, otherRole;
    int revision = -1; // goes up with every change to either system

    static SystemsState fromJson(const QJsonObject &object);
    QString runningName() const;
    QString otherName() const;
    // "The system from before.", "The next system, not started yet."
    QString otherIs() const;
};

class SideFrame;

class SystemsPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // What is there, before anything was compared. `why` is why the last
    // comparison failed, if one did.
    void showState(const SystemsState &state, const QString &why = QString());
    // Both systems are being read: each side fills as it is read.
    void showComparing(const SystemsState &state);
    // An event of the running comparison (plan, step, progress).
    void progress(const QJsonObject &event);
    // `said` is what the agent answered when it was asked to explain, if it
    // was. `when` is when the systems were compared; `stale` says they have
    // changed since.
    void show(const Comparison &comparison, const QString &said, const QDateTime &when, bool stale);

signals:
    void compare();
    void stop();
    void handOver();
    void readAgain();
    void openSystem();

private:
    // Points at one block: on both maps, in the line under them, in the list.
    void point(Node *node, bool inList = true);

    void forget();

    std::shared_ptr<Blocks> blocks_;
    SideFrame *reading_[2] = {nullptr, nullptr};
    QLabel *step_ = nullptr;
    DiskMap *here_ = nullptr, *there_ = nullptr;
    QLabel *pointed_ = nullptr;
    QTreeWidget *list_ = nullptr;
};
