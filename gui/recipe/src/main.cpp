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
    const QCommandLineOption real("try", "From a checkout, with --shots: install this recipe with the real recipe, "
                                         "save a picture at each point, then remove it.", "recipe");
    const QCommandLineOption keep("keep", "With --try: leave the recipe installed.");
    const QCommandLineOption readOnly("read-only", "With --try: install nothing, picture only the pages that read.");
    parser.addOptions({look, shots, real, keep, readOnly});
    parser.process(app);
    if (parser.isSet(look))
        applyLook(parser.value(look));

    Window window;
    window.resize(1100, 700);
    if (parser.isSet(shots) && parser.isSet(real))
        return tryRecipe(window, parser.value(real), parser.value(shots), parser.isSet(keep), parser.isSet(readOnly));
    if (parser.isSet(shots))
        return takeShots(window, parser.value(shots));
    window.show();
    return QApplication::exec();
}
