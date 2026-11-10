#include "client.h"

#include <QJsonArray>
#include <QJsonDocument>

Run::Run(const QString &program, const QStringList &arguments, QObject *parent)
    : QObject(parent)
{
    process_.setProgram(program);
    process_.setArguments(arguments);
    connect(&process_, &QProcess::readyReadStandardError, this, &Run::drain);
    connect(&process_, &QProcess::finished, this, [this](int code, QProcess::ExitStatus status) {
        drain();
        emit finished(status == QProcess::NormalExit && code == 0);
    });
    connect(&process_, &QProcess::errorOccurred, this, [this](QProcess::ProcessError error) {
        if (error != QProcess::FailedToStart)
            return;
        failure_ = QJsonObject{{"reason", tr("%1 could not be started").arg(process_.program())}};
        emit finished(false);
    });
}

void Run::start()
{
    process_.start();
}

void Run::stop()
{
    process_.terminate();
}

void Run::drain()
{
    pending_ += process_.readAllStandardError();
    int end;
    while ((end = pending_.indexOf('\n')) >= 0) {
        const QByteArray line = pending_.left(end);
        pending_.remove(0, end + 1);
        const QJsonObject object = line.startsWith('{') ? QJsonDocument::fromJson(line).object() : QJsonObject();
        if (object.isEmpty()) {
            if (!line.trimmed().isEmpty())
                emit output(QString::fromUtf8(line));
            continue;
        }
        if (object["event"] == QLatin1String("failed"))
            failure_ = object;
        if (object["event"] == QLatin1String("output"))
            emit output(object["line"].toString());
        else
            emit event(object);
    }
}

RecipeClient::RecipeClient(QObject *parent)
    : QObject(parent)
    , program_(qEnvironmentVariable("RECIPE_GUI_RECIPE", "recipe"))
    , rootHelper_(qEnvironmentVariable("RECIPE_GUI_ROOT_HELPER", "pkexec"))
{
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
    if (object["schema_version"].toInt() != 1) {
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
    const QJsonObject list = query({"list", "--json"}, error);
    if (list.isEmpty())
        return false;
    fresh.sourceTitle = list["source"].toObject()["title"].toString();
    fresh.checked = list["source"].toObject()["checked"].toString();
    for (const QJsonValue &value : list["recipes"].toArray())
        fresh.recipes << Recipe::fromJson(value.toObject());

    const QJsonObject installed = query({"installed", "--json"}, error);
    if (installed.isEmpty())
        return false;
    for (const QJsonValue &value : installed["installed"].toArray()) {
        const QJsonObject o = value.toObject();
        InstalledItem item;
        item.name = o["name"].toString();
        item.version = o["version"].toString();
        item.kept = o["kept"].toArray().isEmpty() ? QString() : o["kept"].toArray().first().toString();
        item.installedOn = QDate::fromString(o["installed_on"].toString(), Qt::ISODate);
        item.sizeBytes = o["size_bytes"].toInteger();
        item.keptBytes = o["kept_bytes"].toInteger();
        for (const QJsonValue &program : o["programs"].toArray())
            item.programs << program.toString();
        for (const QJsonValue &entry : o["menu"].toArray())
            item.menu << entry.toString();
        fresh.installed << item;
    }

    const QJsonObject updates = query({"updates", "--json"}, error);
    if (updates.isEmpty())
        return false;
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
    return change({"install", name, "--machine-events"});
}

Run *RecipeClient::remove(const QString &name, bool clearKept)
{
    QStringList arguments{"remove", name, "--machine-events"};
    if (clearKept)
        arguments << "--clear-kept";
    return change(arguments);
}

Run *RecipeClient::goBack(const QString &name)
{
    return change({"go-back", name, "--machine-events"});
}
