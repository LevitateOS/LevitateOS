#include "disk.h"

#include "parts.h"
#include "words.h"

#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QPainter>
#include <QPushButton>
#include <QScrollArea>
#include <QSet>
#include <QVBoxLayout>

#include <sys/statvfs.h>

namespace {

// How full a place has to be before the page says so: under a tenth free, or
// under 5 GiB.
bool nearlyFull(const Space &space)
{
    return space.known && (space.free < (5ll << 30) || space.free * 10 < space.total);
}

qint64 numberOf(const QJsonValue &value)
{
    // lsblk --bytes writes numbers; older versions wrote them as strings.
    return value.isString() ? value.toString().toLongLong() : value.toInteger();
}

void collect(const QJsonObject &node, DiskLayout &layout, const QString &homeDevice, const QString &homeMount, bool *holdsHome)
{
    for (const QJsonValue &value : node["children"].toArray()) {
        const QJsonObject child = value.toObject();
        Partition part;
        part.path = child["path"].toString(QStringLiteral("/dev/") + child["name"].toString());
        part.type = child["type"].toString();
        part.fstype = child["fstype"].toString();
        part.label = child["label"].toString();
        part.partlabel = child["partlabel"].toString();
        part.parttype = child["parttype"].toString().toLower();
        part.size = numberOf(child["size"]);
        QJsonArray mounts = child["mountpoints"].toArray();
        if (mounts.isEmpty() && child["mountpoint"].isString())
            mounts.append(child["mountpoint"]);
        for (const QJsonValue &mount : mounts)
            if (mount.isString() && !mount.toString().startsWith(QLatin1Char('['))) {
                part.mount = mount.toString();
                break;
            }
        if ((!homeDevice.isEmpty() && part.path == homeDevice) || (!homeMount.isEmpty() && part.mount == homeMount))
            *holdsHome = true;
        layout.parts << part;
        collect(child, layout, homeDevice, homeMount, holdsHome);
    }
}

// A bar of coloured parts on a track, drawn from the palette and the colours given.
class Shares : public QWidget {
public:
    Shares(const QList<Slice> &slices, qint64 whole, int height, QWidget *parent = nullptr)
        : QWidget(parent)
        , slices_(slices)
        , whole_(whole)
    {
        setFixedHeight(height);
        setMinimumWidth(60);
    }

protected:
    void paintEvent(QPaintEvent *) override
    {
        QPainter painter(this);
        painter.setRenderHint(QPainter::Antialiasing);
        QColor track = palette().color(QPalette::WindowText);
        track.setAlphaF(0.12f);
        painter.setPen(Qt::NoPen);
        painter.setBrush(track);
        painter.drawRoundedRect(rect(), 3, 3);
        if (whole_ <= 0)
            return;
        painter.setClipRect(rect());
        double left = 0;
        for (const Slice &slice : slices_) {
            if (slice.bytes <= 0)
                continue;
            // A part too small to see is still drawn, so nothing is left out of the bar.
            const double width = qMax(2.0, width_() * double(slice.bytes) / double(whole_));
            painter.setBrush(slice.colour);
            painter.drawRoundedRect(QRectF(left, 0, width, height()), 3, 3);
            left += width + 2;
        }
    }

private:
    double width_() const { return width() - 2.0 * qMax<qsizetype>(0, slices_.size() - 1); }
    QList<Slice> slices_;
    qint64 whole_;
};

struct Described {
    QString title, note, action; // action: "browse", "compare" or ""
};

Described describe(const Partition &part, const DiskInfo &info)
{
    const QString device = part.path;
    if (!device.isEmpty() && device == info.runningDevice)
        return {QObject::tr("The running system"), QObject::tr("The system you are using now · %1").arg(device), QString()};
    if (!device.isEmpty() && device == info.otherDevice) {
        QString title = QObject::tr("The other system"), note;
        if (info.otherRole == QLatin1String("previous")) {
            title = QObject::tr("The system from before");
            note = QObject::tr("What ran before the last change");
        } else if (info.otherRole == QLatin1String("next")) {
            title = QObject::tr("The next system");
            note = QObject::tr("Holds a change that is not started yet");
        } else if (info.otherRole == QLatin1String("copy")) {
            title = QObject::tr("The second copy");
            note = QObject::tr("Made as a copy of the running system");
        }
        note = (note.isEmpty() ? QString() : note + QObject::tr(" · ")) + device;
        if (info.compared && device == info.otherDeviceCompared)
            note += QObject::tr(" · its files took %1 when compared on %2")
                        .arg(bytesText(info.otherFilesBytes), QLocale().toString(info.comparedWhen.date(), QLocale::ShortFormat));
        return {title, note, QStringLiteral("compare")};
    }
    const bool homeHere = (!info.homeDevice.isEmpty() && device == info.homeDevice) || (!info.homeMount.isEmpty() && part.mount == info.homeMount);
    if (homeHere)
        return {QObject::tr("Your files and what programs keep"),
                QObject::tr("Your home folder is here · mounted at %1 · %2").arg(part.mount.isEmpty() ? QObject::tr("?") : part.mount, device),
                QStringLiteral("browse")};
    if (part.fstype == QLatin1String("swap"))
        return {QObject::tr("Swap space"), QObject::tr("Used as swap, not for files · %1").arg(device), QString()};
    if (part.parttype == QLatin1String("c12a7328-f81f-11d2-ba4b-00a0c93ec93b") || part.mount.endsWith(QLatin1String("/boot/efi")))
        return {QObject::tr("Start-up files"), QObject::tr("The EFI system partition · %1").arg(device), QString()};
    if (part.mount == QLatin1String("/boot"))
        return {QObject::tr("Start-up files"), QObject::tr("Mounted at /boot · %1").arg(device), QString()};
    if (part.mount == QLatin1String("/"))
        return {QObject::tr("The running system"), QObject::tr("Mounted at / · %1").arg(device), QString()};
    // What cannot be named is said by what it carries, not guessed.
    const QString name = !part.label.isEmpty() ? part.label : part.partlabel;
    QStringList facts;
    if (!part.fstype.isEmpty())
        facts << QObject::tr("%1 filesystem").arg(part.fstype);
    facts << (part.mount.isEmpty() ? QObject::tr("not mounted") : QObject::tr("mounted at %1").arg(part.mount));
    if (name.isEmpty() && part.fstype.isEmpty())
        facts << QObject::tr("nothing on it that lsblk can read");
    facts << device;
    return {name.isEmpty() ? QObject::tr("A partition without a name") : QObject::tr("Partition “%1”").arg(name),
            facts.join(QObject::tr(" · ")), QString()};
}

QPushButton *small(const QString &words, bool primary = false)
{
    auto *button = new QPushButton(words);
    button->setDefault(primary);
    return button;
}

} // namespace

Space spaceAt(const QString &path)
{
    Space space;
    struct statvfs st;
    if (::statvfs(path.toLocal8Bit().constData(), &st) != 0) {
        space.problem = QObject::tr("%1 could not be measured").arg(path);
        return space;
    }
    space.known = true;
    space.total = qint64(st.f_blocks) * qint64(st.f_frsize);
    space.free = qint64(st.f_bavail) * qint64(st.f_frsize);
    return space;
}

DiskLayout layoutFromLsblk(const QJsonObject &answer, const QString &homeDevice, const QString &homeMount)
{
    DiskLayout best;
    for (const QJsonValue &value : answer["blockdevices"].toArray()) {
        const QJsonObject disk = value.toObject();
        if (disk["type"].toString() != QLatin1String("disk"))
            continue;
        DiskLayout layout;
        bool holdsHome = false;
        collect(disk, layout, homeDevice, homeMount, &holdsHome);
        if ((!homeDevice.isEmpty() && disk["path"].toString() == homeDevice) || (!homeMount.isEmpty() && !disk["mountpoints"].toArray().isEmpty()
                                                                                   && disk["mountpoints"].toArray().contains(homeMount)))
            holdsHome = true;
        if (!holdsHome)
            continue;
        layout.found = true;
        layout.disk = disk["path"].toString(QStringLiteral("/dev/") + disk["name"].toString());
        layout.size = numberOf(disk["size"]);
        best = layout;
        break;
    }
    if (!best.found) {
        best.problem = QObject::tr("lsblk does not list the disk the home folder is on (%1).")
                           .arg(homeDevice.isEmpty() ? homeMount : homeDevice);
        return best;
    }
    for (Partition &part : best.parts) {
        if (part.mount.isEmpty())
            continue;
        const Space space = spaceAt(part.mount);
        if (space.known) {
            part.total = space.total;
            part.used = space.total - space.free;
            part.free = space.free;
        }
    }
    return best;
}

QList<Slice> composition(Node *tree, const QList<Finding> &found, const QString &home, qint64 heldBytes)
{
    qint64 files = 0, programs = 0, others = 0, leftovers = 0;
    // The top-level piece of a root a finding lies in: hidden ones are what programs keep.
    auto top = [&](Node *node) -> Node * {
        while (node->parent && node->parent->parent && node->parent->parent->parent)
            node = node->parent;
        return node;
    };
    for (const auto &root : tree->children) {
        if (root->name == home) {
            for (const auto &child : root->children)
                (child->name.startsWith(QLatin1Char('.')) ? programs : files) += child->bytes;
        } else {
            others += root->bytes;
        }
    }
    for (const Finding &finding : found) {
        leftovers += finding.bytes;
        Node *root = finding.node;
        while (root->parent && root->parent->parent)
            root = root->parent;
        if (root->name != home) {
            others -= finding.bytes;
        } else {
            const Node *piece = top(finding.node);
            (piece->name.startsWith(QLatin1Char('.')) ? programs : files) -= finding.bytes;
        }
    }
    // What was set aside is inside ~/.local; it is its own part, not "kept by programs".
    const qint64 held = qMin(heldBytes, programs);
    programs -= held;
    return {{QObject::tr("Your files"), qMax<qint64>(0, files), QColor(0x4f, 0x93, 0xd6)},
            {QObject::tr("What programs keep (hidden folders of the home folder)"), qMax<qint64>(0, programs), QColor(0x9a, 0x8f, 0x86)},
            {QObject::tr("Known leftovers"), leftovers, QColor(0xe0, 0xa0, 0x3a)},
            {QObject::tr("Set aside by the agent"), held, QColor(0xa7, 0x7b, 0xd1)},
            {QObject::tr("Other folders read"), qMax<qint64>(0, others), QColor(0x5c, 0xb3, 0x7a)}};
}

void DiskPage::show(const DiskInfo &info)
{
    shown_ = info;
    // The page can be taller than the window (many partitions, many rows), so it scrolls.
    QVBoxLayout *outer = fresh();
    outer->setContentsMargins(0, 0, 0, 0);
    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    auto *content = new QWidget;
    scroll->setWidget(content);
    outer->addWidget(scroll);
    auto *column = new QVBoxLayout(content);
    column->setContentsMargins(28, 20, 28, 16);
    column->setSpacing(12);

    // ---- The heading: true of what statvfs says, never of the list.
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    QString sentence;
    Mood mood = Mood::Plain;
    if (!info.home.known) {
        heading_ = tr("The place for your files could not be measured");
        sentence = info.home.problem;
    } else if (nearlyFull(info.home)) {
        heading_ = tr("The place for your files is nearly full");
        mood = Mood::Warn;
        sentence = tr("%1 is left of %2.").arg(bytesText(info.home.free), bytesText(info.home.total));
    } else {
        heading_ = tr("The place for your files has room");
        sentence = tr("%1 is left of %2.").arg(bytesText(info.home.free), bytesText(info.home.total));
    }
    QLabel *head = heading(heading_);
    if (mood != Mood::Plain) {
        QPalette palette = head->palette();
        palette.setColor(QPalette::WindowText, moodColour(palette, mood));
        head->setPalette(palette);
    }
    top->addWidget(head);
    top->addWidget(dim(text(sentence, true)));
    header->addLayout(top, 1);
    auto *again = small(tr("Look again"));
    connect(again, &QPushButton::clicked, this, &DiskPage::lookAgain);
    header->addWidget(again, 0, Qt::AlignBottom);
    column->addLayout(header);

    // ---- What can be freed.
    auto *freeable = new Card(nearlyFull(info.home) ? Card::Open : Card::Plain);
    QVBoxLayout *rows = freeable->column();
    rows->setContentsMargins(14, 6, 14, 6);
    rows->setSpacing(0);
    auto addRow = [&](qint64 bytes, const QString &name, const QString &words, QPushButton *button) {
        auto *row = new QHBoxLayout;
        row->setContentsMargins(0, 7, 0, 7);
        row->setSpacing(12);
        QLabel *size = strong(bytesText(bytes));
        size->setMinimumWidth(78);
        row->addWidget(size);
        auto *what = text(QStringLiteral("<b>%1</b> <span style=\"color:%3\">%2</span>")
                              .arg(name.toHtmlEscaped(), words.toHtmlEscaped(), palette().color(QPalette::PlaceholderText).name()),
                          true);
        what->setTextFormat(Qt::RichText);
        row->addWidget(what, 1);
        row->addWidget(button);
        if (rows->count())
            rows->addWidget(rule());
        rows->addLayout(row);
    };
    qint64 leftovers = 0;
    QStringList kinds;
    QSet<int> seen;
    for (const Finding &finding : info.found) {
        leftovers += finding.bytes;
        if (!seen.contains(finding.kind)) {
            seen << finding.kind;
            // Lower case, except Recipe's name.
            kinds << messKinds().at(finding.kind).title.toLower().replace(QStringLiteral("recipe"), QStringLiteral("Recipe"));
        }
    }
    int rowsShown = 0;
    if (!info.scanning && !info.found.isEmpty()) {
        auto *go = small(tr("Look and hand to the agent…"), true);
        connect(go, &QPushButton::clicked, this, &DiskPage::showMess);
        QString list = kinds.size() > 1 ? kinds.mid(0, kinds.size() - 1).join(tr(", ")) + tr(" and ") + kinds.last() : kinds.value(0);
        addRow(leftovers, tr("Known leftovers"), tr("in %1: %2.").arg(counted(info.found.size(), tr("%1 place"), tr("%1 places")), list), go);
        ++rowsShown;
    }
    if (info.earlierBytes > 0) {
        auto *go = small(tr("Open Recipe…"));
        connect(go, &QPushButton::clicked, this, &DiskPage::openRecipe);
        addRow(info.earlierBytes, tr("Kept by Recipe"),
               info.earlierGenerations > 0
                   ? tr("%1 of installed software, kept so you can go back. Recipe knows which are in use.")
                         .arg(counted(info.earlierGenerations, tr("%1 earlier generation"), tr("%1 earlier generations")))
                   : tr("Earlier versions of installed software, kept so you can go back. Recipe knows which are in use."),
               go);
        ++rowsShown;
    }
    if (info.heldRead && !info.held.empty() && info.held.bytes > 0) {
        auto *go = small(tr("See what is set aside"));
        connect(go, &QPushButton::clicked, this, &DiskPage::showHeld);
        addRow(info.held.bytes, tr("Set aside by the agent"),
               tr("in %1. Nothing is deleted until you let it go.")
                   .arg(counted(info.held.places.size(), tr("%1 place"), tr("%1 places"))),
               go);
        ++rowsShown;
    }
    if (info.scanning) {
        rows->addWidget(dim(text(tr("Looking for leftovers: %1 so far…").arg(bytesText(info.scanBytes)))));
    } else if (!rowsShown) {
        rows->addWidget(dim(text(tr("Nothing that is known to be a leftover was found."))));
    }
    column->addWidget(freeable);

    // ---- Every partition of the disk.
    column->addWidget(strong(tr("Every partition of this disk")));
    if (!info.layoutRead) {
        column->addWidget(dim(text(tr("Asking lsblk what is on the disk…"))));
    } else if (!info.layout.found) {
        column->addWidget(dim(text(info.layout.problem, true)));
        if (info.home.known)
            column->addWidget(dim(text(tr("The home folder is on %1. %2 free of %3.")
                                           .arg(info.homeDevice.isEmpty() ? tr("a device lsblk was not asked about") : info.homeDevice,
                                                bytesText(info.home.free), bytesText(info.home.total)),
                                       true)));
    } else {
        auto *table = new Panel;
        auto *tableBody = new QWidget;
        table->column()->setContentsMargins(0, 0, 0, 0);
        table->column()->addWidget(tableBody);
        auto *grid = new QGridLayout(tableBody);
        grid->setContentsMargins(14, 6, 14, 6);
        grid->setHorizontalSpacing(14);
        grid->setVerticalSpacing(0);
        grid->setColumnStretch(0, 1);
        grid->setColumnMinimumWidth(1, 70);
        grid->setColumnMinimumWidth(2, 190);
        grid->setColumnMinimumWidth(3, 80);
        const QStringList titles{tr("What it holds"), tr("Size"), tr("Used"), tr("Free")};
        for (int c = 0; c < titles.size(); ++c)
            grid->addWidget(dim(text(titles[c])), 0, c);
        int row = 1;
        const QString hint = palette().color(QPalette::PlaceholderText).name();
        for (const Partition &part : info.layout.parts) {
            const Described what = describe(part, info);
            grid->addWidget(rule(), row++, 0, 1, 5);
            auto *label = new QLabel(QStringLiteral("<b>%1</b><br><span style=\"color:%3\">%2</span>")
                                         .arg(what.title.toHtmlEscaped(), what.note.toHtmlEscaped(), hint));
            label->setTextFormat(Qt::RichText);
            label->setWordWrap(true);
            label->setMinimumHeight(40);
            grid->addWidget(label, row, 0);
            grid->addWidget(dim(text(bytesText(part.size))), row, 1);
            if (part.used >= 0 && part.size > 0) {
                const bool tight = part.free * 10 < part.total;
                QColor colour = palette().color(QPalette::Highlight);
                if (tight)
                    colour = moodColour(palette(), Mood::Bad);
                auto *bar = new Shares({{QString(), part.used, colour}}, part.total, 8);
                bar->setToolTip(tr("%1 used of %2").arg(bytesText(part.used), bytesText(part.total)));
                grid->addWidget(bar, row, 2, Qt::AlignVCenter);
                QLabel *free = text(bytesText(part.free));
                if (tight) {
                    QPalette palette = free->palette();
                    palette.setColor(QPalette::WindowText, moodColour(palette, Mood::Bad));
                    free->setPalette(palette);
                }
                grid->addWidget(free, row, 3);
            } else {
                grid->addWidget(dim(text(part.mount.isEmpty() ? tr("not mounted, so not read") : tr("could not be read"))), row, 2, 1, 2);
            }
            if (what.action == QLatin1String("browse")) {
                auto *go = small(tr("Browse…"));
                connect(go, &QPushButton::clicked, this, &DiskPage::showBrowse);
                grid->addWidget(go, row, 4, Qt::AlignRight);
            } else if (what.action == QLatin1String("compare")) {
                auto *go = small(tr("Compare the two…"));
                connect(go, &QPushButton::clicked, this, &DiskPage::showSystems);
                grid->addWidget(go, row, 4, Qt::AlignRight);
            }
            ++row;
        }
        column->addWidget(table);
    }

    // ---- Whose room it is, from the scan.
    auto *whose = new QHBoxLayout;
    whose->addWidget(strong(tr("Whose room it is, in what was read")));
    whose->addStretch();
    if (info.unreadable > 0) {
        QLabel *unread = dim(text(tr("%1 could not be read; their size is not known.")
                                      .arg(counted(info.unreadable, tr("%1 folder"), tr("%1 folders")))));
        whose->addWidget(unread);
    }
    column->addLayout(whose);
    if (info.scanning) {
        column->addWidget(dim(text(tr("Still reading: %1 so far.").arg(bytesText(info.scanBytes)))));
    } else if (info.scannedBytes <= 0) {
        column->addWidget(dim(text(tr("Nothing was read."))));
    } else {
        column->addWidget(new Shares(info.slices, info.scannedBytes, 16));
        // One label that wraps: a long legend never widens the page.
        QStringList entries;
        for (const Slice &slice : info.slices)
            if (slice.bytes > 0)
                entries << QStringLiteral("<span style=\"color:%1\">&#9632;</span> %2")
                               .arg(slice.colour.name(), tr("%1, %2").arg(slice.label, bytesText(slice.bytes)).toHtmlEscaped());
        auto *legend = text(entries.join(QStringLiteral(" &nbsp;&nbsp; ")), true);
        legend->setTextFormat(Qt::RichText);
        column->addWidget(legend);
        // The disk's own count of used room covers more than the folders read; say so only
        // where lengths and room taken are counted alike.
        if (info.home.known && !info.apparent) {
            const qint64 used = info.home.total - info.home.free;
            if (used > info.scannedBytes)
                column->addWidget(dim(text(tr("The disk counts %1 as used; %2 of that is outside the folders that were read.")
                                               .arg(bytesText(used), bytesText(used - info.scannedBytes)),
                                           true)));
        }
    }
    column->addStretch();
}
