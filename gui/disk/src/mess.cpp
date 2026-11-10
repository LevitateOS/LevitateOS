#include "mess.h"

#include <QDateTime>
#include <QLocale>
#include <QObject>

#include "words.h"

#include <algorithm>

namespace {

enum Kind { Cache, Build, Recipe, Trash, OldDownload, Untouched, Picked };

const qint64 worthNaming = 20ll << 20; // smaller finds are not worth a line
const qint64 large = 256ll << 20;
const qint64 day = 86400;

struct Search {
    QString home;
    qint64 now;
    QList<Finding> found;

    void add(int kind, Node *node, const QString &path, const QString &what, const QString &why,
             const QString &how = QString(), bool sure = true)
    {
        if (node->bytes < worthNaming)
            return;
        node->kind = kind;
        found << Finding{kind, node, path, node->bytes, what, why, how, sure};
    }
};

QString when(qint64 seconds)
{
    return QLocale().toString(QDateTime::fromSecsSinceEpoch(seconds).date(), "MMMM yyyy");
}

bool looksLikeDownload(const QString &name)
{
    static const QStringList endings{".iso", ".img", ".rpm", ".deb", ".apk", ".zip", ".tar", ".tar.gz", ".tgz",
                                     ".tar.xz", ".tar.zst", ".tar.bz2", ".AppImage", ".run", ".exe", ".dmg"};
    for (const QString &ending : endings)
        if (name.endsWith(ending, Qt::CaseInsensitive))
            return true;
    return false;
}

// A build tree of the Rust compiler, wherever it was told to put it. Cargo
// marks these as caches; what is inside gives them away.
bool looksLikeRustOutput(const Node *node)
{
    for (const auto &child : node->children)
        if (child->dir
            && (child->name == QLatin1String("debug") || child->name == QLatin1String("release")
                || child->name.contains(QLatin1String("-unknown-"))))
            return true;
    return false;
}

void look(Node *node, const QString &path, Search &s)
{
    const QString name = node->name;
    Node *parent = node->parent;

    if (!node->dir) {
        if (node->files != 1)
            return; // the small files of a folder, taken together
        const bool inDownloads = path.startsWith(s.home + QStringLiteral("/Downloads/"));
        if (inDownloads && looksLikeDownload(name) && s.now - node->lastUsed > 90 * day)
            s.add(OldDownload, node, path, name,
                  QObject::tr("Downloaded and not opened since %1.").arg(when(node->lastUsed)), QString(), false);
        else if (node->bytes >= large && s.now - node->lastUsed > 365 * day)
            s.add(Untouched, node, path, name,
                  QObject::tr("Not opened or changed since %1.").arg(when(node->lastUsed)), QString(), false);
        return;
    }

    // Recipe's own folders are named first: they are cleared through Recipe.
    if (path == s.home + QStringLiteral("/.local/state/recipe") || path == QStringLiteral("/var/lib/recipe/state")) {
        s.add(Recipe, node, path, QObject::tr("Recipe's saved build steps"),
              QObject::tr("Kept so an interrupted or repeated build can continue. Recipe builds again what it needs."),
              QObject::tr("Clear it through Recipe, which knows what a running build still needs."));
        return;
    }
    if (name == QLatin1String("package-generations") && parent && parent->name.endsWith(QLatin1String("recipe"))) {
        s.add(Recipe, node, path, QObject::tr("Versions of installed software that Recipe keeps"),
              QObject::tr("Every version Recipe installed, the one in use among them."),
              QObject::tr("Ask Recipe which one is in use and clear only the others, through Recipe."), false);
        return;
    }
    if (path == s.home + QStringLiteral("/.cache/recipe")) {
        s.add(Recipe, node, path, QObject::tr("Recipe's build images"),
              QObject::tr("The system images Recipe builds in. It downloads them again when a build needs one."),
              QObject::tr("Clear it through Recipe."));
        return;
    }

    if (path == s.home + QStringLiteral("/.cache")) {
        for (auto &child : node->children) {
            const QString below = path + QLatin1Char('/') + child->name;
            if (child->dir && below == s.home + QStringLiteral("/.cache/recipe"))
                look(child.get(), below, s);
            else if (child->dir)
                s.add(Cache, child.get(), below, shortPath(below, s.home),
                      QObject::tr("Kept by a program to be faster. It makes it again when needed."));
        }
        return;
    }
    if (path == s.home + QStringLiteral("/.local/share/Trash")) {
        s.add(Trash, node, path, QObject::tr("The trash"), QObject::tr("Things you already threw away."),
              QObject::tr("Empty the trash."));
        return;
    }
    // What a program keeps for itself is not a leftover, even when it looks
    // like a project: an editor's own tools live here, for one.
    const bool programData = path.startsWith(s.home + QStringLiteral("/.local/share/"))
                             || path.startsWith(s.home + QStringLiteral("/.config/"))
                             || path.startsWith(s.home + QStringLiteral("/.var/"));
    if (programData && !(node->markers & Node::CacheTag)) {
        for (auto &child : node->children)
            look(child.get(), path + QLatin1Char('/') + child->name, s);
        return;
    }
    if (name == QLatin1String("node_modules") && parent && (parent->markers & Node::PackageJson)) {
        s.add(Build, node, path, shortPath(path, s.home),
              QObject::tr("Libraries fetched by the project's own tool, which fetches them again when the project is built."));
        return;
    }
    if (name == QLatin1String("target")
        && ((node->markers & Node::CacheTag) || (parent && (parent->markers & Node::CargoToml)))) {
        s.add(Build, node, path, shortPath(path, s.home),
              QObject::tr("Made by the Rust compiler from the source beside it, and made again by the next build."));
        return;
    }
    if (node->markers & Node::CMakeCache) {
        s.add(Build, node, path, shortPath(path, s.home),
              QObject::tr("Made by CMake from the source beside it, and made again by the next build."));
        return;
    }
    if (node->markers & Node::CacheTag) {
        if (looksLikeRustOutput(node))
            s.add(Build, node, path, shortPath(path, s.home),
                  QObject::tr("Made by the Rust compiler, and made again by the next build."));
        else
            s.add(Cache, node, path, shortPath(path, s.home),
                  QObject::tr("Marked as a cache by the program that made it."));
        return;
    }

    for (auto &child : node->children)
        look(child.get(), path + QLatin1Char('/') + child->name, s);
}

} // namespace

const QList<MessKind> &messKinds()
{
    static const QList<MessKind> kinds{
        {QObject::tr("Caches"), QColor(0xe0, 0xa0, 0x3a)},
        {QObject::tr("Build leftovers"), QColor(0x4f, 0x93, 0xd6)},
        {QObject::tr("Recipe's own"), QColor(0x5c, 0xb3, 0x7a)},
        {QObject::tr("The trash"), QColor(0x9a, 0x8f, 0x86)},
        {QObject::tr("Old downloads"), QColor(0xa7, 0x7b, 0xd1)},
        {QObject::tr("Large and untouched"), QColor(0xd6, 0x6f, 0x8f)},
        {QObject::tr("Picked by you"), QColor(0xdc, 0xd8, 0xcc)},
    };
    return kinds;
}

QList<Finding> findMess(Node *tree, const QString &home, qint64 now)
{
    Search search{home, now, {}};
    for (auto &root : tree->children)
        look(root.get(), root->name, search);
    std::sort(search.found.begin(), search.found.end(),
              [](const Finding &a, const Finding &b) { return a.bytes > b.bytes; });
    return search.found;
}

QString shortPath(const QString &path, const QString &home)
{
    if (path == home)
        return QStringLiteral("~");
    if (path.startsWith(home + QLatin1Char('/')))
        return QLatin1Char('~') + path.mid(home.size());
    return path;
}

Finding pickedByHand(Node *node, const QString &home, QList<Finding> &found)
{
    qint64 known = 0;
    int inside = 0;
    for (int index = found.size() - 1; index >= 0; --index)
        for (Node *up = found[index].node->parent; up; up = up->parent)
            if (up == node) {
                known += found[index].bytes;
                ++inside;
                found.removeAt(index);
                break;
            }
    node->kind = Picked;
    const QString path = node->path();
    QString why = QObject::tr("Picked by hand. It is not known what this is or whether it is still needed.");
    if (inside)
        why += QLatin1Char(' ')
               + QObject::tr("%1 of it, in %2, is known leftovers: caches, build output or the like.")
                     .arg(bytesText(known), counted(inside, QObject::tr("%1 place"), QObject::tr("%1 places")));
    return Finding{Picked, node, path, node->bytes, shortPath(path, home), why,
                   QObject::tr("Find out what it is and what takes the room in it, and tell me before removing anything."),
                   false};
}
