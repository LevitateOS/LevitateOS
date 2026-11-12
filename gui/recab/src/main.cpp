#include "look.h"
#include "settings.h"
#include "shots.h"
#include "window.h"

#include <QApplication>
#include <QCommandLineParser>
#include <QIcon>

int main(int argc, char **argv)
{
    QApplication app(argc, argv);
    QApplication::setApplicationName("recab-gui");
    QApplication::setApplicationDisplayName(QObject::tr("Recab"));
    QApplication::setApplicationVersion("0.1");
    QApplication::setDesktopFileName("org.levitateos.recab");
    QApplication::setWindowIcon(QIcon::fromTheme("system-software-update"));

    QCommandLineParser parser;
    parser.setApplicationDescription(
        QObject::tr("Change and update the locked system: prepared beside it, in effect after a restart."));
    parser.addHelpOption();
    parser.addVersionOption();
    const QCommandLineOption look("look", "From a checkout: a stand-in look, levitate or acorn.", "name");
    const QCommandLineOption shots("shots", "From a checkout: walk the whole lifecycle against the stand-in, save a "
                                            "picture at each point into this folder, then quit.",
                                   "folder");
    const QCommandLineOption real("real", "With --shots: the story the real recab offers, against the stand-in that "
                                          "speaks its status and commands (fixtures/recab-real).");
    parser.addOptions({look, shots, real});
    parser.process(app);
    if (parser.isSet(look))
        applyLook(parser.value(look));
    // A picture run must not reach the person's real system: it needs every stand-in named.
    if (parser.isSet(shots)) {
        const int refused = guardPictureMode("recab-gui", {"RECAB_GUI_RECAB", "RECAB_GUI_AGENT", "RECAB_GUI_REBOOT"}, {"RECAB_GUI_ROOT_HELPER"});
        if (refused)
            return refused;
    }

    Window window;
    // Opens at the size it had; the picture modes always use the first size.
    rememberWindowSize(&window, "recab", {1100, 700}, !parser.isSet(shots));
    if (parser.isSet(shots))
        return parser.isSet(real) ? takeRealShots(window, parser.value(shots)) : takeShots(window, parser.value(shots));
    window.show();
    return QApplication::exec();
}
