#include "pages.h"

#include "map.h"
#include "parts.h"
#include "words.h"

#include <QHeaderView>
#include <QPainter>
#include <QPushButton>
#include <QScrollArea>
#include <QSet>
#include <QSignalBlocker>
#include <QTreeWidgetItemIterator>
#include <QTreeWidget>
#include <QVBoxLayout>

namespace {

QPushButton *primary(const QString &words)
{
    auto *button = new QPushButton(words);
    button->setDefault(true);
    return button;
}

QIcon swatch(const QColor &colour)
{
    QPixmap square(12, 12);
    square.fill(colour);
    return QIcon(square);
}

qint64 sum(const QList<Finding> &found)
{
    qint64 total = 0;
    for (const Finding &finding : found)
        total += finding.bytes;
    return total;
}

} // namespace

// ---------------------------------------------------------------- Where the mess is

void MessPage::showScanning(const QStringList &roots)
{
    QVBoxLayout *column = fresh();
    tree_.reset();
    found_.clear();
    list_ = nullptr;
    map_ = nullptr;
    foot_ = head_ = where_ = nullptr;
    add_ = hand_ = nullptr;
    picked_ = nullptr;
    column->addWidget(heading(tr("Looking for mess")));
    column->addWidget(dim(text(tr("Reading %1. Nothing is changed.").arg(roots.join(tr(", "))), true)));
    progress_ = text(tr("Starting"));
    column->addWidget(progress_);
    column->addStretch();
}

void MessPage::showCount(qint64 files, qint64 bytes)
{
    if (progress_)
        progress_->setText(tr("%1 so far, in %2").arg(bytesText(bytes), counted(files, tr("%1 file"), tr("%1 files"))));
}

void MessPage::show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QStringList &roots,
                    qint64 freeBytes, qint64 freed, const QString &home)
{
    QVBoxLayout *column = fresh();
    progress_ = nullptr;
    tree_ = tree;
    found_ = found;
    home_ = home;
    picked_ = nullptr;

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    head_ = heading(QString());
    top->addWidget(head_);
    QString under = tr("In %1. %2 is free on this disk. Nothing has been removed.")
                        .arg(roots.join(tr(", ")), bytesText(freeBytes));
    if (freed > 0)
        under = tr("%1 was freed by the agent. ").arg(bytesText(freed)) + under;
    top->addWidget(dim(text(under, true)));
    header->addLayout(top, 1);
    auto *again = new QPushButton(tr("Look again"));
    connect(again, &QPushButton::clicked, this, &MessPage::rescan);
    header->addWidget(again, 0, Qt::AlignBottom);
    column->addLayout(header);

    auto *body = new QHBoxLayout;
    body->setSpacing(14);
    auto *left = new QVBoxLayout;
    left->setSpacing(6);
    map_ = new DiskMap;
    map_->setTree(tree);
    left->addWidget(map_, 1);
    // What is pointed at, as a path whose every part can be clicked.
    auto *pointed = new QHBoxLayout;
    where_ = text(QString(), true);
    where_->setTextFormat(Qt::RichText);
    where_->setAlignment(Qt::AlignLeft | Qt::AlignTop);
    where_->setMinimumHeight(2 * where_->fontMetrics().lineSpacing());
    pointed->addWidget(where_, 1);
    add_ = new QPushButton(tr("Add to the list"));
    connect(add_, &QPushButton::clicked, this, &MessPage::addPicked);
    pointed->addWidget(add_, 0, Qt::AlignTop);
    left->addLayout(pointed);
    body->addLayout(left, 3);

    list_ = new QTreeWidget;
    list_->setHeaderLabels({tr("What"), tr("Agent"), tr("Size")});
    list_->setRootIsDecorated(true);
    list_->setIndentation(14);
    list_->setTextElideMode(Qt::ElideLeft);
    list_->header()->setStretchLastSection(false);
    list_->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    list_->header()->setSectionResizeMode(1, QHeaderView::ResizeToContents);
    list_->header()->setSectionResizeMode(2, QHeaderView::ResizeToContents);
    body->addWidget(list_, 2);
    column->addLayout(body, 1);

    connect(where_, &QLabel::linkActivated, this, [this](const QString &steps) {
        // The link says how many folders up from what is pointed at.
        Node *node = picked_;
        for (int up = steps.toInt(); up > 0 && node; --up)
            node = node->parent;
        pick(node);
    });
    connect(list_, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem *line) {
        const QVariant index = line ? line->data(0, Qt::UserRole) : QVariant();
        if (index.isValid())
            pick(found_[index.toInt()].node);
    });
    connect(list_, &QTreeWidget::itemChanged, this, [this] { updateFoot(); });
    connect(map_, &DiskMap::picked, this, &MessPage::pick);

    auto *foot = new QHBoxLayout;
    foot_ = dim(text(QString(), true));
    foot->addWidget(foot_, 1);
    hand_ = primary(tr("Hand it to the agent…"));
    connect(hand_, &QPushButton::clicked, this, &MessPage::handOver);
    foot->addWidget(hand_);
    column->addLayout(foot);
    fillList();
    pick(nullptr);
}

void MessPage::fillList()
{
    // Keep what was unticked: the list is made again when something is added.
    QSet<Node *> unticked;
    for (QTreeWidgetItemIterator it(list_); *it; ++it) {
        const QVariant index = (*it)->data(0, Qt::UserRole);
        if (index.isValid() && (*it)->checkState(0) != Qt::Checked)
            unticked << static_cast<Node *>((*it)->data(1, Qt::UserRole).value<void *>());
    }
    const QSignalBlocker quiet(list_);
    list_->clear();
    const auto &kinds = messKinds();
    for (int kind = 0; kind < kinds.size(); ++kind) {
        qint64 total = 0;
        QList<int> members;
        for (int index = 0; index < found_.size(); ++index)
            if (found_[index].kind == kind) {
                total += found_[index].bytes;
                members << index;
            }
        if (members.isEmpty())
            continue;
        auto *group = new QTreeWidgetItem(list_, {kinds[kind].title, QString(), bytesText(total)});
        group->setIcon(0, swatch(kinds[kind].colour));
        QFont bold = group->font(0);
        bold.setBold(true);
        group->setFont(0, bold);
        // Ticking the kind ticks everything of that kind.
        group->setFlags(Qt::ItemIsEnabled | Qt::ItemIsUserCheckable | Qt::ItemIsAutoTristate);
        for (int index : members) {
            const Finding &finding = found_[index];
            auto *line = new QTreeWidgetItem(
                group, {finding.what, finding.sure ? tr("clears") : tr("asks first"), bytesText(finding.bytes)});
            line->setData(0, Qt::UserRole, index);
            line->setData(1, Qt::UserRole, QVariant::fromValue(static_cast<void *>(finding.node)));
            line->setFlags(Qt::ItemIsEnabled | Qt::ItemIsSelectable | Qt::ItemIsUserCheckable);
            line->setCheckState(0, unticked.contains(finding.node) ? Qt::Unchecked : Qt::Checked);
            line->setToolTip(0, finding.path + QLatin1Char('\n') + finding.why
                                    + (finding.sure ? QString() : QLatin1Char('\n') + tr("The agent asks you first.")));
        }
        // A long kind stays folded: its total says enough until it is opened.
        group->setExpanded(members.size() <= 8);
    }
    head_->setText(found_.isEmpty() ? tr("No mess worth naming")
                                    : tr("%1 of %2 looks like mess").arg(bytesText(sum(found_)), bytesText(tree_->bytes)));
    updateFoot();
}

// The finding that is this node or holds it, if there is one.
const Finding *MessPage::findingAround(Node *node) const
{
    for (Node *up = node; up; up = up->parent)
        if (up->kind >= 0)
            for (const Finding &finding : found_)
                if (finding.node == up)
                    return &finding;
    return nullptr;
}

void MessPage::pick(Node *node)
{
    if (!map_)
        return;
    if (node == tree_.get())
        node = nullptr;
    picked_ = node;
    map_->select(node);
    const Finding *around = node ? findingAround(node) : nullptr;

    // The line of the list that stands for it, or none.
    {
        const QSignalBlocker quiet(list_);
        QTreeWidgetItem *line = nullptr;
        if (around)
            for (QTreeWidgetItemIterator it(list_); *it && !line; ++it)
                if ((*it)->data(0, Qt::UserRole).isValid()
                    && (*it)->data(1, Qt::UserRole).value<void *>() == static_cast<void *>(around->node))
                    line = *it;
        list_->setCurrentItem(line);
        if (line)
            list_->scrollToItem(line);
    }

    if (!node) {
        where_->setText(tr("Click anything on the map to see what it is. What the rules did not name can be added "
                           "to the list by hand."));
        add_->setEnabled(false);
        return;
    }
    // Every part of the path is a link to that folder.
    QStringList parts;
    int steps = 0;
    for (Node *up = node; up && up->parent; up = up->parent, ++steps) {
        const QString name = (up->parent->parent ? up->name : shortPath(up->name, home_)).toHtmlEscaped();
        parts.prepend(steps ? QStringLiteral("<a href=\"%1\">%2</a>").arg(steps).arg(name)
                            : QStringLiteral("<b>%1</b>").arg(name));
    }
    QString words = parts.join(QStringLiteral(" / ")) + QStringLiteral(" &nbsp; ") + bytesText(node->bytes);
    if (around)
        words += QStringLiteral("<br>") + (around->node == node ? tr("On the list: %1").arg(around->why.toHtmlEscaped())
                                                                 : tr("Inside something on the list: %1")
                                                                       .arg(around->what.toHtmlEscaped()));
    else
        words += QStringLiteral("<br>") + tr("Not named by the rules.");
    where_->setText(words);
    add_->setEnabled(!around);
}

void MessPage::addPicked()
{
    if (!picked_ || findingAround(picked_))
        return;
    Node *node = picked_;
    found_ << pickedByHand(node, home_, found_);
    std::sort(found_.begin(), found_.end(), [](const Finding &a, const Finding &b) { return a.bytes > b.bytes; });
    map_->relay();
    fillList();
    pick(node);
}

Node *MessPage::biggestUnnamed() const
{
    // Among the folders directly inside each root: the biggest that holds no finding.
    Node *best = nullptr;
    if (!tree_)
        return best;
    for (const auto &root : tree_->children)
        for (const auto &child : root->children) {
            if (!child->dir || child->kind >= 0)
                continue;
            bool holds = false;
            for (const Finding &finding : found_)
                for (Node *up = finding.node; up && !holds; up = up->parent)
                    holds = up == child.get();
            if (!holds && (!best || child->bytes > best->bytes))
                best = child.get();
        }
    return best;
}

QList<Finding> MessPage::chosen() const
{
    QList<Finding> ticked;
    if (!list_)
        return ticked;
    for (QTreeWidgetItemIterator it(list_); *it; ++it) {
        const QVariant index = (*it)->data(0, Qt::UserRole);
        if (index.isValid() && (*it)->checkState(0) == Qt::Checked)
            ticked << found_[index.toInt()];
    }
    return ticked;
}

void MessPage::updateFoot()
{
    if (!foot_)
        return;
    const QList<Finding> ticked = chosen();
    hand_->setEnabled(!ticked.isEmpty());
    foot_->setText(ticked.isEmpty()
                       ? tr("Nothing is ticked.")
                       : tr("Ticked: %1 in %2. The agent does the clearing; this window removes nothing itself.")
                             .arg(bytesText(sum(ticked)), counted(ticked.size(), tr("%1 place"), tr("%1 places"))));
}

// ---------------------------------------------------------------- The hand-over

void HandPage::show(const QList<Finding> &chosen, qint64 scannedBytes, const QStringList &roots)
{
    QVBoxLayout *column = fresh();
    auto *backLink = new ClickLabel(tr("‹ Back"));
    dim(backLink);
    connect(backLink, &ClickLabel::clicked, this, &HandPage::back);
    column->addWidget(backLink, 0, Qt::AlignLeft);

    qint64 certain = 0;
    QStringList facts;
    for (const Finding &finding : chosen) {
        if (finding.sure)
            certain += finding.bytes;
        QString line = tr("%1 (%2): %3").arg(finding.path, bytesText(finding.bytes), finding.why);
        if (!finding.how.isEmpty())
            line += QLatin1Char(' ') + finding.how;
        if (!finding.sure)
            line += tr(" Ask me first.");
        facts << line;
    }

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Hand the mess to the agent")));
    top->addWidget(dim(text(tr("The agent clears it. This window only found it and says what it is."), true)));
    column->addLayout(top);

    auto *prompt = prompt_ = new PromptPanel(
        tr("%1 of %2 in %3 is taken by things that look like leftovers.")
            .arg(bytesText(sum(chosen)), bytesText(scannedBytes), roots.join(tr(", "))),
        facts,
        tr("At least %1 is free again. Only the things listed are removed. Anything marked \"ask me first\" is asked "
           "about before it goes. What belongs to Recipe is cleared through Recipe, not by deleting its files.")
            .arg(bytesText(certain)));
    // The list of facts is as long as the mess: let it scroll.
    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setWidget(prompt);
    column->addWidget(scroll, 1);

    auto *buttons = new QHBoxLayout;
    auto *cancel = new QPushButton(tr("Cancel"));
    auto *go = primary(tr("Hand it to the agent"));
    connect(cancel, &QPushButton::clicked, this, &HandPage::back);
    connect(go, &QPushButton::clicked, this, [this, prompt] { emit send(prompt->prompt()); });
    buttons->addStretch();
    buttons->addWidget(cancel);
    buttons->addWidget(go);
    column->addLayout(buttons);
}

QString HandPage::prompt() const
{
    return prompt_ ? prompt_->prompt() : QString();
}
