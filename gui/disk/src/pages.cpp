#include "pages.h"

#include "parts.h"
#include "words.h"

#include <QAbstractButton>
#include <QHBoxLayout>
#include <QLabel>
#include <QHeaderView>
#include <QProgressBar>
#include <QPushButton>
#include <QScrollArea>
#include <QSet>
#include <QShortcut>
#include <QSignalBlocker>
#include <QTreeWidget>
#include <QTreeWidgetItemIterator>
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

QString agentWords(const Finding &finding)
{
    if (finding.kind == kindPicked)
        return QObject::tr("finds out, asks");
    return finding.sure ? QObject::tr("clears") : QObject::tr("asks first");
}

const int nodeRole = Qt::UserRole + 1;

} // namespace

// ---------------------------------------------------------------- The mess list

void MessPage::showScanning(const QStringList &roots)
{
    QVBoxLayout *column = fresh();
    tree_.reset();
    found_.clear();
    list_ = nullptr;
    foot_ = head_ = what_ = why_ = nullptr;
    show_ = open_ = takeOff_ = never_button_ = hand_ = nullptr;
    panel_ = nullptr;
    told_ = nullptr;
    picked_ = nullptr;
    column->addWidget(heading(tr("Looking for mess")));
    column->addWidget(dim(text(tr("Reading %1. Nothing is changed.").arg(roots.join(tr(", "))), true)));
    auto *bar = new QProgressBar;
    bar->setRange(0, 0);
    bar->setTextVisible(false);
    bar->setFixedHeight(8);
    bar->setMaximumWidth(360);
    column->addWidget(bar);
    progress_ = text(tr("Starting"));
    column->addWidget(progress_);
    column->addStretch();
}

void MessPage::showCount(qint64 files, qint64 bytes)
{
    if (progress_)
        progress_->setText(tr("%1 so far, in %2").arg(bytesText(bytes), counted(files, tr("%1 file"), tr("%1 files"))));
}

void MessPage::show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QStringList &roots, const Space &space,
                    const QString &home, const QStringList &never, qint64 now)
{
    QVBoxLayout *column = fresh();
    column->setSpacing(10);
    progress_ = nullptr;
    tree_ = tree;
    found_ = found;
    roots_ = roots;
    never_ = never;
    home_ = home;
    now_ = now;
    space_ = space;
    picked_ = nullptr;

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    head_ = heading(QString());
    top->addWidget(head_);
    QStringList shortRoots;
    for (const QString &root : roots)
        shortRoots << shortPath(root, home);
    QString under = tr("In %1.").arg(shortRoots.join(tr(", ")));
    if (space.known)
        under += QLatin1Char(' ') + tr("The disk has %1 left of %2.").arg(bytesText(space.free), bytesText(space.total));
    under += QLatin1Char(' ') + tr("This window deletes nothing here: the agent sets places aside first, and you let them go later.");
    top->addWidget(dim(text(under, true)));
    header->addLayout(top, 1);
    auto *again = new QPushButton(tr("Look again"));
    connect(again, &QPushButton::clicked, this, &MessPage::rescan);
    header->addWidget(again, 0, Qt::AlignBottom);
    column->addLayout(header);

    auto *hint = new QHBoxLayout;
    hint->addWidget(strong(tr("Tick only what you want the agent to take")));
    auto *sure = new ClickLabel(tr("Tick only what clears without asking"));
    sure->setForegroundRole(QPalette::Link);
    connect(sure, &ClickLabel::clicked, this, &MessPage::tickOnlySure);
    hint->addStretch();
    hint->addWidget(sure);
    column->addLayout(hint);

    list_ = new QTreeWidget;
    list_->setHeaderLabels({tr("What and where"), tr("Size"), tr("Last used"), tr("The agent")});
    list_->setRootIsDecorated(true);
    list_->setIndentation(14);
    list_->setTextElideMode(Qt::ElideMiddle);
    list_->header()->setStretchLastSection(false);
    list_->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    for (int c = 1; c < 4; ++c)
        list_->header()->setSectionResizeMode(c, QHeaderView::ResizeToContents);
    column->addWidget(list_, 1);

    panel_ = new Panel;
    auto *inside = static_cast<Panel *>(panel_)->column();
    inside->setSpacing(4);
    what_ = text(QString(), true);
    what_->setTextFormat(Qt::RichText);
    inside->addWidget(what_);
    why_ = text(QString(), true);
    inside->addWidget(why_);
    auto *actions = new QHBoxLayout;
    show_ = new QPushButton(tr("Show in Browse"));
    connect(show_, &QPushButton::clicked, this, [this] {
        if (picked_)
            emit showInBrowse(picked_);
    });
    open_ = new QPushButton(tr("Open in Files"));
    connect(open_, &QPushButton::clicked, this, [this] {
        if (picked_ && picked_->parent)
            emit openFolder(picked_->dir ? picked_->path() : picked_->parent->path());
    });
    takeOff_ = new QPushButton(tr("Take off the list"));
    connect(takeOff_, &QPushButton::clicked, this, [this] {
        if (picked_)
            emit takeOffRequested(picked_);
    });
    never_button_ = new QPushButton(tr("Never suggest this"));
    connect(never_button_, &QPushButton::clicked, this, [this] {
        if (picked_)
            emit neverRequested(picked_);
    });
    actions->addWidget(show_);
    actions->addWidget(open_);
    actions->addStretch();
    actions->addWidget(never_button_);
    actions->addWidget(takeOff_);
    inside->addLayout(actions);
    column->addWidget(panel_);

    auto *toldBox = new QWidget;
    told_ = new QVBoxLayout(toldBox);
    told_->setContentsMargins(0, 0, 0, 0);
    told_->setSpacing(2);
    column->addWidget(toldBox);

    connect(list_, &QTreeWidget::currentItemChanged, this, [this](QTreeWidgetItem *line) {
        if (!line || !line->data(0, nodeRole).isValid())
            return;
        picked_ = static_cast<Node *>(line->data(0, nodeRole).value<void *>());
        updatePanel();
    });
    connect(list_, &QTreeWidget::itemChanged, this, [this] { updateFoot(); });

    auto *foot = new QHBoxLayout;
    foot_ = dim(text(QString(), true));
    foot->addWidget(foot_, 1);
    foot->addWidget(dim(text(tr("Set aside first, so it can come back"))));
    hand_ = primary(tr("Hand it to the agent…"));
    connect(hand_, &QPushButton::clicked, this, &MessPage::handOver);
    foot->addWidget(hand_);
    column->addLayout(foot);
    fillList();
    updatePanel();
    updateTold();
}

void MessPage::setFound(const QList<Finding> &found, const QStringList &never)
{
    found_ = found;
    never_ = never;
    if (!list_)
        return;
    // A place that left the list is no longer chosen.
    bool still = false;
    for (const Finding &finding : found_)
        still = still || finding.node == picked_;
    if (!still)
        picked_ = nullptr;
    fillList();
    updatePanel();
    updateTold();
}

void MessPage::fillList()
{
    // Keep what was unticked: the list is made again when something changes.
    QSet<Node *> unticked;
    for (QTreeWidgetItemIterator it(list_); *it; ++it) {
        if ((*it)->data(0, nodeRole).isValid() && (*it)->checkState(0) != Qt::Checked)
            unticked << static_cast<Node *>((*it)->data(0, nodeRole).value<void *>());
    }
    const QSignalBlocker quiet(list_);
    list_->clear();
    const auto &kinds = messKinds();
    QTreeWidgetItem *current = nullptr;
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
        auto *group = new QTreeWidgetItem(list_, {kinds[kind].title + QStringLiteral("  ·  ") + kindWords(kind), bytesText(total), QString(), QString()});
        group->setIcon(0, swatch(kinds[kind].colour));
        QFont bold = group->font(0);
        bold.setBold(true);
        group->setFont(0, bold);
        group->setTextAlignment(1, Qt::AlignRight | Qt::AlignVCenter);
        // Ticking the kind ticks everything of that kind.
        group->setFlags(Qt::ItemIsEnabled | Qt::ItemIsUserCheckable | Qt::ItemIsAutoTristate);
        for (int index : members) {
            const Finding &finding = found_[index];
            const QString shown = shortPath(finding.path, home_);
            auto *line = new QTreeWidgetItem(group, {finding.what == shown ? shown : finding.what + QStringLiteral("  ·  ") + shown,
                                                     bytesText(finding.bytes), lastUsedText(finding.node->newest(), now_),
                                                     agentWords(finding)});
            line->setData(0, nodeRole, QVariant::fromValue(static_cast<void *>(finding.node)));
            line->setTextAlignment(1, Qt::AlignRight | Qt::AlignVCenter);
            line->setFlags(Qt::ItemIsEnabled | Qt::ItemIsSelectable | Qt::ItemIsUserCheckable);
            line->setCheckState(0, unticked.contains(finding.node) ? Qt::Unchecked : Qt::Checked);
            line->setToolTip(0, finding.path + QLatin1Char('\n') + finding.why
                                    + (finding.sure ? QString() : QLatin1Char('\n') + tr("The agent asks you first.")));
            if (finding.node == picked_)
                current = line;
        }
        // A long kind stays folded: its total says enough until it is opened.
        group->setExpanded(members.size() <= 8);
    }
    if (current) {
        list_->setCurrentItem(current);
        list_->scrollToItem(current);
    }
    head_->setText(found_.isEmpty() ? tr("No mess worth naming")
                                    : tr("%1 look like mess, %2").arg(counted(found_.size(), tr("%1 place"), tr("%1 places")), bytesText(sum(found_))));
    updateFoot();
}

void MessPage::pick(Node *node)
{
    if (!list_)
        return;
    picked_ = nullptr;
    for (QTreeWidgetItemIterator it(list_); *it; ++it)
        if ((*it)->data(0, nodeRole).isValid() && static_cast<Node *>((*it)->data(0, nodeRole).value<void *>()) == node) {
            if ((*it)->parent())
                (*it)->parent()->setExpanded(true);
            list_->setCurrentItem(*it);
            list_->scrollToItem(*it);
            picked_ = node;
        }
    updatePanel();
}

void MessPage::tickOnlySure()
{
    if (!list_)
        return;
    const QSignalBlocker quiet(list_);
    for (QTreeWidgetItemIterator it(list_); *it; ++it) {
        if (!(*it)->data(0, nodeRole).isValid())
            continue;
        const Node *node = static_cast<Node *>((*it)->data(0, nodeRole).value<void *>());
        for (const Finding &finding : found_)
            if (finding.node == node)
                (*it)->setCheckState(0, finding.sure ? Qt::Checked : Qt::Unchecked);
    }
    // The kinds' own boxes follow their places.
    for (int index = 0; index < list_->topLevelItemCount(); ++index) {
        QTreeWidgetItem *group = list_->topLevelItem(index);
        int on = 0;
        for (int child = 0; child < group->childCount(); ++child)
            on += group->child(child)->checkState(0) == Qt::Checked;
        group->setCheckState(0, on == group->childCount() ? Qt::Checked : on ? Qt::PartiallyChecked : Qt::Unchecked);
    }
    updateFoot();
}

void MessPage::updatePanel()
{
    if (!what_)
        return;
    const Finding *finding = nullptr;
    for (const Finding &one : found_)
        if (one.node == picked_)
            finding = &one;
    show_->setEnabled(finding);
    open_->setEnabled(finding);
    takeOff_->setEnabled(finding);
    never_button_->setEnabled(finding);
    if (!finding) {
        what_->setText(tr("Choose a place to see what it is."));
        why_->setText(found_.isEmpty() ? QString() : tr("What the rules did not name can be added to the list from Browse."));
        return;
    }
    what_->setText(QStringLiteral("<b>%1</b> &nbsp; <small>%2</small>")
                       .arg(shortPath(finding->path, home_).toHtmlEscaped(), bytesText(finding->bytes).toHtmlEscaped()));
    QString words = finding->why;
    if (!finding->how.isEmpty())
        words += QLatin1Char(' ') + finding->how;
    if (finding->kind == kindPicked)
        words += QLatin1Char(' ') + tr("The agent is told to find out what it is and to ask before it sets anything aside.");
    else if (!finding->sure)
        words += QLatin1Char(' ') + tr("The agent asks you first.");
    why_->setText(words);
}

// What the person told the app, as far as the app keeps it.
void MessPage::updateTold()
{
    if (!told_)
        return;
    while (QLayoutItem *item = told_->takeAt(0)) {
        delete item->widget();
        delete item;
    }
    told_->addWidget(strong(tr("What you told it")));
    QStringList shortRoots;
    for (const QString &root : roots_)
        shortRoots << shortPath(root, home_);
    told_->addWidget(dim(text(tr("Looked at: %1").arg(shortRoots.join(tr(", "))), true)));
    if (never_.isEmpty()) {
        told_->addWidget(dim(text(tr("Never suggest: nothing yet. Choose a place above and press “Never suggest this”."), true)));
        return;
    }
    told_->addWidget(dim(text(tr("Never suggest: %1").arg(counted(never_.size(), tr("%1 place"), tr("%1 places"))))));
    for (const QString &path : never_) {
        auto *row = new QHBoxLayout;
        row->setContentsMargins(14, 0, 0, 0);
        row->addWidget(text(shortPath(path, home_)));
        auto *again = new ClickLabel(tr("Suggest it again"));
        again->setForegroundRole(QPalette::Link);
        connect(again, &ClickLabel::clicked, this, [this, path] { emit suggestAgain(path); });
        row->addWidget(again);
        row->addStretch();
        auto *holder = new QWidget;
        holder->setLayout(row);
        told_->addWidget(holder);
    }
}

QList<Finding> MessPage::chosen() const
{
    QList<Finding> ticked;
    if (!list_)
        return ticked;
    for (QTreeWidgetItemIterator it(list_); *it; ++it) {
        if (!(*it)->data(0, nodeRole).isValid() || (*it)->checkState(0) != Qt::Checked)
            continue;
        const Node *node = static_cast<Node *>((*it)->data(0, nodeRole).value<void *>());
        for (const Finding &finding : found_)
            if (finding.node == node)
                ticked << finding;
    }
    return ticked;
}

void MessPage::updateFoot()
{
    if (!foot_)
        return;
    const QList<Finding> ticked = chosen();
    hand_->setEnabled(!ticked.isEmpty());
    qint64 certain = 0;
    for (const Finding &finding : ticked)
        if (finding.sure)
            certain += finding.bytes;
    foot_->setText(ticked.isEmpty()
                       ? tr("Nothing is ticked.")
                       : tr("%1 ticked: %2, of which %3 goes without asking.")
                             .arg(counted(ticked.size(), tr("%1 place"), tr("%1 places")), bytesText(sum(ticked)), bytesText(certain)));
}

// ---------------------------------------------------------------- The hand-over

void HandPage::show(const QList<Finding> &chosen, qint64 scannedBytes, const QStringList &roots, const QString &holding,
                    qint64 freeNow, const QString &home)
{
    qint64 certain = 0;
    QStringList facts;
    QStringList asks;
    int recipes = 0;
    for (const Finding &finding : chosen) {
        if (finding.sure)
            certain += finding.bytes;
        else
            asks << shortPath(finding.path, home);
        recipes += !finding.how.isEmpty();
        QString line = tr("%1 (%2): %3").arg(finding.path, bytesText(finding.bytes), finding.why);
        if (!finding.how.isEmpty())
            line += QLatin1Char(' ') + finding.how;
        if (!finding.sure)
            line += tr(" Ask me first.");
        facts << line;
    }
    // The convention is in the issue, the part the person cannot rewrite: the
    // agent must always know where to put things and how to say what it did.
    const QString issue =
        tr("%1 of %2 in %3 is taken by things that look like leftovers.")
            .arg(bytesText(sum(chosen)), bytesText(scannedBytes), roots.join(tr(", ")))
        + QStringLiteral("\n\n")
        + tr("Clear them by moving, not deleting. The holding folder is \"%1\". Move each place you clear to the holding "
             "folder plus the place's full path (a place /home/me/.cache/x goes to %1/home/me/.cache/x), on this same disk, "
             "and write %1/manifest.json as {\"version\": 1, \"entries\": [{\"from\": \"the place\", \"held\": \"its path "
             "beneath the holding folder, without the first /\", \"bytes\": its size in bytes}]}. Say what you did with each "
             "place as one event: {\"event\": \"place\", \"path\": the place, \"result\": \"set-aside\", \"kept\", \"left\" or "
             "\"could-not\", \"reason\": a sentence}.")
              .arg(holding);

    // What happens, in sentences that are true of this list: each depends on
    // what was ticked, never on a fixed picture of it.
    Ask ask;
    ask.title = tr("Hand these %1 to the agent?").arg(counted(chosen.size(), tr("%1 place"), tr("%1 places")));
    ask.happens << tr("The agent is asked to clear %1, %2 in all.")
                       .arg(counted(chosen.size(), tr("%1 place"), tr("%1 places")), bytesText(sum(chosen)));
    ask.happens << tr("What it clears by moving is set aside in the holding folder %1, on the same disk as your files. Nothing is "
                      "gone until you let it go, and you can bring each place back before that.")
                       .arg(shortPath(holding, home));
    if (recipes)
        ask.happens << tr("%1 of them belong to Recipe: the agent clears those through Recipe, so they are not moved and not held.")
                           .arg(recipes);
    if (asks.isEmpty()) {
        ask.happens << tr("It asks you about nothing first: every place on this list clears without asking.");
    } else {
        const QStringList named = asks.mid(0, 3);
        ask.happens << tr("It asks you first about %1: %2%3.")
                           .arg(counted(asks.size(), tr("%1 place"), tr("%1 places")), named.join(tr(", ")),
                                asks.size() > named.size() ? tr(" and %1 more").arg(asks.size() - named.size()) : QString());
    }
    ask.happens << tr("It works as you. A command as administrator runs only if you approve it.");
    if (freeNow >= 0)
        ask.happens << tr("Once you let it go, up to %1 is free: the disk would have up to %2 free instead of %3.")
                           .arg(bytesText(sum(chosen)), bytesText(freeNow + sum(chosen)), bytesText(freeNow));
    else
        ask.happens << tr("Once you let it go, up to %1 is free. How much the disk has free now could not be read.")
                           .arg(bytesText(sum(chosen)));
    ask.issue = issue;
    ask.facts = facts;
    ask.expected =
        tr("At least %1 is moved into the holding folder; nothing is deleted. Only the things listed are touched. Anything "
           "marked \"ask me first\" is asked about before it is moved. What belongs to Recipe is cleared through Recipe, "
           "not by moving its files. The holding folder is the person's to let go later: do not delete it.")
            .arg(bytesText(certain));
    ask.go = tr("Hand it over");
    show(ask);
}

// The layout of the shared "Before it happens" page (kit/before.*): the
// question, the line that nothing has started, what happens, then Cancel and
// the one button. The shared page has no part the person can edit, and the
// request the agent is given has one (the expected result), so this page is
// that layout with the request in it, not a BeforePage followed by another page.
void HandPage::show(const Ask &ask)
{
    QVBoxLayout *column = fresh();
    column->setContentsMargins(28, 20, 28, 16);
    if (!escape_) {
        escape_ = new QShortcut(Qt::Key_Escape, this);
        escape_->setContext(Qt::WidgetWithChildrenShortcut);
        connect(escape_, &QShortcut::activated, this, &HandPage::back);
    }

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(ask.title));
    top->addWidget(dim(text(tr("Nothing has started. This is what will happen if you go on."))));
    column->addLayout(top);

    // The sections scroll; Cancel and the button under them do not.
    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    auto *inner = new QWidget;
    auto *sections = new QVBoxLayout(inner);
    sections->setContentsMargins(0, 0, 4, 0);
    sections->setSpacing(14);

    auto *panel = new Panel;
    panel->column()->addWidget(strong(tr("What happens")));
    for (const QString &sentence : ask.happens) {
        auto *row = new QHBoxLayout;
        row->setSpacing(8);
        row->addWidget(dim(text(QStringLiteral("•"))), 0, Qt::AlignTop);
        row->addWidget(text(sentence, true), 1);
        panel->column()->addLayout(row);
    }
    sections->addWidget(panel);
    auto *request = prompt_ = new PromptPanel(ask.issue, ask.facts, ask.expected);
    sections->addWidget(request);
    sections->addStretch();
    scroll->setWidget(inner);
    column->addWidget(scroll, 1);

    auto *buttons = new QHBoxLayout;
    buttons->setSpacing(8);
    buttons->addWidget(dim(text(tr("Only the expected result is yours to change."), true)), 1);
    auto *cancel = new QPushButton(tr("Cancel"));
    auto *go = new QPushButton(ask.go);
    go->setDefault(true);
    connect(cancel, &QPushButton::clicked, this, &HandPage::back);
    connect(go, &QPushButton::clicked, this, [this, request] { emit send(request->prompt()); });
    buttons->addWidget(cancel);
    buttons->addWidget(go);
    column->addLayout(buttons);
    go->setFocus();
}

QString HandPage::pageText() const
{
    QStringList words;
    for (const QLabel *label : findChildren<QLabel *>())
        words << label->text();
    for (const QAbstractButton *button : findChildren<QAbstractButton *>())
        words << button->text();
    return words.join(QLatin1Char('\n'));
}

QString HandPage::prompt() const
{
    return prompt_ ? prompt_->prompt() : QString();
}
