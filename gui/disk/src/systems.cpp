#include "systems.h"

#include "parts.h"
#include "words.h"

#include <QHeaderView>
#include <QJsonArray>
#include <QLabel>
#include <QLocale>
#include <QMap>
#include <QPainter>
#include <QPushButton>
#include <QTreeWidget>
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

// One bar for both systems: how much is the same, and how much differs in which way.
class AlikeBar : public QWidget {
public:
    AlikeBar(qint64 same, const QList<qint64> &differing)
        : same_(same)
        , differing_(differing)
    {
        setFixedHeight(14);
    }

protected:
    void paintEvent(QPaintEvent *) override
    {
        QPainter painter(this);
        qint64 total = same_;
        for (qint64 part : differing_)
            total += part;
        if (total <= 0)
            return;
        QColor quiet = palette().color(QPalette::WindowText);
        quiet.setAlphaF(0.22f);
        // What differs is drawn at least a few pixels wide, or it could not be seen at all.
        double x = width();
        for (int kind = differing_.size() - 1; kind >= 0; --kind) {
            if (differing_[kind] <= 0)
                continue;
            const double wide = std::max(4.0, double(differing_[kind]) / double(total) * width());
            x -= wide;
            painter.fillRect(QRectF(x, 0, wide - 1, height()), changeKinds().at(kind).colour);
        }
        painter.fillRect(QRectF(0, 0, x - 1, height()), quiet);
    }

private:
    qint64 same_;
    QList<qint64> differing_;
};

QWidget *sideCard(Card::Tone tone, const QString &title, const QString &what, const Side &side)
{
    auto *card = new Card(tone);
    card->column()->setSpacing(2);
    card->column()->addWidget(strong(title));
    card->column()->addWidget(text(what, true));
    card->column()->addWidget(dim(text(
        QObject::tr("%1 in %2").arg(bytesText(side.bytes), counted(side.files, QObject::tr("%1 file"), QObject::tr("%1 files"))))));
    card->column()->addWidget(dim(text(side.device)));
    return card;
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
    if (role == QLatin1String("before"))
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

void SystemsPage::showStart(const QString &why)
{
    QVBoxLayout *column = fresh();
    column->addWidget(heading(tr("Compare the two systems")));
    column->addWidget(dim(text(tr("This computer holds its system twice: the one that is running, and another on its own "
                                  "partition. Comparing them shows how alike they are and what differs, file by file."),
                               true)));
    if (!why.isEmpty()) {
        auto *verdict = new Verdict;
        verdict->column()->addWidget(strong(tr("They could not be compared")));
        verdict->column()->addWidget(text(why, true));
        column->addWidget(verdict);
    }
    auto *panel = new Panel;
    panel->column()->addWidget(text(tr("Both systems are read from start to end, which takes a few minutes and needs the "
                                       "administrator's password. Nothing is changed on either."),
                                    true));
    auto *row = new QHBoxLayout;
    auto *go = new QPushButton(why.isEmpty() ? tr("Compare") : tr("Try again"));
    go->setDefault(true);
    connect(go, &QPushButton::clicked, this, &SystemsPage::compare);
    row->addWidget(go);
    row->addStretch();
    panel->column()->addLayout(row);
    column->addWidget(panel);
    column->addStretch();
}

void SystemsPage::show(const Comparison &c, const QString &said)
{
    QVBoxLayout *column = fresh();

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
    top->addWidget(dim(text(under, true)));
    header->addLayout(top, 1);
    auto *again = new QPushButton(tr("Compare again"));
    connect(again, &QPushButton::clicked, this, &SystemsPage::compare);
    header->addWidget(again, 0, Qt::AlignBottom);
    column->addLayout(header);

    auto *sides = new QHBoxLayout;
    sides->setSpacing(14);
    sides->addWidget(sideCard(Card::On, tr("This system (%1)").arg(c.running.slot), tr("Running now."), c.running), 1);
    const QString otherIs = c.role == QLatin1String("before") ? tr("The system from before.")
                            : c.role == QLatin1String("next") ? tr("The next system, not started yet.")
                                                                : tr("The second copy.");
    sides->addWidget(sideCard(Card::Plain, tr("The other system (%1)").arg(c.other.slot), otherIs, c.other), 1);
    column->addLayout(sides);

    // How much is the same, as one bar with its legend.
    QList<qint64> differing(changeKinds().size(), 0), files(changeKinds().size(), 0);
    for (const Difference &difference : c.differences) {
        differing[difference.change] += std::max(difference.runningBytes, difference.otherBytes);
        ++files[difference.change];
    }
    column->addWidget(new AlikeBar(c.sameBytes, differing));
    auto *legend = new QHBoxLayout;
    legend->setSpacing(18);
    legend->addWidget(dim(text(tr("%1 the same (%2)")
                                   .arg(counted(c.sameFiles, tr("%1 file"), tr("%1 files")), bytesText(c.sameBytes)))));
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
    column->addLayout(legend);

    if (!said.isEmpty()) {
        auto *panel = new Panel;
        panel->column()->addWidget(strong(tr("What the agent says")));
        panel->column()->addWidget(text(said, true));
        column->addWidget(panel);
    }

    if (c.identical()) {
        column->addStretch();
    } else {
        auto *list = new QTreeWidget;
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
            for (const Difference *difference : group.value()) {
                const QString inside = group.key() == QLatin1String("/") ? difference->path.mid(1)
                                                                         : difference->path.mid(group.key().size() + 1);
                auto *line = new QTreeWidgetItem(head, {inside, changeKinds().at(difference->change).words,
                                                        sized(difference->runningBytes), sized(difference->otherBytes)});
                line->setIcon(1, swatch(changeKinds().at(difference->change).colour));
                line->setToolTip(0, difference->path
                                        + (difference->detail.isEmpty() ? QString() : QLatin1Char('\n') + difference->detail));
                if (!difference->detail.isEmpty())
                    line->setText(1, tr("%1: %2").arg(changeKinds().at(difference->change).words, difference->detail));
            }
            // A long folder stays folded: its count says enough until it is opened.
            head->setExpanded(group.value().size() <= 8);
        }
        column->addWidget(list, 1);
    }

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
        hand->setDefault(said.isEmpty());
        connect(hand, &QPushButton::clicked, this, &SystemsPage::handOver);
        foot->addWidget(hand);
    }
    column->addLayout(foot);
}
