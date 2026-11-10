#include "scan.h"

#include <QFile>

#include <algorithm>
#include <cstring>
#include <set>
#include <utility>

#include <dirent.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>

namespace {

const qint64 ownEntry = 1 << 20; // a file this big or bigger is shown by itself

struct Walk {
    bool apparent;
    dev_t device;
    std::set<std::pair<dev_t, ino_t>> linked; // files with several names, counted once
    std::atomic<qint64> *files, *bytes;
    QStringList *unreadable;
    const QThread *thread;
};

void sortBiggestFirst(Node *node)
{
    std::sort(node->children.begin(), node->children.end(),
              [](const std::unique_ptr<Node> &a, const std::unique_ptr<Node> &b) { return a->bytes > b->bytes; });
}

// `fd` is the open folder; it is closed here.
void walk(int fd, Node *node, const QString &path, Walk &w)
{
    DIR *dir = fdopendir(fd);
    if (!dir) {
        close(fd);
        *w.unreadable << path;
        return;
    }
    qint64 smallBytes = 0, smallFiles = 0, smallUsed = 0;
    while (dirent *entry = readdir(dir)) {
        const char *name = entry->d_name;
        if (!std::strcmp(name, ".") || !std::strcmp(name, ".."))
            continue;
        if (w.thread->isInterruptionRequested())
            break;
        struct stat st;
        if (fstatat(dirfd(dir), name, &st, AT_SYMLINK_NOFOLLOW) != 0)
            continue;

        if (S_ISDIR(st.st_mode)) {
            if (st.st_dev != w.device)
                continue; // another filesystem is mounted here
            auto child = std::make_unique<Node>();
            child->name = QFile::decodeName(name);
            child->dir = true;
            child->parent = node;
            const int below = openat(dirfd(dir), name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
            const QString childPath = path + QLatin1Char('/') + child->name;
            if (below < 0)
                *w.unreadable << childPath;
            else
                walk(below, child.get(), childPath, w);
            node->bytes += child->bytes;
            node->files += child->files;
            node->children.push_back(std::move(child));
            continue;
        }

        if (!std::strcmp(name, "CACHEDIR.TAG"))
            node->markers |= Node::CacheTag;
        else if (!std::strcmp(name, "CMakeCache.txt"))
            node->markers |= Node::CMakeCache;
        else if (!std::strcmp(name, "Cargo.toml"))
            node->markers |= Node::CargoToml;
        else if (!std::strcmp(name, "package.json"))
            node->markers |= Node::PackageJson;

        if (st.st_nlink > 1 && !w.linked.insert({st.st_dev, st.st_ino}).second)
            continue;
        const qint64 size = w.apparent || !S_ISREG(st.st_mode) ? qint64(st.st_size) : qint64(st.st_blocks) * 512;
        const qint64 used = std::max(qint64(st.st_atime), qint64(st.st_mtime));
        if (size >= ownEntry) {
            auto child = std::make_unique<Node>();
            child->name = QFile::decodeName(name);
            child->bytes = size;
            child->files = 1;
            child->lastUsed = used;
            child->parent = node;
            node->children.push_back(std::move(child));
        } else {
            smallBytes += size;
            ++smallFiles;
            smallUsed = std::max(smallUsed, used);
        }
        node->bytes += size;
        ++node->files;
        ++*w.files;
        *w.bytes += size;
    }
    closedir(dir);

    if (smallFiles) {
        auto small = std::make_unique<Node>();
        small->name = QStringLiteral("(%1 small files)").arg(smallFiles);
        small->bytes = smallBytes;
        small->files = smallFiles;
        small->lastUsed = smallUsed;
        small->parent = node;
        node->children.push_back(std::move(small));
    }
    sortBiggestFirst(node);
}

} // namespace

QString Node::path() const
{
    if (!parent)
        return name;
    const QString above = parent->path();
    return above.isEmpty() ? name : above + QLatin1Char('/') + name;
}

Scanner::Scanner(const QStringList &roots, bool apparent, QObject *parent)
    : QThread(parent)
    , roots_(roots)
    , apparent_(apparent)
{
}

void Scanner::run()
{
    auto top = std::make_shared<Node>();
    top->dir = true;
    for (const QString &root : roots_) {
        const QByteArray local = QFile::encodeName(root);
        struct stat st;
        const int fd = open(local.constData(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
        if (fd < 0 || fstat(fd, &st) != 0) {
            if (fd >= 0)
                close(fd);
            unreadable_ << root;
            continue;
        }
        auto child = std::make_unique<Node>();
        child->name = root;
        child->dir = true;
        child->parent = top.get();
        Walk w{apparent_, st.st_dev, {}, &files_, &bytes_, &unreadable_, this};
        walk(fd, child.get(), root, w);
        top->bytes += child->bytes;
        top->files += child->files;
        top->children.push_back(std::move(child));
    }
    sortBiggestFirst(top.get());
    tree_ = top;
}
