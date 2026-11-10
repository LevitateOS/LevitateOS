#include "look.h"
#include "shots.h"
#include "window.h"

#include <QApplication>
#include <QCommandLineParser>
#include <QIcon>

int main(int argc, char **argv)
{
    QApplication app(argc, argv);
    QApplication::setApplicationName("disk-gui");
    QApplication::setApplicationDisplayName(QObject::tr("Disk"));
    QApplication::setApplicationVersion("0.1");
    QApplication::setDesktopFileName("org.levitateos.disk");
    QApplication::setWindowIcon(QIcon::fromTheme("drive-harddisk"));

    QCommandLineParser parser;
    parser.setApplicationDescription(QObject::tr("See where the mess on the disk is, and hand it to the agent."));
    parser.addHelpOption();
    parser.addVersionOption();
    const QCommandLineOption look("look", "From a checkout: a stand-in look, levitate or acorn.", "name");
    const QCommandLineOption shots("shots", "From a checkout: look, save a picture of what was found and of the "
                                            "hand-over into this folder, then quit.",
                                   "folder");
    const QCommandLineOption handOver("hand-over", "With --shots: also hand the mess to the agent and look again.");
    parser.addOptions({look, shots, handOver});
    parser.process(app);
    if (parser.isSet(look))
        applyLook(parser.value(look));

    Window window;
    window.resize(1000, 680);
    if (parser.isSet(shots))
        return takeShots(window, parser.value(shots), parser.isSet(handOver));
    window.show();
    return QApplication::exec();
}
