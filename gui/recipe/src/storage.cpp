#include "storage.h"

#include "parts.h"
#include "words.h"

#include <QDateTime>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QLocale>
#include <QPushButton>
#include <QScrollArea>
#include <QVBoxLayout>

#include <algorithm>

namespace {

QString shownName(const Catalog &catalog, const QString &name)
{
    const Recipe *recipe = catalog.recipe(name);
    return recipe ? recipe->shown() : name;
}

QString whenWords(qint64 seconds)
{
    const QDateTime when = QDateTime::fromSecsSinceEpoch(seconds);
    const QString time = QLocale().toString(when.time(), QLocale::ShortFormat);
    if (when.date() == QDate::currentDate())
        return QObject::tr("today at %1").arg(time);
    return QObject::tr("%1 at %2").arg(QLocale().toString(when.date(), QLocale::ShortFormat), time);
}

QString placeWords(Place place)
{
    return place == Place::Me ? QObject::tr("for me") : QObject::tr("for this computer");
}

// The newest earlier versions as names and versions: "tree 2.3.0, GIMP 3.0.2 and 1 more".
QString earlierNames(const Catalog &catalog, Place place)
{
    QStringList all;
    for (const InstalledItem &item : catalog.installed)
        if (item.place == place)
            for (const QString &version : item.earlier)
                all << QObject::tr("%1 %2").arg(shownName(catalog, item.name), version);
    if (all.isEmpty())
        return QString();
    const int shown = 3;
    QString words = all.mid(0, shown).join(QObject::tr(", "));
    if (all.size() > shown)
        words += QObject::tr(" and %1 more").arg(all.size() - shown);
    return words;
}

} // namespace

QString historySentence(const Catalog &catalog, const HistoryEntry &entry)
{
    const QString what = entry.what;
    const QString name = shownName(catalog, entry.package);
    if (what == QLatin1String("clear-earlier"))
        return QObject::tr("Cleared the earlier versions");
    if (what == QLatin1String("install")) {
        QString words = QObject::tr("Installed %1 %2").arg(name, entry.to);
        // `from` is set when it was installed over an installed version.
        if (!entry.from.isEmpty() && entry.from != entry.to)
            words = QObject::tr("Installed %1 %2, over %3").arg(name, entry.to, entry.from);
        return entry.source.isEmpty() ? words : QObject::tr("%1, from %2").arg(words, entry.source);
    }
    if (what == QLatin1String("update")) {
        const QString words = entry.from.isEmpty() || entry.from == entry.to
                                  ? QObject::tr("Updated %1 to %2").arg(name, entry.to)
                                  : QObject::tr("Updated %1 from %2 to %3").arg(name, entry.from, entry.to);
        return entry.source.isEmpty() ? words : QObject::tr("%1, from %2").arg(words, entry.source);
    }
    if (what == QLatin1String("remove"))
        return QObject::tr("Removed %1 %2").arg(name, entry.from);
    if (what == QLatin1String("go-back"))
        return QObject::tr("Went back to %1 %2, from %3").arg(name, entry.to, entry.from);
    if (what == QLatin1String("hold"))
        return QObject::tr("Kept %1 at %2").arg(name, entry.to);
    if (what == QLatin1String("unhold"))
        return QObject::tr("Let %1 update again").arg(name);
    // A kind of change this window does not know: say it as `recipe` did.
    return entry.package.isEmpty() ? entry.what : QObject::tr("%1 %2").arg(entry.what, entry.package);
}

void StoragePage::show(const Catalog &catalog, const Places &places)
{
    QVBoxLayout *column = fresh();
    lines_.clear();

    // The places a store stands behind, in the order the window names them.
    QList<Place> stores;
    for (Place place : {Place::Me, Place::Computer})
        if (catalog.stores[int(place)].has)
            stores << place;

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    if (stores.isEmpty()) {
        top->addWidget(heading(tr("Recipe could not say what it keeps")));
        top->addWidget(dim(text(tr("`recipe storage` gave no answer for any place."), true)));
    } else if (catalog.earlierGenerations == 0) {
        top->addWidget(heading(tr("No earlier versions are kept")));
        top->addWidget(dim(text(tr("Installed software takes %1.").arg(bytesText(catalog.activeBytes)), true)));
    } else {
        top->addWidget(heading(tr("Earlier versions take %1").arg(bytesText(catalog.earlierBytes))));
        top->addWidget(dim(text(tr("Kept beside what is installed, so that you can go back to them. Installed software "
                                   "takes %1.").arg(bytesText(catalog.activeBytes)),
                                true)));
    }
    header->addLayout(top, 1);
    // With one store the page has one button; with two, each row has its own.
    if (stores.size() == 1 && catalog.stores[int(stores.first())].earlierGenerations) {
        auto *clearThem = new QPushButton(tr("Clear earlier versions…"));
        clearThem->setDefault(true);
        clearThem->setToolTip(tr("Deletes every earlier version Recipe keeps. Nothing that is installed changes."));
        const Place place = stores.first();
        connect(clearThem, &QPushButton::clicked, this, [this, place] { emit clearEarlier(place); });
        header->addWidget(clearThem, 0, Qt::AlignBottom);
    }
    column->addLayout(header);

    auto *inner = new QWidget;
    auto *body = new QVBoxLayout(inner);
    body->setContentsMargins(0, 0, 2, 0);
    body->setSpacing(14);

    // What the earlier versions take, one row per store.
    if (!stores.isEmpty()) {
        auto *rows = new Panel;
        auto *grid = new QGridLayout;
        grid->setHorizontalSpacing(16);
        grid->setVerticalSpacing(8);
        grid->setColumnStretch(0, 1);
        grid->addWidget(dim(text(tr("What"))), 0, 0);
        grid->addWidget(dim(text(tr("Size"))), 0, 1);
        int row = 1;
        for (Place place : stores) {
            const Store &store = catalog.stores[int(place)];
            auto *words = new QVBoxLayout;
            words->setSpacing(1);
            words->addWidget(strong(catalog.twoPlaces ? tr("Earlier versions, %1").arg(placeWords(place))
                                                      : tr("Earlier versions of installed programs")));
            if (!store.earlierGenerations) {
                words->addWidget(dim(text(tr("None are kept."))));
            } else {
                const QString names = earlierNames(catalog, place);
                words->addWidget(dim(text(tr("%1. Kept so that you can go back%2. Clearing them ends going back; nothing "
                                             "that is installed changes.")
                                              .arg(counted(store.earlierGenerations, tr("%1 earlier generation"),
                                                           tr("%1 earlier generations")),
                                                   names.isEmpty() ? QString() : tr(": %1").arg(names)),
                                          true)));
            }
            grid->addLayout(words, row, 0);
            grid->addWidget(text(bytesText(store.earlierBytes)), row, 1, Qt::AlignTop);
            if (stores.size() > 1 && store.earlierGenerations) {
                auto *clearThem = new QPushButton(tr("Clear earlier versions, %1…").arg(placeWords(place)));
                connect(clearThem, &QPushButton::clicked, this, [this, place] { emit clearEarlier(place); });
                grid->addWidget(clearThem, row, 2, Qt::AlignTop);
            }
            ++row;
        }
        rows->column()->addLayout(grid);
        body->addWidget(rows);
    }

    // Installed programs, by place: the numbers `recipe storage` gives for each store.
    auto *rooms = new Panel;
    rooms->column()->addWidget(strong(tr("What is installed takes")));
    auto *grid = new QGridLayout;
    grid->setHorizontalSpacing(16);
    grid->setVerticalSpacing(6);
    grid->setColumnStretch(1, 1);
    int row = 0;
    auto line = [&](const QString &place, const QString &words) {
        grid->addWidget(dim(text(place)), row, 0, Qt::AlignTop);
        grid->addWidget(text(words, true), row, 1);
        ++row;
    };
    for (Place place : {Place::Computer, Place::Me}) {
        const Store &store = catalog.stores[int(place)];
        if (store.has) {
            const QString where = !catalog.twoPlaces ? tr("in %1").arg(places.systemRoot.isEmpty() ? tr("the store recipe uses") : places.systemRoot)
                                  : place == Place::Me ? tr("in your own folder")
                                                       : tr("in %1").arg(places.systemRoot);
            line(catalog.twoPlaces ? placeTitle(place) : tr("This store"), tr("%1, %2").arg(bytesText(store.activeBytes), where));
        } else if (place == Place::Computer && catalog.twoPlaces) {
            // A locked computer: recab answers with names and versions, no store stands behind them.
            line(placeTitle(place), tr("In the running system. Recipe reports no size for it."));
        }
    }
    int next = 0;
    for (const InstalledItem &item : catalog.installed)
        next += item.place == Place::Next;
    if (next)
        line(tr("Waiting for a restart"),
             tr("%1 only in the next system. Recipe reports no size for it.").arg(counted(next, tr("%1 program"), tr("%1 programs"))));
    rooms->column()->addLayout(grid);
    rooms->column()->addWidget(dim(text(tr("Only what `recipe storage` reports: what is installed and its earlier versions. Recipe "
                                           "reports no sizes for downloads or builds, so none are shown."),
                                        true)));
    auto *list = new QPushButton(tr("Show what is installed"));
    connect(list, &QPushButton::clicked, this, &StoragePage::openInstalled);
    rooms->column()->addWidget(list, 0, Qt::AlignLeft);
    body->addWidget(rooms);

    // What was done, newest first across the stores.
    struct Line {
        HistoryEntry entry;
        Place place;
    };
    QList<Line> done;
    bool known = false;
    for (Place place : stores) {
        known = known || catalog.stores[int(place)].historyKnown;
        for (const HistoryEntry &entry : catalog.stores[int(place)].history)
            done << Line{entry, place};
    }
    std::stable_sort(done.begin(), done.end(), [](const Line &a, const Line &b) { return a.entry.at > b.entry.at; });
    auto *history = new Panel;
    auto *historyTitle = new QVBoxLayout;
    historyTitle->setSpacing(0);
    historyTitle->addWidget(strong(tr("What was done")));
    historyTitle->addWidget(dim(text(tr("Newest first. Recipe keeps the newest entries of this record."))));
    history->column()->addLayout(historyTitle);
    if (done.isEmpty()) {
        history->column()->addWidget(dim(text(known ? tr("Nothing has been done to the installed software yet")
                                                    : tr("Recipe could not say what was done."), true)));
    } else {
        auto *entries = new QGridLayout;
        entries->setHorizontalSpacing(16);
        entries->setVerticalSpacing(6);
        entries->setColumnStretch(0, 1);
        int at = 0;
        for (const Line &each : done) {
            const QString sentence = historySentence(catalog, each.entry);
            lines_ << sentence;
            entries->addWidget(text(sentence, true), at, 0);
            entries->addWidget(dim(text(catalog.twoPlaces ? tr("%1 · %2").arg(whenWords(each.entry.at), placeWords(each.place))
                                                          : whenWords(each.entry.at))),
                               at, 1);
            ++at;
        }
        history->column()->addLayout(entries);
    }
    body->addWidget(history);
    body->addStretch();

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setWidget(inner);
    column->addWidget(scroll, 1);
}
