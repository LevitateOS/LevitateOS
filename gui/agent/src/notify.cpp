#include "notify.h"

#include <QProcess>

namespace {

// A string as GVariant text, which is what `gdbus call` reads its arguments
// as: in single quotes, with the backslash and the quote escaped.
QString variantString(const QString &text)
{
    QString out = text;
    out.replace(QLatin1Char('\\'), QLatin1String("\\\\"));
    out.replace(QLatin1Char('\''), QLatin1String("\\'"));
    out.replace(QLatin1Char('\n'), QLatin1String("\\n"));
    return QLatin1Char('\'') + out + QLatin1Char('\'');
}

} // namespace

void notify(const Settings &settings, const QString &title, const QString &text)
{
    if (!settings.notices)
        return;
    QString program;
    QStringList arguments;
    if (settings.notifierSet) {
        if (settings.notifier.isEmpty())
            return;
        program = settings.notifier;
        arguments = {title, text};
    } else {
        // org.freedesktop.Notifications.Notify(app_name, replaces_id, app_icon,
        // summary, body, actions, hints, expire_timeout).
        program = QStringLiteral("gdbus");
        arguments = {QStringLiteral("call"), QStringLiteral("--session"), QStringLiteral("--dest"),
                     QStringLiteral("org.freedesktop.Notifications"), QStringLiteral("--object-path"),
                     QStringLiteral("/org/freedesktop/Notifications"), QStringLiteral("--method"),
                     QStringLiteral("org.freedesktop.Notifications.Notify"), variantString(QStringLiteral("LevitateOS")),
                     QStringLiteral("0"), variantString(QString()), variantString(title), variantString(text),
                     QStringLiteral("[]"), QStringLiteral("{}"), QStringLiteral("10000")};
    }
    QProcess process;
    process.setProgram(program);
    process.setArguments(arguments);
    process.setStandardInputFile(QProcess::nullDevice());
    process.setStandardOutputFile(QProcess::nullDevice());
    process.setStandardErrorFile(QProcess::nullDevice());
    process.start();
    if (!process.waitForFinished(3000)) {
        process.kill();
        process.waitForFinished(500);
    }
}
