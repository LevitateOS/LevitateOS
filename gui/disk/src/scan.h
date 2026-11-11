// Reads a folder tree from the real disk into memory: every folder, every
// file of 1 MiB or more, and the small files of each folder as one entry.
// It only reads. It does not follow links or cross into another filesystem.
#pragma once

#include <QString>
#include <QStringList>
#include <QThread>

#include <atomic>
#include <memory>
#include <vector>

struct Node {
    // Names seen directly inside a folder that say what the folder is.
    enum Marker { CacheTag = 1, CMakeCache = 2, CargoToml = 4, PackageJson = 8 };

    QString name;
    qint64 bytes = 0;
    qint64 files = 0;    // how many files this stands for
    qint64 lastUsed = 0; // a file: the later of last read and last change, in seconds
    bool dir = false;
    // The small files of one folder taken together, not a file of its own.
    bool aggregate = false;
    int markers = 0;
    int kind = -1; // which kind of mess this is, once the rules have looked; -1: none
    Node *parent = nullptr;
    std::vector<std::unique_ptr<Node>> children; // biggest first

    QString path() const;
    // The latest time anything below this was read or changed, in seconds; 0
    // when the scan recorded nothing with a time.
    qint64 newest() const;
};

class Scanner : public QThread {
    Q_OBJECT
public:
    // `apparent` counts each file's length instead of the room it takes on
    // disk; a sample tree of empty-but-long files needs it.
    Scanner(const QStringList &roots, bool apparent, QObject *parent = nullptr);

    // The tree, once finished: a nameless top with one child per root.
    std::shared_ptr<Node> tree() const { return tree_; }
    QStringList unreadable() const { return unreadable_; }
    qint64 files() const { return files_; }
    qint64 bytes() const { return bytes_; }

protected:
    void run() override;

private:
    QStringList roots_;
    bool apparent_;
    std::shared_ptr<Node> tree_;
    QStringList unreadable_;
    std::atomic<qint64> files_{0}, bytes_{0};
};
