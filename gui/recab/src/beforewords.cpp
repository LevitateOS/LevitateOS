#include "beforewords.h"

#include <QObject>

namespace beforewords {
namespace {

// The running system's name, or its plain description when recab gave none.
QString runningName(const Status &s)
{
    const QString name = s.name(s.runningRelease);
    return name.isEmpty() ? QObject::tr("the running system") : name;
}

QString fromBefore(const Status &s)
{
    return s.otherRelease.isEmpty() ? QObject::tr("the system from before") : s.name(s.otherRelease);
}

// What the next system holds, from the changes recab lists.
QString holds(const Status &s)
{
    const QStringList titles = s.changeTitles();
    if (titles.isEmpty())
        return QObject::tr("The next system holds no changes yet.");
    const QString count = titles.size() == 1 ? QObject::tr("one change") : QObject::tr("%1 changes").arg(titles.size());
    return QObject::tr("The next system holds %1: %2.").arg(count, titles.join(QObject::tr("; ")));
}

// Whether recab marks the operation destructive (the real recab marks discard).
bool destructive(const Status &s, const QString &id)
{
    return s.operationObject(id)["destructive"].toBool();
}

// The sentence for what the start-up judges, as the page of a trial says it.
QString trialWords(bool toRunning)
{
    return toRunning
               ? QObject::tr("If the next system does not start properly, the computer comes back to the running "
                             "system by itself. Its changes are not lost.")
               : QObject::tr("recab keeps the new system by itself as soon as start-up has finished with nothing "
                             "failed. It waits up to %1 minutes for that.")
                     .arg(recabTrialMinutes);
}

// The "What runs" table: each command in turn, then the password.
void runs(BeforePage::Content &c, const RecabClient &client, const QStringList &commands)
{
    c.tableTitle = QObject::tr("What runs");
    for (qsizetype i = 0; i < commands.size(); ++i)
        c.table.append({i == 0 ? QObject::tr("Runs") : QObject::tr("Then"), commands[i]});
    if (!client.asksPassword())
        c.table.append({QObject::tr("Password"), QObject::tr("This window runs it itself; no password is asked.")});
    else if (commands.size() > 1)
        c.table.append({QObject::tr("Password"), QObject::tr("The administrator password is asked for each of them.")});
    else
        c.table.append({QObject::tr("Password"), QObject::tr("The administrator password is asked for.")});
}

BeforePage::Content make(const QString &title, const QStringList &happens, const RecabClient &client,
                         const QStringList &commands, const QString &go)
{
    BeforePage::Content c;
    c.title = title;
    c.happensTitle = QObject::tr("What happens");
    c.happens = happens;
    runs(c, client, commands);
    c.go = go;
    return c;
}

const QString restartCommand = QStringLiteral("reboot");

} // namespace

BeforePage::Content finish(const RecabClient &client, const Status &s)
{
    return make(QObject::tr("Finish the next system?"),
                {holds(s),
                 QObject::tr("The next system is checked and closed, so it can be started. Until then a restart "
                             "cannot start it."),
                 QObject::tr("The computer does not restart, and %1 is not touched. You choose afterwards when to "
                             "restart into the next system.")
                     .arg(runningName(s))},
                client, {client.commandWords({"finalize"})}, QObject::tr("Finish"));
}

BeforePage::Content restartNow(const RecabClient &client, const Status &s)
{
    const bool set = s.state == QLatin1String("trial_pending");
    QStringList happens;
    QStringList commands;
    if (set) {
        happens << QObject::tr("A restart into the next system is set already, so only the restart is left.");
    } else {
        happens << QObject::tr("recab sets the next system to start at the next restart. Then the computer restarts.");
        commands << client.commandWords({"set-next"});
    }
    commands << restartCommand;
    happens << QObject::tr("The computer restarts now into the next system, and programs that are open are closed. "
                           "Once the restart has begun, this window cannot stop it.")
            << trialWords(false) << trialWords(true);
    return make(set ? QObject::tr("Restart now?") : QObject::tr("Restart into the next system?"), happens, client,
                commands, set ? QObject::tr("Restart") : QObject::tr("Restart into it"));
}

BeforePage::Content restartLater(const RecabClient &client, const Status &s)
{
    return make(QObject::tr("Start the next system at the next restart?"),
                {QObject::tr("recab sets the next system to start at the next restart. The computer does not restart "
                             "now; until then you stay on %1.")
                     .arg(runningName(s)),
                 QObject::tr("Any restart starts the next system, whoever makes it, until you cancel the set restart "
                             "under Getting back."),
                 trialWords(true)},
                client, {client.commandWords({"set-next"})}, QObject::tr("Set it"));
}

BeforePage::Content cancelRestart(const RecabClient &client, const Status &s)
{
    return make(QObject::tr("Cancel the set restart?"),
                {QObject::tr("recab keeps the running system, %1, as the one that starts. The next system stays "
                             "ready, and the next restart starts the running system.")
                     .arg(runningName(s)),
                 QObject::tr("The computer does not restart. You can set the restart again afterwards.")},
                client, {client.commandWords({"commit"})}, QObject::tr("Cancel the restart"));
}

BeforePage::Content keep(const RecabClient &client, const Status &s)
{
    return make(QObject::tr("Keep the running system?"),
                {QObject::tr("%1 becomes the system that every restart starts.").arg(runningName(s)),
                 QObject::tr("recab keeps it by itself as soon as start-up has finished with nothing failed; this "
                             "does it now."),
                 QObject::tr("The computer does not restart.")},
                client, {client.commandWords({"commit"})}, QObject::tr("Keep it"));
}

BeforePage::Content goBack(const RecabClient &client, const Status &s)
{
    const bool onTrial = s.state == QLatin1String("on_trial");
    return make(QObject::tr("Go back to %1?").arg(fromBefore(s)),
                {QObject::tr("The computer restarts into %1, and programs that are open are closed.").arg(fromBefore(s)),
                 onTrial ? QObject::tr("The system you are running now is left as it is, so its changes can be tried "
                                       "again.")
                         : QObject::tr("The system you are running now is kept beside it, so you can return."),
                 QObject::tr("Once the restart has begun, this window cannot stop it.")},
                client, {client.commandWords({"rollback"}), restartCommand}, QObject::tr("Restart into it"));
}

BeforePage::Content reset(const RecabClient &client, const Status &s)
{
    QStringList happens{QObject::tr("Every change in the next system is given up."), holds(s),
                        QObject::tr("It becomes an exact copy of the running system again. Only what differs is "
                                    "copied.")};
    if (destructive(s, QStringLiteral("discard")))
        happens << QObject::tr("recab marks giving up prepared changes destructive: they cannot be recovered "
                               "afterwards.");
    happens << QObject::tr("The running system is not touched, and the computer does not restart. A system from "
                           "before does not come back.");
    return make(QObject::tr("Reset the next system to the running one?"), happens, client,
                {client.commandWords({"discard"}), client.commandWords({"prepare"})}, QObject::tr("Reset"));
}

BeforePage::Content discard(const RecabClient &client, const Status &s)
{
    QStringList happens{QObject::tr("Its changes are given up."), holds(s)};
    if (destructive(s, QStringLiteral("discard")))
        happens << QObject::tr("recab marks giving up prepared changes destructive: they cannot be recovered "
                               "afterwards.");
    happens << QObject::tr("The running system stays exactly as it is, and the computer does not restart.")
            << QObject::tr("The system from before does not come back: its place was already reused.");
    return make(QObject::tr("Discard the next system?"), happens, client, {client.commandWords({"discard"})},
                QObject::tr("Discard"));
}

BeforePage::Content recover(const RecabClient &client, const Status &s)
{
    QStringList happens;
    if (!s.interruptedDuring.isEmpty())
        happens << QObject::tr("Preparing the next system stopped while %1.").arg(s.interruptedDuring);
    happens << QObject::tr("recab looks at what is really there and carries on. Nothing that was finished is done "
                           "again.")
            << QObject::tr("The running system is not touched, and the computer does not restart.");
    return make(QObject::tr("Continue where it stopped?"), happens, client, {client.commandWords({"recover"})},
                QObject::tr("Continue"));
}

BeforePage::Content addRelease(const RecabClient &client, const Status &s, bool fromFile)
{
    // The release as the release page names it: the system's name and its version.
    QString release = s.name(!s.update.version.isEmpty() ? s.update.version : s.updateRelease);
    if (release.isEmpty())
        release = QObject::tr("the release");
    QStringList happens{QObject::tr("The release is merged into the next system, keeping the changes made there."),
                        QObject::tr("If the release and one of those changes disagree about a file, adding stops and "
                                    "you settle it."),
                        QObject::tr("The running system is not touched, and the computer does not restart.")};
    if (s.otherRole == QLatin1String("previous"))
        happens << QObject::tr("The next system takes the place of %1. After this you can no longer go back to %1; "
                               "the running system stays your way back.")
                       .arg(fromBefore(s));
    return make(fromFile ? QObject::tr("Add a release from a file to the next system?")
                         : QObject::tr("Add %1 to the next system?").arg(release),
                happens, client, {client.commandWords({"update", "apply"})}, QObject::tr("Add it"));
}

BeforePage::Content reuse(const RecabClient &client, const Status &s)
{
    BeforePage::Content c;
    c.title = QObject::tr("Make this change in the next system?");
    c.happensTitle = QObject::tr("What happens");
    c.happens = {QObject::tr("The running system is locked, so it is not changed. The change is made in the next "
                             "system and takes effect when you restart."),
                 QObject::tr("The next system starts as an exact copy of the running one. Only what differs is "
                             "copied."),
                 QObject::tr("It takes the place of %1. After this you can no longer go back to %1; the running "
                             "system stays your way back.")
                     .arg(fromBefore(s))};
    c.tableTitle = QObject::tr("What runs");
    c.table = {{QObject::tr("Runs"), client.agentName()}};
    c.go = QObject::tr("Prepare the next system");
    return c;
}

BeforePage::Content restartComputer(const RecabClient &client, const Status &s)
{
    QStringList happens{QObject::tr("The computer restarts now, and programs that are open are closed. Once the "
                                    "restart has begun, this window cannot stop it.")};
    if (s.state == QLatin1String("trial_pending"))
        happens << QObject::tr("A restart into the next system is set, so the next system starts.");
    return make(QObject::tr("Restart the computer?"), happens, client, {restartCommand}, QObject::tr("Restart"));
}

BeforePage::Content operation(const RecabClient &client, const Status &s, const QJsonObject &op)
{
    const QString label = op["label"].toString();
    QStringList argv;
    for (const QJsonValue &word : op["argv"].toArray())
        argv << word.toString();
    QStringList happens{QObject::tr("This runs one operation of recab: %1.").arg(label)};
    if (op["destructive"].toBool())
        happens << QObject::tr("recab marks it destructive. The running system is kept. Prepared changes in the "
                               "other system cannot be recovered.");
    Q_UNUSED(s)
    // The verb, and the second word of `update` and `change`, as recab's own commands read.
    QString command = argv.isEmpty() ? QObject::tr("recab") : QStringLiteral("recab ") + argv.first();
    if (argv.size() > 1 && (argv.first() == QLatin1String("update") || argv.first() == QLatin1String("change")))
        command += QLatin1Char(' ') + argv.at(1);
    return make(label + QLatin1Char('?'), happens, client, {command}, QObject::tr("Continue"));
}

BeforePage::Content releaseStep(const RecabClient &client, const Status &s, const QString &id)
{
    if (id == QLatin1String("update_cancel"))
        return make(QObject::tr("Take the release out of the next system?"),
                    {QObject::tr("recab detaches the release from the next system and retains the changes prepared "
                                 "there."),
                     QObject::tr("The running system is not touched, and the computer does not restart.")},
                    client, {client.commandWords({"update", "cancel"})}, QObject::tr("Take it out"));
    if (id == QLatin1String("update_recover"))
        return make(QObject::tr("Carry on adding the release?"),
                    {QObject::tr("recab continues the merge of the release into the next system where it stopped."),
                     QObject::tr("If the release and one of the changes made there still disagree about a file, it "
                                 "stops again and you settle it."),
                     QObject::tr("The running system is not touched, and the computer does not restart.")},
                    client, {client.commandWords({"update", "recover"})}, QObject::tr("Carry on"));
    Q_UNUSED(s)
    return {};
}

BeforePage::Content adoptBaseline(const RecabClient &client, const Status &s, const QString &manifest)
{
    const QString release = s.mergeRelease().isEmpty() ? QObject::tr("the release") : s.mergeRelease();
    QStringList happens{QObject::tr("recab has no baseline of the original release for the next system, so adding %1 "
                                    "stopped. It never takes the changed system as the original.")
                            .arg(release),
                        QObject::tr("recab imports and verifies the release file %1, and refuses it unless it is "
                                    "exactly the release this system was installed from.")
                            .arg(manifest),
                        QObject::tr("It writes that release as the baseline of the next system. It does not add %1; "
                                    "that is a step of its own afterwards.")
                            .arg(release),
                        QObject::tr("The running system is not touched, and the computer does not restart.")};
    return make(QObject::tr("Adopt the original release as the baseline?"), happens, client,
                {client.commandWords({"update", "adopt-baseline"})}, QObject::tr("Adopt it"));
}

BeforePage::Content reproduce(const RecabClient &client, const Status &s, const QString &package)
{
    const QString release = s.mergeRelease().isEmpty() ? QObject::tr("the release") : s.mergeRelease();
    QStringList happens{QObject::tr("%1 is software added to the next system through Recipe. recab runs its recipe "
                                    "again for %2, through Recipe, using the recipe the next system recorded for it; "
                                    "if that recipe has changed, recab refuses.")
                            .arg(package, release),
                        QObject::tr("Once it is reproduced, recab no longer stops at it. Adding %1 carries on when "
                                    "nothing else is left to settle.")
                            .arg(release),
                        QObject::tr("The running system is not touched, and the computer does not restart.")};
    return make(QObject::tr("Reproduce %1 through Recipe?").arg(package), happens, client,
                {client.commandWords({"update", "reproduce"})}, QObject::tr("Reproduce it"));
}

BeforePage::Content replaceFailed(const RecabClient &client, const Status &s, const QJsonObject &op)
{
    QStringList happens{QObject::tr("%1.").arg(op["label"].toString())};
    // recab's own words for why it did not just prepare again.
    const QString why = s.operation(QStringLiteral("prepare")).reason;
    if (!why.isEmpty())
        happens << QObject::tr("recab says: %1.").arg(why);
    if (op["destructive"].toBool())
        happens << QObject::tr("recab marks it destructive.");
    happens << QObject::tr("The running system is not touched, and the computer does not restart.");
    QStringList argv;
    for (const QJsonValue &word : op["argv"].toArray())
        argv << word.toString();
    return make(op["label"].toString() + QLatin1Char('?'), happens, client,
                {QStringLiteral("recab ") + (argv.isEmpty() ? QStringLiteral("prepare") : argv.first())},
                QObject::tr("Replace it"));
}

} // namespace beforewords
