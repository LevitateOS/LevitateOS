#include "client.h"

#include <QFileInfo>
#include <QJsonArray>
#include <QJsonDocument>
#include <QDir>
#include <QRegularExpression>
#include <QTemporaryFile>

RecipeClient::RecipeClient(QObject *parent)
    : QObject(parent)
    , program_(qEnvironmentVariable("RECIPE_GUI_RECIPE", "recipe"))
    , rootHelper_(qEnvironmentVariable("RECIPE_GUI_ROOT_HELPER", "pkexec"))
    , recipesPath_(qEnvironmentVariable("RECIPE_GUI_RECIPES_PATH"))
    , packageRoot_(qEnvironmentVariable("RECIPE_GUI_PACKAGE_ROOT", "/var/lib/recipe"))
{
}

// The collection to list and install from: `recipe`'s own choice unless told.
QStringList RecipeClient::collection() const
{
    return recipesPath_.isEmpty() ? QStringList() : QStringList{"--recipes-path", recipesPath_};
}

// The system's software, which root owns, not the desktop user's own store.
QStringList RecipeClient::store() const
{
    return packageRoot_.isEmpty() ? QStringList() : QStringList{"--package-root", packageRoot_};
}

QJsonObject RecipeClient::query(const QStringList &arguments, QString *error)
{
    QProcess process;
    process.start(program_, arguments);
    if (!process.waitForFinished(15000) || process.exitCode() != 0) {
        *error = tr("`%1 %2` failed: %3")
                     .arg(program_, arguments.join(' '),
                          process.error() == QProcess::FailedToStart
                              ? process.errorString()
                              : QString::fromUtf8(process.readAllStandardError()).trimmed());
        return {};
    }
    const QJsonObject object = QJsonDocument::fromJson(process.readAllStandardOutput()).object();
    // `installed` carries no version of its own yet; an answer that names one must name ours.
    if (object.contains("schema_version") && object["schema_version"].toInt() != 1) {
        *error = tr("`%1 %2` answered in a form this program does not know (schema %3)")
                     .arg(program_, arguments.join(' '))
                     .arg(object["schema_version"].toInt());
        return {};
    }
    return object;
}

bool RecipeClient::load(Catalog *into, QString *error)
{
    Catalog fresh;
    const QJsonObject list = query(QStringList{"list", "--json"} + collection(), error);
    if (list.isEmpty())
        return false;
    fresh.sourceTitle = list["source"].toObject()["title"].toString();
    fresh.checked = list["source"].toObject()["checked"].toString();
    if (fresh.sourceTitle.isEmpty()) {
        QStringList folders;
        for (const QJsonValue &folder : list["collections"].toArray())
            folders << folder.toString();
        fresh.sourceTitle = tr("Recipes from %1").arg(folders.join(tr(", ")));
    }
    fresh.unreadable = list["problems"].toArray().size();
    for (const QJsonValue &value : list["recipes"].toArray())
        fresh.recipes << Recipe::fromJson(value.toObject());

    const QJsonObject installed = query(QStringList{"installed", "--json"} + store(), error);
    if (installed.isEmpty())
        return false;
    const QString profile = installed["profile"].toString();
    for (const QJsonValue &value : installed["packages"].toArray()) {
        const QJsonObject o = value.toObject();
        InstalledItem item;
        item.name = o["name"].toString();
        item.version = o["version"].toString();
        item.kept = o["kept"].toArray().isEmpty() ? QString() : o["kept"].toArray().first().toString();
        item.installedOn = QDate::fromString(o["installed_on"].toString(), Qt::ISODate);
        item.sizeBytes = o["size_bytes"].toInteger();
        item.keptBytes = o["kept_bytes"].toInteger();
        // What it put on the system, from the receipt's own list of files.
        for (const QJsonValue &file : o["files"].toArray()) {
            const QString path = file.toString();
            if (path.startsWith("bin/"))
                item.programs << path.mid(4);
            else if (path.startsWith("share/applications/") && path.endsWith(".desktop"))
                item.menu << path.mid(19).chopped(8);
            // Until the receipt carries a size, measure what the profile holds.
            if (!o.contains("size_bytes") && !profile.isEmpty())
                item.sizeBytes += QFileInfo(profile + '/' + path).size();
        }
        fresh.installed << item;
    }

    // `recipe updates` does not exist yet. Without it, an update is an
    // installed version that differs from the collection's, which is exactly
    // what `recipe update` acts on; what changed in the recipe stays unknown.
    QString noUpdates;
    const QJsonObject updates = query(QStringList{"updates", "--json"} + collection() + store(), &noUpdates);
    if (updates.isEmpty()) {
        for (const InstalledItem &item : fresh.installed) {
            const Recipe *recipe = fresh.recipe(item.name);
            if (recipe && !recipe->version.isEmpty() && recipe->version != item.version) {
                UpdateItem update{item.name, item.version, recipe->version, {}};
                update.changesKnown = false;
                fresh.updates << update;
            }
        }
    }
    for (const QJsonValue &value : updates["updates"].toArray()) {
        const QJsonObject o = value.toObject();
        UpdateItem item{o["name"].toString(), o["from"].toString(), o["to"].toString(), {}};
        for (const QJsonValue &change : o["changes"].toArray()) {
            const QJsonObject c = change.toObject();
            item.changes << Change{c["field"].toString(), c["before"].toString(), c["after"].toString(),
                                   c["kind"].toString()};
        }
        fresh.updates << item;
    }
    *into = fresh;
    return true;
}

Run *RecipeClient::change(const QStringList &arguments)
{
    if (rootHelper_.isEmpty())
        return new Run(program_, arguments, this);
    return new Run(rootHelper_, QStringList{program_} + arguments, this);
}

Run *RecipeClient::install(const QString &name)
{
    return change(QStringList{"install", name, "--machine-events"} + collection() + store());
}

Run *RecipeClient::update(const QString &name)
{
    return change(QStringList{"update", name, "--machine-events"} + collection() + store());
}

Run *RecipeClient::fixWithAgent(const QString &name, const QString &prompt, bool installed)
{
    auto *file = new QTemporaryFile(QDir::tempPath() + "/recipe-gui-prompt-XXXXXX.txt");
    if (file->open()) {
        file->write(prompt.toUtf8());
        file->close();
    }
    Run *run = change(QStringList{installed ? "update" : "install", name, "--machine-events", "--autofix",
                                  "--autofix-prompt-file", file->fileName()}
                      + collection() + store());
    file->setParent(run); // removed with the run
    return run;
}

Run *RecipeClient::remove(const QString &name, bool clearKept)
{
    QStringList arguments = QStringList{"remove", name, "--machine-events"} + collection() + store();
    if (clearKept)
        arguments << "--clear-kept";
    return change(arguments);
}

Run *RecipeClient::goBack(const QString &name)
{
    return change(QStringList{"go-back", name, "--machine-events"} + store());
}
