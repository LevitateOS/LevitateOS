#include "browse.h"

#include "map.h"
#include "parts.h"
#include "words.h"

#include <QApplication>
#include <QClipboard>
#include <QHBoxLayout>
#include <QHeaderView>
#include <QLabel>
#include <QLineEdit>
#include <QPainter>
#include <QPushButton>
#include <QSet>
#include <QSignalBlocker>
#include <QStyledItemDelegate>
#include <QTabBar>
#include <QTreeWidget>
#include <QVBoxLayout>

#include <algorithm>

namespace {

const int shareRole = Qt::UserRole + 1;
const int nodeRole = Qt::UserRole + 2;
const int maxRows = 300;
const qint64 year = 365ll * 86400;

// A bar for the share a row takes of the folder, drawn in the column that
// holds one.
class ShareDelegate : public QStyledItemDelegate {
public:
    using QStyledItemDelegate::QStyledItemDelegate;
    void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const override
    {
        const QVariant share = index.data(shareRole);
        if (!share.isValid()) {
            QStyledItemDelegate::paint(painter, option, index);
            return;
        }
        QStyleOptionViewItem plain = option;
        plain.text.clear();
        QStyledItemDelegate::paint(painter, plain, index);
        const QRectF track = QRectF(option.rect).adjusted(4, option.rect.height() / 2.0 - 3, -4, -(option.rect.height() / 2.0 - 3));
        QColor ground = option.palette.color(QPalette::WindowText);
        ground.setAlphaF(0.14f);
        painter->setRenderHint(QPainter::Antialiasing);
        painter->setPen(Qt::NoPen);
        painter->setBrush(ground);
        painter->drawRoundedRect(track, 3, 3);
        painter->setBrush(option.palette.color(QPalette::Highlight));
        painter->drawRoundedRect(QRectF(track.left(), track.top(), qMax(2.0, track.width() * share.toDouble()), track.height()), 3, 3);
    }
};

QIcon swatch(const QColor &colour)
{
    QPixmap square(12, 12);
    square.fill(colour);
    return QIcon(square);
}

void collectFiles(Node *node, QList<Node *> &into)
{
    for (const auto &child : node->children) {
        if (child->dir)
            collectFiles(child.get(), into);
        else if (!child->aggregate)
            into << child.get();
    }
}

} // namespace

void BrowsePage::show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QString &home, qint64 now)
{
    // Where the person was is kept across a second look.
    const QString where = folder_ ? folder_->path() : QString();
    const QString at = pointed_ ? pointed_->path() : QString();
    tree_ = std::move(tree);
    found_ = found;
    home_ = home;
    now_ = now;
    folder_ = pointed_ = nullptr;

    QVBoxLayout *column = fresh();
    column->setSpacing(10);
    column->setContentsMargins(28, 16, 28, 10);

    auto *bar = new QHBoxLayout;
    up_ = new QPushButton(tr("Up"));
    connect(up_, &QPushButton::clicked, this, &BrowsePage::up);
    bar->addWidget(up_);
    path_ = text(QString(), true);
    path_->setTextFormat(Qt::RichText);
    // Every part of the path is a link; it says how many folders up from here.
    connect(path_, &QLabel::linkActivated, this, [this](const QString &steps) {
        Node *node = folder_;
        for (int up = steps.toInt(); up > 0 && node; --up)
            node = node->parent;
        goTo(node);
    });
    bar->addWidget(path_, 1);
    searchBox_ = new QLineEdit;
    searchBox_->setPlaceholderText(tr("Search here"));
    searchBox_->setClearButtonEnabled(true);
    searchBox_->setFixedWidth(200);
    searchBox_->setAccessibleName(tr("Search this folder"));
    searchBox_->setText(search_);
    connect(searchBox_, &QLineEdit::textChanged, this, &BrowsePage::setSearch);
    bar->addWidget(searchBox_);
    column->addLayout(bar);

    auto *views = new QHBoxLayout;
    views_ = new QTabBar;
    views_->setDrawBase(false);
    views_->setExpanding(false);
    views_->setFocusPolicy(Qt::TabFocus);
    views_->setAccessibleName(tr("Show"));
    for (const QString &name : {tr("Folders"), tr("Biggest files"), tr("Not used for a year")})
        views_->addTab(name);
    views_->setCurrentIndex(int(view_));
    connect(views_, &QTabBar::currentChanged, this, [this](int index) {
        if (index >= 0 && index != int(view_))
            setView(View(index));
    });
    views->addWidget(views_);
    legend_ = new QLabel;
    legend_->setTextFormat(Qt::RichText);
    legend_->setWordWrap(true);
    legend_->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
    legend_->setSizePolicy(QSizePolicy::Ignored, QSizePolicy::Preferred);
    views->addWidget(legend_, 1);
    column->addLayout(views);

    auto *body = new QHBoxLayout;
    body->setSpacing(12);
    rows_ = new QTreeWidget;
    rows_->setRootIsDecorated(false);
    rows_->setUniformRowHeights(true);
    rows_->setTextElideMode(Qt::ElideMiddle);
    rows_->setFixedWidth(340);
    rows_->setItemDelegateForColumn(2, new ShareDelegate(rows_));
    rows_->header()->setStretchLastSection(false);
    rows_->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    rows_->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    rows_->header()->setSectionResizeMode(2, QHeaderView::Fixed);
    rows_->setColumnWidth(2, 76);
    connect(rows_, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem *item) {
        if (filling_ || !item)
            return;
        if (Node *node = static_cast<Node *>(item->data(0, nodeRole).value<void *>()))
            pick(node);
    });
    connect(rows_, &QTreeWidget::itemActivated, this, [this](QTreeWidgetItem *item) {
        if (Node *node = static_cast<Node *>(item->data(0, nodeRole).value<void *>()))
            if (node->dir)
                goTo(node);
    });
    body->addWidget(rows_);

    auto *right = new QVBoxLayout;
    right->setSpacing(8);
    map_ = new DiskMap;
    map_->setTree(tree_);
    connect(map_, &DiskMap::picked, this, &BrowsePage::pick);
    right->addWidget(map_, 1);

    auto *panel = new Panel;
    auto *inside = panel->column();
    inside->setSpacing(6);
    what_ = text(QString(), true);
    what_->setTextFormat(Qt::RichText);
    inside->addWidget(what_);
    note_ = text(QString(), true);
    inside->addWidget(note_);
    // Two rows, so the panel fits the room beside the list.
    auto *actions = new QHBoxLayout;
    goIn_ = new QPushButton(tr("Go in"));
    connect(goIn_, &QPushButton::clicked, this, &BrowsePage::goIn);
    open_ = new QPushButton(tr("Open in Files"));
    connect(open_, &QPushButton::clicked, this, [this] {
        if (pointed_ && pointed_->parent)
            emit openFolder(pointed_->dir ? pointed_->path() : pointed_->parent->path());
    });
    copy_ = new QPushButton(tr("Copy the path"));
    connect(copy_, &QPushButton::clicked, this, [this] {
        if (!pointed_)
            return;
        QApplication::clipboard()->setText(pointed_->path());
        emit said(tr("Copied %1").arg(shortPath(pointed_->path(), home_)));
    });
    add_ = new QPushButton(tr("Add to the list"));
    connect(add_, &QPushButton::clicked, this, [this] {
        if (pointed_)
            emit addRequested(pointed_);
    });
    never_ = new QPushButton(tr("Never suggest this"));
    connect(never_, &QPushButton::clicked, this, [this] {
        if (const Finding *around = pointed_ ? findingAround(found_, pointed_) : nullptr)
            emit neverRequested(around->node);
    });
    takeOff_ = new QPushButton(tr("Take off the list"));
    connect(takeOff_, &QPushButton::clicked, this, [this] {
        if (const Finding *around = pointed_ ? findingAround(found_, pointed_) : nullptr)
            emit takeOffRequested(around->node);
    });
    actions->addWidget(goIn_);
    actions->addWidget(open_);
    actions->addWidget(copy_);
    actions->addStretch();
    inside->addLayout(actions);
    auto *listActions = new QHBoxLayout;
    listActions->addStretch();
    listActions->addWidget(add_);
    listActions->addWidget(never_);
    listActions->addWidget(takeOff_);
    inside->addLayout(listActions);
    right->addWidget(panel);
    body->addLayout(right, 1);
    column->addLayout(body, 1);

    auto *strip = new QHBoxLayout;
    stripText_ = text(QString(), true);
    strip->addWidget(stripText_, 1);
    listButton_ = new QPushButton(tr("See the list"));
    connect(listButton_, &QPushButton::clicked, this, &BrowsePage::showMess);
    handButton_ = new QPushButton(tr("Hand it to the agent…"));
    handButton_->setDefault(true);
    connect(handButton_, &QPushButton::clicked, this, &BrowsePage::handOver);
    strip->addWidget(listButton_);
    strip->addWidget(handButton_);
    column->addLayout(strip);

    // Back where the person was, else in the home folder, else at the top.
    Node *start = resolve(where);
    if (!start)
        for (const auto &root : tree_->children)
            if (root->name == home_)
                start = root.get();
    folder_ = start ? start : tree_.get();
    drawLegend();
    fill();
    if (Node *node = resolve(at))
        pick(node);
    else
        pick(nullptr);
    drawStrip();
}

Node *BrowsePage::resolve(const QString &path) const
{
    if (path.isEmpty() || !tree_)
        return nullptr;
    // Roots are named by their whole path; below them, one name per level.
    for (const auto &root : tree_->children) {
        if (path == root->name)
            return root.get();
        if (!path.startsWith(root->name + QLatin1Char('/')))
            continue;
        Node *node = root.get();
        for (const QString &part : path.mid(root->name.size() + 1).split(QLatin1Char('/'))) {
            Node *next = nullptr;
            for (const auto &child : node->children)
                if (child->name == part)
                    next = child.get();
            if (!next)
                return nullptr;
            node = next;
        }
        return node;
    }
    return nullptr;
}

void BrowsePage::setFound(const QList<Finding> &found)
{
    found_ = found;
    if (!rows_)
        return;
    map_->relay();
    drawLegend();
    const Node *keep = pointed_;
    fill();
    pick(const_cast<Node *>(keep));
    drawStrip();
}

void BrowsePage::goTo(Node *folder)
{
    if (!folder || !folder->dir || !rows_)
        return;
    folder_ = folder;
    pointed_ = nullptr;
    search_.clear();
    if (searchBox_->text().size()) {
        const QSignalBlocker quiet(searchBox_);
        searchBox_->clear();
    }
    fill();
    pick(nullptr);
}

void BrowsePage::goIn()
{
    if (pointed_ && pointed_->dir && pointed_ != folder_)
        goTo(pointed_);
}

void BrowsePage::up()
{
    if (folder_ && folder_->parent)
        goTo(folder_->parent);
}

void BrowsePage::setView(View view)
{
    view_ = view;
    if (views_ && views_->currentIndex() != int(view)) {
        const QSignalBlocker quiet(views_);
        views_->setCurrentIndex(int(view));
    }
    if (rows_)
        fill();
}

void BrowsePage::setSearch(const QString &words)
{
    search_ = words.trimmed();
    if (searchBox_ && searchBox_->text() != words) {
        const QSignalBlocker quiet(searchBox_);
        searchBox_->setText(words);
    }
    if (rows_)
        fill();
}

QStringList BrowsePage::listed() const
{
    QStringList names;
    if (!rows_)
        return names;
    for (int row = 0; row < rows_->topLevelItemCount(); ++row)
        if (rows_->topLevelItem(row)->data(0, nodeRole).isValid())
            names << rows_->topLevelItem(row)->text(0);
    return names;
}

int BrowsePage::rowOf(Node *node) const
{
    for (int row = 0; row < rows_->topLevelItemCount(); ++row)
        if (static_cast<Node *>(rows_->topLevelItem(row)->data(0, nodeRole).value<void *>()) == node)
            return row;
    return -1;
}

void BrowsePage::fill()
{
    filling_ = true;
    rows_->clear();
    map_->showFolder(folder_ == tree_.get() ? nullptr : folder_);
    drawPath();
    const QString needle = search_;
    auto matches = [&](const QString &name) { return needle.isEmpty() || name.contains(needle, Qt::CaseInsensitive); };
    auto kindOf = [&](Node *node) -> QString {
        if (node->kind < 0)
            return QString();
        return messKinds().at(node->kind).title;
    };
    auto decorateRow = [&](QTreeWidgetItem *item, Node *node) {
        item->setData(0, nodeRole, QVariant::fromValue(static_cast<void *>(node)));
        if (node->kind >= 0) {
            item->setIcon(0, swatch(messKinds().at(node->kind).colour));
            item->setToolTip(0, kindOf(node));
        }
    };

    if (view_ == View::Folders) {
        rows_->setHeaderLabels({tr("Folder"), tr("Size"), QString()});
        qint64 filesHere = 0, bytesHere = 0, shown = 0, total = 0;
        for (const auto &child : folder_->children) {
            if (!child->dir) {
                filesHere += child->files;
                bytesHere += child->bytes;
                continue;
            }
            ++total;
            if (!matches(child->name) || shown >= maxRows)
                continue;
            ++shown;
            const QString name = folder_ == tree_.get() ? shortPath(child->name, home_) : child->name;
            auto *item = new QTreeWidgetItem(rows_, {name, bytesText(child->bytes), QString()});
            item->setData(2, shareRole, folder_->bytes > 0 ? double(child->bytes) / double(folder_->bytes) : 0.0);
            item->setTextAlignment(1, Qt::AlignRight | Qt::AlignVCenter);
            decorateRow(item, child.get());
        }
        if (filesHere > 0 && needle.isEmpty()) {
            auto *item = new QTreeWidgetItem(rows_, {tr("%1 directly here").arg(counted(filesHere, tr("%1 file"), tr("%1 files"))),
                                                     bytesText(bytesHere), QString()});
            item->setFlags(Qt::ItemIsEnabled);
            item->setTextAlignment(1, Qt::AlignRight | Qt::AlignVCenter);
        }
        if (!needle.isEmpty() && !shown) {
            auto *item = new QTreeWidgetItem(rows_, {tr("No folder here has “%1” in its name").arg(needle), QString(), QString()});
            item->setFlags(Qt::ItemIsEnabled);
        } else if (total > shown && needle.isEmpty()) {
            auto *item = new QTreeWidgetItem(rows_, {tr("%1 smaller folders are not listed").arg(total - shown), QString(), QString()});
            item->setFlags(Qt::ItemIsEnabled);
        }
    } else {
        rows_->setHeaderLabels({tr("File"), tr("Size"), tr("Last used")});
        QList<Node *> files;
        collectFiles(folder_, files);
        if (view_ == View::Unused)
            files.erase(std::remove_if(files.begin(), files.end(), [&](Node *n) { return n->lastUsed <= 0 || now_ - n->lastUsed <= year; }),
                        files.end());
        std::sort(files.begin(), files.end(), [](Node *a, Node *b) { return a->bytes > b->bytes; });
        const QString base = folder_->path();
        qint64 shown = 0;
        for (Node *file : files) {
            const QString path = file->path();
            const QString relative = base.isEmpty() ? shortPath(path, home_) : path.mid(base.size() + 1);
            if (!matches(relative) || shown >= maxRows)
                continue;
            ++shown;
            auto *item = new QTreeWidgetItem(rows_, {relative, bytesText(file->bytes), lastUsedText(file->lastUsed, now_)});
            item->setTextAlignment(1, Qt::AlignRight | Qt::AlignVCenter);
            item->setToolTip(0, path);
            decorateRow(item, file);
        }
        const QString none = view_ == View::Unused ? tr("No file of 1 MiB or more here has gone a year unopened and unchanged.")
                                                   : tr("No file of 1 MiB or more here.");
        if (!shown) {
            auto *item = new QTreeWidgetItem(rows_, {needle.isEmpty() ? none : tr("No file here has “%1” in its path").arg(needle), QString(), QString()});
            item->setFlags(Qt::ItemIsEnabled);
        } else {
            auto *item = new QTreeWidgetItem(rows_, {tr("Files under 1 MiB are not listed one by one."), QString(), QString()});
            item->setFlags(Qt::ItemIsEnabled);
            item->setForeground(0, palette().color(QPalette::PlaceholderText));
        }
    }
    filling_ = false;
}

void BrowsePage::drawPath()
{
    up_->setEnabled(folder_ && folder_->parent);
    QStringList parts;
    int steps = 0;
    for (Node *node = folder_; node; node = node->parent, ++steps) {
        QString name = !node->parent ? tr("Folders read") : node->parent->parent ? node->name : shortPath(node->name, home_);
        name = name.toHtmlEscaped();
        parts.prepend(steps ? QStringLiteral("<a href=\"%1\">%2</a>").arg(steps).arg(name) : QStringLiteral("<b>%1</b>").arg(name));
    }
    path_->setText(parts.join(QStringLiteral(" / ")) + QStringLiteral(" &nbsp; <small>%1</small>").arg(bytesText(folder_->bytes)));
}

void BrowsePage::pick(Node *node)
{
    if (!rows_)
        return;
    if (node == tree_.get())
        node = nullptr;
    pointed_ = node;
    map_->select(node == folder_ ? nullptr : node);
    {
        const QSignalBlocker quiet(rows_);
        const int row = node ? rowOf(node) : -1;
        rows_->setCurrentItem(row >= 0 ? rows_->topLevelItem(row) : nullptr);
    }
    drawPanel();
}

void BrowsePage::drawPanel()
{
    Node *shown = pointed_ ? pointed_ : folder_;
    const bool top = shown == tree_.get();
    const Finding *around = (!top && shown) ? findingAround(found_, shown) : nullptr;
    QString name = top ? tr("Everything that was read") : shortPath(shown->path(), home_);
    QString line = QStringLiteral("<b>%1</b> &nbsp; <small>%2").arg(name.toHtmlEscaped(), bytesText(shown->bytes).toHtmlEscaped());
    if (!top) {
        line += shown->dir ? tr(" · %1").arg(counted(shown->files, tr("%1 file"), tr("%1 files"))).toHtmlEscaped() : QString();
        const qint64 used = shown->newest();
        if (used > 0)
            line += tr(" · last used %1").arg(lastUsedText(used, now_)).toHtmlEscaped();
    }
    what_->setText(line + QStringLiteral("</small>"));

    QString words;
    if (top)
        words = tr("The folders this window read. Choose one and go in.");
    else if (!around)
        words = tr("Not named by the rules, so it is not on the list.");
    else if (around->node == shown)
        words = around->why + QLatin1Char(' ')
                + (around->kind == kindPicked
                       ? tr("You picked it. The agent is told to find out what it is and to ask before it sets anything aside.")
                   : around->sure ? tr("The agent sets it aside without asking.")
                                  : tr("The agent asks before it sets it aside."));
    else
        words = tr("Inside %1, which is on the list.").arg(around->what);
    note_->setText(words);

    const bool real = !top;
    goIn_->setEnabled(real && shown->dir && shown != folder_);
    open_->setEnabled(real);
    copy_->setEnabled(real);
    add_->setVisible(real && !around && pointed_);
    takeOff_->setVisible(around != nullptr);
    never_->setVisible(around != nullptr);
}

void BrowsePage::drawStrip()
{
    qint64 bytes = 0;
    for (const Finding &finding : found_)
        bytes += finding.bytes;
    stripText_->setText(found_.isEmpty()
                            ? tr("Nothing is on the list of mess.")
                            : tr("%1 on the list of mess, %2. What the agent clears is set aside first, so it can come back.")
                                  .arg(counted(found_.size(), tr("%1 place is"), tr("%1 places are")), bytesText(bytes)));
    handButton_->setEnabled(!found_.isEmpty());
    listButton_->setEnabled(!found_.isEmpty());
}

// The colours on the map and in the rows, for the kinds of mess on the list: one
// label that wraps, so a long list never widens the page.
void BrowsePage::drawLegend()
{
    QStringList entries;
    QSet<int> seen;
    for (const Finding &finding : found_) {
        if (seen.contains(finding.kind))
            continue;
        seen << finding.kind;
        QString name = messKinds().at(finding.kind).title.toLower().replace(QStringLiteral("recipe"), QStringLiteral("Recipe"));
        entries << QStringLiteral("<span style=\"color:%1\">&#9632;</span> %2").arg(messKinds().at(finding.kind).colour.name(), name.toHtmlEscaped());
    }
    legend_->setText(entries.join(QStringLiteral(" &nbsp; ")));
}
