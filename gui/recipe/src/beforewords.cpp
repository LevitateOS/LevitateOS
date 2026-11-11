#include "beforewords.h"

#include "words.h"

#include <QObject>

QString endpointText(const QString &endpoint)
{
    const qsizetype colon = endpoint.lastIndexOf(':');
    if (colon < 0)
        return endpoint;
    return QObject::tr("%1 (port %2)").arg(endpoint.left(colon), endpoint.mid(colon + 1));
}

QString endpointsText(const QStringList &endpoints)
{
    QStringList shown;
    for (const QString &endpoint : endpoints)
        shown << endpointText(endpoint);
    return shown.join(QObject::tr(", "));
}

QString placeKey(Place place)
{
    return QString::number(int(place));
}

Place placeFromKey(const QString &key)
{
    return Place(key.toInt());
}

namespace {

QString listWords(const QStringList &names)
{
    if (names.size() < 2)
        return names.join(QString());
    return QObject::tr("%1 and %2").arg(names.mid(0, names.size() - 1).join(QObject::tr(", ")), names.last());
}

// The two lines under a place's name.
BeforePage::Choice placeCard(const Places &places, Place place)
{
    BeforePage::Choice card;
    card.key = placeKey(place);
    const QString password = places.password ? QObject::tr("Asks for the administrator password.")
                                             : QObject::tr("This window makes the change itself; no password is asked.");
    if (places.one) {
        card.name = places.systemRoot.isEmpty() ? QObject::tr("The store recipe uses") : QObject::tr("In %1").arg(places.systemRoot);
        card.first = QObject::tr("The one store this window was started with.");
        card.second = QObject::tr("It is the only place this window changes.");
    } else if (place == Place::Me) {
        card.name = placeTitle(Place::Me);
        card.first = QObject::tr("Into your own folder. It works at once, for you only.");
        card.second = QObject::tr("No password.");
    } else if (places.locked) {
        card.name = placeTitle(Place::Computer);
        card.first = QObject::tr("Into the next system. It works after a restart, for everyone.");
        card.second = password;
    } else {
        card.name = placeTitle(Place::Computer);
        card.first = QObject::tr("Into this computer's software folder, %1. It works at once, for everyone.").arg(places.systemRoot);
        card.second = password;
    }
    return card;
}

QString hostOf(const QString &endpoint)
{
    const qsizetype colon = endpoint.lastIndexOf(':');
    return colon < 0 ? endpoint : endpoint.left(colon);
}

// Where it is fetched from is what the recipe declares it may reach.
QString fetchSentence(const Recipe &recipe)
{
    QStringList hosts;
    for (const QString &endpoint : recipe.needs.network)
        if (!hosts.contains(hostOf(endpoint)))
            hosts << hostOf(endpoint);
    const QString what = QObject::tr("%1 %2").arg(recipe.shown(), recipe.version);
    if (hosts.isEmpty())
        return recipe.hooks.contains("build")
                   ? QObject::tr("%1 is built on this computer, inside the sandbox. Nothing is fetched from the network.").arg(what)
                   : QObject::tr("%1 is put in place by its recipe, inside the sandbox. Nothing is fetched from the network.").arg(what);
    return recipe.hooks.contains("build")
               ? QObject::tr("%1 is fetched from %2 and built on this computer, inside the sandbox.").arg(what, listWords(hosts))
               : QObject::tr("%1 is fetched from %2 and put in place by its recipe, inside the sandbox.").arg(what, listWords(hosts));
}

// The recipes it needs that are not there yet are installed with it; the
// ones only the build needs are said apart.
QString needsSentence(const Catalog &catalog, const Recipe &recipe, Place place)
{
    auto there = [&](const QString &name) {
        return catalog.installedItem(name, place) || (place == Place::Computer && catalog.installedItem(name, Place::Next));
    };
    auto shown = [&](const QString &name) {
        const Recipe *found = catalog.recipe(name);
        return found ? found->shown() : name;
    };
    QStringList run, build;
    bool any = false;
    for (const QString &name : recipe.deps) {
        any = true;
        if (!there(name))
            run << shown(name);
    }
    for (const QString &name : recipe.buildDeps) {
        any = true;
        if (!there(name) && !recipe.deps.contains(name))
            build << shown(name);
    }
    if (!any)
        return QString();
    if (run.isEmpty() && build.isEmpty())
        return QObject::tr("The other recipes it needs are installed already.");
    QStringList parts;
    if (!run.isEmpty())
        parts << QObject::tr("Installed with it, because it needs them: %1.").arg(listWords(run));
    if (!build.isEmpty())
        parts << QObject::tr("Needed only to build it: %1.").arg(listWords(build));
    return parts.join(QLatin1Char(' '));
}

// One change of what a recipe may use, as the Updates page says it.
QString changeWords(const Change &change)
{
    if (!change.isList()) {
        const QString how = change.kind == QLatin1String("raised")    ? QObject::tr("more than before")
                            : change.kind == QLatin1String("lowered") ? QObject::tr("less than before")
                                                                      : QObject::tr("changed");
        return QObject::tr("%1 → %2 (%3)").arg(Change::amount(change.before), Change::amount(change.after), how);
    }
    const bool network = change.field == QLatin1String("network");
    auto shown = [network](const QStringList &names) { return network ? endpointsText(names) : names.join(QObject::tr(", ")); };
    QStringList parts;
    if (!change.added().isEmpty())
        parts << QObject::tr("New: %1.").arg(shown(change.added()));
    if (!change.removed().isEmpty())
        parts << QObject::tr("No longer: %1.").arg(shown(change.removed()));
    if (!change.stays().isEmpty())
        parts << QObject::tr("As before: %1.").arg(shown(change.stays()));
    return parts.join(QLatin1Char(' '));
}

QList<QPair<QString, QString>> limits(const Recipe &recipe, bool all)
{
    QList<QPair<QString, QString>> rows;
    rows.append({QObject::tr("Network"), recipe.needs.network.isEmpty()
                                             ? QObject::tr("none")
                                             : QObject::tr("%1. Nothing else.").arg(endpointsText(recipe.needs.network))});
    if (recipe.needs.networkMaxBytes)
        rows.append({QObject::tr("Most it may download"), QObject::tr("up to %1").arg(bytesText(recipe.needs.networkMaxBytes))});
    if (recipe.needs.memoryMib)
        rows.append({QObject::tr("Memory"), QObject::tr("up to %1").arg(bytesText(recipe.needs.memoryMib * 1048576))});
    if (!all)
        return rows;
    if (recipe.needs.storageMaxBytes)
        rows.append({QObject::tr("Disk while it builds"), QObject::tr("up to %1").arg(bytesText(recipe.needs.storageMaxBytes))});
    if (recipe.needs.timeoutSecs)
        rows.append({QObject::tr("Time"), counted(recipe.needs.timeoutSecs / 60, QObject::tr("up to %1 minute"),
                                                  QObject::tr("up to %1 minutes"))});
    rows.append({QObject::tr("Your files"), QObject::tr("none")});
    return rows;
}

} // namespace

BeforePage::Content beforeContent(const Catalog &catalog, const Places &places, const Ask &ask, Place chosen)
{
    BeforePage::Content content;
    const Recipe *found = catalog.recipe(ask.name);
    const Recipe recipe = found ? *found : Recipe{};
    const UpdateItem *update = ask.update ? catalog.update(ask.name, ask.place) : nullptr;
    const QString shown = recipe.shown().isEmpty() ? ask.name : recipe.shown();
    const Place place = ask.update ? ask.place : chosen;

    // The question. With a choice of places it names the one picked.
    if (ask.update)
        content.title = update && !update->sameVersion ? QObject::tr("Update %1 to %2?").arg(shown, update->to)
                                                       : QObject::tr("Update %1?").arg(shown);
    else if (ask.choices.size() > 1)
        content.title = place == Place::Me ? QObject::tr("Install %1 for me?").arg(shown)
                                           : QObject::tr("Install %1 for this computer?").arg(shown);
    else
        content.title = QObject::tr("Install %1?").arg(shown);
    content.go = ask.update ? QObject::tr("Update") : QObject::tr("Install");

    // Where it goes: a choice for an install, a statement for an update.
    content.whereTitle = ask.update ? QObject::tr("Where it is") : QObject::tr("Where it goes");
    for (Place each : ask.update ? QList<Place>{ask.place} : ask.choices)
        content.choices << placeCard(places, each);
    content.chosen = placeKey(place);

    // What happens: only what the catalog says.
    content.happensTitle = QObject::tr("What happens");
    if (update) {
        content.happens << (update->sameVersion
                                ? QObject::tr("%1 stays at version %2; its source now publishes another recipe file for it, "
                                              "and that one is used.").arg(shown, update->from)
                                : QObject::tr("%1 goes from version %2 to %3.").arg(shown, update->from, update->to));
    }
    content.happens << fetchSentence(recipe);
    const QString needs = needsSentence(catalog, recipe, place);
    if (!needs.isEmpty())
        content.happens << needs;
    if (places.locked && !places.one && place != Place::Me)
        content.happens << QObject::tr("The running system is not touched.");

    // What the recipe may use. For an update: what `recipe updates` said changed.
    content.tableTitle = QObject::tr("What the recipe may use");
    const QString sandbox = QObject::tr("The sandbox refuses anything else.");
    if (!update) {
        content.table = limits(recipe, true);
        content.tableNote = sandbox;
    } else if (!update->changesKnown) {
        content.table = limits(recipe, false);
        content.tableNote = update->fromCollection
                                ? QObject::tr("What version %1 was allowed to use is not known here, so the two cannot be "
                                              "compared; this is what the recipe allows now. %2").arg(update->from, sandbox)
                                : QObject::tr("Version %1 was installed before Recipe recorded what a recipe may use, so the "
                                              "two cannot be compared; this is what the newer recipe allows. %2")
                                      .arg(update->from, sandbox);
    } else {
        for (const Change &change : update->changes)
            content.table.append({change.title(), changeWords(change)});
        content.table.append(update->changes.isEmpty()
                                 ? QPair<QString, QString>{QObject::tr("What it may use"), QObject::tr("the same as before: it asks for nothing new")}
                                 : QPair<QString, QString>{QObject::tr("Everything else"), QObject::tr("as before")});
        content.tableNote = QObject::tr("Compared with the recipe version %1 was installed from. %2").arg(update->from, sandbox);
    }

    // The foot: where the recipe is from, and the recipe itself when its file is known.
    const Origin *origin = catalog.origin(recipe.origin);
    content.foot = origin ? QObject::tr("From %1.").arg(origin->shown()) : QString();
    if (!recipe.path.isEmpty())
        content.linkWords = QObject::tr("Read the recipe");
    return content;
}
