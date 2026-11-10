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
    , agent_(qEnvironmentVariable("RECIPE_GUI_AGENT", "codex"))
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

// One question to `recipe`, answered later: the JSON it printed, or why not.
void RecipeClient::query(const QStringList &arguments, Answered then)
{
    auto *run = new Run(program_, arguments, this);
    connect(run, &Run::finished, this, [this, run, arguments, then](bool ok) {
        run->deleteLater();
        const QString asked = tr("`%1 %2`").arg(program_, arguments.join(' '));
        if (!ok) {
            then({}, tr("%1 failed: %2").arg(asked, run->problem()));
            return;
        }
        const QJsonObject object = QJsonDocument::fromJson(run->answer()).object();
        // `installed` carries no version of its own yet; an answer that names one must name ours.
        if (object.contains("schema_version") && object["schema_version"].toInt() != 1) {
            then({}, tr("%1 answered in a form this program does not know (schema %2)")
                         .arg(asked)
                         .arg(object["schema_version"].toInt()));
            return;
        }
        if (object.isEmpty()) {
            then({}, tr("%1 gave no answer this program can read").arg(asked));
            return;
        }
        then(object, QString());
    });
    run->start();
}

static void readList(const QJsonObject &list, Catalog &fresh)
{
    for (const QJsonValue &value : list["sources"].toArray()) {
        const QJsonObject o = value.toObject();
        Origin origin;
        origin.name = o["name"].toString();
        origin.title = o["title"].toString();
        origin.kind = o["kind"].toString();
        origin.address = o["address"].toString();
        origin.path = o["path"].toString();
        origin.commit = o["commit"].toString();
        origin.problem = o["problem"].toString();
        origin.checked = o["checked"].toInteger();
        fresh.origins << origin;
    }
    fresh.unreadable = list["problems"].toArray().size();
    for (const QJsonValue &value : list["recipes"].toArray()) {
        const Recipe recipe = Recipe::fromJson(value.toObject());
        // Two sources can hold one name; the first is the one Recipe uses.
        if (!fresh.recipe(recipe.name))
            fresh.recipes << recipe;
    }
}

static void readInstalled(const QJsonObject &installed, Catalog &fresh)
{
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
}

static void readUpdates(const QJsonObject &updates, Catalog &fresh)
{
    // `recipe updates` does not exist yet. Without it, an update is an
    // installed version that differs from the collection's, which is exactly
    // what `recipe update` acts on; what changed in the recipe stays unknown.
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
}

void RecipeClient::load(Loaded done)
{
    ++loading_;
    auto fresh = std::make_shared<Catalog>();
    auto finish = [this, done, fresh](bool ok, const QString &problem) {
        --loading_;
        done(ok, *fresh, problem);
    };
    query(QStringList{"list", "--json"} + collection() + store(), [this, fresh, finish](const QJsonObject &list, const QString &problem) {
        if (!problem.isEmpty())
            return finish(false, problem);
        readList(list, *fresh);
        query(QStringList{"installed", "--json"} + store(), [this, fresh, finish](const QJsonObject &installed, const QString &problem) {
            if (!problem.isEmpty())
                return finish(false, problem);
            readInstalled(installed, *fresh);
            query(QStringList{"updates", "--json"} + collection() + store(), [fresh, finish](const QJsonObject &updates, const QString &) {
                readUpdates(updates, *fresh);
                finish(true, QString());
            });
        });
    });
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

Run *RecipeClient::refresh(const QString &source)
{
    QStringList arguments{"refresh"};
    if (!source.isEmpty())
        arguments << source;
    return new Run(program_, arguments + store(), this);
}

Run *RecipeClient::addSource(const QString &addressOrFolder)
{
    QString where = addressOrFolder.trimmed();
    if (where.startsWith(QLatin1Char('~')))
        where = QDir::homePath() + where.mid(1);
    const bool folder = where.startsWith(QLatin1Char('/'));
    return change((folder ? QStringList{"source", "add", "--path", where} : QStringList{"source", "add", where}) + store());
}

Run *RecipeClient::removeSource(const QString &name)
{
    return change(QStringList{"source", "remove", name} + store());
}

Run *RecipeClient::agent(const QString &prompt)
{
    auto *run = new Run(agent_, {}, this);
    run->setPrompt(prompt);
    return run;
}

QString RecipeClient::agentName() const
{
    return QFileInfo(agent_).fileName();
}
