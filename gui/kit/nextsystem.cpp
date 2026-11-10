#include "nextsystem.h"

#include "run.h"

#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>

NextSystem::NextSystem(const QString &recab, QObject *parent)
    : QObject(parent)
    , recab_(recab)
{
}

void NextSystem::read()
{
    if (running_)
        return;
    running_ = new Run(recab_, {"status", "--json"}, this);
    connect(running_, &Run::finished, this, [this](bool ok) {
        Run *run = running_;
        running_ = nullptr;
        QStringList titles;
        // A status that cannot be read says nothing: the strip stays away
        // rather than guess.
        if (!ok || !parse(run->answer(), &titles))
            titles.clear();
        run->deleteLater();
        titles_ = titles;
        emit known();
    });
    running_->start();
}

// The same words as the System app uses for a change, so the strip and the
// System page never differ.
static QString titleOf(const QJsonObject &change)
{
    const QString action = change["action"].toString(), package = change["package"].toString();
    QString title;
    if (action == QLatin1String("install"))
        title = QObject::tr("%1, installed").arg(package);
    else if (action == QLatin1String("update"))
        title = QObject::tr("%1, updated").arg(package);
    else if (action == QLatin1String("remove"))
        title = QObject::tr("%1, removed").arg(package);
    else if (action == QLatin1String("edit"))
        title = QObject::tr("%1, edited").arg(package);
    else
        title = QObject::tr("%1 (%2)").arg(package, action);
    if (change["actor"].toString().startsWith(QLatin1String("agent")))
        title = QObject::tr("%1, by the agent").arg(title);
    return title;
}

bool NextSystem::parse(const QByteArray &json, QStringList *titles)
{
    titles->clear();
    const QJsonObject object = QJsonDocument::fromJson(json).object();
    const QJsonObject lifecycle = object["lifecycle"].toObject();
    if (lifecycle.isEmpty())
        return false;
    if (lifecycle["other"].toString() != QLatin1String("next"))
        return true; // read, and there is no next system
    for (const QJsonValue &change : object["preparation"].toObject()["changes"].toArray())
        *titles << titleOf(change.toObject());
    return true;
}
