#include "look.h"

#include <QApplication>
#include <QEventLoop>
#include <QFont>
#include <QPalette>
#include <QStyleFactory>
#include <QTimer>

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
