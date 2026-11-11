#include "after.h"

#include "parts.h"
#include "mess.h"
#include "words.h"

#include <QGridLayout>
#include <QHBoxLayout>
#include <QPushButton>
#include <QScrollArea>
#include <QVBoxLayout>

namespace {

QWidget *figure(const QString &title, const QString &value, const QString &note)
{
    auto *box = new Panel;
    auto *inside = box->column();
    inside->setSpacing(1);
    inside->addWidget(dim(text(title)));
    QLabel *number = heading(value);
    inside->addWidget(number);
    if (!note.isEmpty())
        inside->addWidget(dim(text(note, true)));
    return box;
}

QString doing(const AfterRow &row)
{
    switch (row.kind) {
    case AfterRow::SetAside:
        return row.words.isEmpty() ? QObject::tr("Set aside") : QObject::tr("Set aside: %1").arg(row.words);
    case AfterRow::Kept:
        return QObject::tr("Kept: %1").arg(row.words);
    case AfterRow::Left:
        return QObject::tr("Left alone: %1").arg(row.words);
    case AfterRow::CouldNot:
        return QObject::tr("Could not: %1").arg(row.words);
    case AfterRow::NoWord:
        return QObject::tr("The agent said nothing about this, and it is not in the holding folder.");
    case AfterRow::Unrequested:
        return QObject::tr("Set aside, but it was not on the list you handed over. %1").arg(row.words);
    case AfterRow::Missing:
        return QObject::tr("The manifest names it, but it is not in the holding folder: %1").arg(row.words);
    case AfterRow::BroughtBack:
        return QObject::tr("Brought back: it is where it was");
    case AfterRow::LetGo:
        return QObject::tr("Let go: deleted from the holding folder");
    }
    return {};
}

} // namespace

void AfterPage::showReading()
{
    showLoading(tr("Looking at what was set aside"), tr("Reading the holding folder. Nothing is changed."));
}

void AfterPage::show(const AfterData &data)
{
    shown_ = data;
    QVBoxLayout *column = fresh();
    column->setSpacing(10);

    auto *backLink = new ClickLabel(tr("‹ Mess"));
    dim(backLink);
    connect(backLink, &ClickLabel::clicked, this, &AfterPage::back);
    column->addWidget(backLink, 0, Qt::AlignLeft);

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    const bool holds = data.heldPlaces > 0;
    top->addWidget(heading(holds ? tr("%1 is set aside").arg(bytesText(data.heldBytes)) : tr("Nothing is set aside")));
    QString under;
    if (holds)
        under = data.sameDisk ? tr("Nothing is gone yet. What was set aside sits in a holding folder on the same disk, so no "
                                   "room is free until you let it go. It stays until you bring it back or let it go.")
                              : tr("Nothing is gone yet. What was set aside sits in a holding folder that is not on the disk your "
                                   "home folder is on. It stays until you bring it back or let it go.");
    else
        under = data.letGone ? tr("What was set aside has been let go.") : tr("The holding folder holds nothing.");
    top->addWidget(dim(text(under, true)));
    header->addLayout(top, 1);
    if (holds) {
        auto *all = new QPushButton(tr("Bring everything back"));
        connect(all, &QPushButton::clicked, this, &AfterPage::bringAllBack);
        header->addWidget(all, 0, Qt::AlignBottom);
        auto *go = new QPushButton(tr("Let it all go, free %1").arg(bytesText(data.heldBytes)));
        go->setDefault(true);
        connect(go, &QPushButton::clicked, this, &AfterPage::letGo);
        header->addWidget(go, 0, Qt::AlignBottom);
    }
    column->addLayout(header);

    for (const QString &note : data.notes) {
        auto *card = new Card(Card::Plain);
        card->column()->addWidget(text(note, true));
        column->addWidget(card);
    }
    for (const QString &problem : data.problems) {
        auto *card = new Card(Card::Wrong);
        card->column()->addWidget(text(problem, true));
        column->addWidget(card);
    }

    auto *figures = new QHBoxLayout;
    figures->setSpacing(10);
    const QString measured = tr("measured on the disk, not counted from the list");
    if (data.ranHere)
        figures->addWidget(figure(tr("Free before"), data.freeBefore >= 0 ? bytesText(data.freeBefore) : tr("not measured"),
                                  tr("when you handed it over")));
    figures->addWidget(figure(tr("Free now"), data.freeNow >= 0 ? bytesText(data.freeNow) : tr("not measured"), measured));
    if (holds && data.freeNow >= 0)
        figures->addWidget(figure(tr("Free once you let it go"), bytesText(data.freeNow + data.heldBytes),
                                  tr("what is free now, plus what the holding folder takes")));
    column->addLayout(figures);

    if (!data.rows.isEmpty()) {
        auto *table = new Panel;
        auto *tableBody = new QWidget;
        table->column()->setContentsMargins(0, 0, 0, 0);
        table->column()->addWidget(tableBody);
        auto *grid = new QGridLayout(tableBody);
        grid->setContentsMargins(14, 6, 14, 6);
        grid->setHorizontalSpacing(14);
        grid->setVerticalSpacing(0);
        grid->setColumnStretch(2, 1);
        const QStringList titles{tr("Place"), tr("Size"), tr("What the agent did")};
        grid->addWidget(dim(text(titles[0])), 0, 0);
        grid->addWidget(dim(text(titles[1])), 0, 1);
        grid->addWidget(dim(text(titles[2])), 0, 2);
        int row = 1;
        for (const AfterRow &one : data.rows) {
            grid->addWidget(rule(), row++, 0, 1, 4);
            QLabel *place = text(shortPath(one.path, data.home));
            place->setToolTip(one.path);
            place->setMinimumHeight(34);
            grid->addWidget(place, row, 0);
            grid->addWidget(dim(text(one.bytes > 0 ? bytesText(one.bytes) : QString())), row, 1);
            grid->addWidget(text(doing(one), true), row, 2);
            QPushButton *act = nullptr;
            const QString path = one.path;
            if (one.kind == AfterRow::SetAside || one.kind == AfterRow::Unrequested) {
                act = new QPushButton(tr("Bring back"));
                connect(act, &QPushButton::clicked, this, [this, path] { emit bringBack(path); });
            } else if (one.kind == AfterRow::Kept) {
                act = new QPushButton(tr("Never suggest"));
                connect(act, &QPushButton::clicked, this, [this, path] { emit neverSuggest(path); });
            } else if (one.kind == AfterRow::CouldNot) {
                act = new QPushButton(tr("Try again"));
                connect(act, &QPushButton::clicked, this, [this, path] { emit tryAgain(path); });
            }
            if (act)
                grid->addWidget(act, row, 3, Qt::AlignRight | Qt::AlignVCenter);
            ++row;
        }
        auto *scroll = new QScrollArea;
        scroll->setWidgetResizable(true);
        scroll->setFrameShape(QFrame::NoFrame);
        scroll->setWidget(table);
        column->addWidget(scroll, 1);
    }
    if (data.unlisted > (1 << 20))
        column->addWidget(dim(text(tr("%1 in the holding folder is not in any manifest; letting go deletes it too.")
                                       .arg(bytesText(data.unlisted)),
                                   true)));
    column->addStretch();
    auto *activity = new ClickLabel(tr("Everything the agent did and said, in Activity"));
    activity->setForegroundRole(QPalette::Link);
    connect(activity, &ClickLabel::clicked, this, &AfterPage::openActivity);
    column->addWidget(activity, 0, Qt::AlignLeft);
}
