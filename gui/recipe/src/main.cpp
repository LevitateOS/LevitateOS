#include "shots.h"
#include "window.h"

#include <QApplication>
#include <QCommandLineParser>
#include <QIcon>

int main(int argc, char **argv)
{
    QApplication app(argc, argv);
    QApplication::setApplicationName("recipe-gui");
    QApplication::setApplicationDisplayName(QObject::tr("Recipe"));
    QApplication::setApplicationVersion("0.1");
    QApplication::setDesktopFileName("org.levitateos.recipe");
    QApplication::setWindowIcon(QIcon::fromTheme("system-software-install"));

    QCommandLineParser parser;
    parser.setApplicationDescription(QObject::tr("Find, install and update software from recipes."));
    parser.addHelpOption();
    parser.addVersionOption();
    const QCommandLineOption look("look", "From a checkout: a stand-in look, levitate or acorn.", "name");
    const QCommandLineOption shots("shots", "From a checkout: save a picture of every page into this folder, then quit.",
                                   "folder");
    parser.addOptions({look, shots});
    parser.process(app);
    if (parser.isSet(look))
        applyLook(parser.value(look));

    Window window;
    window.resize(1100, 700);
    if (parser.isSet(shots))
        return takeShots(window, parser.value(shots));
    window.show();
    return QApplication::exec();
}
