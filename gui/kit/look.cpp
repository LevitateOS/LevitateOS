#include "look.h"

#include <QApplication>
#include <QDir>
#include <QElapsedTimer>
#include <QEventLoop>
#include <QFont>
#include <QPalette>
#include <QStyleFactory>
#include <QTimer>

#include <cstdio>
#include <pwd.h>
#include <unistd.h>

void applyLook(const QString &look)
{
    const bool acorn = look == QLatin1String("acorn");
    QApplication::setStyle(QStyleFactory::create("Fusion"));
    QFont font("Noto Sans");
    font.setPointSizeF(10);
    QApplication::setFont(font);

    QPalette p;
    auto set = [&p](QPalette::ColorRole role, const char *colour) { p.setColor(role, QColor(colour)); };
    if (acorn) {
        set(QPalette::Window, "#f2eee4");
        set(QPalette::WindowText, "#14221a");
        set(QPalette::Base, "#fcfaf5");
        set(QPalette::AlternateBase, "#f2eee4");
        set(QPalette::Text, "#14221a");
        set(QPalette::Button, "#ece7da");
        set(QPalette::ButtonText, "#14221a");
        set(QPalette::Highlight, "#644532");
        set(QPalette::HighlightedText, "#f2eee4");
        set(QPalette::Link, "#644532");
        set(QPalette::PlaceholderText, "#59645b");
        set(QPalette::ToolTipBase, "#fcfaf5");
        set(QPalette::ToolTipText, "#14221a");
        set(QPalette::Light, "#ffffff");
        set(QPalette::Midlight, "#f7f4ec");
        set(QPalette::Mid, "#c4c2b3");
        set(QPalette::Dark, "#aeab9b");
        set(QPalette::Shadow, "#7a8075");
    } else {
        set(QPalette::Window, "#202326");
        set(QPalette::WindowText, "#fcfcfc");
        set(QPalette::Base, "#141618");
        set(QPalette::AlternateBase, "#1b1e20");
        set(QPalette::Text, "#fcfcfc");
        set(QPalette::Button, "#31363b");
        set(QPalette::ButtonText, "#fcfcfc");
        set(QPalette::Highlight, "#c7d9ba");
        set(QPalette::HighlightedText, "#14221a");
        set(QPalette::Link, "#c7d9ba");
        set(QPalette::PlaceholderText, "#a8b0b8");
        set(QPalette::ToolTipBase, "#31363b");
        set(QPalette::ToolTipText, "#fcfcfc");
        set(QPalette::Light, "#474c52");
        set(QPalette::Midlight, "#3b3f44");
        set(QPalette::Mid, "#2e3236");
        set(QPalette::Dark, "#16181a");
        set(QPalette::Shadow, "#0c0d0e");
    }
    QApplication::setPalette(p);
}

void settle(int milliseconds)
{
    QEventLoop loop;
    QTimer::singleShot(milliseconds, &loop, &QEventLoop::quit);
    loop.exec();
}

bool waitFor(const std::function<bool()> &holds, int seconds)
{
    QElapsedTimer clock;
    clock.start();
    const qint64 limit = qint64(qMax(seconds * 3, 60)) * 1000;
    while (!holds() && clock.elapsed() < limit)
        settle(50);
    const bool held = holds();
    settle(150);
    return held;
}

namespace {
bool inPictureMode = false;
}

bool pictureMode()
{
    return inPictureMode;
}

int guardPictureMode(const QString &program, const QStringList &programs, const QStringList &mayBeEmpty)
{
    QStringList named{"LEVITATE_ACTIVITY_AGENT", "LEVITATE_AGENT_CODEX", "LEVITATE_AGENT_CLAUDE", "HOME",
                      "XDG_CONFIG_HOME",         "XDG_STATE_HOME",        "XDG_DATA_HOME",        "XDG_RUNTIME_DIR"};
    named << programs;
    QStringList empty{"LEVITATE_AGENT_ROOT_HELPER"};
    empty << mayBeEmpty;
    QStringList missing;
    for (const QString &name : std::as_const(named))
        if (qEnvironmentVariable(name.toLatin1().constData()).isEmpty())
            missing << name;
    for (const QString &name : std::as_const(empty))
        if (!qEnvironmentVariableIsSet(name.toLatin1().constData()))
            missing << name;
    QString sentence;
    if (!missing.isEmpty())
        sentence = QStringLiteral("%1 --shots would reach this computer for real, so it does not start. Name these in the "
                                  "environment first, each a stand-in or a folder of scratch space (the root helper may be "
                                  "named empty): %2. The README of the app has the command.")
                       .arg(program, missing.join(QStringLiteral(", ")));
    const passwd *entry = getpwuid(getuid());
    if (sentence.isEmpty() && entry && QDir::cleanPath(qEnvironmentVariable("HOME")) == QDir::cleanPath(QString::fromLocal8Bit(entry->pw_dir)))
        sentence = QStringLiteral("%1 --shots would write into your own home folder, so it does not start. Point HOME and the "
                                  "XDG folders into a folder of scratch space.")
                       .arg(program);
    if (!sentence.isEmpty()) {
        fprintf(stderr, "%s\n", qPrintable(sentence));
        return 2;
    }
    // A notice on the desktop of whoever runs a check is not wanted; one that names a notifier gets it.
    if (!qEnvironmentVariableIsSet("LEVITATE_AGENT_NOTIFIER"))
        qputenv("LEVITATE_AGENT_NOTIFIER", "");
    inPictureMode = true;
    return 0;
}
