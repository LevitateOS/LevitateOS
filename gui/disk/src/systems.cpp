#include "systems.h"

#include "map.h"
#include "parts.h"
#include "scan.h"
#include "words.h"

#include <QFile>
#include <QHash>
#include <QHeaderView>
#include <QJsonArray>
#include <QLabel>
#include <QLocale>
#include <QMap>
#include <QPainter>
#include <QPushButton>
#include <QSignalBlocker>
#include <QScrollArea>
#include <QTreeWidget>
#include <QTreeWidgetItemIterator>
#include <QVBoxLayout>

#include <algorithm>
#include <cmath>

namespace {

struct ChangeKind {
    QString words;  // in a table cell
    QString brief;  // under the bar
    QString inFact; // in a sentence to the agent
    QColor colour;
};

const QList<ChangeKind> &changeKinds()
{
    static const QList<ChangeKind> kinds{
        {QObject::tr("Only in this system"), QObject::tr("only here"), QObject::tr("only in the running system"),
         QColor(0x4f, 0x93, 0xd6)},
        {QObject::tr("Only in the other"), QObject::tr("only in the other"), QObject::tr("only in the other system"),
         QColor(0xa7, 0x7b, 0xd1)},
        {QObject::tr("Different content"), QObject::tr("different content"), QObject::tr("different content"),
         QColor(0xe0, 0xa0, 0x3a)},
        {QObject::tr("Same content, other settings"), QObject::tr("other settings"),
         QObject::tr("same content, different settings"), QColor(0x9a, 0x8f, 0x86)},
    };
    return kinds;
}

QIcon swatch(const QColor &colour)
{
    QPixmap square(12, 12);
    square.fill(colour);
    return QIcon(square);
}

QString sized(qint64 bytes)
{
    return bytes > 0 ? bytesText(bytes) : QStringLiteral("—");
}

// The folder a difference is listed under: two levels down when the file is
// deeper than that (/usr/bin, /etc/ssh), else its own folder (/etc).
QString groupOf(const QString &path)
{
    const QStringList parts = path.split(QLatin1Char('/'), Qt::SkipEmptyParts);
    if (parts.size() <= 1)
        return QStringLiteral("/");
    return QLatin1Char('/') + parts.mid(0, parts.size() > 2 ? 2 : 1).join(QLatin1Char('/'));
}

Side sideFrom(const QJsonObject &object)
{
    return {object["slot"].toString(), object["device"].toString(), object["files"].toInteger(),
            object["bytes"].toInteger()};
}

QString parentOf(const QString &path)
{
    const qsizetype slash = path.lastIndexOf(QLatin1Char('/'));
    return slash <= 0 ? QStringLiteral("/") : path.left(slash);
}

bool hasFolders(const Node *node)
{
    for (const auto &child : node->children)
        if (child->dir)
            return true;
    return false;
}

} // namespace

// What is known about one block, in both systems.
struct BlockInfo {
    QString path;
    qint64 running = 0, other = 0;
    int counts[4] = {0, 0, 0, 0}; // differences in it, by kind
    int differing() const { return counts[0] + counts[1] + counts[2] + counts[3]; }
};

// Both systems as one tree. A block's place and area come from the bigger of
// its two sizes, so the two maps have the same layout and can be compared by
// eye; a block that holds a difference is at least big enough to be seen.
struct Blocks {
    std::shared_ptr<Node> top;
    QHash<const Node *, BlockInfo> info;
    QHash<QString, Node *> folder; // by path
    QHash<QString, Node *> blockOf; // for each difference's path: the block it is drawn in

    Node *ensure(const QString &path)
    {
        if (Node *known = folder.value(path))
            return known;
        Node *parent = path == QLatin1String("/") ? top.get() : ensure(parentOf(path));
        auto node = std::make_unique<Node>();
        node->dir = true;
        node->name = path == QLatin1String("/") ? path : path.section(QLatin1Char('/'), -1);
        node->parent = parent;
        Node *raw = node.get();
        parent->children.push_back(std::move(node));
        folder.insert(path, raw);
        info[raw].path = path;
        return raw;
    }

    // The files directly in a folder that also has folders: one block of their own.
    Node *filesOf(Node *of)
    {
        for (const auto &child : of->children)
            if (!child->dir)
                return child.get();
        auto node = std::make_unique<Node>();
        node->name = QObject::tr("(files)");
        node->parent = of;
        Node *raw = node.get();
        of->children.push_back(std::move(node));
        info[raw].path = info[of].path;
        return raw;
    }

    // Sets every block's area and adds the differences up into the folders.
    qint64 weigh(Node *node, qint64 least)
    {
        BlockInfo &mine = info[node];
        if (node->children.empty()) {
            node->bytes = std::max(mine.running, mine.other);
            if (mine.differing())
                node->bytes = std::max(node->bytes, least);
            return node->bytes;
        }
        node->bytes = 0;
        for (auto &child : node->children) {
            node->bytes += weigh(child.get(), least);
            for (int kind = 0; kind < 4; ++kind)
                mine.counts[kind] += info[child.get()].counts[kind];
        }
        std::sort(node->children.begin(), node->children.end(),
                  [](const auto &a, const auto &b) { return a->bytes > b->bytes; });
        return node->bytes;
    }

    static std::shared_ptr<Blocks> from(const Comparison &c)
    {
        auto blocks = std::make_shared<Blocks>();
        blocks->top = std::make_shared<Node>();
        blocks->top->dir = true;
        for (const FolderSize &size : c.folders) {
            Node *node = blocks->ensure(size.path);
            blocks->info[node].running = size.runningBytes;
            blocks->info[node].other = size.otherBytes;
        }
        const QList<Node *> folders = blocks->folder.values();
        for (Node *node : folders) {
            if (!hasFolders(node))
                continue;
            qint64 running = blocks->info[node].running, other = blocks->info[node].other;
            for (const auto &child : node->children) {
                running -= blocks->info[child.get()].running;
                other -= blocks->info[child.get()].other;
            }
            if (running > 0 || other > 0) {
                Node *files = blocks->filesOf(node);
                blocks->info[files].running = std::max<qint64>(0, running);
                blocks->info[files].other = std::max<qint64>(0, other);
            }
        }
        for (const Difference &difference : c.differences) {
            QString path = parentOf(difference.path);
            while (!blocks->folder.contains(path) && path != QLatin1String("/"))
                path = parentOf(path);
            Node *in = blocks->ensure(path);
            Node *block = hasFolders(in) ? blocks->filesOf(in) : in;
            ++blocks->info[block].counts[difference.change];
            blocks->blockOf.insert(difference.path, block);
        }
        const BlockInfo whole = blocks->info.value(blocks->folder.value(QStringLiteral("/")));
        blocks->weigh(blocks->top.get(), std::max(whole.running, whole.other) / 300);
        return blocks;
    }

    // "3 only here, 1 different content", or that it is the same in both.
    QString differencesIn(const Node *node) const
    {
        const BlockInfo &about = info[node];
        if (!about.differing())
            return QObject::tr("the same in both");
        QStringList parts;
        for (int kind = 0; kind < 4; ++kind)
            if (about.counts[kind])
                parts << QObject::tr("%1 %2").arg(about.counts[kind]).arg(changeKinds().at(kind).brief);
        return parts.join(QObject::tr(", "));
    }

    QString nameOf(const Node *node) const
    {
        const BlockInfo &about = info[node];
        return node->dir ? about.path : QObject::tr("the files directly in %1").arg(about.path);
    }
};

namespace {

// One system as a map. Both maps are given the same tree, so a block is in
// the same place in both; each draws its own side of it.
class SideMap : public DiskMap {
public:
    SideMap(bool running, std::shared_ptr<Blocks> blocks)
        : running_(running)
        , blocks_(std::move(blocks))
    {
        setTree(blocks_->top);
    }

protected:
    // The kind of difference this side shows for a block, or -1: the kind
    // with the most files among those that can be seen from this side.
    int shown(const Node *node) const
    {
        const BlockInfo &about = blocks_->info[node];
        const int mine = running_ ? Difference::OnlyRunning : Difference::OnlyOther;
        int best = -1;
        for (int kind : {int(Difference::Content), mine, int(Difference::Metadata)})
            if (about.counts[kind] > 0 && (best < 0 || about.counts[kind] > about.counts[best]))
                best = kind;
        return best;
    }
    // How many files the other side has here that this side lacks.
    int missing(const Node *node) const
    {
        return blocks_->info[node].counts[running_ ? Difference::OnlyOther : Difference::OnlyRunning];
    }

    bool loud(const Node *node) const override { return shown(node) >= 0; }
    QColor fillOf(const Node *node, int depth) const override
    {
        const int kind = shown(node);
        return kind >= 0 ? changeKinds().at(kind).colour : DiskMap::fillOf(node, depth);
    }
    qint64 sizeOf(const Node *node) const override
    {
        return running_ ? blocks_->info[node].running : blocks_->info[node].other;
    }
    QString tipOf(const Node *node) const override
    {
        const BlockInfo &about = blocks_->info[node];
        return QObject::tr("%1\n%2 here, %3 in the other\n%4")
            .arg(blocks_->nameOf(node), bytesText(about.running), bytesText(about.other), blocks_->differencesIn(node));
    }
    void decorate(QPainter &painter, const Node *node, const QRectF &rect) const override
    {
        const BlockInfo &about = blocks_->info[node];
        const int kind = shown(node);
        if (kind < 0 && missing(node) > 0) {
            // Nothing differs that this side has, but the other has more here.
            const int theirs = running_ ? Difference::OnlyOther : Difference::OnlyRunning;
            painter.setPen(QPen(changeKinds().at(theirs).colour, 2, Qt::DashLine));
            painter.drawRect(rect.adjusted(1.5, 1.5, -1.5, -1.5));
            return;
        }
        if (kind < 0 || rect.width() < 18 || rect.height() < 15)
            return;
        // How many files differ here, as seen from this side.
        const int mine = running_ ? Difference::OnlyRunning : Difference::OnlyOther;
        const int count = about.counts[Difference::Content] + about.counts[mine] + about.counts[Difference::Metadata];
        QFont small = font();
        small.setPointSizeF(small.pointSizeF() * 0.85);
        small.setBold(true);
        painter.setFont(small);
        painter.setPen(QColor(0x14, 0x18, 0x16));
        painter.drawText(rect.adjusted(3, 2, -4, -2), Qt::AlignRight | Qt::AlignBottom, QString::number(count));
    }

private:
    bool running_;
    std::shared_ptr<Blocks> blocks_;
};

// The name of one system over its map.
QLayout *sideTitle(const QString &title, const QString &what, const Side &side)
{
    auto *column = new QVBoxLayout;
    column->setSpacing(0);
    column->addWidget(strong(title));
    column->addWidget(dim(text(QObject::tr("%1 %2 in %3, on %4")
                                   .arg(what, bytesText(side.bytes),
                                        counted(side.files, QObject::tr("%1 file"), QObject::tr("%1 files")), side.device))));
    return column;
}

} // namespace

// ---------------------------------------------------------------- What recab answered

Comparison Comparison::fromJson(const QJsonObject &object, QString *error)
{
    Comparison c;
    if (object["schema_version"].toInt() != 1) {
        *error = QObject::tr("recab answered in a form this window does not know (schema %1; it reads schema 1).")
                     .arg(object["schema_version"].toInt());
        return c;
    }
    c.running = sideFrom(object["running"].toObject());
    c.other = sideFrom(object["other"].toObject());
    c.role = object["other"].toObject()["role"].toString();
    c.sameFiles = object["same"].toObject()["files"].toInteger();
    c.sameBytes = object["same"].toObject()["bytes"].toInteger();
    c.more = object["more"].toInteger();
    for (const QJsonValue &value : object["own"].toArray())
        c.own << value.toString();
    for (const QJsonValue &value : object["folders"].toArray()) {
        const QJsonObject one = value.toObject();
        c.folders << FolderSize{one["path"].toString(), one["running_bytes"].toInteger(), one["other_bytes"].toInteger()};
    }
    if (c.folders.isEmpty()) {
        *error = QObject::tr("recab did not say what the two systems hold, folder by folder, so they cannot be drawn.");
        return c;
    }
    static const QMap<QString, Difference::Change> changes{{"only-running", Difference::OnlyRunning},
                                                           {"only-other", Difference::OnlyOther},
                                                           {"content", Difference::Content},
                                                           {"metadata", Difference::Metadata}};
    for (const QJsonValue &value : object["differences"].toArray()) {
        const QJsonObject one = value.toObject();
        if (!changes.contains(one["change"].toString())) {
            *error = QObject::tr("recab named a kind of difference this window does not know: %1")
                         .arg(one["change"].toString());
            return c;
        }
        c.differences << Difference{one["path"].toString(), changes.value(one["change"].toString()),
                                    one["running_bytes"].toInteger(), one["other_bytes"].toInteger(),
                                    one["detail"].toString()};
    }
    return c;
}

qint64 Comparison::differingBytes() const
{
    qint64 total = 0;
    for (const Difference &difference : differences)
        total += std::max(difference.runningBytes, difference.otherBytes);
    return total;
}

double Comparison::alike() const
{
    const qint64 all = sameBytes + differingBytes();
    return all > 0 ? double(sameBytes) / double(all) : 1.0;
}

QString Comparison::roleWords() const
{
    if (role == QLatin1String("previous"))
        return QObject::tr("The other system is the one from before your last change. What differs is that change.");
    if (role == QLatin1String("next"))
        return QObject::tr("The other system holds a change that is not started yet. What differs is that change.");
    if (role == QLatin1String("copy"))
        return QObject::tr("The other system was made as a copy of this one. No change has been made since.");
    return QObject::tr("recab did not say what the other system is.");
}

QString Comparison::issue() const
{
    return QObject::tr("The running system (%1) and the other system (%2) differ in %3, %4 in all.")
        .arg(running.slot, other.slot, counted(count(), QObject::tr("%1 file"), QObject::tr("%1 files")),
             bytesText(differingBytes()));
}

QStringList Comparison::facts() const
{
    QStringList facts{roleWords(),
                      QObject::tr("%1 in %2 are the same in both.")
                          .arg(bytesText(sameBytes), counted(sameFiles, QObject::tr("%1 file"), QObject::tr("%1 files")))};
    if (!own.isEmpty())
        facts << QObject::tr("%1 differ by design, because each system names its own partition: %2.")
                     .arg(counted(own.size(), QObject::tr("%1 file"), QObject::tr("%1 files")), own.join(QObject::tr(", ")));
    // By folder first, so a long list still says where the differences are.
    QMap<QString, QList<int>> groups;
    for (const Difference &difference : differences)
        groups[groupOf(difference.path)] << int(difference.change);
    for (auto group = groups.cbegin(); group != groups.cend(); ++group) {
        QStringList parts;
        for (int kind = 0; kind < changeKinds().size(); ++kind)
            if (const auto n = group.value().count(kind))
                parts << QObject::tr("%1 %2").arg(n).arg(changeKinds().at(kind).inFact);
        facts << QObject::tr("%1: %2.").arg(group.key(), parts.join(QObject::tr(", ")));
    }
    // Then the biggest single differences by name.
    QList<Difference> biggest = differences;
    std::sort(biggest.begin(), biggest.end(), [](const Difference &a, const Difference &b) {
        return std::max(a.runningBytes, a.otherBytes) > std::max(b.runningBytes, b.otherBytes);
    });
    for (const Difference &difference : biggest.mid(0, 25)) {
        QString line = QObject::tr("%1: %2").arg(difference.path, changeKinds().at(difference.change).inFact);
        if (difference.change == Difference::Content)
            line += QObject::tr(" (%1 here, %2 in the other)").arg(bytesText(difference.runningBytes), bytesText(difference.otherBytes));
        else if (difference.change == Difference::Metadata)
            line += difference.detail.isEmpty() ? QString() : QObject::tr(" (%1)").arg(difference.detail);
        else
            line += QObject::tr(" (%1)").arg(bytesText(std::max(difference.runningBytes, difference.otherBytes)));
        facts << line;
    }
    if (biggest.size() > 25 || more > 0)
        facts << QObject::tr("%1 more are not listed here; `recab compare --json` lists them.")
                     .arg(counted(biggest.size() - std::min<qsizetype>(25, biggest.size()) + more, QObject::tr("%1 difference"),
                                  QObject::tr("%1 differences")));
    return facts;
}

QString Comparison::expected() const
{
    return QObject::tr("I am told what these differences are: which change each belongs to, and whether anything "
                       "differs that no change explains. Nothing is changed on either system.");
}

// ---------------------------------------------------------------- The page

// One system as a frame that fills while it is read, or stands empty.
class SideFrame : public QWidget {
public:
    SideFrame() { setMinimumSize(200, 150); }
    void set(double done, const QString &words)
    {
        done_ = done;
        words_ = words;
        update();
    }

protected:
    void paintEvent(QPaintEvent *) override
    {
        QPainter painter(this);
        painter.fillRect(rect(), palette().color(QPalette::Base));
        QColor ink = palette().color(QPalette::WindowText);
        QColor fill = ink;
        fill.setAlphaF(0.12f);
        painter.fillRect(QRectF(0, 0, width() * qBound(0.0, done_, 1.0), height()), fill);
        ink.setAlphaF(0.66f);
        painter.setPen(ink);
        painter.drawText(rect(), Qt::AlignCenter, words_);
    }

private:
    double done_ = 0;
    QString words_;
};

// One field of an os-release file, without its quotes.
static QString osField(const QString &path, const QString &field)
{
    QFile file(path);
    if (!file.open(QIODevice::ReadOnly))
        return {};
    for (const QByteArray &raw : file.readAll().split('\n')) {
        const QString line = QString::fromUtf8(raw).trimmed();
        if (line.startsWith(field + QLatin1Char('=')))
            return line.mid(field.size() + 1).remove(QLatin1Char('"'));
    }
    return {};
}

SystemsState SystemsState::fromJson(const QJsonObject &object, const QString &osRelease)
{
    SystemsState s;
    const int schema = object["schema_version"].toInt();
    if (schema == 3) {
        // The stand-in: it names both releases and counts its changes.
        s.known = true;
        s.distro = object["distro"].toString();
        s.key = QString::number(object["revision"].toInt(-1));
        s.runningRelease = object["running"].toObject()["release"].toString();
        s.otherRelease = object["other"].toObject()["release"].toString();
        s.otherRole = object["other"].toObject()["role"].toString();
        return s;
    }
    if (schema != 7 && schema != 8 && schema != 9 && schema != 10) {
        s.problem = QObject::tr("recab answered in a form this window does not know (schema %1; it reads schemas 7 through 10, "
                                "recab's own, and schema 3, its stand-in's).")
                        .arg(schema);
        return s;
    }
    // The real recab: two slots, each with the generation it holds, and what
    // the other one is to the running one. It names no release.
    s.known = true;
    s.distro = osField(osRelease, QStringLiteral("NAME"));
    s.runningRelease = osField(osRelease, QStringLiteral("VERSION_ID"));
    const QString running = object["active_slot"].toString(), other = object["inactive_slot"].toString();
    QString runningGeneration, otherGeneration;
    for (const QJsonValue &value : object["slots"].toArray()) {
        const QJsonObject slot = value.toObject();
        (slot["slot"].toString() == running ? runningGeneration : otherGeneration) =
            slot["generation"].toObject()["id"].toString();
    }
    const QJsonObject preparation = object["preparation"].toObject();
    const QString role = object["lifecycle"].toObject()["other"].toString();
    if (role != QLatin1String("unknown"))
        s.otherRole = role;
    s.runningRelease=object["running_release"].toString(s.runningRelease);
    // The next system continues the running one, and a copy is it.
    if (role == QLatin1String("next") || role == QLatin1String("copy"))
        s.otherRelease = object["prepared_release"].toString(s.runningRelease);
    s.key = QStringLiteral("%1/%2/%3/%4")
                .arg(running, runningGeneration, otherGeneration)
                .arg(preparation["revision"].toInteger(0));
    return s;
}

QString SystemsState::runningName() const
{
    return QStringLiteral("%1 %2").arg(distro, runningRelease).trimmed();
}

QString SystemsState::otherName() const
{
    return QStringLiteral("%1 %2").arg(distro, otherRelease).trimmed();
}

QString SystemsState::otherIs() const
{
    if (otherRole == QLatin1String("previous"))
        return QObject::tr("The system from before.");
    if (otherRole == QLatin1String("next"))
        return QObject::tr("The next system, not started yet.");
    return QObject::tr("The second copy.");
}

void SystemsPage::forget()
{
    here_ = there_ = nullptr;
    reading_[0] = reading_[1] = nullptr;
    step_ = nullptr;
    pointed_ = nullptr;
    list_ = nullptr;
    blocks_.reset();
}

// The two frames with their names over them, as on the compared page.
static QLayout *twoFrames(const SystemsState &state, SideFrame *frames[2])
{
    auto *sides = new QHBoxLayout;
    sides->setSpacing(14);
    for (int side = 0; side < 2; ++side) {
        auto *column = new QVBoxLayout;
        column->setSpacing(4);
        auto *title = new QVBoxLayout;
        title->setSpacing(0);
        title->addWidget(strong(side == 0 ? QObject::tr("This system") : QObject::tr("The other system")));
        title->addWidget(dim(text(side == 0 ? QObject::tr("Running now. %1").arg(state.runningName())
                                            : QObject::tr("%1 %2").arg(state.otherIs(), state.otherName()))));
        column->addLayout(title);
        frames[side] = new SideFrame;
        column->addWidget(frames[side], 1);
        sides->addLayout(column, 1);
    }
    return sides;
}

void SystemsPage::showState(const SystemsState &state, const QString &why)
{
    QVBoxLayout *column = fresh();
    forget();
    if (!state.known) {
        column->addWidget(heading(tr("The two systems could not be read")));
        column->addWidget(text(state.problem, true));
        auto *again = new QPushButton(tr("Try again"));
        again->setDefault(true);
        connect(again, &QPushButton::clicked, this, &SystemsPage::readAgain);
        column->addWidget(again, 0, Qt::AlignLeft);
        column->addStretch();
        return;
    }
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("The two systems have not been compared")));
    top->addWidget(dim(text(tr("This computer holds its system twice: the one that is running, and another on its own "
                               "partition. Comparing them shows whether they are the same, block by block."),
                            true)));
    header->addLayout(top, 1);
    auto *system = new QPushButton(tr("Open System…"));
    system->setToolTip(tr("Changing, updating and going back are done there"));
    connect(system, &QPushButton::clicked, this, &SystemsPage::openSystem);
    header->addWidget(system, 0, Qt::AlignBottom);
    column->addLayout(header);

    if (!why.isEmpty()) {
        auto *verdict = new Verdict;
        verdict->column()->addWidget(strong(tr("They could not be compared")));
        verdict->column()->addWidget(text(why, true));
        column->addWidget(verdict);
    }
    SideFrame *frames[2];
    column->addLayout(twoFrames(state, frames), 1);
    frames[0]->set(0, tr("Not read yet"));
    frames[1]->set(0, tr("Not read yet"));

    auto *foot = new QHBoxLayout;
    foot->addWidget(dim(text(tr("Both systems are read from start to end, which takes a few minutes and needs the "
                                "administrator's password. Nothing is changed on either."),
                             true)),
                    1);
    auto *go = new QPushButton(why.isEmpty() ? tr("Compare") : tr("Try again"));
    go->setDefault(true);
    connect(go, &QPushButton::clicked, this, &SystemsPage::compare);
    foot->addWidget(go);
    column->addLayout(foot);
}

void SystemsPage::showComparing(const SystemsState &state)
{
    QVBoxLayout *column = fresh();
    forget();
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Comparing the two systems")));
    top->addWidget(dim(text(tr("Both are read from start to end. Nothing is changed on either."), true)));
    header->addLayout(top, 1);
    auto *halt = new QPushButton(tr("Stop"));
    connect(halt, &QPushButton::clicked, this, [this, halt] {
        halt->setEnabled(false);
        emit stop();
    });
    header->addWidget(halt, 0, Qt::AlignBottom);
    column->addLayout(header);

    column->addLayout(twoFrames(state, reading_), 1);
    reading_[0]->set(0, tr("Waiting for the administrator's password"));
    reading_[1]->set(0, tr("Waiting"));
    step_ = dim(text(tr("Starting"), true));
    column->addWidget(step_);
}

void SystemsPage::progress(const QJsonObject &event)
{
    if (!reading_[0] || !step_)
        return;
    const QString kind = event["event"].toString(), step = event["step"].toString();
    const int side = step == QLatin1String("running") ? 0 : step == QLatin1String("other") ? 1 : -1;
    if (kind == QLatin1String("step")) {
        const bool done = event["status"] == QLatin1String("done");
        if (side >= 0)
            reading_[side]->set(done ? 1 : 0, done ? tr("Read") : tr("Reading"));
        if (step == QLatin1String("compare") && !done)
            step_->setText(tr("Both are read. Setting them side by side."));
        else if (side >= 0 && !done)
            step_->setText(side == 0 ? tr("Reading the running system") : tr("Reading the other system"));
    } else if (kind == QLatin1String("progress") && side >= 0) {
        const qint64 done = event["done"].toInteger(), total = event["total"].toInteger();
        if (total > 0) {
            reading_[side]->set(double(done) / double(total),
                                tr("Read %1 of %2").arg(bytesText(done), bytesText(total)));
            step_->setText((side == 0 ? tr("Reading the running system: %1 of %2")
                                      : tr("Reading the other system: %1 of %2"))
                               .arg(bytesText(done), bytesText(total)));
        }
    }
}

void SystemsPage::show(const Comparison &c, const QString &said, const QDateTime &when, bool stale)
{
    QVBoxLayout *column = fresh();
    forget();

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    QString title;
    if (c.identical())
        title = tr("The two systems are identical");
    else {
        // Never "100%" when something differs.
        const double percent = std::min(99.9, std::floor(c.alike() * 1000) / 10);
        title = tr("The two systems are %1% the same").arg(QLocale().toString(percent, 'f', 1));
    }
    top->addWidget(heading(title));
    QString under = c.roleWords();
    if (c.identical() && c.role != QLatin1String("copy"))
        under = tr("Every file is the same in both.");
    else if (!c.identical())
        under = tr("%1, %2 in all. ").arg(counted(c.count(), tr("%1 file differs"), tr("%1 files differ")),
                                          bytesText(c.differingBytes()))
                + under;
    if (when.isValid()) {
        const QString time = QLocale().toString(when.time(), QLocale::ShortFormat);
        under += QLatin1Char(' ')
                 + (when.date() == QDate::currentDate()
                        ? tr("Compared today at %1.").arg(time)
                        : tr("Compared on %1 at %2.").arg(QLocale().toString(when.date(), QLocale::ShortFormat), time));
    }
    top->addWidget(dim(text(under, true)));
    header->addLayout(top, 1);
    auto *system = new QPushButton(tr("Open System…"));
    system->setToolTip(tr("Changing, updating and going back are done there"));
    connect(system, &QPushButton::clicked, this, &SystemsPage::openSystem);
    header->addWidget(system, 0, Qt::AlignBottom);
    auto *again = new QPushButton(tr("Compare again"));
    again->setDefault(stale);
    connect(again, &QPushButton::clicked, this, &SystemsPage::compare);
    header->addWidget(again, 0, Qt::AlignBottom);
    column->addLayout(header);

    // Only what is under the header scrolls: the maps and the list keep a size
    // that can be read, and the header and the frame's strip stay in place.
    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    auto *inside = new QWidget;
    auto *body = new QVBoxLayout(inside);
    body->setContentsMargins(0, 0, 8, 0);
    body->setSpacing(14);
    scroll->setWidget(inside);
    column->addWidget(scroll, 1);

    if (stale) {
        auto *old = new Card(Card::Open);
        old->column()->addWidget(strong(tr("The systems have changed since this was compared")));
        old->column()->addWidget(text(tr("What is shown is how they were then. Compare again to see how they are now."), true));
        body->addWidget(old);
    }

    // The two systems as blocks, side by side, with one layout for both.
    blocks_ = Blocks::from(c);
    here_ = new SideMap(true, blocks_);
    there_ = new SideMap(false, blocks_);
    const QString otherIs = c.role == QLatin1String("previous") ? tr("The system from before.")
                            : c.role == QLatin1String("next") ? tr("The next system, not started yet.")
                                                                : tr("The second copy.");
    auto *sides = new QHBoxLayout;
    sides->setSpacing(14);
    auto *left = new QVBoxLayout;
    left->setSpacing(4);
    left->addLayout(sideTitle(tr("This system (%1)").arg(c.running.slot), tr("Running now."), c.running));
    left->addWidget(here_, 1);
    auto *right = new QVBoxLayout;
    right->setSpacing(4);
    right->addLayout(sideTitle(tr("The other system (%1)").arg(c.other.slot), otherIs, c.other));
    right->addWidget(there_, 1);
    sides->addLayout(left, 1);
    sides->addLayout(right, 1);
    body->addLayout(sides, 5);
    for (DiskMap *map : {here_, there_}) {
        map->setMinimumSize(200, 150);
        connect(map, &DiskMap::picked, this, [this](Node *node) { point(node); });
    }
    pointed_ = dim(text(QString(), true));
    body->addWidget(pointed_);

    QList<qint64> files(changeKinds().size(), 0);
    for (const Difference &difference : c.differences)
        ++files[difference.change];
    // In the frame the page has less room than one line for the sentence and
    // the kinds together: the sentence over them, not beside them.
    auto *legend = new QHBoxLayout;
    legend->setSpacing(18);
    auto *explain = dim(text(tr("A coloured block holds files that differ; its number says how many:"), true));
    for (int kind = 0; kind < changeKinds().size(); ++kind) {
        if (!files[kind])
            continue;
        auto *entry = new QHBoxLayout;
        entry->setSpacing(5);
        auto *square = new QLabel;
        square->setPixmap(swatch(changeKinds().at(kind).colour).pixmap(10, 10));
        entry->addWidget(square);
        entry->addWidget(text(tr("%1 %2").arg(files[kind]).arg(changeKinds().at(kind).brief)));
        legend->addLayout(entry);
    }
    legend->addStretch();
    if (!c.identical()) {
        body->addWidget(explain);
        body->addLayout(legend);
    } else {
        delete legend;
        delete explain;
    }

    if (!said.isEmpty()) {
        auto *panel = new Panel;
        panel->column()->addWidget(strong(tr("What the agent says")));
        panel->column()->addWidget(text(said, true));
        body->addWidget(panel);
    }

    list_ = nullptr;
    if (!c.identical()) {
        auto *list = list_ = new QTreeWidget;
        list->setHeaderLabels({tr("Where"), tr("Difference"), tr("Here"), tr("In the other")});
        list->setIndentation(14);
        list->setTextElideMode(Qt::ElideMiddle);
        list->header()->setStretchLastSection(false);
        list->header()->setSectionResizeMode(0, QHeaderView::Stretch);
        for (int section = 1; section < 4; ++section)
            list->header()->setSectionResizeMode(section, QHeaderView::ResizeToContents);
        QMap<QString, QList<const Difference *>> groups;
        for (const Difference &difference : c.differences)
            groups[groupOf(difference.path)] << &difference;
        for (auto group = groups.cbegin(); group != groups.cend(); ++group) {
            qint64 here = 0, there = 0;
            for (const Difference *difference : group.value()) {
                here += difference->runningBytes;
                there += difference->otherBytes;
            }
            auto *head = new QTreeWidgetItem(
                list, {group.key(), counted(group.value().size(), tr("%1 file differs"), tr("%1 files differ")),
                       sized(here), sized(there)});
            QFont bold = head->font(0);
            bold.setBold(true);
            head->setFont(0, bold);
            head->setData(0, Qt::UserRole, QVariant::fromValue(static_cast<void *>(blocks_->folder.value(group.key()))));
            for (const Difference *difference : group.value()) {
                const QString inside = group.key() == QLatin1String("/") ? difference->path.mid(1)
                                                                         : difference->path.mid(group.key().size() + 1);
                auto *line = new QTreeWidgetItem(head, {inside, changeKinds().at(difference->change).words,
                                                        sized(difference->runningBytes), sized(difference->otherBytes)});
                line->setIcon(1, swatch(changeKinds().at(difference->change).colour));
                line->setData(0, Qt::UserRole,
                              QVariant::fromValue(static_cast<void *>(blocks_->blockOf.value(difference->path))));
                line->setToolTip(0, difference->path
                                        + (difference->detail.isEmpty() ? QString() : QLatin1Char('\n') + difference->detail));
                if (!difference->detail.isEmpty())
                    line->setText(1, tr("%1: %2").arg(changeKinds().at(difference->change).words, difference->detail));
            }
            // A long folder stays folded: its count says enough until it is opened.
            head->setExpanded(group.value().size() <= 8);
        }
        // Room for six rows at least; the page scrolls when there is not more.
        list->setMinimumHeight(list->header()->sizeHint().height() + 6 * (list->fontMetrics().height() + 10) + 4);
        body->addWidget(list, 2);
        connect(list, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem *line) {
            if (line)
                point(static_cast<Node *>(line->data(0, Qt::UserRole).value<void *>()), false);
        });
    }
    point(nullptr);

    auto *foot = new QHBoxLayout;
    QStringList notes;
    if (!c.own.isEmpty())
        notes << tr("%1 differ by design, because each system names its own partition; they are not counted.")
                     .arg(counted(c.own.size(), tr("%1 file"), tr("%1 files")));
    if (c.more > 0)
        notes << tr("%1 more are not listed.").arg(counted(c.more, tr("%1 difference"), tr("%1 differences")));
    notes << tr("Nothing was changed.");
    foot->addWidget(dim(text(notes.join(QLatin1Char(' ')), true)), 1);
    if (!c.identical()) {
        auto *hand = new QPushButton(tr("Ask the agent what differs…"));
        hand->setDefault(said.isEmpty() && !stale);
        connect(hand, &QPushButton::clicked, this, &SystemsPage::handOver);
        foot->addWidget(hand);
    }
    body->addLayout(foot);
}

void SystemsPage::point(Node *node, bool inList)
{
    if (!blocks_ || !here_)
        return;
    if (node == blocks_->top.get())
        node = nullptr;
    here_->select(node);
    there_->select(node);
    if (!node) {
        pointed_->setText(blocks_->info.value(blocks_->folder.value(QStringLiteral("/"))).differing()
                              ? tr("Each block is a folder, in the same place on both sides. Click one to see "
                                   "whether it is the same in both.")
                              : tr("Each block is a folder, in the same place on both sides. No block differs."));
        return;
    }
    const BlockInfo about = blocks_->info.value(node);
    pointed_->setText(tr("%1: %2 here, %3 in the other; %4.")
                          .arg(blocks_->nameOf(node), bytesText(about.running), bytesText(about.other),
                               blocks_->differencesIn(node)));
    if (!inList || !list_)
        return;
    // The first line of the list that is this block or lies in it.
    const QSignalBlocker quiet(list_);
    QTreeWidgetItem *found = nullptr;
    for (QTreeWidgetItemIterator it(list_); *it && !found; ++it) {
        if (!(*it)->parent())
            continue;
        for (Node *up = static_cast<Node *>((*it)->data(0, Qt::UserRole).value<void *>()); up && !found; up = up->parent)
            if (up == node)
                found = *it;
    }
    list_->setCurrentItem(found);
    if (found)
        list_->scrollToItem(found);
}
